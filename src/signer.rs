use anyhow::{anyhow, Context, Result};
use base64::{engine::general_purpose::STANDARD as BASE64, Engine};
use libxml::parser::Parser;
use libxml::tree::c14n::{CanonicalizationMode, CanonicalizationOptions};
use libxml::tree::node::Node;
use openssl::{hash::MessageDigest, pkey::PKey, pkey::Private, sign::Signer};
use quick_xml::se::to_string;
use std::fs;

use crate::messages::dsig;
use crate::xml_utils::{
    find_node_recursive, ALG_C14N_EXCL, ALG_ENVELOPED_SIG, ALG_RSA_SHA256, ALG_SHA256,
    XMLDSIG_NS,
};

/// Load a PEM private key from disk with an optional passphrase.
///
/// Call this **once at startup** and store the returned `PKey<Private>` in your
/// application state.  Calling it per-message is expensive (disk I/O + PEM
/// decoding) and must be avoided in production.
///
/// Errors include the underlying I/O details to aid debugging.
pub fn load_signing_key(path: &str, password: Option<&str>) -> Result<PKey<Private>> {
    let key_bytes = fs::read(path)
        .with_context(|| format!("Cannot read private key at {path:?}"))?;
    let key = if let Some(pass) = password {
        PKey::private_key_from_pem_passphrase(&key_bytes, pass.as_bytes())
            .context("Failed to decrypt private key (wrong passphrase?)")?
    } else {
        PKey::private_key_from_pem(&key_bytes).context("Failed to parse private key PEM")?
    };
    Ok(key)
}

/// Strip PEM headers/footers and return the raw base64 certificate data.
///
/// Handles both LF and CRLF line endings (`str::lines` normalises both).
/// Returns an error if the result is empty (corrupt or non-PEM file).
pub fn load_public_cert(path: &str) -> Result<String> {
    let cert_content = fs::read_to_string(path)
        .with_context(|| format!("Cannot read certificate at {path:?}"))?;
    let raw_base64: String = cert_content
        .lines()
        .filter(|line| !line.starts_with("-----"))
        .collect();
    if raw_base64.trim().is_empty() {
        anyhow::bail!(
            "Certificate at {path:?} contains no base64 data after stripping PEM headers"
        );
    }
    Ok(raw_base64)
}

fn c14n_exclusive(node: &mut Node) -> Result<String> {
    node.canonicalize(CanonicalizationOptions {
        mode: CanonicalizationMode::ExclusiveCanonical1_0,
        with_comments: false,
        inclusive_ns_prefixes: vec![],
    })
    .map_err(|_| anyhow!("Exclusive C14N canonicalization failed"))
}

fn canonicalize_document_subtree(doc: &libxml::tree::Document) -> Result<String> {
    let root = doc
        .get_root_element()
        .ok_or_else(|| anyhow!("Document has no root element"))?;
    let mut document_node = find_node_recursive(&root, "Document")
        .ok_or_else(|| anyhow!("<Document> element not found; refusing to sign wrong subtree"))?;
    c14n_exclusive(&mut document_node)
}

fn canonicalize_signed_info_xml(signed_info_xml: &str) -> Result<String> {
    let parser = Parser::default();
    let doc = parser
        .parse_string(signed_info_xml)
        .map_err(|_| anyhow!("Failed to parse SignedInfo XML for C14N"))?;
    let mut root = doc
        .get_root_element()
        .ok_or_else(|| anyhow!("No root element in SignedInfo XML"))?;
    c14n_exclusive(&mut root)
}

