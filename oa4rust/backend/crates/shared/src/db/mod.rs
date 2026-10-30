// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

// ──────────────────────────────────────────────────────────────────────────────
// db — 数据库连接池 + SQL 方言抽象
// ──────────────────────────────────────────────────────────────────────────────

pub mod dialect;
pub mod rewriter;

use deadpool_postgres::tokio_postgres::{Config, NoTls};
use deadpool_postgres::{Manager, Pool};
use dotenvy::dotenv;
use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use std::env;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DbError {
    #[error("connection pool error: {0}")]
    PoolError(String),
}

impl From<deadpool_postgres::PoolError> for DbError {
    fn from(e: deadpool_postgres::PoolError) -> Self {
        DbError::PoolError(e.to_string())
    }
}

/// 两个连接池共用的 sizing / TLS 配置——单一事实来源。
///
/// 历史上 `create_pool`（deadpool）不设容量上限、`create_sea_orm_pool`
/// 硬编码 `max_connections(20)`，两处各写各的。现统一由本结构派生：
///   - `DB_POOL_MAX_SIZE`  最大连接数（默认 20；非法值回退默认，最小钳到 1）
///   - `DB_POOL_MIN_IDLE`  最小空闲连接（默认 0 不预热，避免无库环境启动即建连；
///     仅 SeaORM 池生效——deadpool 的 PoolBuilder 无 min_idle API）
///   - `DB_TLS`            "1"/"true"/"on"/"yes" 开启 TLS（默认关，本地开发 NoTls）
///
/// TLS 语义（不引入任何真实凭证）：
///   - deadpool 池：`DB_TLS` 开 → rustls（webpki 根证书）连接器；
///   - SeaORM 池：`DB_TLS` 开 → 给 URL 追加 `sslmode=require`，
///     由 sqlx 的 runtime-tokio-rustls 后端协商 TLS。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DbPoolConfig {
    pub max_size: usize,
    pub min_idle: usize,
    pub tls: bool,
}

impl Default for DbPoolConfig {
    fn default() -> Self {
        Self {
            max_size: 20,
            min_idle: 0,
            tls: false,
        }
    }
}

impl DbPoolConfig {
    /// 纯解析逻辑（与 env 读取分离，便于无副作用单测）。
    fn resolve(max_size: Option<&str>, min_idle: Option<&str>, tls: Option<&str>) -> Self {
        let mut cfg = Self::default();
        if let Some(v) = max_size {
            if let Ok(n) = v.trim().parse::<usize>() {
                cfg.max_size = n.max(1);
            }
        }
        if let Some(v) = min_idle {
            if let Ok(n) = v.trim().parse::<usize>() {
                cfg.min_idle = n;
            }
        }
        if let Some(v) = tls {
            cfg.tls = is_truthy(v);
        }
        cfg
    }

    /// 从环境变量读取池配置。
    pub fn from_env() -> Self {
        Self::resolve(
            env::var("DB_POOL_MAX_SIZE").ok().as_deref(),
            env::var("DB_POOL_MIN_IDLE").ok().as_deref(),
            env::var("DB_TLS").ok().as_deref(),
        )
    }
}

/// 统一读取池配置（env）。deadpool 池与 SeaORM 池都从这里派生 sizing/TLS。
pub fn pool_config() -> DbPoolConfig {
    DbPoolConfig::from_env()
}

fn is_truthy(v: &str) -> bool {
    matches!(
        v.trim().to_ascii_lowercase().as_str(),
        "1" | "true" | "on" | "yes"
    )
}

const DEFAULT_DATABASE_URL: &str = "postgres://o2server:password@localhost:5432/oa4rust";

/// 解析 DATABASE_URL 环境变量，缺省回退到本地默认值。抽成纯函数便于无环境
/// 副作用地测试回退分支（测试直接喂 `Err`，不必真的清除进程级环境变量——
/// 否则会与并行跑的其他读 DATABASE_URL 的测试（如 crud 真库臂）互相污染）。
fn resolve_database_url(v: Result<String, std::env::VarError>) -> String {
    v.unwrap_or_else(|_| DEFAULT_DATABASE_URL.to_string())
}

fn database_url() -> String {
    dotenv().ok();
    resolve_database_url(env::var("DATABASE_URL"))
}

/// 给连接 URL 追加 sslmode 查询参数。SeaORM/sqlx 从 URL query 解析 sslmode，
/// TLS 后端由 sqlx 的 runtime-tokio-rustls feature 提供，无需额外依赖。
fn with_sslmode(url: &str, mode: &str) -> String {
    let sep = if url.contains('?') { '&' } else { '?' };
    format!("{url}{sep}sslmode={mode}")
}

