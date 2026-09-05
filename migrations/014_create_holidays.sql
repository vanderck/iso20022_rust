-- =============================================================================
-- Migration 014: holidays – public holiday / working-day calendar
-- =============================================================================
--
-- Backs Calendar::next_working_day / Calendar::count_business_days, which
-- replace SlaTracker's previous weekends-only count_business_days. A date
-- falling inside [from_date, to_date] on a configured holiday row is shifted
-- to shift_to_date (itself re-checked, so a shift landing on a weekend or
-- another holiday keeps advancing rather than silently violating the rule
-- it was meant to enforce).
--
-- Holidays are scoped per calendar_country (e.g. 'BE' for TARGET2/Belgium,
-- 'US' for FEDWIRE, 'GB' for CHAPS) so more than one settlement calendar can
-- be configured without rows colliding.

CREATE TABLE iso20022.holidays (
    holiday_id       BIGSERIAL       PRIMARY KEY,

    -- ISO country code the holiday applies to; matches
    -- InstitutionConfig::sla_calendar_country.
    calendar_country VARCHAR(8)      NOT NULL,

    from_date        DATE            NOT NULL,
    to_date           DATE           NOT NULL,

    -- Date a due date falling inside [from_date, to_date] is moved to.
    -- Re-validated by Calendar::next_working_day in case it's itself a
    -- weekend or another holiday.
    shift_to_date    DATE            NOT NULL,

    description      TEXT,

    created_at       TIMESTAMPTZ     NOT NULL DEFAULT NOW(),

    CONSTRAINT holidays_date_range_chk CHECK (to_date >= from_date)
);

CREATE INDEX idx_holidays_country_range
    ON iso20022.holidays (calendar_country, from_date, to_date);
