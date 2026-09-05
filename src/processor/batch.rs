use crate::processor::handler_support::HandlerSupport as H;
use crate::processor::journey_event::{
    JourneyEvent, JourneyEventKind, JourneyRefs, PartyData, SupportedMessage,
};

/// A single extracted transaction from a multi-transaction ISO 20022 message.
#[derive(Debug, Clone)]
pub struct ExtractedTx {
    pub refs: JourneyRefs,
    pub iso_status: Option<String>,
    pub reason_code: Option<String>,
    pub description: String,
    /// Debtor/creditor party data, populated only where the source message
    /// carries it (pain.001, pacs.008); `Default` (all `None`) otherwise.
    pub party: PartyData,
}

/// Extract an account identifier (IBAN preferred, else the proprietary
/// `Othr/Id`) from an optional cash account name/id pair of fields shared by
/// the pain.001 and pacs.008 XSDs (`Option<Iban>`, `Option<Othr>`).
fn account_id(iban: Option<&str>, othr_id: Option<&str>) -> Option<String> {
    iban.map(str::to_string).or_else(|| othr_id.map(str::to_string))
}

// ── pacs.008: CreditTransferTransaction70 ────────────────────────────────

pub fn extract_pacs008_txs(
    body: &crate::messages::pacs00800113::FIToFICustomerCreditTransferV13,
) -> Vec<ExtractedTx> {
    let msg_id = &body.grp_hdr.msg_id;
    body.cdt_trf_tx_inf.iter().enumerate().map(|(i, tx)| {
        let debtor_account = tx.dbtr_acct.as_ref().and_then(|a| a.id.as_ref()).and_then(|id| {
            account_id(id.iban.as_deref(), id.othr.as_ref().map(|o| o.id.as_str()))
        });
        let creditor_account = tx.cdtr_acct.as_ref().and_then(|a| a.id.as_ref()).and_then(|id| {
            account_id(id.iban.as_deref(), id.othr.as_ref().map(|o| o.id.as_str()))
        });
        ExtractedTx {
            refs: JourneyRefs {
                original_msg_id: Some(msg_id.clone()),
                end_to_end_id:   Some(tx.pmt_id.end_to_end_id.clone()),
                tx_id:           tx.pmt_id.tx_id.clone(),
                uetr:            tx.pmt_id.uetr.clone(),
                clr_sys_ref:     tx.pmt_id.clr_sys_ref.clone(),
                case_id: None, assignment_id: None,
            },
            iso_status: None,
            reason_code: None,
            description: format!("FI credit transfer tx {}/{}: msg_id={}",
                i + 1, body.cdt_trf_tx_inf.len(), msg_id),
            party: PartyData {
                debtor_name: tx.dbtr.nm.clone(),
                debtor_account,
                creditor_name: tx.cdtr.nm.clone(),
                creditor_account,
            },
        }
    }).collect()
}

// ── pacs.002: PaymentTransaction164 ──────────────────────────────────────

pub fn extract_pacs002_txs(
    body: &crate::messages::pacs00200115::FIToFIPaymentStatusReportV15,
) -> Vec<ExtractedTx> {
    let msg_id = &body.grp_hdr.msg_id;
    let grp = body.orgnl_grp_inf_and_sts.first();
    let grp_status = grp.and_then(|g| H::ne(&g.grp_sts));
    let grp_msg_id = grp.map(|g| g.orgnl_msg_id.clone());

    if body.tx_inf_and_sts.is_empty() {
        // Group-level only status (no per-tx breakdown).
        return vec![ExtractedTx {
            refs: JourneyRefs {
                original_msg_id: grp_msg_id,
                ..Default::default()
            },
            iso_status: grp_status,
            reason_code: grp.and_then(|g| g.sts_rsn_inf.first())
                .and_then(|sri| sri.rsn.as_ref())
                .and_then(|r| H::ne(&r.cd).or_else(|| H::ne(&r.prtry))),
            description: format!("FI payment status (group-level): msg_id={}", msg_id),
            party: PartyData::default(),
        }];
    }

    body.tx_inf_and_sts.iter().enumerate().map(|(i, tx)| {
        ExtractedTx {
            refs: JourneyRefs {
                original_msg_id: grp_msg_id.clone(),
                end_to_end_id:   tx.orgnl_end_to_end_id.clone(),
                tx_id:           tx.orgnl_tx_id.clone(),
                uetr:            tx.orgnl_uetr.clone(),
                clr_sys_ref:     None,
                case_id: None, assignment_id: None,
            },
            iso_status: H::ne(&tx.tx_sts).or_else(|| grp_status.clone()),
            reason_code: tx.sts_rsn_inf.first()
                .and_then(|sri| sri.rsn.as_ref())
                .and_then(|r| H::ne(&r.cd).or_else(|| H::ne(&r.prtry))),
            description: format!("FI payment status tx {}/{}: msg_id={}",
                i + 1, body.tx_inf_and_sts.len(), msg_id),
            party: PartyData::default(),
        }
    }).collect()
}

// ── pacs.004: PaymentTransaction163 ──────────────────────────────────────

