// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::db::rewriter::rewrite_pg_to_mysql;
use std::sync::OnceLock;

// ──────────────────────────────────────────────────────────────────────────────
// SqlDialect — 最小 SQL 方言抽象层
//
// 只抽象当前查询实际用到的差异点：
//   - 参数占位符：$1/$2（PG） vs ?（MySQL）
//   - 类型转换：::text / ::bigint（PG） vs CAST(...AS CHAR/BIGINT)（MySQL）
//   - JSON 列类型：jsonb（PG） vs json（MySQL）
//   - 标识符引号："name"（PG） vs `name`（MySQL）
//   - 时间函数：NOW()（两者相同，保留方法以備后续扩展）
//   - 模糊匹配：ILIKE（PG） vs LIKE（MySQL 默认不区分大小写）
// ──────────────────────────────────────────────────────────────────────────────

pub trait SqlDialect: Send + Sync {
    fn name(&self) -> &'static str;

    fn quote_ident(&self, name: &str) -> String;

    fn param(&self, n: usize) -> String;

    fn now(&self) -> &'static str;

    fn json_type(&self) -> &'static str;

    fn cast_text(&self, expr: &str) -> String;

    fn cast_bigint(&self, expr: &str) -> String;

    fn ilike_op(&self) -> &'static str;

    fn format_sql(&self, sql: &str) -> String;

    fn cast_text_param(&self, n: usize) -> String {
        self.cast_text(&self.param(n))
    }

    fn cast_bigint_param(&self, n: usize) -> String {
        self.cast_bigint(&self.param(n))
    }
}

// ──────────────────────────────────────────────────────────────────────────────
// PostgresDialect
// ──────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy)]
pub struct PostgresDialect;

impl Default for PostgresDialect {
    fn default() -> Self {
        Self::new()
    }
}

impl PostgresDialect {
    pub fn new() -> Self {
        Self
    }
}

impl SqlDialect for PostgresDialect {
    fn name(&self) -> &'static str {
        "postgres"
    }

    fn quote_ident(&self, name: &str) -> String {
        format!("\"{}\"", name)
    }

    fn param(&self, n: usize) -> String {
        format!("${}", n)
    }

    fn now(&self) -> &'static str {
        "NOW()"
    }

    fn json_type(&self) -> &'static str {
        "jsonb"
    }

    fn cast_text(&self, expr: &str) -> String {
        format!("{}::text", expr)
    }

    fn cast_bigint(&self, expr: &str) -> String {
        format!("{}::bigint", expr)
    }

    fn ilike_op(&self) -> &'static str {
        "ILIKE"
    }

    fn format_sql(&self, sql: &str) -> String {
        sql.to_string()
    }
}

// ──────────────────────────────────────────────────────────────────────────────
// MySQLDialect
// ──────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy)]
pub struct MySQLDialect;

impl Default for MySQLDialect {
    fn default() -> Self {
        Self::new()
    }
}

impl MySQLDialect {
    pub fn new() -> Self {
        Self
    }
}

impl SqlDialect for MySQLDialect {
    fn name(&self) -> &'static str {
        "mysql"
    }

    fn quote_ident(&self, name: &str) -> String {
        format!("`{}`", name)
    }

    fn param(&self, _n: usize) -> String {
        "?".to_string()
    }

    fn now(&self) -> &'static str {
        "NOW()"
    }

    fn json_type(&self) -> &'static str {
        "json"
    }

    fn cast_text(&self, expr: &str) -> String {
        format!("CAST({} AS CHAR)", expr)
    }

    fn cast_bigint(&self, expr: &str) -> String {
        format!("CAST({} AS BIGINT)", expr)
    }

    fn ilike_op(&self) -> &'static str {
        "LIKE"
    }

    fn format_sql(&self, sql: &str) -> String {
        rewrite_pg_to_mysql(sql)
    }
}

// ──────────────────────────────────────────────────────────────────────────────
// 全局方言实例（从 DATABASE_DIALECT 环境变量读取，默认 postgres）
// ──────────────────────────────────────────────────────────────────────────────

// ──────────────────────────────────────────────────────────────────────────────
// DamengDialect — 达梦数据库（兼容 PostgreSQL 协议）
// ──────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy)]
pub struct DamengDialect;

impl SqlDialect for DamengDialect {
    fn name(&self) -> &'static str {
        "dameng"
    }
    fn quote_ident(&self, name: &str) -> String {
        format!("\"{}\"", name)
    }
    fn param(&self, n: usize) -> String {
        format!("${}", n)
    }
    fn now(&self) -> &'static str {
        "NOW()"
    }
    fn json_type(&self) -> &'static str {
        "jsonb"
    }
    fn cast_text(&self, expr: &str) -> String {
        format!("{}::text", expr)
    }
    fn cast_bigint(&self, expr: &str) -> String {
        format!("{}::bigint", expr)
    }
    fn ilike_op(&self) -> &'static str {
        "ILIKE"
    }
    fn format_sql(&self, sql: &str) -> String {
        sql.to_string()
    }
}

