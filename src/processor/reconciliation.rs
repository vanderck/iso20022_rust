use crate::crudgrpc::{CreateRequest, DbPath, Fld, ReadRequest};
use crate::context::CrudClient;

/// Reconciliation entry extracted from a camt.052/053/054 statement.
#[derive(Debug, Clone)]
pub struct StatementEntry {
    pub account_id: Option<String>,
    pub entry_ref: Option<String>,
    pub amount: Option<String>,
    pub currency: Option<String>,
    pub credit_debit: Option<String>, // CRDT / DBIT
    pub booking_date: Option<String>,
    pub end_to_end_id: Option<String>,
    pub tx_id: Option<String>,
    pub uetr: Option<String>,
}

/// Result of reconciling a single statement entry.
#[derive(Debug, Clone)]
pub enum ReconResult {
    /// Matched to a known journey.
    Matched { journey_id: String },
    /// No matching journey found — potential orphan or delayed booking.
    Unmatched,
    /// Matched but amount differs.
    AmountMismatch { journey_id: String, expected: String, actual: String },
}

pub struct Reconciliation;

impl Reconciliation {
    fn db_path(table: &str) -> DbPath {
        DbPath { server: "CBS".into(), schema: "iso20022".into(), table: table.into() }
    }

    /// Attempt to match a statement entry against a booked payment journey.
    pub async fn reconcile_entry(
        crud: &mut CrudClient,
        entry: &StatementEntry,
    ) -> Result<ReconResult, tonic::Status> {
        // Try to find the journey by UETR, then EndToEndId, then TxId.
        let journey = Self::find_journey(crud, entry).await?;

        match journey {
            None => {
                // Record unmatched entry for manual investigation.
                Self::record_recon_event(crud, entry, "UNMATCHED", None).await?;
                Ok(ReconResult::Unmatched)
            }
            Some((jid, expected_amount)) => {
                // Check amount consistency.
                if let (Some(stmt_amt), Some(exp_amt)) = (&entry.amount, &expected_amount) {
                    let amounts_match = match (
                        stmt_amt.trim().parse::<rust_decimal::Decimal>(),
                        exp_amt.trim().parse::<rust_decimal::Decimal>(),
                    ) {
                        (Ok(a), Ok(b)) => a == b,
                        _ => stmt_amt.trim() == exp_amt.trim(),
                    };
                    if !amounts_match {
                        Self::record_recon_event(crud, entry, "AMOUNT_MISMATCH", Some(&jid)).await?;
                        return Ok(ReconResult::AmountMismatch {
                            journey_id: jid,
                            expected: exp_amt.to_string(),
                            actual: stmt_amt.clone(),
                        });
                    }
                }
                Self::record_recon_event(crud, entry, "MATCHED", Some(&jid)).await?;
                Ok(ReconResult::Matched { journey_id: jid })
            }
        }
    }

    /// Find a journey matching the statement entry's refs.
    async fn find_journey(
        crud: &mut CrudClient,
        entry: &StatementEntry,
    ) -> Result<Option<(String, Option<String>)>, tonic::Status> {
        // (journey_id, settlement_amount)
        let columns = vec!["journey_id".into(), "settlement_amount".into()];

        for (col, val) in [
            ("uetr", entry.uetr.as_deref()),
            ("end_to_end_id", entry.end_to_end_id.as_deref()),
            ("tx_id", entry.tx_id.as_deref()),
        ] {
            if let Some(v) = val {
                if v.is_empty() { continue; }
                let req = ReadRequest {
                    path: Some(Self::db_path("payment_journeys")),
                    column: columns.clone(),
                    where_clause: vec![Fld { name: col.into(), relation: None, value: v.to_string() }],
                    page: 0, page_size: Some(1),
                };
                let resp = crud.read(req).await?.into_inner();
                if let Some(row) = resp.data.into_iter().next() {
                    let jid = row.list.first().cloned().unwrap_or_default();
                    let amt = row.list.get(1).cloned();
                    if !jid.is_empty() {
                        return Ok(Some((jid, amt)));
                    }
                }
            }
        }
        Ok(None)
    }

    /// Record a reconciliation event in the audit table.
    async fn record_recon_event(
        crud: &mut CrudClient,
        entry: &StatementEntry,
        result: &str,
        journey_id: Option<&str>,
    ) -> Result<(), tonic::Status> {
        let mut data = vec![
            Fld { name: "result".into(), relation: None, value: result.to_string() },
            Fld { name: "entry_ref".into(), relation: None, value: entry.entry_ref.clone().unwrap_or_default() },
            Fld { name: "amount".into(), relation: None, value: entry.amount.clone().unwrap_or_default() },
            Fld { name: "currency".into(), relation: None, value: entry.currency.clone().unwrap_or_default() },
            Fld { name: "credit_debit".into(), relation: None, value: entry.credit_debit.clone().unwrap_or_default() },
        ];
        if let Some(jid) = journey_id {
            data.push(Fld { name: "journey_id".into(), relation: None, value: jid.to_string() });
        }
        if let Some(ref e2e) = entry.end_to_end_id {
            data.push(Fld { name: "end_to_end_id".into(), relation: None, value: e2e.clone() });
        }
        let _ = crud.create(CreateRequest {
            who: "reconciliation".into(), review: Some(false),
            path: Some(Self::db_path("reconciliation_log")),
            data,
        }).await?;
        Ok(())
    }

    /// Bulk-reconcile all entries from a camt.053 statement.
    /// Returns (matched_count, unmatched_count, mismatch_count).
    pub async fn reconcile_statement(
        crud: &mut CrudClient,
        entries: &[StatementEntry],
    ) -> Result<(usize, usize, usize), tonic::Status> {
        let mut matched = 0;
        let mut unmatched = 0;
        let mut mismatched = 0;
        for entry in entries {
            match Self::reconcile_entry(crud, entry).await? {
                ReconResult::Matched { .. } => {
                    crate::processor::metrics::record_reconciliation("matched");
                    matched += 1;
                }
                ReconResult::Unmatched => {
                    crate::processor::metrics::record_reconciliation("unmatched");
                    unmatched += 1;
                }
                ReconResult::AmountMismatch { .. } => {
                    crate::processor::metrics::record_reconciliation("amount_mismatch");
                    mismatched += 1;
                }
            }
        }
        tracing::info!(
            "Reconciliation complete: matched={} unmatched={} mismatched={}",
            matched, unmatched, mismatched
        );
        Ok((matched, unmatched, mismatched))
    }
}
