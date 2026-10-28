// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

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
//   - TIMESTAMPTZ → DATETIME(6)
//   - TIMESTAMP WITH TIME ZONE → DATETIME(6)
//   - TIMESTAMP WITHOUT TIME ZONE → TIMESTAMP
//   - ON CONFLICT (cols) DO NOTHING → INSERT IGNORE INTO
//   - ON CONFLICT (cols) DO UPDATE SET ... → ON DUPLICATE KEY UPDATE
//   - DO $$ ... $$ 匿名块 → 提取块内 DDL，删除 PG 系统目录条件
//   - ALTER TABLE ADD COLUMN IF NOT EXISTS → 删除 IF NOT EXISTS
//   - gen_random_uuid() → UUID()
//   - pg_catalog.xxx → xxx
//
// 所有替换只作用于**代码段**：单引号字符串、双引号标识符、`--` 行注释与
// `/* */` 块注释原样保留。迁移脚本里的 `'type is jsonb'`、`-- ILIKE` 这类
// 文本不能被改写，否则会把数据/注释改坏。
//
// 不依赖 regex，纯手工扫描以最小化依赖。
// ──────────────────────────────────────────────────────────────────────────────

/// 将 PostgreSQL 语法的 SQL 重写为 MySQL 兼容语法。
pub fn rewrite_pg_to_mysql(sql: &str) -> String {
    let s = replace_do_blocks(sql);
    let s = replace_pg_params(&s);
    let s = replace_cast_ops(&s);
    let s = replace_jsonb(&s);
    let s = replace_ilike(&s);
    let s = replace_ident_quotes(&s);
    let s = replace_timestamptz(&s);
    let s = replace_on_conflict_do_nothing(&s);
    let s = replace_alter_if_not_exists(&s);
    let s = replace_gen_random_uuid(&s);

    replace_pg_catalog(&s)
}

// ──────────────────────────────────────────────────────────────────────────────
// 通用扫描工具
// ──────────────────────────────────────────────────────────────────────────────

fn utf8_len(first: u8) -> usize {
    match first {
        0x00..=0x7F => 1,
        0xC0..=0xDF => 2,
        0xE0..=0xEF => 3,
        _ => 4,
    }
}

/// 从 `start` 处的引号开始，返回闭合引号之后的下标。
/// SQL 用双写引号表示字面引号（`''` / `""` / ``` `` ```）。
fn skip_quoted(bytes: &[u8], start: usize, quote: u8) -> usize {
    let len = bytes.len();
    let mut i = start + 1;
    while i < len {
        if bytes[i] == quote {
            if i + 1 < len && bytes[i + 1] == quote {
                i += 2;
                continue;
            }
            return i + 1;
        }
        i += 1;
    }
    len
}

/// 把 SQL 拆成「代码段」与「字面量/注释段」，只对代码段调用 `rewrite`。
fn rewrite_code_spans<F>(sql: &str, mut rewrite: F) -> String
where
    F: FnMut(&str) -> String,
{
    let bytes = sql.as_bytes();
    let len = bytes.len();
    let mut out = String::with_capacity(sql.len());
    let mut i = 0usize;
    let mut code_start = 0usize;

    while i < len {
        let b = bytes[i];
        if b == b'\'' || b == b'"' || b == b'`' {
            if code_start < i {
                out.push_str(&rewrite(&sql[code_start..i]));
            }
            let end = skip_quoted(bytes, i, b);
            out.push_str(&sql[i..end]);
            i = end;
            code_start = i;
            continue;
        }
        if b == b'-' && i + 1 < len && bytes[i + 1] == b'-' {
            if code_start < i {
                out.push_str(&rewrite(&sql[code_start..i]));
            }
            let start = i;
            i += 2;
            while i < len && bytes[i] != b'\n' {
                i += 1;
            }
            out.push_str(&sql[start..i]);
            code_start = i;
            continue;
        }
        if b == b'/' && i + 1 < len && bytes[i + 1] == b'*' {
            if code_start < i {
                out.push_str(&rewrite(&sql[code_start..i]));
            }
            let start = i;
            i += 2;
            while i + 1 < len && !(bytes[i] == b'*' && bytes[i + 1] == b'/') {
                i += 1;
            }
            i = (i + 2).min(len);
            out.push_str(&sql[start..i]);
            code_start = i;
            continue;
        }
        i += utf8_len(b).min(len - i);
    }

    if code_start < len {
        out.push_str(&rewrite(&sql[code_start..]));
    }
    out
}

fn is_ident_byte(b: u8) -> bool {
    b.is_ascii_alphanumeric() || b == b'_' || b == b'$'
}

