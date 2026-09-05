use crate::context::{CrudClient, UidgenClient};
use crate::crudgrpc::{CreateRequest, DbPath, Fld, ReadRequest, UpdateRequest};

use super::journey_event::{JourneyEvent, JourneyRefs, PartyData};
use super::state_effects::StateEffects;

pub struct HandlerSupport;

impl HandlerSupport {
    pub fn db_path(table: &str) -> DbPath {
        DbPath { server: "CBS".into(), schema: "iso20022".into(), table: table.into() }
    }

    /// Mint a cluster-unique outbound message id via uidgen, formatted as
    /// `<prefix>-<snowflake-id>`. Replaces the previous
    /// `format!("{prefix}-{}", now.timestamp_millis())` pattern, which had no
    /// cross-instance uniqueness guarantee and could collide within the same
    /// millisecond under concurrent load.
    pub async fn next_msg_id(uidgen: &mut UidgenClient, prefix: &str) -> Result<String, tonic::Status> {
        let resp = uidgen.next_id(crate::uidgen::NextIdRequest {}).await?;
        Ok(format!("{prefix}-{}", resp.into_inner().id))
    }

    /// Trim and reject empty / "unknown" strings.
    pub fn ne(s: &Option<String>) -> Option<String> {
        s.as_ref().and_then(|v| {
            let t = v.trim().to_string();
            if t.is_empty() || t.eq_ignore_ascii_case("unknown") { None } else { Some(t) }
        })
    }

    // ── CRUD helpers ─────────────────────────────────────────────────────

    pub async fn read_one(
        crud: &mut CrudClient,
        table: &str, column: &str, where_clause: Vec<Fld>,
    ) -> Result<Option<String>, tonic::Status> {
        let req = ReadRequest {
            path: Some(Self::db_path(table)),
            column: vec![column.into()], where_clause,
            page: 0, page_size: Some(1),
        };
        let resp = crud.read(req).await?.into_inner();
        Ok(resp.data.into_iter().next().and_then(|r| r.list.into_iter().next()))
    }

    pub async fn archive_message(
        crud: &mut CrudClient,
        msg_type: &str, direction: &str, msg_id: &str, raw_xml: &str,
    ) -> Result<(), tonic::Status> {
        let exists = Self::read_one(crud, "iso_messages", "message_internal_id", vec![
            Fld { name: "msg_type".into(), relation: None, value: msg_type.into() },
            Fld { name: "direction".into(), relation: None, value: direction.into() },
            Fld { name: "msg_id".into(), relation: None, value: msg_id.into() },
        ]).await?;
        if exists.is_some() { return Ok(()); }
        let _ = crud.create(CreateRequest {
            who: "iso_handler".into(), review: Some(false),
            path: Some(Self::db_path("iso_messages")),
            data: vec![
                Fld { name: "msg_type".into(), relation: None, value: msg_type.into() },
                Fld { name: "direction".into(), relation: None, value: direction.into() },
                Fld { name: "msg_id".into(), relation: None, value: msg_id.into() },
                Fld { name: "raw_xml".into(), relation: None, value: raw_xml.into() },
            ],
        }).await?;
        Ok(())
    }

    pub async fn link_message_to_journey(
        crud: &mut CrudClient,
        msg_id: &str, journey_id: &str,
    ) -> Result<(), tonic::Status> {
        let _ = crud.update(UpdateRequest {
            who: "iso_handler".into(), review: Some(false),
            path: Some(Self::db_path("iso_messages")),
            where_clause: vec![Fld { name: "msg_id".into(), relation: None, value: msg_id.to_string() }],
            data: vec![Fld { name: "journey_id".into(), relation: None, value: journey_id.to_string() }],
        }).await?;
        Ok(())
    }

    // ── Journey resolution ───────────────────────────────────────────────

    pub async fn resolve_journey_id(
        crud: &mut CrudClient,
        refs: &JourneyRefs,
    ) -> Result<Option<String>, tonic::Status> {
        for (tbl, col, val) in [
            ("payment_journeys", "uetr",          refs.uetr.as_deref()),
            ("payment_journeys", "tx_id",         refs.tx_id.as_deref()),
            ("payment_journeys", "end_to_end_id", refs.end_to_end_id.as_deref()),
            ("iso_messages",     "msg_id",        refs.original_msg_id.as_deref()),
            ("payment_journeys", "clr_sys_ref",   refs.clr_sys_ref.as_deref()),
        ] {
            if let Some(v) = val {
                if let Some(id) = Self::read_one(crud, tbl, "journey_id",
                    vec![Fld { name: col.into(), relation: None, value: v.into() }]).await? {
                    return Ok(Some(id));
                }
            }
        }
        Ok(None)
    }

    pub async fn create_journey(
        crud: &mut CrudClient,
        refs: &JourneyRefs, msg_type: &str, msg_id: &str,
    ) -> Result<String, tonic::Status> {
        Self::create_journey_with_party(crud, refs, msg_type, msg_id, &PartyData::default()).await
    }

