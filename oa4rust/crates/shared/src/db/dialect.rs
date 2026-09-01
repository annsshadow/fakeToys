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
    fn name(&self) -> &'static str { "dameng" }
    fn quote_ident(&self, name: &str) -> String { format!("\"{}\"", name) }
    fn param(&self, n: usize) -> String { format!("${}", n) }
    fn now(&self) -> &'static str { "NOW()" }
    fn json_type(&self) -> &'static str { "jsonb" }
    fn cast_text(&self, expr: &str) -> String { format!("{}::text", expr) }
    fn cast_bigint(&self, expr: &str) -> String { format!("{}::bigint", expr) }
    fn ilike_op(&self) -> &'static str { "ILIKE" }
    fn format_sql(&self, sql: &str) -> String { sql.to_string() }
}

// ──────────────────────────────────────────────────────────────────────────────
// KingbaseDialect — 人大金仓（兼容 MySQL 协议）
// ──────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy)]
pub struct KingbaseDialect;

impl SqlDialect for KingbaseDialect {
    fn name(&self) -> &'static str { "kingbase" }
    fn quote_ident(&self, name: &str) -> String { format!("`{}`", name) }
    fn param(&self, _n: usize) -> String { "?".to_string() }
    fn now(&self) -> &'static str { "NOW()" }
    fn json_type(&self) -> &'static str { "json" }
    fn cast_text(&self, expr: &str) -> String { format!("CAST({} AS VARCHAR)", expr) }
    fn cast_bigint(&self, expr: &str) -> String { format!("CAST({} AS BIGINT)", expr) }
    fn ilike_op(&self) -> &'static str { "LIKE" }
    fn format_sql(&self, sql: &str) -> String { rewrite_pg_to_mysql(sql) }
}

static DIALECT: OnceLock<Box<dyn SqlDialect>> = OnceLock::new();

pub fn dialect() -> &'static dyn SqlDialect {
    DIALECT.get_or_init(|| {
        let raw = std::env::var("DB_DIALECT")
            .or_else(|_| std::env::var("DATABASE_DIALECT"))
            .unwrap_or_else(|_| "postgres".to_string())
            .to_lowercase();
        match raw.as_str() {
            "mysql" => Box::new(MySQLDialect::new()),
            "dameng" => Box::new(DamengDialect),
            "kingbase" => Box::new(KingbaseDialect),
            _ => Box::new(PostgresDialect::new()),
        }
    }).as_ref()
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
        assert_eq!(MySQLDialect::new().format_sql(sql), "SELECT * FROM t WHERE id = ? AND name = ?");
    }

    #[test]
    fn mysql_format_sql_handles_multi_digit() {
        let sql = "SELECT * FROM t WHERE a = $10 AND b = $2";
        assert_eq!(MySQLDialect::new().format_sql(sql), "SELECT * FROM t WHERE a = ? AND b = ?");
    }

    #[test]
    fn mysql_format_sql_preserves_dollar_in_string() {
        let sql = "INSERT INTO t (name) VALUES ('price $100') WHERE id = $1";
        assert_eq!(MySQLDialect::new().format_sql(sql), "INSERT INTO t (name) VALUES ('price $100') WHERE id = ?");
    }

    #[test]
    fn postgres_param_and_cast() {
        let d = PostgresDialect::new();
        assert_eq!(d.param(1), "$1");
        assert_eq!(d.cast_text("change_password_time"), "change_password_time::text");
        assert_eq!(d.cast_bigint("$1"), "$1::bigint");
    }

    #[test]
    fn mysql_param_and_cast() {
        let d = MySQLDialect::new();
        assert_eq!(d.param(1), "?");
        assert_eq!(d.cast_text("change_password_time"), "CAST(change_password_time AS CHAR)");
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
        std::env::set_var("DB_DIALECT", "mysql");
        std::env::set_var("DATABASE_DIALECT", "postgres");
        let d = dialect();
        assert_eq!(d.name(), "mysql");
        std::env::remove_var("DB_DIALECT");
        std::env::remove_var("DATABASE_DIALECT");
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
        assert_eq!(KingbaseDialect.format_sql(sql), "SELECT * FROM t WHERE id = ?");
    }

}