/// rustls TLS connector（webpki 系统根证书集合）。仅 `DB_TLS` 开启时构造。
///
/// 显式指定 ring provider（不依赖进程级默认 provider 的解析），
/// 协议版本取安全默认集；ring provider 必然支持该组合。
fn rustls_connector() -> tokio_postgres_rustls::MakeRustlsConnect {
    let mut roots = rustls::RootCertStore::empty();
    roots.extend(webpki_roots::TLS_SERVER_ROOTS.iter().cloned());
    let config = rustls::ClientConfig::builder_with_provider(std::sync::Arc::new(
        rustls::crypto::ring::default_provider(),
    ))
    .with_safe_default_protocol_versions()
    .expect("ring provider supports safe default TLS versions")
    .with_root_certificates(roots)
    .with_no_client_auth();
    tokio_postgres_rustls::MakeRustlsConnect::new(config)
}

/// 用显式 URL 构建 deadpool 池（TLS/sizing 仍从 `pool_config()` 读环境变量）。
///
/// 独立成带参版本的原因：测试要验证「非法 URL / 指定 URL」路径时**不修改进程级
/// `DATABASE_URL`**——并发跑的其他测试（crud 真库臂等）会读到脏值（曾导致
/// `RelativeUrlWithoutBase` / Connection refused 随机红）。生产入口
/// [`create_pool`] 读取环境变量后委托到这里。
pub async fn create_pool_with_url(database_url: String) -> Result<Pool, DbError> {
    let pool_cfg = pool_config();

    let _dialect_name = env::var("DB_DIALECT")
        .or_else(|_| env::var("DATABASE_DIALECT"))
        .unwrap_or_else(|_| "postgres".to_string());

    let url = url::Url::parse(&database_url).expect("invalid DATABASE_URL");
    let host = url.host_str().expect("no host in DATABASE_URL");
    let port = url.port().unwrap_or(5432);
    let user = url.username();
    let password = url.password().unwrap_or("");
    let dbname = url.path().trim_start_matches('/');

    let mut cfg = Config::new();
    cfg.host(host)
        .port(port)
        .user(user)
        .password(password)
        .dbname(dbname);

    let mgr = if pool_cfg.tls {
        Manager::new(cfg, rustls_connector())
    } else {
        Manager::new(cfg, NoTls)
    };
    let pool = Pool::builder(mgr)
        .max_size(pool_cfg.max_size)
        .build()
        .map_err(|e| DbError::PoolError(e.to_string()))?;

    Ok(pool)
}

pub async fn create_pool() -> Result<Pool, DbError> {
    create_pool_with_url(database_url()).await
}

/// 用显式 URL 创建 SeaORM DatabaseConnection（sizing/TLS 与 deadpool 池同源，
/// 见 `DbPoolConfig`）。懒连接（connect_lazy）：与 deadpool 池一致，构建时不实际
/// 建连——无库环境（如 arity 探针、core_entity 的 HTTP 臂测试）也能构建成功，
/// 首次查询才尝试连接。
///
/// 独立成带参版本的原因同 [`create_pool_with_url`]：测试不修改进程级
/// `DATABASE_URL`，避免与并行测试互相污染。
pub async fn create_sea_orm_pool_with_url(
    database_url: String,
) -> Result<DatabaseConnection, DbError> {
    let pool_cfg = pool_config();
    let database_url = if pool_cfg.tls {
        with_sslmode(&database_url, "require")
    } else {
        database_url
    };

    let mut options = ConnectOptions::new(database_url);
    options
        .max_connections(pool_cfg.max_size.min(u32::MAX as usize) as u32)
        .min_connections(pool_cfg.min_idle.min(u32::MAX as usize) as u32)
        .sqlx_logging(false)
        .connect_lazy(true);

    Database::connect(options)
        .await
        .map_err(|e| DbError::PoolError(e.to_string()))
}

pub async fn create_sea_orm_pool() -> Result<DatabaseConnection, DbError> {
    create_sea_orm_pool_with_url(database_url()).await
}

/// LIKE/ILIKE 通配符转义（% _ \），防关键词注入通配扫描；
/// 转义后的值方可拼入 '%' || $1 || '%' 类模式绑定。对齐 o2server
/// StringTools.escapeSqlLikeKey（bbs/personal/query_service 既有同型助手）。
pub fn escape_like(input: &str) -> String {
    input
        .replace('\\', "\\\\")
        .replace('%', "\\%")
        .replace('_', "\\_")
}

pub use dialect::{dialect, MySQLDialect, PostgresDialect, SqlDialect};
pub use rewriter::rewrite_pg_to_mysql;

