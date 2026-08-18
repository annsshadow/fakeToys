use std::sync::{Arc, OnceLock};

use anyhow::Context as _;
use deadpool_postgres::tokio_postgres::{Config, NoTls};
use deadpool_postgres::{Manager, Pool};
use tokio::runtime::Handle;
use tracing::info;

// ──────────────────────────────────────────────────────────────────────────────
// Integration test database lifecycle
//
// One disposable database per test process, named oa4rust_test_<pid>.
// Created lazily on first use, dropped on process exit via Drop guard.
// ──────────────────────────────────────────────────────────────────────────────

pub static TEST_DB: OnceLock<Arc<Pool>> = OnceLock::new();
pub static TEST_DB_NAME: OnceLock<String> = OnceLock::new();

/// Test database context: owns the database name, drops the DB on drop.
/// The pool itself is stored separately in TEST_DB so it remains available
/// after the context is dropped.
pub struct TestContext {
    pool: Arc<Pool>,
    db_name: String,
}

impl TestContext {
    /// Returns a clone of the shared pool for use in scenarios.
    pub fn pool(&self) -> Arc<Pool> {
        self.pool.clone()
    }
}

impl Drop for TestContext {
    fn drop(&mut self) {
        let db_name = self.db_name.clone();
        // Use try_current to avoid panicking if no tokio runtime is active.
        if let Ok(handle) = Handle::try_current() {
            handle.spawn(async move {
                if let Err(e) = drop_database(&db_name).await {
                    tracing::warn!(db = %db_name, error = %e, "failed to drop test database");
                }
            });
        } else {
            // Fallback: run synchronously (may block but ensures cleanup).
            let rt = tokio::runtime::Runtime::new().ok();
            if let Some(r) = rt {
                r.block_on(async {
                    if let Err(e) = drop_database(&db_name).await {
                        tracing::warn!(db = %db_name, error = %e, "failed to drop test database");
                    }
                });
            }
        }
    }
}

async fn drop_database(db_name: &str) -> anyhow::Result<()> {
    let base_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| {
        "postgres://o2server:password@localhost:5432/postgres".to_string()
    });
    let url = url::Url::parse(&base_url).context("invalid DATABASE_URL")?;
    let host = url.host_str().context("no host in DATABASE_URL")?;
    let port = url.port().unwrap_or(5432);
    let user = url.username();
    let password = url.password().unwrap_or("");

    let mut cfg = Config::new();
    cfg.host(host).port(port).user(user).password(password).dbname("postgres");
    let mgr = Manager::new(cfg, NoTls);
    let admin_pool = Pool::builder(mgr).max_size(2).build().context("failed to build admin pool")?;
    let client = admin_pool.get().await.context("failed to acquire admin connection")?;
    // Terminate all other connections first
    let _ = client.execute(
        &format!(
            "SELECT pg_terminate_backend(pg_stat_activity.pid) \
             FROM pg_stat_activity \
             WHERE pg_stat_activity.datname = '{}' \
             AND pid <> pg_backend_pid()",
            db_name
        ),
        &[],
    )
    .await;
    client
        .execute(&format!("DROP DATABASE IF EXISTS \"{}\"", db_name), &[])
        .await
        .context("failed to drop test database")?;
    Ok(())
}

/// Initialize the test database and run migrations.
/// Must be called before any async test code. Safe to call multiple times;
/// subsequent calls return the same context.
///
/// Returns an Arc<TestContext> which keeps the database alive until dropped.
/// Call .pool() to get the shared Arc<Pool> for test code.
pub fn init_test_database() -> Arc<TestContext> {
    let db_name = format!("oa4rust_test_{}", std::process::id());
    TEST_DB_NAME.get_or_init(|| db_name.clone());

    let pool = TEST_DB
        .get_or_init(|| {
            let rt = runtime_or_new();
            let p = rt
                .handle()
                .block_on(async { setup_database(&db_name).await })
                .expect("failed to set up test database");
            Arc::new(p)
        })
        .clone();

    Arc::new(TestContext {
        pool: pool.clone(),
        db_name: db_name.clone(),
    })
}

/// Get the already-initialized test pool, or panic with a clear message.
pub fn test_pool() -> Arc<Pool> {
    TEST_DB
        .get()
        .expect("test database not initialized; call init_test_database() first")
        .clone()
}

