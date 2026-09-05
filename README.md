# ISO 20022 Payment Processor

A standalone Rust service that receives, validates, routes, and generates
[ISO 20022](https://www.iso20022.org/) financial messages (the `pain.*`,
`pacs.*`, and `camt.*` families) for a bank's core payment operations:
customer payment initiation, interbank clearing/settlement, returns,
recalls, investigations, AML/sanctions screening, account
statements/notifications, and reconciliation against the core banking
system (CBS).

It is not a bank in a box — it is the ISO 20022 messaging layer that sits
in front of a core banking system (CBS) and a settlement engine, both
accessed over gRPC, and in front of a downstream customer-facing system
reached over HTTP webhooks.

## How it fits together

```
                         ┌──────────────────────────────────────────┐
 SWIFT / counterparty ──▶│  POST /webhook   (axum HTTP server)      │
  or customer ERP        └──────────────────┬───────────────────────┘
                                             │ hashed + enqueued
                                             ▼
                          iso20022.message_ledger  (Postgres work queue)
                                             │
                     ┌───────────────────────┼────────────────────────┐
                     ▼                                                ▼
           Payment workers (N)                              Notification workers (N)
     claim IN rows, detect msg type,                  claim OUT rows, POST to the
     XSD-validate (+ XMLDSig verify),                 customer callback URL, retry
     dispatch to a handler                             with backoff on 429/5xx
                     │
                     ▼
        handlers/<msg>.rs  →  mapper::get_handler()
     deserializes the message, then calls into:
                     │
      ┌──────────────┼───────────────────────────────┐
      ▼               ▼                                ▼
 processor::batch  processor::compliance         processor::flow_coordinator
 (extract txs from  (duplicate check,             (journey resolution/creation,
  the parsed doc)    AML/sanctions screening)       state transitions, CBS booking,
      │                                              outbound message generation)
      ▼
 processor::handler_support / state_effects / state_machine
 (journey CRUD, status_history audit trail, state-machine transitions)
                     │
                     ▼
        CrudService (gRPC)  ──▶  Postgres (payment_journeys, iso_messages,
                                  status_history, investigations,
                                  screening_log, reconciliation_log,
                                  cbs_bookings, ibans)
        EngineService (gRPC) ──▶ ledger authorisation / settlement
```

Every inbound and outbound message passes through `iso20022.message_ledger`
first — the HTTP handler's only job is to authenticate, sniff the message
type, hash the body for dedup, and enqueue a row. All actual processing
happens in a pool of async workers that claim rows with `FOR UPDATE SKIP
LOCKED`, so the service scales horizontally by adding worker
processes/replicas, and a crash mid-processing just leaves the row
reclaimable (see `PROCESSING_LEASE_SECS` in `src/main.rs`).

## Supported messages

| Message | Meaning | Direction |
|---|---|---|
| `pain.001.001.12` | Customer Credit Transfer Initiation | inbound (customer → bank) |
| `pain.002.001.14` | Customer Payment Status Report | outbound (bank → customer) |
| `pain.007.001.12` | Customer Payment Reversal | inbound |
| `pacs.008.001.13` | FI To FI Customer Credit Transfer | inbound + outbound (interbank) |
| `pacs.002.001.15` | FI To FI Payment Status Report | inbound + outbound |
| `pacs.004.001.14` | Payment Return | inbound + outbound |
| `pacs.007.001.13` | FI To FI Payment Reversal | inbound |
| `pacs.028.001.06` | FI To FI Payment Status Request | inbound |
| `camt.026.001.10` | Unable To Apply | inbound |
| `camt.027.001.10` | Claim Non Receipt | inbound |
| `camt.029.001.13` | Resolution Of Investigation | inbound + outbound |
| `camt.052.001.13` | Bank To Customer Account Report | outbound |
| `camt.053.001.13` | Bank To Customer Statement | outbound |
| `camt.054.001.13` | Bank To Customer Debit/Credit Notification | outbound |
| `camt.056.001.11` | FI To FI Payment Cancellation Request (recall) | inbound |

Each message type has a hand-written handler in `src/handlers/`, a
`consume()`/`generate()` implementation of the `mapper::Message` trait, and
strongly-typed request/response structs in `src/messages/` generated from
the official ISO 20022 XSDs (`serde` + `quick-xml` derive-based
(de)serialization, `validator` for field-level constraints).

Unknown *minor* versions fall back to the nearest compiled handler for the
same message family (`pacs.008.001.14` → the `pacs.008.001.13` handler),
logging a warning and incrementing the `iso20022_version_fallback_total`
metric, rather than dead-lettering messages the day a counterparty rolls a
schema forward. Unknown message *families* are rejected outright.

## Project layout