/// 大小写不敏感、带标识符边界检查的替换（只用于已切出的代码段）。
fn replace_token(code: &str, needle: &str, replacement: &str) -> String {
    let bytes = code.as_bytes();
    let len = bytes.len();
    let needle_bytes = needle.as_bytes();
    let nlen = needle_bytes.len();
    let mut out = String::with_capacity(code.len());
    let mut i = 0usize;

    while i < len {
        let matches = i + nlen <= len
            && bytes[i..i + nlen].eq_ignore_ascii_case(needle_bytes)
            && (!is_ident_byte(needle_bytes[0]) || i == 0 || !is_ident_byte(bytes[i - 1]))
            && (!is_ident_byte(needle_bytes[nlen - 1])
                || i + nlen == len
                || !is_ident_byte(bytes[i + nlen]));
        if matches {
            out.push_str(replacement);
            i += nlen;
        } else {
            let ch_len = utf8_len(bytes[i]).min(len - i);
            out.push_str(&code[i..i + ch_len]);
            i += ch_len;
        }
    }
    out
}

fn replace_code_token(sql: &str, needle: &str, replacement: &str) -> String {
    rewrite_code_spans(sql, |code| replace_token(code, needle, replacement))
}

/// 在代码段中查找 `needle`（跳过字符串、标识符与注释）。
fn find_token_in_code(sql: &str, needle: &str, from: usize) -> Option<usize> {
    let bytes = sql.as_bytes();
    let len = bytes.len();
    let mut i = from.min(len);

    while i < len {
        let b = bytes[i];
        if b == b'\'' || b == b'"' || b == b'`' {
            i = skip_quoted(bytes, i, b);
            continue;
        }
        if b == b'-' && i + 1 < len && bytes[i + 1] == b'-' {
            i += 2;
            while i < len && bytes[i] != b'\n' {
                i += 1;
            }
            continue;
        }
        if b == b'/' && i + 1 < len && bytes[i + 1] == b'*' {
            i += 2;
            while i + 1 < len && !(bytes[i] == b'*' && bytes[i + 1] == b'/') {
                i += 1;
            }
            i = (i + 2).min(len);
            continue;
        }
        if sql[i..].starts_with(needle) {
            return Some(i);
        }
        i += utf8_len(b).min(len - i);
    }
    None
}

fn rfind_token_in_code(sql: &str, needle: &str, before: usize) -> Option<usize> {
    let mut found = None;
    let mut from = 0usize;
    while let Some(pos) = find_token_in_code(sql, needle, from) {
        if pos >= before {
            break;
        }
        found = Some(pos);
        from = pos + needle.len();
    }
    found
}

// ──────────────────────────────────────────────────────────────────────────────
// DO $$ ... $$ 匿名块处理
// ──────────────────────────────────────────────────────────────────────────────

fn replace_do_blocks(sql: &str) -> String {
    let mut result = String::with_capacity(sql.len());
    let mut remaining = sql;

    while let Some(do_pos) = remaining.find("DO $$") {
        result.push_str(&remaining[..do_pos]);

        let content_start = do_pos + "DO $$".len();
        let Some(end_pos) = find_dollar_block_end(remaining, content_start) else {
            // 没有闭合的 `$$`：原样保留，绝不吞掉后续 SQL。
            result.push_str(&remaining[do_pos..]);
            remaining = "";
            break;
        };

        let block = &remaining[content_start..end_pos];
        result.push_str(&rewrite_do_block_content(block));

        let mut rest = &remaining[end_pos + 2..];
        // 吃掉 `$$` 后的分号，保留其后的换行作为语句分隔。
        if let Some(stripped) = rest.strip_prefix(';') {
            rest = stripped;
        }
        if !rest.is_empty() && !rest.starts_with('\n') && !rest.starts_with('\r') {
            result.push('\n');
        }
        remaining = rest;
    }

    result.push_str(remaining);
    result
}

/// 找到与 `DO $$` 配对的结束 `$$`（跳过字符串字面量）。
fn find_dollar_block_end(sql: &str, from: usize) -> Option<usize> {
    let bytes = sql.as_bytes();
    let len = bytes.len();
    let mut i = from;

    while i + 1 < len {
        if bytes[i] == b'\'' {
            i = skip_quoted(bytes, i, b'\'');
            continue;
        }
        if bytes[i] == b'$' && bytes[i + 1] == b'$' {
            return Some(i);
        }
        i += 1;
    }
    None
}

fn rewrite_do_block_content(block: &str) -> String {
    let pg_catalog_patterns = [
        "PG_TABLES",
        "PG_CONSTRAINT",
        "PG_CLASS",
        "PG_ATTRIBUTE",
        "PG_CATALOG.PG_TABLES",
        "PG_CATALOG.PG_CONSTRAINT",
        "PG_CATALOG.PG_CLASS",
        "PG_CATALOG.PG_ATTRIBUTE",
    ];

    block
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .filter(|line| {
            let upper = line.to_ascii_uppercase();
            let is_control_flow = upper == "DECLARE"
                || upper == "BEGIN"
                || upper == "END"
                || upper.starts_with("END;")
                || upper.starts_with("END ")
                || upper.starts_with("IF ")
                || upper.starts_with("ELSIF ")
                || upper == "ELSE"
                || upper.starts_with("LOOP")
                || upper.starts_with("EXCEPTION")
                || upper.starts_with("WHEN ")
                || upper == "COMMIT";
            let mentions_pg_catalog = pg_catalog_patterns.iter().any(|p| upper.contains(p));
            !is_control_flow && !mentions_pg_catalog
        })
        .collect::<Vec<_>>()
        .join("\n")
}

