use async_trait::async_trait;
use quick_xml::de::from_str;

use crate::mapper::Message;
use crate::messages::pain00100112 as m;
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
        let body = &doc.cstmr_cdt_trf_initn;
        let msg_type = "pain.001.001.12";
        let msg_id = body.grp_hdr.msg_id.clone();

        let mut scope = TransactionScope::new(ctx.crud_client.clone());

        H::archive_message(scope.crud(), msg_type, "INBOUND", &msg_id, msg)
            .await.map_err(|e| e.to_string())?;

        let txs = batch::extract_pain001_txs(body);
        let first_refs = txs.first().map(|t| t.refs.clone()).unwrap_or_default();
        let mut last_jid = None;
        let mut last_next = None;

        for etx in txs {
            let refs = etx.refs.clone();
            let party = etx.party.clone();
            // pain.001: always create a journey per tx if none exists.
            let jid = match H::resolve_journey_id(scope.crud(), &refs).await.map_err(|e| e.to_string())? {
                Some(id) => id,
                None => H::create_journey_with_party(scope.crud(), &refs, msg_type, &msg_id, &party)
                    .await.map_err(|e| e.to_string())?,
            };
            let event = etx.into_event(SupportedMessage::Pain001, JourneyEventKind::CustomerPaymentInitiated);
            let next = H::process_event(scope.crud(), msg_type, &msg_id, event)
                .await.map_err(|e| e.to_string())?;
            last_jid = Some(jid);
            last_next = next;
        }

        // Flow: book in CBS, generate pacs.008 outbound.
        let flow = FlowCoordinator::execute(
            scope.crud(), &ctx.db_pool, ctx.engine_client.clone(), &ctx.http_client, &ctx.config, SupportedMessage::Pain001, &msg_id, &first_refs,
            None, None, counterparty_bic.unwrap_or("UNKNOWN"),
        ).await.map_err(|e| e.to_string())?;

        scope.commit().await.map_err(|e| e.to_string())?;

        Ok(format!("processed {} msg_id={} journey={:?} next={:?} outbound={}",
            msg_type, msg_id, last_jid, last_next, flow.outbound_messages.len()))
    }

    async fn generate(&self, ctx: &crate::ProcessingContext) -> Result<String, String> {
        // In production: read pending outbound payment from CBS via OutboundBuilder.
        let now = chrono::Utc::now();
        let msg_id = H::next_msg_id(&mut ctx.uidgen_client.clone(), "PAIN001").await.map_err(|e| e.to_string())?;
        let body = m::CustomerCreditTransferInitiationV12 {
            grp_hdr: m::GroupHeader114 { msg_id: msg_id.clone(), cre_dt_tm: now, nb_of_txs: "1".into(), ..Default::default() },
            ..Default::default()
        };
        let doc_xml = quick_xml::se::to_string(&m::Document::new(body)).map_err(|e| format!("{e}"))?;
        self.wrap(&doc_xml, "pain.001.001.12", &msg_id, &ctx.config.own_bic, "RECEIVERBIC")
    }
}
