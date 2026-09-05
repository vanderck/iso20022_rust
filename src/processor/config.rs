use std::time::Duration;

/// Institution-level configuration for the ISO 20022 processor.
///
/// In production, load from environment variables, a config file, or a vault.
#[derive(Debug, Clone)]
pub struct InstitutionConfig {
    // ── Identity ─────────────────────────────────────────────────────────
    /// This institution's SWIFT BIC (8 or 11 chars).
    pub own_bic: String,

    /// SWIFT business service identifier for outbound envelopes.
    /// e.g. "swift.cbprplus.02" for CBPR+, "swift.fin.mt" for FIN.
    pub biz_svc: Option<String>,

    // ── Signing ──────────────────────────────────────────────────────────
    pub signing_key_path: Option<String>,
    pub signing_key_password: Option<String>,
    pub signing_cert_base64: Option<String>,
    pub signing_enabled: bool,

    pub verify_pubkey_pem_path: Option<String>,

    // ── Compliance ───────────────────────────────────────────────────────
    pub screening_enabled: bool,
    pub screening_service_url: String,
    /// Bearer token for the screening vendor API. Required by most vendors
    /// (Fircosoft, Dow Jones, ComplyAdvantage, SWIFT Sanctions Screening,
    /// etc.); left unset only for a vendor that authenticates some other way
    /// (mTLS, IP allowlisting).
    pub screening_api_key: Option<String>,
    pub screening_timeout: Duration,
    pub duplicate_check_enabled: bool,

    // ── SLA ──────────────────────────────────────────────────────────────
    pub recall_sla_days: u32,
    pub unable_to_apply_sla_days: u32,
    pub claim_non_receipt_sla_days: u32,
    pub sla_check_interval: Duration,
    /// ISO 3166-1 country code for business day calendar.
    /// Used to exclude weekends and public holidays from SLA calculations.
    pub sla_calendar_country: String,

    // ── Retry / Dead-letter ──────────────────────────────────────────────
    /// Maximum retries before a message moves to DEAD_LETTER status.
    pub max_retries: i32,

    // ── Connection tuning (hints for main.rs) ────────────────────────────
    /// gRPC keep-alive interval for the CRUD channel.
    pub grpc_keepalive_interval: Duration,
    /// gRPC keep-alive timeout.
    pub grpc_keepalive_timeout: Duration,
    /// gRPC concurrency limit per connection.
    pub grpc_concurrency_limit: usize,
}

impl Default for InstitutionConfig {
    fn default() -> Self {
        Self {
            own_bic: "OWNBICXXXX".into(),
            biz_svc: Some("swift.cbprplus.02".into()),

            signing_key_path: None,
            signing_key_password: None,
            signing_cert_base64: None,
            signing_enabled: false,
            verify_pubkey_pem_path: None,

            screening_enabled: false,
            screening_service_url: "http://localhost:9090/screen".into(),
            screening_api_key: None,
            screening_timeout: Duration::from_secs(10),
            duplicate_check_enabled: true,

            recall_sla_days: 10,
            unable_to_apply_sla_days: 15,
            claim_non_receipt_sla_days: 15,
            sla_check_interval: Duration::from_secs(60),
            sla_calendar_country: "BE".into(), // Belgium (TARGET2 calendar)

            max_retries: 10,

            grpc_keepalive_interval: Duration::from_secs(30),
            grpc_keepalive_timeout: Duration::from_secs(10),
            grpc_concurrency_limit: 100,
        }
    }
}

