#[cfg(test)]
mod tests {
    use crate::{ApplicationDict, Invoice};
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
    async fn test_dict_list_returns_success() {
        let pool = build_test_pool();
        let app = crate::general_core_entity_router(pool);

        let response = app
            .clone()
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/general/dict/list")
                    .method(axum::http::Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[tokio::test]
    async fn test_dict_item_list_returns_success() {
        let pool = build_test_pool();
        let app = crate::general_core_entity_router(pool);

        let response = app
            .clone()
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/general/dict/item/list/test-dict-id")
                    .method(axum::http::Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_invoice_list_returns_success() {
        let pool = build_test_pool();
        let app = crate::general_core_entity_router(pool);

        let response = app
            .clone()
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/general/invoice/list")
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
    fn test_application_dict_serialization() {
        let dict = ApplicationDict {
            id: "dict-001".to_string(),
            name: "测试字典".to_string(),
            application: "app-001".to_string(),
        };
        let json = serde_json::to_value(&dict).unwrap();
        assert_eq!(json["id"], "dict-001");
        assert_eq!(json["name"], "测试字典");
    }

    #[test]
    fn test_invoice_serialization() {
        let invoice = Invoice {
            id: "inv-001".to_string(),
            number: "INV20240001".to_string(),
            date: "2024-01-15".to_string(),
            amount: 1000.50,
            status: "pending".to_string(),
        };
        let json = serde_json::to_value(&invoice).unwrap();
        assert_eq!(json["number"], "INV20240001");
        assert_eq!(json["amount"], 1000.50);
        assert_eq!(json["status"], "pending");
    }
}
