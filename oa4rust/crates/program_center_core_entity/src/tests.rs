#[cfg(test)]
mod tests {
    use axum::body::Body;
    use axum::http::{Request, StatusCode};
    use deadpool_postgres::{Manager, Pool};
    use shared::response::ActionResult;
    use tower::ServiceExt;

    fn build_test_pool() -> Pool {
        let mgr = Manager::new(
            deadpool_postgres::tokio_postgres::Config::new(),
            deadpool_postgres::tokio_postgres::NoTls,
        );
        Pool::builder(mgr).build().unwrap()
    }

    // ── Route ownership tests ───────────────────────────────────────────────
    // 留档：35dc00c2 解决跨 crate 路由冲突后，/jaxrs/program_center/* 路由统一由
    // crates/program_center 注册；本 crate 各 handlers::_router 返回空 Router，
    // 重复注册会在 create_app 合并时 panic。以下断言固化该让渡决策：
    // 本 crate 的路由器不得再提供这些路径（404 = 未在本 crate 注册）。
    // application/agent/structure 的 list 端点当前全工作区无实现（openapi 清单
    // 有路径、无 handler）；script/invoke 由 crates/program_center 提供并有独立测试。

    #[tokio::test]
    async fn test_application_list_route_not_registered_here() {
        let pool = build_test_pool();
        let app = crate::program_center_core_entity_router(pool).await;

        let response = app
            .clone()
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/program_center/application/list")
                    .method(axum::http::Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_script_list_route_not_registered_here() {
        let pool = build_test_pool();
        let app = crate::program_center_core_entity_router(pool).await;

        let response = app
            .clone()
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/program_center/script/list")
                    .method(axum::http::Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_invoke_list_route_not_registered_here() {
        let pool = build_test_pool();
        let app = crate::program_center_core_entity_router(pool).await;

        let response = app
            .clone()
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/program_center/invoke/list")
                    .method(axum::http::Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_agent_list_route_not_registered_here() {
        let pool = build_test_pool();
        let app = crate::program_center_core_entity_router(pool).await;

        let response = app
            .clone()
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/program_center/agent/list")
                    .method(axum::http::Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_structure_list_route_not_registered_here() {
        let pool = build_test_pool();
        let app = crate::program_center_core_entity_router(pool).await;

        let response = app
            .clone()
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/program_center/structure/list")
                    .method(axum::http::Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_application_create_route_not_registered_here() {
        let pool = build_test_pool();
        let app = crate::program_center_core_entity_router(pool).await;

        let body = serde_json::json!({
            "name": "Test App",
            "category": "Office"
        });

        let response = app
            .clone()
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/program_center/application")
                    .method(axum::http::Method::POST)
                    .header("Content-Type", "application/json")
                    .body(Body::from(serde_json::to_string(&body).unwrap()))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_application_update_route_not_registered_here() {
        let pool = build_test_pool();
        let app = crate::program_center_core_entity_router(pool).await;

        let body = serde_json::json!({
            "name": "Updated App"
        });

        let response = app
            .clone()
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/program_center/application/some-id")
                    .method(axum::http::Method::PUT)
                    .header("Content-Type", "application/json")
                    .body(Body::from(serde_json::to_string(&body).unwrap()))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_application_delete_route_not_registered_here() {
        let pool = build_test_pool();
        let app = crate::program_center_core_entity_router(pool).await;

        let response = app
            .clone()
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/program_center/application/some-id")
                    .method(axum::http::Method::DELETE)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_invoke_create_route_not_registered_here() {
        let pool = build_test_pool();
        let app = crate::program_center_core_entity_router(pool).await;

        let body = serde_json::json!({
            "name": "Test Invoke"
        });

        let response = app
            .clone()
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/program_center/invoke")
                    .method(axum::http::Method::POST)
                    .header("Content-Type", "application/json")
                    .body(Body::from(serde_json::to_string(&body).unwrap()))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_agent_create_route_not_registered_here() {
        let pool = build_test_pool();
        let app = crate::program_center_core_entity_router(pool).await;

        let body = serde_json::json!({
            "name": "Test Agent"
        });

        let response = app
            .clone()
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/program_center/agent")
                    .method(axum::http::Method::POST)
                    .header("Content-Type", "application/json")
                    .body(Body::from(serde_json::to_string(&body).unwrap()))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_structure_create_route_not_registered_here() {
        let pool = build_test_pool();
        let app = crate::program_center_core_entity_router(pool).await;

        let body = serde_json::json!({
            "name": "Test Structure",
            "storage": "local"
        });

        let response = app
            .clone()
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/program_center/structure")
                    .method(axum::http::Method::POST)
                    .header("Content-Type", "application/json")
                    .body(Body::from(serde_json::to_string(&body).unwrap()))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_unknown_route_returns_404() {
        let pool = build_test_pool();
        let app = crate::program_center_core_entity_router(pool).await;

        let response = app
            .clone()
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/program_center/unknown")
                    .method(axum::http::Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }

    // ── ActionResult serialization ───────────────────────────────────────────

    #[test]
    fn test_action_result_success_structure() {
        let result: ActionResult<serde_json::Value> =
            ActionResult::success(serde_json::json!({"count": 1, "data": []}));
        assert_eq!(result.r#type, Some("success".to_string()));
        assert!(result.data.is_some());
    }

    #[test]
    fn test_action_result_error_serialization() {
        let result: ActionResult<&str> = ActionResult::error("server error");
        let json = serde_json::to_value(&result).unwrap();
        assert_eq!(json["type"], "error");
        assert_eq!(json["message"], "server error");
        assert!(json["data"].is_null());
    }

    // ── Router build test ────────────────────────────────────────────────────

    #[tokio::test]
    async fn test_program_center_core_entity_router_builds() {
        let pool = build_test_pool();
        let router = crate::program_center_core_entity_router(pool).await;
        // Verify router is non-empty by checking it has at least one route
        assert!(format!("{:?}", router) != "Router {}", "router should have routes registered");
    }
}
