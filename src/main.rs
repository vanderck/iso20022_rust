use anyhow::{Context, Result};
use axum::{
    body::Bytes,
    extract::{DefaultBodyLimit, State},
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use deadpool_postgres::{Manager, ManagerConfig, Pool, RecyclingMethod};
use failsafe::futures::CircuitBreaker;
use failsafe::{Config, StateMachine};
use quick_xml::events::Event;
use quick_xml::reader::Reader;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::{
    collections::HashMap,
    env::current_dir,
    io::{BufReader, Cursor},
    sync::Arc,
    time::Duration,
};
use tokio::{
    net::TcpListener,
    signal,
    sync::{Notify, Semaphore},
    task::{spawn_blocking, JoinSet},
    time::sleep,
};
use tokio_postgres::NoTls;
use tokio_util::sync::CancellationToken;
use tonic::transport::Channel;
use tracing::{error, info, instrument, warn};

// Library imports
use iso20022::{crudgrpc::crud_service_client::CrudServiceClient, processor::config::InstitutionConfig};
use iso20022::engine::engine_service_client::EngineServiceClient;
use iso20022::uidgen::uid_gen_service_client::UidGenServiceClient;
use iso20022::interceptor::AuthInterceptor;
use iso20022::mapper::get_handler;
use iso20022::processor::apphdr::AppHdrMeta;
use iso20022::processor::healthcheck::HealthState;
use iso20022::processor::metrics;
use iso20022::processor::sla_tracker::SlaTracker;
use iso20022::verifier::{validate_and_verify_once, CompiledSchema};
use iso20022::ProcessingContext;

// --- CONSTANTS ---

const NOTIFICATION_BATCH_SIZE: usize = 50;
const NOTIFICATION_CONCURRENCY: usize = 10;

// Safety bound for XML sniffing / malformed XML CPU spin protection.
const MAX_XML_SCAN_BYTES: usize = 1024 * 1024; // 1MB

// Lease duration for DB row ownership while "PROCESSING".
// Implemented via next_retry_at used as a lease-until timestamp for PROCESSING rows.
const PROCESSING_LEASE_SECS: i64 = 15 * 60; // 15 minutes

// --- TYPES ---
type SharedCircuitBreaker = Arc<
    StateMachine<failsafe::failure_policy::ConsecutiveFailures<std::iter::Repeat<Duration>>, ()>,
>;

enum WorkerOutcome {
    Success(()),
    PermanentFailure(String), // Domain error - Does NOT trip CB
    TransientFailure(String), // System error - TRIPS CB
    NoHandler,
}

struct PaymentJob {
    id: i64,
    raw_xml: Bytes,
    db_msg_type: Option<String>,
    retry_count: i32,
}

struct OutboundJob {
    id: i64,
    raw_xml: Bytes,
    retry_count: i32,
}

#[derive(Clone)]
struct AppState {
    ctx: ProcessingContext,
    schemas: Arc<HashMap<String, Arc<CompiledSchema>>>,
    notify: Arc<Notify>,
    customer_callback_url: Arc<String>,
    health: HealthState,
    /// Optional API key required in `Authorization: Bearer <key>` on /webhook.
    /// Set via WEBHOOK_API_KEY env var; if absent, auth is disabled (dev only).
    ///
    /// TODO: Replace with mTLS client certificates for production SWIFT connectivity.
    webhook_api_key: Option<Arc<String>>,
    /// XMLDSig public key PEM bytes, loaded once at startup from
    /// `InstitutionConfig::verify_pubkey_pem_path` when set. `None` disables
    /// signature verification (schema validation still runs independently).
    verify_pubkey_pem: Option<Arc<Vec<u8>>>,
}

#[derive(Deserialize, Clone)]
struct AppConfig {
    host: String,
    user: String,
    password: String,
    dbname: String,
    payment_worker_count: usize,
    notification_worker_count: usize,
    engine_host: String,
    crud_host: String,
    uidgen_host: String,
    engine_api_key: String,
    crud_api_key: String,
    uidgen_api_key: String,
    max_body_size: usize,
    customer_callback_url: String,
    /// Maximum Postgres pool connections. Defaults to workers+8.
    db_pool_max: Option<usize>,
    /// Optional static API key for /webhook. Absent = no auth (dev/internal only).
    webhook_api_key: Option<String>,
}

impl AppConfig {
    fn from_env() -> Result<Self> {
        let payment_workers = std::env::var("PAYMENT_WORKERS")
            .unwrap_or_else(|_| "4".into())
            .parse()
            .context("PAYMENT_WORKERS must be an integer")?;
        let notify_workers = std::env::var("NOTIFY_WORKERS")
            .unwrap_or_else(|_| "3".into())
            .parse()
            .context("NOTIFY_WORKERS must be an integer")?;
        Ok(Self {
            host: std::env::var("DB_HOST").context("DB_HOST missing")?,
            user: std::env::var("DB_USER").context("DB_USER missing")?,
            password: std::env::var("DB_PASS").context("DB_PASS missing")?,
            dbname: std::env::var("DB_NAME").context("DB_NAME missing")?,
            payment_worker_count: payment_workers,
            notification_worker_count: notify_workers,
            engine_host: std::env::var("ENGINE_HOST")
                .unwrap_or_else(|_| "http://localhost:50052".to_string()),
            crud_host: std::env::var("CRUD_HOST")
                .unwrap_or_else(|_| "http://localhost:50053".to_string()),
            uidgen_host: std::env::var("UIDGEN_HOST")
                .unwrap_or_else(|_| "http://localhost:50054".to_string()),
            engine_api_key: std::env::var("ENGINE_API_KEY").context("ENGINE_API_KEY missing")?,
            crud_api_key: std::env::var("CRUD_API_KEY").context("CRUD_API_KEY missing")?,
            uidgen_api_key: std::env::var("UIDGEN_API_KEY").context("UIDGEN_API_KEY missing")?,
            max_body_size: std::env::var("MAX_BODY_SIZE_MB")
                .unwrap_or_else(|_| "10".into())
                .parse::<usize>()
                .context("MAX_BODY_SIZE_MB must be an integer")?
                * 1024
                * 1024,
            customer_callback_url: std::env::var("CUSTOMER_CALLBACK_URL")
                .unwrap_or_else(|_| "https://customer-erp.com/webhook/payment-status".to_string()),
            db_pool_max: std::env::var("DB_POOL_MAX")
                .ok()
                .and_then(|v| v.parse().ok()),
            webhook_api_key: std::env::var("WEBHOOK_API_KEY").ok(),
        })
    }
}

// --- MAIN ---
#[tokio::main]
async fn main() -> Result<()> {
    // Structured logging: respects RUST_LOG env var (e.g. "iso20022=debug,warn").
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
        )
        .with_target(true)
        .init();

    // Install Prometheus metrics recorder (must be called before any metric recording).
    let metrics_handle = iso20022::processor::metrics::install_metrics()
        .map_err(|e| anyhow::anyhow!("Failed to install Prometheus metrics recorder: {e}"))?;

    let config = AppConfig::from_env()?;
    let pool = build_db_pool(&config)?;
    let health_state = HealthState::new();

    // Tune idle pool to match total expected concurrency rather than just worker count.
    let idle_per_host = (config.notification_worker_count * NOTIFICATION_CONCURRENCY).max(8);
    let http_client = reqwest::Client::builder()
        .timeout(Duration::from_secs(10))
        .pool_max_idle_per_host(idle_per_host)
        .build()?;

    info!("Loading schemas...");
    let xsd_dir = current_dir()?.join("xsd_to_rs/xsds/");
    let mut schema_map = HashMap::new();
    let definitions = vec![
        ("camt.026.001.10", vec!["wrapper_camt.026.001.10.xsd", "camt.026.001.10.xsd"]),
        ("camt.027.001.10", vec!["wrapper_camt.027.001.10.xsd", "camt.027.001.10.xsd"]),
        ("camt.029.001.13", vec!["wrapper_camt.029.001.13.xsd", "camt.029.001.13.xsd"]),
        ("camt.052.001.13", vec!["wrapper_camt.052.001.13.xsd", "camt.052.001.13.xsd"]),
        ("camt.053.001.13", vec!["wrapper_camt.053.001.13.xsd", "camt.053.001.13.xsd"]),
        ("camt.054.001.13", vec!["wrapper_camt.054.001.13.xsd", "camt.054.001.13.xsd"]),
        ("camt.056.001.11", vec!["wrapper_camt.056.001.11.xsd", "camt.056.001.11.xsd"]),
        ("pacs.002.001.15", vec!["wrapper_pacs.002.001.15.xsd", "pacs.002.001.15.xsd"]),
        ("pacs.004.001.14", vec!["wrapper_pacs.004.001.14.xsd", "pacs.004.001.14.xsd"]),
        ("pacs.007.001.13", vec!["wrapper_pacs.007.001.13.xsd", "pacs.007.001.13.xsd"]),
        ("pacs.008.001.13", vec!["wrapper_pacs.008.001.13.xsd", "pacs.008.001.13.xsd"]),
        ("pacs.028.001.06", vec!["wrapper_pacs.028.001.06.xsd", "pacs.028.001.06.xsd"]),
        ("pain.001.001.12", vec!["wrapper_pain.001.001.12.xsd", "pain.001.001.12.xsd"]),
        ("pain.002.001.14", vec!["wrapper_pain.002.001.14.xsd", "pain.002.001.14.xsd"]),
        ("pain.007.001.12", vec!["wrapper_pain.007.001.12.xsd", "pain.007.001.12.xsd"]),
    ];

    for (key, candidates) in definitions {
        let chosen = candidates
            .iter()
            .map(|file| xsd_dir.join(file))
            .find(|path| path.exists());
        if let Some(path) = chosen {
            let s = CompiledSchema::load_with_key(key, &path)
                .context(format!("Failed to load {:?}", path))?;
            schema_map.insert(key.to_string(), Arc::new(s));
        } else {
            warn!("No schema file found for {} in {:?}", key, xsd_dir);
        }
    }

    let notify = Arc::new(Notify::new());

    let institution_config = InstitutionConfig::from_env();
    let verify_pubkey_pem = match institution_config.verify_pubkey_pem_path.as_deref() {
        Some(path) => {
            let bytes = std::fs::read(path)
                .with_context(|| format!("Failed to read VERIFY_PUBKEY_PEM_PATH at {path:?}"))?;
            info!("XMLDSig signature verification enabled (public key: {})", path);
            Some(Arc::new(bytes))
        }
        None => {
            warn!("VERIFY_PUBKEY_PEM_PATH not set; inbound XMLDSig signatures will NOT be verified");
            None
        }
    };

    let state = AppState {
        ctx: ProcessingContext {
            db_pool: pool.clone(),
            http_client,
            engine_client: EngineServiceClient::with_interceptor(
                Channel::from_shared(config.engine_host.clone())?.connect_lazy(),
                AuthInterceptor::new(&config.engine_api_key)?,
            ),
            crud_client: CrudServiceClient::with_interceptor(
                Channel::from_shared(config.crud_host.clone())?.connect_lazy(),
                AuthInterceptor::new(&config.crud_api_key)?,
            ),
            uidgen_client: UidGenServiceClient::with_interceptor(
                Channel::from_shared(config.uidgen_host.clone())?.connect_lazy(),
                AuthInterceptor::new(&config.uidgen_api_key)?,
            ),
            config: institution_config,
        },
        schemas: Arc::new(schema_map),
        notify: notify.clone(),
        customer_callback_url: Arc::new(config.customer_callback_url.clone()),
        health: health_state,
        webhook_api_key: config.webhook_api_key.clone().map(Arc::new),
        verify_pubkey_pem,
    };

    let token = CancellationToken::new();

    let banking_cb = Arc::new(
        Config::new()
            .failure_policy(failsafe::failure_policy::consecutive_failures(
                5,
                std::iter::repeat(Duration::from_secs(10)),
            ))
            .build(),
    );

    let notify_cb = Arc::new(
        Config::new()
            .failure_policy(failsafe::failure_policy::consecutive_failures(
                3,
                std::iter::repeat(Duration::from_secs(30)),
            ))
            .build(),
    );

    let mut handles = Vec::new();

    for id in 0..config.payment_worker_count {
        handles.push(tokio::spawn(spawn_supervised_worker(
            "Payment",
            id,
            state.clone(),
            token.clone(),
            banking_cb.clone(),
            payment_worker_loop,
        )));
    }

    for id in 0..config.notification_worker_count {
        handles.push(tokio::spawn(spawn_supervised_worker(
            "Notification",
            id,
            state.clone(),
            token.clone(),
            notify_cb.clone(),
            notification_worker_loop,
        )));
    }

    handles.push(tokio::spawn(maintenance_loop(
        pool.clone(),
        notify.clone(),
        token.clone(),
        state.ctx.config.max_retries,
    )));

    handles.push(tokio::spawn(sla_tracker_loop(
        state.ctx.crud_client.clone(),
        state.ctx.config.clone(),
        token.clone(),
    )));

    let metrics_handle_shared = Arc::new(metrics_handle);
    let app = Router::new()
        .route("/webhook", post(handle_webhook))
        .route("/health", get(health))
        .route(
            "/metrics",
            get(move || {
                let h = metrics_handle_shared.clone();
                async move { h.render() }
            }),
        )
        .with_state(state)
        .layer(DefaultBodyLimit::max(config.max_body_size));

    let listen_addr = std::env::var("LISTEN_ADDR")
        .unwrap_or_else(|_| "0.0.0.0:3000".to_string());
    let listener = TcpListener::bind(&listen_addr).await?;
    info!("Server listening on {}", listener.local_addr()?);

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown(token.clone()))
        .await?;

    for h in handles {
        let _ = h.await;
    }
    Ok(())
}