// ──────────────────────────────────────────────────────────────────────────────
// KingbaseDialect — 人大金仓（兼容 MySQL 协议）
// ──────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy)]
pub struct KingbaseDialect;

impl SqlDialect for KingbaseDialect {
    fn name(&self) -> &'static str {
        "kingbase"
    }
    fn quote_ident(&self, name: &str) -> String {
        format!("`{}`", name)
    }
    fn param(&self, _n: usize) -> String {
        "?".to_string()
    }
    fn now(&self) -> &'static str {
        "NOW()"
    }
    fn json_type(&self) -> &'static str {
        "json"
    }
    fn cast_text(&self, expr: &str) -> String {
        format!("CAST({} AS VARCHAR)", expr)
    }
    fn cast_bigint(&self, expr: &str) -> String {
        format!("CAST({} AS BIGINT)", expr)
    }
    fn ilike_op(&self) -> &'static str {
        "LIKE"
    }
    fn format_sql(&self, sql: &str) -> String {
        rewrite_pg_to_mysql(sql)
    }
}

static DIALECT: OnceLock<Box<dyn SqlDialect>> = OnceLock::new();

/// 按方言名构造实例：**纯函数**，不读环境、不碰全局。
///
/// 为什么要单独抽出来：`dialect()` 背后的 `OnceLock` 全进程只能初始化一次，
/// 而「哪个测试先跑到」决定了它初始化成什么。此前 `dialect.rs` 的用例靠
/// `set_var("DB_DIALECT","mysql")` + `dialect()` 来断言优先级 —— 于是：
///   - 若它先跑，全局被钉成 mysql，任何后续需要 postgres 的测试（例如迁移运行器）
///     都会拿 MySQL 语法去打 PG，报 `syntax error at end of input`；
///   - 若它后跑，`OnceLock` 已是 postgres，它自己的断言反而红。
/// 抽成纯函数后，优先级由 `dialect_name_from_env()` 表达，映射由本函数表达，
/// 两者都能脱离全局状态被测。
pub fn dialect_from(name: &str) -> Box<dyn SqlDialect> {
    match name.to_lowercase().as_str() {
        "mysql" => Box::new(MySQLDialect::new()),
        "dameng" => Box::new(DamengDialect),
        "kingbase" => Box::new(KingbaseDialect),
        _ => Box::new(PostgresDialect::new()),
    }
}

/// 从环境变量解析方言名：`DB_DIALECT` 优先于 `DATABASE_DIALECT`，缺省 `postgres`。
pub fn dialect_name_from_env() -> String {
    std::env::var("DB_DIALECT")
        .or_else(|_| std::env::var("DATABASE_DIALECT"))
        .unwrap_or_else(|_| "postgres".to_string())
}

pub fn dialect() -> &'static dyn SqlDialect {
    DIALECT
        .get_or_init(|| dialect_from(&dialect_name_from_env()))
        .as_ref()
}

