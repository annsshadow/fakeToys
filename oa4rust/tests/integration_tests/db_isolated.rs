//! Helper to create an isolated test database per test (no shared OnceLock).
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

use super::db::{TestPool, TestContext, setup_postgres_database};

static SEQ: AtomicUsize = AtomicUsize::new(0);

/// Create a fresh test database with a unique name.
pub async fn init_isolated() -> Arc<TestContext> {
    let seq = SEQ.fetch_add(1, Ordering::Relaxed);
    let db_name = format!("oa4rust_test_iso_{}", seq);
    let p = setup_postgres_database(&db_name)
        .await
        .expect("failed to set up isolated PostgreSQL test database");
    Arc::new(TestContext {
        pool: Arc::new(TestPool::Postgres(p)),
        db_name,
    })
}