// --- SUPERVISOR ---
async fn spawn_supervised_worker<F, Fut>(
    name: &'static str,
    id: usize,
    state: AppState,
    token: CancellationToken,
    cb: SharedCircuitBreaker,
    worker_fn: F,
) where
    F: Fn(usize, AppState, CancellationToken, SharedCircuitBreaker) -> Fut + Copy + Send + 'static,
    Fut: std::future::Future<Output = ()> + Send + 'static,
{
    loop {
        if token.is_cancelled() {
            break;
        }
        let handle = tokio::spawn(worker_fn(id, state.clone(), token.clone(), cb.clone()));
        match handle.await {
            Ok(_) => {
                info!("{} Worker {} exited normally.", name, id);
                break;
            }
            Err(e) => {
                error!(
                    "CRITICAL: {} Worker {} crashed: {:?}. Restarting in 1s...",
                    name, id, e
                );
                sleep(Duration::from_secs(1)).await;
            }
        }
    }
}

// --- DB-DRIVEN CLAIMING ---

async fn claim_next_inbound_job(pool: &Pool, max_retries: i32) -> Result<Option<PaymentJob>> {
    let client = pool.get().await?;

    // Claims exactly one eligible inbound row, leases it, and returns payload.
    // Rows whose retry_count has reached max_retries are not claimed (they stay
    // as FAILED so the maintenance loop can move them to DEAD_LETTER).
    let row_opt = client
        .query_opt(
            "WITH picked AS (
                 SELECT message
                 FROM iso20022.message_ledger
                 WHERE direction = 'IN'
                   AND (
                     status = 'PENDING'
                     OR (status = 'FAILED' AND next_retry_at <= NOW() AND retry_count < $1)
                   )
                 ORDER BY created_at
                 FOR UPDATE SKIP LOCKED
                 LIMIT 1
             )
             UPDATE iso20022.message_ledger ml
             SET status='PROCESSING',
                 updated_at=NOW(),
                 next_retry_at = NOW() + ($2 || ' seconds')::interval
             FROM picked
             WHERE ml.message = picked.message
             RETURNING ml.message, ml.raw_xml_bytes, ml.msg_type, ml.retry_count",
            &[&max_retries, &PROCESSING_LEASE_SECS],
        )
        .await?;

    Ok(row_opt.map(|r| {
        let id: i64 = r.get(0);
        let bytes: Vec<u8> = r.get(1);
        let msg_type: Option<String> = r.try_get(2).ok();
        let retry_count: i32 = r.try_get(3).unwrap_or(0);
        PaymentJob {
            id,
            raw_xml: Bytes::from(bytes),
            db_msg_type: msg_type,
            retry_count,
        }
    }))
}

