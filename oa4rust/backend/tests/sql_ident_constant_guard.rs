// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

//! SQL 插值列名常量守卫：把列名插进 SQL 的 helper，其列参数必须是**小写**或
//! **已带 SQL 引号**。
//!
//! # 这个测试防的缺陷
//!
//! PostgreSQL 对不带引号的标识符折叠成小写：DDL（migrations/065）建的列是
//! `"xworkCompleted"`（混合大小写），SQL 里写不加引号的 `xworkCompleted` 会变成
//! `xworkcompleted`——**不是同一个列**，fresh DB 上必 500。2026-10-26 CI PG 日志
//! 实证过 5 处（`processplatform_assemble_surface` 的 pp_c_attachment/pp_c_snap
//! 查询，F-6）。
//!
//! 缺陷形态：helper 用 `format!` 把列名常量插进 SQL（`WHERE {ref_col} = $1`），
//! 而 FI-002 列扫描器对 `format!` 模板的 `{}` 插值内容不可见（B-5）。本守卫把
//! 「插值点的列常量必须引号」钉成静态判据。
//!
//! # 口径与边界（fail-loud）
//!
//! - 管辖的 helper 家族（`processplatform_assemble_surface` 私有，全仓仅此一处）：
//!   `u2_att_get_with_check` / `u2_att_store_new` / `u2_att_copy` /
//!   `u2_snap_by_type` / `u2_att_ext_download_handler!`；
//! - 判据是 helper 名之后的**第一个字符串字面量**——上述五个形态里它都是列参数
//!   （store_new 的第 6 参、get_with_check 的第 3 参、snap_by_type 的第 2 参、
//!   copy 的第 4 参、宏的第 2 参）。传变量而非字面量的调用无法静态判定，跳过；
//! - 合法值：全小写（`"xwork"`——折叠后仍是 xwork）或已引号（raw string
//!   `r#""xworkCompleted""#` 或转义形态，运行期值以引号开头）；
//! - 反向注入自证见下：裸混合大小写常量必须被点名。

use std::fs;
use std::path::{Path, PathBuf};

/// 把列名插进 SQL 的 helper 家族（私有于 processplatform_assemble_surface）。
const HELPERS: &[&str] = &[
    "u2_att_get_with_check(",
    "u2_att_store_new(",
    "u2_att_copy(",
    "u2_snap_by_type(",
    "u2_att_ext_download_handler!",
];

fn is_ident(s: &str) -> bool {
    !s.is_empty()
        && s.chars()
            .next()
            .map(|c| c.is_ascii_alphabetic() || c == '_')
            .unwrap_or(false)
        && s.chars().all(|c| c.is_ascii_alphanumeric() || c == '_')
}

/// 把字节偏移向下取到字符边界（防切片 panic）。
fn floor_boundary(text: &str, mut i: usize) -> usize {
    if i >= text.len() {
        return text.len();
    }
    while !text.is_char_boundary(i) {
        i -= 1;
    }
    i
}

/// 取 `start` 起 window 字节内第一个字符串字面量的**内容**（regular 或 raw）。
/// regular 串按 Rust 规则处理反斜杠转义（`\"` → `"` 进内容——正是判据需要的：
/// 转义引号形态的内容以引号开头，视为「已引号」）。
fn first_string_literal(text: &str, start: usize, window: usize) -> Option<String> {
    let bytes = text.as_bytes();
    let limit = floor_boundary(text, (start + window).min(text.len()));
    let mut i = start;
    while i < limit {
        let b = bytes[i];
        if b == b'r' {
            // raw string r"..."
            if i + 1 < limit && bytes[i + 1] == b'"' {
                if let Some(q) = text[i + 2..limit].find('"') {
                    return Some(text[i + 2..i + 2 + q].to_string());
                }
            }
            // raw string r#"..."#
            if text[i..limit].starts_with("r#\"") {
                if let Some(q) = text[i + 3..limit].find("\"#") {
                    return Some(text[i + 3..i + 3 + q].to_string());
                }
            }
        }
        if b == b'"' {
            let mut content = String::new();
            let mut j = i + 1;
            while j < limit {
                if bytes[j] == b'\\' {
                    if j + 1 < limit {
                        if let Some(c) = text[j + 1..].chars().next() {
                            content.push(c);
                        }
                    }
                    j += 2;
                    if j < limit && !text.is_char_boundary(j) {
                        while j < limit && !text.is_char_boundary(j) {
                            j += 1;
                        }
                    }
                    continue;
                }
                if bytes[j] == b'"' {
                    return Some(content);
                }
                if let Some(c) = text[j..].chars().next() {
                    content.push(c);
                    j += c.len_utf8();
                } else {
                    break;
                }
            }
            return Some(content);
        }
        i += 1;
    }
    None
}

