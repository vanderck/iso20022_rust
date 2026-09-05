use crate::context::CrudClient;
use crate::crudgrpc::{DbPath, Fld, ReadRequest, CreateRequest};
use crate::processor::config::InstitutionConfig;
use serde::Deserialize;

// ═══════════════════════════════════════════════════════════════════════════
// UETR Generation
// ═══════════════════════════════════════════════════════════════════════════

/// Generate a new UUIDv4 UETR per SWIFT gpi requirements.
pub fn generate_uetr() -> String {
    uuid::Uuid::new_v4().to_string()
}

// ═══════════════════════════════════════════════════════════════════════════
// Business-Level Duplicate Detection
// ═══════════════════════════════════════════════════════════════════════════

/// Check whether a payment with the same business key already exists.
///
/// Business key = (debtor_account, creditor_account, amount, currency, end_to_end_id, value_date).
///
/// `exclude_journey_id` must be the journey already created for the payment
/// being checked (pain.001/pacs.008 create the journey before this check
/// runs — see `FlowCoordinator::execute`). Without excluding it, the query
/// can non-deterministically read back that just-inserted row as the
/// "existing" duplicate — since it always matches its own business key —
/// and silently miss a genuine, older duplicate journey. Excluding it
/// guarantees any hit returned here is a different, pre-existing payment.
///
/// Returns `Ok(Some(journey_id))` if a duplicate is found, `Ok(None)` if clean.
pub async fn check_duplicate_payment(
    crud: &mut crate::context::CrudClient,
    exclude_journey_id: &str,
    end_to_end_id: Option<&str>,
    amount: Option<&str>,
    currency: Option<&str>,
    value_date: Option<&str>,
) -> Result<Option<String>, tonic::Status> {
    let e2e = match end_to_end_id {
        Some(v) if !v.is_empty() && v != "NOTPROVIDED" => v,
        _ => return Ok(None), // Cannot dedup without EndToEndId
    };

    let mut where_clause = vec![
        Fld { name: "end_to_end_id".into(), relation: None, value: e2e.to_string() },
        Fld { name: "journey_id".into(), relation: Some("!=".into()), value: exclude_journey_id.to_string() },
    ];
    if let Some(a) = amount {
        where_clause.push(Fld { name: "settlement_amount".into(), relation: None, value: a.to_string() });
    }
    if let Some(c) = currency {
        where_clause.push(Fld { name: "settlement_currency".into(), relation: None, value: c.to_string() });
    }
    if let Some(d) = value_date {
        where_clause.push(Fld { name: "settlement_date".into(), relation: None, value: d.to_string() });
    }

    let req = ReadRequest {
        path: Some(DbPath { server: "CBS".into(), schema: "iso20022".into(), table: "payment_journeys".into() }),
        column: vec!["journey_id".into()],
        where_clause,
        page: 0,
        page_size: Some(1),
    };
    let resp = crud.read(req).await?.into_inner();
    let result = resp.data.into_iter().next().and_then(|r| r.list.into_iter().next());
    if result.is_some() {
        crate::processor::metrics::record_duplicate_detected();
    }
    Ok(result)
}

// ═══════════════════════════════════════════════════════════════════════════
// AML / Sanctions Screening
// ═══════════════════════════════════════════════════════════════════════════

/// Screening decision returned by the compliance service.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ScreeningResult {
    /// Payment cleared — proceed with booking.
    Clear,
    /// Potential match — hold for manual review.
    PendingReview { hit_id: String, details: String },
    /// Confirmed sanctions match — reject the payment.
    Blocked { hit_id: String, details: String },
}

/// Vendor-agnostic response contract expected from `config.screening_service_url`.
///
/// `decision` must be one of `"CLEAR"` / `"PENDING"` / `"BLOCK"`. This shape is
/// deliberately generic (not modelled on any single vendor's native API) so a
/// thin adapter in front of whichever vendor is actually contracted — SWIFT
/// Sanctions Screening, Fircosoft/LSEG, Dow Jones Risk & Compliance,
/// ComplyAdvantage, etc. — is all that's needed to plug it in here.
#[derive(Debug, Deserialize)]
struct ScreeningApiResponse {
    decision: String,
    hit_id: Option<String>,
    details: Option<String>,
}

