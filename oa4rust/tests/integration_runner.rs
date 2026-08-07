//! OA4Rust 集成测试运行器 (U8)
//!
//! 这是集成测试的入口。完整流程应当：构建全量 Router（`src/main.rs` 的
//! `create_app`）并用 HTTP 客户端发起请求、断言 `/health` 与受保护端点的行为。
//!
//! 由于本仓库的集成测试依赖真实 PostgreSQL，当 `DATABASE_URL` 未设置时，所有需要
//! 连库的用例通过环境变量守卫提前返回，保证 `cargo test` 在无库环境也能通过
//! （满足 R33 "数据库不可用时集成测试跳过而非 panic" 的要求）。
//!
//! 运行真实集成测试：
//! ```bash
//! DATABASE_URL=postgres://o2server:password@localhost:5432/oa4rust \
//!   cargo test --test integration_runner -- --ignored
//! ```

#[ignore = "requires DATABASE_URL and a running PostgreSQL"]
#[tokio::test]
async fn app_boots_and_health_ok() {
    if std::env::var("DATABASE_URL").is_err() {
        eprintln!("DATABASE_URL 未设置，跳过集成冒烟测试");
        return;
    }
    // 真实实现依赖 binary crate 的 create_app（位于 src/main.rs）。
    // 待 lib crate 暴露 create_app 后，可在此构造 Router 并断言 /health 返回 200。
    unimplemented!("enable once create_app is exposed via the lib crate and DATABASE_URL is set");
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