```
src/
  main.rs                 Axum HTTP server, worker pools, DB-driven job claiming,
                           circuit breakers, maintenance/dead-letter loop, SLA loop
  lib.rs                  Library root — re-exports ProcessingContext
  context.rs              ProcessingContext: DB pool, HTTP client, gRPC clients, config
  mapper.rs                Message trait, envelope wrapping, handler dispatch table
  signer.rs / verifier.rs  XMLDSig signing / signature + XSD schema verification
  xml_utils.rs             Namespace-agnostic bounded-depth XML node search
  handlers/                One file per ISO 20022 message type (consume + generate)
  messages/                Generated request/response types (one file per XSD)
  processor/
    batch.rs                Extract per-transaction rows out of a parsed document
    flow_coordinator.rs      Orchestrates screening → booking → outbound generation
    handler_support.rs       Journey resolution/creation, event processing helpers
    state_machine.rs         Pure event → next-state transition function
    state_effects.rs         Persists state transitions + status_history audit rows
    transition_policy.rs     Which state → state transitions are legal
    outbound_builder.rs       Builds pacs.002/004/008, camt.029 response XML
    compliance.rs            Duplicate-payment check, AML/sanctions screening call
    reconciliation.rs        Matches camt.053/054 lines against payment_journeys
    sla_tracker.rs            Business-day SLA breach detection + escalation
    transaction_scope.rs      Deferred-write batching helper for handlers
    apphdr.rs                 head.001 AppHdr sender/receiver BIC extraction
    version_router.rs         Message-version → handler-name resolution
    healthcheck.rs            /health state (DB/CRUD/outbound freshness)
    metrics.rs                Prometheus metric recorders (/metrics endpoint)
    config.rs                 InstitutionConfig (BIC, signing, screening, SLA, retry)
proto/                    gRPC service definitions (CrudService, EngineService),
                           compiled by build.rs into the crudgrpc/engine modules
db/                        Numbered Postgres migrations plus apply.sh — see db/README.md
```

## Building

### System dependencies

The build needs one thing beyond the Rust toolchain, because the `libxml`
crate links a native library rather than using a pure-Rust equivalent:

```bash
# Debian/Ubuntu
sudo apt-get install -y libxml2-dev pkg-config
```

`libxml2-dev` is required by the `libxml` crate (used for canonicalization
during XMLDSig signing/verification). `protoc` is *not* required: `build.rs`
uses `protoc-bin-vendored`, matching the rest of this ecosystem.

> **The `.proto` files are the single source of truth.** `build.rs` compiles
> `proto/engine.proto` and `proto/crudgrpc.proto`, and `lib.rs` pulls the
> result in with `tonic::include_proto!`.
>
> This used to work the other way around: `src/crudgrpc.rs` and `src/engine.rs`
> were prost-generated files checked into the repository and used directly,
> while `build.rs` compiled the `.proto` files into `$OUT_DIR` where nothing
> ever read them. The effect was that the `.proto` files here were decorative
> - the contract this service actually spoke was frozen in the checked-in
> Rust, and editing a proto changed nothing. Both files have been deleted.
>
> `proto/engine.proto` and `proto/crudgrpc.proto` are byte-identical copies of
> `vanderck/engine`'s and `vanderck/CRUDGRPC`'s own files. **Re-copy them when
> the upstream contract changes** - nothing automates it, but a drift now
> surfaces as a compile error here instead of a runtime failure.

### Build, test, lint

```bash
cargo build --all-targets
cargo test
cargo clippy --all-targets   # clean — no warnings
```

## Running

The binary is `iso20022d` (`cargo run --bin iso20022d`). Configuration is
entirely environment-variable driven — there is no config file.

### Required

| Variable | Description |
|---|---|
| `DB_HOST`, `DB_USER`, `DB_PASS`, `DB_NAME` | Postgres connection |
| `UPSTREAM_API_KEY` | Bearer token presented to `engine` and `crudgrpc` on every outbound gRPC call |

`UPSTREAM_API_KEY` is required and has no default. Both upstreams authenticate
every RPC against a single shared token in the `authorization` metadata key and
answer `UNAUTHENTICATED` otherwise. This service previously built both clients
with no interceptor and sent no credential at all, so every call it made to
`engine` was refused; failing at startup is better than failing per-request.

### Common optional settings

| Variable | Default | Description |
|---|---|---|
| `LISTEN_ADDR` | `0.0.0.0:3000` | HTTP bind address (`/webhook`, `/health`, `/metrics`) |
| `PAYMENT_WORKERS` | `4` | Inbound message worker pool size |
| `NOTIFY_WORKERS` | `3` | Outbound notification worker pool size |
| `ENGINE_HOST` | `http://localhost:50052` | Settlement `EngineService` gRPC endpoint |
| `CRUD_HOST` | `http://localhost:50053` | CBS `CrudService` gRPC endpoint |
| `CUSTOMER_CALLBACK_URL` | `https://customer-erp.com/webhook/payment-status` | Where outbound customer notifications (pain.002 etc.) are POSTed |
| `MAX_BODY_SIZE_MB` | `10` | Max accepted webhook body size |
| `DB_POOL_MAX` | workers + 8 | Postgres pool size cap |
| `WEBHOOK_API_KEY` | unset (auth disabled) | If set, `/webhook` requires `Authorization: Bearer <key>` |
| `MAX_RETRIES` | `10` | Retries before a message moves to `DEAD_LETTER` |
| `OWN_BIC` | `OWNBICXXXX` | This institution's BIC, used in outbound envelopes |
| `SCREENING_ENABLED` | `false` | Enable the AML/sanctions screening call in `FlowCoordinator` |
| `SCREENING_SERVICE_URL` | `http://localhost:9090/screen` | Vendor-agnostic screening endpoint (expects `{decision, hit_id, details}`) |
| `VERIFY_PUBKEY_PEM_PATH` | unset (verification disabled) | PEM public key used to verify inbound XMLDSig signatures |
| `SIGNING_KEY_PATH` | unset | Private key for outbound XMLDSig signing (see **Known limitations**) |

