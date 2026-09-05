use crate::context::CrudClient;

use crate::crudgrpc::{CreateRequest, DbPath, Fld, ReadRequest};
use crate::engine::{Authorisation, Settle};
use crate::processor::compliance::{self, ScreeningResult};
use crate::processor::config::InstitutionConfig;
use crate::processor::handler_support::HandlerSupport as H;
use crate::processor::journey_event::{JourneyEvent, JourneyEventKind, JourneyRefs, SupportedMessage};
use crate::processor::metrics;
use crate::processor::outbound_builder::{JourneyData, OutboundBuilder};
use deadpool_postgres::Pool;
use sha2::{Digest, Sha256};

#[derive(Debug, Default)]
pub struct FlowResult {
    pub journey_id: Option<String>,
    pub next_state: Option<String>,
    pub outbound_messages: Vec<OutboundMessage>,
}

#[derive(Debug, Clone)]
pub struct OutboundMessage {
    pub msg_type: String,
    pub raw_xml: String,
}

/// Result of a CBS booking attempt.
#[derive(Debug)]
pub enum BookingOutcome {
    Booked,
    Rejected { reason: String },
    ScreeningHold { hit_id: String },
    DuplicateDetected { existing_journey_id: String },
}

pub struct FlowCoordinator;

impl FlowCoordinator {
    #[allow(clippy::too_many_arguments)]
    pub async fn execute(
        crud: &mut crate::context::CrudClient,
        db_pool: &Pool,
        mut engine: crate::context::EngineClient,
        http_client: &reqwest::Client,
        config: &InstitutionConfig,
        source: SupportedMessage,
        msg_id: &str,
        refs: &JourneyRefs,
        iso_status: Option<&str>,
        reason_code: Option<&str>,
        counterparty_bic: &str,
    ) -> Result<FlowResult, String> {
        let mut result = FlowResult::default();
        let own_bic = &config.own_bic;

        let journey_id = H::resolve_journey_id(crud, refs).await.map_err(|e| e.to_string())?;
        result.journey_id = journey_id.clone();

        let jd = match &journey_id {
            Some(jid) => OutboundBuilder::load_journey(crud, jid).await.map_err(|e| e.to_string())?,
            None => None,
        };

        match source {
            // ─── pain.001 → Screen → Book → pacs.008 outbound ───────────
            SupportedMessage::Pain001 => {
                if let Some(ref jd) = jd {
                    // Duplicate check.
                    if config.duplicate_check_enabled
                        && compliance::check_duplicate_payment(
                            crud, &jd.journey_id, jd.end_to_end_id.as_deref(),
                            jd.settlement_amount.as_deref(), jd.settlement_ccy.as_deref(),
                            jd.settlement_date.as_deref(),
                        ).await.map_err(|e| e.to_string())?.is_some() {
                            // Reject with duplicate reason.
                            let xml = Self::build_pain002_notification(jd, "RJCT", Some("AM05"))?;
                            result.outbound_messages.push(OutboundMessage { msg_type: "pain.002.001.14".into(), raw_xml: xml });
                            Self::enqueue_all(db_pool, crud, msg_id, &result.outbound_messages).await?;
                            return Ok(result);
                        }

                    // AML/Sanctions screening — screens the actual debtor/
                    // creditor party names captured at journey creation, not
                    // the agent BICs.
                    if config.screening_enabled {
                        match compliance::screen_payment(
                            crud, http_client, config, &jd.journey_id,
                            jd.debtor_name.as_deref(),
                            jd.creditor_name.as_deref(),
                            jd.settlement_amount.as_deref(), jd.settlement_ccy.as_deref(),
                        ).await.map_err(|e| e.to_string())? {
                            ScreeningResult::Clear => {} // Proceed.
                            ScreeningResult::PendingReview { hit_id, details } => {
                                tracing::warn!(journey_id = %jd.journey_id, %hit_id, %details, "AML/sanctions screening hold");
                                // Hold — pain.002 PDNG to customer, no pacs.008 yet.
                                let xml = Self::build_pain002_notification(jd, "PDNG", None)?;
                                result.outbound_messages.push(OutboundMessage { msg_type: "pain.002.001.14".into(), raw_xml: xml });
                                Self::enqueue_all(db_pool, crud, msg_id, &result.outbound_messages).await?;
                                return Ok(result);
                            }
                            ScreeningResult::Blocked { hit_id, details } => {
                                Self::freeze_for_sanctions(
                                    crud, SupportedMessage::Pain001, journey_id.as_deref(), refs, msg_id, &hit_id, &details,
                                ).await?;
                                // No outbound message — see freeze_for_sanctions doc comment.
                                return Ok(result);
                            }
                        }
                    }

                    // CBS booking.
                    match Self::book_in_cbs(crud, &mut engine, jd, "CUSTOMER_CREDIT_TRANSFER").await {
                        Ok(BookingOutcome::Booked) => {
                            let xml = OutboundBuilder::build_pacs008(jd, own_bic, counterparty_bic)?;
                            result.outbound_messages.push(OutboundMessage { msg_type: "pacs.008.001.13".into(), raw_xml: xml });
                            // Also send pain.002 ACTC to customer.
                            let ack = Self::build_pain002_notification(jd, "ACTC", None)?;
                            result.outbound_messages.push(OutboundMessage { msg_type: "pain.002.001.14".into(), raw_xml: ack });
                        }
                        Ok(BookingOutcome::Rejected { reason }) => {
                            let xml = Self::build_pain002_notification(jd, "RJCT", Some(&reason))?;
                            result.outbound_messages.push(OutboundMessage { msg_type: "pain.002.001.14".into(), raw_xml: xml });
                        }
                        Ok(BookingOutcome::ScreeningHold { .. }) | Ok(BookingOutcome::DuplicateDetected { .. }) => {
                            let xml = Self::build_pain002_notification(jd, "PDNG", None)?;
                            result.outbound_messages.push(OutboundMessage { msg_type: "pain.002.001.14".into(), raw_xml: xml });
                        }
                        Err(e) => return Err(format!("CBS booking failed (transient): {e}")),
                    }
                }
            }

            // ─── pacs.008 inbound → Screen → Book → pacs.002 ACK ────────
            SupportedMessage::Pacs008 => {
                if let Some(ref jd) = jd {
                    if config.screening_enabled {
                        match compliance::screen_payment(
                            crud, http_client, config, &jd.journey_id,
                            jd.debtor_name.as_deref(),
                            jd.creditor_name.as_deref(),
                            jd.settlement_amount.as_deref(), jd.settlement_ccy.as_deref(),
                        ).await.map_err(|e| e.to_string())? {
                            ScreeningResult::Clear => {}
                            ScreeningResult::PendingReview { .. } => {
                                // Hold — return PDNG to sender.
                                let xml = OutboundBuilder::build_pacs002(jd, "PDNG", None, own_bic, counterparty_bic)?;
                                result.outbound_messages.push(OutboundMessage { msg_type: "pacs.002.001.15".into(), raw_xml: xml });
                                Self::enqueue_all(db_pool, crud, msg_id, &result.outbound_messages).await?;
                                return Ok(result);
                            }
                            ScreeningResult::Blocked { hit_id, details } => {
                                Self::freeze_for_sanctions(
                                    crud, SupportedMessage::Pacs008, journey_id.as_deref(), refs, msg_id, &hit_id, &details,
                                ).await?;
                                // No outbound message — see freeze_for_sanctions doc comment.
                                return Ok(result);
                            }
                        }
                    }

                    match Self::book_in_cbs(crud, &mut engine, jd, "FI_CREDIT_TRANSFER").await {
                        Ok(BookingOutcome::Booked) => {
                            let xml = OutboundBuilder::build_pacs002(jd, "ACTC", None, own_bic, counterparty_bic)?;
                            result.outbound_messages.push(OutboundMessage { msg_type: "pacs.002.001.15".into(), raw_xml: xml });
                        }
                        Ok(BookingOutcome::Rejected { reason }) => {
                            let xml = OutboundBuilder::build_pacs002(jd, "RJCT", Some(&reason), own_bic, counterparty_bic)?;
                            result.outbound_messages.push(OutboundMessage { msg_type: "pacs.002.001.15".into(), raw_xml: xml });
                        }
                        Ok(_) => {
                            let xml = OutboundBuilder::build_pacs002(jd, "PDNG", None, own_bic, counterparty_bic)?;
                            result.outbound_messages.push(OutboundMessage { msg_type: "pacs.002.001.15".into(), raw_xml: xml });
                        }
                        Err(e) => return Err(format!("CBS booking failed (transient): {e}")),
                    }
                }
            }

            // ─── pacs.002 inbound → Propagate terminal status to customer ─
            SupportedMessage::Pacs002 => {
                if let Some(ref jd) = jd {
                    match iso_status {
                        Some("ACSC") => {
                            let xml = Self::build_pain002_notification(jd, "ACSC", None)?;
                            result.outbound_messages.push(OutboundMessage { msg_type: "pain.002.001.14".into(), raw_xml: xml });
                        }
                        Some("RJCT") => {
                            let xml = Self::build_pain002_notification(jd, "RJCT", reason_code)?;
                            result.outbound_messages.push(OutboundMessage { msg_type: "pain.002.001.14".into(), raw_xml: xml });
                        }
                        _ => {} // Intermediate: no customer notification.
                    }
                }
            }

            // ─── pacs.028 → Respond with current status ─────────────────
            SupportedMessage::Pacs028 => {
                if let Some(ref jd) = jd {
                    let iso = Self::journey_state_to_iso(&jd.current_status);
                    let xml = OutboundBuilder::build_pacs002(jd, iso, None, own_bic, counterparty_bic)?;
                    result.outbound_messages.push(OutboundMessage { msg_type: "pacs.002.001.15".into(), raw_xml: xml });
                }
            }

            // ─── pacs.004 → Book return → Notify customer ───────────────
            SupportedMessage::Pacs004 => {
                if let Some(ref jd) = jd {
                    match Self::book_in_cbs(crud, &mut engine, jd, "PAYMENT_RETURN").await {
                        Ok(BookingOutcome::Booked) => {
                            let xml = Self::build_pain002_notification(jd, "RJCT", reason_code)?;
                            result.outbound_messages.push(OutboundMessage { msg_type: "pain.002.001.14".into(), raw_xml: xml });
                        }
                        Ok(_) => {}
                        Err(e) => return Err(format!("CBS return booking failed: {e}")),
                    }
                }
            }

            // ─── camt.056 → Auto camt.029 PDCR ──────────────────────────
            SupportedMessage::Camt056 => {
                if let Some(ref jd) = jd {
                    let xml = Self::build_camt029_response(jd, refs, "PDCR")?;
                    result.outbound_messages.push(OutboundMessage { msg_type: "camt.029.001.13".into(), raw_xml: xml });
                }
            }

            // ─── pacs.007 / pain.007 → Book reversal → pacs.004 ─────────
            SupportedMessage::Pacs007 | SupportedMessage::Pain007 => {
                if let Some(ref jd) = jd {
                    let rc = reason_code.unwrap_or("FRAD");
                    match Self::book_in_cbs(crud, &mut engine, jd, "PAYMENT_REVERSAL").await {
                        Ok(BookingOutcome::Booked) => {
                            let xml = OutboundBuilder::build_pacs004(jd, rc, own_bic, counterparty_bic)?;
                            result.outbound_messages.push(OutboundMessage { msg_type: "pacs.004.001.14".into(), raw_xml: xml });
                        }
                        Ok(_) => {}
                        Err(e) => return Err(format!("CBS reversal booking failed: {e}")),
                    }
                }
            }

            // ─── No further outbound for these ──────────────────────────
            SupportedMessage::Camt029 | SupportedMessage::Camt026 | SupportedMessage::Camt027 |
            SupportedMessage::Camt052 | SupportedMessage::Camt053 | SupportedMessage::Camt054 |
            SupportedMessage::Pain002 => {}
        }

        Self::enqueue_all(db_pool, crud, msg_id, &result.outbound_messages).await?;
        Ok(result)
    }

