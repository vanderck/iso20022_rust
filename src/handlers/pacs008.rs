use async_trait::async_trait;
use quick_xml::de::from_str;

use crate::mapper::Message;
use crate::messages::pacs00800113 as m;
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
        let body = &doc.fi_to_fi_cstmr_cdt_trf;
        let msg_type = "pacs.008.001.13";
        let msg_id = body.grp_hdr.msg_id.clone();

        let mut scope = TransactionScope::new(ctx.crud_client.clone());
        H::archive_message(scope.crud(), msg_type, "INBOUND", &msg_id, msg)
            .await.map_err(|e| e.to_string())?;

        let txs = batch::extract_pacs008_txs(body);
        let mut first_refs = None;
        for etx in txs {
            let refs = etx.refs.clone();
            if first_refs.is_none() { first_refs = Some(refs.clone()); }
            // pacs.008 is an initiating payment – create journey if needed.
            if H::resolve_journey_id(scope.crud(), &refs).await.map_err(|e| e.to_string())?.is_none() {
                H::create_journey_with_party(scope.crud(), &refs, msg_type, &msg_id, &etx.party)
                    .await.map_err(|e| e.to_string())?;
            }
            let event = etx.into_event(SupportedMessage::Pacs008, JourneyEventKind::FiPaymentSent);
            H::process_event(scope.crud(), msg_type, &msg_id, event)
                .await.map_err(|e| e.to_string())?;
        }

        // Flow: book → pacs.002 ACK outbound.
        let refs = first_refs.unwrap_or_default();
        let flow = FlowCoordinator::execute(
            scope.crud(), &ctx.db_pool, ctx.engine_client.clone(), &ctx.http_client, &ctx.config, SupportedMessage::Pacs008, &msg_id, &refs,
            None, None, counterparty_bic.unwrap_or("UNKNOWN"),
        ).await.map_err(|e| e.to_string())?;

        scope.commit().await.map_err(|e| e.to_string())?;
        Ok(format!("processed {} msg_id={} outbound={}", msg_type, msg_id, flow.outbound_messages.len()))
    }

    async fn generate(&self, ctx: &crate::ProcessingContext) -> Result<String, String> {
        // Read journey from CBS and build a real pacs.008.
        // Stub: caller must provide journey_id via ctx or param in production.
        let now = chrono::Utc::now();
        let msg_id = H::next_msg_id(&mut ctx.uidgen_client.clone(), "PACS008").await.map_err(|e| e.to_string())?;
        let body = m::FIToFICustomerCreditTransferV13 {
            grp_hdr: m::GroupHeader131 { msg_id: msg_id.clone(), cre_dt_tm: now, nb_of_txs: "1".into(), ..Default::default() },
            ..Default::default()
        };
        let doc_xml = quick_xml::se::to_string(&m::Document::new(body)).map_err(|e| format!("{e}"))?;
        self.wrap(&doc_xml, "pacs.008.001.13", &msg_id, &ctx.config.own_bic, "RECEIVERBIC")
    }
}