// ──────────────────────────────────────────────────────────────────────────────
// $N → ?
// ──────────────────────────────────────────────────────────────────────────────

fn replace_pg_params(sql: &str) -> String {
    rewrite_code_spans(sql, |code| {
        let mut out = String::with_capacity(code.len());
        let mut chars = code.chars().peekable();
        while let Some(c) = chars.next() {
            if c == '$' && matches!(chars.peek(), Some(d) if d.is_ascii_digit()) {
                out.push('?');
                chars.next();
                while matches!(chars.peek(), Some(d) if d.is_ascii_digit()) {
                    chars.next();
                }
                continue;
            }
            out.push(c);
        }
        out
    })
}

// ──────────────────────────────────────────────────────────────────────────────
// expr::type → CAST(expr AS type)
// ──────────────────────────────────────────────────────────────────────────────

fn replace_cast_ops(sql: &str) -> String {
    let bytes = sql.as_bytes();
    let len = bytes.len();
    let mut out = String::with_capacity(sql.len());
    let mut i = 0usize;
    // `out` 中「最后一个完整表达式」的起点。遇到 `::` 时直接截断到这里再包 CAST，
    // 避免旧实现「表达式已经输出、又整体拼一遍」造成的重复。
    let mut last_expr: Option<usize> = None;
    // 函数/标识符起点：`COUNT(` 的表达式起点是 `COUNT`，不是 `(`。
    let mut last_ident: Option<usize> = None;
    let mut paren_stack: Vec<usize> = Vec::new();

    while i < len {
        let b = bytes[i];

        if b == b'\'' || b == b'"' || b == b'`' {
            let start = out.len();
            let end = skip_quoted(bytes, i, b);
            out.push_str(&sql[i..end]);
            i = end;
            last_expr = Some(start);
            last_ident = (b != b'\'').then_some(start);
            continue;
        }

        if b == b'-' && i + 1 < len && bytes[i + 1] == b'-' {
            let start = i;
            i += 2;
            while i < len && bytes[i] != b'\n' {
                i += 1;
            }
            out.push_str(&sql[start..i]);
            continue;
        }
        if b == b'/' && i + 1 < len && bytes[i + 1] == b'*' {
            let start = i;
            i += 2;
            while i + 1 < len && !(bytes[i] == b'*' && bytes[i + 1] == b'/') {
                i += 1;
            }
            i = (i + 2).min(len);
            out.push_str(&sql[start..i]);
            continue;
        }

        if b == b':' && i + 1 < len && bytes[i + 1] == b':' {
            let (type_end, type_name) = scan_cast_type(sql, i + 2);
            if !type_name.is_empty() {
                if let Some(expr_start) = last_expr.filter(|&s| s <= out.len()) {
                    let expr = out[expr_start..].trim_end().to_string();
                    if !expr.is_empty() {
                        out.truncate(expr_start);
                        out.push_str("CAST(");
                        out.push_str(&expr);
                        out.push_str(" AS ");
                        out.push_str(&map_cast_type(type_name));
                        out.push(')');
                        last_expr = Some(expr_start);
                        last_ident = None;
                        i = type_end;
                        continue;
                    }
                }
            }
            // 认不出表达式或类型：保留 `::`，至少不丢字符。
            out.push_str("::");
            i += 2;
            last_expr = None;
            last_ident = None;
            continue;
        }

        if b == b'(' {
            // 紧贴标识符的 `(` 是函数调用（表达式起点是函数名）；
            // 前面有空白/运算符的 `(` 是分组（表达式起点是括号本身）。
            let is_call = i > 0 && is_ident_byte(bytes[i - 1]);
            let group_start = if is_call {
                last_ident.unwrap_or(out.len())
            } else {
                out.len()
            };
            paren_stack.push(group_start);
            out.push('(');
            i += 1;
            last_expr = None;
            last_ident = None;
            continue;
        }

        if b == b')' {
            out.push(')');
            i += 1;
            last_expr = paren_stack.pop();
            last_ident = None;
            continue;
        }

        if b == b'.' {
            // 点号是标识符链的一部分：`u.name` 的表达式起点仍是 `u`。
            out.push('.');
            i += 1;
            continue;
        }

        if b == b'?' {
            let start = out.len();
            out.push('?');
            i += 1;
            last_expr = Some(start);
            last_ident = None;
            continue;
        }

        if b == b'$' && i + 1 < len && bytes[i + 1].is_ascii_digit() {
            let start = out.len();
            let mut end = i + 1;
            while end < len && bytes[end].is_ascii_digit() {
                end += 1;
            }
            out.push_str(&sql[i..end]);
            i = end;
            last_expr = Some(start);
            last_ident = None;
            continue;
        }

        if b.is_ascii_digit() {
            let start = out.len();
            let mut end = i + 1;
            while end < len {
                let c = bytes[end];
                if c.is_ascii_digit() || c == b'.' {
                    end += 1;
                } else if (c == b'e' || c == b'E') && end + 1 < len {
                    end += 1;
                    if bytes[end] == b'+' || bytes[end] == b'-' {
                        end += 1;
                    }
                } else {
                    break;
                }
            }
            out.push_str(&sql[i..end]);
            i = end;
            last_expr = Some(start);
            last_ident = None;
            continue;
        }

        if b.is_ascii_alphabetic() || b == b'_' || b >= 0x80 {
            let start = out.len();
            let mut end = i + 1;
            if b < 0x80 {
                while end < len
                    && (bytes[end].is_ascii_alphanumeric()
                        || bytes[end] == b'_'
                        || bytes[end] == b'$')
                {
                    end += 1;
                }
            } else {
                end = i + utf8_len(b).min(len - i);
            }
            let continued = out.trim_end().ends_with('.');
            out.push_str(&sql[i..end]);
            i = end;
            if continued {
                last_expr = last_expr.or(Some(start));
                last_ident = last_expr;
            } else {
                last_expr = Some(start);
                last_ident = Some(start);
            }
            continue;
        }

        let ch_len = utf8_len(b).min(len - i);
        out.push_str(&sql[i..i + ch_len]);
        i += ch_len;
        if !b.is_ascii_whitespace() {
            last_expr = None;
            last_ident = None;
        }
    }

    out
}