    /// Same as [`create_journey`](Self::create_journey), additionally persisting
    /// debtor/creditor party data captured from the originating message so that
    /// any later-generated outbound `pacs.008` for this journey can be built
    /// with real party identification instead of empty `<Dbtr/>`/`<Cdtr/>`.
    pub async fn create_journey_with_party(
        crud: &mut CrudClient,
        refs: &JourneyRefs, msg_type: &str, msg_id: &str, party: &PartyData,
    ) -> Result<String, tonic::Status> {
        let journey_id = format!("JRN-{}", uuid::Uuid::new_v4());
        // Generate UETR if not present (required for SWIFT gpi origination).
        let uetr = refs.uetr.clone()
            .unwrap_or_else(crate::processor::compliance::generate_uetr);
        let mut data = vec![
            Fld { name: "journey_id".into(),         relation: None, value: journey_id.clone() },
            Fld { name: "current_status".into(),     relation: None, value: "RECEIVED".into() },
            Fld { name: "initiating_msg_type".into(), relation: None, value: msg_type.into() },
            Fld { name: "initiating_msg_id".into(),  relation: None, value: msg_id.into() },
            Fld { name: "uetr".into(),               relation: None, value: uetr },
        ];
        if let Some(v) = refs.tx_id.as_deref() {
            data.push(Fld { name: "tx_id".into(), relation: None, value: v.to_string() });
        }
        if let Some(v) = refs.end_to_end_id.as_deref() {
            data.push(Fld { name: "end_to_end_id".into(), relation: None, value: v.to_string() });
        }
        if let Some(v) = refs.clr_sys_ref.as_deref() {
            data.push(Fld { name: "clr_sys_ref".into(), relation: None, value: v.to_string() });
        }
        if let Some(v) = party.debtor_name.as_deref() {
            data.push(Fld { name: "debtor_name".into(), relation: None, value: v.to_string() });
        }
        if let Some(v) = party.debtor_account.as_deref() {
            data.push(Fld { name: "debtor_account".into(), relation: None, value: v.to_string() });
        }
        if let Some(v) = party.creditor_name.as_deref() {
            data.push(Fld { name: "creditor_name".into(), relation: None, value: v.to_string() });
        }
        if let Some(v) = party.creditor_account.as_deref() {
            data.push(Fld { name: "creditor_account".into(), relation: None, value: v.to_string() });
        }
        let _ = crud.create(CreateRequest {
            who: "iso_handler".into(), review: Some(false),
            path: Some(Self::db_path("payment_journeys")), data,
        }).await?;
        crate::processor::metrics::record_journey_created(msg_type);
        Ok(journey_id)
    }

    // ── Event processing ─────────────────────────────────────────────────

    pub fn event_id(msg_type: &str, msg_id: &str, event: &JourneyEvent) -> String {
        let code = event.status_history_code();
        if let Some(u) = event.refs.uetr.as_deref()          { return format!("{msg_type}|{msg_id}|uetr={u}|{code}"); }
        if let Some(t) = event.refs.tx_id.as_deref()         { return format!("{msg_type}|{msg_id}|tx={t}|{code}"); }
        if let Some(e) = event.refs.end_to_end_id.as_deref() { return format!("{msg_type}|{msg_id}|e2e={e}|{code}"); }
        format!("{msg_type}|{msg_id}|{code}")
    }

    pub async fn process_event(
        crud: &mut CrudClient,
        msg_type: &str, msg_id: &str, event: JourneyEvent,
    ) -> Result<Option<String>, tonic::Status> {
        let journey_id = Self::resolve_journey_id(crud, &event.refs).await?;
        let eid = Self::event_id(msg_type, msg_id, &event);
        StateEffects::append_status_history(crud, journey_id.as_deref(), &event, &eid).await?;
        if let Some(jid) = journey_id.as_deref() {
            let _ = Self::link_message_to_journey(crud, msg_id, jid).await;
            let next = StateEffects::apply_event_to_journey(crud, jid, &event).await?;
            return Ok(next.map(|s| s.as_db_value().to_string()));
        }
        Ok(None)
    }

    pub async fn open_investigation(
        crud: &mut CrudClient,
        journey_id: Option<&str>, refs: &JourneyRefs,
        investigation_type: &str, reason_code: Option<&str>, additional_info: &str,
    ) -> Result<(), tonic::Status> {
        let key = refs.assignment_id.as_deref()
            .or(refs.case_id.as_deref())
            .unwrap_or(additional_info);
        let exists = Self::read_one(crud, "investigations", "investigation_id",
            vec![Fld { name: "assignment_id".into(), relation: None, value: key.to_string() }]).await?;
        if exists.is_some() { return Ok(()); }
        let mut data = vec![
            Fld { name: "assignment_id".into(),       relation: None, value: key.to_string() },
            Fld { name: "investigation_type".into(),  relation: None, value: investigation_type.to_string() },
            Fld { name: "investigation_status".into(), relation: None, value: "OPEN".to_string() },
            Fld { name: "additional_info".into(),     relation: None, value: additional_info.to_string() },
        ];
        if let Some(j) = journey_id { data.push(Fld { name: "journey_id".into(), relation: None, value: j.to_string() }); }
        if let Some(c) = refs.case_id.as_deref() { data.push(Fld { name: "case_id".into(), relation: None, value: c.to_string() }); }
        if let Some(r) = reason_code { data.push(Fld { name: "reason_code".into(), relation: None, value: r.to_string() }); }
        let _ = crud.create(CreateRequest { who: "iso_handler".into(), review: Some(false),
            path: Some(Self::db_path("investigations")), data }).await?;
        crate::processor::metrics::record_investigation_opened(investigation_type);
        Ok(())
    }

