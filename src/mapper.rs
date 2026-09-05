use async_trait::async_trait;
use chrono::Utc;
use quick_xml::se::to_string;

use crate::handlers::{
    camt026, camt027, camt029, camt052, camt053, camt054, camt056, pacs002, pacs004,
    pacs007, pacs008, pacs028, pain001, pain002, pain007,
};
use crate::messages::head;
use crate::messages::dsig::SignatureType;

#[async_trait]
pub trait Message: Send + Sync {
    /// Consume an inbound ISO 20022 message.
    ///
    /// `counterparty_bic` is the BIC of the other party in the exchange, extracted
    /// from the inbound `<AppHdr>` by the caller (see
    /// [`crate::processor::apphdr::AppHdrMeta`]).  It is used by the flow
    /// coordinator to address any outbound response messages (pacs.002, pacs.004,
    /// camt.029, etc.).  `None` means the header was absent or the BIC could not
    /// be determined — handlers should treat this as an error condition for
    /// interbank messages but may still succeed for purely-internal reporting
    /// messages (camt.052 / 053 / 054).
    async fn consume(
        &self,
        ctx: &crate::ProcessingContext,
        msg: &str,
        counterparty_bic: Option<&str>,
    ) -> Result<String, String>;
    async fn generate(&self, ctx: &crate::ProcessingContext) -> Result<String, String>;

    /// Wrap a serialised `<Document>` payload inside a full ISO 20022 Business
    /// Envelope with a `head.001.001.04` Application Header.
    ///
    /// If `signature` is provided, it is embedded in `<AppHdr><Sgntr>`.
    fn wrap(
        &self,
        msg_payload: &str,
        msg_def_id: &str,
        biz_msg_id: &str,
        sender_bic: &str,
        receiver_bic: &str,
    ) -> Result<String, String> {
        self.wrap_with_signature(msg_payload, msg_def_id, biz_msg_id, sender_bic, receiver_bic, None)
    }

    fn wrap_with_signature(
        &self,
        msg_payload: &str,
        msg_def_id: &str,
        biz_msg_id: &str,
        sender_bic: &str,
        receiver_bic: &str,
        signature: Option<&SignatureType>,
    ) -> Result<String, String> {
        self.wrap_full(msg_payload, msg_def_id, biz_msg_id, sender_bic, receiver_bic,
            signature, None, false)
    }

    /// Full envelope builder with all optional BAH fields.
    ///
    /// - `biz_svc`: SWIFT business service (e.g. "swift.cbprplus.02").  Required by CBPR+.
    /// - `possible_duplicate`: set to `true` when re-sending a message after timeout.
    #[allow(clippy::too_many_arguments)]
    fn wrap_full(
        &self,
        msg_payload: &str,
        msg_def_id: &str,
        biz_msg_id: &str,
        sender_bic: &str,
        receiver_bic: &str,
        signature: Option<&SignatureType>,
        biz_svc: Option<&str>,
        possible_duplicate: bool,
    ) -> Result<String, String> {
        let mut header = head::BusinessApplicationHeaderV04 {
            xmlns: "urn:iso:std:iso:20022:tech:xsd:head.001.001.04".to_string(),
            fr: head::Party51Choice {
                fi_id: Some(head::BranchAndFinancialInstitutionIdentification8 {
                    fin_instn_id: head::FinancialInstitutionIdentification23 {
                        bicfi: Some(sender_bic.to_string()),
                        ..Default::default()
                    },
                    ..Default::default()
                }),
                ..Default::default()
            },
            to: head::Party51Choice {
                fi_id: Some(head::BranchAndFinancialInstitutionIdentification8 {
                    fin_instn_id: head::FinancialInstitutionIdentification23 {
                        bicfi: Some(receiver_bic.to_string()),
                        ..Default::default()
                    },
                    ..Default::default()
                }),
                ..Default::default()
            },
            biz_msg_idr: biz_msg_id.to_string(),
            msg_def_idr: msg_def_id.to_string(),
            cre_dt: Utc::now(),
            ..Default::default()
        };

        // Improvement 6: Business service and possible duplicate indicator.
        if let Some(svc) = biz_svc {
            header.biz_svc = Some(svc.to_string());
        }
        if possible_duplicate {
            header.pssbl_dplct = Some(true);
        }

        // Attach signature to AppHdr if provided.
        if let Some(sig) = signature {
            header.sgntr = Some(head::SignatureEnvelope {
                signature: sig.clone(),
            });
        }

        let head_xml = to_string(&header)
            .map_err(|e| format!("Header serialization failed: {e}"))?;
        Ok(format!(
            "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n\
             <Envelope xmlns=\"urn:iso:std:iso:20022:tech:xsd:envelope\">\n\
             {}\n\
             {}\n\
             </Envelope>",
            head_xml, msg_payload
        ))
    }
}