/// 扫描 `::` 后面的类型名，支持 `timestamp with time zone`、`double precision`
/// 这类多词类型与 `[]` 数组后缀。返回（类型结束下标, 类型原文）。
fn scan_cast_type(sql: &str, start: usize) -> (usize, &str) {
    let bytes = sql.as_bytes();
    let len = bytes.len();
    let mut i = start;
    while i < len && bytes[i].is_ascii_whitespace() {
        i += 1;
    }
    let type_start = i;
    while i < len && (bytes[i].is_ascii_alphanumeric() || bytes[i] == b'_') {
        i += 1;
    }
    if i == type_start {
        return (start, "");
    }

    let mut lower = sql[type_start..i].to_ascii_lowercase();
    loop {
        let mut j = i;
        while j < len && bytes[j].is_ascii_whitespace() {
            j += 1;
        }
        let word_start = j;
        while j < len && (bytes[j].is_ascii_alphanumeric() || bytes[j] == b'_') {
            j += 1;
        }
        if word_start == j {
            break;
        }
        let word = sql[word_start..j].to_ascii_lowercase();
        let candidate = format!("{lower} {word}");
        let known = matches!(
            candidate.as_str(),
            "timestamp with"
                | "timestamp without"
                | "timestamp with time"
                | "timestamp without time"
                | "timestamp with time zone"
                | "timestamp without time zone"
                | "time with"
                | "time without"
                | "time with time"
                | "time without time"
                | "time with time zone"
                | "time without time zone"
                | "double precision"
                | "character varying"
                | "bit varying"
        );
        if !known {
            break;
        }
        lower = candidate;
        i = j;
    }

    let mut bracket = i;
    while bracket < len && bytes[bracket].is_ascii_whitespace() {
        bracket += 1;
    }
    if bracket + 1 < len && bytes[bracket] == b'[' && bytes[bracket + 1] == b']' {
        i = bracket + 2;
    }

    (i, &sql[type_start..i])
}

fn map_cast_type(type_name: &str) -> String {
    let normalized = type_name.trim().to_ascii_lowercase();
    let base = normalized.strip_suffix("[]").unwrap_or(&normalized).trim();
    match base {
        "text" | "varchar" | "character varying" | "char" | "character" | "bpchar" | "uuid"
        | "citext" => "CHAR".to_string(),
        "jsonb" => "json".to_string(),
        "json" => "JSON".to_string(),
        "bigint" | "int8" => "BIGINT".to_string(),
        "integer" | "int" | "int4" | "smallint" | "int2" => "SIGNED".to_string(),
        "boolean" | "bool" => "UNSIGNED".to_string(),
        "timestamptz" | "timestamp with time zone" | "time with time zone" => {
            "DATETIME(6)".to_string()
        }
        "timestamp" | "timestamp without time zone" | "datetime" => "DATETIME".to_string(),
        "date" => "DATE".to_string(),
        "double precision" | "float8" | "real" | "float4" => "DOUBLE".to_string(),
        "numeric" | "decimal" => "DECIMAL".to_string(),
        "bytea" => "BINARY".to_string(),
        _ => normalized.to_ascii_uppercase(),
    }
}