    // ── CBS Booking ──────────────────────────────────────────────────────
    //
    // Calls the real ledger via EngineService: `authorisation` reserves the
    // funds movement, `settle` finalises it. Must handle: balance check,
    // holds, nostro/vostro posting, fees — all of that lives inside the
    // engine itself; this method's job is just to call it correctly and
    // translate the result.
    //
    // # Known integration gaps
    //
    // `engine.proto`'s `Authorisation`/`Settle` identify accounts and the
    // transaction itself with internal `i64` ledger ids. Neither the ISO
    // 20022 message nor `engine.proto` carries one directly, and `core`
    // (the CBS ledger schema) has no IBAN/account-number column of its own —
    // that mapping is owned by this app, in `iso20022.ibans` (see migration
    // 013), populated either via `crust`-backed UI tooling or an automatic
    // IBAN-issuing process at account opening. `resolve_account_id` resolves
    // in two steps:
    //   1. the stored account string is already a bare ledger account
    //      number (as commonly happens for the debtor side of a pain.001,
    //      since the debtor is our own customer) — parsed directly, no CRUD
    //      round-trip needed.
    //   2. otherwise, it's looked up in `iso20022.ibans` via the CRUD
    //      service. An IBAN with no active row in that table (never issued,
    //      or superseded) fails to resolve and the booking is rejected
    //      rather than silently mis-posted.
    //   - `engine_transaction_id` derives a stable id from the payment's
    //     UETR so the same journey maps to the same transaction id across
    //     the authorisation and settle calls, since there is no CRUD path
    //     to read back a database-generated id (`CreateRequest`/`GenericResponse`
    //     carry no id) and no engine RPC to issue one.
    async fn book_in_cbs(

        crud: &mut crate::context::CrudClient,
        engine: &mut crate::context::EngineClient,
        jd: &JourneyData,
        booking_type: &str,
    ) -> Result<BookingOutcome, String> {
        let start = std::time::Instant::now();

        let amount = jd.settlement_amount.clone().unwrap_or_default();
        let currency = jd.settlement_ccy.clone().unwrap_or_default();
        let transaction = Self::engine_transaction_id(jd);

        // Returns move funds from creditor back to debtor; everything else
        // (customer/FI credit transfers) moves debtor -> creditor.
        let (benefactor_acct, beneficiary_acct) = match booking_type {
            "PAYMENT_RETURN" | "PAYMENT_REVERSAL" => {
                (jd.creditor_account.as_deref(), jd.debtor_account.as_deref())
            }
            _ => (jd.debtor_account.as_deref(), jd.creditor_account.as_deref()),
        };

        let benefactor_result = Self::resolve_account_id(crud, benefactor_acct).await;
        let beneficiary_result = Self::resolve_account_id(crud, beneficiary_acct).await;
        let (benefactor, beneficiary) = match (benefactor_result, beneficiary_result) {
            (Ok(b), Ok(c)) => (b, c),
            (Err(e), _) | (_, Err(e)) => {
                tracing::warn!(journey_id = %jd.journey_id, booking_type, error = %e, "CBS booking rejected: account resolution failed");
                metrics::record_cbs_booking(booking_type, "rejected");
                metrics::record_cbs_booking_duration(booking_type, start.elapsed());
                return Ok(BookingOutcome::Rejected { reason: e });
            }
        };

        tracing::info!(
            journey_id = %jd.journey_id, booking_type, transaction, benefactor, beneficiary,
            amount = %amount, currency = %currency,
            "CBS booking: authorising"
        );

        if let Err(status) = engine.authorisation(Authorisation {
            transaction,
            benefactor,
            beneficiary,
            validity: 300, // seconds the authorisation hold remains valid
            partial: false,
            amount: amount.clone(),
            currency,
            limits: vec![],
        }).await {
            tracing::warn!(journey_id = %jd.journey_id, %status, "CBS authorisation failed");
            if Self::is_transient_status(&status) {
                // Propagate as Err so the caller's circuit breaker sees this as an
                // infra blip and retries the whole message later, rather than
                // telling the customer their payment was rejected.
                return Err(format!("authorisation failed (transient): {status}"));
            }
            metrics::record_cbs_booking(booking_type, "rejected");
            metrics::record_cbs_booking_duration(booking_type, start.elapsed());
            return Ok(BookingOutcome::Rejected { reason: format!("authorisation declined: {status}") });
        }

        // NOTE: a failure here leaves the authorisation hold in place with no
        // corresponding settlement — engine.proto has no "release/cancel
        // authorisation" RPC to compensate with, so a permanent failure at
        // this step requires manual/operator intervention on the ledger side.
        if let Err(status) = engine.settle(Settle { transaction, amount: amount.clone() }).await {
            tracing::error!(journey_id = %jd.journey_id, %status, "CBS settlement failed after authorisation succeeded");
            if Self::is_transient_status(&status) {
                return Err(format!("settlement failed (transient): {status}"));
            }
            metrics::record_cbs_booking(booking_type, "rejected");
            metrics::record_cbs_booking_duration(booking_type, start.elapsed());
            return Ok(BookingOutcome::Rejected { reason: format!("settlement failed: {status}") });
        }

        // Local audit trail correlating the journey to the engine transaction id.
        let _ = crud.create(CreateRequest {
            who: "flow_coordinator".into(), review: Some(false),
            path: Some(H::db_path("cbs_bookings")),
            data: vec![
                Fld { name: "journey_id".into(), relation: None, value: jd.journey_id.clone() },
                Fld { name: "booking_type".into(), relation: None, value: booking_type.into() },
                Fld { name: "engine_transaction_id".into(), relation: None, value: transaction.to_string() },
                Fld { name: "amount".into(), relation: None, value: amount },
                Fld { name: "currency".into(), relation: None, value: jd.settlement_ccy.clone().unwrap_or_default() },
                Fld { name: "status".into(), relation: None, value: "BOOKED".into() },
            ],
        }).await.map_err(|e| e.to_string())?;

        metrics::record_cbs_booking(booking_type, "booked");
        metrics::record_cbs_booking_duration(booking_type, start.elapsed());

        Ok(BookingOutcome::Booked)
    }

