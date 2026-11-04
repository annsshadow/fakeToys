// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

//! OA4Rust 集成测试运行器 (U6)
//!
//! 入口测试：初始化一次性测试数据库，然后按场景顺序执行所有集成测试。
//!
//! 运行方式：
//! ```bash
//! cargo test --test integration_runner -- --ignored
//! ```
//!
//! 环境变量：
//! - `DATABASE_URL` — 管理员连接 URL（默认 `postgres://o2server:password@localhost:5432/postgres`）
//! - `SESSION_HMAC_SECRET` — 可选，会话 token HMAC 密钥（未设置时不签名）

mod integration_tests;

use integration_tests::db::init_test_database_async;
use integration_tests::db_isolated::init_isolated;

/// 集成测试主入口。
///
/// 使用 `#[ignore]` 标记，避免在无数据库环境的 `cargo test` 中运行。
/// 通过 `--ignored` 显式启用。
///
/// 初始化流程：
/// 1. 同步初始化一次性测试数据库（`oa4rust_test_<pid>`）
/// 2. 执行所有迁移
/// 3. 注入测试数据（admin 用户 + 会话）
/// 4. 按顺序运行每个 cross-crate 场景
#[ignore = "requires a running database server"]
#[tokio::test]
async fn integration_scenarios() {
    use integration_tests::db_isolated::init_isolated;
    let _ctx = init_isolated().await;

    integration_tests::scenarios::org_person_meeting::org_person_meeting_flow().await;
    integration_tests::scenarios::bbs_correlation::bbs_correlation_flow().await;
    integration_tests::scenarios::file_upload::file_upload_flow().await;
    integration_tests::scenarios::program_center_core_entity::program_center_core_entity_application_flow().await;
}

/// 数据完整性场景入口 (U9: 并发/数据完整性)。
///
/// 独立于主流程，便于通过 `cargo test --test integration_runner data_integrity`
/// 单独过滤执行；无数据库环境时随 `#[ignore]` 跳过。
#[ignore = "requires a running database server"]
#[tokio::test]
async fn data_integrity_scenarios() {
    let _ctx = init_test_database_async().await;

    integration_tests::scenarios::data_integrity::concurrent_document_updates().await;
    integration_tests::scenarios::data_integrity::soft_delete_isolation().await;
}

#[ignore = "requires a running database server"]
#[tokio::test]
async fn pagination_contract() {
    let _ctx = init_test_database_async().await;
    integration_tests::scenarios::pagination_contract::pagination_contract_flow().await;
}

/// O24：`organization/core/express` 的 200/500/503 三条行为级用例。
#[ignore = "requires a running database server"]
#[tokio::test]
async fn organization_express_config_contract() {
    let ctx = init_isolated().await;
    integration_tests::scenarios::organization_core_express::organization_express_config_contract_flow(
        ctx.pool.clone(),
    )
    .await;
}

/// O24-3 批次 5：draft/start 写 xstatus='started'（GET/POST 共用 handler）。
#[ignore = "requires a running database server"]
#[tokio::test]
async fn draft_start_status_contract() {
    let ctx = init_isolated().await;
    integration_tests::scenarios::draft_start_status::draft_start_status_flow(ctx.pool.clone())
        .await;
}

/// O24-3 批次 9：E42703 属性表路径（personattribute / unitattribute 无 name 列）。
#[ignore = "requires a running database server"]
#[tokio::test]
async fn attribute_paths_contract() {
    let ctx = init_isolated().await;
    integration_tests::scenarios::attribute_paths::attribute_paths_flow(ctx.pool.clone()).await;
}

/// O24-3 批次 11：E42601 document filter 纯 tail 拼进 WHERE。
#[ignore = "requires a running database server"]
#[tokio::test]
async fn document_filter_paths_contract() {
    let ctx = init_isolated().await;
    integration_tests::scenarios::document_filter_paths::document_filter_paths_flow(
        ctx.pool.clone(),
    )
    .await;
}

/// O24-3 批次 10：E42804 脚本软删 / mergeitem 类型不匹配。
#[ignore = "requires a running database server"]
#[tokio::test]
async fn script_delete_paths_contract() {
    let ctx = init_isolated().await;
    integration_tests::scenarios::script_delete_paths::script_delete_paths_flow(ctx.pool.clone())
        .await;
}

/// O24-3 批次 8：E42P10 upsert 路径（attendance statistic/do、calendar control config）。
#[ignore = "requires a running database server"]
#[tokio::test]
async fn upsert_paths_contract() {
    let ctx = init_isolated().await;
    integration_tests::scenarios::upsert_paths::upsert_paths_flow(ctx.pool.clone()).await;
}

/// O24-3 批次 7：wrong-type 配置写路径（bbs max_forum_count、general 文本布尔列）。
#[ignore = "requires a running database server"]
#[tokio::test]
async fn wrong_type_config_contract() {
    let ctx = init_isolated().await;
    integration_tests::scenarios::wrong_type_config::wrong_type_config_flow(ctx.pool.clone()).await;
}

/// O24-3 批次 6：E23502 实体引用必填（hotpic application/info_id、portal page portal_id）。
#[ignore = "requires a running database server"]
#[tokio::test]
async fn entity_reference_required_contract() {
    let ctx = init_isolated().await;
    integration_tests::scenarios::entity_reference_required::entity_reference_required_flow(
        ctx.pool.clone(),
    )
    .await;
}

/// O24-3 批次 4：INSERT 未写必填主键/规范列的遗留给定族（3 条 E23502）。
#[ignore = "requires a running database server"]
#[tokio::test]
async fn insert_required_id_contract() {
    let ctx = init_isolated().await;
    integration_tests::scenarios::insert_required_id::insert_required_id_flow(ctx.pool.clone())
        .await;
}

/// O24-3 批次 3：查询设计器 permission 的 `$1` 复用冲突（4 条基线）。
#[ignore = "requires a running database server"]
#[tokio::test]
async fn query_permission_binding_contract() {
    let ctx = init_isolated().await;
    integration_tests::scenarios::query_permission_binding::query_permission_binding_flow(
        ctx.pool.clone(),
    )
    .await;
}

/// O24-3 批次 2：JSONB 列按 String 绑定的同根因族（4 条基线）。
#[ignore = "requires a running database server"]
#[tokio::test]
async fn jsonb_param_binding_contract() {
    let ctx = init_isolated().await;
    integration_tests::scenarios::jsonb_param_binding::jsonb_param_binding_flow(ctx.pool.clone())
        .await;
}

/// O24-3 批次 1：门户表面发布「写库后 500」的行为级用例。
#[ignore = "requires a running database server"]
#[tokio::test]
async fn portal_surface_publish_contract() {
    let ctx = init_isolated().await;
    integration_tests::scenarios::portal_surface_publish::portal_surface_publish_flow(
        ctx.pool.clone(),
    )
    .await;
}

#[cfg(test)]
mod contract {
    use shared::response::ActionResult;

    #[test]
    fn action_result_shape_is_stable() {
        let s = ActionResult::success("ok");
        assert_eq!(s.r#type.as_deref(), Some("success"));
        assert!(s.data.is_some());

        let e: ActionResult<serde_json::Value> = ActionResult::error("boom");
        assert_eq!(e.r#type.as_deref(), Some("error"));
        assert!(e.data.is_none());
    }
}