/// Sign a document payload and return an XMLDSig `<Signature>` structure.
///
/// # Parameters
/// - `doc_raw`            – raw XML of the message to sign
/// - `key`                – pre-loaded signing key (use `load_signing_key` once at startup)
/// - `public_cert_base64` – PEM base64 (headers stripped) embedded in `<X509Certificate>`
///
/// # Signature scheme
/// Exclusive C14N (exc-c14n) over the `<Document>` subtree, SHA-256 digest,
/// RSA-SHA256 signature over the canonicalized `<SignedInfo>`.
pub fn sign_payload(
    doc_raw: &str,
    key: &PKey<Private>,
    public_cert_base64: &str,
) -> Result<dsig::SignatureType> {
    if public_cert_base64.trim().is_empty() {
        anyhow::bail!("`public_cert_base64` must not be empty");
    }

    let parser = Parser::default();
    let doc = parser
        .parse_string(doc_raw)
        .map_err(|_| anyhow!("Failed to parse document XML for signing"))?;

    // Digest over canonicalized <Document> subtree (exclusive C14N).
    let doc_c14n = canonicalize_document_subtree(&doc)?;
    let mut hasher = openssl::hash::Hasher::new(MessageDigest::sha256())
        .context("SHA-256 hasher initialisation failed")?;
    hasher
        .update(doc_c14n.as_bytes())
        .context("Hasher update failed")?;
    let digest_bytes = hasher.finish().context("Hasher finish failed")?;
    let digest_base64 = BASE64.encode(digest_bytes);

    // Build SignedInfo.
    let si = dsig::SignedInfoType {
        canonicalization_method: dsig::CanonicalizationMethodType {
            algorithm: ALG_C14N_EXCL.to_string(),
        },
        signature_method: dsig::SignatureMethodType {
            algorithm: ALG_RSA_SHA256.to_string(),
            hmac_output_length: None,
        },
        reference: vec![dsig::ReferenceType {
            uri: Some("".to_string()),
            transforms: Some(dsig::TransformsType {
                transform: vec![
                    dsig::TransformType {
                        algorithm: ALG_ENVELOPED_SIG.to_string(),
                        ..Default::default()
                    },
                    dsig::TransformType {
                        algorithm: ALG_C14N_EXCL.to_string(),
                        ..Default::default()
                    },
                ],
            }),
            digest_method: dsig::DigestMethodType {
                algorithm: ALG_SHA256.to_string(),
            },
            digest_value: digest_base64,
            ..Default::default()
        }],
        ..Default::default()
    };

    let signed_info_xml =
        to_string(&si).map_err(|e| anyhow!("Failed to serialise SignedInfo to XML: {e}"))?;

    // Canonicalize SignedInfo before signing (exclusive C14N over the serialised element).
    let signed_info_c14n = canonicalize_signed_info_xml(&signed_info_xml)?;

    let mut signer =
        Signer::new(MessageDigest::sha256(), key).context("Failed to create OpenSSL Signer")?;
    signer
        .update(signed_info_c14n.as_bytes())
        .context("Signer update failed")?;
    let sig_bytes = signer.sign_to_vec().context("Signing operation failed")?;
    let signature_base64 = BASE64.encode(sig_bytes);

    Ok(dsig::SignatureType {
        xmlns: XMLDSIG_NS.to_string(),
        xmlns_ds: XMLDSIG_NS.to_string(),
        signed_info: si,
        signature_value: dsig::SignatureValueType {
            value: signature_base64,
            ..Default::default()
        },
        key_info: Some(dsig::KeyInfoType {
            x509_data: Some(dsig::X509DataType {
                x509_certificate: public_cert_base64.to_string(),
                ..Default::default()
            }),
            ..Default::default()
        }),
        ..Default::default()
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use openssl::rsa::Rsa;
    use openssl::symm::Cipher;
    use std::path::PathBuf;

    fn scratch_path(name: &str) -> PathBuf {
        std::env::temp_dir().join(format!("iso20022-signer-test-{}-{name}", uuid::Uuid::new_v4()))
    }

    fn gen_key() -> PKey<Private> {
        let rsa = Rsa::generate(2048).expect("rsa keygen");
        PKey::from_rsa(rsa).expect("pkey from rsa")
    }

    // ── load_signing_key ────────────────────────────────────────────────

    #[test]
    fn load_signing_key_fails_for_missing_file() {
        let err = load_signing_key("/nonexistent/path/key.pem", None).unwrap_err();
        assert!(err.to_string().contains("Cannot read private key"));
    }

    #[test]
    fn load_signing_key_reads_unencrypted_pem() {
        let key = gen_key();
        let pem = key.private_key_to_pem_pkcs8().unwrap();
        let path = scratch_path("plain.pem");
        std::fs::write(&path, &pem).unwrap();

        let loaded = load_signing_key(path.to_str().unwrap(), None);
        std::fs::remove_file(&path).ok();
        assert!(loaded.is_ok());
    }

    #[test]
    fn load_signing_key_decrypts_with_correct_passphrase() {
        let key = gen_key();
        let pem = key.private_key_to_pem_pkcs8_passphrase(Cipher::aes_256_cbc(), b"correct-horse").unwrap();
        let path = scratch_path("encrypted.pem");
        std::fs::write(&path, &pem).unwrap();

        let loaded = load_signing_key(path.to_str().unwrap(), Some("correct-horse"));
        std::fs::remove_file(&path).ok();
        assert!(loaded.is_ok());
    }

    #[test]
    fn load_signing_key_fails_with_wrong_passphrase() {
        let key = gen_key();
        let pem = key.private_key_to_pem_pkcs8_passphrase(Cipher::aes_256_cbc(), b"correct-horse").unwrap();
        let path = scratch_path("wrongpass.pem");
        std::fs::write(&path, &pem).unwrap();

        let loaded = load_signing_key(path.to_str().unwrap(), Some("wrong-passphrase"));
        std::fs::remove_file(&path).ok();
        assert!(loaded.is_err());
    }

    // ── load_public_cert ────────────────────────────────────────────────

    #[test]
    fn load_public_cert_fails_for_missing_file() {
        let err = load_public_cert("/nonexistent/path/cert.pem").unwrap_err();
        assert!(err.to_string().contains("Cannot read certificate"));
    }

    #[test]
    fn load_public_cert_strips_pem_headers() {
        let path = scratch_path("cert.pem");
        std::fs::write(&path, "-----BEGIN CERTIFICATE-----\nQUJDRA==\nMORE=\n-----END CERTIFICATE-----\n").unwrap();
        let cert = load_public_cert(path.to_str().unwrap());
        std::fs::remove_file(&path).ok();
        assert_eq!(cert.unwrap(), "QUJDRA==MORE=");
    }

    #[test]
    fn load_public_cert_handles_crlf_line_endings() {
        let path = scratch_path("cert_crlf.pem");
        std::fs::write(&path, "-----BEGIN CERTIFICATE-----\r\nQUJDRA==\r\n-----END CERTIFICATE-----\r\n").unwrap();
        let cert = load_public_cert(path.to_str().unwrap());
        std::fs::remove_file(&path).ok();
        assert_eq!(cert.unwrap(), "QUJDRA==");
    }

    #[test]
    fn load_public_cert_rejects_content_with_no_data_after_stripping() {
        let path = scratch_path("empty_cert.pem");
        std::fs::write(&path, "-----BEGIN CERTIFICATE-----\n-----END CERTIFICATE-----\n").unwrap();
        let cert = load_public_cert(path.to_str().unwrap());
        std::fs::remove_file(&path).ok();
        assert!(cert.is_err());
    }

    // ── sign_payload ────────────────────────────────────────────────────

    const SAMPLE_DOC: &str = r#"<Envelope><AppHdr/><Document><Foo>bar</Foo></Document></Envelope>"#;

    #[test]
    fn sign_payload_rejects_empty_cert() {
        let key = gen_key();
        let err = sign_payload(SAMPLE_DOC, &key, "").unwrap_err();
        assert!(err.to_string().contains("must not be empty"));
        let err_ws = sign_payload(SAMPLE_DOC, &key, "   ").unwrap_err();
        assert!(err_ws.to_string().contains("must not be empty"));
    }

    #[test]
    fn sign_payload_rejects_malformed_xml() {
        // libxml2's default parser is lenient about some things (e.g. an
        // unclosed tag can still parse if there's a plausible structure), but
        // genuinely non-XML input (or empty input) must still fail to parse.
        let key = gen_key();
        let err = sign_payload("", &key, "Y2VydA==").unwrap_err();
        assert!(err.to_string().contains("Failed to parse document XML"));
    }

    #[test]
    fn sign_payload_rejects_xml_without_document_element() {
        let key = gen_key();
        let err = sign_payload("<Envelope><AppHdr/></Envelope>", &key, "Y2VydA==").unwrap_err();
        assert!(err.to_string().contains("refusing to sign wrong subtree"));
    }

    #[test]
    fn sign_payload_produces_expected_algorithms_and_embeds_cert() {
        let key = gen_key();
        let sig = sign_payload(SAMPLE_DOC, &key, "Y2VydA==").unwrap();

        assert_eq!(sig.signed_info.canonicalization_method.algorithm, ALG_C14N_EXCL);
        assert_eq!(sig.signed_info.signature_method.algorithm, ALG_RSA_SHA256);
        assert_eq!(sig.signed_info.reference.len(), 1);
        assert_eq!(sig.signed_info.reference[0].digest_method.algorithm, ALG_SHA256);
        assert!(!sig.signed_info.reference[0].digest_value.is_empty());
        assert!(!sig.signature_value.value.is_empty());
        assert_eq!(
            sig.key_info.unwrap().x509_data.unwrap().x509_certificate,
            "Y2VydA=="
        );
    }

    #[test]
    fn sign_payload_is_deterministic_in_digest_for_the_same_document() {
        let key = gen_key();
        let sig1 = sign_payload(SAMPLE_DOC, &key, "Y2VydA==").unwrap();
        let sig2 = sign_payload(SAMPLE_DOC, &key, "Y2VydA==").unwrap();
        // Same canonicalized <Document> subtree -> same digest both times.
        assert_eq!(sig1.signed_info.reference[0].digest_value, sig2.signed_info.reference[0].digest_value);
    }

    #[test]
    fn sign_payload_digest_changes_when_document_content_changes() {
        let key = gen_key();
        let sig1 = sign_payload(SAMPLE_DOC, &key, "Y2VydA==").unwrap();
        let sig2 = sign_payload(
            r#"<Envelope><AppHdr/><Document><Foo>different</Foo></Document></Envelope>"#,
            &key, "Y2VydA==",
        ).unwrap();
        assert_ne!(sig1.signed_info.reference[0].digest_value, sig2.signed_info.reference[0].digest_value);
    }
}