impl InstitutionConfig {
    pub fn from_env() -> Self {
        let mut c = Self::default();
        if let Ok(v) = std::env::var("OWN_BIC")                 { c.own_bic = v; }
        if let Ok(v) = std::env::var("BIZ_SVC")                 { c.biz_svc = Some(v); }
        if let Ok(v) = std::env::var("SIGNING_KEY_PATH")         { c.signing_key_path = Some(v); c.signing_enabled = true; }
        if let Ok(v) = std::env::var("SIGNING_KEY_PASSWORD")     { c.signing_key_password = Some(v); }
        if let Ok(v) = std::env::var("SIGNING_CERT_BASE64")      { c.signing_cert_base64 = Some(v); }
        if let Ok(v) = std::env::var("VERIFY_PUBKEY_PEM_PATH")   { c.verify_pubkey_pem_path = Some(v); }
        if let Ok(v) = std::env::var("SCREENING_ENABLED")        { c.screening_enabled = v == "true" || v == "1"; }
        if let Ok(v) = std::env::var("SCREENING_SERVICE_URL")    { c.screening_service_url = v; }
        if let Ok(v) = std::env::var("SCREENING_API_KEY")        { c.screening_api_key = Some(v); }
        if let Ok(v) = std::env::var("SCREENING_TIMEOUT_SECS") {
            match v.parse::<u64>().map(Duration::from_secs) {
                Ok(d) => c.screening_timeout = d,
                Err(_) => eprintln!("Warning: invalid SCREENING_TIMEOUT_SECS={v:?}, using default {:?}", c.screening_timeout),
            }
        }
        if let Ok(v) = std::env::var("DUPLICATE_CHECK_ENABLED")  { c.duplicate_check_enabled = v != "false" && v != "0"; }
        if let Ok(v) = std::env::var("RECALL_SLA_DAYS") {
            match v.parse() {
                Ok(n) => c.recall_sla_days = n,
                Err(_) => eprintln!("Warning: invalid RECALL_SLA_DAYS={v:?}, using default {}", c.recall_sla_days),
            }
        }
        if let Ok(v) = std::env::var("SLA_CALENDAR_COUNTRY")     { c.sla_calendar_country = v; }
        if let Ok(v) = std::env::var("MAX_RETRIES") {
            match v.parse() {
                Ok(n) => c.max_retries = n,
                Err(_) => eprintln!("Warning: invalid MAX_RETRIES={v:?}, using default {}", c.max_retries),
            }
        }
        c
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_config_has_sane_baseline_values() {
        let c = InstitutionConfig::default();
        assert_eq!(c.own_bic, "OWNBICXXXX");
        assert!(!c.signing_enabled);
        assert!(!c.screening_enabled);
        assert!(c.duplicate_check_enabled);
        assert_eq!(c.max_retries, 10);
        assert_eq!(c.sla_calendar_country, "BE");
    }

    /// Bundles every `from_env` assertion into one test function rather than
    /// splitting per-variable, since Rust test threads share process-global
    /// environment state - separate `#[test]` fns setting/clearing the same
    /// env vars would race under the default parallel test runner.
    #[test]
    fn from_env_overrides_and_fallback_behaviour() {
        // Clean slate: make sure none of these are set from a previous run/
        // the outer environment before asserting default fallback behaviour.
        for var in [
            "OWN_BIC", "BIZ_SVC", "SIGNING_KEY_PATH", "SIGNING_KEY_PASSWORD",
            "SIGNING_CERT_BASE64", "VERIFY_PUBKEY_PEM_PATH", "SCREENING_ENABLED",
            "SCREENING_SERVICE_URL", "SCREENING_API_KEY", "SCREENING_TIMEOUT_SECS",
            "DUPLICATE_CHECK_ENABLED", "RECALL_SLA_DAYS", "SLA_CALENDAR_COUNTRY", "MAX_RETRIES",
        ] {
            std::env::remove_var(var);
        }

        // No env vars set -> pure defaults.
        let c = InstitutionConfig::from_env();
        assert_eq!(c.own_bic, "OWNBICXXXX");
        assert_eq!(c.max_retries, 10);
        assert!(c.duplicate_check_enabled);

        // Valid overrides take effect.
        std::env::set_var("OWN_BIC", "TESTBICX");
        std::env::set_var("SIGNING_KEY_PATH", "/tmp/key.pem");
        std::env::set_var("SCREENING_ENABLED", "true");
        std::env::set_var("SCREENING_TIMEOUT_SECS", "5");
        std::env::set_var("RECALL_SLA_DAYS", "20");
        std::env::set_var("MAX_RETRIES", "3");
        // "0"/"false" both count as "disabled" for DUPLICATE_CHECK_ENABLED.
        std::env::set_var("DUPLICATE_CHECK_ENABLED", "false");

        let c = InstitutionConfig::from_env();
        assert_eq!(c.own_bic, "TESTBICX");
        assert_eq!(c.signing_key_path.as_deref(), Some("/tmp/key.pem"));
        // Setting SIGNING_KEY_PATH implicitly enables signing.
        assert!(c.signing_enabled);
        assert!(c.screening_enabled);
        assert_eq!(c.screening_timeout, Duration::from_secs(5));
        assert_eq!(c.recall_sla_days, 20);
        assert_eq!(c.max_retries, 3);
        assert!(!c.duplicate_check_enabled);

        // Invalid numeric values fall back to the default rather than
        // panicking or leaving a garbage value.
        std::env::set_var("SCREENING_TIMEOUT_SECS", "not-a-number");
        std::env::set_var("RECALL_SLA_DAYS", "not-a-number");
        std::env::set_var("MAX_RETRIES", "not-a-number");
        let c = InstitutionConfig::from_env();
        assert_eq!(c.screening_timeout, Duration::from_secs(10)); // default
        assert_eq!(c.recall_sla_days, 10); // default
        assert_eq!(c.max_retries, 10); // default

        // "1" and "0" are accepted spellings alongside "true"/"false".
        std::env::set_var("SCREENING_ENABLED", "1");
        assert!(InstitutionConfig::from_env().screening_enabled);
        std::env::set_var("SCREENING_ENABLED", "0");
        assert!(!InstitutionConfig::from_env().screening_enabled);
        std::env::set_var("DUPLICATE_CHECK_ENABLED", "0");
        assert!(!InstitutionConfig::from_env().duplicate_check_enabled);

        // Clean up so this test doesn't leak env state into other tests.
        for var in [
            "OWN_BIC", "BIZ_SVC", "SIGNING_KEY_PATH", "SIGNING_KEY_PASSWORD",
            "SIGNING_CERT_BASE64", "VERIFY_PUBKEY_PEM_PATH", "SCREENING_ENABLED",
            "SCREENING_SERVICE_URL", "SCREENING_API_KEY", "SCREENING_TIMEOUT_SECS",
            "DUPLICATE_CHECK_ENABLED", "RECALL_SLA_DAYS", "SLA_CALENDAR_COUNTRY", "MAX_RETRIES",
        ] {
            std::env::remove_var(var);
        }
    }
}