async fn claim_next_outbound_job(pool: &Pool, max_retries: i32) -> Result<Option<OutboundJob>> {
    let client = pool.get().await?;

    // Claims up to one outbound row, leases it, and returns payload.
    // Rows whose retry_count has reached max_retries are not claimed.
    let row_opt = client
        .query_opt(
            "WITH picked AS (
                 SELECT message
                 FROM iso20022.message_ledger
                 WHERE direction = 'OUT'
                   AND (
                     status = 'PENDING'
                     OR (status = 'FAILED' AND next_retry_at <= NOW() AND retry_count < $1)
                   )
                 ORDER BY message ASC
                 FOR UPDATE SKIP LOCKED
                 LIMIT 1
             )
             UPDATE iso20022.message_ledger ml
             SET status='PROCESSING',
                 updated_at=NOW(),
                 next_retry_at = NOW() + ($2 || ' seconds')::interval
             FROM picked
             WHERE ml.message = picked.message
             RETURNING ml.message, ml.raw_xml_bytes, ml.retry_count",
            &[&max_retries, &PROCESSING_LEASE_SECS],
        )
        .await?;

    Ok(row_opt.map(|r| {
        let id: i64 = r.get(0);
        let bytes: Vec<u8> = r.get(1);
        let retry_count: i32 = r.get(2);
        OutboundJob {
            id,
            raw_xml: Bytes::from(bytes),
            retry_count,
        }
    }))
}

