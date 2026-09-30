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

pub async fn create_pool() -> Result<Pool, DbError> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://o2server:password@localhost:5432/oa4rust".to_string());

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

    let mgr = Manager::new(cfg, NoTls);
    let pool = Pool::builder(mgr)
        .build()
        .map_err(|e| DbError::PoolError(e.to_string()))?;

    Ok(pool)
}

/// 创建 SeaORM DatabaseConnection（与 create_pool 并行）
pub async fn create_sea_orm_pool() -> Result<DatabaseConnection, DbError> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://o2server:password@localhost:5432/oa4rust".to_string());

    let mut options = ConnectOptions::new(database_url);
    options.max_connections(20).sqlx_logging(false);

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
