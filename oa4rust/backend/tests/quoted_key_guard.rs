// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

//! 转义引号键缺陷守卫（docs/plans/2026-09-20-001 §6.7 / 阶段 E6）
//!
//! 禁止源码出现 `"\"key\""` 形态的字面量 —— 键名里**字面包含引号**（转义错误）。
//!
//! 该缺陷曾造成 351 处问题，其中：
//!   - 81 处 `row.get("\"col\"")` 读不存在的列 → **panic(500)**
//!   - 115 处 `("\"key\"".to_string(), ..)` 产出带引号的响应键 → 前端读不到
//!   - 32 处 `payload.get("\"key\"")` 请求键读不到 → 字段静默丢失
//!   - 5 处 `.route("..{\"id\"}")` 路由参数名畸形
//!
//! 唯一合法例外是 **SQL 标识符引用**（如 `quote_ident` 的返回值/断言：
//! `assert_eq!(d.quote_ident("clueId"), "\"clueId\"")`）——那里的引号是 SQL 语义。
//!
//! 本测试是 E6 门禁：任何新增的转义引号键都会让它失败。

use std::fs;
use std::path::{Path, PathBuf};

/// 合法例外文件（相对仓库内 oa4rust/ 的路径）：SQL 标识符引用。
const ALLOWED_FILES: &[&str] = &["crates/shared/src/db/dialect.rs"];

/// 行内出现该标记即跳过（SQL 标识符引用语境）。
const SKIP_MARKERS: &[&str] = &["quote_ident", "IDENT"];

/// 模式开始：`"` `\` `"`
const OPEN: &str = r#""\""#;
/// 模式结束：`\` `"` `"`
const CLOSE: &str = r#"\"""#;

fn is_ident(s: &str) -> bool {
    !s.is_empty()
        && s.chars()
            .next()
            .map(|c| c.is_ascii_alphabetic() || c == '_')
            .unwrap_or(false)
        && s.chars().all(|c| c.is_ascii_alphanumeric() || c == '_')
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

/// 扫描单行，命中「转义引号键」返回 Some(键名)。
/// 抽成独立函数的原因（O3）：守卫必须自带破坏实验，证明修复误报后仍能抓真问题。
fn find_violation_in_line(line: &str) -> Option<String> {
    if SKIP_MARKERS.iter().any(|m| line.contains(m)) {
        return None;
    }
    let mut search_from = 0usize;
    while let Some(rel_idx) = line[search_from..].find(OPEN) {
        let start = search_from + rel_idx + OPEN.len();
        let Some(close_rel) = line[start..].find(CLOSE) else {
            break;
        };
        let key = &line[start..start + close_rel];
        if is_ident(key) {
            return Some(key.to_string());
        }
        search_from = start + close_rel + CLOSE.len();
    }
    None
}

#[test]
fn no_escaped_quote_keys() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let mut files = Vec::new();
    collect_rs(&root.join("crates"), &mut files);
    collect_rs(&root.join("src"), &mut files);

    let mut violations: Vec<String> = Vec::new();
    for f in &files {
        let rel = f
            .strip_prefix(root)
            .unwrap_or(f)
            .to_string_lossy()
            .replace('\\', "/");
        if ALLOWED_FILES.contains(&rel.as_str()) {
            continue;
        }
        let Ok(text) = fs::read_to_string(f) else {
            continue;
        };
        for (i, line) in text.lines().enumerate() {
            if find_violation_in_line(line).is_some() {
                violations.push(format!("{}:{}: {}", rel, i + 1, line.trim()));
            }
        }
    }

    assert!(
        violations.is_empty(),
        "发现 {} 处「转义引号键缺陷」（键名里字面包含引号）——见 docs/plans/2026-09-20-001 §6.7：\n{}",
        violations.len(),
        violations.join("\n")
    );
}

/// O3 破坏实验（守卫自检）：修好误报后，守卫必须仍能抓住真缺陷。
/// 三类历史真实缺陷形态（row.get 带引号列名 / 带引号响应键 / 带引号请求键）
/// 都必须被同一扫描逻辑捕获；合法形态（raw string、SQL 标识符引用、
/// 普通字符串）必须放行——后者正是 O3 修掉的误报类。
#[test]
fn guard_still_catches_real_violations() {
    // 破坏样本：与历史缺陷同型
    assert_eq!(
        find_violation_in_line(r#"let x = row.get("\"name\"");"#).as_deref(),
        Some("name"),
        "row.get 带引号列名必须被捕获"
    );
    assert_eq!(
        find_violation_in_line(r#"m.insert("\"key\"".to_string(), v);"#).as_deref(),
        Some("key"),
        "带引号响应键必须被捕获"
    );
    assert_eq!(
        find_violation_in_line(r#"payload.get("\"id\"")"#).as_deref(),
        Some("id"),
        "带引号请求键必须被捕获"
    );

    // 放行样本：O3 误报类（raw string 的 "paths"）与 SQL 标识符引用
    assert_eq!(
        find_violation_in_line(r##"assert!(first.contains(r#""paths""#));"##),
        None,
        "raw string 不含转义引号序列，不得误报"
    );
    assert_eq!(
        find_violation_in_line(r#"assert_eq!(d.quote_ident("clueId"), "\"clueId\"");"#),
        None,
        "quote_ident 行必须跳过（SQL 标识符语义）"
    );
    assert_eq!(
        find_violation_in_line(r#"let s = "plain text";"#),
        None,
        "普通字符串不得误报"
    );
}
