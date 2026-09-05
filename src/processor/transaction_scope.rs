use crate::context::CrudClient;
use crate::crudgrpc::{CreateRequest, DbPath, Fld, ReadRequest, UpdateRequest};

/// A buffered operation that will be flushed on commit.
#[derive(Debug, Clone)]
enum CrudOp {
    Create(CreateRequest),
    Update(UpdateRequest),
}

/// `TransactionScope` accumulates CRUD writes and flushes them in a single
/// sequential burst on `commit()`.  If the scope is dropped without commit,
/// no writes are sent (implicit rollback).
///
/// Reads are executed immediately (they must return data for subsequent logic),
/// but writes are deferred.
///
/// # Limitations
///
/// This is *not* a database transaction — the CRUD service does not expose
/// multi-operation atomic commits.  What this gives us:
///
/// - **All-or-nothing intent**: if the handler panics or returns Err before
///   `commit()`, zero writes escape.
/// - **Reduced round-trips**: writes are batched and sent sequentially at the
///   end rather than interleaved with reads throughout the handler.
/// - **Idempotent replay safety**: callers still rely on the per-operation
///   dedup guards (e.g. `archive_message` checks existence before inserting),
///   so a retry after partial commit is safe.
///
/// For true atomicity, the CRUD service would need a `BeginTx / CommitTx`
/// envelope — this scope is designed to be upgraded to that when available.
pub struct TransactionScope {
    crud: CrudClient,
    pending: Vec<CrudOp>,
    committed: bool,
}

impl TransactionScope {
    pub fn new(crud: CrudClient) -> Self {
        Self { crud, pending: Vec::new(), committed: false }
    }

    /// Mutable reference to the underlying CRUD client for immediate reads.
    pub fn crud(&mut self) -> &mut CrudClient {
        &mut self.crud
    }

    // ── Deferred writes ──────────────────────────────────────────────────

    pub fn stage_create(&mut self, req: CreateRequest) {
        self.pending.push(CrudOp::Create(req));
    }

    pub fn stage_update(&mut self, req: UpdateRequest) {
        self.pending.push(CrudOp::Update(req));
    }

    // ── Convenience: read-through helpers ────────────────────────────────

    pub async fn read_one(
        &mut self, table: &str, column: &str, where_clause: Vec<Fld>,
    ) -> Result<Option<String>, tonic::Status> {
        let req = ReadRequest {
            path: Some(DbPath { server: "CBS".into(), schema: "iso20022".into(), table: table.into() }),
            column: vec![column.into()],
            where_clause,
            page: 0,
            page_size: Some(1),
        };
        let resp = self.crud.read(req).await?.into_inner();
        Ok(resp.data.into_iter().next().and_then(|r| r.list.into_iter().next()))
    }

    // ── Commit ───────────────────────────────────────────────────────────

    /// Flush all staged writes to the CRUD service.  Returns the count of
    /// operations sent.  If any write fails, returns the error immediately
    /// (remaining ops are NOT sent — partial commit).
    ///
    /// **Partial-commit observability**: when a failure occurs mid-batch the
    /// error is logged with `sent` (already dispatched) and `total` (intended)
    /// counts so that an operator can identify which operations may need manual
    /// compensation.  The CRUD service does not expose multi-op transactions,
    /// so true atomicity requires a `BeginTx / CommitTx` envelope — upgrade
    /// this method when that becomes available.
    pub async fn commit(mut self) -> Result<usize, tonic::Status> {
        let total = self.pending.len();
        let mut sent = 0usize;
        for op in self.pending.drain(..) {
            let result = match op {
                CrudOp::Create(req) => self.crud.create(req).await.map(|_| ()),
                CrudOp::Update(req) => self.crud.update(req).await.map(|_| ()),
            };
            if let Err(e) = result {
                tracing::error!(
                    sent,
                    total,
                    error = %e,
                    "TransactionScope partial commit: {sent}/{total} ops sent before failure; \
                     manual compensation may be required for already-sent operations",
                );
                return Err(e);
            }
            sent += 1;
        }
        self.committed = true;
        Ok(sent)
    }

    /// Explicitly discard all staged operations.
    pub fn rollback(mut self) {
        self.pending.clear();
        self.committed = true; // suppress Drop warning
    }
}

impl Drop for TransactionScope {
    fn drop(&mut self) {
        if !self.committed && !self.pending.is_empty() {
            tracing::warn!(
                "TransactionScope dropped with {} uncommitted operations (implicit rollback)",
                self.pending.len()
            );
        }
    }
}