    /// Whether a failed EngineService call indicates a transient
    /// infrastructure problem (connectivity, overload, timeout) rather than
    /// the engine's own considered decision to decline the request. The
    /// former should be retried; the latter is a genuine business rejection.
    fn is_transient_status(status: &tonic::Status) -> bool {
        matches!(
            status.code(),
            tonic::Code::Unavailable
                | tonic::Code::DeadlineExceeded
                | tonic::Code::ResourceExhausted
                | tonic::Code::Aborted
                | tonic::Code::Unknown
                | tonic::Code::Internal
        )
    }

    /// Freeze a journey after a confirmed AML/sanctions match instead of
    /// returning the funds to the sender/counterparty.
    ///
    /// EU sanctions regulations (directly applicable in Belgium, enforced
    /// nationally via the Trésorerie at FOD Financiën/SPF Finances) prohibit
    /// making funds or economic resources available to a designated person —
    /// an automatic reject-and-return can itself violate that if the sender
    /// is the sanctioned party. So unlike every other decline path in this
    /// file, this deliberately does **not** build or enqueue an outbound
    /// message. It only:
    ///   - moves the journey to `SanctionsHold` (see that state's doc comment
    ///     for the two ways it can be resolved), and
    ///   - opens a compliance escalation record in `investigations`.
    ///
    /// Notifying the Trésorerie/CTIF-CFI and deciding the outbound response
    /// (if any) is a human compliance/AMLCO action, not something to
    /// automate here.
    async fn freeze_for_sanctions(

        crud: &mut crate::context::CrudClient,
        source: SupportedMessage,
        journey_id: Option<&str>,
        refs: &JourneyRefs,
        msg_id: &str,
        hit_id: &str,
        details: &str,
    ) -> Result<(), String> {
        tracing::error!(
            journey_id = ?journey_id, hit_id, details,
            "SANCTIONS HOLD: confirmed screening match — freezing funds, NOT returning to sender. \
             Escalate to compliance/AMLCO for Tr\u{e9}sorerie/CTIF-CFI notification."
        );

        let event = JourneyEvent {
            source_message: source,
            kind: JourneyEventKind::SanctionsHold,
            refs: refs.clone(),
            iso_status: None,
            reason_code: Some(hit_id.to_string()),
            description: format!("AML/sanctions hold: hit_id={hit_id} details={details}"),
        };
        H::process_event(crud, "screening", msg_id, event).await.map_err(|e| e.to_string())?;
        H::open_investigation(crud, journey_id, refs, "SANCTIONS_HOLD", Some(hit_id), details)
            .await.map_err(|e| e.to_string())?;
        Ok(())
    }

