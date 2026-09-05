use anyhow::{anyhow, Context, Result};
use base64::{engine::general_purpose::STANDARD as BASE64, Engine};
use libxml::parser::Parser;
use libxml::schemas::{SchemaParserContext, SchemaValidationContext};
use libxml::tree::c14n::{CanonicalizationMode, CanonicalizationOptions};
use openssl::hash::{Hasher, MessageDigest};
use openssl::pkey::PKey;
use openssl::sign::Verifier;
use std::cell::RefCell;
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::sync::Arc;

use crate::xml_utils::{find_node_recursive, ALG_C14N_EXCL, ALG_RSA_SHA256, ALG_SHA256};

/// Per-thread cached schema parser together with the identity of the byte
/// buffer it was built from.
///
/// `bytes_ptr` is the raw address of the `Arc<Vec<u8>>` allocation inside
/// [`CompiledSchema`].  When a schema is reloaded at runtime a new `Arc` is
/// constructed, so the pointer changes even if the key string is reused.
/// Comparing pointers here is intentional and safe: we never dereference
/// `bytes_ptr`; we only use it as a cheap identity token.
struct CachedParser {
    context: SchemaParserContext,
    bytes_ptr: usize,
}

thread_local! {
    static TL_SCHEMA_PARSERS: RefCell<HashMap<String, CachedParser>> =
        RefCell::new(HashMap::new());
}

#[derive(Clone)]
pub struct CompiledSchema {
    pub(crate) key: String,
    pub(crate) xsd_bytes: Arc<Vec<u8>>,
}

impl CompiledSchema {
    pub fn load_with_key(key: &str, path: &Path) -> Result<Self> {
        let xsd_bytes =
            fs::read(path).with_context(|| format!("Failed to read XSD file at {:?}", path))?;
        Ok(Self { key: key.to_string(), xsd_bytes: Arc::new(xsd_bytes) })
    }

    /// Validate `xml` against this schema.
    ///
    /// Parses the XML on every call.  For the combined parse-once path that
    /// also runs signature verification, use [`validate_and_verify_once`].
    pub fn validate_xml(&self, xml: &str) -> Result<()> {
        let parser = Parser::default();
        let doc = parser
            .parse_string(xml)
            .map_err(|_| anyhow!("Failed to parse input XML"))?;
        validate_doc_with_schema(&doc, self)
    }
}

/// Combined parse-once path:
/// 1. Parses `xml` once.
/// 2. Validates against `schema` (if provided).
/// 3. Verifies the XMLDSig signature (if a public key is provided).
///
/// Both schema validation and signature verification read algorithm attributes
/// from the document and reject any algorithm that is not explicitly supported.
pub fn validate_and_verify_once(
    xml: &str,
    schema: Option<&CompiledSchema>,
    public_key_pem: Option<&[u8]>,
) -> Result<()> {
    let parser = Parser::default();
    let doc = parser
        .parse_string(xml)
        .map_err(|_| anyhow!("Failed to parse XML"))?;

    if let Some(schema) = schema {
        validate_doc_with_schema(&doc, schema)?;
    }

    if let Some(pubkey) = public_key_pem {
        verify_signature_from_doc(&doc, pubkey)?;
    }

    Ok(())
}

/// Verify only the XMLDSig signature in `xml` (no schema validation).
///
/// Delegates to [`validate_and_verify_once`] so the document is parsed only once.
pub fn verify_signature(xml: &str, public_key_pem: &[u8]) -> Result<()> {
    validate_and_verify_once(xml, None, Some(public_key_pem))
}

// ── Internal helpers ──────────────────────────────────────────────────────────

