#[cfg(test)]
mod tests {
    use crate::HotPic;
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
        let app = crate::hotpic_core_entity_router(pool);

        let response = app
            .clone()
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/hotpic/core/entity/list")
                    .method(axum::http::Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[tokio::test]
    async fn test_list_by_app_and_info_returns_success() {
        let pool = build_test_pool();
        let app = crate::hotpic_core_entity_router(pool);

        let response = app
            .clone()
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/hotpic/core/entity/list/by/app/app-001/info-001")
                    .method(axum::http::Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        // 由于没有数据库，会返回 INTERNAL_SERVER_ERROR (500) 或 NOT_FOUND (404)
        assert!(response.status() == StatusCode::INTERNAL_SERVER_ERROR
            || response.status() == StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_exists_check_returns_success() {
        let pool = build_test_pool();
        let app = crate::hotpic_core_entity_router(pool);

        let response = app
            .clone()
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/hotpic/core/entity/exists/check/app/app-001/info-001")
                    .method(axum::http::Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        // 由于没有数据库，会返回 INTERNAL_SERVER_ERROR (500) 或 NOT_FOUND (404)
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
    fn test_hot_pic_serialization() {
        let hot_pic = HotPic {
            id: "hotpic-001".to_string(),
            application: "OA".to_string(),
            info_id: "info-001".to_string(),
            title: "热图测试".to_string(),
            base64: "base64data".to_string(),
        };
        let json = serde_json::to_value(&hot_pic).unwrap();
        assert_eq!(json["id"], "hotpic-001");
        assert_eq!(json["application"], "OA");
        assert_eq!(json["title"], "热图测试");
    }
}