// ──────────────────────────────────────────────────────────────────────────────
// jsonb → json
// ──────────────────────────────────────────────────────────────────────────────

fn replace_jsonb(sql: &str) -> String {
    replace_code_token(sql, "jsonb", "json")
}

// ──────────────────────────────────────────────────────────────────────────────
// ILIKE → LIKE（MySQL 默认不区分大小写）
// ──────────────────────────────────────────────────────────────────────────────

fn replace_ilike(sql: &str) -> String {
    replace_code_token(sql, "ILIKE", "LIKE")
}

// ──────────────────────────────────────────────────────────────────────────────
// "ident" → `ident`（PG 双引号 → MySQL 反引号）
// ──────────────────────────────────────────────────────────────────────────────

fn replace_ident_quotes(sql: &str) -> String {
    let bytes = sql.as_bytes();
    let len = bytes.len();
    let mut out = String::with_capacity(sql.len());
    let mut i = 0usize;

    while i < len {
        let b = bytes[i];
        if b == b'\'' {
            let end = skip_quoted(bytes, i, b);
            out.push_str(&sql[i..end]);
            i = end;
            continue;
        }
        if b == b'-' && i + 1 < len && bytes[i + 1] == b'-' {
            let start = i;
            i += 2;
            while i < len && bytes[i] != b'\n' {
                i += 1;
            }
            out.push_str(&sql[start..i]);
            continue;
        }
        if b == b'/' && i + 1 < len && bytes[i + 1] == b'*' {
            let start = i;
            i += 2;
            while i + 1 < len && !(bytes[i] == b'*' && bytes[i + 1] == b'/') {
                i += 1;
            }
            i = (i + 2).min(len);
            out.push_str(&sql[start..i]);
            continue;
        }
        if b == b'"' {
            out.push('`');
            i += 1;
            while i < len {
                if bytes[i] == b'"' {
                    if i + 1 < len && bytes[i + 1] == b'"' {
                        out.push('"');
                        i += 2;
                    } else {
                        out.push('`');
                        i += 1;
                        break;
                    }
                } else {
                    let ch_len = utf8_len(bytes[i]).min(len - i);
                    out.push_str(&sql[i..i + ch_len]);
                    i += ch_len;
                }
            }
            continue;
        }
        let ch_len = utf8_len(b).min(len - i);
        out.push_str(&sql[i..i + ch_len]);
        i += ch_len;
    }

    out
}

// ──────────────────────────────────────────────────────────────────────────────
// TIMESTAMPTZ → DATETIME(6)
// ──────────────────────────────────────────────────────────────────────────────

fn replace_timestamptz(sql: &str) -> String {
    let s = replace_code_token(sql, "TIMESTAMP WITHOUT TIME ZONE", "TIMESTAMP");
    let s = replace_code_token(&s, "TIMESTAMP WITH TIME ZONE", "DATETIME(6)");
    replace_code_token(&s, "TIMESTAMPTZ", "DATETIME(6)")
}

// ──────────────────────────────────────────────────────────────────────────────
// ON CONFLICT (cols) DO NOTHING → INSERT IGNORE INTO ...（保留 VALUES 部分）
// ON CONFLICT (cols) DO UPDATE SET ... → ON DUPLICATE KEY UPDATE ...（MySQL 语法）
// ──────────────────────────────────────────────────────────────────────────────

fn replace_on_conflict_do_nothing(sql: &str) -> String {
    let mut result = String::with_capacity(sql.len());
    let mut remaining = sql;

    while let Some(pos) = find_token_in_code(remaining, "ON CONFLICT", 0) {
        result.push_str(&remaining[..pos]);

        // 只在本条语句内找 DO NOTHING / DO UPDATE，避免误吃下一条语句。
        let stmt_end = find_token_in_code(remaining, ";", pos).unwrap_or(remaining.len());
        let stmt = &remaining[..stmt_end];

        if let Some(do_nothing_pos) = find_token_in_code(stmt, "DO NOTHING", pos) {
            if let Some(insert_pos) = rfind_token_in_code(&result, "INSERT INTO", result.len()) {
                let before = result[..insert_pos].to_string();
                let after_insert = result[insert_pos + "INSERT INTO".len()..]
                    .trim_end()
                    .to_string();
                result = before + "INSERT IGNORE INTO" + &after_insert;
            }

            let after = &remaining[do_nothing_pos + "DO NOTHING".len()..];
            if let Some(sc_pos) = after.find(';') {
                remaining = &after[sc_pos..];
            } else {
                remaining = "";
            }
            continue;
        }

        if let Some(do_update_pos) = find_token_in_code(stmt, "DO UPDATE", pos) {
            let conflict_section = &stmt[pos..];
            if let Some(paren_end) = conflict_section.find(')') {
                let after_conflict_target = pos + paren_end + 1;
                let after_target = &remaining[after_conflict_target..];
                if let Some(set_pos) = find_token_in_code(after_target, "SET", 0) {
                    result.push_str("ON DUPLICATE KEY UPDATE");
                    remaining = &after_target[set_pos + 3..];
                } else {
                    result.push_str("ON DUPLICATE KEY UPDATE");
                    remaining = after_target;
                }
            } else {
                result.push_str("ON DUPLICATE KEY UPDATE");
                remaining = &remaining[do_update_pos..];
            }
            continue;
        }

        remaining = &remaining[pos + "ON CONFLICT".len()..];
    }

    result.push_str(remaining);
    result
}

