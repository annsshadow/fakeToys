// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

//! 引号大写关系守卫：SQL 里 `FROM/JOIN/UPDATE/INTO "UPPERCASE_NAME"`（带引号且
//! 含大写字母的关系名）必须有一个**生效中的**迁移创建它。
//!
//! # 这个测试防的缺陷
//!
//! PostgreSQL 对带引号标识符大小写敏感：`"PP_E_APPLICATION"` 与
//! `pp_e_application` 是两个不同的关系。`processplatform_assemble_surface` 的两个
//! handler（`application_list_key_key` / `application_list_terminal_terminal`，
//! lib.rs:887/975）一直查询 `"PP_E_APPLICATION"` 视图，而**没有任何生效中的迁移
//! 创建过它**——唯一来源是 `migrations/archive/089_fix_500_endpoints.sql.disabled`，
//! 当年因「views over nonexistent uppercase tables」被 archive（`27335205f`），
//! 迁移运行器（`shared/src/migrate.rs:84`）排除 archive，从未应用。后果：
//! dev/live 库（089 的列被手工应用过）能跑，fresh DB 必 500。
//!
//! FI-002 列扫描器看不见这类缺陷：它的模型把引号大写标识符折叠成小写
//! （`"PP_E_APPLICATION"` → `pp_e_application`，表存在、列也齐）——见
//! `scripts/fi002_column_scan.py` 头部 B-2 盲区登记。本守卫补这一格：
//! **大小写敏感**地核对「引号大写关系 ↔ 生效迁移的 CREATE」。
//!
//! 修复（migrations/112）后差集为空；豁免名单默认为空。
//!
//! # 口径
//!
//! - 扫 `crates/*/src`、`src/` 的 Rust 源码，但剥掉 `#[cfg(test)] mod ... { ... }`
//!   区间（括号配平，与 fi002_column_scan 的 cfg_test_ranges 同口径）——夹具 SQL
//!   常打虚构关系，不算违例；
//! - 只认**生效中**的迁移：`migrations/*.sql` 顶层（排除 `*_rollback.sql` 与
//!   `archive/`——archive 永不应用，算上它会让守卫对已禁用迁移失效）；
//! - `CREATE [OR REPLACE] TABLE|VIEW [IF NOT EXISTS] "NAME"` 都算（DO $$ 块内
//!   的也算——整文件正则扫，112 的视图就在 DO 块里）。

use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};

fn is_ident_byte(b: u8) -> bool {
    b.is_ascii_alphanumeric() || b == b'_'
}

/// 标识符形状：`[A-Za-z_][A-Za-z0-9_]*`。关系名/列名都必须过这一关——
/// 没有它，`route("/api/x/update", ...)` 的收尾引号会被当成关系名的开引号，
/// 提取出 `, post(crate::...)` 这类垃圾（P1-2 实跑分判出来的假阳性族）。
fn is_ident(s: &str) -> bool {
    !s.is_empty()
        && s.chars()
            .next()
            .map(|c| c.is_ascii_alphabetic() || c == '_')
            .unwrap_or(false)
        && s.chars().all(|c| c.is_ascii_alphanumeric() || c == '_')
}

/// UTF-8 字符宽度（用于字节游标安全步进，保证切片永不落在字符中间）。
fn utf8_width(b: u8) -> usize {
    if b < 0x80 {
        1
    } else if b >> 5 == 0b110 {
        2
    } else if b >> 4 == 0b1110 {
        3
    } else if b >> 3 == 0b11110 {
        4
    } else {
        1
    }
}

