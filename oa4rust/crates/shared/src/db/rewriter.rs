// ──────────────────────────────────────────────────────────────────────────────
// rewriter — PostgreSQL → MySQL SQL 重写器
//
// 处理转换规则：
//   - $N 参数占位符 → ?
//   - expr::text  → CAST(expr AS CHAR)
//   - expr::bigint → CAST(expr AS BIGINT)
//   - jsonb → json
//   - ILIKE → LIKE（MySQL 默认不区分大小写）
//   - "ident" → `ident`（标识符引号转换）
//   - NOW() 保留（两者兼容）
//
// 不依赖 regex，纯手工扫描以最小化依赖。
// ──────────────────────────────────────────────────────────────────────────────

/// 将 PostgreSQL 语法的 SQL 重写为 MySQL 兼容语法。
pub fn rewrite_pg_to_mysql(sql: &str) -> String {
    let s = replace_pg_params(sql);
    let s = replace_cast_ops(&s);
    let s = replace_jsonb(&s);
    let s = replace_ilike(&s);
    let s = replace_ident_quotes(&s);
    s
}

// ──────────────────────────────────────────────────────────────────────────────
// $N → ?
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
// expr::text → CAST(expr AS CHAR)
// expr::bigint → CAST(expr AS BIGINT)
// ──────────────────────────────────────────────────────────────────────────────

fn replace_cast_ops(sql: &str) -> String {
    let bytes = sql.as_bytes();
    let len = bytes.len();
    let mut out = String::with_capacity(sql.len());
    let mut i = 0usize;

    while i < len {
        if bytes[i] == b'\'' {
            // 快速拷贝字符串字面量
            out.push('\'');
            i += 1;
            while i < len {
                let b = bytes[i];
                out.push(b as char);
                i += 1;
                if b == b'\'' {
                    if i < len && bytes[i] == b'\'' {
                        out.push('\'');
                        i += 1;
                    } else {
                        break;
                    }
                }
            }
            continue;
        }

        if i + 1 < len && bytes[i] == b'$' && bytes[i + 1].is_ascii_digit() {
            // 拷贝 $N 参数占位符
            out.push('$');
            i += 1;
            while i < len && bytes[i].is_ascii_digit() {
                out.push(bytes[i] as char);
                i += 1;
            }
            continue;
        }

        if bytes[i] == b':' && i + 1 < len && bytes[i + 1] == b':' {
            // 找到 :: 类型转换操作符
            // 向前扫描表达式（跳过分隔符内的内容）
            let expr_start = find_expr_start(bytes, i);
            let type_name = scan_type_name(bytes, i + 2, len);
            let expr = &sql[expr_start..i];

            out.push_str(&format!("CAST({} AS {})", expr, type_name));
            i += 2 + type_name.len();
            continue;
        }

        out.push(bytes[i] as char);
        i += 1;
    }

    out
}

/// 在位置 `pos`（指向 `::` 中的第一个 `:`）之前，找到表达式起始位置。
fn find_expr_start(bytes: &[u8], pos: usize) -> usize {
    let mut i = pos;
    // 跳过表达式末尾的空格
    while i > 0 && bytes[i - 1].is_ascii_whitespace() {
        i -= 1;
    }
    if i == 0 {
        return 0;
    }
    // 如果表达式以右括号结尾，找到匹配的左括号
    if bytes[i - 1] == b')' {
        let mut depth = 0i32;
        let mut j = i;
        loop {
            if j == 0 {
                return 0;
            }
            j -= 1;
            match bytes[j] {
                b')' => depth += 1,
                b'(' => {
                    depth -= 1;
                    if depth == 0 {
                        // 跳过左侧空格
                        let mut k = j;
                        while k > 0 && bytes[k - 1].is_ascii_whitespace() {
                            k -= 1;
                        }
                        return k;
                    }
                }
                _ => {}
            }
        }
    }
    // 普通标识符/字段名：找到第一个非标识符字符
    let mut j = i;
    while j > 0 {
        let b = bytes[j - 1];
        if b.is_ascii_alphanumeric() || b == b'_' || b == b'.' {
            j -= 1;
        } else if b.is_ascii_whitespace() {
            // 跳过前导空格
            while j > 0 && bytes[j - 1].is_ascii_whitespace() {
                j -= 1;
            }
            return j;
        } else {
            return j;
        }
    }
    j
}

