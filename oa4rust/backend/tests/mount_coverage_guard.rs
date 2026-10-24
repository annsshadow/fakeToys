// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

//! 挂载覆盖守卫：create_app 必须 merge 所有公开 `router()` 的 crate。
//!
//! # 这个测试防的缺陷
//!
//! 契约 fixture（`oa4rust/tests/contracts/backend-registered-routes.fixture.ts`）
//! 由各 crate 的 `.route()` 字面量生成——它只证明「路由**已注册**」，
//! 不证明「路由**已挂载**」。2026-10-24 发现 4 个 crate 的 router 早已实现
//! 且被前端实际调用，却从未 merge 进 create_app：
//!
//! - `empower`（授权委托 CRUD 14 条，Personal.vue 全族调用）
//! - `organization_assemble_personal`（个人中心设置/角色，Personal.vue:659）
//! - `organization_assemble_authentication`（组织目录认证流 53 条）
//! - `base_core_project`（ServerApp 状态页/CMS 缓存刷新）
//!
//! 生产环境全部 404，而所有测试全绿。本测试把「已注册 ≠ 已挂载」这类
//! 缺陷变成本机可测：差集非空即红。
//!
//! 豁免（确有意不挂载）须在 `EXEMPT` 登记并写明理由——空名单是默认状态。

#[test]
fn every_crate_with_public_router_is_merged_into_create_app() {
    use std::collections::HashSet;
    use std::path::Path;

    /// 确有意不挂载的 crate（登记理由）。当前为空：2026-10-24 补齐后无遗留。
    const EXEMPT: &[&str] = &[];

    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let lib_rs = std::fs::read_to_string(manifest_dir.join("src/lib.rs"))
        .expect("读取 src/lib.rs 失败——测试前提不成立");

    // create_app 的 merge 目标 crate 名集合：.merge(<ident>::...
    let mut merged: HashSet<String> = HashSet::new();
    let mut rest = lib_rs.as_str();
    while let Some(i) = rest.find(".merge(") {
        rest = &rest[i + ".merge(".len()..];
        let ident: String = rest
            .chars()
            .take_while(|c| c.is_alphanumeric() || *c == '_')
            .collect();
        if !ident.is_empty() {
            merged.insert(ident);
        }
    }

    // 扫 crates/*/src 下**非测试**源码里公开 router() 的 crate。
    let crates_dir = manifest_dir.join("crates");
    let mut unmounted: Vec<String> = Vec::new();
    for entry in std::fs::read_dir(&crates_dir).expect("读取 crates/ 失败") {
        let entry = entry.expect("遍历 crates/ 失败");
        if !entry.file_type().expect("stat 失败").is_dir() {
            continue;
        }
        let name = entry.file_name().to_string_lossy().into_owned();
        if merged.contains(&name) || EXEMPT.contains(&name.as_str()) {
            continue;
        }
        if crate_defines_public_router(&entry.path()) {
            unmounted.push(name);
        }
    }
    unmounted.sort();

    assert!(
        unmounted.is_empty(),
        "以下 crate 公开了 router() 但未 merge 进 create_app（生产 404 风险）：{unmounted:?}\n\
         \n\
         修复：在 src/lib.rs 的 create_app 补 .merge(<crate>::router(...))；\n\
         若确有意不挂载，在 EXEMPT 登记并写明理由。\n\
         参考：2026-10-24 一次性补齐 4 个漏接线 crate 的提交。"
    );
}

/// crate 目录下是否存在非测试源码公开 `pub [async] fn router(`。
fn crate_defines_public_router(crate_dir: &std::path::Path) -> bool {
    fn walk(dir: &std::path::Path) -> bool {
        let Ok(entries) = std::fs::read_dir(dir) else {
            return false;
        };
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                if walk(&path) {
                    return true;
                }
            } else if path.extension().is_some_and(|e| e == "rs") {
                // 跳过测试文件：其中的 router 多为 mock，不进生产路由表。
                let is_test_file = path
                    .file_name()
                    .map(|n| n.to_string_lossy().contains("test"))
                    .unwrap_or(false);
                if is_test_file {
                    continue;
                }
                if let Ok(text) = std::fs::read_to_string(&path) {
                    if text.contains("pub fn router(") || text.contains("pub async fn router(") {
                        return true;
                    }
                }
            }
        }
        false
    }
    walk(crate_dir)
}