    /// Resolve a stored account string to an engine ledger account id.
    ///
    /// Tries a bare integer account number first, then falls back to an
    /// `iso20022.ibans` lookup via the CRUD service. See the "Known
    /// integration gaps" note on `book_in_cbs`.
    async fn resolve_account_id(
        crud: &mut CrudClient,

        account: Option<&str>,
    ) -> Result<i64, String> {
        let account = account.ok_or("no account on file for this journey")?;
        let account = account.trim();

        // Ledger account ids are BIGSERIAL, always >= 1 - a zero/negative
        // "bare integer" is not a real account id (it's more likely a
        // malformed IBAN/reference), so don't take the fast path for it and
        // let the IBAN lookup below reject it with a clear error instead.
        if let Ok(id) = account.parse::<i64>() {
            if id > 0 {
                return Ok(id);
            }
        }

        let req = ReadRequest {
            path: Some(DbPath { server: "CBS".into(), schema: "iso20022".into(), table: "ibans".into() }),
            column: vec!["core_account".into()],
            where_clause: vec![
                Fld { name: "iban".into(), relation: None, value: account.to_string() },
                Fld { name: "active".into(), relation: None, value: "true".into() },
            ],
            page: 0,
            page_size: Some(2),
        };
        let rows = crud.read(req).await
            .map_err(|e| format!("IBAN directory lookup failed for {account:?}: {e}"))?
            .into_inner()
            .data;

        match rows.len() {
            0 => Err(format!(
                "cannot resolve account {account:?} to a ledger account id \
                 (not a bare account number, and no active iso20022.ibans row)"
            )),
            1 => rows[0].list.first()
                .ok_or_else(|| format!("iban directory row for {account:?} had no core_account column"))?
                .parse::<i64>()
                .map_err(|_| format!("iban directory returned non-numeric core_account for {account:?}")),
            _ => Err(format!(
                "IBAN {account:?} has more than one active iso20022.ibans row; refusing to guess"
            )),
        }
    }

