use async_trait::async_trait;
use quick_xml::de::from_str;
use tracing::warn;
use crate::mapper::Message;
use crate::messages::camt02900113 as m;
use crate::processor::{flow_coordinator::FlowCoordinator, handler_support::HandlerSupport as H,
    journey_event::{JourneyEvent, JourneyEventKind, JourneyRefs, SupportedMessage},
    transaction_scope::TransactionScope};

pub struct Document;

/// Map a cancellation's per-tx status (preferred) or investigation-level
/// confirmation code (fallback) to the journey event kind it represents.
fn resolve_kind(tx_cxl_sts: Option<&m::CancellationIndividualStatus1Code>, inv_conf: Option<&str>) -> JourneyEventKind {
    match tx_cxl_sts {
        Some(m::CancellationIndividualStatus1Code::Rjcr) => JourneyEventKind::InvestigationResolutionRejected,
        Some(m::CancellationIndividualStatus1Code::Accr) => JourneyEventKind::InvestigationResolutionAccepted,
        Some(m::CancellationIndividualStatus1Code::Pdcr) => JourneyEventKind::InvestigationResolutionPending,
        _ => match inv_conf {
            Some("CNCL") | Some("MODI") | Some("IPAY") | Some("ACDA") => JourneyEventKind::InvestigationResolutionAccepted,
            Some("RJCR") => JourneyEventKind::InvestigationResolutionRejected,
            _ => JourneyEventKind::InvestigationResolutionPending,
        },
    }
}

/// The investigation-closing status to record for a resolved event kind.
fn resolution_status_for(kind: JourneyEventKind) -> &'static str {
    match kind {
        JourneyEventKind::InvestigationResolutionAccepted => "RESOLVED",
        JourneyEventKind::InvestigationResolutionRejected => "REJECTED",
        _ => "PENDING",
    }
}

