use tracing::warn;

/// Metadata extracted from the ISO 20022 Business Application Header (head.001.001.04).
///
/// This is parsed from the `<AppHdr>` element in the inbound envelope before
/// the handler's `consume()` is called, so that the counterparty BIC and
/// business service are available to the flow coordinator.
#[derive(Debug, Clone, Default)]
pub struct AppHdrMeta {
    /// BIC of the sender (from `<AppHdr><Fr><FIId><FinInstnId><BICFI>`)
    pub sender_bic: Option<String>,
    /// BIC of the receiver (from `<AppHdr><To><FIId><FinInstnId><BICFI>`)
    pub receiver_bic: Option<String>,
    /// Business message identifier
    pub biz_msg_id: Option<String>,
    /// Message definition identifier (e.g. "pacs.008.001.13")
    pub msg_def_id: Option<String>,
    /// Business service (e.g. "swift.cbprplus.02")
    pub biz_svc: Option<String>,
    /// Whether the message is marked as a possible duplicate
    pub possible_duplicate: bool,
    /// Creation date-time of the header
    pub creation_date: Option<String>,
}

impl AppHdrMeta {
    /// Extract AppHdr metadata from raw XML.
    ///
    /// This uses quick_xml tag scanning rather than full deserialization of the
    /// head.001 struct, because:
    /// - The AppHdr may be in a different namespace prefix than expected
    /// - We need this to work even if the Document body fails deserialization
    /// - It's called before handler dispatch, so must be fast
    pub fn extract(xml: &str) -> Self {
        let mut meta = Self::default();

        // Try to find <AppHdr> block boundaries.
        let (Some(start), Some(end)) = (xml.find("<AppHdr"), xml.find("</AppHdr>")) else {
            return meta;
        };
        if end <= start {
            warn!("AppHdr close tag appears before open tag; skipping header extraction");
            return meta;
        }
        let hdr_block = &xml[start..end];

        // Extract Fr BIC (sender).
        meta.sender_bic = Self::extract_nested_bic(hdr_block, "Fr");
        // Extract To BIC (receiver).
        meta.receiver_bic = Self::extract_nested_bic(hdr_block, "To");
        // BizMsgIdr
        meta.biz_msg_id = Self::tag_content(hdr_block, "BizMsgIdr");
        // MsgDefIdr
        meta.msg_def_id = Self::tag_content(hdr_block, "MsgDefIdr");
        // BizSvc
        meta.biz_svc = Self::tag_content(hdr_block, "BizSvc");
        // PssblDplct
        if let Some(pd) = Self::tag_content(hdr_block, "PssblDplct") {
            meta.possible_duplicate = pd.eq_ignore_ascii_case("true");
        }
        // CreDt
        meta.creation_date = Self::tag_content(hdr_block, "CreDt");

        meta
    }

    /// Determine the counterparty BIC relative to our institution.
    ///
    /// If we are the receiver (our BIC matches `receiver_bic`), then the
    /// counterparty is the sender.  Otherwise, the counterparty is the receiver.
    pub fn counterparty_bic(&self, own_bic: &str) -> Option<&str> {
        // Normalize comparison to 8-char BIC (ignore branch code).
        // `get` keeps us char-boundary-safe if own_bic is shorter than 8 bytes
        // or (defensively) contains non-ASCII.
        let own_8 = own_bic.get(..8).unwrap_or(own_bic);

        if let Some(ref recv) = self.receiver_bic {
            if recv.starts_with(own_8) {
                // We're the receiver; counterparty is sender.
                return self.sender_bic.as_deref();
            }
        }
        if let Some(ref send) = self.sender_bic {
            if send.starts_with(own_8) {
                // We're the sender; counterparty is receiver.
                return self.receiver_bic.as_deref();
            }
        }
        // Fallback: assume we're the receiver (inbound message), counterparty is sender.
        self.sender_bic.as_deref()
    }

    // ── Simple tag extraction (does not need full parser) ────────────────

    fn tag_content(xml: &str, tag: &str) -> Option<String> {
        let open = format!("<{tag}>");
        let close = format!("</{tag}>");
        let start = xml.find(&open)? + open.len();
        let end = xml[start..].find(&close)? + start;
        let val = xml[start..end].trim();
        if val.is_empty() || val.starts_with('<') { None } else { Some(val.to_string()) }
    }

    fn extract_nested_bic(xml: &str, party_tag: &str) -> Option<String> {
        // Look for <Fr> or <To> block, then find <BICFI> inside it.
        let open = format!("<{party_tag}>");
        let close = format!("</{party_tag}>");
        let start = xml.find(&open)? + open.len();
        let end = xml[start..].find(&close)? + start;
        let block = &xml[start..end];
        Self::tag_content(block, "BICFI")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_apphdr() {
        let xml = r#"<?xml version="1.0"?>
        <Envelope>
          <AppHdr xmlns="urn:iso:std:iso:20022:tech:xsd:head.001.001.04">
            <Fr><FIId><FinInstnId><BICFI>SENDERBICX</BICFI></FinInstnId></FIId></Fr>
            <To><FIId><FinInstnId><BICFI>RECVRBICXX</BICFI></FinInstnId></FIId></To>
            <BizMsgIdr>MSG-12345</BizMsgIdr>
            <MsgDefIdr>pacs.008.001.13</MsgDefIdr>
            <BizSvc>swift.cbprplus.02</BizSvc>
            <CreDt>2025-01-15T10:30:00Z</CreDt>
          </AppHdr>
          <Document xmlns="urn:iso:std:iso:20022:tech:xsd:pacs.008.001.13">
          </Document>
        </Envelope>"#;

        let meta = AppHdrMeta::extract(xml);
        assert_eq!(meta.sender_bic.as_deref(), Some("SENDERBICX"));
        assert_eq!(meta.receiver_bic.as_deref(), Some("RECVRBICXX"));
        assert_eq!(meta.biz_msg_id.as_deref(), Some("MSG-12345"));
        assert_eq!(meta.msg_def_id.as_deref(), Some("pacs.008.001.13"));
        assert_eq!(meta.biz_svc.as_deref(), Some("swift.cbprplus.02"));
        assert!(!meta.possible_duplicate);
    }

    #[test]
    fn test_counterparty_bic() {
        let meta = AppHdrMeta {
            sender_bic: Some("SENDERBICX".into()),
            receiver_bic: Some("OWNBICXXXX".into()),
            ..Default::default()
        };
        assert_eq!(meta.counterparty_bic("OWNBICXXXX"), Some("SENDERBICX"));
    }
}