// --- WORKER LOOPS ---

async fn payment_worker_loop(
    id: usize,
    state: AppState,
    token: CancellationToken,
    cb: SharedCircuitBreaker,
) {
    loop {
        if token.is_cancelled() {
            break;
        }

        // If circuit is open, back off (no DB churn).
        if !cb.is_call_permitted() {
            sleep(Duration::from_secs(5)).await;
            continue;
        }

        match claim_next_inbound_job(&state.ctx.db_pool, state.ctx.config.max_retries).await {
            Ok(Some(job)) => {
                if let Err(e) = process_payment_job(id, job, &state, &cb).await {
                    error!("Payment Worker {}: Exec error: {}", id, e);
                }
                // Loop immediately to claim more.
            }
            Ok(None) => {
                // Nothing to do: sleep until notified (or periodic short timeout).
                tokio::select! {
                    _ = token.cancelled() => break,
                    _ = state.notify.notified() => {},
                    _ = sleep(Duration::from_millis(250)) => {},
                }
            }
            Err(e) => {
                error!("Payment Worker {}: DB claim error: {}", id, e);
                sleep(Duration::from_secs(1)).await;
            }
        }
    }
}

async fn notification_worker_loop(
    _id: usize,
    state: AppState,
    token: CancellationToken,
    cb: SharedCircuitBreaker,
) {
    let semaphore = Arc::new(Semaphore::new(NOTIFICATION_CONCURRENCY));
    let mut join_set: JoinSet<()> = JoinSet::new();

    loop {
        // Reap finished tasks without blocking
        while join_set.try_join_next().is_some() {}

        if token.is_cancelled() {
            // stop starting new work; drain in-flight
            if join_set.is_empty() {
                break;
            }
            let _ = join_set.join_next().await;
            continue;
        }

        // If circuit is open, back off and avoid DB churn.
        if !cb.is_call_permitted() {
            sleep(Duration::from_secs(2)).await;
            continue;
        }

        // Claim up to NOTIFICATION_BATCH_SIZE jobs; if none, wait.
        let mut batch: Vec<OutboundJob> = Vec::with_capacity(NOTIFICATION_BATCH_SIZE);
        for _ in 0..NOTIFICATION_BATCH_SIZE {
            match claim_next_outbound_job(&state.ctx.db_pool, state.ctx.config.max_retries).await {
                Ok(Some(job)) => batch.push(job),
                Ok(None) => break,
                Err(e) => {
                    error!("Notification claim error: {}", e);
                    break;
                }
            }
        }

        if batch.is_empty() {
            tokio::select! {
                _ = token.cancelled() => break,
                _ = state.notify.notified() => {},
                _ = sleep(Duration::from_millis(250)) => {},
            }
            continue;
        }

        for job in batch {
            if token.is_cancelled() {
                break;
            }
            let state_c = state.clone();
            let cb_c = cb.clone();
            let sem_c = semaphore.clone();

            if let Ok(permit) = sem_c.acquire_owned().await {
                join_set.spawn(async move {
                    let _permit = permit;
                    let job_id = job.id;
                    if let Err(e) = process_single_notification(job, state_c, cb_c).await {
                        error!("Notification error {}: {}", job_id, e);
                    }
                });
            }
        }
    }
}

// --- PAYMENT PROCESSING ---

#[derive(Debug, Clone)]
struct DetectedMessage {
    doc_type: Option<String>,
    has_apphdr: bool,
}

