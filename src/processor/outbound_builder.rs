use chrono::Utc;
use quick_xml::se::to_string as xml_to_string;
use rust_decimal::Decimal;

use crate::crudgrpc::{Fld, ReadRequest, DbPath};
use crate::messages::{pacs00200115 as pacs002, pacs00800113 as pacs008};
use crate::processor::handler_support::HandlerSupport as H;
use crate::context::CrudClient;

/// Builds an outbound `MsgId` that fits the field it goes into.
///
/// ISO 20022 types `MsgId` as `Max35Text`. The previous form -
/// `PACS002-{millis}-{uuid}` - is 58 characters, so every outbound pacs.002,
/// pacs.008 and pacs.004 this service emitted carried a MsgId 23 characters
/// over the limit. Nothing here caught it: the generated Rust aliases
/// `Max35Text` to a plain `String` with no length check, and schema validation
/// runs on the inbound path only. A counterparty validating against the
/// published XSD would have rejected all of them.
///
/// The replacement is `{prefix}-{millis}-{12 hex}`: 8 + 13 + 1 + 12 = 34
/// characters for a 7-character prefix, one under the limit. The 12 hex digits
/// come from a v4 UUID's random bits, so collision within a millisecond needs
/// 2^48 draws rather than being merely unlikely by construction.
fn outbound_msg_id(prefix: &str, millis: i64) -> String {
    let uuid = uuid::Uuid::new_v4().simple().to_string();
    let id = format!("{prefix}-{millis}-{}", &uuid[..12]);
    debug_assert!(
        id.len() <= 35,
        "MsgId '{id}' is {} chars; ISO 20022 Max35Text permits 35",
        id.len()
    );
    id
}

/// Data read from the CBS for a single payment journey.
#[derive(Debug, Clone, Default)]
pub struct JourneyData {
    pub journey_id:       String,
    pub current_status:   String,
    pub uetr:             Option<String>,
    pub tx_id:            Option<String>,
    pub end_to_end_id:    Option<String>,
    pub clr_sys_ref:      Option<String>,
    pub settlement_amount: Option<String>,
    pub settlement_ccy:   Option<String>,
    pub settlement_date:  Option<String>,
    pub instructing_agent: Option<String>,
    pub instructed_agent:  Option<String>,
    pub debtor_agent:     Option<String>,
    pub creditor_agent:   Option<String>,
    pub initiating_msg_id: Option<String>,
    pub initiating_msg_type: Option<String>,
    pub debtor_name:      Option<String>,
    pub debtor_account:   Option<String>,
    pub creditor_name:    Option<String>,
    pub creditor_account: Option<String>,
}

pub struct OutboundBuilder;

impl OutboundBuilder {
    fn db_path(table: &str) -> DbPath {
        DbPath { server: "CBS".into(), schema: "iso20022".into(), table: table.into() }
    }

    // ── Read journey from CBS ────────────────────────────────────────────

