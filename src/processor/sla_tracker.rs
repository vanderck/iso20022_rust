use chrono::{Datelike, NaiveDate, Utc, Weekday};
use crate::crudgrpc::{DbPath, Fld, ReadRequest, UpdateRequest};
use crate::processor::calendar::Calendar;
use crate::processor::config::InstitutionConfig;
use crate::context::CrudClient;

pub struct SlaTracker;

#[derive(Debug, Clone)]
pub struct SlaBreach {
    pub investigation_id: String,
    pub assignment_id: String,
    pub investigation_type: String,
    pub journey_id: Option<String>,
    pub business_days_open: i64,
    pub sla_days: u32,
}

impl SlaTracker {
    fn db_path(table: &str) -> DbPath {
        DbPath { server: "CBS".into(), schema: "iso20022".into(), table: table.into() }
    }

    /// Check all open investigations against their SLA deadline.
    pub async fn check_breaches(
        crud: &mut crate::context::CrudClient,
        config: &InstitutionConfig,
    ) -> Result<Vec<SlaBreach>, tonic::Status> {
        let req = ReadRequest {
            path: Some(Self::db_path("investigations")),
            column: vec![
                "investigation_id".into(), "assignment_id".into(),
                "investigation_type".into(), "journey_id".into(), "created_at".into(),
            ],
            where_clause: vec![
                Fld { name: "investigation_status".into(), relation: Some("IN".into()),
                       value: "OPEN,PENDING".to_string() },
            ],
            page: 0, page_size: Some(500),
        };
        let resp = crud.read(req).await?.into_inner();
        let today = Utc::now().date_naive();
        let mut breaches = Vec::new();

        for row in resp.data {
            let cols = &row.list;
            let inv_id = cols.first().cloned().unwrap_or_default();
            let assignment_id = cols.get(1).cloned().unwrap_or_default();
            let inv_type = cols.get(2).cloned().unwrap_or_default();
            let journey_id = cols.get(3).cloned();
            let created_at_str = cols.get(4).cloned().unwrap_or_default();

            let sla_days = match inv_type.as_str() {
                "camt.056.001.11" => config.recall_sla_days,
                "camt.026.001.10" => config.unable_to_apply_sla_days,
                "camt.027.001.10" => config.claim_non_receipt_sla_days,
                _ => config.recall_sla_days,
            };

            // Parse created_at to a date.
            let created_date = created_at_str.get(..10)
                .and_then(|s| NaiveDate::parse_from_str(s, "%Y-%m-%d").ok());

            if let Some(cd) = created_date {
                let biz_days = Calendar::count_business_days(crud, &config.sla_calendar_country, cd, today).await?;
                if biz_days > sla_days as i64 {
                    breaches.push(SlaBreach {
                        investigation_id: inv_id, assignment_id, investigation_type: inv_type,
                        journey_id, business_days_open: biz_days, sla_days,
                    });
                }
            }
        }
        Ok(breaches)
    }

    /// Mark an investigation as SLA-breached and log an alert.
    pub async fn escalate(
        crud: &mut CrudClient,
        breach: &SlaBreach,
    ) -> Result<(), tonic::Status> {
        tracing::warn!(
            investigation_id = %breach.investigation_id,
            assignment_id = %breach.assignment_id,
            business_days_open = %breach.business_days_open,
            sla_days = %breach.sla_days,
            "SLA BREACH: investigation exceeded deadline"
        );

        crate::processor::metrics::record_sla_breach(&breach.investigation_type);

        let _ = crud.update(UpdateRequest {
            who: "sla_tracker".into(), review: Some(false),
            path: Some(Self::db_path("investigations")),
            where_clause: vec![Fld {
                name: "investigation_id".into(), relation: None,
                value: breach.investigation_id.clone(),
            }],
            data: vec![
                Fld { name: "investigation_status".into(), relation: None, value: "SLA_BREACHED".into() },
            ],
        }).await?;

        // ┌──────────────────────────────────────────────────────────────┐
        // │ PLACEHOLDER: SLA breach escalation                          │
        // │                                                              │
        // │ Wire in your alerting / incident management here:            │
        // │ - Send alert to operations dashboard (e.g. PagerDuty)       │
        // │ - Create JIRA/ServiceNow ticket                              │
        // │ - For SEPA Instant recalls: auto-accept if within rules     │
        // │ - Push notification to compliance team                       │
        // └──────────────────────────────────────────────────────────────┘

        Ok(())
    }
}

/// Count business days between two dates, excluding weekends only.
///
/// Pure, DB-free fallback kept for offline unit tests. Production code
/// (`SlaTracker::check_breaches`) calls `Calendar::count_business_days`
/// instead, which also excludes configured public holidays via the
/// `iso20022.holidays` table (TARGET2/FEDWIRE/CHAPS etc., scoped by
/// `calendar_country`) — this weekend-only version is what you get when no
/// holidays are configured for a country, so it stays as the documented
/// baseline behavior rather than being deleted.
#[cfg_attr(not(test), allow(dead_code))]
fn count_business_days_excluding_weekends(from: NaiveDate, to: NaiveDate) -> i64 {
    if to <= from { return 0; }
    let mut count = 0i64;
    let mut d = from;
    while d < to {
        if !matches!(d.weekday(), Weekday::Sat | Weekday::Sun) {
            count += 1;
        }
        d += chrono::Duration::days(1);
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_business_days_weekdays_only() {
        // Monday to Friday = 5 business days
        let mon = NaiveDate::from_ymd_opt(2025, 1, 6).unwrap(); // Monday
        let sat = NaiveDate::from_ymd_opt(2025, 1, 11).unwrap(); // Saturday
        assert_eq!(count_business_days_excluding_weekends(mon, sat), 5);
    }

    #[test]
    fn test_business_days_across_weekend() {
        // Monday to next Monday = 5 business days (skips Sat+Sun)
        let mon1 = NaiveDate::from_ymd_opt(2025, 1, 6).unwrap();
        let mon2 = NaiveDate::from_ymd_opt(2025, 1, 13).unwrap();
        assert_eq!(count_business_days_excluding_weekends(mon1, mon2), 5);
    }
}
