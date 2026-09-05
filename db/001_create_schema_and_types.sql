-- =============================================================================
-- Migration 001: Create iso20022 schema and shared enum types
-- =============================================================================


BEGIN;

CREATE SCHEMA IF NOT EXISTS iso20022;

-- ---------------------------------------------------------------------------
-- Message direction: inbound vs outbound
-- ---------------------------------------------------------------------------
DO $do$
BEGIN
    CREATE TYPE iso20022.message_direction AS ENUM (
        'IN',
        'OUT'
    );
EXCEPTION
    -- Postgres has no CREATE TYPE ... IF NOT EXISTS; catching the
    -- duplicate is the idiomatic way to make this re-runnable.
    WHEN duplicate_object THEN NULL;
END $do$;

-- ---------------------------------------------------------------------------
-- Ledger processing status (message_ledger.status)
-- ---------------------------------------------------------------------------
DO $do$
BEGIN
    CREATE TYPE iso20022.ledger_status AS ENUM (
        'PENDING',
        'PROCESSING',
        'COMPLETED',
        'FAILED',
        'REJECTED',
        'DEAD_LETTER'
    );
EXCEPTION
    -- Postgres has no CREATE TYPE ... IF NOT EXISTS; catching the
    -- duplicate is the idiomatic way to make this re-runnable.
    WHEN duplicate_object THEN NULL;
END $do$;

-- ---------------------------------------------------------------------------
-- Payment journey lifecycle states (payment_journeys.current_status)
-- ---------------------------------------------------------------------------
DO $do$
BEGIN
    CREATE TYPE iso20022.journey_state AS ENUM (
        'RECEIVED',
        'CUSTOMER_ACCEPTED',
        'CUSTOMER_REJECTED',
        'TECHNICAL_ACCEPTED',
        'PENDING',
        'READY_FOR_EXECUTION',
        'SENT_TO_NETWORK',
        'SETTLEMENT_IN_PROCESS',
        'SETTLED',
        'REJECTED',
        'UNABLE_TO_APPLY',
        'RETURN_REQUESTED',
        'RETURNED',
        'CUSTOMER_REVERSAL_REQUESTED',
        'FI_REVERSAL_REQUESTED',
        'RECALL_REQUESTED',
        'RECALL_REJECTED',
        'REVERSED',
        'INVESTIGATION_OPEN',
        'INVESTIGATION_PENDING_RESPONSE',
        'INVESTIGATION_RESOLVED',
        'COMPLETED',
        'CANCELLED'
    );
EXCEPTION
    -- Postgres has no CREATE TYPE ... IF NOT EXISTS; catching the
    -- duplicate is the idiomatic way to make this re-runnable.
    WHEN duplicate_object THEN NULL;
END $do$;

-- ---------------------------------------------------------------------------
-- Investigation lifecycle states (investigations.investigation_status)
-- ---------------------------------------------------------------------------
DO $do$
BEGIN
    CREATE TYPE iso20022.investigation_status AS ENUM (
        'OPEN',
        'PENDING',
        'CLOSED',
        'SLA_BREACHED'
    );
EXCEPTION
    -- Postgres has no CREATE TYPE ... IF NOT EXISTS; catching the
    -- duplicate is the idiomatic way to make this re-runnable.
    WHEN duplicate_object THEN NULL;
END $do$;

-- ---------------------------------------------------------------------------
-- AML/sanctions screening outcomes (screening_log.result)
-- ---------------------------------------------------------------------------
DO $do$
BEGIN
    CREATE TYPE iso20022.screening_result AS ENUM (
        'CLEAR',
        'PENDING',
        'BLOCKED'
    );
EXCEPTION
    -- Postgres has no CREATE TYPE ... IF NOT EXISTS; catching the
    -- duplicate is the idiomatic way to make this re-runnable.
    WHEN duplicate_object THEN NULL;
END $do$;

-- ---------------------------------------------------------------------------
-- Bank statement reconciliation outcomes (reconciliation_log.result)
-- ---------------------------------------------------------------------------
DO $do$
BEGIN
    CREATE TYPE iso20022.reconciliation_result AS ENUM (
        'MATCHED',
        'UNMATCHED',
        'AMOUNT_MISMATCH'
    );
EXCEPTION
    -- Postgres has no CREATE TYPE ... IF NOT EXISTS; catching the
    -- duplicate is the idiomatic way to make this re-runnable.
    WHEN duplicate_object THEN NULL;
END $do$;

COMMIT;
