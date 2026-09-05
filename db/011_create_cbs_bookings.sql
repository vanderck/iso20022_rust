-- =============================================================================
-- Migration 011: cbs_bookings – ledger booking audit trail
-- =============================================================================
--
-- Written by FlowCoordinator::book_in_cbs after a successful
-- EngineService authorisation+settle call. This table was referenced by the
-- CRUD calls in book_in_cbs from the start but never had a migration of its
-- own — it only worked because the CRUD service tolerated writes to a table
-- it didn't need to validate against a local schema. Adding it here now that
-- book_in_cbs also needs to persist the correlating engine transaction id.


BEGIN;

CREATE TABLE IF NOT EXISTS iso20022.cbs_bookings (
    booking_id              BIGSERIAL                       PRIMARY KEY,

    journey_id              VARCHAR(50)
                                REFERENCES iso20022.payment_journeys (journey_id)
                                ON DELETE SET NULL,

    -- 'CUSTOMER_CREDIT_TRANSFER' | 'FI_CREDIT_TRANSFER' | 'PAYMENT_RETURN' | 'PAYMENT_REVERSAL'
    booking_type            VARCHAR(50)                     NOT NULL,

    -- Correlates this booking with the EngineService Authorisation/Settle
    -- calls (see FlowCoordinator::engine_transaction_id).
    engine_transaction_id   BIGINT,

    amount                  NUMERIC(20, 5),
    currency                CHAR(3),

    -- Currently always 'BOOKED' — book_in_cbs only writes this row after
    -- authorisation+settle both succeed; rejections are surfaced upstream as
    -- BookingOutcome::Rejected and never reach this table.
    status                  VARCHAR(20)                     NOT NULL,

    created_at              TIMESTAMPTZ                     NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_cbs_bookings_journey_id
    ON iso20022.cbs_bookings (journey_id)
    WHERE journey_id IS NOT NULL;

CREATE INDEX IF NOT EXISTS idx_cbs_bookings_engine_transaction_id
    ON iso20022.cbs_bookings (engine_transaction_id)
    WHERE engine_transaction_id IS NOT NULL;

COMMIT;