    pub async fn close_investigation(
        crud: &mut CrudClient,
        refs: &JourneyRefs, resolution_status: &str,
    ) -> Result<(), tonic::Status> {
        if let Some(k) = refs.assignment_id.as_deref().or(refs.case_id.as_deref()) {
            let _ = crud.update(UpdateRequest {
                who: "iso_handler".into(), review: Some(false),
                path: Some(Self::db_path("investigations")),
                where_clause: vec![Fld { name: "assignment_id".into(), relation: None, value: k.to_string() }],
                data: vec![Fld { name: "investigation_status".into(), relation: None, value: resolution_status.to_string() }],
            }).await?;
            crate::processor::metrics::record_investigation_closed("unknown", resolution_status);
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ── ne ──────────────────────────────────────────────────────────────

    #[test]
    fn ne_none_stays_none() {
        assert_eq!(HandlerSupport::ne(&None), None);
    }

    #[test]
    fn ne_rejects_empty_and_whitespace_only() {
        assert_eq!(HandlerSupport::ne(&Some("".into())), None);
        assert_eq!(HandlerSupport::ne(&Some("   ".into())), None);
        assert_eq!(HandlerSupport::ne(&Some("\t\n".into())), None);
    }

    #[test]
    fn ne_rejects_unknown_case_insensitively() {
        assert_eq!(HandlerSupport::ne(&Some("unknown".into())), None);
        assert_eq!(HandlerSupport::ne(&Some("UNKNOWN".into())), None);
        assert_eq!(HandlerSupport::ne(&Some("Unknown".into())), None);
        assert_eq!(HandlerSupport::ne(&Some("  unknown  ".into())), None);
    }

    #[test]
    fn ne_trims_and_keeps_real_values() {
        assert_eq!(HandlerSupport::ne(&Some("  hello  ".into())), Some("hello".to_string()));
        assert_eq!(HandlerSupport::ne(&Some("unknowable".into())), Some("unknowable".to_string()));
    }

    #[test]
    fn db_path_uses_iso20022_schema_and_cbs_server() {
        let path = HandlerSupport::db_path("payment_journeys");
        assert_eq!(path.server, "CBS");
        assert_eq!(path.schema, "iso20022");
        assert_eq!(path.table, "payment_journeys");
    }

    // ── event_id ────────────────────────────────────────────────────────

    fn event_with_refs(refs: JourneyRefs) -> JourneyEvent {
        JourneyEvent {
            source_message: crate::processor::journey_event::SupportedMessage::Pacs008,
            kind: crate::processor::journey_event::JourneyEventKind::FiPaymentSent,
            refs,
            iso_status: None,
            reason_code: None,
            description: "test".into(),
        }
    }

    #[test]
    fn event_id_prefers_uetr_over_everything_else() {
        let refs = JourneyRefs {
            uetr: Some("U1".into()),
            tx_id: Some("T1".into()),
            end_to_end_id: Some("E1".into()),
            ..Default::default()
        };
        let id = HandlerSupport::event_id("pacs.008", "MSG1", &event_with_refs(refs));
        assert!(id.contains("uetr=U1"));
        assert!(!id.contains("tx=T1"));
    }

    #[test]
    fn event_id_falls_back_to_tx_id_when_no_uetr() {
        let refs = JourneyRefs { tx_id: Some("T1".into()), end_to_end_id: Some("E1".into()), ..Default::default() };
        let id = HandlerSupport::event_id("pacs.008", "MSG1", &event_with_refs(refs));
        assert!(id.contains("tx=T1"));
    }

    #[test]
    fn event_id_falls_back_to_end_to_end_id_when_no_uetr_or_tx_id() {
        let refs = JourneyRefs { end_to_end_id: Some("E1".into()), ..Default::default() };
        let id = HandlerSupport::event_id("pacs.008", "MSG1", &event_with_refs(refs));
        assert!(id.contains("e2e=E1"));
    }

    #[test]
    fn event_id_falls_back_to_msg_type_and_msg_id_when_no_refs_present() {
        let id = HandlerSupport::event_id("pacs.008", "MSG1", &event_with_refs(JourneyRefs::default()));
        assert_eq!(id, "pacs.008|MSG1|FI_PAYMENT_SENT");
    }

    #[test]
    fn event_id_is_stable_for_identical_inputs() {
        let refs = JourneyRefs { uetr: Some("U1".into()), ..Default::default() };
        let a = HandlerSupport::event_id("pacs.008", "MSG1", &event_with_refs(refs.clone()));
        let b = HandlerSupport::event_id("pacs.008", "MSG1", &event_with_refs(refs));
        assert_eq!(a, b);
    }
}