#[cfg(test)]
mod escape_like_tests {
    use super::escape_like;

    // 轮66 回归：LIKE/ILIKE 通配符注入转义。意图：用户搜索词中的 % _ \
    // 必须变成字面量，不得改变模式匹配语义（如搜索 "_" 不得命中全表）。
    #[test]
    fn escapes_wildcards_and_backslash_in_order() {
        let cases = [
            ("plain", "plain"),
            ("50%", "50\\%"),
            ("under_score", "under\\_score"),
            ("back\\slash", "back\\\\slash"),
            ("%_\\", "\\%\\_\\\\"),
            ("", ""),
            ("百分号%", "百分号\\%"),
        ];
        for (input, expected) in cases {
            assert_eq!(escape_like(input), expected, "input={input:?}");
        }
    }

    #[test]
    fn double_escape_differs_from_single() {
        // 转义结果二次转义必须不同于一次（防双重转义接线回归，轮66 踩坑）
        let once = escape_like("a%b_c\\d");
        let twice = escape_like(&once);
        assert_ne!(once, twice);
    }
}

#[cfg(test)]
mod pool_config_tests {
    use super::*;

    #[test]
    fn defaults_are_20_max_zero_idle_no_tls() {
        // 默认口径：本地开发 NoTls、不预热连接（无库环境启动不报错）。
        assert_eq!(
            DbPoolConfig::resolve(None, None, None),
            DbPoolConfig {
                max_size: 20,
                min_idle: 0,
                tls: false
            }
        );
    }

    #[test]
    fn env_overrides_parse_clamp_and_fallback() {
        let c = DbPoolConfig::resolve(Some(" 50 "), Some("4"), Some("1"));
        assert_eq!(c.max_size, 50);
        assert_eq!(c.min_idle, 4);
        assert!(c.tls);
        // 非法 / 空值一律回退默认，不 panic
        assert_eq!(
            DbPoolConfig::resolve(Some("abc"), Some(""), Some("maybe")),
            DbPoolConfig::default()
        );
        // max_size 至少为 1（0 会让 deadpool 永远拿不到连接）
        assert_eq!(DbPoolConfig::resolve(Some("0"), None, None).max_size, 1);
    }

    #[test]
    fn truthy_flag_values() {
        for v in ["1", "true", "TRUE", " on ", "yes", "Yes"] {
            assert!(is_truthy(v), "{v:?} should be truthy");
        }
        for v in ["0", "false", "off", "no", "", "enable"] {
            assert!(!is_truthy(v), "{v:?} should be falsy");
        }
    }

    #[test]
    fn sslmode_appended_to_url() {
        assert_eq!(
            with_sslmode("postgres://h:5432/db", "require"),
            "postgres://h:5432/db?sslmode=require"
        );
        assert_eq!(
            with_sslmode("postgres://h:5432/db?application_name=x", "require"),
            "postgres://h:5432/db?application_name=x&sslmode=require"
        );
    }
}

#[cfg(test)]
mod pool_construction_tests {
    use super::*;
    use crate::testing::ENV_LOCK;