/// The wire-format string for a per-tx cancellation status, used as the
/// event's `iso_status` when present (falling back to the investigation
/// confirmation code otherwise - see `consume`).
fn tx_cxl_sts_str(s: &m::CancellationIndividualStatus1Code) -> &'static str {
    match s {
        m::CancellationIndividualStatus1Code::Rjcr => "RJCR",
        m::CancellationIndividualStatus1Code::Accr => "ACCR",
        m::CancellationIndividualStatus1Code::Pdcr => "PDCR",
        m::CancellationIndividualStatus1Code::Unknown => {
            warn!("camt029: received unrecognized CancellationIndividualStatus1Code, treating as UNKNOWN");
            "UNKNOWN"
        }
    }
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
        let body = &doc.rsltn_of_invstgtn;
        let msg_type = "camt.029.001.13";
        let msg_id = body.assgnmt.id.clone();
        let mut scope = TransactionScope::new(ctx.crud_client.clone());
        H::archive_message(scope.crud(), msg_type, "INBOUND", &msg_id, msg).await.map_err(|e| e.to_string())?;

        // Transaction-level from first cancellation detail.
        let tx = body.cxl_dtls.first().and_then(|cd| cd.tx_inf_and_sts.first());

        // Status: per-tx (RJCR/ACCR/PDCR) → investigation-level conf.
        let inv_conf = H::ne(&body.sts.conf);
        let kind = resolve_kind(tx.and_then(|t| t.tx_cxl_sts.as_ref()), inv_conf.as_deref());

        let reason = tx.and_then(|t| t.cxl_sts_rsn_inf.first())
            .and_then(|sri| sri.rsn.as_ref())
            .and_then(|r| H::ne(&r.cd).or_else(|| H::ne(&r.prtry)));

        let case_id = body.rslvd_case.as_ref().map(|c| c.id.clone());
        let refs = JourneyRefs {
            original_msg_id: tx.and_then(|t| t.orgnl_grp_inf.as_ref()).map(|g| g.orgnl_msg_id.clone()),
            end_to_end_id: tx.and_then(|t| t.orgnl_end_to_end_id.clone()),
            tx_id: tx.and_then(|t| t.orgnl_tx_id.clone()),
            uetr: tx.and_then(|t| t.orgnl_uetr.clone()),
            clr_sys_ref: tx.and_then(|t| t.orgnl_clr_sys_ref.clone()),
            case_id, assignment_id: Some(msg_id.clone()),
        };

        // Close or update investigation.
        let resolution_status = resolution_status_for(kind);
        H::close_investigation(scope.crud(), &refs, resolution_status).await.map_err(|e| e.to_string())?;

        let tx_sts_str = tx.and_then(|t| t.tx_cxl_sts.as_ref()).map(|s| tx_cxl_sts_str(s).to_string());
        let iso_status = tx_sts_str.or(inv_conf);

        let event = JourneyEvent { source_message: SupportedMessage::Camt029,
            kind, refs: refs.clone(), iso_status, reason_code: reason,
            description: format!("Investigation resolution: assgnmt_id={}", msg_id) };
        H::process_event(scope.crud(), msg_type, &msg_id, event).await.map_err(|e| e.to_string())?;

        let flow = FlowCoordinator::execute(scope.crud(), &ctx.db_pool, ctx.engine_client.clone(), &ctx.http_client, &ctx.config, SupportedMessage::Camt029, &msg_id, &refs,
            None, None, counterparty_bic.unwrap_or("UNKNOWN")).await.map_err(|e| e.to_string())?;
        scope.commit().await.map_err(|e| e.to_string())?;
        Ok(format!("processed {} assgnmt_id={} outbound={}", msg_type, msg_id, flow.outbound_messages.len()))
    }

    async fn generate(&self, ctx: &crate::ProcessingContext) -> Result<String, String> {
        let now = chrono::Utc::now();
        let id = H::next_msg_id(&mut ctx.uidgen_client.clone(), "CAMT029").await.map_err(|e| e.to_string())?;
        let body = m::ResolutionOfInvestigationV13 { assgnmt: m::CaseAssignment6 { id: id.clone(), cre_dt_tm: now, ..Default::default() }, ..Default::default() };
        let doc_xml = quick_xml::se::to_string(&m::Document::new(body)).map_err(|e| format!("{e}"))?;
        self.wrap(&doc_xml, "camt.029.001.13", &id, &ctx.config.own_bic, "RECEIVERBIC")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resolve_kind_prefers_per_tx_status_over_investigation_conf() {
        // Per-tx RJCR must win even if the investigation-level conf code
        // would otherwise say "accepted".
        assert_eq!(
            resolve_kind(Some(&m::CancellationIndividualStatus1Code::Rjcr), Some("CNCL")),
            JourneyEventKind::InvestigationResolutionRejected
        );
        assert_eq!(
            resolve_kind(Some(&m::CancellationIndividualStatus1Code::Accr), None),
            JourneyEventKind::InvestigationResolutionAccepted
        );
        assert_eq!(
            resolve_kind(Some(&m::CancellationIndividualStatus1Code::Pdcr), None),
            JourneyEventKind::InvestigationResolutionPending
        );
    }

    #[test]
    fn resolve_kind_falls_back_to_investigation_conf_when_no_per_tx_status() {
        assert_eq!(resolve_kind(None, Some("CNCL")), JourneyEventKind::InvestigationResolutionAccepted);
        assert_eq!(resolve_kind(None, Some("MODI")), JourneyEventKind::InvestigationResolutionAccepted);
        assert_eq!(resolve_kind(None, Some("IPAY")), JourneyEventKind::InvestigationResolutionAccepted);
        assert_eq!(resolve_kind(None, Some("ACDA")), JourneyEventKind::InvestigationResolutionAccepted);
        assert_eq!(resolve_kind(None, Some("RJCR")), JourneyEventKind::InvestigationResolutionRejected);
    }

    #[test]
    fn resolve_kind_defaults_to_pending_when_neither_signal_present() {
        assert_eq!(resolve_kind(None, None), JourneyEventKind::InvestigationResolutionPending);
        assert_eq!(resolve_kind(None, Some("SOMETHING_ELSE")), JourneyEventKind::InvestigationResolutionPending);
        // An UNKNOWN per-tx status (unrecognized wire value) also falls
        // through to the conf-code branch, not straight to Pending.
        assert_eq!(
            resolve_kind(Some(&m::CancellationIndividualStatus1Code::Unknown), Some("CNCL")),
            JourneyEventKind::InvestigationResolutionAccepted
        );
    }

    #[test]
    fn resolution_status_for_maps_each_kind() {
        assert_eq!(resolution_status_for(JourneyEventKind::InvestigationResolutionAccepted), "RESOLVED");
        assert_eq!(resolution_status_for(JourneyEventKind::InvestigationResolutionRejected), "REJECTED");
        assert_eq!(resolution_status_for(JourneyEventKind::InvestigationResolutionPending), "PENDING");
        // Any other kind (defensive - shouldn't occur in practice here) also
        // falls back to PENDING rather than panicking.
        assert_eq!(resolution_status_for(JourneyEventKind::CustomerPaymentInitiated), "PENDING");
    }

    #[test]
    fn tx_cxl_sts_str_maps_known_codes_and_labels_unknown_explicitly() {
        assert_eq!(tx_cxl_sts_str(&m::CancellationIndividualStatus1Code::Rjcr), "RJCR");
        assert_eq!(tx_cxl_sts_str(&m::CancellationIndividualStatus1Code::Accr), "ACCR");
        assert_eq!(tx_cxl_sts_str(&m::CancellationIndividualStatus1Code::Pdcr), "PDCR");
        assert_eq!(tx_cxl_sts_str(&m::CancellationIndividualStatus1Code::Unknown), "UNKNOWN");
    }
}