// ──────────────────────────────────────────────────────────────────────────────
// ALTER TABLE ... ADD COLUMN IF NOT EXISTS → 删除 IF NOT EXISTS
// ──────────────────────────────────────────────────────────────────────────────

fn replace_alter_if_not_exists(sql: &str) -> String {
    replace_code_token(sql, "ADD COLUMN IF NOT EXISTS", "ADD COLUMN")
}

// ──────────────────────────────────────────────────────────────────────────────
// gen_random_uuid() → UUID()
// ──────────────────────────────────────────────────────────────────────────────

fn replace_gen_random_uuid(sql: &str) -> String {
    replace_code_token(sql, "gen_random_uuid()", "UUID()")
}

// ──────────────────────────────────────────────────────────────────────────────
// pg_catalog.xxx → xxx（删除 pg_catalog schema 前缀）
// ──────────────────────────────────────────────────────────────────────────────

fn replace_pg_catalog(sql: &str) -> String {
    replace_code_token(sql, "pg_catalog.", "")
}

// ──────────────────────────────────────────────────────────────────────────────
// 单元测试
// ──────────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

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

    #[test]
    fn replaces_nested_casts_without_duplicating_the_expression() {
        assert_eq!(
            rewrite_pg_to_mysql("SELECT $1::text::jsonb"),
            "SELECT CAST(CAST(? AS CHAR) AS json)"
        );
    }

    #[test]
    fn replaces_parenthesized_cast() {
        assert_eq!(
            rewrite_pg_to_mysql("SELECT (a + b)::bigint FROM t"),
            "SELECT CAST((a + b) AS BIGINT) FROM t"
        );
    }

    #[test]
    fn replaces_cast_of_string_literal() {
        assert_eq!(
            rewrite_pg_to_mysql("SELECT '42'::integer"),
            "SELECT CAST('42' AS SIGNED)"
        );
    }

    #[test]
    fn replaces_cast_of_dotted_identifier() {
        assert_eq!(
            rewrite_pg_to_mysql("SELECT u.name::text FROM users u"),
            "SELECT CAST(u.name AS CHAR) FROM users u"
        );
    }

    #[test]
    fn replaces_multiword_cast_type() {
        assert_eq!(
            rewrite_pg_to_mysql("SELECT created_at::timestamp with time zone FROM t"),
            "SELECT CAST(created_at AS DATETIME(6)) FROM t"
        );
    }

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
            "SELECT 'type is jsonb' AS note"
        );
    }

    #[test]
    fn does_not_replace_keywords_inside_string_literals() {
        let sql = "SELECT 'jsonb ILIKE TIMESTAMPTZ $1' AS note, name ILIKE $2";
        assert_eq!(
            rewrite_pg_to_mysql(sql),
            "SELECT 'jsonb ILIKE TIMESTAMPTZ $1' AS note, name LIKE ?"
        );
    }

    #[test]
    fn does_not_replace_keywords_inside_comments() {
        let sql = "-- jsonb ILIKE TIMESTAMPTZ $1\nSELECT name ILIKE $2";
        assert_eq!(
            rewrite_pg_to_mysql(sql),
            "-- jsonb ILIKE TIMESTAMPTZ $1\nSELECT name LIKE ?"
        );
    }

    #[test]
    fn does_not_rewrite_jsonb_as_identifier_prefix() {
        assert_eq!(
            rewrite_pg_to_mysql("SELECT jsonb_path_exists(data, '$.a') FROM t"),
            "SELECT jsonb_path_exists(data, '$.a') FROM t"
        );
    }

    #[test]
    fn replaces_ilike() {
        assert_eq!(
            rewrite_pg_to_mysql("SELECT * FROM users WHERE name ILIKE '%foo%'"),
            "SELECT * FROM users WHERE name LIKE '%foo%'"
        );
    }

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

    #[test]
    fn preserves_doubled_quotes_in_identifiers() {
        assert_eq!(
            rewrite_pg_to_mysql("SELECT \"a\"\"b\" FROM t"),
            "SELECT `a\"b` FROM t"
        );
    }

    #[test]
    fn preserves_now_function() {
        assert_eq!(rewrite_pg_to_mysql("SELECT NOW()"), "SELECT NOW()");
    }

    #[test]
    fn replaces_timestamptz() {
        assert_eq!(
            rewrite_pg_to_mysql("applied_at TIMESTAMPTZ NOT NULL DEFAULT now()"),
            "applied_at DATETIME(6) NOT NULL DEFAULT now()"
        );
    }

    #[test]
    fn replaces_on_conflict_do_nothing() {
        let sql = "INSERT INTO t (id) VALUES ($1) ON CONFLICT (id) DO NOTHING;";
        assert_eq!(
            rewrite_pg_to_mysql(sql),
            "INSERT IGNORE INTO t (id) VALUES (?);"
        );
    }

    #[test]
    fn replaces_on_conflict_do_update() {
        let sql =
            "INSERT INTO t (id, name) VALUES ($1, $2) ON CONFLICT (id) DO UPDATE SET name = $2;";
        assert_eq!(
            rewrite_pg_to_mysql(sql),
            "INSERT INTO t (id, name) VALUES (?, ?) ON DUPLICATE KEY UPDATE name = ?;"
        );
    }

    #[test]
    fn ignores_on_conflict_inside_string_literal() {
        let sql = "INSERT INTO t (note) VALUES ('ON CONFLICT (id) DO NOTHING');";
        assert_eq!(
            rewrite_pg_to_mysql(sql),
            "INSERT INTO t (note) VALUES ('ON CONFLICT (id) DO NOTHING');"
        );
    }

    #[test]
    fn replaces_timestamp_without_timezone() {
        assert_eq!(
            rewrite_pg_to_mysql("create_time TIMESTAMP WITHOUT TIME ZONE"),
            "create_time TIMESTAMP"
        );
    }

    #[test]
    fn replaces_timestamp_with_timezone() {
        assert_eq!(
            rewrite_pg_to_mysql("create_time TIMESTAMP WITH TIME ZONE"),
            "create_time DATETIME(6)"
        );
    }

    #[test]
    fn strips_do_block_pg_catalog() {
        let sql = "\
            DO $$\n\
            BEGIN\n\
            IF EXISTS (SELECT 1 FROM pg_tables WHERE tablename = 'foo') THEN\n\
                ALTER TABLE foo RENAME TO bar;\n\
            END IF;\n\
            END $$;\n\
            CREATE TABLE t (id VARCHAR(255) PRIMARY KEY);";
        let expected = "\
            ALTER TABLE foo RENAME TO bar;\n\
            CREATE TABLE t (id VARCHAR(255) PRIMARY KEY);";
        assert_eq!(rewrite_pg_to_mysql(sql), expected);
    }

    #[test]
    fn keeps_the_rest_when_do_block_is_unterminated() {
        let sql = "DO $$\nBEGIN\nALTER TABLE t ADD COLUMN c TEXT;\nSELECT 1;";
        assert_eq!(rewrite_pg_to_mysql(sql), sql);
    }

    #[test]
    fn strips_alter_if_not_exists() {
        assert_eq!(
            rewrite_pg_to_mysql("ALTER TABLE x ADD COLUMN IF NOT EXISTS col TEXT"),
            "ALTER TABLE x ADD COLUMN col TEXT"
        );
    }

    #[test]
    fn replaces_gen_random_uuid() {
        assert_eq!(
            rewrite_pg_to_mysql("id VARCHAR(255) PRIMARY KEY DEFAULT gen_random_uuid()"),
            "id VARCHAR(255) PRIMARY KEY DEFAULT UUID()"
        );
    }

    #[test]
    fn replaces_pg_catalog_prefix() {
        assert_eq!(
            rewrite_pg_to_mysql("SELECT * FROM pg_catalog.pg_tables"),
            "SELECT * FROM pg_tables"
        );
    }

    #[test]
    fn rewrites_complex_query() {
        let pg_sql = "\
            SELECT u.id, u.name::text, u.data::jsonb \
            FROM users u \
            WHERE u.email ILIKE $1 AND u.status = $2";
        let expected = "\
            SELECT u.id, CAST(u.name AS CHAR), CAST(u.data AS json) \
            FROM users u \
            WHERE u.email LIKE ? AND u.status = ?";
        assert_eq!(rewrite_pg_to_mysql(pg_sql), expected);
    }

    #[test]
    fn handles_insert_with_jsonb_and_params() {
        let pg_sql = "INSERT INTO events (data, created_at) VALUES ($1::jsonb, NOW())";
        let expected = "INSERT INTO events (data, created_at) VALUES (CAST(? AS json), NOW())";
        assert_eq!(rewrite_pg_to_mysql(pg_sql), expected);
    }

    #[test]
    fn handles_cast_with_quoted_ident() {
        let pg_sql = "SELECT \"userId\"::text FROM t WHERE id = $1";
        let expected = "SELECT CAST(`userId` AS CHAR) FROM t WHERE id = ?";
        assert_eq!(rewrite_pg_to_mysql(pg_sql), expected);
    }

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

    #[test]
    fn rewrites_full_migration_header() {
        let sql = "\
            CREATE TABLE IF NOT EXISTS schema_migrations (\n\
                version     TEXT PRIMARY KEY,\n\
                applied_at  TIMESTAMPTZ NOT NULL DEFAULT now(),\n\
                checksum    TEXT NOT NULL,\n\
                execution_ms INTEGER NOT NULL DEFAULT 0\n\
            );";
        let expected = "\
            CREATE TABLE IF NOT EXISTS schema_migrations (\n\
                version     TEXT PRIMARY KEY,\n\
                applied_at  DATETIME(6) NOT NULL DEFAULT now(),\n\
                checksum    TEXT NOT NULL,\n\
                execution_ms INTEGER NOT NULL DEFAULT 0\n\
            );";
        assert_eq!(rewrite_pg_to_mysql(sql), expected);
    }

    // ── 2026-10-29 覆盖率轮：注释扫描 / 多字节 / 数组类型 / 多语句 ON CONFLICT ──

    #[test]
    fn line_comments_are_preserved_and_skipped_by_cast_scan() {
        let sql = "SELECT 1 -- keep ::text here
, 'x'::varchar";
        let out = rewrite_pg_to_mysql(sql);
        assert!(
            out.contains("-- keep ::text here"),
            "注释内容原样保留：{out}"
        );
        assert!(
            out.contains("CAST('x' AS CHAR)"),
            "注释外的 cast 正常改写：{out}"
        );
        let comment = &out[out.find("--").unwrap()..out.find('\n').unwrap()];
        assert!(!comment.contains("CAST"), "cast 扫描不得进注释：{comment}");
    }

    #[test]
    fn block_comments_are_preserved_and_skipped_by_cast_scan() {
        let sql = "SELECT /* ::text */ 'x'::varchar";
        let out = rewrite_pg_to_mysql(sql);
        assert!(out.contains("/* ::text */"), "块注释原样保留：{out}");
        assert!(out.contains("CAST"), "注释外的 cast 正常改写：{out}");
    }

    #[test]
    fn multibyte_identifiers_do_not_break_byte_scanning() {
        // 中文字符按 UTF-8 多字节推进，注释里的多字节不得吃掉换行
        let sql = "SELECT '中文内容' -- 中文注释 ::text
, 1::int";
        let out = rewrite_pg_to_mysql(sql);
        assert!(out.contains("中文内容"));
        assert!(out.contains("中文注释"));
        assert!(
            out.contains("CAST(1 AS SIGNED)"),
            "多字节注释后的 cast 正常改写：{out}"
        );
    }

    #[test]
    fn unrecognized_cast_left_operand_is_preserved_verbatim() {
        // 左操作数认不出表达式：保留 `::`，至少不丢字符
        let sql = "SELECT ,::text";
        let out = rewrite_pg_to_mysql(sql);
        assert!(out.contains("::text"), "认不出时必须原样保留：{out}");
    }

    #[test]
    fn numeric_literal_then_cast_is_tracked() {
        let sql = "SELECT 123::text";
        let out = rewrite_pg_to_mysql(sql);
        assert!(
            out.contains("CAST(123 AS CHAR)"),
            "数字字面量 cast 改写：{out}"
        );
    }

    #[test]
    fn sized_array_cast_keeps_dimensions() {
        let sql = "SELECT $1::text[3]";
        let out = rewrite_pg_to_mysql(sql);
        // text[]/text[N] 家族映射 CHAR，维度原样保留
        assert!(out.contains("CHAR"), "数组 cast 映射 CHAR：{out}");
        assert!(out.contains("[3]"), "维度不得丢：{out}");
    }

    #[test]
    fn on_conflict_do_update_rewrites_to_on_duplicate_key_in_multi_statement() {
        let sql = "INSERT INTO t (a) VALUES (1) ON CONFLICT (id) DO UPDATE SET a = excluded.a;
SELECT 2::int";
        let out = rewrite_pg_to_mysql(sql);
        assert!(
            out.contains("ON DUPLICATE KEY UPDATE"),
            "DO UPDATE → ON DUPLICATE KEY UPDATE：{out}"
        );
        assert!(
            out.contains("a = excluded.a"),
            "冲突更新赋值原样保留：{out}"
        );
        assert!(
            out.contains("CAST(2 AS SIGNED)"),
            "第二条语句的 cast 也改写：{out}"
        );
    }
}
