use async_trait::async_trait;
use quick_xml::de::from_str;
use crate::mapper::Message;
use crate::messages::camt05200113 as m;
use crate::processor::{handler_support::HandlerSupport as H,
    reconciliation::{Reconciliation, StatementEntry},
    transaction_scope::TransactionScope};

pub struct Document;

fn account_id(id: Option<&m::AccountIdentification4Choice>) -> Option<String> {
    id.and_then(|id| id.iban.clone().or_else(|| id.othr.as_ref().map(|o| o.id.clone())))
}

fn credit_debit_str(cd: &m::CreditDebitCode) -> String {
    match cd {
        m::CreditDebitCode::Crdt => "CRDT",
        m::CreditDebitCode::Dbit => "DBIT",
        m::CreditDebitCode::Unknown => "UNKNOWN",
    }.to_string()
}

fn booking_date_str(d: Option<&m::DateAndDateTime2Choice>) -> Option<String> {
    d.and_then(|d| d.dt.map(|v| v.to_string()).or_else(|| d.dt_tm.map(|v| v.date_naive().to_string())))
}

/// Flatten every `Ntry` (and its `NtryDtls/TxDtls`, when present) across all
/// `Rpt` blocks of a camt.052 into reconciliation-ready entries. See
/// `handlers::camt053::extract_entries` for the same shape on end-of-day
/// statements.
pub fn extract_entries(body: &m::BankToCustomerAccountReportV13) -> Vec<StatementEntry> {
    let mut out = Vec::new();
    for rpt in &body.rpt {
        let acct_id = account_id(rpt.acct.id.as_ref());
        for ntry in &rpt.ntry {
            let entry_ref = ntry.ntry_ref.clone().or_else(|| ntry.acct_svcr_ref.clone());
            let entry_amount = ntry.amt.value.to_string();
            let entry_ccy = ntry.amt.ccy.clone();
            let entry_cdt_dbt = credit_debit_str(&ntry.cdt_dbt_ind);
            let booking_date = booking_date_str(ntry.bookg_dt.as_ref());

            let tx_dtls: Vec<_> = ntry.ntry_dtls.iter().flat_map(|d| d.tx_dtls.iter()).collect();
            if tx_dtls.is_empty() {
                out.push(StatementEntry {
                    account_id: acct_id.clone(),
                    entry_ref,
                    amount: Some(entry_amount),
                    currency: Some(entry_ccy),
                    credit_debit: Some(entry_cdt_dbt),
                    booking_date,
                    end_to_end_id: None, tx_id: None, uetr: None,
                });
                continue;
            }
            for tx in tx_dtls {
                let refs = tx.refs.as_ref();
                out.push(StatementEntry {
                    account_id: acct_id.clone(),
                    entry_ref: entry_ref.clone(),
                    amount: tx.amt.as_ref().map(|a| a.value.to_string()).or_else(|| Some(entry_amount.clone())),
                    currency: tx.amt.as_ref().map(|a| a.ccy.clone()).or_else(|| Some(entry_ccy.clone())),
                    credit_debit: tx.cdt_dbt_ind.as_ref().map(credit_debit_str).or_else(|| Some(entry_cdt_dbt.clone())),
                    booking_date: booking_date.clone(),
                    end_to_end_id: refs.and_then(|r| r.end_to_end_id.clone()),
                    tx_id: refs.and_then(|r| r.tx_id.clone()),
                    uetr: refs.and_then(|r| r.uetr.clone()),
                });
            }
        }
    }
    out
}

#[async_trait]
impl Message for Document {
    async fn consume(
        &self,
        ctx: &crate::ProcessingContext,
        msg: &str,
        _counterparty_bic: Option<&str>,
    ) -> Result<String, String> {
        let doc: m::Document = from_str(msg).map_err(|e| e.to_string())?;
        let msg_type = "camt.052.001.13";
        let msg_id = doc.bk_to_cstmr_acct_rpt.grp_hdr.msg_id.clone();
        let mut scope = TransactionScope::new(ctx.crud_client.clone());
        H::archive_message(scope.crud(), msg_type, "INBOUND", &msg_id, msg).await.map_err(|e| e.to_string())?;

        let entries = extract_entries(&doc.bk_to_cstmr_acct_rpt);
        let (matched, unmatched, mismatched) = Reconciliation::reconcile_statement(scope.crud(), &entries)
            .await.map_err(|e| e.to_string())?;

        scope.commit().await.map_err(|e| e.to_string())?;
        Ok(format!(
            "processed {} msg_id={} entries={} matched={} unmatched={} mismatched={}",
            msg_type, msg_id, entries.len(), matched, unmatched, mismatched
        ))
    }