    /// 保存并在 drop 时恢复环境变量，避免测试之间互相污染。
    struct EnvRestore(Vec<(&'static str, Option<String>)>);

    impl EnvRestore {
        fn apply(vars: &[(&'static str, Option<&str>)]) -> Self {
            let saved = vars
                .iter()
                .map(|(key, _)| (*key, env::var(key).ok()))
                .collect();
            for (key, value) in vars {
                match value {
                    Some(value) => env::set_var(key, value),
                    None => env::remove_var(key),
                }
            }
            Self(saved)
        }
    }

    impl Drop for EnvRestore {
        fn drop(&mut self) {
            for (key, value) in &self.0 {
                match value {
                    Some(value) => env::set_var(key, value),
                    None => env::remove_var(key),
                }
            }
        }
    }

    #[tokio::test]
    #[allow(clippy::await_holding_lock)] // ENV_LOCK 必须覆盖整个 await，串行化全局环境变量
    async fn create_pool_builds_lazily_and_honours_max_size() {
        let _guard = ENV_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        // 不再设置 DATABASE_URL：URL 通过参数传入（`create_pool_with_url`），
        // 避免清除/覆盖进程级环境变量污染并行跑的其他测试（crud 真库臂等）。
        let _env = EnvRestore::apply(&[("DB_TLS", None), ("DB_POOL_MAX_SIZE", Some("3"))]);

        // deadpool 是懒连接：构建 pool 不建连，因此无库环境也能验证 URL 解析与 sizing。
        let pool = create_pool_with_url("postgres://user:pass@localhost:5432/db".to_string())
            .await
            .expect("pool construction");
        assert_eq!(pool.status().max_size, 3);
    }

    #[tokio::test]
    #[allow(clippy::await_holding_lock)]
    async fn create_pool_tls_branch_builds_rustls_manager() {
        let _guard = ENV_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        let _env = EnvRestore::apply(&[("DB_TLS", Some("1")), ("DB_POOL_MAX_SIZE", Some("2"))]);

        // TLS 分支会构造 rustls connector（ring provider + webpki 根证书）。
        // 同样不实际建连；只要 connector 构造失败就会在这里红。
        let pool = create_pool_with_url("postgres://user:pass@localhost:5432/db".to_string())
            .await
            .expect("TLS pool construction");
        assert_eq!(pool.status().max_size, 2);
    }

    #[tokio::test]
    #[allow(clippy::await_holding_lock)]
    async fn create_pool_falls_back_to_default_database_url() {
        let _guard = ENV_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        let _env = EnvRestore::apply(&[("DB_TLS", None), ("DB_POOL_MAX_SIZE", None)]);

        // 回退分支用纯函数验证（不碰进程级 DATABASE_URL，避免污染并行测试）：
        // 环境变量缺失时 `resolve_database_url` 必须返回内置默认值。
        assert_eq!(
            resolve_database_url(Err(std::env::VarError::NotPresent)),
            DEFAULT_DATABASE_URL
        );
        assert_eq!(
            resolve_database_url(Ok("postgres://user:pass@localhost:5432/db".to_string())),
            "postgres://user:pass@localhost:5432/db"
        );

        // 默认 URL 本身必须能构建 pool（懒连接，不实际建连）。
        let pool = create_pool_with_url(DEFAULT_DATABASE_URL.to_string())
            .await
            .expect("default URL must parse");
        assert_eq!(pool.status().max_size, 20);
    }

    #[test]
    fn pool_config_reads_env_end_to_end() {
        let _guard = ENV_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        let _env = EnvRestore::apply(&[
            ("DB_POOL_MAX_SIZE", Some("7")),
            ("DB_POOL_MIN_IDLE", Some("2")),
            ("DB_TLS", Some("yes")),
        ]);

        assert_eq!(
            pool_config(),
            DbPoolConfig {
                max_size: 7,
                min_idle: 2,
                tls: true,
            }
        );
    }

    #[tokio::test]
    #[allow(clippy::await_holding_lock)]
    async fn create_sea_orm_pool_maps_invalid_url_to_pool_error() {
        let _guard = ENV_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        let _env = EnvRestore::apply(&[
            ("DB_TLS", None),
            ("DB_POOL_MAX_SIZE", Some("2")),
            ("DB_POOL_MIN_IDLE", Some("1")),
        ]);

        // 非法 URL 在 sqlx 解析阶段就会失败；这条路径不依赖任何真实数据库。
        // URL 通过参数传入，不修改进程级 DATABASE_URL（避免污染并行测试）。
        let err = create_sea_orm_pool_with_url("not-a-postgres-url".to_string())
            .await
            .expect_err("invalid URL must fail");
        assert!(matches!(err, DbError::PoolError(_)), "got {err:?}");
    }

    #[tokio::test]
    #[allow(clippy::await_holding_lock)]
    async fn create_sea_orm_pool_tls_branch_appends_sslmode() {
        let _guard = ENV_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        let _env = EnvRestore::apply(&[
            ("DB_TLS", Some("on")),
            ("DB_POOL_MAX_SIZE", Some("2")),
            ("DB_POOL_MIN_IDLE", Some("0")),
        ]);

        // DB_TLS 开时必须先拼 `sslmode=require` 再交给 sqlx；非法 URL 仍会失败，
        // 但失败发生在拼接之后，能覆盖 TLS 分支而不需要真实 TLS 服务器。
        let err = create_sea_orm_pool_with_url("not-a-postgres-url".to_string())
            .await
            .expect_err("invalid URL must fail even with TLS");
        assert!(matches!(err, DbError::PoolError(_)), "got {err:?}");
    }

    #[test]
    #[should_panic(expected = "invalid DATABASE_URL")]
    fn create_pool_with_url_without_host_panics() {
        // URL 缺少 host（`@` 后直接 `/db`）时必须在构建阶段 panic（fail-fast），
        // 而不是带一个空 host 的配置进入运行期（连接行为不可预期）。
        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .expect("build rt");
        rt.block_on(async {
            let _ = create_pool_with_url("postgres://user:pass@/db".to_string())
                .await
                .expect("must panic");
        });
    }
}