/// 从一段源码文本里抽「带引号且含大写字母」的关系引用，返回 (关系名, 上下文)。
fn quoted_relation_refs(text: &str) -> Vec<(String, String)> {
    let bytes = text.as_bytes();
    let lower = text.to_ascii_lowercase();
    const KEYWORDS: [&str; 4] = ["from", "join", "update", "insert into"];
    let mut out: Vec<(String, String)> = Vec::new();
    let mut i = 0usize;
    while i < bytes.len() {
        let b = bytes[i];
        if b >= 0x80 {
            i += utf8_width(b);
            continue;
        }
        // 关键字匹配（要求单词边界）
        let mut after_kw: Option<usize> = None;
        for kw in KEYWORDS {
            if lower[i..].starts_with(kw) {
                let before_ok = i == 0 || !is_ident_byte(bytes[i - 1]);
                let after = i + kw.len();
                let after_ok = after >= bytes.len() || !is_ident_byte(bytes[after]);
                if before_ok && after_ok {
                    after_kw = Some(after);
                    break;
                }
            }
        }
        if let Some(after) = after_kw {
            // 关键字后跳过空白，可选跳过一个反斜杠（Rust 源码里关系名是转义形态
            // `FROM \"NAME\"`——磁盘上的字面量文本含反斜杠），然后期待 '"'。
            let mut j = after;
            while j < bytes.len() && bytes[j].is_ascii_whitespace() {
                j += 1;
            }
            if j < bytes.len() && bytes[j] == b'\\' {
                j += 1;
            }
            if j < bytes.len() && bytes[j] == b'"' {
                let start = j + 1;
                if let Some(rel_end) = text[start..].find('"') {
                    let raw = &text[start..start + rel_end];
                    // 转义闭合引号 \" 前的反斜杠不属于关系名（PP_E_APPLICATION\
                    // → PP_E_APPLICATION）——端口到 Rust 前 Python 实跑抓到的第二类假阳性。
                    let rel = raw.strip_suffix('\\').unwrap_or(raw);
                    if is_ident(rel) && rel.chars().any(|c| c.is_ascii_uppercase()) {
                        let ctx: String = text[start..].chars().take(70).collect();
                        out.push((rel.to_string(), ctx));
                    }
                }
            }
            i = after;
            continue;
        }
        i += 1;
    }
    out
}

/// 剥掉 `#[cfg(test)] mod name { ... }` 区间（花括号配平；字符串/注释里的花括号
/// 不配平——与 fi002_column_scan 的同款粗口径，测试夹具 SQL 不进判据）。
fn strip_cfg_test_modules(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    let bytes = text.as_bytes();
    let mut i = 0usize;
    while i < bytes.len() {
        // 找下一个 #[cfg(test)]
        let marker = "#[cfg(test)]";
        if text[i..].starts_with(marker) {
            // 向后找 mod ... {
            let mut j = i + marker.len();
            while j < bytes.len() && bytes[j] != b'{' {
                // 只允许在合理窗口内找 mod 声明（防把普通属性后的函数体误剥）
                if j - i > 200 {
                    break;
                }
                j += 1;
            }
            if j < bytes.len() && bytes[j] == b'{' && text[i..j].contains("mod") {
                // 花括号配平跳过整块
                let mut depth = 0usize;
                let mut k = j;
                while k < bytes.len() {
                    match bytes[k] {
                        b'{' => depth += 1,
                        b'}' => {
                            depth -= 1;
                            if depth == 0 {
                                break;
                            }
                        }
                        _ => {}
                    }
                    k += 1;
                }
                out.push('\n');
                i = k + 1;
                continue;
            }
        }
        // 普通字符：按 UTF-8 宽度整字符拷入
        let w = utf8_width(bytes[i]).max(1);
        let end = (i + w).min(text.len());
        if text.is_char_boundary(end) {
            out.push_str(&text[i..end]);
        } else {
            out.push(' ');
        }
        i = end;
    }
    out
}

