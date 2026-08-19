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
        replace_pg_params(sql)
    }
}

// ──────────────────────────────────────────────────────────────────────────────
// 全局方言实例（从 DATABASE_DIALECT 环境变量读取，默认 postgres）
// ──────────────────────────────────────────────────────────────────────────────

static DIALECT: OnceLock<Box<dyn SqlDialect>> = OnceLock::new();

pub fn dialect() -> &'static dyn SqlDialect {
    DIALECT.get_or_init(|| {
        let raw = std::env::var("DATABASE_DIALECT")
            .unwrap_or_else(|_| "postgres".to_string())
            .to_lowercase();
        match raw.as_str() {
            "mysql" => Box::new(MySQLDialect::new()),
            _ => Box::new(PostgresDialect::new()),
        }
    }).as_ref()
}

// ──────────────────────────────────────────────────────────────────────────────
// 内部辅助：将 PostgreSQL 的 $N 占位符替换为 MySQL 的 ?
// 纯手工扫描，无需引入 regex 依赖。
// ──────────────────────────────────────────────────────────────────────────────

fn replace_pg_params(sql: &str) -> String {
    let mut out = String::with_capacity(sql.len());
    let mut chars = sql.chars().peekable();
    let mut in_string = false;
    while let Some(c) = chars.next() {
        if c == '\'' {
            in_string = !in_string;
            out.push(c);
        } else if !in_string && c == '$' {
            if let Some(&d) = chars.peek() {
                if d.is_ascii_digit() {
                    out.push('?');
                    chars.next();
                    while let Some(&d2) = chars.peek() {
                        if d2.is_ascii_digit() {
                            chars.next();
                        } else {
                            break;
                        }
                    }
                    continue;
                }
            }
            out.push('$');
        } else {
            out.push(c);
        }
    }
    out
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
    fn dialect_env_switch() {
        std::env::set_var("DATABASE_DIALECT", "mysql");
        let d = dialect();
        assert_eq!(d.name(), "mysql");
        std::env::remove_var("DATABASE_DIALECT");
    }
}
