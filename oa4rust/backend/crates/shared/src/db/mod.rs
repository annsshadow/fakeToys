// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

// ──────────────────────────────────────────────────────────────────────────────
// db — 数据库连接池 + SQL 方言抽象
// ──────────────────────────────────────────────────────────────────────────────

pub mod dialect;
pub mod rewriter;

use deadpool_postgres::tokio_postgres::{Config, NoTls};
use deadpool_postgres::{Manager, Pool, Runtime, Timeouts};
use dotenvy::dotenv;
use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use std::env;
use std::time::Duration;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DbError {
    #[error("connection pool error: {0}")]
    PoolError(String),
    /// DATABASE_URL 形状不合法（scheme / host / user / dbname）。
    /// 启动期 fail-loud：修前这里 panic，配置错时只有一条 backtrace，
    /// 运维看不出是哪个字段错了。
    #[error("invalid database configuration: {0}")]
    Config(String),
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
    /// 池满时等待连接的上界（`None` = 无限等待，不推荐）。
    pub wait_timeout: Option<Duration>,
    /// 新建连接（TCP + 认证）的上界（`None` = 无限等待）。
    pub create_timeout: Option<Duration>,
    /// 单条 SQL 在服务端执行的上界（`None` = 不限制）。
    ///
    /// 与 `laiyipao` 的 `statement_timeout` 同义：请求超时层只能让**客户端**
    /// 不再等待，PG 仍会在后端把慢查询跑完并占着连接。statement_timeout
    /// 让服务端自己取消，连接立刻回到池里。
    pub statement_timeout: Option<Duration>,
}

/// 池满等待默认 10s：小于全局请求超时（30s），保证排队失败先于请求超时暴露，
/// 客户端拿到的是「池忙」而不是一个笼统的 408。
const DEFAULT_WAIT_TIMEOUT: Duration = Duration::from_secs(10);
/// 新建连接默认 10s：覆盖跨机房握手，又能让「网络黑洞」在下一次请求前失败。
const DEFAULT_CREATE_TIMEOUT: Duration = Duration::from_secs(10);
/// 单条 SQL 默认 30s：与全局请求超时同一量级，但由 PG 侧执行，
/// 保证请求超时后连接不会继续被慢查询占住。
const DEFAULT_STATEMENT_TIMEOUT: Duration = Duration::from_secs(30);

impl Default for DbPoolConfig {
    fn default() -> Self {
        Self {
            max_size: 20,
            min_idle: 0,
            tls: false,
            wait_timeout: Some(DEFAULT_WAIT_TIMEOUT),
            create_timeout: Some(DEFAULT_CREATE_TIMEOUT),
            statement_timeout: Some(DEFAULT_STATEMENT_TIMEOUT),
        }
    }
}

/// 解析「秒数」配置：未设置取默认，`0` 显式关闭，非法值回退默认
/// （与既有 `DB_POOL_MAX_SIZE` 口径一致）。
fn resolve_seconds(raw: Option<&str>, default: Option<Duration>) -> Option<Duration> {
    match raw {
        None => default,
        Some(v) => match v.trim().parse::<u64>() {
            Ok(0) => None,
            Ok(n) => Some(Duration::from_secs(n)),
            Err(_) => default,
        },
    }
}

impl DbPoolConfig {
    /// 纯解析逻辑（与 env 读取分离，便于无副作用单测）。
    fn resolve(
        max_size: Option<&str>,
        min_idle: Option<&str>,
        tls: Option<&str>,
        wait_timeout: Option<&str>,
        create_timeout: Option<&str>,
        statement_timeout: Option<&str>,
    ) -> Self {
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
        cfg.wait_timeout = resolve_seconds(wait_timeout, cfg.wait_timeout);
        cfg.create_timeout = resolve_seconds(create_timeout, cfg.create_timeout);
        cfg.statement_timeout = resolve_seconds(statement_timeout, cfg.statement_timeout);
        cfg
    }

