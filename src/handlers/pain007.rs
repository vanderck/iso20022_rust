use async_trait::async_trait;
use quick_xml::de::from_str;
use crate::mapper::Message;
use crate::messages::pain00700112 as m;
use crate::processor::{batch, flow_coordinator::FlowCoordinator, handler_support::HandlerSupport as H,
    journey_event::{JourneyEventKind, SupportedMessage}, transaction_scope::TransactionScope};

pub struct Document;

#[async_trait]
impl Message for Document {
    async fn consume(
        &self,
        ctx: &crate::ProcessingContext,
        msg: &str,
        counterparty_bic: Option<&str>,
    ) -> Result<String, String> {
        let doc: m::Document = from_str(msg).map_err(|e| e.to_string())?;
        let body = &doc.cstmr_pmt_rvsl;
        let msg_type = "pain.007.001.12";
        let msg_id = body.grp_hdr.msg_id.clone();
        let mut scope = TransactionScope::new(ctx.crud_client.clone());
        H::archive_message(scope.crud(), msg_type, "INBOUND", &msg_id, msg).await.map_err(|e| e.to_string())?;

        let txs = batch::extract_pain007_txs(body);
        let mut first_refs = None;
        let mut last_reason = None;
        for etx in txs {
            if first_refs.is_none() { first_refs = Some(etx.refs.clone()); }
            last_reason = etx.reason_code.clone();
            let refs = etx.refs.clone();
            let jid = H::resolve_journey_id(scope.crud(), &refs).await.map_err(|e| e.to_string())?;
            H::open_investigation(scope.crud(), jid.as_deref(), &refs, msg_type,
                last_reason.as_deref(), &etx.description).await.map_err(|e| e.to_string())?;
            let event = etx.into_event(SupportedMessage::Pain007, JourneyEventKind::CustomerReversalRequested);
            H::process_event(scope.crud(), msg_type, &msg_id, event).await.map_err(|e| e.to_string())?;
        }

        let refs = first_refs.unwrap_or_default();
        let flow = FlowCoordinator::execute(scope.crud(), &ctx.db_pool, ctx.engine_client.clone(), &ctx.http_client, &ctx.config, SupportedMessage::Pain007, &msg_id, &refs,
            None, last_reason.as_deref(), counterparty_bic.unwrap_or("UNKNOWN")).await.map_err(|e| e.to_string())?;
        scope.commit().await.map_err(|e| e.to_string())?;
        Ok(format!("processed {} msg_id={} outbound={}", msg_type, msg_id, flow.outbound_messages.len()))
    }

    async fn generate(&self, ctx: &crate::ProcessingContext) -> Result<String, String> {
        let now = chrono::Utc::now();
        let msg_id = H::next_msg_id(&mut ctx.uidgen_client.clone(), "PAIN007").await.map_err(|e| e.to_string())?;
        let body = m::CustomerPaymentReversalV12 {
            grp_hdr: m::GroupHeader124 { msg_id: msg_id.clone(), cre_dt_tm: now, ..Default::default() },
            orgnl_grp_inf: m::OriginalGroupHeader20 { orgnl_msg_id: "ORIG".into(), orgnl_msg_nm_id: "pain.001.001.12".into(), ..Default::default() },
            ..Default::default()
        };
        let doc_xml = quick_xml::se::to_string(&m::Document::new(body)).map_err(|e| format!("{e}"))?;
        self.wrap(&doc_xml, "pain.007.001.12", &msg_id, &ctx.config.own_bic, "RECEIVERBIC")
    }
}