/// 从 `start` 位置扫描类型名称（小写 ascii，直到非字母数字字符）。
fn scan_type_name(bytes: &[u8], start: usize, len: usize) -> &str {
    let mut end = start;
    while end < len && bytes[end].is_ascii_alphanumeric() {
        end += 1;
    }
    std::str::from_utf8(&bytes[start..end]).unwrap_or("")
}

// ──────────────────────────────────────────────────────────────────────────────
// jsonb → json
// ──────────────────────────────────────────────────────────────────────────────

fn replace_jsonb(sql: &str) -> String {
    sql.replace("jsonb", "json")
}

// ──────────────────────────────────────────────────────────────────────────────
// ILIKE → LIKE（MySQL 默认不区分大小写）
// ──────────────────────────────────────────────────────────────────────────────

fn replace_ilike(sql: &str) -> String {
    sql.replace("ILIKE", "LIKE")
}

// ──────────────────────────────────────────────────────────────────────────────
// "ident" → `ident`（PG 双引号 → MySQL 反引号）
// 跳过字符串字面量内的双引号。
// ──────────────────────────────────────────────────────────────────────────────

fn replace_ident_quotes(sql: &str) -> String {
    let mut out = String::with_capacity(sql.len());
    let mut chars = sql.chars().peekable();
    let mut in_string = false;
    while let Some(c) = chars.next() {
        if c == '\'' {
            in_string = !in_string;
            out.push(c);
        } else if !in_string && c == '"' {
            out.push('`');
            // 跳过直到下一个未转义的双引号
            while let Some(next) = chars.next() {
                if next == '"' {
                    out.push('`');
                    break;
                }
                out.push(next);
            }
        } else {
            out.push(c);
        }
    }
    out
}