    pub async fn load_journey(
        crud: &mut CrudClient,
        journey_id: &str,
    ) -> Result<Option<JourneyData>, tonic::Status> {
        let columns = vec![
            "journey_id", "current_status", "uetr", "tx_id", "end_to_end_id",
            "clr_sys_ref", "settlement_amount", "settlement_currency",
            "settlement_date", "instructing_agent", "instructed_agent",
            "debtor_agent", "creditor_agent", "initiating_msg_id", "initiating_msg_type",
            "debtor_name", "debtor_account", "creditor_name", "creditor_account",
        ];
        let req = ReadRequest {
            path: Some(Self::db_path("payment_journeys")),
            column: columns.iter().map(|c| c.to_string()).collect(),
            where_clause: vec![Fld { name: "journey_id".into(), relation: None, value: journey_id.into() }],
            page: 0, page_size: Some(1),
        };
        let resp = crud.read(req).await?.into_inner();
        let Some(row) = resp.data.into_iter().next() else { return Ok(None); };
        let cols = row.list;

        Ok(Some(JourneyData {
            journey_id:        cols.first().cloned().unwrap_or_default(),
            current_status:    cols.get(1).cloned().unwrap_or_default(),
            uetr:              cols.get(2).cloned().and_then(|s| H::ne(&Some(s))),
            tx_id:             cols.get(3).cloned().and_then(|s| H::ne(&Some(s))),
            end_to_end_id:     cols.get(4).cloned().and_then(|s| H::ne(&Some(s))),
            clr_sys_ref:       cols.get(5).cloned().and_then(|s| H::ne(&Some(s))),
            settlement_amount: cols.get(6).cloned().and_then(|s| H::ne(&Some(s))),
            settlement_ccy:    cols.get(7).cloned().and_then(|s| H::ne(&Some(s))),
            settlement_date:   cols.get(8).cloned().and_then(|s| H::ne(&Some(s))),
            instructing_agent: cols.get(9).cloned().and_then(|s| H::ne(&Some(s))),
            instructed_agent:  cols.get(10).cloned().and_then(|s| H::ne(&Some(s))),
            debtor_agent:      cols.get(11).cloned().and_then(|s| H::ne(&Some(s))),
            creditor_agent:    cols.get(12).cloned().and_then(|s| H::ne(&Some(s))),
            initiating_msg_id: cols.get(13).cloned().and_then(|s| H::ne(&Some(s))),
            initiating_msg_type: cols.get(14).cloned().and_then(|s| H::ne(&Some(s))),
            debtor_name:       cols.get(15).cloned().and_then(|s| H::ne(&Some(s))),
            debtor_account:    cols.get(16).cloned().and_then(|s| H::ne(&Some(s))),
            creditor_name:     cols.get(17).cloned().and_then(|s| H::ne(&Some(s))),
            creditor_account:  cols.get(18).cloned().and_then(|s| H::ne(&Some(s))),
        }))
    }

    // ── Build pacs.002 status report ─────────────────────────────────────

    /// Build a pacs.002 reporting the current status of a journey.
    /// Used in response to pacs.028 (status request) or after processing pacs.008.
    pub fn build_pacs002(
        jd: &JourneyData,
        status_code: &str,
        reason_code: Option<&str>,
        sender_bic: &str,
        receiver_bic: &str,
    ) -> Result<String, String> {
        let now = Utc::now();
        let msg_id = outbound_msg_id("PACS002", now.timestamp_millis());

        let mut tx = pacs002::PaymentTransaction164 {
            orgnl_end_to_end_id: jd.end_to_end_id.clone(),
            orgnl_tx_id:         jd.tx_id.clone(),
            orgnl_uetr:          jd.uetr.clone(),
            tx_sts:              Some(status_code.to_string()),
            ..Default::default()
        };

        if let Some(rc) = reason_code {
            tx.sts_rsn_inf.push(pacs002::StatusReasonInformation14 {
                rsn: Some(pacs002::StatusReason6Choice {
                    cd: Some(rc.to_string()),
                    ..Default::default()
                }),
                ..Default::default()
            });
        }

        let grp = pacs002::OriginalGroupHeader22 {
            orgnl_msg_id:    jd.initiating_msg_id.clone().unwrap_or_else(|| "UNKNOWN".into()),
            orgnl_msg_nm_id: jd.initiating_msg_type.clone().unwrap_or_else(|| "pacs.008.001.13".into()),
            grp_sts:         Some(status_code.to_string()),
            ..Default::default()
        };

        let hdr = pacs002::GroupHeader120 {
            msg_id: msg_id.clone(),
            cre_dt_tm: now,
            instg_agt: Some(bfii(sender_bic)),
            instd_agt: Some(bfii(receiver_bic)),
            ..Default::default()
        };

        let body = pacs002::FIToFIPaymentStatusReportV15 {
            grp_hdr: hdr,
            orgnl_grp_inf_and_sts: vec![grp],
            tx_inf_and_sts: vec![tx],
            ..Default::default()
        };

        let doc = pacs002::Document::new(body);
        xml_to_string(&doc).map_err(|e| format!("pacs.002 serialization failed: {e}"))
    }