See `src/processor/config.rs` (`InstitutionConfig::from_env`) and
`src/main.rs` (`AppConfig::from_env`) for the full list.

### Database

Apply migrations in order before starting the service — see
[`db/README.md`](db/README.md) for the full list and an
entity diagram.

```bash
./db/apply.sh                       # or: ./db/apply.sh "postgresql://user@host/db"
```

### XSD schemas

At startup the service looks for XSD files under `xsd_to_rs/xsds/` (relative
to the working directory) for each of the 15 supported message types, to
validate inbound messages before dispatch. **This directory is not part of
the repository** — provide it at deploy time, or inbound messages will skip
XSD validation entirely (a warning is logged per missing schema at startup;
signature verification and business-logic processing still run).

## Observability

- `GET /health` — JSON `HealthReport` (DB pool checkout, CRUD gRPC
  freshness, outbound-notification freshness). Returns `503` when
  unhealthy.
- `GET /metrics` — Prometheus exposition format (`src/processor/metrics.rs`).
- Structured logs via `tracing`, controlled by `RUST_LOG` (defaults to
  `info`).

## Testing

```bash
cargo test
```

168 unit tests cover the pure/testable core: message routing and version
fallback, XML canonicalization and namespace-agnostic search, XMLDSig
signing/verification (including tamper and wrong-key rejection), XSD
validation, the journey state machine and transition policy, per-message
transaction extraction (`processor::batch`), outbound message building,
duplicate-payment and screening guard clauses, SLA business-day
calculations, and config parsing/fallback behaviour.

These are unit tests against pure logic and guard clauses — they don't
require a live Postgres, CRUD service, or engine, so `cargo test` runs
standalone. There is no integration test suite exercising the full
webhook → worker → gRPC → Postgres path; that would need a docker-compose
environment with all three dependencies running.

## Known limitations

These are pre-existing gaps found during review, not regressions — noted
here so they're visible rather than silently discovered in production:

- **Outbound XMLDSig signing is implemented but not wired up.**
  `signer::load_signing_key`/`load_public_cert` and `mapper::sign_and_wrap`
  exist and are unit-tested, and `InstitutionConfig` has
  `signing_enabled`/`signing_key_path`/`signing_cert_base64`, but nothing in
  `main.rs` loads a signing key at startup, and every handler's
  `generate()` calls the unsigned `self.wrap(...)` unconditionally. Setting
  `SIGNING_KEY_PATH` currently has no effect on outbound messages. Wiring
  this up means loading the key once into `AppState`/`ProcessingContext`
  and switching each handler's `generate()` to `mapper::sign_and_wrap` when
  a key is present.
- **Customer callbacks for inbound pain.001/pacs.008 aren't sent.** Per the
  comment in `main.rs::process_payment_job`, interbank outbound messages
  (pacs.002/004/008 written to the CBS's `outbound_queue` via
  `FlowCoordinator`) work correctly, but pain.002 customer notifications
  aren't yet written to the local `message_ledger` (`direction='OUT'`), so
  the notification workers never pick them up.
- **DB connections are unencrypted** (`NoTls` in `build_db_pool`) and the
  `/webhook` endpoint's bearer-token auth is optional dev-mode-friendly
  auth, not mTLS — both flagged as `TODO`s in `main.rs` for production
  hardening (SWIFT connectivity should use mTLS or an HSM-backed scheme).
- **`TransactionScope::stage_create`/`stage_update`** offer an opt-in
  deferred/batched write path, but no handler currently uses it — all
  writes go through the immediate `HandlerSupport`/`StateEffects` helpers
  instead.

## Recent cleanup

- Removed ~1,280 lines of boilerplate `impl Default` blocks across the
  generated `src/messages/*.rs` files in favor of `#[derive(Default)]` +
  `#[default]` (mechanical, behavior-preserving).
- Fixed `HealthState::record_outbound_success()` never being called —
  `/health`'s outbound-notification freshness check was permanently
  reporting stale/unhealthy regardless of actual notification traffic.
- Removed `StateEffects::status_history_event_exists`, dead code left over
  from before `append_status_history` switched to an optimistic-insert +
  `AlreadyExists` pattern.
- Wired `record_version_fallback` and `record_maintenance_action` into
  their obvious call sites (`mapper::get_handler`'s fallback path and
  `main.rs`'s maintenance loop) — both metrics existed but had no caller.
- `cargo clippy --all-targets` is clean (0 warnings); `cargo test` passes
  168/168.