    /// 从环境变量读取池配置。
    pub fn from_env() -> Self {
        Self::resolve(
            env::var("DB_POOL_MAX_SIZE").ok().as_deref(),
            env::var("DB_POOL_MIN_IDLE").ok().as_deref(),
            env::var("DB_TLS").ok().as_deref(),
            env::var("DB_POOL_WAIT_TIMEOUT_SECONDS").ok().as_deref(),
            env::var("DB_POOL_CREATE_TIMEOUT_SECONDS").ok().as_deref(),
            env::var("DB_STATEMENT_TIMEOUT_SECONDS").ok().as_deref(),
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

/// 给连接 URL 追加 sqlx 的 `options[statement_timeout]=<ms>`；
/// sqlx 会把它翻译成 PG 启动参数 `-c statement_timeout=<ms>`。
/// URL 解析失败时原样返回，由后续连接流程统一报错。
fn with_statement_timeout(url: &str, ms: u128) -> String {
    let Ok(mut parsed) = url::Url::parse(url) else {
        return url.to_string();
    };
    parsed
        .query_pairs_mut()
        .append_pair("options[statement_timeout]", &ms.to_string());
    parsed.to_string()
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

/// 解析 DATABASE_URL 的各个字段。纯函数：非法配置在这里变成可读的
/// [`DbError::Config`]，调用方（启动入口）以 `?` 上抛，不再 panic。
///
/// 校验项与 tokio-postgres 的实际要求对齐：
///   - scheme 只接受 `postgres` / `postgresql`；
///   - host、user、数据库名都必填（缺任何一个，连接阶段都会给出难以定位的错误）。
fn parse_pool_url(database_url: &str) -> Result<(String, u16, String, String, String), DbError> {
    let url = url::Url::parse(database_url)
        .map_err(|e| DbError::Config(format!("DATABASE_URL 不是合法 URL: {e}")))?;
    if url.scheme() != "postgres" && url.scheme() != "postgresql" {
        return Err(DbError::Config(format!(
            "DATABASE_URL 的 scheme 必须是 postgres 或 postgresql，实际 {:?}",
            url.scheme()
        )));
    }
    let host = url
        .host_str()
        .filter(|h| !h.is_empty())
        .ok_or_else(|| DbError::Config("DATABASE_URL 缺少 host".to_string()))?;
    let user = url.username();
    if user.is_empty() {
        return Err(DbError::Config("DATABASE_URL 缺少用户".to_string()));
    }
    let dbname = url.path().trim_start_matches('/');
    if dbname.is_empty() {
        return Err(DbError::Config(
            "DATABASE_URL 缺少数据库名（路径部分）".to_string(),
        ));
    }
    Ok((
        host.to_string(),
        url.port().unwrap_or(5432),
        user.to_string(),
        url.password().unwrap_or("").to_string(),
        dbname.to_string(),
    ))
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

    // R25：非法 DATABASE_URL 由 panic 改为结构化错误（启动入口以 anyhow
    // 上下文包裹后直接退出，错误信息能定位到具体字段）。
    let (host, port, user, password, dbname) = parse_pool_url(&database_url)?;

    let mut cfg = Config::new();
    cfg.host(host)
        .port(port)
        .user(user)
        .password(password)
        .dbname(dbname);

    // R26：单条 SQL 的服务端上界。走 startup options（libpq 同义），
    // 每个新建连接都会带上；`0` 表示不限制（与 resolve_seconds 口径一致）。
    if let Some(timeout) = pool_cfg.statement_timeout {
        cfg.options(format!("-c statement_timeout={}", timeout.as_millis()));
    }

    let mgr = if pool_cfg.tls {
        Manager::new(cfg, rustls_connector())
    } else {
        Manager::new(cfg, NoTls)
    };
    let pool = Pool::builder(mgr)
        .max_size(pool_cfg.max_size)
        // deadpool 配了 timeouts 就必须显式指定 runtime（否则 build 报
        // "Timeouts require a runtime"）；本服务只跑 Tokio。
        .runtime(Runtime::Tokio1)
        // R6：池满等待与建连都有上界。修前 `get()` 在池满时无限排队、
        // 建连遇到网络黑洞会一直挂到 TCP 自身超时（可能数分钟）。
        .timeouts(Timeouts {
            wait: pool_cfg.wait_timeout,
            create: pool_cfg.create_timeout,
            recycle: None,
        })
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
    let mut database_url = if pool_cfg.tls {
        with_sslmode(&database_url, "require")
    } else {
        database_url
    };
    // R26：SeaORM 连接同样带服务端 statement_timeout（与 deadpool 池同源）。
    if let Some(timeout) = pool_cfg.statement_timeout {
        database_url = with_statement_timeout(&database_url, timeout.as_millis());
    }

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
        // 默认口径：本地开发 NoTls、不预热连接（无库环境启动不报错）；
        // 池满等待与建连默认各有 10s 上界。
        assert_eq!(
            DbPoolConfig::resolve(None, None, None, None, None, None),
            DbPoolConfig {
                max_size: 20,
                min_idle: 0,
                tls: false,
                wait_timeout: Some(Duration::from_secs(10)),
                create_timeout: Some(Duration::from_secs(10)),
                statement_timeout: Some(Duration::from_secs(30)),
            }
        );
    }

    #[test]
    fn env_overrides_parse_clamp_and_fallback() {
        let c = DbPoolConfig::resolve(
            Some(" 50 "),
            Some("4"),
            Some("1"),
            Some("3"),
            Some("7"),
            Some("5"),
        );
        assert_eq!(c.max_size, 50);
        assert_eq!(c.min_idle, 4);
        assert!(c.tls);
        assert_eq!(c.wait_timeout, Some(Duration::from_secs(3)));
        assert_eq!(c.create_timeout, Some(Duration::from_secs(7)));
        assert_eq!(c.statement_timeout, Some(Duration::from_secs(5)));
        // 非法 / 空值一律回退默认，不 panic
        assert_eq!(
            DbPoolConfig::resolve(
                Some("abc"),
                Some(""),
                Some("maybe"),
                Some("x"),
                Some(""),
                Some("y")
            ),
            DbPoolConfig::default()
        );
        // max_size 至少为 1（0 会让 deadpool 永远拿不到连接）
        assert_eq!(
            DbPoolConfig::resolve(Some("0"), None, None, None, None, None).max_size,
            1
        );
    }

    #[test]
    fn zero_disables_pool_timeouts() {
        // 0 = 显式关闭：排查长事务时允许无限等待，但必须是**有人做的决定**。
        let c = DbPoolConfig::resolve(None, None, None, Some("0"), Some("0"), Some("0"));
        assert_eq!(c.wait_timeout, None);
        assert_eq!(c.create_timeout, None);
        assert_eq!(c.statement_timeout, None);
        // 空白值按未设置处理 → 取默认，而不是关掉。
        let blank = DbPoolConfig::resolve(None, None, None, Some("  "), Some("  "), Some("  "));
        assert_eq!(blank.wait_timeout, DbPoolConfig::default().wait_timeout);
        assert_eq!(
            blank.statement_timeout,
            DbPoolConfig::default().statement_timeout
        );
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

    #[test]
    fn statement_timeout_appended_to_url() {
        // sqlx 从 `options[statement_timeout]=<ms>` 生成启动参数
        // `-c statement_timeout=<ms>`；已有 query 时必须追加而不是覆盖。
        let url = with_statement_timeout("postgres://h:5432/db", 30_000);
        assert_eq!(
            url,
            "postgres://h:5432/db?options%5Bstatement_timeout%5D=30000"
        );
        let url = with_statement_timeout("postgres://h:5432/db?sslmode=require", 2_500);
        assert_eq!(
            url,
            "postgres://h:5432/db?sslmode=require&options%5Bstatement_timeout%5D=2500"
        );
        // 解析不了的 URL 原样返回（后续连接阶段统一报错）。
        assert_eq!(with_statement_timeout("not a url", 100), "not a url");
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
            ("DB_POOL_WAIT_TIMEOUT_SECONDS", Some("4")),
            ("DB_POOL_CREATE_TIMEOUT_SECONDS", Some("6")),
            ("DB_STATEMENT_TIMEOUT_SECONDS", Some("5")),
        ]);

        assert_eq!(
            pool_config(),
            DbPoolConfig {
                max_size: 7,
                min_idle: 2,
                tls: true,
                wait_timeout: Some(Duration::from_secs(4)),
                create_timeout: Some(Duration::from_secs(6)),
                statement_timeout: Some(Duration::from_secs(5)),
            }
        );
    }

    /// R6：池满等待必须有上界——修前 `get()` 会一直排队，
    /// 客户端表现是请求永久挂起。
    ///
    /// 需真库（`TEST_DATABASE_URL` 或 `DATABASE_URL`）；无库时显式 skip。
    #[tokio::test]
    #[allow(clippy::await_holding_lock)]
    async fn saturated_pool_waits_only_until_wait_timeout() {
        let _guard = ENV_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        let Ok(url) = env::var("TEST_DATABASE_URL").or_else(|_| env::var("DATABASE_URL")) else {
            eprintln!("skip: 无 TEST_DATABASE_URL / DATABASE_URL，池满等待需真库");
            return;
        };
        let _env = EnvRestore::apply(&[
            ("DB_POOL_MAX_SIZE", Some("1")),
            ("DB_POOL_WAIT_TIMEOUT_SECONDS", Some("1")),
            ("DB_POOL_CREATE_TIMEOUT_SECONDS", Some("5")),
            ("DB_TLS", None),
        ]);

        let pool = create_pool_with_url(url).await.expect("pool construction");
        let held = pool.get().await.expect("第一条连接应立刻拿到");

        let started = std::time::Instant::now();
        let second = pool.get().await;
        let elapsed = started.elapsed();

        match second {
            Err(deadpool_postgres::PoolError::Timeout(_)) => {}
            Ok(_) => panic!("max_size=1 且连接已被占用，不应再拿到连接"),
            Err(e) => panic!("期望池满等待超时，实际 {e:?}"),
        }
        assert!(
            elapsed >= Duration::from_millis(900),
            "等待时间短于 wait_timeout（可能没生效）：{elapsed:?}"
        );
        assert!(
            elapsed < Duration::from_secs(5),
            "等待未被 wait_timeout 截断：{elapsed:?}"
        );
        drop(held);
    }

    /// R26：statement_timeout 由服务端执行取消慢查询，连接随后仍可用。
    ///
    /// 修前只有客户端请求超时：客户端不再等待，但 PG 后端仍把 pg_sleep(3)
    /// 跑完并占着那条连接，池容量在这种「慢查询风暴」里会被吃光。
    ///
    /// 需真库（`TEST_DATABASE_URL` 或 `DATABASE_URL`）；无库时显式 skip。
    #[tokio::test]
    #[allow(clippy::await_holding_lock)]
    async fn statement_timeout_cancels_slow_query_and_keeps_connection() {
        let _guard = ENV_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        let Ok(url) = env::var("TEST_DATABASE_URL").or_else(|_| env::var("DATABASE_URL")) else {
            eprintln!("skip: 无 TEST_DATABASE_URL / DATABASE_URL，statement_timeout 需真库");
            return;
        };
        let kill = |v: Option<&str>| EnvRestore::apply(&[("DB_TLS", None), ("DB_STATEMENT_TIMEOUT_SECONDS", v)]);

        // 默认：未配置时为 30s（connection startup options 生效）。
        {
            let _env = kill(None);
            let pool = create_pool_with_url(url.clone()).await.expect("pool construction");
            let client = pool.get().await.expect("get");
            let shown: String = client
                .query_one("SHOW statement_timeout", &[])
                .await
                .expect("SHOW")
                .get(0);
            assert_eq!(shown, "30s", "默认 statement_timeout 应为 30s");
        }

        // 配置 2s：pg_sleep(3) 必须在 2s 附近被 PG 取消（57014），
        // 且同一条连接随后仍能执行 SELECT 1（取消的是语句，不是连接）。
        let _env = kill(Some("2"));
        let pool = create_pool_with_url(url).await.expect("pool construction");
        let client = pool.get().await.expect("get");
        let shown: String = client
            .query_one("SHOW statement_timeout", &[])
            .await
            .expect("SHOW")
            .get(0);
        assert_eq!(shown, "2s");

        let started = std::time::Instant::now();
        let err = client
            .query("SELECT pg_sleep(3)", &[])
            .await
            .expect_err("超过 statement_timeout 的查询必须被取消");
        assert_eq!(
            err.code().map(|c| c.code()),
            Some("57014"),
            "期望 query_canceled，实际 {err}"
        );
        assert!(
            started.elapsed() < Duration::from_millis(2500),
            "取消发生得太晚（statement_timeout 可能没生效）：{:?}",
            started.elapsed()
        );

        let one: i32 = client
            .query_one("SELECT 1", &[])
            .await
            .expect("取消语句后连接仍应可用")
            .get(0);
        assert_eq!(one, 1);
    }

    /// R6：建连遇到「只接受 TCP 但永不回应握手」的服务端时，
    /// 必须在 create_timeout 内失败，而不是挂到 TCP 自身超时。
    #[tokio::test]
    #[allow(clippy::await_holding_lock)]
    async fn create_timeout_bounds_hung_handshake() {
        let _guard = ENV_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        let _env = EnvRestore::apply(&[
            ("DB_POOL_MAX_SIZE", Some("1")),
            ("DB_POOL_WAIT_TIMEOUT_SECONDS", Some("5")),
            ("DB_POOL_CREATE_TIMEOUT_SECONDS", Some("1")),
            ("DB_TLS", None),
        ]);

        // 本地起一个「黑洞」监听：接受连接后什么都不做，PG 客户端会卡在
        // 等待 StartupMessage 的响应上——这正是 create_timeout 要兜住的场景。
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let port = listener.local_addr().unwrap().port();
        tokio::spawn(async move {
            let mut held = Vec::new();
            while let Ok((stream, _)) = listener.accept().await {
                held.push(stream); // 保持连接打开但永不回应
            }
        });

        let pool = create_pool_with_url(format!(
            "postgres://user:pass@127.0.0.1:{port}/db"
        ))
        .await
        .expect("pool construction");

        let started = std::time::Instant::now();
        let got = pool.get().await;
        let elapsed = started.elapsed();
        assert!(
            matches!(got, Err(deadpool_postgres::PoolError::Timeout(_))),
            "期望建连超时，实际 {got:?}"
        );
        assert!(
            elapsed < Duration::from_secs(4),
            "建连未被 create_timeout 截断：{elapsed:?}"
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

    /// R25：DATABASE_URL 的每一类形状错误都必须给出可定位的 `DbError::Config`，
    /// 而不是 panic（修前两条 `expect` 会把启动失败变成 backtrace）。
    #[test]
    fn parse_pool_url_rejects_malformed_with_config_error() {
        let cases = [
            // `postgres://user:pass@/db` 在 url 解析阶段就报 empty host。
            ("postgres://user:pass@/db", "不是合法 URL"),
            ("file:///path", "scheme"),
            ("not-a-postgres-url", "不是合法 URL"),
            ("postgres://user:pass@localhost:5432", "数据库名"),
            ("postgres://localhost:5432/db", "缺少用户"),
        ];
        for (raw, needle) in cases {
            let err = parse_pool_url(raw).expect_err(raw);
            assert!(matches!(err, DbError::Config(_)), "raw={raw} err={err:?}");
            assert!(
                err.to_string().contains(needle),
                "raw={raw} 错误信息应包含 {needle:?}，实际 {err}"
            );
        }

        // postgresql:// 与 postgres:// 等价；标准 URL 解析结果逐字段正确。
        let parsed = parse_pool_url("postgresql://user:pass@localhost:5432/db").unwrap();
        assert_eq!(parsed.0, "localhost");
        assert_eq!(parsed.1, 5432);
        assert_eq!(parsed.2, "user");
        assert_eq!(parsed.3, "pass");
        assert_eq!(parsed.4, "db");
    }

    /// 真实入口（`create_pool_with_url`）遇到非法 URL 必须返回 Err，
    /// 不能再 panic —— 这条钉住「启动失败可读」的行为。
    #[tokio::test]
    async fn create_pool_with_url_invalid_returns_error_without_panic() {
        let err = create_pool_with_url("postgres://user:pass@/db".to_string())
            .await
            .expect_err("缺 host 的 URL 必须失败");
        assert!(matches!(err, DbError::Config(_)), "got {err:?}");
        assert!(err.to_string().contains("不是合法 URL"), "实际 {err}");
    }
}
