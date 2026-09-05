-- =============================================================================
-- Migration 010: payment_journeys – debtor/creditor party data
-- =============================================================================
--
-- Previously only agent BICs (debtor_agent/creditor_agent) were captured on a
-- journey. An outbound pacs.008 built from a pain.001-initiated journey
-- (OutboundBuilder::build_pacs008) requires real Debtor/Creditor party
-- identification — pacs.008's <Dbtr>/<Cdtr> elements are mandatory, and a
-- credit transfer with empty party blocks will be rejected by any real
-- clearing network. These columns let the handler capture that data once,
-- at journey-creation time, from the originating pain.001 (or an inbound
-- pacs.008), so it can be reused when building outbound messages later.


BEGIN;

ALTER TABLE iso20022.payment_journeys
    ADD COLUMN IF NOT EXISTS debtor_name       VARCHAR(140),
    ADD COLUMN IF NOT EXISTS debtor_account    VARCHAR(34),
    ADD COLUMN IF NOT EXISTS creditor_name     VARCHAR(140),
    ADD COLUMN IF NOT EXISTS creditor_account  VARCHAR(34);

COMMIT;
