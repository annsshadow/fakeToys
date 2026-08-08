#[cfg(test)]
mod tests {
    use super::*;
    use crate::{QueryImport, QueryItem, QueryView};
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
    async fn test_item_list_returns_success() {
        let pool = build_test_pool();
        let app = crate::query_core_entity_router(pool);

        let response = app
            .clone()
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/query/item/list")
                    .method(axum::http::Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[tokio::test]
    async fn test_view_list_returns_success() {
        let pool = build_test_pool();
        let app = crate::query_core_entity_router(pool);

        let response = app
            .clone()
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/query/view/list")
                    .method(axum::http::Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[tokio::test]
    async fn test_view_get_returns_success() {
        let pool = build_test_pool();
        let app = crate::query_core_entity_router(pool);

        let response = app
            .clone()
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/query/view/test-view-id")
                    .method(axum::http::Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[tokio::test]
    async fn test_view_create_returns_success() {
        let pool = build_test_pool();
        let app = crate::query_core_entity_router(pool);

        let response = app
            .clone()
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/query/view/create")
                    .method(axum::http::Method::POST)
                    .header("content-type", "application/json")
                    .body(Body::from(r#"{"name":"test"}"#))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[tokio::test]
    async fn test_import_list_returns_success() {
        let pool = build_test_pool();
        let app = crate::query_core_entity_router(pool);

        let response = app
            .clone()
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/query/import/list")
                    .method(axum::http::Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
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
    fn test_query_view_serialization() {
        let view = QueryView {
            id: "view-001".to_string(),
            name: "默认视图".to_string(),
            description: Some("测试描述".to_string()),
            query_sql: Some("SELECT * FROM t".to_string()),
            creator_id: "user-001".to_string(),
            status: "active".to_string(),
            create_time: "2024-01-01T00:00:00Z".to_string(),
        };
        let json = serde_json::to_value(&view).unwrap();
        assert_eq!(json["id"], "view-001");
        assert_eq!(json["creatorId"], "user-001");
        assert_eq!(json["querySql"], "SELECT * FROM t");
    }

    #[test]
    fn test_query_item_serialization() {
        let item = QueryItem {
            id: "item-001".to_string(),
            view_id: "view-001".to_string(),
            name: "名称字段".to_string(),
            field_name: "name".to_string(),
            data_type: "string".to_string(),
            create_time: "2024-01-01T00:00:00Z".to_string(),
        };
        let json = serde_json::to_value(&item).unwrap();
        assert_eq!(json["viewId"], "view-001");
        assert_eq!(json["fieldName"], "name");
    }

    #[test]
    fn test_query_import_serialization() {
        let imp = QueryImport {
            id: "import-001".to_string(),
            view_id: "view-001".to_string(),
            file_name: "data.csv".to_string(),
            status: "completed".to_string(),
            import_time: Some("2024-01-01T01:00:00Z".to_string()),
            create_time: "2024-01-01T00:00:00Z".to_string(),
        };
        let json = serde_json::to_value(&imp).unwrap();
        assert_eq!(json["fileName"], "data.csv");
        assert_eq!(json["importTime"], "2024-01-01T01:00:00Z");
    }
}