fn validate_doc_with_schema(doc: &libxml::tree::Document, schema: &CompiledSchema) -> Result<()> {
    TL_SCHEMA_PARSERS.with(|cache| {
        let mut cache = cache.borrow_mut();

        // Evict the cached parser if the schema bytes have been replaced (e.g.
        // hot-reload).  We compare the raw Arc<Vec<u8>> pointer: same allocation
        // → same bytes → reuse; new allocation → bytes changed → rebuild.
        let current_ptr = Arc::as_ptr(&schema.xsd_bytes) as usize;
        let stale = cache.get(&schema.key).map(|e| e.bytes_ptr != current_ptr).unwrap_or(true);
        if stale {
            let spc = SchemaParserContext::from_buffer(schema.xsd_bytes.as_slice());
            cache.insert(schema.key.clone(), CachedParser { context: spc, bytes_ptr: current_ptr });
        }

        let entry = cache
            .get_mut(&schema.key)
            .ok_or_else(|| anyhow!("Internal error: schema parser missing after insert"))?;

        let mut validation_context = SchemaValidationContext::from_parser(&mut entry.context)
            .map_err(|_| anyhow!("Failed to create XSD validation context"))?;

        if let Err(errors) = validation_context.validate_document(doc) {
            let err_msgs: Vec<String> = errors
                .iter()
                .map(|e| e.message.clone().unwrap_or_else(|| "Unknown XSD error".into()))
                .collect();
            return Err(anyhow!("XSD validation failed: {}", err_msgs.join("; ")));
        }
        Ok(())
    })
}

/// Assert that `node` carries an `Algorithm` attribute equal to `expected`.
/// Returns a descriptive error otherwise.
fn check_algorithm(node: &libxml::tree::node::Node, elem: &str, expected: &str) -> Result<()> {
    let alg = node
        .get_attribute("Algorithm")
        .ok_or_else(|| anyhow!("<{elem}> is missing the required Algorithm attribute"))?;
    if alg != expected {
        anyhow::bail!(
            "Unsupported algorithm in <{elem}>: got {alg:?}, expected {expected:?}"
        );
    }
    Ok(())
}