    // ── Build pacs.008 credit transfer ───────────────────────────────────

    /// Build a pacs.008 from a journey that was initiated by pain.001.
    ///
    /// Populates `Dbtr`/`Cdtr` (name) and, where the originating pain.001 (or
    /// inbound pacs.008) supplied an account, `DbtrAcct`/`CdtrAcct` (IBAN or
    /// proprietary account id) — see `PartyData` / `batch::extract_pain001_txs`.
    /// `DbtrAgt`/`CdtrAgt` fall back to the sender/receiver BIC of this hop when
    /// the journey has no more specific agent on record. A pacs.008 with empty
    /// party blocks is schema-valid but will be rejected by real clearing
    /// networks, so real names are required here — `NOTPROVIDED` is used only
    /// as a last-resort placeholder when the source message carried none.
    pub fn build_pacs008(
        jd: &JourneyData,
        sender_bic: &str,
        receiver_bic: &str,
    ) -> Result<String, String> {
        let now = Utc::now();
        let msg_id = outbound_msg_id("PACS008", now.timestamp_millis());

        let amount = jd.settlement_amount.as_deref()
            .and_then(|s| s.parse::<Decimal>().ok())
            .ok_or_else(|| format!(
                "pacs.008 build failed: journey {} has no parseable settlement_amount ({:?})",
                jd.journey_id, jd.settlement_amount
            ))?;

        if amount <= Decimal::ZERO {
            return Err(format!(
                "pacs.008 build failed: journey {} has non-positive amount {}",
                jd.journey_id, amount
            ));
        }

        let ccy = jd.settlement_ccy.clone().ok_or_else(|| format!(
            "pacs.008 build failed: journey {} has no settlement_ccy",
            jd.journey_id
        ))?;
        let sttlm_dt = jd.settlement_date.as_deref()
            .and_then(|s| chrono::NaiveDate::parse_from_str(s, "%Y-%m-%d").ok());

        let pmt_id = pacs008::PaymentIdentification13 {
            end_to_end_id: jd.end_to_end_id.clone().unwrap_or_else(|| "NOTPROVIDED".into()),
            tx_id:         jd.tx_id.clone(),
            uetr:          jd.uetr.clone(),
            clr_sys_ref:   jd.clr_sys_ref.clone(),
            ..Default::default()
        };

        let dbtr_agt_bic = jd.debtor_agent.as_deref().unwrap_or(sender_bic);
        let cdtr_agt_bic = jd.creditor_agent.as_deref().unwrap_or(receiver_bic);

        let tx = pacs008::CreditTransferTransaction70 {
            pmt_id,
            intr_bk_sttlm_amt: pacs008::ActiveCurrencyAndAmount {
                ccy: ccy.clone(),
                value: amount,
            },
            intr_bk_sttlm_dt: sttlm_dt,
            dbtr: party_id_p8(jd.debtor_name.as_deref()),
            dbtr_acct: jd.debtor_account.as_deref().map(cash_account_p8),
            dbtr_agt: bfii_p8(dbtr_agt_bic),
            cdtr_agt: bfii_p8(cdtr_agt_bic),
            cdtr: party_id_p8(jd.creditor_name.as_deref()),
            cdtr_acct: jd.creditor_account.as_deref().map(cash_account_p8),
            ..Default::default()
        };

        let hdr = pacs008::GroupHeader131 {
            msg_id: msg_id.clone(),
            cre_dt_tm: now,
            nb_of_txs: "1".into(),
            ttl_intr_bk_sttlm_amt: Some(pacs008::ActiveCurrencyAndAmount {
                ccy, value: amount,
            }),
            intr_bk_sttlm_dt: sttlm_dt,
            sttlm_inf: pacs008::SettlementInstruction15 {
                sttlm_mtd: pacs008::SettlementMethod1Code::Inga,
                ..Default::default()
            },
            instg_agt: Some(bfii_p8(sender_bic)),
            instd_agt: Some(bfii_p8(receiver_bic)),
            ..Default::default()
        };

        let body = pacs008::FIToFICustomerCreditTransferV13 {
            grp_hdr: hdr,
            cdt_trf_tx_inf: vec![tx],
            ..Default::default()
        };

        let doc = pacs008::Document::new(body);
        xml_to_string(&doc).map_err(|e| format!("pacs.008 serialization failed: {e}"))
    }