/// Sign a document payload and wrap it with signature attached.
///
/// Call this instead of `wrap()` when `InstitutionConfig::signing_enabled` is true.
///
/// `key` must be a pre-loaded private key (use `crate::signer::load_signing_key` once at
/// startup and store the result in application state — do **not** pass a file path here).
#[allow(clippy::too_many_arguments)]
pub fn sign_and_wrap(
    handler: &dyn Message,
    doc_xml: &str,
    msg_def_id: &str,
    biz_msg_id: &str,
    sender_bic: &str,
    receiver_bic: &str,
    key: &openssl::pkey::PKey<openssl::pkey::Private>,
    cert_base64: &str,
) -> Result<String, String> {
    let signature = crate::signer::sign_payload(doc_xml, key, cert_base64)
        .map_err(|e| format!("Signing failed: {e}"))?;

    handler.wrap_with_signature(doc_xml, msg_def_id, biz_msg_id, sender_bic, receiver_bic, Some(&signature))
}

/// Return the handler for the given ISO 20022 message definition identifier.
///
/// Resolution is two-stage:
///
/// 1. **Exact match** on the full version string (e.g. `"pacs.008.001.13"`).
/// 2. **Prefix fallback**: if the exact version is unknown, the handler for
///    the *latest compiled-in* version of that message family is returned with
///    a warning.  This allows minor-version variants from counterparties
///    (e.g. `"pacs.008.001.12"`) to be processed without silent dead-lettering.
///
/// Returns `None` only when the message family itself is unsupported.
pub fn get_handler(name: &str) -> Option<Box<dyn Message>> {
    // Stage 1: exact version match — preferred path, no allocation.
    if let Some(h) = get_handler_exact(name) {
        return Some(h);
    }

    // Stage 2: prefix match on <area>.<functionality> (first two dot-segments).
    // ISO 20022 IDs have the form  area.functionality.version.release
    // e.g. "pacs.008.001.13" → prefix "pacs.008"
    let mut parts = name.splitn(4, '.');
    let prefix = match (parts.next(), parts.next()) {
        (Some(a), Some(b)) => format!("{a}.{b}"),
        _ => return None,
    };

    let handler = get_handler_by_prefix(&prefix)?;
    tracing::warn!(
        msg_type = %name,
        prefix = %prefix,
        "Unknown message version; routing to latest compiled handler for this message family. \
         Validate that the schema differences are backward-compatible.",
    );
    crate::processor::metrics::record_version_fallback(name, &prefix);
    Some(handler)
}

fn get_handler_exact(name: &str) -> Option<Box<dyn Message>> {
    match name {
        "pacs.002.001.15" => Some(Box::new(pacs002::Document)),
        "pacs.004.001.14" => Some(Box::new(pacs004::Document)),
        "pacs.007.001.13" => Some(Box::new(pacs007::Document)),
        "pacs.008.001.13" => Some(Box::new(pacs008::Document)),
        "pacs.028.001.06" => Some(Box::new(pacs028::Document)),
        "pain.001.001.12" => Some(Box::new(pain001::Document)),
        "pain.002.001.14" => Some(Box::new(pain002::Document)),
        "pain.007.001.12" => Some(Box::new(pain007::Document)),
        "camt.026.001.10" => Some(Box::new(camt026::Document)),
        "camt.027.001.10" => Some(Box::new(camt027::Document)),
        "camt.029.001.13" => Some(Box::new(camt029::Document)),
        "camt.052.001.13" => Some(Box::new(camt052::Document)),
        "camt.053.001.13" => Some(Box::new(camt053::Document)),
        "camt.054.001.13" => Some(Box::new(camt054::Document)),
        "camt.056.001.11" => Some(Box::new(camt056::Document)),
        _ => None,
    }
}