#[instrument(skip(state, cb, job), fields(msg_id = %job.id))]
async fn process_payment_job(
    _worker_id: usize,
    job: PaymentJob,
    state: &AppState,
    cb: &SharedCircuitBreaker,
) -> Result<()> {
    let start = std::time::Instant::now();

    // Prefer msg_type stored in DB (from webhook). If unknown/legacy, detect.
    let db_type = job
        .db_msg_type
        .clone()
        .filter(|s| s != "unknown" && s != "head.001");

    let detected = if db_type.is_some() {
        DetectedMessage {
            doc_type: db_type.clone(),
            has_apphdr: false,
        }
    } else {
        let xml_slice = job.raw_xml.clone();
        spawn_blocking(move || detect_message(&xml_slice)).await?
    };

    let msg_type = detected.doc_type.clone().unwrap_or_else(|| {
        if detected.has_apphdr {
            "head.001".to_string()
        } else {
            "unknown".to_string()
        }
    });

    // Validate only business-document types we actually have schemas for.
    // High-throughput path: parse ONCE and run XSD (and optionally signature) inside the same blocking task.
    if let Some(doc_type) = detected.doc_type.as_ref() {
        if let Some(schema) = state.schemas.get(doc_type) {
            let xml_ptr = job.raw_xml.clone();
            let schema_ptr = schema.clone();
            // Cloning the Arc is cheap; owning it lets the 'static spawn_blocking
            // closure borrow the key bytes for the duration of the call.
            let pubkey_arc = state.verify_pubkey_pem.clone();

            let val_res = spawn_blocking(move || {
                let s = std::str::from_utf8(&xml_ptr)
                    .map_err(|e| anyhow::anyhow!("Invalid UTF-8 in message body: {e}"))?;
                let pubkey: Option<&[u8]> = pubkey_arc.as_deref().map(|v| v.as_slice());
                validate_and_verify_once(s, Some(&schema_ptr), pubkey)
            })
            .await?;

            if let Err(e) = val_res {
                error!("Validation failed for {}: {}", job.id, e);
                metrics::record_validation_failure(&msg_type);
                metrics::record_message_processed(&msg_type, "rejected");
                metrics::record_message_duration(&msg_type, start.elapsed());
                let client = state.ctx.db_pool.get().await?;
                client
                    .execute(
                        "UPDATE iso20022.message_ledger
                         SET status='REJECTED',
                             fail_reason=$1,
                             updated_at=NOW(),
                             next_retry_at=NULL
                         WHERE message=$2",
                        &[&e.to_string(), &job.id],
                    )
                    .await?;
                return Ok(());
            }
        }
    }

    // Extract AppHdr metadata (sender/receiver BIC, BizMsgIdr, etc.) once so the
    // handler and flow coordinator can address any outbound response messages to
    // the correct counterparty.  Prior to this, handlers passed a hardcoded
    // "COUNTERPARTY" string which produced invalid outbound pacs.002/pacs.008.
    let counterparty_bic_owned: Option<String> = match std::str::from_utf8(&job.raw_xml) {
        Ok(xml_str) => {
            let meta = AppHdrMeta::extract(xml_str);
            meta.counterparty_bic(&state.ctx.config.own_bic).map(str::to_string)
        }
        Err(_) => None,
    };
    if counterparty_bic_owned.is_none() && detected.has_apphdr {
        warn!(
            msg_id = job.id, msg_type = %msg_type,
            "Could not resolve counterparty BIC from AppHdr; outbound responses will use 'UNKNOWN'"
        );
    }

    let outcome_result = cb
        .call(async {
            if let Some(handler) = get_handler(&msg_type) {
                match std::str::from_utf8(&job.raw_xml) {
                    Ok(xml_str) => match handler
                        .consume(&state.ctx, xml_str, counterparty_bic_owned.as_deref())
                        .await
                    {
                        Ok(_msg) => Ok(WorkerOutcome::Success(())),
                        Err(e) => {
                            if is_transient_error(&e) {
                                Err(e) // trips CB
                            } else {
                                Ok(WorkerOutcome::PermanentFailure(e))
                            }
                        }
                    },
                    Err(e) => Ok(WorkerOutcome::PermanentFailure(format!(
                        "Invalid UTF-8: {}",
                        e
                    ))),
                }
            } else {
                Ok(WorkerOutcome::NoHandler)
            }
        })
        .await;

    let outcome = match outcome_result {
        Ok(o) => o,
        Err(failsafe::Error::Inner(e)) => WorkerOutcome::TransientFailure(e),
        Err(failsafe::Error::Rejected) => WorkerOutcome::TransientFailure("Circuit Open".into()),
    };

    let mut client = state.ctx.db_pool.get().await?;
    let tx = client.transaction().await?;

    match outcome {
        WorkerOutcome::Success(_) => {
            // The handler only returns Success after its CRUD calls (journey
            // state, status history, CBS booking audit rows, ...) succeeded,
            // so this is a valid proxy for "CRUD gRPC is up" — see
            // `healthcheck.rs`'s `HealthReport::crud_ok`.
            state.health.record_crud_success();
            tx.execute(
                "UPDATE iso20022.message_ledger
                 SET status='COMPLETED',
                     updated_at=NOW(),
                     next_retry_at=NULL
                 WHERE message=$1",
                &[&job.id],
            )
            .await?;

            // NOTE: Customer-facing notifications (pain.002 etc.) are currently
            // enqueued to `outbound_queue` in the CBS via the CRUD service inside
            // FlowCoordinator::enqueue_all.  A SWIFT adapter in the CBS is
            // responsible for interbank delivery; the notification worker here
            // handles the customer-callback channel.
            //
            // To deliver pain.002 notifications via the customer callback URL, the
            // flow coordinator must also write pain.002 rows to the local
            // `message_ledger` (direction='OUT').  This requires either:
            //   a) Passing `&Pool` into `FlowCoordinator::execute` (preferred), OR
            //   b) Returning the messages up the call stack and inserting here.
            //
            // Until that refactor is done, customer callbacks are NOT sent for
            // inbound pain.001 / pacs.008.  Interbank messages still go to the CBS
            // outbound_queue correctly.

            tx.commit().await?;
            metrics::record_message_processed(&msg_type, "success");
            metrics::record_message_duration(&msg_type, start.elapsed());
            state.notify.notify_waiters();
        }
        WorkerOutcome::TransientFailure(e) => {
            // Exponential backoff + jitter based on the actual retry_count
            // returned by the claim query.  Previously this used a fixed 30s
            // base which meant inbound retries never actually backed off.
            let base = exp_backoff_secs(job.retry_count).clamp(30, 3600);
            let jittered = jitter_secs(base, job.id, job.retry_count);
            tx.execute(
                "UPDATE iso20022.message_ledger
                 SET status='FAILED',
                     retry_count=retry_count+1,
                     next_retry_at=NOW() + ($1 || ' seconds')::interval,
                     fail_reason=$2,
                     updated_at=NOW()
                 WHERE message=$3",
                &[&(jittered as i64), &e, &job.id],
            )
            .await?;
            tx.commit().await?;
            metrics::record_message_processed(&msg_type, "transient_failure");
            metrics::record_message_duration(&msg_type, start.elapsed());
        }
        WorkerOutcome::PermanentFailure(reason) => {
            tx.execute(
                "UPDATE iso20022.message_ledger
                 SET status='REJECTED',
                     fail_reason=$1,
                     updated_at=NOW(),
                     next_retry_at=NULL
                 WHERE message=$2",
                &[&reason, &job.id],
            )
            .await?;
            tx.commit().await?;
            metrics::record_message_processed(&msg_type, "permanent_failure");
            metrics::record_message_duration(&msg_type, start.elapsed());
        }
        WorkerOutcome::NoHandler => {
            tx.execute(
                "UPDATE iso20022.message_ledger
                 SET status='REJECTED',
                     fail_reason='No Handler',
                     updated_at=NOW(),
                     next_retry_at=NULL
                 WHERE message=$1",
                &[&job.id],
            )
            .await?;
            tx.commit().await?;
            metrics::record_message_processed(&msg_type, "no_handler");
            metrics::record_message_duration(&msg_type, start.elapsed());
        }
    }

    Ok(())
}

