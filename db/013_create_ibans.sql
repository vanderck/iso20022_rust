-- =============================================================================
-- Migration 013: ibans – IBAN → core ledger account directory
-- =============================================================================
--
-- `core.accounts` (the CBS ledger schema) has no IBAN/account-number column
-- at all — just an internal `account bigint` id. The IBAN a counterparty
-- actually uses to address a payment is minted and owned here, in this
-- app's own database, and linked to the core account it resolves to.
-- Rows are written either by a human via `crust`-backed UI tooling, or by
-- an automatic IBAN-issuing process at account opening — both write
-- through the same `crust`/CRUD path this app already uses for every
-- other table, so no dedicated RPC is needed to read it back.
--
-- `core_account` is not a Postgres FK: `core` is a separate schema/database
-- reached only through the CRUD service, not a local table this schema can
-- reference directly.


BEGIN;

CREATE TABLE IF NOT EXISTS iso20022.ibans (
    iban            VARCHAR(34)     PRIMARY KEY,

    -- core.accounts.account this IBAN resolves to.
    core_account    BIGINT          NOT NULL,

    -- 'UI' (created interactively via crust) | 'AUTO' (issued automatically
    -- at account opening).
    source          VARCHAR(10)     NOT NULL DEFAULT 'AUTO',

    -- False once superseded/closed; inactive IBANs are kept for audit
    -- history but no longer resolve for new bookings.
    active          BOOLEAN         NOT NULL DEFAULT TRUE,

    created_at      TIMESTAMPTZ     NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_ibans_core_account
    ON iso20022.ibans (core_account);

COMMIT;