pub fn extract_pacs004_txs(
    body: &crate::messages::pacs00400114::PaymentReturnV14,
) -> Vec<ExtractedTx> {
    let msg_id = &body.grp_hdr.msg_id;
    let grp_msg_id = body.orgnl_grp_inf.as_ref().map(|g| g.orgnl_msg_id.clone());

    body.tx_inf.iter().enumerate().map(|(i, tx)| {
        ExtractedTx {
            refs: JourneyRefs {
                original_msg_id: grp_msg_id.clone(),
                end_to_end_id:   tx.orgnl_end_to_end_id.clone(),
                tx_id:           tx.orgnl_tx_id.clone(),
                uetr:            tx.orgnl_uetr.clone(),
                clr_sys_ref:     tx.orgnl_clr_sys_ref.clone(),
                case_id: None, assignment_id: None,
            },
            iso_status: None,
            reason_code: tx.rtr_rsn_inf.first()
                .and_then(|ri| ri.rsn.as_ref())
                .and_then(|r| H::ne(&r.cd).or_else(|| H::ne(&r.prtry))),
            description: format!("FI return tx {}/{}: msg_id={}",
                i + 1, body.tx_inf.len(), msg_id),
            party: PartyData::default(),
        }
    }).collect()
}

// ── pacs.007: PaymentTransaction149 ──────────────────────────────────────

pub fn extract_pacs007_txs(
    body: &crate::messages::pacs00700113::FIToFIPaymentReversalV13,
) -> Vec<ExtractedTx> {
    let msg_id = &body.grp_hdr.msg_id;
    let grp_msg_id = body.orgnl_grp_inf.as_ref().map(|g| g.orgnl_msg_id.clone());

    body.tx_inf.iter().enumerate().map(|(i, tx)| {
        ExtractedTx {
            refs: JourneyRefs {
                original_msg_id: grp_msg_id.clone(),
                end_to_end_id:   tx.orgnl_end_to_end_id.clone(),
                tx_id:           tx.orgnl_tx_id.clone(),
                uetr:            tx.orgnl_uetr.clone(),
                clr_sys_ref:     tx.orgnl_clr_sys_ref.clone(),
                case_id: None, assignment_id: None,
            },
            iso_status: None,
            reason_code: tx.rvsl_rsn_inf.first()
                .and_then(|ri| ri.rsn.as_ref())
                .and_then(|r| H::ne(&r.cd).or_else(|| H::ne(&r.prtry))),
            description: format!("FI reversal tx {}/{}: msg_id={}",
                i + 1, body.tx_inf.len(), msg_id),
            party: PartyData::default(),
        }
    }).collect()
}

// ── pain.001: CreditTransferTransaction61 via PaymentInstruction44 ───────

pub fn extract_pain001_txs(
    body: &crate::messages::pain00100112::CustomerCreditTransferInitiationV12,
) -> Vec<ExtractedTx> {
    let msg_id = &body.grp_hdr.msg_id;
    let mut out = Vec::new();
    for (pi, pmt) in body.pmt_inf.iter().enumerate() {
        // Dbtr/DbtrAcct are carried once per PaymentInstruction44 (pmt_inf) and
        // shared by every transaction within it; Cdtr/CdtrAcct are per-tx.
        let debtor_name = pmt.dbtr.nm.clone();
        let debtor_account = pmt.dbtr_acct.id.as_ref().and_then(|id| {
            account_id(id.iban.as_deref(), id.othr.as_ref().map(|o| o.id.as_str()))
        });
        for (ti, tx) in pmt.cdt_trf_tx_inf.iter().enumerate() {
            let creditor_account = tx.cdtr_acct.as_ref().and_then(|a| a.id.as_ref()).and_then(|id| {
                account_id(id.iban.as_deref(), id.othr.as_ref().map(|o| o.id.as_str()))
            });
            out.push(ExtractedTx {
                refs: JourneyRefs {
                    original_msg_id: Some(msg_id.clone()),
                    end_to_end_id:   Some(tx.pmt_id.end_to_end_id.clone()),
                    tx_id:           None,
                    uetr:            tx.pmt_id.uetr.clone(),
                    clr_sys_ref:     None,
                    case_id: None, assignment_id: None,
                },
                iso_status: None,
                reason_code: None,
                description: format!("Customer payment pmt_inf[{}] tx[{}]: msg_id={}",
                    pi, ti, msg_id),
                party: PartyData {
                    debtor_name: debtor_name.clone(),
                    debtor_account: debtor_account.clone(),
                    creditor_name: tx.cdtr.as_ref().and_then(|c| c.nm.clone()),
                    creditor_account,
                },
            });
        }
    }
    if out.is_empty() {
        out.push(ExtractedTx {
            refs: JourneyRefs { original_msg_id: Some(msg_id.clone()), ..Default::default() },
            iso_status: None, reason_code: None,
            description: format!("Customer payment initiation (no txs): msg_id={}", msg_id),
            party: PartyData::default(),
        });
    }
    out
}

// ── pain.002: PaymentTransaction160 via OriginalPaymentInstruction51 ─────