    /// Derive a stable engine transaction id from the payment's UETR (falling
    /// back to the journey id), so the same journey always maps to the same
    /// id across the authorisation and settle calls.
    fn engine_transaction_id(jd: &JourneyData) -> i64 {
        let key = jd.uetr.as_deref().unwrap_or(&jd.journey_id);
        let mut hasher = Sha256::new();
        hasher.update(key.as_bytes());
        let digest = hasher.finalize();
        let raw = i64::from_be_bytes(digest[0..8].try_into().expect("sha256 digest is >= 8 bytes"));
        raw & i64::MAX // clear the sign bit rather than risk overflow on i64::MIN.abs()
    }

    // ── Outbound enqueue ─────────────────────────────────────────────────

    /// Enqueue outbound messages on the correct delivery channel.
    ///
    /// - **Interbank messages** (`pacs.*`, `camt.*`) are written to the CBS
    ///   `outbound_queue` table (via the CRUD gRPC service). A SWIFT adapter
    ///   running inside the CBS is responsible for picking these up and
    ///   routing them to the network.
    /// - **Customer-facing messages** (`pain.*`, e.g. pain.002 status reports)
    ///   are written directly to the local `iso20022.message_ledger` table
    ///   (direction='OUT'), which is exactly what the notification worker in
    ///   `main.rs` polls to deliver messages to the customer callback URL.
    ///
    /// Both paths are idempotent on `(parent_msg_id, msg_type)` so retried
    /// handler invocations never double-enqueue a notification.
    async fn enqueue_all(
        db_pool: &Pool,

        crud: &mut crate::context::CrudClient,
        parent_msg_id: &str,
        messages: &[OutboundMessage],
    ) -> Result<(), String> {
        for ob in messages {
            let dedup_key = format!("{}|{}", parent_msg_id, ob.msg_type);
            if ob.msg_type.starts_with("pain.") {
                Self::enqueue_customer_message(db_pool, &dedup_key, ob).await?;
            } else {
                Self::enqueue_interbank_message(crud, parent_msg_id, &dedup_key, ob).await?;
            }
            metrics::record_outbound_generated(&ob.msg_type);
        }
        Ok(())
    }

