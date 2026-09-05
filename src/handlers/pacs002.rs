use async_trait::async_trait;
use quick_xml::de::from_str;

use crate::mapper::Message;
use crate::messages::pacs00200115 as m;
use crate::processor::batch;
use crate::processor::flow_coordinator::FlowCoordinator;
use crate::processor::handler_support::HandlerSupport as H;
use crate::processor::journey_event::{JourneyEventKind, SupportedMessage};
use crate::processor::transaction_scope::TransactionScope;

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
        let body = &doc.fi_to_fi_pmt_sts_rpt;
        let msg_type = "pacs.002.001.15";
        let msg_id = body.grp_hdr.msg_id.clone();

        let mut scope = TransactionScope::new(ctx.crud_client.clone());
        H::archive_message(scope.crud(), msg_type, "INBOUND", &msg_id, msg)
            .await.map_err(|e| e.to_string())?;

        let txs = batch::extract_pacs002_txs(body);
        let mut last_status = None;
        let mut last_reason = None;
        let mut first_refs = None;

        for etx in txs {
            let refs = etx.refs.clone();
            if first_refs.is_none() { first_refs = Some(refs.clone()); }
            last_status = etx.iso_status.clone();
            last_reason = etx.reason_code.clone();
            let event = etx.into_event(SupportedMessage::Pacs002, JourneyEventKind::FiStatusReceived);
            H::process_event(scope.crud(), msg_type, &msg_id, event)
                .await.map_err(|e| e.to_string())?;
        }

        // Flow: propagate terminal status to customer.
        let refs = first_refs.unwrap_or_default();
        let flow = FlowCoordinator::execute(
            scope.crud(), &ctx.db_pool, ctx.engine_client.clone(), &ctx.http_client, &ctx.config, SupportedMessage::Pacs002, &msg_id, &refs,
            last_status.as_deref(), last_reason.as_deref(), counterparty_bic.unwrap_or("UNKNOWN"),
        ).await.map_err(|e| e.to_string())?;

        scope.commit().await.map_err(|e| e.to_string())?;
        Ok(format!("processed {} msg_id={} outbound={}", msg_type, msg_id, flow.outbound_messages.len()))
    }

    async fn generate(&self, ctx: &crate::ProcessingContext) -> Result<String, String> {
        let now = chrono::Utc::now();
        let msg_id = H::next_msg_id(&mut ctx.uidgen_client.clone(), "PACS002").await.map_err(|e| e.to_string())?;
        let body = m::FIToFIPaymentStatusReportV15 {
            grp_hdr: m::GroupHeader120 { msg_id: msg_id.clone(), cre_dt_tm: now, ..Default::default() },
            ..Default::default()
        };
        let doc_xml = quick_xml::se::to_string(&m::Document::new(body)).map_err(|e| format!("{e}"))?;
        self.wrap(&doc_xml, "pacs.002.001.15", &msg_id, &ctx.config.own_bic, "RECEIVERBIC")
    }
}
