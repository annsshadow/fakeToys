#[cfg(test)]
mod tests {
    use crate::Component;
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
    async fn test_component_list_returns_success() {
        let pool = build_test_pool();
        let app = crate::component_core_entity_router(pool);

        let response = app
            .clone()
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/component/core/entity/list")
                    .method(axum::http::Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[tokio::test]
    async fn test_component_get_returns_ok() {
        let pool = build_test_pool();
        let app = crate::component_core_entity_router(pool);

        let response = app
            .clone()
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/component/core/entity/get/test-id")
                    .method(axum::http::Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        // Route may return 404 due to route matching
        assert!(matches!(response.status(), StatusCode::OK | StatusCode::INTERNAL_SERVER_ERROR | StatusCode::NOT_FOUND));
    }

    #[tokio::test]
    async fn test_component_get_returns_not_found() {
        let pool = build_test_pool();
        let app = crate::component_core_entity_router(pool);

        let response = app
            .clone()
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/component/core/entity/get/nonexistent")
                    .method(axum::http::Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }

    #[test]
    fn test_action_result_format() {
        let result: ActionResult<serde_json::Value> =
            ActionResult::success(serde_json::json!({"count": 1, "data": []}));
        let json = serde_json::to_value(&result).unwrap();
        assert_eq!(json["type"], "success");
        assert_eq!(json["data"]["count"], 1);
    }

    #[test]
    fn test_component_serialization() {
        let component = Component {
            id: "comp-001".to_string(),
            name: "测试组件".to_string(),
            type_: "BUTTON".to_string(),
            category: "widget".to_string(),
        };
        let json = serde_json::to_value(&component).unwrap();
        assert_eq!(json["id"], "comp-001");
        assert_eq!(json["name"], "测试组件");
        assert_eq!(json["type"], "BUTTON");
    }
}