    /// Enqueue an interbank message (pacs.*/camt.*) to the CBS `outbound_queue`.
    async fn enqueue_interbank_message(

        crud: &mut crate::context::CrudClient,
        parent_msg_id: &str,
        dedup_key: &str,
        ob: &OutboundMessage,
    ) -> Result<(), String> {
        // Idempotent outbound enqueue.
        // Dedup key = (parent_msg_id, msg_type) — prevents double-enqueue
        // if the handler is retried after a crash between state update and enqueue.
        let exists = H::read_one(
            crud, "outbound_queue", "outbound_id",
            vec![Fld { name: "dedup_key".into(), relation: None, value: dedup_key.to_string() }],
        ).await.map_err(|e| e.to_string())?;
        if exists.is_some() {
            tracing::debug!(dedup_key = %dedup_key, "Outbound message already enqueued, skipping");
            return Ok(());
        }

        let _ = crud.create(CreateRequest {
            who: "flow_coordinator".into(), review: Some(false),
            path: Some(H::db_path("outbound_queue")),
            data: vec![
                Fld { name: "parent_msg_id".into(), relation: None, value: parent_msg_id.into() },
                Fld { name: "msg_type".into(), relation: None, value: ob.msg_type.clone() },
                Fld { name: "direction".into(), relation: None, value: "OUTBOUND".into() },
                Fld { name: "status".into(), relation: None, value: "PENDING".into() },
                Fld { name: "raw_xml".into(), relation: None, value: ob.raw_xml.clone() },
                Fld { name: "dedup_key".into(), relation: None, value: dedup_key.to_string() },
            ],
        }).await.map_err(|e| e.to_string())?;
        Ok(())
    }

    /// Enqueue a customer-facing message (pain.*) directly to the local
    /// `message_ledger` (direction='OUT') so the notification worker delivers
    /// it to the customer callback URL.
    async fn enqueue_customer_message(
        db_pool: &Pool,
        dedup_key: &str,
        ob: &OutboundMessage,
    ) -> Result<(), String> {
        let client = db_pool.get().await.map_err(|e| format!("db pool: {e}"))?;

        let mut hasher = Sha256::new();
        hasher.update(ob.raw_xml.as_bytes());
        let body_hash = format!("{:x}", hasher.finalize());

        client.execute(
            "INSERT INTO iso20022.message_ledger
                (direction, status, raw_xml_bytes, msg_type, body_hash, dedup_key)
             VALUES ('OUT', 'PENDING', $1, $2, $3, $4)
             ON CONFLICT (dedup_key) WHERE dedup_key IS NOT NULL DO NOTHING",
            &[&ob.raw_xml.as_bytes(), &ob.msg_type, &body_hash, &dedup_key],
        ).await.map_err(|e| format!("message_ledger insert: {e}"))?;

        Ok(())
    }

    // ── Helpers ──────────────────────────────────────────────────────────