    async fn generate(&self, ctx: &crate::ProcessingContext) -> Result<String, String> {
        let now = chrono::Utc::now(); let msg_id = H::next_msg_id(&mut ctx.uidgen_client.clone(), "CAMT052").await.map_err(|e| e.to_string())?;
        let body = m::BankToCustomerAccountReportV13 { grp_hdr: m::GroupHeader116 { msg_id: msg_id.clone(), cre_dt_tm: now, ..Default::default() }, ..Default::default() };
        let doc_xml = quick_xml::se::to_string(&m::Document::new(body)).map_err(|e| format!("{e}"))?;
        self.wrap(&doc_xml, "camt.052.001.13", &msg_id, &ctx.config.own_bic, "RECEIVERBIC")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn acct_id(iban: Option<&str>, othr: Option<&str>) -> m::AccountIdentification4Choice {
        m::AccountIdentification4Choice {
            iban: iban.map(str::to_string),
            othr: othr.map(|o| m::GenericAccountIdentification1 { id: o.to_string(), ..Default::default() }),
        }
    }

    #[test]
    fn account_id_prefers_iban_over_othr() {
        assert_eq!(account_id(Some(&acct_id(Some("BE68539007547034"), Some("PROP1")))), Some("BE68539007547034".to_string()));
    }

    #[test]
    fn account_id_falls_back_to_othr_when_no_iban() {
        assert_eq!(account_id(Some(&acct_id(None, Some("PROP1")))), Some("PROP1".to_string()));
    }

    #[test]
    fn account_id_is_none_when_absent_entirely() {
        assert_eq!(account_id(None), None);
    }

    #[test]
    fn credit_debit_str_maps_all_variants() {
        assert_eq!(credit_debit_str(&m::CreditDebitCode::Crdt), "CRDT");
        assert_eq!(credit_debit_str(&m::CreditDebitCode::Dbit), "DBIT");
        assert_eq!(credit_debit_str(&m::CreditDebitCode::Unknown), "UNKNOWN");
    }

    #[test]
    fn booking_date_str_prefers_date_over_date_time() {
        let d = m::DateAndDateTime2Choice {
            dt: Some(chrono::NaiveDate::from_ymd_opt(2026, 1, 15).unwrap()),
            dt_tm: Some(chrono::DateTime::parse_from_rfc3339("2026-01-16T10:00:00Z").unwrap().with_timezone(&chrono::Utc)),
        };
        assert_eq!(booking_date_str(Some(&d)), Some("2026-01-15".to_string()));
    }

    #[test]
    fn booking_date_str_falls_back_to_date_time_when_no_bare_date() {
        let d = m::DateAndDateTime2Choice {
            dt: None,
            dt_tm: Some(chrono::DateTime::parse_from_rfc3339("2026-01-16T10:00:00Z").unwrap().with_timezone(&chrono::Utc)),
        };
        assert_eq!(booking_date_str(Some(&d)), Some("2026-01-16".to_string()));
    }

    #[test]
    fn booking_date_str_is_none_when_absent_entirely() {
        assert_eq!(booking_date_str(None), None);
    }

    fn entry_amount(value: &str, ccy: &str) -> m::ActiveOrHistoricCurrencyAndAmount {
        m::ActiveOrHistoricCurrencyAndAmount { value: value.parse().unwrap(), ccy: ccy.to_string() }
    }

    #[test]
    fn extract_entries_empty_report_yields_no_rows() {
        let body = m::BankToCustomerAccountReportV13 { grp_hdr: m::GroupHeader116 { msg_id: "MSG1".into(), ..Default::default() }, ..Default::default() };
        assert!(extract_entries(&body).is_empty());
    }

    #[test]
    fn extract_entries_without_tx_details_yields_one_entry_level_row() {
        let body = m::BankToCustomerAccountReportV13 {
            grp_hdr: m::GroupHeader116 { msg_id: "MSG2".into(), ..Default::default() },
            rpt: vec![m::AccountReport37 {
                acct: m::CashAccount43 { id: Some(acct_id(Some("BE68539007547034"), None)), ..Default::default() },
                ntry: vec![m::ReportEntry15 {
                    ntry_ref: Some("REF1".into()),
                    amt: entry_amount("100.00", "EUR"),
                    cdt_dbt_ind: m::CreditDebitCode::Crdt,
                    ..Default::default()
                }],
                ..Default::default()
            }],
            ..Default::default()
        };
        let entries = extract_entries(&body);
        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0].account_id.as_deref(), Some("BE68539007547034"));
        assert_eq!(entries[0].entry_ref.as_deref(), Some("REF1"));
        assert_eq!(entries[0].amount.as_deref(), Some("100.00"));
        assert_eq!(entries[0].currency.as_deref(), Some("EUR"));
        assert_eq!(entries[0].credit_debit.as_deref(), Some("CRDT"));
        assert_eq!(entries[0].end_to_end_id, None);
    }

    #[test]
    fn extract_entries_with_tx_details_yields_one_row_per_underlying_tx() {
        let body = m::BankToCustomerAccountReportV13 {
            grp_hdr: m::GroupHeader116 { msg_id: "MSG3".into(), ..Default::default() },
            rpt: vec![m::AccountReport37 {
                acct: m::CashAccount43 { id: Some(acct_id(Some("BE68539007547034"), None)), ..Default::default() },
                ntry: vec![m::ReportEntry15 {
                    amt: entry_amount("100.00", "EUR"),
                    cdt_dbt_ind: m::CreditDebitCode::Dbit,
                    ntry_dtls: vec![m::EntryDetails14 {
                        tx_dtls: vec![
                            m::EntryTransaction15 {
                                refs: Some(m::TransactionReferences6 { end_to_end_id: Some("E2E1".into()), ..Default::default() }),
                                amt: Some(entry_amount("40.00", "EUR")),
                                ..Default::default()
                            },
                            m::EntryTransaction15 {
                                refs: Some(m::TransactionReferences6 { end_to_end_id: Some("E2E2".into()), ..Default::default() }),
                                // No per-tx amount - falls back to the entry-level amount/ccy/cdt_dbt.
                                ..Default::default()
                            },
                        ],
                        ..Default::default()
                    }],
                    ..Default::default()
                }],
                ..Default::default()
            }],
            ..Default::default()
        };
        let entries = extract_entries(&body);
        assert_eq!(entries.len(), 2);
        assert_eq!(entries[0].end_to_end_id.as_deref(), Some("E2E1"));
        assert_eq!(entries[0].amount.as_deref(), Some("40.00"));
        assert_eq!(entries[1].end_to_end_id.as_deref(), Some("E2E2"));
        assert_eq!(entries[1].amount.as_deref(), Some("100.00")); // fell back to entry-level
        assert_eq!(entries[1].credit_debit.as_deref(), Some("DBIT")); // fell back too
    }
}