/// 从 `create` 关键字之后解析 `[or replace] table|view [if not exists] "NAME"`，
/// 返回引号内的关系名（大小写敏感）。不是表/视图的 CREATE（索引/序列等）返回 None。
fn parse_created_name(text: &str, after_create: usize) -> Option<String> {
    let bytes = text.as_bytes();
    let mut i = after_create;
    // skip ws
    while i < bytes.len() && bytes[i].is_ascii_whitespace() {
        i += 1;
    }
    // optional: or replace
    if text.len() >= i + 10 && text[i..i + 10].eq_ignore_ascii_case("or replace") {
        i += 10;
        while i < bytes.len() && bytes[i].is_ascii_whitespace() {
            i += 1;
        }
    }
    // table | view（必须是独立词）。先算布尔再改 i——闭包捕获 i 会与后面的
    // `i += 5` 撞 borrow check（NLL 救不了：word 在 else-if 分支仍被使用）。
    let is_word = |w: &str| -> bool {
        text.len() >= i + w.len()
            && text[i..i + w.len()].eq_ignore_ascii_case(w)
            && (i + w.len() >= bytes.len() || !is_ident_byte(bytes[i + w.len()]))
    };
    // 先算布尔再改 i——闭包捕获 i 会与后面的 `i += 5` 撞 borrow check
    // （NLL 在 is_word 最后一次使用后结束借用，无需显式 drop；显式 drop 反而会踩
    // clippy::drop_non_drop——suspicious 组默认告警，-D warnings 下即错误）。
    let is_table = is_word("table");
    let is_view = is_word("view");
    if is_table {
        i += 5;
    } else if is_view {
        i += 4;
    } else {
        return None;
    }
    while i < bytes.len() && bytes[i].is_ascii_whitespace() {
        i += 1;
    }
    // optional: if not exists
    if text.len() >= i + 13 && text[i..i + 13].eq_ignore_ascii_case("if not exists") {
        i += 13;
        while i < bytes.len() && bytes[i].is_ascii_whitespace() {
            i += 1;
        }
    }
    // quoted relation name
    if i < bytes.len() && bytes[i] == b'"' {
        let start = i + 1;
        if let Some(end) = text[start..].find('"') {
            let rel = &text[start..start + end];
            if is_ident(rel) && rel.chars().any(|c| c.is_ascii_uppercase()) {
                return Some(rel.to_string());
            }
        }
    }
    None
}

/// 从生效中的迁移里收集「已创建的关系名」（大小写敏感）。
fn created_relations(migrations_dir: &Path) -> HashSet<String> {
    let mut created = HashSet::new();
    let mut files: Vec<PathBuf> = Vec::new();
    if let Ok(entries) = fs::read_dir(migrations_dir) {
        for e in entries.flatten() {
            let p = e.path();
            let name = p.file_name().to_string_lossy().into_owned();
            if p.is_file()
                && name.ends_with(".sql")
                && !name.ends_with("_rollback.sql")
                && !name.ends_with(".disabled")
            {
                files.push(p);
            }
        }
    }
    files.sort();
    for f in files {
        let Ok(text) = fs::read_to_string(&f) else {
            continue;
        };
        let lower = text.to_ascii_lowercase();
        let mut from = 0usize;
        while let Some(i) = lower[from..].find("create") {
            let at = from + i;
            let after = at + "create".len();
            if let Some(rel) = parse_created_name(&text, after) {
                created.insert(rel);
            }
            from = after;
        }
    }
    created
}

/// 纯判据：一段源码里引用了但 created 里没有的引号大写关系。
fn violations_in(text: &str, created: &HashSet<String>) -> Vec<String> {
    quoted_relation_refs(text)
        .into_iter()
        .filter(|(rel, _)| !created.contains(rel))
        .map(|(rel, ctx)| format!("关系 {rel:?} 无生效迁移创建（{ctx}…）"))
        .collect()
}