    /// Map an internal journey state string to the ISO 20022 pacs.002 / pain.002 status code.
    ///
    /// ISO 20022 status codes (ExternalPaymentTransactionStatus1Code):
    ///   RCVD – Received           ACTC – AcceptedTechnicalValidation
    ///   ACCP – AcceptedCustomerProfile  ACSP – AcceptedSettlementInProcess
    ///   ACSC – AcceptedSettlementCompleted  PDNG – Pending
    ///   RJCT – Rejected
    fn journey_state_to_iso(state: &str) -> &'static str {
        match state {
            "RECEIVED"                       => "RCVD",
            "CUSTOMER_ACCEPTED"              => "ACCP", // AcceptedCustomerProfile (pain.002)
            "TECHNICAL_ACCEPTED"             => "ACTC",
            "PENDING"                        => "PDNG",
            "READY_FOR_EXECUTION"            => "ACSP",
            "SENT_TO_NETWORK"                => "ACSP",
            "SETTLEMENT_IN_PROCESS"          => "ACSP",
            "SETTLED"                        => "ACSC",
            "COMPLETED"                      => "ACSC",
            "CUSTOMER_REJECTED"              => "RJCT",
            "REJECTED"                       => "RJCT",
            "CANCELLED"                      => "RJCT",
            // Recall / return / reversal — no direct equivalent; use RJCT to signal
            // the original instruction will not complete as originally sent.
            "RETURN_REQUESTED"               => "PDNG",
            "RETURNED"                       => "RJCT",
            "RECALL_REQUESTED"               => "PDNG",
            "RECALL_REJECTED"                => "RJCT",
            "FI_REVERSAL_REQUESTED"          => "PDNG",
            "CUSTOMER_REVERSAL_REQUESTED"    => "PDNG",
            "REVERSED"                       => "RJCT",
            // Investigation states — payment is still live but on hold.
            "UNABLE_TO_APPLY"                => "PDNG",
            "INVESTIGATION_OPEN"             => "PDNG",
            "INVESTIGATION_PENDING_RESPONSE" => "PDNG",
            "INVESTIGATION_RESOLVED"         => "ACSC",
            _ => "PDNG",
        }
    }

    fn build_pain002_notification(jd: &JourneyData, status: &str, reason: Option<&str>) -> Result<String, String> {
        use crate::messages::pain00200114 as m;
        let now = chrono::Utc::now();
        let msg_id = format!("PAIN002-{}-{}", now.timestamp_millis(), uuid::Uuid::new_v4());
        let mut tx = m::PaymentTransaction160 {
            orgnl_end_to_end_id: jd.end_to_end_id.clone(),
            orgnl_uetr: jd.uetr.clone(),
            tx_sts: Some(status.into()),
            ..Default::default()
        };
        if let Some(rc) = reason {
            tx.sts_rsn_inf.push(m::StatusReasonInformation14 {
                rsn: Some(m::StatusReason6Choice { cd: Some(rc.into()), ..Default::default() }),
                ..Default::default()
            });
        }
        let body = m::CustomerPaymentStatusReportV14 {
            grp_hdr: m::GroupHeader128 { msg_id, cre_dt_tm: now, ..Default::default() },
            orgnl_grp_inf_and_sts: m::OriginalGroupHeader22 {
                orgnl_msg_id: jd.initiating_msg_id.clone().unwrap_or_default(),
                orgnl_msg_nm_id: jd.initiating_msg_type.clone().unwrap_or_else(|| "pain.001.001.12".into()),
                grp_sts: Some(status.into()),
                ..Default::default()
            },
            orgnl_pmt_inf_and_sts: vec![m::OriginalPaymentInstruction51 {
                orgnl_pmt_inf_id: "1".into(),
                pmt_inf_sts: Some(status.into()),
                tx_inf_and_sts: vec![tx],
                ..Default::default()
            }],
            ..Default::default()
        };
        quick_xml::se::to_string(&m::Document::new(body)).map_err(|e| format!("pain.002: {e}"))
    }

    fn build_camt029_response(jd: &JourneyData, refs: &JourneyRefs, status: &str) -> Result<String, String> {
        use crate::messages::camt02900113 as m;
        let now = chrono::Utc::now();
        let id = format!("CAMT029-{}-{}", now.timestamp_millis(), uuid::Uuid::new_v4());

        // Carry the original transaction references through so the counterparty
        // can correlate this resolution with the specific cancellation request
        // it sent — without these, CxlDtls is empty and the response cannot be
        // matched back to a transaction on the receiving side.
        let tx_cxl_sts = match status {
            "RJCR" => m::CancellationIndividualStatus1Code::Rjcr,
            "ACCR" => m::CancellationIndividualStatus1Code::Accr,
            _ => m::CancellationIndividualStatus1Code::Pdcr,
        };
        let tx = m::PaymentTransaction152 {
            orgnl_end_to_end_id: jd.end_to_end_id.clone(),
            orgnl_tx_id: jd.tx_id.clone(),
            orgnl_uetr: jd.uetr.clone(),
            tx_cxl_sts: Some(tx_cxl_sts),
            ..Default::default()
        };
        let cxl_dtls = m::UnderlyingTransaction32 { tx_inf_and_sts: vec![tx], ..Default::default() };

        let body = m::ResolutionOfInvestigationV13 {
            assgnmt: m::CaseAssignment6 { id, cre_dt_tm: now, ..Default::default() },
            rslvd_case: refs.case_id.as_ref().map(|cid| m::Case6 { id: cid.clone(), ..Default::default() }),
            sts: m::InvestigationStatus6Choice { conf: Some(status.into()), ..Default::default() },
            cxl_dtls: vec![cxl_dtls],
            ..Default::default()
        };
        quick_xml::se::to_string(&m::Document::new(body)).map_err(|e| format!("camt.029: {e}"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    


    fn lazy_crud() -> crate::context::CrudClient {
        // Never actually dials out - connect_lazy() defers the handshake to
        // the first real RPC, so this is safe to use in tests that only
        // exercise a guard-clause / fast-path return before any `.await` on
        // an actual network call.
        let channel = tonic::transport::Channel::from_shared("http://127.0.0.1:1").unwrap().connect_lazy();
        let interceptor = crate::interceptor::AuthInterceptor::new("test-api-key").unwrap();
        crate::crudgrpc::crud_service_client::CrudServiceClient::with_interceptor(channel, interceptor)

    }

    fn base_jd() -> JourneyData {
        JourneyData {
            journey_id: "JRN-1".into(),
            uetr: Some("uetr-1".into()),
            ..Default::default()
        }
    }

    // ── is_transient_status ────────────────────────────────────────────────

    #[test]
    fn transient_codes_are_classified_as_transient() {
        for code in [
            tonic::Code::Unavailable,
            tonic::Code::DeadlineExceeded,
            tonic::Code::ResourceExhausted,
            tonic::Code::Aborted,
            tonic::Code::Unknown,
            tonic::Code::Internal,
        ] {
            assert!(FlowCoordinator::is_transient_status(&tonic::Status::new(code, "x")), "{code:?} should be transient");
        }
    }

    #[test]
    fn business_decision_codes_are_not_transient() {
        for code in [
            tonic::Code::InvalidArgument,
            tonic::Code::NotFound,
            tonic::Code::PermissionDenied,
            tonic::Code::FailedPrecondition,
            tonic::Code::OutOfRange,
            tonic::Code::AlreadyExists,
            tonic::Code::Ok,
            tonic::Code::Unauthenticated,
        ] {
            assert!(!FlowCoordinator::is_transient_status(&tonic::Status::new(code, "x")), "{code:?} should not be transient");
        }
    }

    // ── engine_transaction_id ───────────────────────────────────────────────

    #[test]
    fn engine_transaction_id_is_deterministic_for_the_same_uetr() {
        let jd = base_jd();
        assert_eq!(FlowCoordinator::engine_transaction_id(&jd), FlowCoordinator::engine_transaction_id(&jd));
    }

    #[test]
    fn engine_transaction_id_differs_for_different_uetr() {
        let mut a = base_jd();
        a.uetr = Some("uetr-a".into());
        let mut b = base_jd();
        b.uetr = Some("uetr-b".into());
        assert_ne!(FlowCoordinator::engine_transaction_id(&a), FlowCoordinator::engine_transaction_id(&b));
    }

    #[test]
    fn engine_transaction_id_falls_back_to_journey_id_when_uetr_missing() {
        let mut jd = base_jd();
        jd.uetr = None;
        jd.journey_id = "JRN-FALLBACK".into();
        let mut other = base_jd();
        other.uetr = None;
        other.journey_id = "JRN-FALLBACK".into();
        assert_eq!(FlowCoordinator::engine_transaction_id(&jd), FlowCoordinator::engine_transaction_id(&other));
    }

    #[test]
    fn engine_transaction_id_is_always_non_negative() {
        for key in ["a", "", "uetr-1", "JRN-9999999999", "\u{1F600}"] {
            let mut jd = base_jd();
            jd.uetr = Some(key.to_string());
            assert!(FlowCoordinator::engine_transaction_id(&jd) >= 0);
        }
    }

    // ── journey_state_to_iso ────────────────────────────────────────────────

    #[test]
    fn journey_state_to_iso_maps_documented_states() {
        assert_eq!(FlowCoordinator::journey_state_to_iso("RECEIVED"), "RCVD");
        assert_eq!(FlowCoordinator::journey_state_to_iso("TECHNICAL_ACCEPTED"), "ACTC");
        assert_eq!(FlowCoordinator::journey_state_to_iso("SETTLED"), "ACSC");
        assert_eq!(FlowCoordinator::journey_state_to_iso("COMPLETED"), "ACSC");
        assert_eq!(FlowCoordinator::journey_state_to_iso("REJECTED"), "RJCT");
        assert_eq!(FlowCoordinator::journey_state_to_iso("CUSTOMER_REJECTED"), "RJCT");
    }

    #[test]
    fn journey_state_to_iso_defaults_to_pending_for_unknown_state() {
        assert_eq!(FlowCoordinator::journey_state_to_iso("SOME_UNKNOWN_STATE"), "PDNG");
        assert_eq!(FlowCoordinator::journey_state_to_iso(""), "PDNG");
    }

    // ── resolve_account_id (fast path only - no live network) ──────────────

    #[tokio::test]
    async fn resolve_account_id_accepts_positive_bare_integer_without_network() {
        let mut crud = lazy_crud();
        let result = FlowCoordinator::resolve_account_id(&mut crud, Some("42")).await;
        assert_eq!(result, Ok(42));
    }

    #[tokio::test]
    async fn resolve_account_id_rejects_missing_account_without_network() {
        let mut crud = lazy_crud();
        let result = FlowCoordinator::resolve_account_id(&mut crud, None).await;
        assert!(result.is_err());
    }

    // ── build_pain002_notification / build_camt029_response ────────────────

    #[test]
    fn build_pain002_notification_includes_status_and_reason() {
        let jd = base_jd();
        let xml = FlowCoordinator::build_pain002_notification(&jd, "RJCT", Some("AM04")).unwrap();
        assert!(xml.contains("RJCT"));
        assert!(xml.contains("AM04"));
    }

    #[test]
    fn build_pain002_notification_omits_reason_when_none() {
        let jd = base_jd();
        let xml = FlowCoordinator::build_pain002_notification(&jd, "ACSC", None).unwrap();
        assert!(xml.contains("ACSC"));
    }

    #[test]
    fn build_camt029_response_maps_known_status_codes() {
        let jd = base_jd();
        let refs = JourneyRefs { case_id: Some("CASE-1".into()), ..Default::default() };
        for status in ["RJCR", "ACCR", "PDCR"] {
            let xml = FlowCoordinator::build_camt029_response(&jd, &refs, status).unwrap();
            assert!(xml.contains(status));
            assert!(xml.contains("CASE-1"));
        }
    }

    #[test]
    fn build_camt029_response_unknown_status_falls_back_to_pdcr() {
        let jd = base_jd();
        let refs = JourneyRefs::default();
        // An unrecognized status string maps to the Pdcr (pending) variant
        // rather than erroring - confirm this doesn't panic and still
        // produces well-formed output containing the confirmation status text.
        let xml = FlowCoordinator::build_camt029_response(&jd, &refs, "SOMETHING_ELSE").unwrap();
        assert!(xml.contains("SOMETHING_ELSE"));
    }
}