/// Screen a payment against AML/sanctions lists.
///
/// Calls `config.screening_service_url` with the payment's party names and
/// amount, bearer-authenticated with `config.screening_api_key`. This is a
/// **regulatory control, not a best-effort check**: for an institution
/// operating in Belgium this is what stands in for the mandatory screening
/// against the EU consolidated sanctions list, the UN list, and the Belgian
/// national list (Royal Decree of 28 December 2004), supervised by the NBB.
///
/// # Fail-closed
///
/// If the vendor is unreachable, times out, or returns something this
/// function can't parse, that is treated as a **transient error**, not as
/// `Clear` — the payment is held and retried rather than silently allowed
/// through because the screening call didn't work. `is_transient_error` in
/// `main.rs` string-matches on `"unavailable"`/`"timeout"`, so error messages
/// here are worded to match that and get retried via the circuit breaker
/// instead of being dead-lettered or (worse) treated as a permanent decline.
#[allow(clippy::too_many_arguments)]
pub async fn screen_payment(
    crud: &mut CrudClient,
    http_client: &reqwest::Client,
    config: &InstitutionConfig,
    journey_id: &str,
    debtor_name: Option<&str>,
    creditor_name: Option<&str>,
    amount: Option<&str>,
    currency: Option<&str>,
) -> Result<ScreeningResult, tonic::Status> {
    let req_body = serde_json::json!({
        "journey_id": journey_id,
        "debtor": debtor_name,
        "creditor": creditor_name,
        "amount": amount,
        "currency": currency,
    });

    let mut req = http_client
        .post(&config.screening_service_url)
        .timeout(config.screening_timeout)
        .json(&req_body);
    if let Some(key) = config.screening_api_key.as_deref() {
        req = req.bearer_auth(key);
    }

    let resp = req.send().await.map_err(|e| {
        tonic::Status::unavailable(format!("screening service unavailable: {e}"))
    })?;

    let status = resp.status();
    if !status.is_success() {
        return Err(tonic::Status::unavailable(format!(
            "screening service unavailable: HTTP {status}"
        )));
    }

    let body: ScreeningApiResponse = resp.json().await.map_err(|e| {
        tonic::Status::unavailable(format!(
            "screening service unavailable: unparseable response: {e}"
        ))
    })?;

    let result = match body.decision.as_str() {
        "CLEAR" => ScreeningResult::Clear,
        "PENDING" => ScreeningResult::PendingReview {
            hit_id: body.hit_id.unwrap_or_default(),
            details: body.details.unwrap_or_default(),
        },
        "BLOCK" => ScreeningResult::Blocked {
            hit_id: body.hit_id.unwrap_or_default(),
            details: body.details.unwrap_or_default(),
        },
        other => {
            return Err(tonic::Status::unavailable(format!(
                "screening service unavailable: unknown decision {other:?}"
            )));
        }
    };

    let result_str = match &result {
        ScreeningResult::Clear => "CLEAR",
        ScreeningResult::PendingReview { .. } => "PENDING",
        ScreeningResult::Blocked { .. } => "BLOCKED",
    };

    tracing::info!(
        journey_id = %journey_id,
        debtor = ?debtor_name,
        creditor = ?creditor_name,
        amount = ?amount,
        result = %result_str,
        "AML/sanctions screening decision"
    );

    // Record screening attempt and decision in the compliance audit table.
    // Belgian AML law requires retaining this kind of CDD/transaction record
    // for a minimum period (currently 10 years) after the relationship/
    // transaction ends — retention/purge policy for this table is a separate
    // operational concern, not something this call handles.
    crud.create(CreateRequest {
        who: "screening".into(),
        review: Some(false),
        path: Some(DbPath { server: "CBS".into(), schema: "iso20022".into(), table: "screening_log".into() }),
        data: vec![
            Fld { name: "journey_id".into(), relation: None, value: journey_id.to_string() },
            Fld { name: "debtor_name".into(), relation: None, value: debtor_name.unwrap_or("").to_string() },
            Fld { name: "creditor_name".into(), relation: None, value: creditor_name.unwrap_or("").to_string() },
            Fld { name: "amount".into(), relation: None, value: amount.unwrap_or("").to_string() },
            Fld { name: "currency".into(), relation: None, value: currency.unwrap_or("").to_string() },
            Fld { name: "result".into(), relation: None, value: result_str.to_string() },
        ],
    }).await?;

    crate::processor::metrics::record_screening_result(result_str);
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    
    

    fn lazy_crud() -> CrudClient {
        // Never dials out - safe for guard-clause paths that return before
        // any real RPC is awaited.
        let channel = tonic::transport::Channel::from_shared("http://127.0.0.1:1").unwrap().connect_lazy();
        let interceptor = crate::interceptor::AuthInterceptor::new("test-api-key").unwrap();
        crate::crudgrpc::crud_service_client::CrudServiceClient::with_interceptor(channel, interceptor)

    }

    #[test]
    fn generate_uetr_produces_a_valid_v4_uuid() {
        let uetr = generate_uetr();
        let parsed = uuid::Uuid::parse_str(&uetr).expect("should be a valid UUID");
        assert_eq!(parsed.get_version_num(), 4);
    }

    #[test]
    fn generate_uetr_is_unique_across_calls() {
        let a = generate_uetr();
        let b = generate_uetr();
        assert_ne!(a, b);
    }

    // ── check_duplicate_payment guard clauses (no live network needed) ────

    #[tokio::test]
    async fn check_duplicate_payment_short_circuits_on_missing_end_to_end_id() {
        let mut crud = lazy_crud();
        let result = check_duplicate_payment(&mut crud, "JRN-1", None, Some("100"), Some("EUR"), None).await;
        assert_eq!(result.unwrap(), None);
    }

    #[tokio::test]
    async fn check_duplicate_payment_short_circuits_on_empty_end_to_end_id() {
        let mut crud = lazy_crud();
        let result = check_duplicate_payment(&mut crud, "JRN-1", Some(""), None, None, None).await;
        assert_eq!(result.unwrap(), None);
    }

    #[tokio::test]
    async fn check_duplicate_payment_short_circuits_on_notprovided_sentinel() {
        let mut crud = lazy_crud();
        let result = check_duplicate_payment(&mut crud, "JRN-1", Some("NOTPROVIDED"), None, None, None).await;
        assert_eq!(result.unwrap(), None);
    }
}