fn verify_signature_from_doc(doc: &libxml::tree::Document, public_key_pem: &[u8]) -> Result<()> {
    let public_key = PKey::public_key_from_pem(public_key_pem)
        .context("Failed to parse public key PEM")?;

    let root = doc
        .get_root_element()
        .ok_or_else(|| anyhow!("Document has no root element"))?;

    // Navigate the expected envelope: <AppHdr><Sgntr><Signature>…
    let mut document_node = find_node_recursive(&root, "Document")
        .ok_or_else(|| anyhow!("No <Document> element found"))?;
    let head_node = find_node_recursive(&root, "AppHdr")
        .ok_or_else(|| anyhow!("No <AppHdr> element found"))?;
    let sgntr_node = find_node_recursive(&head_node, "Sgntr")
        .ok_or_else(|| anyhow!("No <Sgntr> found in <AppHdr>"))?;
    let signature_node = find_node_recursive(&sgntr_node, "Signature")
        .ok_or_else(|| anyhow!("No <Signature> found in <Sgntr>"))?;
    let mut signed_info_node = find_node_recursive(&signature_node, "SignedInfo")
        .ok_or_else(|| anyhow!("No <SignedInfo> found in <Signature>"))?;

    // ── Algorithm validation ──────────────────────────────────────────────────
    // Reject any algorithm other than exclusive C14N + RSA-SHA256 + SHA-256.
    // This prevents algorithm-substitution attacks.
    let c14n_method_node = find_node_recursive(&signed_info_node, "CanonicalizationMethod")
        .ok_or_else(|| anyhow!("No <CanonicalizationMethod> found in <SignedInfo>"))?;
    check_algorithm(&c14n_method_node, "CanonicalizationMethod", ALG_C14N_EXCL)?;

    let sig_method_node = find_node_recursive(&signed_info_node, "SignatureMethod")
        .ok_or_else(|| anyhow!("No <SignatureMethod> found in <SignedInfo>"))?;
    check_algorithm(&sig_method_node, "SignatureMethod", ALG_RSA_SHA256)?;

    let reference_node = find_node_recursive(&signed_info_node, "Reference")
        .ok_or_else(|| anyhow!("No <Reference> found in <SignedInfo>"))?;
    let digest_method_node = find_node_recursive(&reference_node, "DigestMethod")
        .ok_or_else(|| anyhow!("No <DigestMethod> found in <Reference>"))?;
    check_algorithm(&digest_method_node, "DigestMethod", ALG_SHA256)?;

    // ── Digest verification ───────────────────────────────────────────────────
    let digest_value_node = find_node_recursive(&reference_node, "DigestValue")
        .ok_or_else(|| anyhow!("No <DigestValue> found in <Reference>"))?;
    let digest_value_clean = digest_value_node
        .get_content()
        .replace(['\n', '\r', ' '], "");

    let c14n_opts = CanonicalizationOptions {
        mode: CanonicalizationMode::ExclusiveCanonical1_0,
        inclusive_ns_prefixes: vec![],
        with_comments: false,
    };

    let c14n_payload = document_node
        .canonicalize(c14n_opts.clone())
        .map_err(|_| anyhow!("Payload C14N failed"))?;

    let mut hasher =
        Hasher::new(MessageDigest::sha256()).context("SHA-256 hasher initialisation failed")?;
    hasher.update(c14n_payload.as_bytes()).context("Hasher update failed")?;
    let computed_digest_bytes = hasher.finish().context("Hasher finish failed")?;

    // Constant-time comparison — prevents timing side-channel on the digest.
    let expected_digest_bytes = BASE64
        .decode(&digest_value_clean)
        .map_err(|_| anyhow!("Invalid base64 in <DigestValue>"))?;
    if expected_digest_bytes.len() != computed_digest_bytes.len()
        || !openssl::memcmp::eq(&expected_digest_bytes, &computed_digest_bytes)
    {
        return Err(anyhow!(
            "Digest mismatch: document content does not match the signed digest"
        ));
    }

    // ── Signature verification ────────────────────────────────────────────────
    let signature_value_node = find_node_recursive(&signature_node, "SignatureValue")
        .ok_or_else(|| anyhow!("No <SignatureValue> found in <Signature>"))?;
    let signature_clean = signature_value_node
        .get_content()
        .replace(['\n', '\r', ' '], "");
    let signature_bytes = BASE64
        .decode(&signature_clean)
        .map_err(|_| anyhow!("Invalid base64 in <SignatureValue>"))?;

    let c14n_signed_info = signed_info_node
        .canonicalize(c14n_opts)
        .map_err(|_| anyhow!("SignedInfo C14N failed"))?;

    let mut verifier = Verifier::new(MessageDigest::sha256(), &public_key)
        .context("Failed to create OpenSSL Verifier")?;
    verifier
        .update(c14n_signed_info.as_bytes())
        .context("Verifier update failed")?;

    if !verifier.verify(&signature_bytes).context("Signature verification error")? {
        return Err(anyhow!("Signature is invalid"));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::messages::dsig;
    use crate::signer::sign_payload;
    use openssl::rsa::Rsa;

    fn gen_key() -> PKey<openssl::pkey::Private> {
        let rsa = Rsa::generate(2048).expect("rsa keygen");
        PKey::from_rsa(rsa).expect("pkey from rsa")
    }

    /// Sign `document_xml` (must contain a `<Document>` element somewhere
    /// under its root - see `signer.rs`) and return the full envelope with
    /// the resulting `<Signature>` embedded under `<AppHdr><Sgntr>`, plus the
    /// signer's public key PEM to verify against.
    fn sign_and_embed(document_xml_with_placeholder: &str) -> (String, Vec<u8>, dsig::SignatureType) {
        let key = gen_key();
        let pubkey_pem = key.public_key_to_pem().unwrap();

        let unsigned = document_xml_with_placeholder.replace("__SGNTR__", "");
        let sig = sign_payload(&unsigned, &key, "Y2VydA==").unwrap();
        let sig_xml = quick_xml::se::to_string(&sig).unwrap();
        let signed = document_xml_with_placeholder.replace("__SGNTR__", &sig_xml);
        (signed, pubkey_pem, sig)
    }

    const TEMPLATE: &str =
        r#"<Envelope><AppHdr><Sgntr>__SGNTR__</Sgntr></AppHdr><Document><Foo>bar</Foo></Document></Envelope>"#;

    fn embed(sig: &dsig::SignatureType, template: &str) -> String {
        let sig_xml = quick_xml::se::to_string(sig).unwrap();
        template.replace("__SGNTR__", &sig_xml)
    }

    // ── happy path ──────────────────────────────────────────────────────

    #[test]
    fn verify_signature_accepts_a_correctly_signed_document() {
        let (signed_xml, pubkey_pem, _sig) = sign_and_embed(TEMPLATE);
        assert!(verify_signature(&signed_xml, &pubkey_pem).is_ok());
    }

    // ── tamper detection ────────────────────────────────────────────────

    #[test]
    fn verify_signature_rejects_tampered_document_content() {
        let (signed_xml, pubkey_pem, _sig) = sign_and_embed(TEMPLATE);
        let tampered = signed_xml.replace("<Foo>bar</Foo>", "<Foo>tampered</Foo>");
        let err = verify_signature(&tampered, &pubkey_pem).unwrap_err();
        assert!(err.to_string().contains("Digest mismatch"), "unexpected error: {err}");
    }

    #[test]
    fn verify_signature_rejects_tampered_signature_value() {
        let key = gen_key();
        let pubkey_pem = key.public_key_to_pem().unwrap();
        let unsigned = TEMPLATE.replace("__SGNTR__", "");
        let mut sig = sign_payload(&unsigned, &key, "Y2VydA==").unwrap();

        // Flip one character in the base64 signature value - still valid
        // base64 (so it decodes), but the bytes and hence the signature no
        // longer match.
        let mut chars: Vec<char> = sig.signature_value.value.chars().collect();
        chars[0] = if chars[0] == 'A' { 'B' } else { 'A' };
        sig.signature_value.value = chars.into_iter().collect();

        let signed_xml = embed(&sig, TEMPLATE);
        let err = verify_signature(&signed_xml, &pubkey_pem).unwrap_err();
        assert!(err.to_string().contains("Signature is invalid"), "unexpected error: {err}");
    }

    #[test]
    fn verify_signature_rejects_wrong_public_key() {
        let (signed_xml, _pubkey_pem, _sig) = sign_and_embed(TEMPLATE);
        let other_key = gen_key();
        let wrong_pubkey_pem = other_key.public_key_to_pem().unwrap();
        let err = verify_signature(&signed_xml, &wrong_pubkey_pem).unwrap_err();
        assert!(err.to_string().contains("Signature is invalid"), "unexpected error: {err}");
    }

    #[test]
    fn verify_signature_rejects_unsupported_canonicalization_algorithm() {
        let key = gen_key();
        let pubkey_pem = key.public_key_to_pem().unwrap();
        let unsigned = TEMPLATE.replace("__SGNTR__", "");
        let mut sig = sign_payload(&unsigned, &key, "Y2VydA==").unwrap();
        sig.signed_info.canonicalization_method.algorithm = "http://example.com/bogus-c14n".into();

        let signed_xml = embed(&sig, TEMPLATE);
        let err = verify_signature(&signed_xml, &pubkey_pem).unwrap_err();
        assert!(err.to_string().contains("Unsupported algorithm"), "unexpected error: {err}");
    }

    #[test]
    fn verify_signature_rejects_unsupported_signature_method_algorithm() {
        let key = gen_key();
        let pubkey_pem = key.public_key_to_pem().unwrap();
        let unsigned = TEMPLATE.replace("__SGNTR__", "");
        let mut sig = sign_payload(&unsigned, &key, "Y2VydA==").unwrap();
        sig.signed_info.signature_method.algorithm = "http://example.com/bogus-sig-method".into();

        let signed_xml = embed(&sig, TEMPLATE);
        let err = verify_signature(&signed_xml, &pubkey_pem).unwrap_err();
        assert!(err.to_string().contains("Unsupported algorithm"), "unexpected error: {err}");
    }

    #[test]
    fn verify_signature_rejects_invalid_base64_digest_value() {
        let key = gen_key();
        let pubkey_pem = key.public_key_to_pem().unwrap();
        let unsigned = TEMPLATE.replace("__SGNTR__", "");
        let mut sig = sign_payload(&unsigned, &key, "Y2VydA==").unwrap();
        sig.signed_info.reference[0].digest_value = "not valid base64!!".into();

        let signed_xml = embed(&sig, TEMPLATE);
        let err = verify_signature(&signed_xml, &pubkey_pem).unwrap_err();
        assert!(err.to_string().contains("Invalid base64"), "unexpected error: {err}");
    }

    // ── missing structural elements ─────────────────────────────────────

    #[test]
    fn verify_signature_rejects_document_with_no_root() {
        let err = verify_signature("", &[]).unwrap_err();
        assert!(err.to_string().contains("Failed to parse XML"), "unexpected error: {err}");
    }

    #[test]
    fn verify_signature_rejects_missing_document_element() {
        let (signed_xml, pubkey_pem, _sig) = sign_and_embed(TEMPLATE);
        let no_document = signed_xml.replace("<Document><Foo>bar</Foo></Document>", "");
        let err = verify_signature(&no_document, &pubkey_pem).unwrap_err();
        assert!(err.to_string().contains("No <Document> element found"), "unexpected error: {err}");
    }

    #[test]
    fn verify_signature_rejects_missing_apphdr() {
        let (signed_xml, pubkey_pem, _sig) = sign_and_embed(TEMPLATE);
        let no_apphdr = signed_xml.replacen("<AppHdr>", "<NotAppHdr>", 1).replacen("</AppHdr>", "</NotAppHdr>", 1);
        let err = verify_signature(&no_apphdr, &pubkey_pem).unwrap_err();
        assert!(err.to_string().contains("No <AppHdr> element found"), "unexpected error: {err}");
    }

    #[test]
    fn verify_signature_rejects_missing_sgntr() {
        let (signed_xml, pubkey_pem, sig) = sign_and_embed(TEMPLATE);
        let sig_xml = quick_xml::se::to_string(&sig).unwrap();
        let no_sgntr = signed_xml.replace(&format!("<Sgntr>{sig_xml}</Sgntr>"), &sig_xml);
        let err = verify_signature(&no_sgntr, &pubkey_pem).unwrap_err();
        assert!(err.to_string().contains("No <Sgntr> found"), "unexpected error: {err}");
    }

    #[test]
    fn verify_signature_rejects_malformed_public_key_pem() {
        let (signed_xml, _pubkey_pem, _sig) = sign_and_embed(TEMPLATE);
        let err = verify_signature(&signed_xml, b"not a real pem key").unwrap_err();
        assert!(err.to_string().contains("Failed to parse public key PEM"), "unexpected error: {err}");
    }

    // ── validate_and_verify_once combined path ──────────────────────────

    #[test]
    fn validate_and_verify_once_with_no_schema_and_no_key_only_parses() {
        let (signed_xml, _pubkey_pem, _sig) = sign_and_embed(TEMPLATE);
        assert!(validate_and_verify_once(&signed_xml, None, None).is_ok());
    }

    #[test]
    fn validate_and_verify_once_runs_signature_check_when_key_provided() {
        let (signed_xml, pubkey_pem, _sig) = sign_and_embed(TEMPLATE);
        assert!(validate_and_verify_once(&signed_xml, None, Some(&pubkey_pem)).is_ok());

        let tampered = signed_xml.replace("<Foo>bar</Foo>", "<Foo>tampered</Foo>");
        assert!(validate_and_verify_once(&tampered, None, Some(&pubkey_pem)).is_err());
    }
}
