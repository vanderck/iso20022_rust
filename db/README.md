# ISO 20022 PostgreSQL Migrations

Numbered sequential migrations for the `iso20022` schema.

## Apply all migrations

```bash
cd migrations && ./apply.sh          # uses the standard PG* environment variables
cd migrations && ./apply.sh "postgresql://user@host/db"
```

`apply.sh` applies the numbered files in order and records what it applied in
`iso20022.schema_migrations`.

Every file is idempotent and wrapped in a single transaction, so re-running is
safe: an already-applied file is a no-op, and a file that fails rolls back
rather than leaving the schema half-migrated. That was not previously true —
none of these files could be run twice, and none was transactional, so a
migration that failed partway left the schema in whatever state it had reached.
The tracking table is for visibility ("what is applied here, and when"), not
for correctness.

Running the files by hand with `psql -f` in numeric order still works and
reaches the same state.

## File overview

| File | Description |
|------|-------------|
| `001_create_schema_and_types.sql` | `iso20022` schema + all PostgreSQL enum types |
| `002_create_message_ledger.sql` | Inbound/outbound message processing queue |
| `003_create_payment_journeys.sql` | Core payment lifecycle table |
| `004_create_iso_messages.sql` | Full ISO 20022 message archive |
| `005_create_status_history.sql` | Immutable payment event audit log |
| `006_create_investigations.sql` | Recall and investigation tracking (camt.026/027/029/056) |
| `007_create_screening_log.sql` | AML/sanctions screening audit trail |
| `008_create_reconciliation_log.sql` | Bank statement reconciliation results |
| `009_add_message_ledger_dedup_key.sql` | Idempotent customer-notification enqueue key on `message_ledger` |
| `010_add_payment_journeys_party_data.sql` | Debtor/creditor name+account columns on `payment_journeys` |
| `011_create_cbs_bookings.sql` | Ledger booking audit trail written by `book_in_cbs` |
| `012_add_sanctions_hold_state.sql` | `SANCTIONS_HOLD` journey state for confirmed screening matches |
| `013_create_ibans.sql` | IBAN → core ledger account directory, used to resolve CBS bookings |

## Schema diagram

```
message_ledger          (independent — work queue)

payment_journeys        (root entity)
    │
    ├── iso_messages    (FK → payment_journeys.journey_id, nullable)
    ├── status_history  (FK → payment_journeys.journey_id, nullable)
    ├── investigations  (FK → payment_journeys.journey_id, nullable)
    ├── screening_log   (FK → payment_journeys.journey_id, NOT NULL)
    └── reconciliation_log (FK → payment_journeys.journey_id, nullable)
```

## Environment variables (from `AppConfig`)

The application connects using:

| Variable   | Default     | Description              |
|------------|-------------|--------------------------|
| `DB_HOST`  | `localhost` | PostgreSQL hostname       |
| `DB_USER`  | —           | Database user             |
| `DB_PASS`  | —           | Database password         |
| `DB_NAME`  | —           | Database name             |
