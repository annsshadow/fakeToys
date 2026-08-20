use std::sync::{Arc, OnceLock};

use anyhow::Context as _;
use deadpool_postgres::tokio_postgres::{Config, NoTls};
use deadpool_postgres::{Manager, Pool};
use mysql_async::Opts;
use mysql_async::prelude::Queryable;
use tokio::runtime::Handle;
use tracing::info;

// ──────────────────────────────────────────────────────────────────────────────
// Integration test database lifecycle
//
// One disposable database per test process, named oa4rust_test_<pid>.
// Created lazily on first use, dropped on process exit via Drop guard.
//
// Supports both PostgreSQL (default) and MySQL (DATABASE_DIALECT=mysql).
// MySQL: migrations run via SQL rewriter; scenario tests require PostgreSQL.
// ──────────────────────────────────────────────────────────────────────────────

pub static TEST_DB: OnceLock<Arc<TestPool>> = OnceLock::new();
pub static TEST_DB_NAME: OnceLock<String> = OnceLock::new();

/// Test database pool abstraction
#[derive(Clone)]
pub enum TestPool {
    Postgres(Pool),
    MySQL(Arc<mysql_async::Pool>),
}

impl TestPool {
    pub fn as_pg(&self) -> Option<&Pool> {
        match self {
            TestPool::Postgres(p) => Some(p),
            TestPool::MySQL(_) => None,
        }
    }
}

/// Test database context: owns the database name, drops the DB on drop.
pub struct TestContext {
    pub pool: Arc<TestPool>,
    pub db_name: String,
}

impl TestContext {
    pub fn pool(&self) -> Arc<TestPool> {
        self.pool.clone()
    }
}

impl Drop for TestContext {
    fn drop(&mut self) {
        let db_name = self.db_name.clone();
        if let Ok(handle) = Handle::try_current() {
            handle.spawn(async move {
                if let Err(e) = drop_database(&db_name).await {
                    tracing::warn!(db = %db_name, error = %e, "failed to drop test database");
                }
            });
        } else {
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

fn is_mysql_dialect() -> bool {
    std::env::var("DATABASE_DIALECT")
        .or_else(|_| std::env::var("DB_DIALECT"))
        .map(|d| d.eq_ignore_ascii_case("mysql"))
        .unwrap_or(false)
}

/// Initialize the test database and run migrations.
pub fn init_test_database() -> Arc<TestContext> {
    let db_name = format!("oa4rust_test_{}", std::process::id());
    TEST_DB_NAME.get_or_init(|| db_name.clone());

    let pool = TEST_DB
        .get_or_init(|| {
            if is_mysql_dialect() {
                match init_mysql_database(&db_name) {
                    Ok(p) => return Arc::new(p),
                    Err(e) => panic!("failed to init MySQL test database: {}", e),
                }
            }
            let rt = runtime_or_new();
            let p = rt
                .handle()
                .block_on(async { setup_postgres_database(&db_name).await })
                .expect("failed to set up PostgreSQL test database");
            Arc::new(TestPool::Postgres(p))
        })
        .clone();

    Arc::new(TestContext {
        pool: pool.clone(),
        db_name: db_name.clone(),
    })
}

/// Get the already-initialized test pool, or panic with a clear message.
pub fn test_pool() -> Arc<TestPool> {
    TEST_DB
        .get()
        .expect("test database not initialized; call init_test_database() first")
        .clone()
}

/// Get a mutable reference to the OnceLock so the test binary can call init.
pub fn lazy_lock() -> &'static OnceLock<Arc<TestPool>> {
    &TEST_DB
}

fn runtime_or_new() -> std::sync::Arc<tokio::runtime::Runtime> {
    match Handle::try_current() {
        Ok(_) => Arc::new(
            tokio::runtime::Runtime::new()
                .expect("failed to create fallback tokio runtime"),
        ),
        Err(_) => {
            let rt = tokio::runtime::Runtime::new()
                .expect("failed to create tokio runtime for test db setup");
            Arc::new(rt)
        }
    }
}

async fn setup_postgres_database(test_db_name: &str) -> anyhow::Result<Pool> {
    dotenvy::dotenv().ok();

    let base_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| {
        "postgres://o2server:password@localhost:5432/postgres".to_string()
    });

    info!(db = %test_db_name, "setting up PostgreSQL integration test database");

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

    info!(db = %test_db_name, "PostgreSQL test database created");

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

    shared::migrate::run_migrations(&test_pool)
        .await
        .context("migrations failed")?;

    info!(db = %test_db_name, "PostgreSQL migrations applied");

    Ok(test_pool)
}

// ──────────────────────────────────────────────────────────────────────────────
// MySQL test database initialization
// ──────────────────────────────────────────────────────────────────────────────

fn encode_url_component(s: &str) -> String {
    s.chars()
        .map(|c| match c {
            'A'..='Z' | 'a'..='z' | '0'..='9' | '-' | '_' | '.' | '~' => c.to_string(),
            ' ' => "%20".to_string(),
            _ => format!("%{:02X}", c as u8),
        })
        .collect()
}

fn build_mysql_url(
    host: &str,
    port: u16,
    user: &str,
    password: Option<&str>,
    dbname: &str,
) -> String {
    let encoded_user = encode_url_component(user);
    let auth = match password {
        Some(pw) => format!("{}:{}", encoded_user, encode_url_component(pw)),
        None => encoded_user,
    };
    let db_part = if dbname.is_empty() {
        String::new()
    } else {
        format!("/{}", encode_url_component(dbname))
    };
    format!("mysql://{}{}:{}{}", auth, host, port, db_part)
}

fn init_mysql_database(db_name: &str) -> anyhow::Result<TestPool> {
    let base_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "mysql://o2server:password@localhost:3306/oa4rust".to_string());

