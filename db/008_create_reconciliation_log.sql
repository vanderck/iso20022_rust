-- =============================================================================
-- Migration 008: reconciliation_log – bank statement reconciliation audit
-- =============================================================================
--
-- Records the outcome of reconciling each entry from an inbound bank statement
-- (camt.052 Intraday Report, camt.053 End-of-Day Statement,
--  camt.054 Credit/Debit Notification).
--
-- Written by reconciliation.rs (record_recon_event).
-- Unmatched and amount-mismatched entries are flagged here for manual review.


BEGIN;

CREATE TABLE IF NOT EXISTS iso20022.reconciliation_log (
    reconciliation_id       BIGSERIAL                       PRIMARY KEY,

    -- Reconciliation outcome for this statement entry.
    result                  iso20022.reconciliation_result  NOT NULL,

    -- The journey matched to this statement entry (NULL when UNMATCHED).
    journey_id              VARCHAR(50)
                                REFERENCES iso20022.payment_journeys (journey_id)
                                ON DELETE SET NULL,

    -- ── Statement entry fields ────────────────────────────────────────────

    -- End-to-end reference from the statement entry (NtryDtls/TxDtls/Refs/EndToEndId).
    end_to_end_id           VARCHAR(35),

    -- Statement entry reference (Ntry/NtryRef).
    entry_ref               VARCHAR(105),

    -- Entry amount.
    amount                  NUMERIC(20, 5),

    -- ISO 4217 currency code.
    currency                CHAR(3),

    -- Credit ('CRDT') or Debit ('DBIT') indicator.
    credit_debit            CHAR(4)
                                CHECK (credit_debit IN ('CRDT', 'DBIT')),

    created_at              TIMESTAMPTZ                     NOT NULL DEFAULT NOW()
);

-- Journey-level reconciliation lookups.
CREATE INDEX IF NOT EXISTS idx_reconciliation_log_journey_id
    ON iso20022.reconciliation_log (journey_id)
    WHERE journey_id IS NOT NULL;

-- Operations: find all unmatched or mismatched entries for manual review.
CREATE INDEX IF NOT EXISTS idx_reconciliation_log_result
    ON iso20022.reconciliation_log (result)
    WHERE result IN ('UNMATCHED', 'AMOUNT_MISMATCH');

-- Lookup by end_to_end_id for re-matching jobs.
CREATE INDEX IF NOT EXISTS idx_reconciliation_log_end_to_end_id
    ON iso20022.reconciliation_log (end_to_end_id)
    WHERE end_to_end_id IS NOT NULL;

COMMIT;
