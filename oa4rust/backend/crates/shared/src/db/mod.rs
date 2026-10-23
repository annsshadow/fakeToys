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

fn database_url() -> String {
    dotenv().ok();
    env::var("DATABASE_URL").unwrap_or_else(|_| DEFAULT_DATABASE_URL.to_string())
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

pub async fn create_pool() -> Result<Pool, DbError> {
    let database_url = database_url();
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

/// 创建 SeaORM DatabaseConnection（sizing/TLS 与 deadpool 池同源，见 DbPoolConfig）。
pub async fn create_sea_orm_pool() -> Result<DatabaseConnection, DbError> {
    let pool_cfg = pool_config();
    let database_url = if pool_cfg.tls {
        with_sslmode(&database_url(), "require")
    } else {
        database_url()
    };

    let mut options = ConnectOptions::new(database_url);
    options
        .max_connections(pool_cfg.max_size.min(u32::MAX as usize) as u32)
        .min_connections(pool_cfg.min_idle.min(u32::MAX as usize) as u32)
        .sqlx_logging(false);

    Database::connect(options)
        .await
        .map_err(|e| DbError::PoolError(e.to_string()))
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
