-- =============================================================================
-- Migration 007: screening_log – AML / sanctions screening audit trail
-- =============================================================================
--
-- Records every AML/sanctions screening attempt and its outcome.
-- Written by compliance.rs (screen_payment) regardless of the screening result
-- so the compliance audit trail is always complete.
--
-- In the current implementation screen_payment() is a stub that always returns
-- CLEAR.  The schema is production-ready for a real screening service
-- integration — replace the stub body and the hit_id column will be populated
-- with the reference returned by the vendor.


BEGIN;

CREATE TABLE IF NOT EXISTS iso20022.screening_log (
    screening_id            BIGSERIAL                       PRIMARY KEY,

    -- Payment journey that was screened.
    journey_id              VARCHAR(50)                     NOT NULL
                                REFERENCES iso20022.payment_journeys (journey_id)
                                ON DELETE CASCADE,

    -- Party names submitted to the screening service.
    debtor_name             VARCHAR(255),
    creditor_name           VARCHAR(255),

    -- Payment amount and currency submitted for context.
    amount                  NUMERIC(20, 5),
    currency                CHAR(3),

    -- Screening decision.
    result                  iso20022.screening_result       NOT NULL,

    -- Vendor-assigned hit reference, populated when result = PENDING or BLOCKED.
    hit_id                  VARCHAR(100),

    created_at              TIMESTAMPTZ                     NOT NULL DEFAULT NOW()
);

-- Journey-level screening history.
CREATE INDEX IF NOT EXISTS idx_screening_log_journey_id
    ON iso20022.screening_log (journey_id);

-- Compliance queries: find all BLOCKED or PENDING screening events.
CREATE INDEX IF NOT EXISTS idx_screening_log_result
    ON iso20022.screening_log (result)
    WHERE result IN ('PENDING', 'BLOCKED');

COMMIT;