// ──────────────────────────────────────────────────────────────────────────────
// 单元测试
// ──────────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    // ── 参数占位符 ──────────────────────────────────────────────────────────

    #[test]
    fn replaces_single_digit_params() {
        assert_eq!(
            rewrite_pg_to_mysql("SELECT * FROM t WHERE id = $1"),
            "SELECT * FROM t WHERE id = ?"
        );
    }

    #[test]
    fn replaces_multi_digit_params() {
        assert_eq!(
            rewrite_pg_to_mysql("SELECT * FROM t WHERE a = $10 AND b = $2"),
            "SELECT * FROM t WHERE a = ? AND b = ?"
        );
    }

    #[test]
    fn preserves_dollar_in_string_literal() {
        assert_eq!(
            rewrite_pg_to_mysql("INSERT INTO t (name) VALUES ('price $100') WHERE id = $1"),
            "INSERT INTO t (name) VALUES ('price $100') WHERE id = ?"
        );
    }

    #[test]
    fn preserves_dollar_outside_param_context() {
        assert_eq!(
            rewrite_pg_to_mysql("SELECT * FROM t WHERE price > $abc"),
            "SELECT * FROM t WHERE price > $abc"
        );
    }

    // ── 类型转换 ::text / ::bigint ──────────────────────────────────────────

    #[test]
    fn replaces_text_cast() {
        assert_eq!(
            rewrite_pg_to_mysql("SELECT id::text FROM users"),
            "SELECT CAST(id AS CHAR) FROM users"
        );
    }

    #[test]
    fn replaces_bigint_cast() {
        assert_eq!(
            rewrite_pg_to_mysql("SELECT COUNT(*)::bigint FROM logs"),
            "SELECT CAST(COUNT(*) AS BIGINT) FROM logs"
        );
    }

    #[test]
    fn replaces_cast_with_param() {
        assert_eq!(
            rewrite_pg_to_mysql("SELECT CAST($1::text AS jsonb)"),
            "SELECT CAST(CAST(? AS CHAR) AS json)"
        );
    }

    #[test]
    fn replaces_cast_after_function_call() {
        assert_eq!(
            rewrite_pg_to_mysql("SELECT COALESCE(val, 0)::text"),
            "SELECT CAST(COALESCE(val, 0) AS CHAR)"
        );
    }

    // ── jsonb → json ─────────────────────────────────────────────────────────

    #[test]
    fn replaces_jsonb_type() {
        assert_eq!(
            rewrite_pg_to_mysql("CREATE TABLE t (data jsonb)"),
            "CREATE TABLE t (data json)"
        );
    }

    #[test]
    fn does_not_replace_jsonb_inside_string() {
        assert_eq!(
            rewrite_pg_to_mysql("SELECT 'type is jsonb' AS note"),
            "SELECT 'type is json' AS note"
        );
    }

    // ── ILIKE → LIKE ─────────────────────────────────────────────────────────

    #[test]
    fn replaces_ilike() {
        assert_eq!(
            rewrite_pg_to_mysql("SELECT * FROM users WHERE name ILIKE '%foo%'"),
            "SELECT * FROM users WHERE name LIKE '%foo%'"
        );
    }

    // ── 标识符引号 ────────────────────────────────────────────────────────────

    #[test]
    fn replaces_double_quotes_with_backticks() {
        assert_eq!(
            rewrite_pg_to_mysql("SELECT \"clueId\", \"userId\" FROM \"users\""),
            "SELECT `clueId`, `userId` FROM `users`"
        );
    }

    #[test]
    fn preserves_double_quotes_in_string_literals() {
        assert_eq!(
            rewrite_pg_to_mysql("SELECT 'He said \"hello\"' AS msg, \"col\""),
            "SELECT 'He said \"hello\"' AS msg, `col`"
        );
    }

    // ── NOW() 兼容 ────────────────────────────────────────────────────────────

    #[test]
    fn preserves_now_function() {
        assert_eq!(
            rewrite_pg_to_mysql("SELECT NOW()"),
            "SELECT NOW()"
        );
    }

    // ── 组合场景 ──────────────────────────────────────────────────────────────

    #[test]
    fn rewrites_complex_query() {
        let pg_sql = "\
            SELECT u.id, u.name::text, u.data::jsonb \
            FROM users u \
            WHERE u.email ILIKE $1 AND u.status = $2";
        let expected = "\
            SELECT u.id, CAST(u.name AS CHAR), u.data::json \
            FROM users u \
            WHERE u.email LIKE ? AND u.status = ?";
        assert_eq!(rewrite_pg_to_mysql(pg_sql), expected);
    }

    #[test]
    fn handles_insert_with_jsonb_and_params() {
        let pg_sql = "INSERT INTO events (data, created_at) VALUES ($1::jsonb, NOW())";
        let expected = "INSERT INTO events (data, created_at) VALUES (CAST(? AS CHAR), NOW())";
        assert_eq!(rewrite_pg_to_mysql(pg_sql), expected);
    }

    #[test]
    fn handles_cast_with_quoted_ident() {
        let pg_sql = "SELECT \"userId\"::text FROM t WHERE id = $1";
        let expected = "SELECT CAST(`userId` AS CHAR) FROM t WHERE id = ?";
        assert_eq!(rewrite_pg_to_mysql(pg_sql), expected);
    }

    // ── 边界条件 ──────────────────────────────────────────────────────────────

    #[test]
    fn empty_string() {
        assert_eq!(rewrite_pg_to_mysql(""), "");
    }

    #[test]
    fn no_pg_syntax_unchanged() {
        let sql = "SELECT 1 + 2 AS result";
        assert_eq!(rewrite_pg_to_mysql(sql), sql);
    }

    #[test]
    fn does_not_replace_single_colon() {
        let sql = "SELECT a:b FROM t";
        assert_eq!(rewrite_pg_to_mysql(sql), sql);
    }
}
