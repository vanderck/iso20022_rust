use chrono::{Datelike, Duration, NaiveDate, Weekday};

use crate::context::CrudClient;
use crate::crudgrpc::{DbPath, Fld, ReadRequest};

/// Maximum re-shifts `next_working_day` will follow before giving up.
///
/// A shift landing on another weekend/holiday keeps advancing; this bound
/// only exists to protect against a misconfigured holiday table producing a
/// `shift_to_date` cycle (e.g. two holiday rows pointing at each other).
const MAX_SHIFT_ITERATIONS: u8 = 30;

/// Public holiday / working-day calendar, backed by `iso20022.holidays`.
///
/// Replaces `SlaTracker`'s previous weekends-only day counting. Holidays are
/// scoped by `calendar_country` (matching `InstitutionConfig::sla_calendar_country`,
/// e.g. `"BE"` for TARGET2) so more than one settlement calendar can be
/// configured without rows colliding.
pub struct Calendar;

impl Calendar {
    fn db_path() -> DbPath {
        DbPath { server: "CBS".into(), schema: "iso20022".into(), table: "holidays".into() }
    }

    /// Look up the configured `shift_to_date` for `date`, if it falls inside
    /// a holiday range for `country`. `None` means `date` isn't a holiday.
    async fn holiday_shift(
        crud: &mut CrudClient,
        country: &str,
        date: NaiveDate,
    ) -> Result<Option<NaiveDate>, tonic::Status> {
        let date_str = date.format("%Y-%m-%d").to_string();
        let req = ReadRequest {
            path: Some(Self::db_path()),
            column: vec!["shift_to_date".into()],
            where_clause: vec![
                Fld { name: "calendar_country".into(), relation: None, value: country.to_string() },
                Fld { name: "from_date".into(), relation: Some("<=".into()), value: date_str.clone() },
                Fld { name: "to_date".into(), relation: Some(">=".into()), value: date_str },
            ],
            page: 0,
            page_size: Some(1),
        };
        let resp = crud.read(req).await?.into_inner();
        Ok(resp
            .data
            .into_iter()
            .next()
            .and_then(|row| row.list.into_iter().next())
            .and_then(|s| NaiveDate::parse_from_str(&s, "%Y-%m-%d").ok()))
    }

    /// Advance `date` forward until it is neither a weekend nor a configured
    /// holiday for `country`, following `shift_to_date` re-shifts as needed.
    pub async fn next_working_day(
        crud: &mut CrudClient,
        country: &str,
        date: NaiveDate,
    ) -> Result<NaiveDate, tonic::Status> {
        let mut d = date;
        for _ in 0..MAX_SHIFT_ITERATIONS {
            if matches!(d.weekday(), Weekday::Sat | Weekday::Sun) {
                d += Duration::days(1);
                continue;
            }
            match Self::holiday_shift(crud, country, d).await? {
                Some(shifted) if shifted != d => d = shifted,
                _ => return Ok(d),
            }
        }
        Ok(d)
    }

    /// Count business days between `from` (exclusive) and `to` (exclusive),
    /// excluding weekends and any configured holiday for `country`.
    ///
    /// Falls back to weekend-only counting when no holiday rows are
    /// configured for `country` — see `sla_tracker::count_business_days_excluding_weekends`
    /// for the pure, DB-free equivalent used by offline tests.
    pub async fn count_business_days(
        crud: &mut CrudClient,
        country: &str,
        from: NaiveDate,
        to: NaiveDate,
    ) -> Result<i64, tonic::Status> {
        if to <= from {
            return Ok(0);
        }
        let mut count = 0i64;
        let mut d = from;
        while d < to {
            if !matches!(d.weekday(), Weekday::Sat | Weekday::Sun)
                && Self::holiday_shift(crud, country, d).await?.is_none()
            {
                count += 1;
            }
            d += Duration::days(1);
        }
        Ok(count)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn lazy_crud() -> CrudClient {
        let channel = tonic::transport::Channel::from_shared("http://127.0.0.1:1").unwrap().connect_lazy();
        let interceptor = crate::interceptor::AuthInterceptor::new("test-api-key").unwrap();
        crate::crudgrpc::crud_service_client::CrudServiceClient::with_interceptor(channel, interceptor)
    }

    #[tokio::test]
    async fn count_business_days_short_circuits_when_to_not_after_from() {
        // Never dials out - returns before any RPC is awaited since `to <= from`.
        let mut crud = lazy_crud();
        let d = NaiveDate::from_ymd_opt(2025, 1, 6).unwrap();
        let result = Calendar::count_business_days(&mut crud, "BE", d, d).await;
        assert_eq!(result.unwrap(), 0);
    }

    #[tokio::test]
    async fn count_business_days_same_direction_guard_also_short_circuits() {
        let mut crud = lazy_crud();
        let to = NaiveDate::from_ymd_opt(2025, 1, 6).unwrap();
        let from = NaiveDate::from_ymd_opt(2025, 1, 10).unwrap();
        let result = Calendar::count_business_days(&mut crud, "BE", from, to).await;
        assert_eq!(result.unwrap(), 0);
    }
}
