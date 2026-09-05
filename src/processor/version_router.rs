use quick_xml::de::from_str;
use tracing::warn;

/// Maps a message definition identifier to the handler name used in `get_handler()`.
///
/// When SWIFT upgrades a message version (e.g. pacs.008.001.13 → pacs.008.001.14),
/// the XSD structures are almost always backward-compatible.  This router lets
/// the new version fall through to the existing handler, so you don't reject
/// every message the day SWIFT publishes a new schema.
///
/// # Policy
///
/// - Exact match: use the handler for that version.
/// - Minor version bump (same base): route to the highest handler we have.
/// - Unknown base: return None (rejected by caller).
///
/// When you regenerate types from the new XSD and add a handler, add the exact
/// match here and remove the fallback.
pub fn resolve_handler_name(msg_def_id: &str) -> Option<&'static str> {
    // Exact matches for versions we have typed handlers for.
    match msg_def_id {
        "pacs.002.001.15" => return Some("pacs.002.001.15"),
        "pacs.004.001.14" => return Some("pacs.004.001.14"),
        "pacs.007.001.13" => return Some("pacs.007.001.13"),
        "pacs.008.001.13" => return Some("pacs.008.001.13"),
        "pacs.028.001.06" => return Some("pacs.028.001.06"),
        "pain.001.001.12" => return Some("pain.001.001.12"),
        "pain.002.001.14" => return Some("pain.002.001.14"),
        "pain.007.001.12" => return Some("pain.007.001.12"),
        "camt.026.001.10" => return Some("camt.026.001.10"),
        "camt.027.001.10" => return Some("camt.027.001.10"),
        "camt.029.001.13" => return Some("camt.029.001.13"),
        "camt.052.001.13" => return Some("camt.052.001.13"),
        "camt.053.001.13" => return Some("camt.053.001.13"),
        "camt.054.001.13" => return Some("camt.054.001.13"),
        "camt.056.001.11" => return Some("camt.056.001.11"),
        _ => {}
    }

    // Fallback: extract base (e.g. "pacs.008") and route to our handler.
    let base = extract_base(msg_def_id)?;
    let fallback = match base {
        "pacs.002" => Some("pacs.002.001.15"),
        "pacs.004" => Some("pacs.004.001.14"),
        "pacs.007" => Some("pacs.007.001.13"),
        "pacs.008" => Some("pacs.008.001.13"),
        "pacs.028" => Some("pacs.028.001.06"),
        "pain.001" => Some("pain.001.001.12"),
        "pain.002" => Some("pain.002.001.14"),
        "pain.007" => Some("pain.007.001.12"),
        "camt.026" => Some("camt.026.001.10"),
        "camt.027" => Some("camt.027.001.10"),
        "camt.029" => Some("camt.029.001.13"),
        "camt.052" => Some("camt.052.001.13"),
        "camt.053" => Some("camt.053.001.13"),
        "camt.054" => Some("camt.054.001.13"),
        "camt.056" => Some("camt.056.001.11"),
        _ => None,
    };

    if let Some(fb) = fallback {
        warn!(
            requested = %msg_def_id,
            fallback = %fb,
            "Version fallback: routing unknown version to existing handler"
        );
    }

    fallback
}

/// Extract the base message name (e.g. "pacs.008" from "pacs.008.001.14").
fn extract_base(msg_def_id: &str) -> Option<&str> {
    // Format: "xxxx.NNN.NNN.NN" — base is first two dot-separated segments.
    let mut dots = 0;
    for (i, ch) in msg_def_id.char_indices() {
        if ch == '.' {
            dots += 1;
            if dots == 2 {
                return Some(&msg_def_id[..i]);
            }
        }
    }
    None
}

/// Attempt deserialization with a lenient fallback.
///
/// First tries strict deserialization.  If that fails, logs a warning and
/// returns the error — callers can decide whether to proceed with raw XML
/// tag extraction as a degraded path.
///
/// # Usage in handlers:
///
/// ```ignore
/// let doc = lenient_deserialize::<m::Document>(msg)?;
/// // If this returns Ok, the full typed struct is available.
/// // If it returns Err, the handler should use the raw XML fallback.
/// ```
pub fn lenient_deserialize<T: serde::de::DeserializeOwned>(xml: &str) -> Result<T, String> {
    match from_str::<T>(xml) {
        Ok(doc) => Ok(doc),
        Err(e) => {
            let err_str = e.to_string();
            // Check if the error is in an optional / supplementary field.
            // These are common when a counterparty includes proprietary extensions.
            if err_str.contains("SplmtryData")
                || err_str.contains("unknown field")
                || err_str.contains("unknown variant")
            {
                warn!(
                    error = %err_str,
                    "Deserialization warning: non-critical field failed, message may still be processable"
                );
            }
            Err(format!("Deserialization failed: {}", err_str))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_exact_match() {
        assert_eq!(resolve_handler_name("pacs.008.001.13"), Some("pacs.008.001.13"));
    }

    #[test]
    fn test_version_fallback() {
        // Future version falls back to current handler.
        assert_eq!(resolve_handler_name("pacs.008.001.14"), Some("pacs.008.001.13"));
        assert_eq!(resolve_handler_name("pacs.002.001.16"), Some("pacs.002.001.15"));
        assert_eq!(resolve_handler_name("camt.056.001.12"), Some("camt.056.001.11"));
    }

    #[test]
    fn test_unknown_base() {
        assert_eq!(resolve_handler_name("admi.007.001.01"), None);
    }

    #[test]
    fn test_extract_base() {
        assert_eq!(extract_base("pacs.008.001.13"), Some("pacs.008"));
        assert_eq!(extract_base("camt.056.001.11"), Some("camt.056"));
        assert_eq!(extract_base("bad"), None);
    }
}
