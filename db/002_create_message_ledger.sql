-- =============================================================================
-- Migration 002: message_ledger – inbound and outbound message processing queue
-- =============================================================================
--
-- Central work queue consumed by the inbound and outbound worker pools.
-- Workers claim rows with FOR UPDATE SKIP LOCKED to prevent double-processing.
-- body_hash (SHA-256 of raw payload) provides inbound deduplication.


BEGIN;

CREATE TABLE IF NOT EXISTS iso20022.message_ledger (
    -- Surrogate primary key; exposed to callers as the "message ID".
    message         BIGSERIAL                       PRIMARY KEY,

    -- IN  = inbound message received via /webhook
    -- OUT = outbound message queued for delivery to customer callback
    direction       iso20022.message_direction      NOT NULL,

    -- Current processing state of this message.
    status          iso20022.ledger_status          NOT NULL DEFAULT 'PENDING',

    -- Full raw XML payload (stored as bytes to preserve original encoding).
    raw_xml_bytes   BYTEA                           NOT NULL,

    -- ISO 20022 message type hint sniffed from the XML root element,
    -- e.g. 'pain.001.001.12'.  NULL means detection failed.
    msg_type        VARCHAR(50),

    -- SHA-256 hex digest of raw_xml_bytes.  Used for inbound dedup:
    -- a second submission of an identical payload is rejected (or ignored)
    -- rather than processed twice.
    body_hash       CHAR(64)                        UNIQUE,

    -- Number of processing attempts made so far.
    retry_count     INTEGER                         NOT NULL DEFAULT 0,

    -- Workers set this to NOW() + lease_interval when they claim a row.
    -- The maintenance loop uses it to detect stale leases and reset them.
    next_retry_at   TIMESTAMPTZ,

    -- Human-readable error from the last failed processing attempt.
    fail_reason     TEXT,

    created_at      TIMESTAMPTZ                     NOT NULL DEFAULT NOW(),
    updated_at      TIMESTAMPTZ                     NOT NULL DEFAULT NOW()
);

-- Workers filter and order by these columns on every claim query.
CREATE INDEX IF NOT EXISTS idx_message_ledger_inbound_claim
    ON iso20022.message_ledger (created_at)
    WHERE direction = 'IN'
      AND status IN ('PENDING', 'FAILED');

CREATE INDEX IF NOT EXISTS idx_message_ledger_outbound_claim
    ON iso20022.message_ledger (message)
    WHERE direction = 'OUT'
      AND status IN ('PENDING', 'FAILED');

-- Maintenance loop queries for rows eligible for dead-lettering.
CREATE INDEX IF NOT EXISTS idx_message_ledger_failed
    ON iso20022.message_ledger (status)
    WHERE status = 'FAILED';

-- Timestamp trigger to keep updated_at current on every write.
CREATE OR REPLACE FUNCTION iso20022.set_updated_at()
RETURNS TRIGGER LANGUAGE plpgsql AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$;

DROP TRIGGER IF EXISTS trg_message_ledger_updated_at ON iso20022.message_ledger;
CREATE TRIGGER trg_message_ledger_updated_at
    BEFORE UPDATE ON iso20022.message_ledger
    FOR EACH ROW EXECUTE FUNCTION iso20022.set_updated_at();

COMMIT;