#[test]
fn every_quoted_uppercase_relation_is_created_by_a_migration() {
    /// 确有意豁免的关系（登记理由）。当前为空：112 已建 PP_E_APPLICATION。
    const EXEMPT: &[&str] = &[];

    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let created = created_relations(&root.join("migrations"));

    let mut files: Vec<PathBuf> = Vec::new();
    collect_rs(&root.join("crates"), &mut files);
    collect_rs(&root.join("src"), &mut files);

    let mut violations: Vec<String> = Vec::new();
    for f in &files {
        let rel = f
            .strip_prefix(root)
            .unwrap_or(f)
            .to_string_lossy()
            .replace('\\', "/");
        let Ok(text) = fs::read_to_string(f) else {
            continue;
        };
        let prod = strip_cfg_test_modules(&text);
        for (name, ctx) in quoted_relation_refs(&prod) {
            if EXEMPT.contains(&name.as_str()) || created.contains(&name) {
                continue;
            }
            violations.push(format!("{rel}: 关系 {name:?} 无生效迁移创建（{ctx}…）"));
        }
    }

    assert!(
        violations.is_empty(),
        "发现 {} 处「引号大写关系无生效迁移」——PostgreSQL 对带引号标识符大小写敏感，\n\
         fresh DB 上这些 SQL 必 500（FI-002 扫描器的大小写折叠模型看不见，见 B-2）。\n\
         修法：在 migrations/ 补幂等 CREATE（参考 112 的 PP_E_APPLICATION 视图）；\n\
         确有意豁免请在 EXEMPT 登记理由。违例：\n{}",
        violations.len(),
        violations.join("\n")
    );
}

#[test]
fn guard_has_teeth_reverse_injection() {
    // 反向注入自证：空宇宙下假引用一个引号大写关系，必须被点名；
    // 不带引号/全小写的引用不误伤。
    let created: HashSet<String> = HashSet::new();

    let bad = violations_in(
        "client.query(\"SELECT xid FROM \\\"PP_GHOST_TABLE\\\" WHERE xid = $1\", &[])",
        &created,
    );
    assert_eq!(bad.len(), 1, "注入的幽灵关系必须被点名: {bad:?}");
    assert!(bad[0].contains("PP_GHOST_TABLE"));

    let unquoted = violations_in(
        "client.query(\"SELECT xid FROM pp_e_application WHERE xid = $1\", &[])",
        &created,
    );
    assert!(unquoted.is_empty(), "不带引号不该管辖: {unquoted:?}");

    let quoted_lower = violations_in(
        "client.query(\"SELECT xid FROM \\\"pp_e_application\\\" WHERE xid = $1\", &[])",
        &created,
    );
    assert!(
        quoted_lower.is_empty(),
        "引号但全小写（折叠后与小写表同名）不属本守卫管辖: {quoted_lower:?}"
    );

    // INSERT INTO / UPDATE / JOIN / raw-string 四种形态都要认
    for sql in [
        "client.execute(\"INSERT INTO \\\"PP_GHOST_2\\\" (id) VALUES ($1)\", &[])",
        "client.execute(\"UPDATE \\\"PP_GHOST_3\\\" SET x = $1\", &[])",
        "client.query(\"SELECT 1 FROM a JOIN \\\"PP_GHOST_4\\\" ON a.id = b.id\", &[])",
        "client.query(r#\"SELECT xid FROM \"PP_GHOST_5\" WHERE xid = $1\"#, &[])",
    ] {
        let v = violations_in(sql, &created);
        assert_eq!(v.len(), 1, "形态未覆盖: {sql} -> {v:?}");
    }

    // 负对照：不带引号 / 全小写引号 / 路径串收尾引号 / utoipa 参数串都不误伤
    for sql in [
        "client.query(\"SELECT xid FROM pp_e_application WHERE xid = $1\", &[])",
        "client.query(\"SELECT xid FROM \\\"pp_e_application\\\" WHERE xid = $1\", &[])",
        "router.route(\"/api/file/folder/update\", post(update_handler))",
        "params((\"from\" = String, Path, description = \"from parameter\"))",
    ] {
        assert!(
            violations_in(sql, &created).is_empty(),
            "负对照被误伤: {sql}"
        );
    }
}

fn collect_rs(dir: &Path, out: &mut Vec<PathBuf>) {
    let Ok(entries) = fs::read_dir(dir) else {
        return;
    };
    for entry in entries.flatten() {
        let p = entry.path();
        if p.is_dir() {
            if p.file_name().map(|n| n == "target").unwrap_or(false) {
                continue;
            }
            collect_rs(&p, out);
        } else if p.extension().map(|x| x == "rs").unwrap_or(false) {
            out.push(p);
        }
    }
}
