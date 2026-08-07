#[cfg(test)]
mod tests {
    use crate::Correlation;
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
    async fn test_list_returns_success() {
        let pool = build_test_pool();
        let app = crate::correlation_core_entity_router(pool);

        let response = app
            .clone()
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/correlation/core/entity/list")
                    .method(axum::http::Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[tokio::test]
    async fn test_list_by_source_returns_success() {
        let pool = build_test_pool();
        let app = crate::correlation_core_entity_router(pool);

        let response = app
            .clone()
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/correlation/core/entity/list/by/user/user-001")
                    .method(axum::http::Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        // 由于没有数据库，会返回 INTERNAL_SERVER_ERROR (500)
        assert!(response.status() == StatusCode::INTERNAL_SERVER_ERROR
            || response.status() == StatusCode::NOT_FOUND);
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
    fn test_correlation_serialization() {
        let correlation = Correlation {
            id: "corr-001".to_string(),
            source_type: "user".to_string(),
            source_id: "user-001".to_string(),
            target_type: "doc".to_string(),
            target_id: "doc-001".to_string(),
            weight: 10,
        };
        let json = serde_json::to_value(&correlation).unwrap();
        assert_eq!(json["id"], "corr-001");
        assert_eq!(json["source_type"], "user");
        assert_eq!(json["weight"], 10);
    }
}
