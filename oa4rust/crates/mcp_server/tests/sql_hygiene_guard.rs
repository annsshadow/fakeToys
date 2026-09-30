// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

//! SQL 卫生回归守卫：静态扫描全仓 `crates/*/src` 非测试源，钉死已根治的反模式不得复发。
//!
//! 覆盖历轮已清零的缺陷类（轮 28/31/33/34）：
//!   1. 时间列 `::text` 排序（`::text DESC/ASC`）——破坏 btree 索引排序；
//!   2. `deleted_at::text IS NULL`——谓词表达式不匹配部分索引 `deleted_at IS NULL`；
//!   3. 裸 `(page - 1) * size|count` 分页乘法——i64 溢出/下溢（须 `saturating_mul`）。
//! 这些是「名义修好、未来易回潮」的隐患，用编译期常驻测试固化。

use std::fs;
use std::path::{Path, PathBuf};

fn crates_dir() -> PathBuf {
    // crates/mcp_server -> oa4rust/crates
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn walk_rs(dir: &Path, out: &mut Vec<PathBuf>) {
    let Ok(rd) = fs::read_dir(dir) else {
        return;
    };
    for e in rd.flatten() {
        let p = e.path();
        if p.is_dir() {
            if p.file_name().map(|n| n == "target").unwrap_or(false) {
                continue;
            }
            walk_rs(&p, out);
        } else if p.extension().map(|x| x == "rs").unwrap_or(false) {
            out.push(p);
        }
    }
}

/// 测试专属文件（这些反模式在测试断言里可能出现，不构成缺陷）。
fn is_test_file(p: &Path) -> bool {
    let s = p.to_string_lossy();
    s.contains("/tests/") || s.contains("test") || s.contains("mock")
}

const PATTERNS: &[&str] = &[
    "::text DESC",
    "::text ASC",
    "deleted_at::text IS",
    "(page - 1) * size",
    "(page - 1) * count",
];

#[test]
fn sql_hygiene_antipatterns_stay_absent() {
    let root = crates_dir();
    let mut files: Vec<PathBuf> = Vec::new();
    for c in fs::read_dir(root.join("crates")).unwrap() {
        let src = c.unwrap().path().join("src");
        walk_rs(&src, &mut files);
    }
    assert!(
        !files.is_empty(),
        "未扫描到任何 crate 源文件，路径异常: {root:?}"
    );

    let mut violations: Vec<String> = Vec::new();
    for f in &files {
        if is_test_file(f) {
            continue;
        }
        let Ok(text) = fs::read_to_string(f) else {
            continue;
        };
        for pat in PATTERNS {
            if text.contains(pat) {
                violations.push(format!("{} 含已禁止反模式 {:?}\n", f.display(), pat));
            }
        }
    }
    assert!(
        violations.is_empty(),
        "SQL 卫生反模式复发（见明细，须按既有修复手法改写）：\n{}",
        violations.join("\n")
    );
}
