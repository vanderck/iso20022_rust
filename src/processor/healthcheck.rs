use std::sync::atomic::{AtomicI64, Ordering};
use std::sync::Arc;

/// Shared health state, updated by workers and read by the /health endpoint.
///
/// Wire this into your `AppState` in main.rs.
#[derive(Clone)]
pub struct HealthState {
    inner: Arc<HealthInner>,
}

struct HealthInner {
    /// Epoch millis of the last successful outbound notification send.
    last_outbound_success_ms: AtomicI64,
    /// Epoch millis of the last successful CRUD gRPC call.
    last_crud_success_ms: AtomicI64,
    /// Epoch millis of the last successful DB pool checkout.
    last_db_success_ms: AtomicI64,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct HealthReport {
    pub status: &'static str,
    pub db_ok: bool,
    pub crud_ok: bool,
    pub outbound_ok: bool,
    pub last_outbound_age_secs: Option<i64>,
    pub last_crud_age_secs: Option<i64>,
    pub warnings: Vec<String>,
}

impl Default for HealthState {
    fn default() -> Self {
        Self::new()
    }
}

impl HealthState {
    pub fn new() -> Self {
        Self {
            inner: Arc::new(HealthInner {
                last_outbound_success_ms: AtomicI64::new(0),
                last_crud_success_ms: AtomicI64::new(0),
                last_db_success_ms: AtomicI64::new(0),
            }),
        }
    }

    pub fn record_outbound_success(&self) {
        let now = chrono::Utc::now().timestamp_millis();
        self.inner.last_outbound_success_ms.store(now, Ordering::Relaxed);
    }

    pub fn record_crud_success(&self) {
        let now = chrono::Utc::now().timestamp_millis();
        self.inner.last_crud_success_ms.store(now, Ordering::Relaxed);
    }

    pub fn record_db_success(&self) {
        let now = chrono::Utc::now().timestamp_millis();
        self.inner.last_db_success_ms.store(now, Ordering::Relaxed);
    }

    /// Build a health report.
    ///
    /// `db_pool_ok`: result of a DB pool checkout attempt (done by caller).
    pub fn report(&self, db_pool_ok: bool) -> HealthReport {
        let now = chrono::Utc::now().timestamp_millis();
        let mut warnings = Vec::new();

        // CRUD: consider stale if no success in last 60 seconds.
        let crud_ms = self.inner.last_crud_success_ms.load(Ordering::Relaxed);
        let crud_ok = crud_ms > 0 && (now - crud_ms) < 60_000;
        let crud_age = if crud_ms > 0 { Some((now - crud_ms) / 1000) } else { None };
        if !crud_ok && crud_ms > 0 {
            warnings.push(format!("CRUD gRPC last success {}s ago", crud_age.unwrap_or(0)));
        }

        // Outbound: consider stale if no success in last 300 seconds AND workers are expected.
        let out_ms = self.inner.last_outbound_success_ms.load(Ordering::Relaxed);
        let outbound_ok = out_ms > 0 && (now - out_ms) < 300_000;
        let out_age = if out_ms > 0 { Some((now - out_ms) / 1000) } else { None };
        if !outbound_ok && out_ms > 0 {
            warnings.push(format!("Outbound last success {}s ago", out_age.unwrap_or(0)));
        }

        let status = if db_pool_ok && crud_ok {
            "healthy"
        } else if db_pool_ok {
            "degraded"
        } else {
            "unhealthy"
        };

        HealthReport {
            status, db_ok: db_pool_ok, crud_ok, outbound_ok,
            last_outbound_age_secs: out_age,
            last_crud_age_secs: crud_age,
            warnings,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fresh_state_reports_unhealthy_crud_and_outbound_before_any_success_recorded() {
        let health = HealthState::new();
        let report = health.report(true);
        assert!(report.db_ok);
        assert!(!report.crud_ok);
        assert!(!report.outbound_ok);
        assert_eq!(report.last_crud_age_secs, None);
        assert_eq!(report.last_outbound_age_secs, None);
        // No prior success recorded yet, so there's nothing stale to warn about.
        assert!(report.warnings.is_empty());
    }

    #[test]
    fn db_pool_failure_forces_unhealthy_status_regardless_of_crud_state() {
        let health = HealthState::new();
        health.record_crud_success();
        let report = health.report(false);
        assert_eq!(report.status, "unhealthy");
        assert!(!report.db_ok);
    }

    #[test]
    fn crud_success_makes_status_healthy_when_db_is_also_ok() {
        let health = HealthState::new();
        health.record_crud_success();
        let report = health.report(true);
        assert_eq!(report.status, "healthy");
        assert!(report.crud_ok);
    }

    #[test]
    fn db_ok_without_crud_success_is_degraded_not_healthy() {
        let health = HealthState::new();
        let report = health.report(true);
        assert_eq!(report.status, "degraded");
    }

    #[test]
    fn outbound_success_is_reflected_in_the_report() {
        let health = HealthState::new();
        // Regression test: record_outbound_success previously had no caller
        // anywhere in the binary, so `outbound_ok` was permanently false and
        // `/health` could never reflect a working notification channel.
        health.record_outbound_success();
        let report = health.report(true);
        assert!(report.outbound_ok);
        assert_eq!(report.last_outbound_age_secs, Some(0));
    }

    #[test]
    fn default_impl_matches_new() {
        let health = HealthState::default();
        let report = health.report(true);
        assert!(!report.crud_ok);
        assert!(!report.outbound_ok);
    }
}
