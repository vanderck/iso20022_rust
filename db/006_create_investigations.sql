-- =============================================================================
-- Migration 006: investigations – payment investigation and recall tracking
-- =============================================================================
--
-- Tracks open investigations initiated by camt.026 (UnableToApply),
-- camt.027 (ClaimNonReceipt), camt.029 (ResolutionOfInvestigation), and
-- camt.056 (FIToFIPaymentCancellationRequest / Recall).
--
-- The SLA tracker reads this table to detect overdue investigations and
-- escalates them to 'SLA_BREACHED'.


BEGIN;

CREATE TABLE IF NOT EXISTS iso20022.investigations (
    investigation_id        BIGSERIAL                       PRIMARY KEY,

    -- Related payment journey (may be NULL if the journey cannot be resolved).
    journey_id              VARCHAR(50)
                                REFERENCES iso20022.payment_journeys (journey_id)
                                ON DELETE SET NULL,

    -- SWIFT case assignment ID (CaseAssgnmt/Id) used as the correlation key.
    -- This is the primary dedup key checked by open_investigation().
    assignment_id           VARCHAR(105)                    UNIQUE,

    -- Case ID (Case/Id) from the investigation message header.
    case_id                 VARCHAR(105),

    -- ISO 20022 message type that originated the investigation,
    -- e.g. 'camt.056.001.11', 'camt.026.001.10', 'camt.027.001.10'.
    investigation_type      VARCHAR(50)                     NOT NULL,

    -- Current lifecycle status of the investigation.
    investigation_status    iso20022.investigation_status   NOT NULL DEFAULT 'OPEN',

    -- ISO reason code from the originating message,
    -- e.g. 'DUPL' (duplicate), 'CUST' (customer request).
    reason_code             VARCHAR(35),

    -- Free-text details from the investigation message body.
    additional_info         TEXT,

    -- Used by SlaTracker to compute business_days_open relative to today.
    created_at              TIMESTAMPTZ                     NOT NULL DEFAULT NOW(),
    updated_at              TIMESTAMPTZ                     NOT NULL DEFAULT NOW()
);

-- SLA tracker reads all OPEN/PENDING investigations.
CREATE INDEX IF NOT EXISTS idx_investigations_open
    ON iso20022.investigations (created_at)
    WHERE investigation_status IN ('OPEN', 'PENDING');

-- Journey-level investigation lookup.
CREATE INDEX IF NOT EXISTS idx_investigations_journey_id
    ON iso20022.investigations (journey_id)
    WHERE journey_id IS NOT NULL;

-- Lookup by case_id for resolution correlation.
CREATE INDEX IF NOT EXISTS idx_investigations_case_id
    ON iso20022.investigations (case_id)
    WHERE case_id IS NOT NULL;

DROP TRIGGER IF EXISTS trg_investigations_updated_at ON iso20022.investigations;
CREATE TRIGGER trg_investigations_updated_at
    BEFORE UPDATE ON iso20022.investigations
    FOR EACH ROW EXECUTE FUNCTION iso20022.set_updated_at();

COMMIT;
