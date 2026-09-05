-- =============================================================================
-- Migration 009: message_ledger.dedup_key – idempotent customer notification enqueue
-- =============================================================================
--
-- FlowCoordinator writes customer-facing outbound messages (pain.002, pain.007)
-- directly into this table (direction='OUT') so the notification worker can
-- deliver them to the customer callback URL. Unlike inbound messages, each
-- generated outbound document embeds a freshly-minted MsgId/UUID, so its
-- raw bytes (and therefore body_hash) differ on every retry of the handler
-- that produced it — body_hash alone cannot dedupe re-enqueue attempts.
--
-- dedup_key mirrors the convention already used for the CBS outbound_queue
-- table: "<parent_msg_id>|<msg_type>". A partial unique index enforces that
-- the same (parent inbound message, outbound message type) pair is enqueued
-- at most once, regardless of how many times the handler is retried.


BEGIN;

ALTER TABLE iso20022.message_ledger
    ADD COLUMN IF NOT EXISTS dedup_key VARCHAR(160);

CREATE UNIQUE INDEX IF NOT EXISTS idx_message_ledger_dedup_key
    ON iso20022.message_ledger (dedup_key)
    WHERE dedup_key IS NOT NULL;

COMMIT;