pub fn extract_pain002_txs(
    body: &crate::messages::pain00200114::CustomerPaymentStatusReportV14,
) -> Vec<ExtractedTx> {
    let msg_id = &body.grp_hdr.msg_id;
    let grp = &body.orgnl_grp_inf_and_sts;
    let grp_status = H::ne(&grp.grp_sts);
    let mut out = Vec::new();

    for pmt in &body.orgnl_pmt_inf_and_sts {
        let pmt_status = H::ne(&pmt.pmt_inf_sts).or_else(|| grp_status.clone());
        for (ti, tx) in pmt.tx_inf_and_sts.iter().enumerate() {
            out.push(ExtractedTx {
                refs: JourneyRefs {
                    original_msg_id: Some(grp.orgnl_msg_id.clone()),
                    end_to_end_id:   tx.orgnl_end_to_end_id.clone(),
                    tx_id:           None,
                    uetr:            tx.orgnl_uetr.clone(),
                    clr_sys_ref:     None,
                    case_id: None, assignment_id: None,
                },
                iso_status: H::ne(&tx.tx_sts).or_else(|| pmt_status.clone()),
                reason_code: tx.sts_rsn_inf.first()
                    .and_then(|sri| sri.rsn.as_ref())
                    .and_then(|r| H::ne(&r.cd).or_else(|| H::ne(&r.prtry))),
                description: format!("Customer payment status tx[{}]: msg_id={}", ti, msg_id),
                party: PartyData::default(),
            });
        }
        // If no per-tx breakdown, emit one event per payment-info block.
        if pmt.tx_inf_and_sts.is_empty() {
            out.push(ExtractedTx {
                refs: JourneyRefs {
                    original_msg_id: Some(grp.orgnl_msg_id.clone()),
                    ..Default::default()
                },
                iso_status: pmt_status,
                reason_code: pmt.sts_rsn_inf.first()
                    .and_then(|sri| sri.rsn.as_ref())
                    .and_then(|r| H::ne(&r.cd).or_else(|| H::ne(&r.prtry))),
                description: format!("Customer payment status (pmt-level): msg_id={}", msg_id),
                party: PartyData::default(),
            });
        }
    }
    if out.is_empty() {
        // Group-level only.
        out.push(ExtractedTx {
            refs: JourneyRefs {
                original_msg_id: Some(grp.orgnl_msg_id.clone()),
                ..Default::default()
            },
            iso_status: grp_status,
            reason_code: None,
            description: format!("Customer payment status (group-level): msg_id={}", msg_id),
            party: PartyData::default(),
        });
    }
    out
}

// ── pain.007: PaymentTransaction156 via OriginalPaymentInstruction50 ─────

pub fn extract_pain007_txs(
    body: &crate::messages::pain00700112::CustomerPaymentReversalV12,
) -> Vec<ExtractedTx> {
    let msg_id = &body.grp_hdr.msg_id;
    let mut out = Vec::new();
    for pmt in &body.orgnl_pmt_inf_and_rvsl {
        for (ti, tx) in pmt.tx_inf.iter().enumerate() {
            out.push(ExtractedTx {
                refs: JourneyRefs {
                    original_msg_id: Some(body.orgnl_grp_inf.orgnl_msg_id.clone()),
                    end_to_end_id:   tx.orgnl_end_to_end_id.clone(),
                    tx_id:           None,
                    uetr:            tx.orgnl_uetr.clone(),
                    clr_sys_ref:     None,
                    case_id: None, assignment_id: None,
                },
                iso_status: None,
                reason_code: tx.rvsl_rsn_inf.first()
                    .or_else(|| pmt.rvsl_rsn_inf.first())
                    .and_then(|ri| ri.rsn.as_ref())
                    .and_then(|r| H::ne(&r.cd).or_else(|| H::ne(&r.prtry))),
                description: format!("Customer reversal tx[{}]: msg_id={}", ti, msg_id),
                party: PartyData::default(),
            });
        }
    }
    if out.is_empty() {
        out.push(ExtractedTx {
            refs: JourneyRefs { original_msg_id: Some(body.orgnl_grp_inf.orgnl_msg_id.clone()), ..Default::default() },
            iso_status: None, reason_code: None,
            description: format!("Customer reversal (no txs): msg_id={}", msg_id),
            party: PartyData::default(),
        });
    }
    out
}

// ── camt.056: PaymentTransaction155 via UnderlyingTransaction34 ──────────

pub fn extract_camt056_txs(
    body: &crate::messages::camt05600111::FIToFIPaymentCancellationRequestV11,
) -> Vec<ExtractedTx> {
    let assgnmt_id = &body.assgnmt.id;
    let case_id = body.case.as_ref().map(|c| c.id.clone());
    let mut out = Vec::new();
    for u in &body.undrlyg {
        for (ti, tx) in u.tx_inf.iter().enumerate() {
            out.push(ExtractedTx {
                refs: JourneyRefs {
                    original_msg_id: tx.orgnl_grp_inf.as_ref().map(|g| g.orgnl_msg_id.clone()),
                    end_to_end_id:   tx.orgnl_end_to_end_id.clone(),
                    tx_id:           tx.orgnl_tx_id.clone(),
                    uetr:            tx.orgnl_uetr.clone(),
                    clr_sys_ref:     tx.orgnl_clr_sys_ref.clone(),
                    case_id:         case_id.clone(),
                    assignment_id:   Some(assgnmt_id.clone()),
                },
                iso_status: None,
                reason_code: tx.cxl_rsn_inf.first()
                    .and_then(|cri| cri.rsn.as_ref())
                    .and_then(|r| H::ne(&r.cd).or_else(|| H::ne(&r.prtry))),
                description: format!("Cancellation request tx[{}]: assgnmt_id={}", ti, assgnmt_id),
                party: PartyData::default(),
            });
        }
    }
    if out.is_empty() {
        out.push(ExtractedTx {
            refs: JourneyRefs { case_id, assignment_id: Some(assgnmt_id.clone()), ..Default::default() },
            iso_status: None, reason_code: None,
            description: format!("Cancellation request (no txs): assgnmt_id={}", assgnmt_id),
            party: PartyData::default(),
        });
    }
    out
}

// ── Helpers to convert ExtractedTx into JourneyEvent ─────────────────────