/// 一段源码里的违例：helper 的首个字面量是裸混合大小写标识符。
fn bad_constants(text: &str) -> Vec<(String, String)> {
    let mut out = Vec::new();
    for &h in HELPERS {
        let mut from = 0usize;
        while let Some(k) = text[from..].find(h) {
            let at = from + k;
            if let Some(content) = first_string_literal(text, at + h.len(), 300) {
                let bare_mixed = is_ident(&content)
                    && content.chars().any(|c| c.is_ascii_uppercase())
                    && !content.starts_with('"');
                if bare_mixed {
                    out.push((h.trim_end_matches('(').to_string(), content));
                }
            }
            from = at + h.len();
        }
    }
    out
}

#[test]
fn sql_interpolated_column_constants_are_quoted_or_lowercase() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
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
        for (helper, col) in bad_constants(&text) {
            violations.push(format!(
                "{rel}: {helper} 的列常量 {col:?} 未引号——PG 会折叠成小写而与 DDL 的混合大小写列不符（fresh DB 必 500）"
            ));
        }
    }

    assert!(
        violations.is_empty(),
        "发现 {} 处「SQL 插值列常量未引号」——混合大小写列名必须带引号\n\
         （raw string r#\"\"x\"\"# 或转义形态；全小写也可）。修法参考 F-6（2026-10-26）。违例：\n{}",
        violations.len(),
        violations.join("\n")
    );
}

#[test]
fn guard_has_teeth_reverse_injection() {
    // 裸混合大小写 → 点名；小写 / raw string 引号形态 → 不误伤。
    let bad = bad_constants("u2_att_get_with_check(&pool, &id, \"xworkCompleted\", &v)");
    assert_eq!(bad.len(), 1, "裸混合大小写必须被点名: {bad:?}");
    assert_eq!(bad[0].1, "xworkCompleted");

    for ok_src in [
        "u2_att_get_with_check(&pool, &id, \"xwork\", &v)",
        "u2_att_get_with_check(&pool, &id, r#\"\"xworkCompleted\"\"#, &v)",
        "u2_snap_by_type(&pool, \"xworkCompleted\", &work, \"snap\")", // 该行本身违法——单独确认
    ] {
        let v = bad_constants(ok_src);
        if ok_src.contains("u2_snap_by_type") {
            assert_eq!(v.len(), 1, "snap 形态也要管辖: {v:?}");
        } else {
            assert!(v.is_empty(), "合法形态被误伤: {ok_src} -> {v:?}");
        }
    }

    // store_new / copy / 宏 三种形态都要认
    assert_eq!(
        bad_constants("u2_att_store_new(&pool, &p, &id, &n, bytes, \"xworkCompleted\", &v)").len(),
        1
    );
    assert_eq!(
        bad_constants("u2_att_copy(&pool, &p, &b, \"xworkCompleted\", &wc, false)").len(),
        1
    );
    assert_eq!(
        bad_constants(
            "u2_att_ext_download_handler!(attachment_u2c_download_wc_ext, \"xworkCompleted\");"
        )
        .len(),
        1
    );
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
