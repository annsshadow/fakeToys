#[cfg(test)]
mod tests {
    use super::*;
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

    #[tokio::test]
    async fn test_application_list_returns_success() {
        let pool = build_test_pool();
        let app = program_center_core_entity_router(pool);

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

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_script_list_returns_success() {
        let pool = build_test_pool();
        let app = program_center_core_entity_router(pool);

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

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_invoke_list_returns_success() {
        let pool = build_test_pool();
        let app = program_center_core_entity_router(pool);

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

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_agent_list_returns_success() {
        let pool = build_test_pool();
        let app = program_center_core_entity_router(pool);

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

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_structure_list_returns_success() {
        let pool = build_test_pool();
        let app = program_center_core_entity_router(pool);

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

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[test]
    fn test_action_result_success_structure() {
        let result: ActionResult<serde_json::Value> =
            ActionResult::success(serde_json::json!({"count": 1, "data": []}));
        assert_eq!(result.r#type, Some("success".to_string()));
        assert!(result.data.is_some());
    }

    #[test]
    fn test_application_list_result_format() {
        let result: ActionResult<serde_json::Value> = ActionResult::success(serde_json::json!({
            "count": 3,
            "data": [
                {"id": "1", "name": "测试应用", "category": "办公"}
            ]
        }));
        let json = serde_json::to_value(&result).unwrap();
        assert_eq!(json["type"], "success");
        assert_eq!(json["data"]["count"], 3);
    }
}