impl ExtractedTx {
    pub fn into_event(self, source: SupportedMessage, kind: JourneyEventKind) -> JourneyEvent {
        JourneyEvent {
            source_message: source,
            kind,
            refs: self.refs,
            iso_status: self.iso_status,
            reason_code: self.reason_code,
            description: self.description,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::messages::pacs00200115 as p002;
    use crate::messages::pacs00400114 as p004;
    use crate::messages::pacs00700113 as p007;
    use crate::messages::pacs00800113 as p008;
    use crate::messages::pain00100112 as p001;
    use crate::messages::pain00200114 as p002r;
    use crate::messages::pain00700112 as p007r;
    use crate::messages::camt05600111 as c056;

    // ── account_id ────────────────────────────────────────────────────────

    #[test]
    fn account_id_prefers_iban_over_othr() {
        assert_eq!(account_id(Some("BE68539007547034"), Some("PROP123")), Some("BE68539007547034".to_string()));
    }

    #[test]
    fn account_id_falls_back_to_othr_when_no_iban() {
        assert_eq!(account_id(None, Some("PROP123")), Some("PROP123".to_string()));
    }

    #[test]
    fn account_id_is_none_when_neither_present() {
        assert_eq!(account_id(None, None), None);
    }

    // ── extract_pacs008_txs ───────────────────────────────────────────────

    fn cash_account(iban: Option<&str>, othr: Option<&str>) -> p008::CashAccount40 {
        p008::CashAccount40 {
            id: Some(p008::AccountIdentification4Choice {
                iban: iban.map(str::to_string),
                othr: othr.map(|o| p008::GenericAccountIdentification1 { id: o.to_string(), ..Default::default() }),
            }),
            ..Default::default()
        }
    }

    #[test]
    fn extract_pacs008_txs_maps_party_and_account_with_iban_preference() {
        let body = p008::FIToFICustomerCreditTransferV13 {
            grp_hdr: p008::GroupHeader131 { msg_id: "MSGID1".into(), ..Default::default() },
            cdt_trf_tx_inf: vec![p008::CreditTransferTransaction70 {
                pmt_id: p008::PaymentIdentification13 {
                    end_to_end_id: "E2E1".into(),
                    tx_id: Some("TX1".into()),
                    uetr: Some("UETR1".into()),
                    ..Default::default()
                },
                dbtr: p008::PartyIdentification272 { nm: Some("Alice".into()), ..Default::default() },
                dbtr_acct: Some(cash_account(Some("BE68539007547034"), Some("IGNORED"))),
                cdtr: p008::PartyIdentification272 { nm: Some("Bob".into()), ..Default::default() },
                cdtr_acct: Some(cash_account(None, Some("PROP999"))),
                ..Default::default()
            }],
            ..Default::default()
        };
        let txs = extract_pacs008_txs(&body);
        assert_eq!(txs.len(), 1);
        let tx = &txs[0];
        assert_eq!(tx.refs.original_msg_id.as_deref(), Some("MSGID1"));
        assert_eq!(tx.refs.end_to_end_id.as_deref(), Some("E2E1"));
        assert_eq!(tx.refs.tx_id.as_deref(), Some("TX1"));
        assert_eq!(tx.refs.uetr.as_deref(), Some("UETR1"));
        assert_eq!(tx.party.debtor_name.as_deref(), Some("Alice"));
        assert_eq!(tx.party.debtor_account.as_deref(), Some("BE68539007547034")); // IBAN wins
        assert_eq!(tx.party.creditor_name.as_deref(), Some("Bob"));
        assert_eq!(tx.party.creditor_account.as_deref(), Some("PROP999")); // Othr fallback
        assert!(tx.description.contains("1/1"));
    }

    #[test]
    fn extract_pacs008_txs_handles_missing_account_blocks() {
        let body = p008::FIToFICustomerCreditTransferV13 {
            grp_hdr: p008::GroupHeader131 { msg_id: "MSGID2".into(), ..Default::default() },
            cdt_trf_tx_inf: vec![p008::CreditTransferTransaction70 {
                pmt_id: p008::PaymentIdentification13 { end_to_end_id: "E2E2".into(), ..Default::default() },
                dbtr_acct: None,
                cdtr_acct: None,
                ..Default::default()
            }],
            ..Default::default()
        };
        let txs = extract_pacs008_txs(&body);
        assert_eq!(txs[0].party.debtor_account, None);
        assert_eq!(txs[0].party.creditor_account, None);
    }

    #[test]
    fn extract_pacs008_txs_numbers_multiple_transactions() {
        let mk_tx = |e2e: &str| p008::CreditTransferTransaction70 {
            pmt_id: p008::PaymentIdentification13 { end_to_end_id: e2e.into(), ..Default::default() },
            ..Default::default()
        };
        let body = p008::FIToFICustomerCreditTransferV13 {
            grp_hdr: p008::GroupHeader131 { msg_id: "MSGID3".into(), ..Default::default() },
            cdt_trf_tx_inf: vec![mk_tx("A"), mk_tx("B"), mk_tx("C")],
            ..Default::default()
        };
        let txs = extract_pacs008_txs(&body);
        assert_eq!(txs.len(), 3);
        assert!(txs[0].description.contains("1/3"));
        assert!(txs[1].description.contains("2/3"));
        assert!(txs[2].description.contains("3/3"));
    }

    #[test]
    fn extract_pacs008_txs_empty_input_yields_no_rows() {
        let body = p008::FIToFICustomerCreditTransferV13 {
            grp_hdr: p008::GroupHeader131 { msg_id: "MSGID4".into(), ..Default::default() },
            ..Default::default()
        };
        assert!(extract_pacs008_txs(&body).is_empty());
    }

    // ── extract_pacs002_txs ───────────────────────────────────────────────

    #[test]
    fn extract_pacs002_txs_group_level_only_when_no_per_tx_breakdown() {
        let body = p002::FIToFIPaymentStatusReportV15 {
            grp_hdr: p002::GroupHeader120 { msg_id: "STS1".into(), ..Default::default() },
            orgnl_grp_inf_and_sts: vec![p002::OriginalGroupHeader22 {
                orgnl_msg_id: "ORIG1".into(),
                orgnl_msg_nm_id: "pacs.008.001.13".into(),
                grp_sts: Some("RJCT".into()),
                sts_rsn_inf: vec![p002::StatusReasonInformation14 {
                    rsn: Some(p002::StatusReason6Choice { cd: Some("AC04".into()), ..Default::default() }),
                    ..Default::default()
                }],
                ..Default::default()
            }],
            tx_inf_and_sts: vec![],
            ..Default::default()
        };
        let txs = extract_pacs002_txs(&body);
        assert_eq!(txs.len(), 1);
        assert_eq!(txs[0].iso_status.as_deref(), Some("RJCT"));
        assert_eq!(txs[0].reason_code.as_deref(), Some("AC04"));
        assert_eq!(txs[0].refs.original_msg_id.as_deref(), Some("ORIG1"));
    }

    #[test]
    fn extract_pacs002_txs_per_tx_status_falls_back_to_group_status() {
        let body = p002::FIToFIPaymentStatusReportV15 {
            grp_hdr: p002::GroupHeader120 { msg_id: "STS2".into(), ..Default::default() },
            orgnl_grp_inf_and_sts: vec![p002::OriginalGroupHeader22 {
                orgnl_msg_id: "ORIG2".into(),
                orgnl_msg_nm_id: "pacs.008.001.13".into(),
                grp_sts: Some("ACSP".into()),
                ..Default::default()
            }],
            tx_inf_and_sts: vec![p002::PaymentTransaction164 {
                orgnl_end_to_end_id: Some("E2E".into()),
                tx_sts: None, // no per-tx status -> should inherit group status
                ..Default::default()
            }],
            ..Default::default()
        };
        let txs = extract_pacs002_txs(&body);
        assert_eq!(txs.len(), 1);
        assert_eq!(txs[0].iso_status.as_deref(), Some("ACSP"));
    }

    #[test]
    fn extract_pacs002_txs_reason_code_prefers_code_over_proprietary() {
        let body = p002::FIToFIPaymentStatusReportV15 {
            grp_hdr: p002::GroupHeader120 { msg_id: "STS3".into(), ..Default::default() },
            tx_inf_and_sts: vec![p002::PaymentTransaction164 {
                tx_sts: Some("RJCT".into()),
                sts_rsn_inf: vec![p002::StatusReasonInformation14 {
                    rsn: Some(p002::StatusReason6Choice { cd: Some("AM04".into()), prtry: Some("PROP".into()) }),
                    ..Default::default()
                }],
                ..Default::default()
            }],
            ..Default::default()
        };
        let txs = extract_pacs002_txs(&body);
        assert_eq!(txs[0].reason_code.as_deref(), Some("AM04"));
    }

    #[test]
    fn extract_pacs002_txs_reason_code_falls_back_to_proprietary_when_no_code() {
        let body = p002::FIToFIPaymentStatusReportV15 {
            grp_hdr: p002::GroupHeader120 { msg_id: "STS4".into(), ..Default::default() },
            tx_inf_and_sts: vec![p002::PaymentTransaction164 {
                tx_sts: Some("RJCT".into()),
                sts_rsn_inf: vec![p002::StatusReasonInformation14 {
                    rsn: Some(p002::StatusReason6Choice { cd: None, prtry: Some("PROP-ONLY".into()) }),
                    ..Default::default()
                }],
                ..Default::default()
            }],
            ..Default::default()
        };
        let txs = extract_pacs002_txs(&body);
        assert_eq!(txs[0].reason_code.as_deref(), Some("PROP-ONLY"));
    }

    // ── extract_pain001_txs ───────────────────────────────────────────────

    #[test]
    fn extract_pain001_txs_shares_debtor_across_multiple_tx_in_one_pmt_inf() {
        let mk_tx = |e2e: &str, cdtr_name: &str| p001::CreditTransferTransaction61 {
            pmt_id: p001::PaymentIdentification6 { end_to_end_id: e2e.into(), ..Default::default() },
            cdtr: Some(p001::PartyIdentification272 { nm: Some(cdtr_name.into()), ..Default::default() }),
            ..Default::default()
        };
        let body = p001::CustomerCreditTransferInitiationV12 {
            grp_hdr: p001::GroupHeader114 { msg_id: "PAIN1".into(), ..Default::default() },
            pmt_inf: vec![p001::PaymentInstruction44 {
                dbtr: p001::PartyIdentification272 { nm: Some("Debtor Co".into()), ..Default::default() },
                dbtr_acct: p001::CashAccount40 {
                    id: Some(p001::AccountIdentification4Choice { iban: Some("BE01".into()), othr: None }),
                    ..Default::default()
                },
                cdt_trf_tx_inf: vec![mk_tx("E1", "Creditor One"), mk_tx("E2", "Creditor Two")],
                ..Default::default()
            }],
            ..Default::default()
        };
        let txs = extract_pain001_txs(&body);
        assert_eq!(txs.len(), 2);
        for tx in &txs {
            assert_eq!(tx.party.debtor_name.as_deref(), Some("Debtor Co"));
            assert_eq!(tx.party.debtor_account.as_deref(), Some("BE01"));
        }
        assert_eq!(txs[0].party.creditor_name.as_deref(), Some("Creditor One"));
        assert_eq!(txs[1].party.creditor_name.as_deref(), Some("Creditor Two"));
    }

    #[test]
    fn extract_pain001_txs_no_payment_instructions_yields_one_placeholder_row() {
        let body = p001::CustomerCreditTransferInitiationV12 {
            grp_hdr: p001::GroupHeader114 { msg_id: "PAIN-EMPTY".into(), ..Default::default() },
            pmt_inf: vec![],
            ..Default::default()
        };
        let txs = extract_pain001_txs(&body);
        assert_eq!(txs.len(), 1);
        assert_eq!(txs[0].refs.original_msg_id.as_deref(), Some("PAIN-EMPTY"));
        assert!(txs[0].description.contains("no txs"));
    }

    #[test]
    fn extract_pain001_txs_pmt_inf_with_no_transactions_contributes_nothing() {
        // A pmt_inf block with zero cdt_trf_tx_inf entries shouldn't emit any
        // row for that block (only the overall empty-output fallback applies
        // when NO pmt_inf produced any row at all).
        let body = p001::CustomerCreditTransferInitiationV12 {
            grp_hdr: p001::GroupHeader114 { msg_id: "PAIN-MIXED".into(), ..Default::default() },
            pmt_inf: vec![
                p001::PaymentInstruction44 { cdt_trf_tx_inf: vec![], ..Default::default() },
                p001::PaymentInstruction44 {
                    cdt_trf_tx_inf: vec![p001::CreditTransferTransaction61 {
                        pmt_id: p001::PaymentIdentification6 { end_to_end_id: "ONLY".into(), ..Default::default() },
                        ..Default::default()
                    }],
                    ..Default::default()
                },
            ],
            ..Default::default()
        };
        let txs = extract_pain001_txs(&body);
        assert_eq!(txs.len(), 1);
        assert_eq!(txs[0].refs.end_to_end_id.as_deref(), Some("ONLY"));
    }

    // ── extract_camt056_txs ───────────────────────────────────────────────

    #[test]
    fn extract_camt056_txs_propagates_case_and_assignment_ids() {
        let body = c056::FIToFIPaymentCancellationRequestV11 {
            assgnmt: c056::CaseAssignment6 { id: "ASSGN1".into(), ..Default::default() },
            case: Some(c056::Case6 { id: "CASE1".into(), ..Default::default() }),
            undrlyg: vec![c056::UnderlyingTransaction34 {
                tx_inf: vec![c056::PaymentTransaction155 {
                    orgnl_end_to_end_id: Some("E2E".into()),
                    orgnl_uetr: Some("UETR".into()),
                    cxl_rsn_inf: vec![c056::PaymentCancellationReason6 {
                        rsn: Some(c056::CancellationReason33Choice { cd: Some("CUST".into()), ..Default::default() }),
                        ..Default::default()
                    }],
                    ..Default::default()
                }],
                ..Default::default()
            }],
            ..Default::default()
        };
        let txs = extract_camt056_txs(&body);
        assert_eq!(txs.len(), 1);
        assert_eq!(txs[0].refs.case_id.as_deref(), Some("CASE1"));
        assert_eq!(txs[0].refs.assignment_id.as_deref(), Some("ASSGN1"));
        assert_eq!(txs[0].reason_code.as_deref(), Some("CUST"));
    }

    #[test]
    fn extract_camt056_txs_no_underlying_transactions_yields_placeholder() {
        let body = c056::FIToFIPaymentCancellationRequestV11 {
            assgnmt: c056::CaseAssignment6 { id: "ASSGN2".into(), ..Default::default() },
            case: None,
            undrlyg: vec![],
            ..Default::default()
        };
        let txs = extract_camt056_txs(&body);
        assert_eq!(txs.len(), 1);
        assert_eq!(txs[0].refs.assignment_id.as_deref(), Some("ASSGN2"));
        assert_eq!(txs[0].refs.case_id, None);
    }

    // ── extract_pacs004_txs ───────────────────────────────────────────────

    #[test]
    fn extract_pacs004_txs_maps_refs_and_reason_per_tx() {
        let body = p004::PaymentReturnV14 {
            grp_hdr: p004::GroupHeader123 { msg_id: "MSGID1".into(), ..Default::default() },
            orgnl_grp_inf: Some(p004::OriginalGroupHeader19 { orgnl_msg_id: "ORIG1".into(), ..Default::default() }),
            tx_inf: vec![p004::PaymentTransaction163 {
                orgnl_end_to_end_id: Some("E2E1".into()),
                orgnl_tx_id: Some("TX1".into()),
                orgnl_uetr: Some("UETR1".into()),
                orgnl_clr_sys_ref: Some("CLR1".into()),
                rtr_rsn_inf: vec![p004::PaymentReturnReason7 {
                    rsn: Some(p004::ReturnReason5Choice { cd: Some("AC04".into()), ..Default::default() }),
                    ..Default::default()
                }],
                ..Default::default()
            }],
            ..Default::default()
        };
        let txs = extract_pacs004_txs(&body);
        assert_eq!(txs.len(), 1);
        assert_eq!(txs[0].refs.original_msg_id.as_deref(), Some("ORIG1"));
        assert_eq!(txs[0].refs.end_to_end_id.as_deref(), Some("E2E1"));
        assert_eq!(txs[0].refs.tx_id.as_deref(), Some("TX1"));
        assert_eq!(txs[0].refs.uetr.as_deref(), Some("UETR1"));
        assert_eq!(txs[0].refs.clr_sys_ref.as_deref(), Some("CLR1"));
        assert_eq!(txs[0].reason_code.as_deref(), Some("AC04"));
    }

    #[test]
    fn extract_pacs004_txs_reason_code_falls_back_to_proprietary_when_no_code() {
        let body = p004::PaymentReturnV14 {
            grp_hdr: p004::GroupHeader123 { msg_id: "MSGID2".into(), ..Default::default() },
            tx_inf: vec![p004::PaymentTransaction163 {
                rtr_rsn_inf: vec![p004::PaymentReturnReason7 {
                    rsn: Some(p004::ReturnReason5Choice { cd: None, prtry: Some("CUSTOM".into()) }),
                    ..Default::default()
                }],
                ..Default::default()
            }],
            ..Default::default()
        };
        assert_eq!(extract_pacs004_txs(&body)[0].reason_code.as_deref(), Some("CUSTOM"));
    }

    #[test]
    fn extract_pacs004_txs_empty_input_yields_no_rows() {
        let body = p004::PaymentReturnV14 { grp_hdr: p004::GroupHeader123 { msg_id: "MSGID3".into(), ..Default::default() }, ..Default::default() };
        assert!(extract_pacs004_txs(&body).is_empty());
    }

    // ── extract_pacs007_txs ───────────────────────────────────────────────

    #[test]
    fn extract_pacs007_txs_maps_refs_and_reason_per_tx() {
        let body = p007::FIToFIPaymentReversalV13 {
            grp_hdr: p007::GroupHeader127 { msg_id: "MSGID1".into(), ..Default::default() },
            orgnl_grp_inf: Some(p007::OriginalGroupHeader20 { orgnl_msg_id: "ORIG1".into(), ..Default::default() }),
            tx_inf: vec![p007::PaymentTransaction149 {
                orgnl_end_to_end_id: Some("E2E1".into()),
                orgnl_tx_id: Some("TX1".into()),
                orgnl_uetr: Some("UETR1".into()),
                orgnl_clr_sys_ref: Some("CLR1".into()),
                rvsl_rsn_inf: vec![p007::PaymentReversalReason10 {
                    rsn: Some(p007::ReversalReason4Choice { cd: Some("AM09".into()), ..Default::default() }),
                    ..Default::default()
                }],
                ..Default::default()
            }],
            ..Default::default()
        };
        let txs = extract_pacs007_txs(&body);
        assert_eq!(txs.len(), 1);
        assert_eq!(txs[0].refs.original_msg_id.as_deref(), Some("ORIG1"));
        assert_eq!(txs[0].refs.end_to_end_id.as_deref(), Some("E2E1"));
        assert_eq!(txs[0].refs.tx_id.as_deref(), Some("TX1"));
        assert_eq!(txs[0].refs.uetr.as_deref(), Some("UETR1"));
        assert_eq!(txs[0].reason_code.as_deref(), Some("AM09"));
    }

    #[test]
    fn extract_pacs007_txs_numbers_multiple_transactions() {
        let mk_tx = |e2e: &str| p007::PaymentTransaction149 {
            orgnl_end_to_end_id: Some(e2e.into()),
            ..Default::default()
        };
        let body = p007::FIToFIPaymentReversalV13 {
            grp_hdr: p007::GroupHeader127 { msg_id: "MSGID2".into(), ..Default::default() },
            tx_inf: vec![mk_tx("A"), mk_tx("B")],
            ..Default::default()
        };
        let txs = extract_pacs007_txs(&body);
        assert_eq!(txs.len(), 2);
        assert!(txs[0].description.contains("1/2"));
        assert!(txs[1].description.contains("2/2"));
    }

    #[test]
    fn extract_pacs007_txs_empty_input_yields_no_rows() {
        let body = p007::FIToFIPaymentReversalV13 { grp_hdr: p007::GroupHeader127 { msg_id: "MSGID3".into(), ..Default::default() }, ..Default::default() };
        assert!(extract_pacs007_txs(&body).is_empty());
    }

    // ── extract_pain002_txs ───────────────────────────────────────────────

    #[test]
    fn extract_pain002_txs_per_tx_status_falls_back_through_pmt_to_group() {
        let body = p002r::CustomerPaymentStatusReportV14 {
            grp_hdr: p002r::GroupHeader128 { msg_id: "MSGID1".into(), ..Default::default() },
            orgnl_grp_inf_and_sts: p002r::OriginalGroupHeader22 {
                orgnl_msg_id: "ORIG1".into(),
                orgnl_msg_nm_id: "pain.001.001.12".into(),
                grp_sts: Some("RJCT".into()),
                ..Default::default()
            },
            orgnl_pmt_inf_and_sts: vec![p002r::OriginalPaymentInstruction51 {
                orgnl_pmt_inf_id: "PMTINF1".into(),
                pmt_inf_sts: None, // no pmt-level status either - must fall through to group.
                tx_inf_and_sts: vec![p002r::PaymentTransaction160 {
                    orgnl_end_to_end_id: Some("E2E1".into()),
                    orgnl_uetr: Some("UETR1".into()),
                    tx_sts: None,
                    ..Default::default()
                }],
                ..Default::default()
            }],
            ..Default::default()
        };
        let txs = extract_pain002_txs(&body);
        assert_eq!(txs.len(), 1);
        assert_eq!(txs[0].refs.original_msg_id.as_deref(), Some("ORIG1"));
        assert_eq!(txs[0].refs.end_to_end_id.as_deref(), Some("E2E1"));
        assert_eq!(txs[0].refs.uetr.as_deref(), Some("UETR1"));
        assert_eq!(txs[0].iso_status.as_deref(), Some("RJCT")); // fell all the way back to group status
    }

    #[test]
    fn extract_pain002_txs_pmt_level_status_used_when_no_per_tx_breakdown() {
        let body = p002r::CustomerPaymentStatusReportV14 {
            grp_hdr: p002r::GroupHeader128 { msg_id: "MSGID2".into(), ..Default::default() },
            orgnl_grp_inf_and_sts: p002r::OriginalGroupHeader22 {
                orgnl_msg_id: "ORIG2".into(),
                orgnl_msg_nm_id: "pain.001.001.12".into(),
                ..Default::default()
            },
            orgnl_pmt_inf_and_sts: vec![p002r::OriginalPaymentInstruction51 {
                orgnl_pmt_inf_id: "PMTINF2".into(),
                pmt_inf_sts: Some("ACCP".into()),
                tx_inf_and_sts: vec![],
                ..Default::default()
            }],
            ..Default::default()
        };
        let txs = extract_pain002_txs(&body);
        assert_eq!(txs.len(), 1);
        assert_eq!(txs[0].iso_status.as_deref(), Some("ACCP"));
        assert!(txs[0].description.contains("pmt-level"));
    }

    #[test]
    fn extract_pain002_txs_group_level_only_when_no_payment_blocks() {
        let body = p002r::CustomerPaymentStatusReportV14 {
            grp_hdr: p002r::GroupHeader128 { msg_id: "MSGID3".into(), ..Default::default() },
            orgnl_grp_inf_and_sts: p002r::OriginalGroupHeader22 {
                orgnl_msg_id: "ORIG3".into(),
                orgnl_msg_nm_id: "pain.001.001.12".into(),
                grp_sts: Some("PDNG".into()),
                ..Default::default()
            },
            orgnl_pmt_inf_and_sts: vec![],
            ..Default::default()
        };
        let txs = extract_pain002_txs(&body);
        assert_eq!(txs.len(), 1);
        assert_eq!(txs[0].iso_status.as_deref(), Some("PDNG"));
        assert!(txs[0].description.contains("group-level"));
    }

    // ── extract_pain007_txs ───────────────────────────────────────────────

    #[test]
    fn extract_pain007_txs_per_tx_reason_falls_back_to_pmt_level() {
        let body = p007r::CustomerPaymentReversalV12 {
            grp_hdr: p007r::GroupHeader124 { msg_id: "MSGID1".into(), ..Default::default() },
            orgnl_grp_inf: p007r::OriginalGroupHeader20 {
                orgnl_msg_id: "ORIG1".into(),
                orgnl_msg_nm_id: "pain.001.001.12".into(),
                ..Default::default()
            },
            orgnl_pmt_inf_and_rvsl: vec![p007r::OriginalPaymentInstruction50 {
                orgnl_pmt_inf_id: "PMTINF1".into(),
                rvsl_rsn_inf: vec![p007r::PaymentReversalReason10 {
                    rsn: Some(p007r::ReversalReason4Choice { cd: Some("PMT-LEVEL".into()), ..Default::default() }),
                    ..Default::default()
                }],
                tx_inf: vec![p007r::PaymentTransaction156 {
                    orgnl_end_to_end_id: Some("E2E1".into()),
                    orgnl_uetr: Some("UETR1".into()),
                    rvsl_rsn_inf: vec![], // no per-tx reason - must fall back to the pmt-level one.
                    ..Default::default()
                }],
                ..Default::default()
            }],
            ..Default::default()
        };
        let txs = extract_pain007_txs(&body);
        assert_eq!(txs.len(), 1);
        assert_eq!(txs[0].refs.original_msg_id.as_deref(), Some("ORIG1"));
        assert_eq!(txs[0].refs.end_to_end_id.as_deref(), Some("E2E1"));
        assert_eq!(txs[0].refs.uetr.as_deref(), Some("UETR1"));
        assert_eq!(txs[0].reason_code.as_deref(), Some("PMT-LEVEL"));
    }

    #[test]
    fn extract_pain007_txs_per_tx_reason_wins_over_pmt_level() {
        let body = p007r::CustomerPaymentReversalV12 {
            grp_hdr: p007r::GroupHeader124 { msg_id: "MSGID2".into(), ..Default::default() },
            orgnl_grp_inf: p007r::OriginalGroupHeader20 {
                orgnl_msg_id: "ORIG2".into(),
                orgnl_msg_nm_id: "pain.001.001.12".into(),
                ..Default::default()
            },
            orgnl_pmt_inf_and_rvsl: vec![p007r::OriginalPaymentInstruction50 {
                orgnl_pmt_inf_id: "PMTINF2".into(),
                rvsl_rsn_inf: vec![p007r::PaymentReversalReason10 {
                    rsn: Some(p007r::ReversalReason4Choice { cd: Some("PMT-LEVEL".into()), ..Default::default() }),
                    ..Default::default()
                }],
                tx_inf: vec![p007r::PaymentTransaction156 {
                    rvsl_rsn_inf: vec![p007r::PaymentReversalReason10 {
                        rsn: Some(p007r::ReversalReason4Choice { cd: Some("TX-LEVEL".into()), ..Default::default() }),
                        ..Default::default()
                    }],
                    ..Default::default()
                }],
                ..Default::default()
            }],
            ..Default::default()
        };
        assert_eq!(extract_pain007_txs(&body)[0].reason_code.as_deref(), Some("TX-LEVEL"));
    }

    #[test]
    fn extract_pain007_txs_no_txs_yields_placeholder_row() {
        let body = p007r::CustomerPaymentReversalV12 {
            grp_hdr: p007r::GroupHeader124 { msg_id: "MSGID3".into(), ..Default::default() },
            orgnl_grp_inf: p007r::OriginalGroupHeader20 {
                orgnl_msg_id: "ORIG3".into(),
                orgnl_msg_nm_id: "pain.001.001.12".into(),
                ..Default::default()
            },
            orgnl_pmt_inf_and_rvsl: vec![],
            ..Default::default()
        };
        let txs = extract_pain007_txs(&body);
        assert_eq!(txs.len(), 1);
        assert_eq!(txs[0].refs.original_msg_id.as_deref(), Some("ORIG3"));
        assert!(txs[0].description.contains("no txs"));
    }
}