// --- NOTIFICATION PROCESSING ---

async fn process_single_notification(
    job: OutboundJob,
    state: AppState,
    cb: SharedCircuitBreaker,
) -> Result<()> {
    let client = state.ctx.db_pool.get().await?;

    // Send request inside CB
    let send_res = cb
        .call(async {
            let resp = state
                .ctx
                .http_client
                .post(state.customer_callback_url.as_str())
                .body(job.raw_xml.to_vec())
                .header("Content-Type", "application/xml")
                .header("X-Message-Id", job.id.to_string())
                .send()
                .await?;

            Ok::<reqwest::Response, reqwest::Error>(resp)
        })
        .await;

    match send_res {
        Ok(resp) => {
            let status = resp.status();

            if status.is_success() {
                client
                    .execute(
                        "UPDATE iso20022.message_ledger
                         SET status='SENT',
                             updated_at=NOW(),
                             next_retry_at=NULL,
                             fail_reason=NULL
                         WHERE message=$1",
                        &[&job.id],
                    )
                    .await
                    .map_err(|e| anyhow::anyhow!("DB update after notification send failed: {e}"))?;
                state.health.record_outbound_success();
                return Ok(());
            }

            // Retry only on retryable statuses:
            // - 429 Too Many Requests
            // - 5xx
            if status.as_u16() == 429 || status.is_server_error() {
                let base = exp_backoff_secs(job.retry_count).min(3600);
                let jittered = jitter_secs(base, job.id, job.retry_count);
                if let Err(e) = client
                    .execute(
                        "UPDATE iso20022.message_ledger
                         SET status='FAILED',
                             retry_count=retry_count+1,
                             next_retry_at=NOW()+ ($1 || ' seconds')::interval,
                             fail_reason=$2,
                             updated_at=NOW()
                         WHERE message=$3",
                        &[
                            &(jittered as i64),
                            &format!("HTTP {}", status.as_u16()),
                            &job.id,
                        ],
                    )
                    .await
                {
                    error!("Notification DB retry-update failed for job {}: {}", job.id, e);
                }
                return Ok(());
            }

            // Non-retryable 4xx -> permanent reject (prevents infinite retries on bad payload)
            if status.is_client_error() {
                if let Err(e) = client
                    .execute(
                        "UPDATE iso20022.message_ledger
                         SET status='REJECTED',
                             next_retry_at=NULL,
                             fail_reason=$1,
                             updated_at=NOW()
                         WHERE message=$2",
                        &[&format!("HTTP {}", status.as_u16()), &job.id],
                    )
                    .await
                {
                    error!("Notification DB reject-update failed for job {}: {}", job.id, e);
                }
                return Ok(());
            }

            // Fallback: treat as transient
            let base = exp_backoff_secs(job.retry_count).min(3600);
            let jittered = jitter_secs(base, job.id, job.retry_count);
            if let Err(e) = client
                .execute(
                    "UPDATE iso20022.message_ledger
                     SET status='FAILED',
                         retry_count=retry_count+1,
                         next_retry_at=NOW()+ ($1 || ' seconds')::interval,
                         fail_reason=$2,
                         updated_at=NOW()
                     WHERE message=$3",
                    &[
                        &(jittered as i64),
                        &format!("HTTP {}", status.as_u16()),
                        &job.id,
                    ],
                )
                .await
            {
                error!("Notification DB fallback-update failed for job {}: {}", job.id, e);
            }
        }
        Err(e) => {
            // CB rejects or reqwest error -> transient backoff
            let base = exp_backoff_secs(job.retry_count).min(3600);
            let jittered = jitter_secs(base, job.id, job.retry_count);
            if let Err(db_err) = client
                .execute(
                    "UPDATE iso20022.message_ledger
                     SET status='FAILED',
                         retry_count=retry_count+1,
                         next_retry_at=NOW()+ ($1 || ' seconds')::interval,
                         fail_reason=$2,
                         updated_at=NOW()
                     WHERE message=$3",
                    &[&(jittered as i64), &format!("{:?}", e), &job.id],
                )
                .await
            {
                error!("Notification DB error-update failed for job {}: {}", job.id, db_err);
            }
        }
    }

    Ok(())
}