/// Get a mutable reference to the OnceLock so the test binary can call init.
pub fn lazy_lock() -> &'static OnceLock<Arc<Pool>> {
    &TEST_DB
}

fn runtime_or_new() -> std::sync::Arc<tokio::runtime::Runtime> {
    match Handle::try_current() {
        Ok(h) => {
            // Already in a runtime; return a no-op Arc so the handle stays valid
            // by relying on the existing runtime. We still need to return an Arc
            // for the caller, so create a new runtime and return its Arc.
            Arc::new(
                tokio::runtime::Runtime::new()
                    .expect("failed to create fallback tokio runtime"),
            )
        }
        Err(_) => {
            let rt = tokio::runtime::Runtime::new()
                .expect("failed to create tokio runtime for test db setup");
            Arc::new(rt)
        }
    }
}

async fn setup_database(test_db_name: &str) -> anyhow::Result<Pool> {
    dotenvy::dotenv().ok();

    let base_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| {
        "postgres://o2server:password@localhost:5432/postgres".to_string()
    });

    info!(db = %test_db_name, "setting up integration test database");

    let url = url::Url::parse(&base_url).context("invalid DATABASE_URL")?;
    let host = url.host_str().context("no host in DATABASE_URL")?;
    let port = url.port().unwrap_or(5432);
    let user = url.username();
    let password = url.password().unwrap_or("");

    let mut admin_cfg = Config::new();
    admin_cfg
        .host(host)
        .port(port)
        .user(user)
        .password(password)
        .dbname("postgres");

    let admin_mgr = Manager::new(admin_cfg, NoTls);
    let admin_pool = Pool::builder(admin_mgr)
        .max_size(5)
        .build()
        .context("failed to build admin pool")?;

    let admin_client = admin_pool
        .get()
        .await
        .context("failed to acquire admin connection")?;

    admin_client
        .execute(&format!("DROP DATABASE IF EXISTS \"{}\"", test_db_name), &[])
        .await
        .context("failed to drop existing test database")?;

    admin_client
        .execute(
            &format!("CREATE DATABASE \"{}\"", test_db_name),
            &[],
        )
        .await
        .context("failed to create test database")?;

    info!(db = %test_db_name, "test database created");

    let mut test_cfg = Config::new();
    test_cfg
        .host(host)
        .port(port)
        .user(user)
        .password(password)
        .dbname(test_db_name);

    let test_mgr = Manager::new(test_cfg, NoTls);
    let test_pool = Pool::builder(test_mgr)
        .max_size(10)
        .build()
        .context("failed to build test pool")?;

    run_migrations(&test_pool)
        .await
        .context("migrations failed")?;

    info!(db = %test_db_name, "migrations applied");

    Ok(test_pool)
}

/// Execute all SQL files in the migrations/ directory in lexicographic order.
async fn run_migrations(pool: &Pool) -> anyhow::Result<()> {
    let migrations_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("migrations");

    if !migrations_dir.exists() {
        return Ok(());
    }

    let mut entries: Vec<_> = std::fs::read_dir(&migrations_dir)
        .context("failed to read migrations directory")?
        .filter_map(|e| e.ok())
        .filter(|e| {
            let path = e.path();
            path.extension().map(|x| x == "sql").unwrap_or(false)
                && !path.file_stem()
                    .and_then(|s| s.to_str())
                    .map(|s| s.ends_with("_rollback"))
                    .unwrap_or(false)
        })
        .collect();

    entries.sort_by_key(|e| e.path());

    let client = pool.get().await.context("failed to acquire migration connection")?;

    client
        .batch_execute(
            "CREATE TABLE IF NOT EXISTS schema_migrations (
                version     TEXT PRIMARY KEY,
                applied_at  TIMESTAMPTZ NOT NULL DEFAULT now(),
                checksum    TEXT NOT NULL,
                execution_ms INTEGER NOT NULL DEFAULT 0
            );",
        )
        .await
        .context("failed to create schema_migrations table")?;

    for entry in entries {
        let path = entry.path();
        let sql = std::fs::read_to_string(&path)
            .with_context(|| format!("failed to read migration: {}", path.display()))?;

        if sql.trim().is_empty() {
            continue;
        }

        client
            .batch_execute(&sql)
            .await
            .with_context(|| format!("migration failed: {}", path.display()))?;
    }

    Ok(())
}
