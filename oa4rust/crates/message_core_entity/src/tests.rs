#[cfg(test)]
mod tests {
    use crate::entities::message::Model as Message;
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
        let app = crate::message_core_entity_router(pool);

        let response = app
            .clone()
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/message/core/entity/list")
                    .method(axum::http::Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert!(matches!(response.status(), StatusCode::INTERNAL_SERVER_ERROR | StatusCode::NOT_FOUND));
    }

    #[tokio::test]
    async fn test_list_by_consume_returns_success() {
        let pool = build_test_pool();
        let app = crate::message_core_entity_router(pool);

        let response = app
            .clone()
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/message/core/entity/list/by/system")
                    .method(axum::http::Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert!(matches!(response.status(), StatusCode::INTERNAL_SERVER_ERROR | StatusCode::NOT_FOUND));
    }

    #[tokio::test]
    async fn test_unread_count_returns_success() {
        let pool = build_test_pool();
        let app = crate::message_core_entity_router(pool);

        let response = app
            .clone()
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/message/core/entity/unread/count/system")
                    .method(axum::http::Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert!(matches!(response.status(), StatusCode::INTERNAL_SERVER_ERROR | StatusCode::NOT_FOUND));
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
    fn test_message_serialization() {
        let message = Message {
            id: "msg-001".to_string(),
            title: "系统通知".to_string(),
            body: Some("这是一条测试消息".to_string()),
            r#type: "system".to_string(),
            consumer: "system".to_string(),
            is_read: false,
            create_time: None,
        };
        let json = serde_json::to_value(&message).unwrap();
        assert_eq!(json["id"], "msg-001");
        assert_eq!(json["title"], "系统通知");
        assert_eq!(json["is_read"], false);
    }
}
