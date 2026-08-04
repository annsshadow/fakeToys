#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Item, View};
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

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_item_access_list_returns_success() {
        let pool = build_test_pool();
        let app = crate::query_core_entity_router(pool);

        let response = app
            .clone()
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/query/item/access/list/test-item-id")
                    .method(axum::http::Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
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

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_import_model_list_returns_success() {
        let pool = build_test_pool();
        let app = crate::query_core_entity_router(pool);

        let response = app
            .clone()
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/query/import/model/list")
                    .method(axum::http::Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_import_record_list_returns_success() {
        let pool = build_test_pool();
        let app = crate::query_core_entity_router(pool);

        let response = app
            .clone()
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/query/import/record/list")
                    .method(axum::http::Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
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
    fn test_item_serialization() {
        let item = Item {
            id: "item-001".to_string(),
            name: "测试查询".to_string(),
            application: "app-001".to_string(),
            item_access: None,
        };
        let json = serde_json::to_value(&item).unwrap();
        assert_eq!(json["id"], "item-001");
        assert_eq!(json["name"], "测试查询");
    }

    #[test]
    fn test_view_serialization() {
        let view = View {
            id: "view-001".to_string(),
            name: "默认视图".to_string(),
            item_id: "item-001".to_string(),
            view_type: "list".to_string(),
        };
        let json = serde_json::to_value(&view).unwrap();
        assert_eq!(json["id"], "view-001");
        assert_eq!(json["viewType"], "list");
    }
}
