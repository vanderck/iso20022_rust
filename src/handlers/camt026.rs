use async_trait::async_trait;
use quick_xml::de::from_str;
use crate::mapper::Message;
use crate::messages::camt02600110 as m;
use crate::processor::{flow_coordinator::FlowCoordinator, handler_support::HandlerSupport as H,
    journey_event::{JourneyEvent, JourneyEventKind, JourneyRefs, SupportedMessage},
    transaction_scope::TransactionScope};

pub struct Document;

fn refs_from_underlying(u: &m::UnderlyingTransaction8Choice, assgnmt_id: &str, case_id: Option<String>) -> JourneyRefs {
    if let Some(ib) = u.intr_bk.as_ref() {
        return JourneyRefs {
            original_msg_id: ib.orgnl_grp_inf.as_ref().map(|g| g.orgnl_msg_id.clone()),
            end_to_end_id: ib.orgnl_end_to_end_id.clone(),
            tx_id: ib.orgnl_tx_id.clone(),
            uetr: ib.orgnl_uetr.clone(),
            clr_sys_ref: None, case_id,
            assignment_id: Some(assgnmt_id.to_string()),
        };
    }
    if let Some(init) = u.initn.as_ref() {
        return JourneyRefs {
            original_msg_id: init.orgnl_grp_inf.as_ref().map(|g| g.orgnl_msg_id.clone()),
            end_to_end_id: init.orgnl_end_to_end_id.clone(),
            tx_id: None,
            uetr: init.orgnl_uetr.clone(),
            clr_sys_ref: None, case_id,
            assignment_id: Some(assgnmt_id.to_string()),
        };
    }
    JourneyRefs { case_id, assignment_id: Some(assgnmt_id.to_string()), ..Default::default() }
}

#[async_trait]
impl Message for Document {
    async fn consume(
        &self,
        ctx: &crate::ProcessingContext,
        msg: &str,
        counterparty_bic: Option<&str>,
    ) -> Result<String, String> {
        let doc: m::Document = from_str(msg).map_err(|e| e.to_string())?;
        let body = &doc.ubl_to_apply;
        let msg_type = "camt.026.001.10";
        let msg_id = body.assgnmt.id.clone();
        let mut scope = TransactionScope::new(ctx.crud_client.clone());
        H::archive_message(scope.crud(), msg_type, "INBOUND", &msg_id, msg).await.map_err(|e| e.to_string())?;

        let case_id = body.case.as_ref().map(|c| c.id.clone());
        let refs = refs_from_underlying(&body.undrlyg, &msg_id, case_id);

        let jid = H::resolve_journey_id(scope.crud(), &refs).await.map_err(|e| e.to_string())?;
        H::open_investigation(scope.crud(), jid.as_deref(), &refs, msg_type, None,
            &format!("Unable to apply: assgnmt_id={}", msg_id)).await.map_err(|e| e.to_string())?;

        let event = JourneyEvent { source_message: SupportedMessage::Camt026,
            kind: JourneyEventKind::UnableToApply, refs: refs.clone(),
            iso_status: None, reason_code: None,
            description: format!("Unable to apply: assgnmt_id={}", msg_id) };
        H::process_event(scope.crud(), msg_type, &msg_id, event).await.map_err(|e| e.to_string())?;

        let flow = FlowCoordinator::execute(scope.crud(), &ctx.db_pool, ctx.engine_client.clone(), &ctx.http_client, &ctx.config, SupportedMessage::Camt026, &msg_id, &refs,
            None, None, counterparty_bic.unwrap_or("UNKNOWN")).await.map_err(|e| e.to_string())?;
        scope.commit().await.map_err(|e| e.to_string())?;
        Ok(format!("processed {} assgnmt_id={} outbound={}", msg_type, msg_id, flow.outbound_messages.len()))
    }

    async fn generate(&self, ctx: &crate::ProcessingContext) -> Result<String, String> {
        let now = chrono::Utc::now();
        let id = H::next_msg_id(&mut ctx.uidgen_client.clone(), "CAMT026").await.map_err(|e| e.to_string())?;
        let body = m::UnableToApplyV10 { assgnmt: m::CaseAssignment6 { id: id.clone(), cre_dt_tm: now, ..Default::default() }, ..Default::default() };
        let doc_xml = quick_xml::se::to_string(&m::Document::new(body)).map_err(|e| format!("{e}"))?;
        self.wrap(&doc_xml, "camt.026.001.10", &id, &ctx.config.own_bic, "RECEIVERBIC")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn refs_from_underlying_prefers_interbank_variant() {
        let u = m::UnderlyingTransaction8Choice {
            intr_bk: Some(m::UnderlyingPaymentTransaction8 {
                orgnl_grp_inf: Some(m::UnderlyingGroupInformation1 { orgnl_msg_id: "MSG1".into(), ..Default::default() }),
                orgnl_end_to_end_id: Some("E2E1".into()),
                orgnl_tx_id: Some("TX1".into()),
                orgnl_uetr: Some("UETR1".into()),
                ..Default::default()
            }),
            initn: Some(m::UnderlyingPaymentInstruction9 {
                orgnl_end_to_end_id: Some("SHOULD_NOT_BE_USED".into()),
                ..Default::default()
            }),
            ..Default::default()
        };
        let refs = refs_from_underlying(&u, "ASSGN1", Some("CASE1".into()));
        assert_eq!(refs.original_msg_id.as_deref(), Some("MSG1"));
        assert_eq!(refs.end_to_end_id.as_deref(), Some("E2E1"));
        assert_eq!(refs.tx_id.as_deref(), Some("TX1"));
        assert_eq!(refs.uetr.as_deref(), Some("UETR1"));
        assert_eq!(refs.case_id.as_deref(), Some("CASE1"));
        assert_eq!(refs.assignment_id.as_deref(), Some("ASSGN1"));
    }

    #[test]
    fn refs_from_underlying_falls_back_to_initiation_variant() {
        let u = m::UnderlyingTransaction8Choice {
            intr_bk: None,
            initn: Some(m::UnderlyingPaymentInstruction9 {
                orgnl_grp_inf: Some(m::UnderlyingGroupInformation1 { orgnl_msg_id: "MSG2".into(), ..Default::default() }),
                orgnl_end_to_end_id: Some("E2E2".into()),
                orgnl_uetr: Some("UETR2".into()),
                ..Default::default()
            }),
            ..Default::default()
        };
        let refs = refs_from_underlying(&u, "ASSGN2", None);
        assert_eq!(refs.original_msg_id.as_deref(), Some("MSG2"));
        assert_eq!(refs.end_to_end_id.as_deref(), Some("E2E2"));
        assert_eq!(refs.uetr.as_deref(), Some("UETR2"));
        // The initiation variant has no OrgnlTxId field at all.
        assert_eq!(refs.tx_id, None);
        assert_eq!(refs.clr_sys_ref, None);
        assert_eq!(refs.case_id, None);
    }

    #[test]
    fn refs_from_underlying_yields_bare_refs_when_neither_variant_present() {
        let u = m::UnderlyingTransaction8Choice { intr_bk: None, initn: None, ..Default::default() };
        let refs = refs_from_underlying(&u, "ASSGN3", Some("CASE3".into()));
        assert_eq!(refs.original_msg_id, None);
        assert_eq!(refs.end_to_end_id, None);
        assert_eq!(refs.case_id.as_deref(), Some("CASE3"));
        assert_eq!(refs.assignment_id.as_deref(), Some("ASSGN3"));
    }
}
