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

use integration_tests::db::init_test_database;

/// 集成测试主入口。
///
/// 使用 `#[ignore]` 标记，避免在无 PostgreSQL 环境的 `cargo test` 中运行。
/// 通过 `--ignored` 显式启用。
///
/// 初始化流程：
/// 1. 同步初始化一次性测试数据库（`oa4rust_test_<pid>`）
/// 2. 执行所有迁移
/// 3. 注入测试数据（admin 用户 + 会话）
/// 4. 按顺序运行每个 cross-crate 场景
#[ignore = "requires a running database server"]
#[test]
fn integration_scenarios() {
    let _ctx = init_test_database();

    // Run all scenario tests sequentially to avoid port conflicts
    // (each scenario spins up its own HTTP server on a random port)
    // Note: each scenario is its own #[tokio::test] — call them directly
    // without .await (they manage their own runtime via the test macro).

    integration_tests::scenarios::org_person_meeting::org_person_meeting_flow();
    integration_tests::scenarios::bbs_correlation::bbs_correlation_flow();
    integration_tests::scenarios::file_upload::file_upload_flow();
    integration_tests::scenarios::program_center_core_entity::program_center_core_entity_application_flow();
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