// ──────────────────────────────────────────────────────────────────────────────
// 单元测试：验证 format_sql 在两个方言上的行为
// ──────────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn postgres_format_sql_is_identity() {
        let sql = "SELECT * FROM t WHERE id = $1 AND name = $2";
        assert_eq!(PostgresDialect::new().format_sql(sql), sql);
    }

    #[test]
    fn mysql_format_sql_replaces_params() {
        let sql = "SELECT * FROM t WHERE id = $1 AND name = $2";
        assert_eq!(
            MySQLDialect::new().format_sql(sql),
            "SELECT * FROM t WHERE id = ? AND name = ?"
        );
    }

    #[test]
    fn mysql_format_sql_handles_multi_digit() {
        let sql = "SELECT * FROM t WHERE a = $10 AND b = $2";
        assert_eq!(
            MySQLDialect::new().format_sql(sql),
            "SELECT * FROM t WHERE a = ? AND b = ?"
        );
    }

    #[test]
    fn mysql_format_sql_preserves_dollar_in_string() {
        let sql = "INSERT INTO t (name) VALUES ('price $100') WHERE id = $1";
        assert_eq!(
            MySQLDialect::new().format_sql(sql),
            "INSERT INTO t (name) VALUES ('price $100') WHERE id = ?"
        );
    }

    #[test]
    fn postgres_param_and_cast() {
        let d = PostgresDialect::new();
        assert_eq!(d.param(1), "$1");
        assert_eq!(
            d.cast_text("change_password_time"),
            "change_password_time::text"
        );
        assert_eq!(d.cast_bigint("$1"), "$1::bigint");
    }

    #[test]
    fn mysql_param_and_cast() {
        let d = MySQLDialect::new();
        assert_eq!(d.param(1), "?");
        assert_eq!(
            d.cast_text("change_password_time"),
            "CAST(change_password_time AS CHAR)"
        );
        assert_eq!(d.cast_bigint("$1"), "CAST($1 AS BIGINT)");
    }

    #[test]
    fn postgres_quote_ident() {
        let d = PostgresDialect::new();
        assert_eq!(d.quote_ident("clueId"), "\"clueId\"");
    }

    #[test]
    fn mysql_quote_ident() {
        let d = MySQLDialect::new();
        assert_eq!(d.quote_ident("clueId"), "`clueId`");
    }

    #[test]
    fn postgres_json_type() {
        assert_eq!(PostgresDialect::new().json_type(), "jsonb");
    }

    #[test]
    fn mysql_json_type() {
        assert_eq!(MySQLDialect::new().json_type(), "json");
    }

    #[test]
    fn postgres_ilike() {
        assert_eq!(PostgresDialect::new().ilike_op(), "ILIKE");
    }

    #[test]
    fn mysql_ilike() {
        assert_eq!(MySQLDialect::new().ilike_op(), "LIKE");
    }

    #[test]
    fn postgres_dialect_name() {
        assert_eq!(PostgresDialect::new().name(), "postgres");
    }

    #[test]
    fn mysql_dialect_name() {
        assert_eq!(MySQLDialect::new().name(), "mysql");
    }

    #[test]
    fn db_dialect_takes_priority_over_databases_dialect() {
        // ⚠️ 这条**不能**再通过 `dialect()` 断言：那会初始化全局 OnceLock，
        // 把方言钉死成 mysql，进而污染所有需要 postgres 的测试（迁移运行器实测
        // 报 `syntax error at end of input`）。改为断言解析层，优先级逻辑不变。
        //
        // 仍要改环境变量，因此与其它改环境的测试共用一把锁（见 testing::ENV_LOCK）。
        let _guard = crate::testing::ENV_LOCK
            .lock()
            .unwrap_or_else(|e| e.into_inner());

        let saved_db = std::env::var("DB_DIALECT").ok();
        let saved_database = std::env::var("DATABASE_DIALECT").ok();

        std::env::set_var("DB_DIALECT", "mysql");
        std::env::set_var("DATABASE_DIALECT", "postgres");
        assert_eq!(dialect_name_from_env(), "mysql", "DB_DIALECT 必须优先");
        assert_eq!(dialect_from(&dialect_name_from_env()).name(), "mysql");

        std::env::remove_var("DB_DIALECT");
        assert_eq!(
            dialect_name_from_env(),
            "postgres",
            "缺 DB_DIALECT 时回落到 DATABASE_DIALECT"
        );

        std::env::remove_var("DATABASE_DIALECT");
        assert_eq!(dialect_name_from_env(), "postgres", "两者都缺省时回落到 postgres");

        // 大小写不敏感（配置里写成 MySQL 不该被当成未知值）
        assert_eq!(dialect_from("MySQL").name(), "mysql");
        assert_eq!(dialect_from("UNKNOWN-DIALECT").name(), "postgres");

        match saved_db {
            Some(v) => std::env::set_var("DB_DIALECT", v),
            None => std::env::remove_var("DB_DIALECT"),
        }
        match saved_database {
            Some(v) => std::env::set_var("DATABASE_DIALECT", v),
            None => std::env::remove_var("DATABASE_DIALECT"),
        }
    }

    #[test]
    fn dameng_dialect_name() {
        assert_eq!(DamengDialect.name(), "dameng");
    }

    #[test]
    fn kingbase_dialect_name() {
        assert_eq!(KingbaseDialect.name(), "kingbase");
    }

    #[test]
    fn dameng_quote_ident_uses_double_quotes() {
        let d = DamengDialect;
        assert_eq!(d.quote_ident("foo"), "\"foo\"");
    }

    #[test]
    fn kingbase_quote_ident_uses_backticks() {
        let d = KingbaseDialect;
        assert_eq!(d.quote_ident("foo"), "`foo`");
    }

    #[test]
    fn dameng_param_same_as_postgres() {
        let d = DamengDialect;
        assert_eq!(d.param(1), "$1");
        assert_eq!(d.param(2), "$2");
    }

    #[test]
    fn kingbase_param_same_as_mysql() {
        let d = KingbaseDialect;
        assert_eq!(d.param(1), "?");
    }

    #[test]
    fn dameng_json_type_jsonb() {
        assert_eq!(DamengDialect.json_type(), "jsonb");
    }

    #[test]
    fn kingbase_json_type_json() {
        assert_eq!(KingbaseDialect.json_type(), "json");
    }

    #[test]
    fn dameng_ilike() {
        assert_eq!(DamengDialect.ilike_op(), "ILIKE");
    }

    #[test]
    fn kingbase_ilike() {
        assert_eq!(KingbaseDialect.ilike_op(), "LIKE");
    }

    #[test]
    fn dameng_format_sql_is_identity() {
        let sql = "SELECT * FROM t WHERE id = $1";
        assert_eq!(DamengDialect.format_sql(sql), sql);
    }

    #[test]
    fn kingbase_format_sql_uses_mysql_style() {
        let sql = "SELECT * FROM t WHERE id = $1";
        assert_eq!(
            KingbaseDialect.format_sql(sql),
            "SELECT * FROM t WHERE id = ?"
        );
    }
}
