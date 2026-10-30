// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

//! 测试模块在册守卫：`src/*tests*.rs` 文件必须在 `lib.rs` 里有对应的 `mod` 声明。
//!
//! # 这个测试防的缺陷
//!
//! 2026-10-29 发现 6 个 crate 的 `src/tests.rs` 文件存在，但 `lib.rs` 只声明了
//! `tests_generated`，从未声明 `mod tests;`：
//!
//! - `console` / `ldap` / `openapi` / `orm`
//! - `processplatform_assemble_surface` / `processplatform_core_express`
//!
//! 后果是文件里的断言**一条都不参与编译**，`cargo test` 仍然是绿的。
//! 「文件在」不等于「测试在跑」；本守卫把这类静默失效变成本机可测。
//!
//! # 判据
//!
//! 对每个 crate 的 `src/` 顶层文件，若文件名满足以下任一形状，则视为测试模块文件：
//!
//! - `tests.rs`
//! - `tests_*.rs`（如 `tests_generated.rs`）
//! - `*_tests.rs`（如 `security_gate_tests.rs`）
//!
//! 该文件名（去扩展名）必须出现在 `lib.rs` 的 `mod <name>;` / `mod <name> {` 中，
//! 或通过 `include!` 引用 `<name>.rs`（`parity` 用的是后一种形态）。
//!
//! 若确有文件不应编入 crate，请删除该文件或把理由写进本守卫的豁免表——
//! 不要留着"看起来有测试"的孤儿文件。

/// 已知有意不编入 crate 的测试模块文件（相对 crate 的 `src/` 路径），附理由。
/// 当前为空：2026-10-29 补齐 6 个 crate 后无遗留。
const EXEMPT: &[&str] = &[];

#[test]
fn every_test_module_file_is_declared_in_lib_rs() {
    use std::path::Path;

    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let crates_dir = manifest_dir.join("crates");
    let mut orphans: Vec<String> = Vec::new();

    for entry in std::fs::read_dir(&crates_dir).expect("读取 crates/ 失败——测试前提不成立")
    {
        let entry = entry.expect("遍历 crates/ 失败");
        if !entry.file_type().expect("stat 失败").is_dir() {
            continue;
        }
        let crate_name = entry.file_name().to_string_lossy().into_owned();
        let src_dir = entry.path().join("src");
        let lib_rs = match std::fs::read_to_string(src_dir.join("lib.rs")) {
            Ok(text) => text,
            Err(_) => continue, // 非 lib crate（或结构不同）不在本守卫范围
        };

        for src_entry in std::fs::read_dir(&src_dir).expect("读取 crate src/ 失败") {
            let src_entry = src_entry.expect("遍历 crate src/ 失败");
            let path = src_entry.path();
            if !path.is_file() || path.extension().is_none_or(|ext| ext != "rs") {
                continue;
            }
            let Some(stem) = path.file_stem().and_then(|s| s.to_str()) else {
                continue;
            };
            let is_test_module =
                stem == "tests" || stem.starts_with("tests_") || stem.ends_with("_tests");
            if !is_test_module {
                continue;
            }

            let relative = format!("{crate_name}/src/{stem}.rs");
            if EXEMPT.contains(&relative.as_str()) {
                continue;
            }

            let declared = lib_rs.contains(&format!("mod {stem};"))
                || lib_rs.contains(&format!("mod {stem} {{"))
                || lib_rs.contains(&format!("mod {stem}{{"))
                || lib_rs.contains(&format!("{stem}.rs\""));
            if !declared {
                orphans.push(relative);
            }
        }
    }
    orphans.sort();

    assert!(
        orphans.is_empty(),
        "以下测试模块文件存在于 src/，但 lib.rs 没有 `mod` 声明或 include!（文件在 ≠ 测试在跑）：{orphans:?}\n\
         \n\
         修复：在 crate 的 src/lib.rs 加 `#[cfg(test)] mod <name>;`；\n\
         若文件本就不该编译，删除它，或在 EXEMPT 登记并写明理由。\n\
         参考：2026-10-29 补齐 console/ldap/openapi/orm/processplatform_* 六个 crate 的提交。"
    );
}