// --- MAINTENANCE ---
async fn maintenance_loop(pool: Pool, notify: Arc<Notify>, token: CancellationToken, max_retries: i32) {
    let mut interval = tokio::time::interval(Duration::from_secs(5));

    loop {
        tokio::select! {
            _ = token.cancelled() => break,
            _ = interval.tick() => {
                let client = match pool.get().await {
                    Ok(c) => c,
                    Err(e) => { error!("Maintenance: DB Error {}", e); continue; }
                };

                // Reclaim stuck PROCESSING rows by lease expiration.
                let updated = client.execute(
                    "UPDATE iso20022.message_ledger
                     SET status='PENDING',
                         updated_at=NOW(),
                         next_retry_at=NULL
                     WHERE status='PROCESSING'
                       AND next_retry_at IS NOT NULL
                       AND next_retry_at <= NOW()",
                    &[]
                ).await;

                match updated {
                    Ok(n) if n > 0 => {
                        warn!("Maintenance reclaimed {} stuck PROCESSING rows", n);
                        metrics::record_maintenance_action("reclaim_stuck", n);
                        notify.notify_waiters();
                    }
                    Ok(_) => {}
                    Err(e) => error!("Maintenance reclaim failed: {}", e),
                }

                // Promote FAILED rows that have exhausted all retries to DEAD_LETTER.
                let dead = client.execute(
                    "UPDATE iso20022.message_ledger
                     SET status='DEAD_LETTER',
                         updated_at=NOW()
                     WHERE status='FAILED'
                       AND retry_count >= $1",
                    &[&max_retries]
                ).await;

                match dead {
                    Ok(n) if n > 0 => {
                        warn!("Maintenance moved {} messages to DEAD_LETTER", n);
                        metrics::record_maintenance_action("dead_letter_promote", n);
                    }
                    Ok(_) => {}
                    Err(e) => error!("Maintenance dead-letter promotion failed: {}", e),
                }
            }
        }
    }
}

// --- SLA TRACKING ---
//
// SlaTracker::check_breaches/escalate previously existed but were never
// invoked from anywhere, so recall/unable-to-apply/claim-non-receipt
// investigations never actually got escalated once they overran their SLA
// window. This loop polls on the configured interval and escalates any
// investigation still OPEN/PENDING past its deadline.
async fn sla_tracker_loop(
    mut crud: iso20022::context::CrudClient,
    config: InstitutionConfig,
    token: CancellationToken,
) {
    let mut interval = tokio::time::interval(config.sla_check_interval);
    loop {
        tokio::select! {
            _ = token.cancelled() => break,
            _ = interval.tick() => {
                match SlaTracker::check_breaches(&mut crud, &config).await {
                    Ok(breaches) => {
                        for breach in &breaches {
                            if let Err(e) = SlaTracker::escalate(&mut crud, breach).await {
                                error!(
                                    investigation_id = %breach.investigation_id,
                                    "SLA escalation failed: {}", e
                                );
                            }
                        }
                    }
                    Err(e) => error!("SLA breach check failed: {}", e),
                }
            }
        }
    }
}

// --- HTTP HANDLERS ---

#[derive(Serialize)]
struct WebhookResponse {
    message_id: Option<i64>,
    status: &'static str,
}

async fn handle_webhook(
    State(s): State<AppState>,
    headers: HeaderMap,
    body: Bytes,
) -> Result<impl IntoResponse, StatusCode> {
    // ── Authentication ────────────────────────────────────────────────────
    // TODO: Replace static API key with mTLS client certificates for SWIFT
    // connectivity, or an HSM-backed JWT for customer ERP integration.
    if let Some(expected_key) = &s.webhook_api_key {
        let bearer = headers
            .get("Authorization")
            .and_then(|v| v.to_str().ok())
            .and_then(|v| v.strip_prefix("Bearer "));
        match bearer {
            Some(token) if token == expected_key.as_str() => {}
            _ => {
                warn!("Webhook rejected: missing or invalid Authorization header");
                return Err(StatusCode::UNAUTHORIZED);
            }
        }
    }

    // ── Content-Type validation ───────────────────────────────────────────
    let ct = headers
        .get("Content-Type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("");
    if !ct.contains("xml") && !ct.contains("octet-stream") {
        warn!("Webhook rejected: unsupported Content-Type {:?}", ct);
        return Err(StatusCode::UNSUPPORTED_MEDIA_TYPE);
    }

    let body_clone = body.clone();
    let detected = spawn_blocking(move || detect_message(&body_clone))
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let msg_type = detected.doc_type.unwrap_or_else(|| {
        if detected.has_apphdr {
            "head.001".to_string()
        } else {
            "unknown".to_string()
        }
    });

    let mut hasher = Sha256::new();
    hasher.update(&body);
    let body_hash = format!("{:x}", hasher.finalize());

    let db = s
        .ctx
        .db_pool
        .get()
        .await
        .map_err(|_| StatusCode::SERVICE_UNAVAILABLE)?;

    let row = db
        .query_opt(
            "INSERT INTO iso20022.message_ledger (direction, status, raw_xml_bytes, msg_type, body_hash)
             VALUES ('IN', 'PENDING', $1, $2, $3)
             ON CONFLICT (body_hash) DO UPDATE
               SET updated_at = NOW()
               WHERE iso20022.message_ledger.status IN ('PENDING','FAILED')
             RETURNING message, status",
            &[&body.to_vec(), &msg_type, &body_hash],
        )
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    match row {
        Some(r) => {
            let msg_id: i64 = r.get(0);
            let status: String = r.get(1);

            // Wake workers: DB-driven claiming will pick it up.
            if status == "PENDING" || status == "FAILED" {
                s.notify.notify_waiters();
                return Ok((
                    StatusCode::ACCEPTED,
                    Json(WebhookResponse { message_id: Some(msg_id), status: "accepted" }),
                ));
            }
            // Duplicate body_hash — message already processed or in-flight.
            Ok((
                StatusCode::OK,
                Json(WebhookResponse { message_id: Some(msg_id), status: "duplicate" }),
            ))
        }
        None => Ok((
            StatusCode::OK,
            Json(WebhookResponse { message_id: None, status: "duplicate" }),
        )),
    }
}

/// Health endpoint — returns a JSON HealthReport.
/// HTTP 200 = healthy/degraded, HTTP 503 = unhealthy.
async fn health(State(s): State<AppState>) -> impl IntoResponse {
    let db_ok = s.ctx.db_pool.get().await.is_ok();
    if db_ok {
        s.health.record_db_success();
    }
    let report = s.health.report(db_ok);
    let status = if report.status == "unhealthy" {
        StatusCode::SERVICE_UNAVAILABLE
    } else {
        StatusCode::OK
    };
    (status, Json(report))
}

// --- UTILS ---

// Detect both AppHdr presence and the ISO20022 business document type.
// Uses Reader::buffer_position() to bound work reliably on malformed XML.
//
// High-throughput improvement: normalize Document xmlns URI into message id via split on last ':'.
fn detect_message(xml: &[u8]) -> DetectedMessage {
    let mut reader = Reader::from_reader(BufReader::new(Cursor::new(xml)));
    let mut buf = Vec::new();
    let mut has_apphdr = false;
    let mut doc_type: Option<String> = None;

    loop {
        if reader.buffer_position() as usize > MAX_XML_SCAN_BYTES {
            break;
        }

        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(ref e)) => {
                if e.name().as_ref() == b"AppHdr" {
                    has_apphdr = true;
                }

                if e.name().as_ref() == b"Document" && doc_type.is_none() {
                    for attr in e.attributes().flatten() {
                        let k = attr.key.as_ref();
                        if k == b"xmlns" || k.starts_with(b"xmlns:") {
                            let v = String::from_utf8_lossy(&attr.value);

                            // Normalize ISO20022 urn -> message id
                            if let Some(last) = v.rsplit(':').next() {
                                if last.contains("pain.")
                                    || last.contains("pacs.")
                                    || last.contains("camt.")
                                {
                                    doc_type = Some(last.to_string());
                                    break;
                                }
                            }

                            // Fallback legacy mapping
                            if v.contains("pain.001") {
                                doc_type = Some("pain.001.001.12".into());
                                break;
                            }
                            if v.contains("pain.002") {
                                doc_type = Some("pain.002.001.14".into());
                                break;
                            }
                            if v.contains("pacs.008") {
                                doc_type = Some("pacs.008.001.13".into());
                                break;
                            }
                            if v.contains("camt.053") {
                                doc_type = Some("camt.053.001.13".into());
                                break;
                            }
                        }
                    }
                }

                if doc_type.is_none() {
                    let name_bytes = e.name(); // keep it alive
                    let name = String::from_utf8_lossy(name_bytes.as_ref());
                    if name.contains("pain.") || name.contains("pacs.") || name.contains("camt.") {
                        doc_type = Some(name.to_string());
                    }
                }

                if doc_type.is_some() && has_apphdr {
                    break;
                }
            }
            Ok(Event::Eof) => break,
            Err(_) => break,
            _ => {}
        }

        buf.clear();
    }

    DetectedMessage {
        doc_type,
        has_apphdr,
    }
}

