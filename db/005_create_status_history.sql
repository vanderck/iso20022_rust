-- =============================================================================
-- Migration 005: status_history – immutable payment event audit log
-- =============================================================================
--
-- Append-only record of every event that occurs on a payment journey.
-- Provides a complete audit trail and supports idempotent event processing:
-- the UNIQUE constraint on (status_code, related_msg_id) prevents duplicate
-- audit rows even when a handler is retried after a partial failure.
--
-- The engine performs an optimistic INSERT and treats a unique-constraint
-- violation (gRPC AlreadyExists) as success, eliminating the TOCTOU race
-- that would exist with a prior existence check.


BEGIN;

CREATE TABLE IF NOT EXISTS iso20022.status_history (
    status_id               BIGSERIAL                       PRIMARY KEY,

    -- The journey this event belongs to.  NULL is allowed for the rare case
    -- where an event arrives before a journey can be resolved (e.g. a status
    -- report for an unknown payment that is recorded for compliance purposes).
    journey_id              VARCHAR(50)
                                REFERENCES iso20022.payment_journeys (journey_id)
                                ON DELETE SET NULL,

    -- Machine-readable event code from JourneyEvent.status_history_code(),
    -- e.g. 'CUSTOMER_PAYMENT_INITIATED', 'FI_STATUS_UPDATE', 'FI_RETURN'.
    status_code             VARCHAR(100)                    NOT NULL,

    -- Human-readable description of the event.
    event_description       TEXT,

    -- Dedup key: composite of msg_type|msg_id|refs|status_code.
    -- Used both for idempotency and as a correlation key.
    related_msg_id          VARCHAR(255),

    -- ISO 20022 reason code carried by the source message,
    -- e.g. 'AC01' (Incorrect Account Number), 'CUST' (customer decision).
    reason_code             VARCHAR(35),

    created_at              TIMESTAMPTZ                     NOT NULL DEFAULT NOW(),

    -- Idempotency guard: the same logical event from the same source message
    -- must not produce duplicate audit rows.
    CONSTRAINT uq_status_history_event UNIQUE (status_code, related_msg_id)
);

-- Journey timeline queries.
CREATE INDEX IF NOT EXISTS idx_status_history_journey_id
    ON iso20022.status_history (journey_id, created_at)
    WHERE journey_id IS NOT NULL;

-- Existence check used by status_history_event_exists() (legacy read path).
CREATE INDEX IF NOT EXISTS idx_status_history_code_msg
    ON iso20022.status_history (status_code, related_msg_id);

COMMIT;