    // ── Build pacs.004 payment return ────────────────────────────────────

    pub fn build_pacs004(
        jd: &JourneyData,
        reason_code: &str,
        sender_bic: &str,
        receiver_bic: &str,
    ) -> Result<String, String> {
        use crate::messages::pacs00400114 as m;
        let now = Utc::now();
        let msg_id = outbound_msg_id("PACS004", now.timestamp_millis());

        let amount = jd.settlement_amount.as_deref()
            .and_then(|s| s.parse::<Decimal>().ok())
            .ok_or_else(|| format!(
                "pacs.004 build failed: journey {} has no parseable settlement_amount ({:?})",
                jd.journey_id, jd.settlement_amount
            ))?;

        if amount <= Decimal::ZERO {
            return Err(format!(
                "pacs.004 build failed: journey {} has non-positive amount {}",
                jd.journey_id, amount
            ));
        }

        let ccy = jd.settlement_ccy.clone().ok_or_else(|| format!(
            "pacs.004 build failed: journey {} has no settlement_ccy",
            jd.journey_id
        ))?;

        let mut tx = m::PaymentTransaction163 {
            orgnl_end_to_end_id: jd.end_to_end_id.clone(),
            orgnl_tx_id:         jd.tx_id.clone(),
            orgnl_uetr:          jd.uetr.clone(),
            orgnl_clr_sys_ref:   jd.clr_sys_ref.clone(),
            rtrd_intr_bk_sttlm_amt: m::ActiveCurrencyAndAmount { ccy, value: amount },
            ..Default::default()
        };
        tx.rtr_rsn_inf.push(m::PaymentReturnReason7 {
            rsn: Some(m::ReturnReason5Choice { cd: Some(reason_code.into()), ..Default::default() }),
            ..Default::default()
        });

        let body = m::PaymentReturnV14 {
            grp_hdr: m::GroupHeader123 {
                msg_id: msg_id.clone(), cre_dt_tm: now, nb_of_txs: "1".into(),
                instg_agt: Some(m::BranchAndFinancialInstitutionIdentification8 {
                    fin_instn_id: m::FinancialInstitutionIdentification23 { bicfi: Some(sender_bic.to_string()), ..Default::default() },
                    ..Default::default()
                }),
                instd_agt: Some(m::BranchAndFinancialInstitutionIdentification8 {
                    fin_instn_id: m::FinancialInstitutionIdentification23 { bicfi: Some(receiver_bic.to_string()), ..Default::default() },
                    ..Default::default()
                }),
                ..Default::default()
            },
            orgnl_grp_inf: Some(m::OriginalGroupHeader19 {
                orgnl_msg_id: jd.initiating_msg_id.clone().unwrap_or_default(),
                orgnl_msg_nm_id: jd.initiating_msg_type.clone().unwrap_or_else(|| "pacs.008.001.13".into()),
                ..Default::default()
            }),
            tx_inf: vec![tx],
            ..Default::default()
        };

        let doc = m::Document::new(body);
        xml_to_string(&doc).map_err(|e| format!("pacs.004 serialization failed: {e}"))
    }
}

// ── BIC helper: build BranchAndFinancialInstitutionIdentification8 ───────

fn bfii(bic: &str) -> pacs002::BranchAndFinancialInstitutionIdentification8 {
    pacs002::BranchAndFinancialInstitutionIdentification8 {
        fin_instn_id: pacs002::FinancialInstitutionIdentification23 {
            bicfi: Some(bic.to_string()),
            ..Default::default()
        },
        ..Default::default()
    }
}