    let url = url::Url::parse(&base_url).context("invalid DATABASE_URL")?;
    let host = url.host_str().context("no host in DATABASE_URL")?.to_string();
    let port = url.port().unwrap_or(3306);
    let user = url.username().to_string();
    let password_opt = url.password().map(|s| s.to_string());

    let admin_url = build_mysql_url(&host, port, &user, password_opt.as_deref(), "");

    let rt = Arc::new(
        tokio::runtime::Runtime::new()
            .expect("failed to create tokio runtime for MySQL test db setup"),
    );

    rt.block_on(async {
        let admin_opts = Opts::from_url(&admin_url)
            .expect("failed to parse MySQL admin URL");
        let admin_pool = mysql_async::Pool::new(admin_opts);

        let mut admin_conn = admin_pool
            .get_conn()
            .await
            .context("failed to get MySQL admin connection")?;

        admin_conn
            .exec_drop(format!("DROP DATABASE IF EXISTS `{}`", db_name), ())
            .await
            .context("failed to drop existing MySQL test database")?;

        admin_conn
            .exec_drop(format!("CREATE DATABASE `{}`", db_name), ())
            .await
            .context("failed to create MySQL test database")?;

        tracing::info!(db = %db_name, "MySQL test database created");

        let test_url = build_mysql_url(&host, port, &user, password_opt.as_deref(), db_name);
        let test_pool = Arc::new(mysql_async::Pool::new(
            Opts::from_url(&test_url).expect("failed to parse MySQL test URL")
        ));

        run_mysql_migrations(&test_pool)
            .await
            .context("MySQL migrations failed")?;

        tracing::info!(db = %db_name, "MySQL migrations applied");

        anyhow::Ok(TestPool::MySQL(test_pool))
    })
}

async fn run_mysql_migrations(pool: &mysql_async::Pool) -> anyhow::Result<()> {
    use mysql_async::prelude::Queryable;
    use mysql_async::params::Params;

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

    let mut conn = pool
        .get_conn()
        .await
        .context("failed to acquire MySQL migration connection")?;

    let d = shared::db::dialect();
    let create_table_sql = d.format_sql(
        "CREATE TABLE IF NOT EXISTS schema_migrations (\n\
            version     TEXT PRIMARY KEY,\n\
            applied_at  TIMESTAMPTZ NOT NULL DEFAULT now(),\n\
            checksum    TEXT NOT NULL,\n\
            execution_ms INTEGER NOT NULL DEFAULT 0\n\
        );",
    );
    conn.exec_drop(create_table_sql, Params::from(()))
        .await
        .context("failed to create schema_migrations table in MySQL")?;

    for entry in entries {
        let path = entry.path();
        let sql = std::fs::read_to_string(&path)
            .with_context(|| format!("failed to read migration: {}", path.display()))?;

        if sql.trim().is_empty() {
            continue;
        }

        let rewritten = d.format_sql(&sql);
        conn.exec_drop(rewritten, Params::from(()))
            .await
            .with_context(|| format!("MySQL migration failed: {}", path.display()))?;
    }

    Ok(())
}