// Centralized transient classification (still string-based due to handler API).
fn is_transient_error(e: &str) -> bool {
    let s = e.to_lowercase();
    let patterns = [
        "timeout",
        "timed out",
        "deadline",
        "connection",
        "connect",
        "refused",
        "reset",
        "broken pipe",
        "eof",
        "dns",
        "tls",
        "ssl",
        "network",
        "unavailable",
        "temporarily",
        "too many requests",
        "429",
        "503",
        "502",
        "gateway",
    ];
    patterns.iter().any(|p| s.contains(p))
}

// 2^retries with cap safety; returns seconds.
fn exp_backoff_secs(retries: i32) -> u64 {
    let r = retries.max(0) as u32;
    2u64.checked_pow(r).unwrap_or(3600).min(3600)
}

// Deterministic jitter in [0.7, 1.3] without extra crate.
fn jitter_secs(base: u64, msg_id: i64, retries: i32) -> u64 {
    let mut x = (msg_id as u64) ^ ((retries as u64).wrapping_mul(0x9E37_79B9_7F4A_7C15));
    x ^= x >> 12;
    x ^= x << 25;
    x ^= x >> 27;
    let frac = (x % 601) as i64 - 300; // [-300..300]
    let milli = 1000i64 + frac; // [700..1300]
    let jittered = (base as i128 * milli as i128) / 1000i128;
    jittered.max(1) as u64
}

fn build_db_pool(cfg: &AppConfig) -> Result<Pool> {
    let mut c = tokio_postgres::Config::new();
    c.user(&cfg.user)
        .host(&cfg.host)
        .dbname(&cfg.dbname)
        .password(&cfg.password);

    // TODO: Enable TLS for DB connections in production.
    // Replace `NoTls` with `tokio_postgres_rustls::MakeRustlsConnect` (add dep) and
    // supply a CA certificate bundle via DB_TLS_CA_CERT env var.
    let mgr = Manager::from_config(
        c,
        NoTls,
        ManagerConfig {
            recycling_method: RecyclingMethod::Fast,
        },
    );

    // Pool max: explicit env override, else workers + maintenance headroom.
    let pool_size = cfg.db_pool_max.unwrap_or(
        cfg.payment_worker_count + cfg.notification_worker_count + 8,
    );

    Pool::builder(mgr)
        .max_size(pool_size)
        .build()
        .context("Pool build failed")
}

async fn shutdown(token: CancellationToken) {
    // Handle both Ctrl-C (SIGINT) and SIGTERM (sent by container runtimes).
    #[cfg(unix)]
    {
        use tokio::signal::unix::{signal, SignalKind};
        let mut sigterm = match signal(SignalKind::terminate()) {
            Ok(s) => s,
            Err(e) => {
                warn!("Failed to register SIGTERM handler: {e}; falling back to SIGINT only");
                token.cancel();
                return;
            }
        };
        tokio::select! {
            _ = signal::ctrl_c() => { info!("Received SIGINT — shutting down"); }
            _ = sigterm.recv()   => { info!("Received SIGTERM — shutting down"); }
        }
    }
    #[cfg(not(unix))]
    {
        signal::ctrl_c().await.ok();
        info!("Received Ctrl-C — shutting down");
    }
    token.cancel();
}