fn bfii_p8(bic: &str) -> pacs008::BranchAndFinancialInstitutionIdentification8 {
    pacs008::BranchAndFinancialInstitutionIdentification8 {
        fin_instn_id: pacs008::FinancialInstitutionIdentification23 {
            bicfi: Some(bic.to_string()),
            ..Default::default()
        },
        ..Default::default()
    }
}

// ── Party helpers: build PartyIdentification272 / CashAccount40 ─────────

/// Build a minimal `PartyIdentification272` from a name captured off the
/// originating message. `Dbtr`/`Cdtr` are mandatory elements on pacs.008, so
/// even without a name we must emit the element — `NOTPROVIDED` documents
/// that the upstream message/journey carried no party name rather than
/// silently emitting an empty, semantically void element.
fn party_id_p8(name: Option<&str>) -> pacs008::PartyIdentification272 {
    pacs008::PartyIdentification272 {
        nm: Some(name.unwrap_or("NOTPROVIDED").to_string()),
        ..Default::default()
    }
}

/// Build a `CashAccount40` from an account identifier, using the IBAN slot
/// when the value is IBAN-shaped and the proprietary `Othr/Id` slot otherwise.
fn cash_account_p8(account: &str) -> pacs008::CashAccount40 {
    let id = if crate::messages::regex_utils::IBAN2007_IDENTIFIER_REGEX.is_match(account) {
        pacs008::AccountIdentification4Choice { iban: Some(account.to_string()), ..Default::default() }
    } else {
        pacs008::AccountIdentification4Choice {
            othr: Some(pacs008::GenericAccountIdentification1 { id: account.to_string(), ..Default::default() }),
            ..Default::default()
        }
    };
    pacs008::CashAccount40 { id: Some(id), ..Default::default() }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn base_jd() -> JourneyData {
        JourneyData {
            journey_id: "JRN-1".into(),
            current_status: "SETTLED".into(),
            uetr: Some("uetr-1".into()),
            tx_id: Some("tx-1".into()),
            end_to_end_id: Some("e2e-1".into()),
            clr_sys_ref: None,
            settlement_amount: Some("100.50".into()),
            settlement_ccy: Some("EUR".into()),
            settlement_date: Some("2026-01-15".into()),
            instructing_agent: None,
            instructed_agent: None,
            debtor_agent: None,
            creditor_agent: None,
            initiating_msg_id: Some("MSG-1".into()),
            initiating_msg_type: Some("pain.001.001.12".into()),
            debtor_name: Some("Alice".into()),
            debtor_account: Some("BE68539007547034".into()),
            creditor_name: Some("Bob".into()),
            creditor_account: Some("PROP-999".into()),
        }
    }

    // ── party/account helpers ─────────────────────────────────────────────

    #[test]
    fn party_id_p8_uses_provided_name() {
        let p = party_id_p8(Some("Alice"));
        assert_eq!(p.nm.as_deref(), Some("Alice"));
    }

    #[test]
    fn party_id_p8_defaults_to_notprovided_when_missing() {
        let p = party_id_p8(None);
        assert_eq!(p.nm.as_deref(), Some("NOTPROVIDED"));
    }

    #[test]
    fn cash_account_p8_routes_iban_shaped_value_to_iban_slot() {
        let acct = cash_account_p8("BE68539007547034");
        let id = acct.id.unwrap();
        assert_eq!(id.iban.as_deref(), Some("BE68539007547034"));
        assert!(id.othr.is_none());
    }

    #[test]
    fn cash_account_p8_routes_non_iban_value_to_othr_slot() {
        let acct = cash_account_p8("PROP-999");
        let id = acct.id.unwrap();
        assert!(id.iban.is_none());
        assert_eq!(id.othr.unwrap().id, "PROP-999");
    }

    #[test]
    fn bfii_helpers_set_bicfi() {
        assert_eq!(bfii("SENDERBIC").fin_instn_id.bicfi.as_deref(), Some("SENDERBIC"));
        assert_eq!(bfii_p8("SENDERBIC").fin_instn_id.bicfi.as_deref(), Some("SENDERBIC"));
    }

    // ── build_pacs002 ──────────────────────────────────────────────────────

    #[test]
    fn build_pacs002_includes_status_and_original_refs() {
        let jd = base_jd();
        let xml = OutboundBuilder::build_pacs002(&jd, "ACSC", None, "SENDERBIC", "RECEIVERBIC").unwrap();
        assert!(xml.contains("ACSC"));
        assert!(xml.contains("e2e-1"));
        assert!(xml.contains("tx-1"));
        assert!(xml.contains("uetr-1"));
        assert!(xml.contains("SENDERBIC"));
        assert!(xml.contains("RECEIVERBIC"));
    }

    #[test]
    fn build_pacs002_includes_reason_code_when_provided() {
        let jd = base_jd();
        let xml = OutboundBuilder::build_pacs002(&jd, "RJCT", Some("AM04"), "SENDERBIC", "RECEIVERBIC").unwrap();
        assert!(xml.contains("AM04"));
    }

    #[test]
    fn build_pacs002_falls_back_to_unknown_msg_id_when_missing() {
        let mut jd = base_jd();
        jd.initiating_msg_id = None;
        let xml = OutboundBuilder::build_pacs002(&jd, "ACSC", None, "SENDERBIC", "RECEIVERBIC").unwrap();
        assert!(xml.contains("UNKNOWN"));
    }

    // ── build_pacs008 ──────────────────────────────────────────────────────

    #[test]
    fn build_pacs008_happy_path_contains_amount_ccy_and_parties() {
        let jd = base_jd();
        let xml = OutboundBuilder::build_pacs008(&jd, "SENDERBIC", "RECEIVERBIC").unwrap();
        assert!(xml.contains("100.50") || xml.contains("100.5"));
        assert!(xml.contains("EUR"));
        assert!(xml.contains("Alice"));
        assert!(xml.contains("Bob"));
        assert!(xml.contains("BE68539007547034"));
        assert!(xml.contains("PROP-999"));
    }

    #[test]
    fn build_pacs008_falls_back_to_sender_receiver_bic_when_no_agent_on_journey() {
        let jd = base_jd();
        let xml = OutboundBuilder::build_pacs008(&jd, "SENDERBIC", "RECEIVERBIC").unwrap();
        assert!(xml.contains("SENDERBIC"));
        assert!(xml.contains("RECEIVERBIC"));
    }

    #[test]
    fn build_pacs008_prefers_journey_agent_over_hop_bic_when_present() {
        let mut jd = base_jd();
        jd.debtor_agent = Some("DEBTORAGENTBIC".into());
        jd.creditor_agent = Some("CREDITORAGENTBIC".into());
        let xml = OutboundBuilder::build_pacs008(&jd, "SENDERBIC", "RECEIVERBIC").unwrap();
        assert!(xml.contains("DEBTORAGENTBIC"));
        assert!(xml.contains("CREDITORAGENTBIC"));
    }

    #[test]
    fn build_pacs008_rejects_missing_amount() {
        let mut jd = base_jd();
        jd.settlement_amount = None;
        assert!(OutboundBuilder::build_pacs008(&jd, "S", "R").is_err());
    }

    #[test]
    fn build_pacs008_rejects_unparseable_amount() {
        let mut jd = base_jd();
        jd.settlement_amount = Some("not-a-number".into());
        assert!(OutboundBuilder::build_pacs008(&jd, "S", "R").is_err());
    }

    #[test]
    fn build_pacs008_rejects_zero_amount() {
        let mut jd = base_jd();
        jd.settlement_amount = Some("0".into());
        assert!(OutboundBuilder::build_pacs008(&jd, "S", "R").is_err());
    }

    #[test]
    fn build_pacs008_rejects_negative_amount() {
        let mut jd = base_jd();
        jd.settlement_amount = Some("-5.00".into());
        assert!(OutboundBuilder::build_pacs008(&jd, "S", "R").is_err());
    }

    #[test]
    fn build_pacs008_rejects_missing_currency() {
        let mut jd = base_jd();
        jd.settlement_ccy = None;
        assert!(OutboundBuilder::build_pacs008(&jd, "S", "R").is_err());
    }

    #[test]
    fn build_pacs008_defaults_end_to_end_id_when_missing() {
        let mut jd = base_jd();
        jd.end_to_end_id = None;
        let xml = OutboundBuilder::build_pacs008(&jd, "S", "R").unwrap();
        assert!(xml.contains("NOTPROVIDED"));
    }

    #[test]
    fn build_pacs008_tolerates_unparseable_settlement_date() {
        let mut jd = base_jd();
        jd.settlement_date = Some("not-a-date".into());
        // Should not error - the date is simply omitted, not fatal.
        assert!(OutboundBuilder::build_pacs008(&jd, "S", "R").is_ok());
    }

    // ── build_pacs004 ──────────────────────────────────────────────────────

    #[test]
    fn build_pacs004_happy_path_contains_reason_and_refs() {
        let jd = base_jd();
        let xml = OutboundBuilder::build_pacs004(&jd, "AC04", "SENDERBIC", "RECEIVERBIC").unwrap();
        assert!(xml.contains("AC04"));
        assert!(xml.contains("e2e-1"));
        assert!(xml.contains("uetr-1"));
        assert!(xml.contains("EUR"));
    }

    #[test]
    fn build_pacs004_rejects_non_positive_amount() {
        let mut jd = base_jd();
        jd.settlement_amount = Some("0".into());
        assert!(OutboundBuilder::build_pacs004(&jd, "AC04", "S", "R").is_err());
    }

    #[test]
    fn build_pacs004_rejects_missing_currency() {
        let mut jd = base_jd();
        jd.settlement_ccy = None;
        assert!(OutboundBuilder::build_pacs004(&jd, "AC04", "S", "R").is_err());
    }

    // ── MsgId length ────────────────────────────────────────────────────────

    /// ISO 20022 types MsgId as Max35Text. The generated Rust aliases that to a
    /// plain String with no length check, and schema validation runs on the
    /// inbound path only - so nothing else in this service will catch a MsgId
    /// that is too long. This is the check.
    #[test]
    fn outbound_msg_ids_fit_max35text() {
        for prefix in ["PACS002", "PACS008", "PACS004"] {
            // A far-future timestamp, so this keeps holding as millis gains a
            // digit rather than passing only for today's clock.
            for millis in [0_i64, 1_786_700_000_000, 99_999_999_999_999] {
                let id = outbound_msg_id(prefix, millis);
                assert!(
                    id.len() <= 35,
                    "MsgId '{id}' is {} chars for prefix {prefix} at {millis}; Max35Text permits 35",
                    id.len()
                );
                assert!(id.starts_with(prefix), "'{id}' must remain identifiable by prefix");
            }
        }
    }

    #[test]
    fn outbound_msg_ids_are_distinct_within_the_same_millisecond() {
        // The millisecond is not enough on its own - two messages built in the
        // same tick must still differ, which is what the random suffix is for.
        let a = outbound_msg_id("PACS008", 1_786_700_000_000);
        let b = outbound_msg_id("PACS008", 1_786_700_000_000);
        assert_ne!(a, b);
    }

    #[test]
    fn the_previous_msg_id_form_would_have_failed_this() {
        // Regression: pins down what was actually wrong, so a future change
        // back to a full UUID fails here rather than at a counterparty.
        let old = format!("PACS002-{}-{}", 1_786_700_000_000_i64, uuid::Uuid::new_v4());
        assert!(old.len() > 35, "the form this replaced was {} chars", old.len());
    }
}
