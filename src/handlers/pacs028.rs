use async_trait::async_trait;
use quick_xml::de::from_str;
use crate::mapper::Message;
use crate::messages::pacs02800106 as m;
use crate::processor::{flow_coordinator::FlowCoordinator, handler_support::HandlerSupport as H,
    journey_event::{JourneyEvent, JourneyEventKind, JourneyRefs, SupportedMessage}, transaction_scope::TransactionScope};

pub struct Document;

/// Build the journey refs for a status-request from its first tx/group
/// entries (both are optional in the wire format - a request may carry
/// only group-level or only tx-level original references, or neither).
fn refs_from(tx: Option<&m::PaymentTransaction158>, grp: Option<&m::OriginalGroupInformation27>) -> JourneyRefs {
    JourneyRefs {
        original_msg_id: grp.map(|g| g.orgnl_msg_id.clone()),
        end_to_end_id: tx.and_then(|t| t.orgnl_end_to_end_id.clone()),
        tx_id: tx.and_then(|t| t.orgnl_tx_id.clone()),
        uetr: tx.and_then(|t| t.orgnl_uetr.clone()),
        clr_sys_ref: tx.and_then(|t| t.clr_sys_ref.clone()),
        case_id: None, assignment_id: None,
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
        let body = &doc.fi_to_fi_pmt_sts_req;
        let msg_type = "pacs.028.001.06";
        let msg_id = body.grp_hdr.msg_id.clone();
        let mut scope = TransactionScope::new(ctx.crud_client.clone());
        H::archive_message(scope.crud(), msg_type, "INBOUND", &msg_id, msg).await.map_err(|e| e.to_string())?;

        let refs = refs_from(body.tx_inf.first(), body.orgnl_grp_inf.first());
        let event = JourneyEvent { source_message: SupportedMessage::Pacs028,
            kind: JourneyEventKind::StatusRequestReceived, refs: refs.clone(),
            iso_status: None, reason_code: None,
            description: format!("Status request: msg_id={}", msg_id) };
        H::process_event(scope.crud(), msg_type, &msg_id, event).await.map_err(|e| e.to_string())?;

        // Flow: respond with pacs.002 containing current journey status.
        let flow = FlowCoordinator::execute(scope.crud(), &ctx.db_pool, ctx.engine_client.clone(), &ctx.http_client, &ctx.config, SupportedMessage::Pacs028, &msg_id, &refs,
            None, None, counterparty_bic.unwrap_or("UNKNOWN")).await.map_err(|e| e.to_string())?;
        scope.commit().await.map_err(|e| e.to_string())?;
        Ok(format!("processed {} msg_id={} outbound={}", msg_type, msg_id, flow.outbound_messages.len()))
    }

    async fn generate(&self, ctx: &crate::ProcessingContext) -> Result<String, String> {
        let now = chrono::Utc::now(); let msg_id = H::next_msg_id(&mut ctx.uidgen_client.clone(), "PACS028").await.map_err(|e| e.to_string())?;
        let body = m::FIToFIPaymentStatusRequestV06 { grp_hdr: m::GroupHeader109 { msg_id: msg_id.clone(), cre_dt_tm: now, ..Default::default() }, ..Default::default() };
        let doc_xml = quick_xml::se::to_string(&m::Document::new(body)).map_err(|e| format!("{e}"))?;
        self.wrap(&doc_xml, "pacs.028.001.06", &msg_id, &ctx.config.own_bic, "RECEIVERBIC")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn refs_from_maps_both_group_and_tx_level_references() {
        let grp = m::OriginalGroupInformation27 { orgnl_msg_id: "MSG1".into(), ..Default::default() };
        let tx = m::PaymentTransaction158 {
            orgnl_end_to_end_id: Some("E2E1".into()),
            orgnl_tx_id: Some("TX1".into()),
            orgnl_uetr: Some("UETR1".into()),
            clr_sys_ref: Some("CLR1".into()),
            ..Default::default()
        };
        let refs = refs_from(Some(&tx), Some(&grp));
        assert_eq!(refs.original_msg_id.as_deref(), Some("MSG1"));
        assert_eq!(refs.end_to_end_id.as_deref(), Some("E2E1"));
        assert_eq!(refs.tx_id.as_deref(), Some("TX1"));
        assert_eq!(refs.uetr.as_deref(), Some("UETR1"));
        assert_eq!(refs.clr_sys_ref.as_deref(), Some("CLR1"));
    }

    #[test]
    fn refs_from_yields_all_none_when_neither_tx_nor_group_present() {
        // A status-request with no per-tx or group breakdown at all - the
        // fields default to None rather than panicking on the missing data.
        let refs = refs_from(None, None);
        assert_eq!(refs.original_msg_id, None);
        assert_eq!(refs.end_to_end_id, None);
        assert_eq!(refs.tx_id, None);
        assert_eq!(refs.uetr, None);
        assert_eq!(refs.clr_sys_ref, None);
    }

    #[test]
    fn refs_from_handles_group_only_with_no_tx_breakdown() {
        let grp = m::OriginalGroupInformation27 { orgnl_msg_id: "MSG2".into(), ..Default::default() };
        let refs = refs_from(None, Some(&grp));
        assert_eq!(refs.original_msg_id.as_deref(), Some("MSG2"));
        assert_eq!(refs.end_to_end_id, None);
        assert_eq!(refs.tx_id, None);
    }
}