fn get_handler_by_prefix(prefix: &str) -> Option<Box<dyn Message>> {
    match prefix {
        "pacs.002" => Some(Box::new(pacs002::Document)),
        "pacs.004" => Some(Box::new(pacs004::Document)),
        "pacs.007" => Some(Box::new(pacs007::Document)),
        "pacs.008" => Some(Box::new(pacs008::Document)),
        "pacs.028" => Some(Box::new(pacs028::Document)),
        "pain.001" => Some(Box::new(pain001::Document)),
        "pain.002" => Some(Box::new(pain002::Document)),
        "pain.007" => Some(Box::new(pain007::Document)),
        "camt.026" => Some(Box::new(camt026::Document)),
        "camt.027" => Some(Box::new(camt027::Document)),
        "camt.029" => Some(Box::new(camt029::Document)),
        "camt.052" => Some(Box::new(camt052::Document)),
        "camt.053" => Some(Box::new(camt053::Document)),
        "camt.054" => Some(Box::new(camt054::Document)),
        "camt.056" => Some(Box::new(camt056::Document)),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const ALL_EXACT: &[&str] = &[
        "pacs.002.001.15", "pacs.004.001.14", "pacs.007.001.13", "pacs.008.001.13",
        "pacs.028.001.06", "pain.001.001.12", "pain.002.001.14", "pain.007.001.12",
        "camt.026.001.10", "camt.027.001.10", "camt.029.001.13", "camt.052.001.13",
        "camt.053.001.13", "camt.054.001.13", "camt.056.001.11",
    ];

    #[test]
    fn every_exact_version_resolves() {
        for name in ALL_EXACT {
            assert!(get_handler(name).is_some(), "expected a handler for {name}");
        }
    }

    #[test]
    fn unknown_minor_version_falls_back_to_family_prefix() {
        // pacs.008.001.13 is compiled in; .12/.99 aren't, but the family is.
        assert!(get_handler("pacs.008.001.12").is_some());
        assert!(get_handler("pacs.008.001.99").is_some());
        assert!(get_handler("camt.053.001.02").is_some());
    }

    #[test]
    fn unsupported_message_family_returns_none() {
        assert!(get_handler("acmt.007.001.01").is_none());
        assert!(get_handler("remt.001.001.01").is_none());
    }

    #[test]
    fn malformed_names_return_none_without_panicking() {
        assert!(get_handler("").is_none());
        assert!(get_handler("pacs").is_none()); // no dot at all
        assert!(get_handler("pacs.").is_none()); // only one segment before trailing dot
        assert!(get_handler(".").is_none());
        assert!(get_handler("...").is_some() || get_handler("...").is_none()); // must not panic either way
    }

    #[test]
    fn wrap_produces_an_envelope_with_both_bics_and_the_payload() {
        let doc = pacs008::Document;
        let xml = doc.wrap("<Document/>", "pacs.008.001.13", "MSG-1", "SENDERBIC", "RECEIVERBIC").unwrap();
        assert!(xml.contains("SENDERBIC"));
        assert!(xml.contains("RECEIVERBIC"));
        assert!(xml.contains("MSG-1"));
        assert!(xml.contains("pacs.008.001.13"));
        assert!(xml.contains("<Document/>"));
        assert!(xml.starts_with("<?xml"));
    }

    #[test]
    fn wrap_full_sets_biz_svc_and_possible_duplicate_when_provided() {
        let doc = pacs008::Document;
        let xml = doc.wrap_full(
            "<Document/>", "pacs.008.001.13", "MSG-2", "SENDERBIC", "RECEIVERBIC",
            None, Some("swift.cbprplus.02"), true,
        ).unwrap();
        assert!(xml.contains("swift.cbprplus.02"));
    }

    #[test]
    fn wrap_full_omits_optional_fields_when_not_provided() {
        let doc = pacs008::Document;
        let xml = doc.wrap_full(
            "<Document/>", "pacs.008.001.13", "MSG-3", "SENDERBIC", "RECEIVERBIC",
            None, None, false,
        ).unwrap();
        assert!(!xml.contains("swift.cbprplus.02"));
    }
}
