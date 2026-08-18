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

    #[tokio::test]
    async fn test_work_list_route_exists() {
        let pool = build_test_pool();
        let app = crate::processplatform_core_entity_router(pool);

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/process/work/list")
                    .method(axum::http::Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_ne!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_work_get_route_exists() {
        let pool = build_test_pool();
        let app = crate::processplatform_core_entity_router(pool);

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/process/work/test-id")
                    .method(axum::http::Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_ne!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_task_list_route_exists() {
        let pool = build_test_pool();
        let app = crate::processplatform_core_entity_router(pool);

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/process/task/list")
                    .method(axum::http::Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_ne!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_task_get_route_exists() {
        let pool = build_test_pool();
        let app = crate::processplatform_core_entity_router(pool);

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/process/task/test-id")
                    .method(axum::http::Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_ne!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_ticket_list_route_exists() {
        let pool = build_test_pool();
        let app = crate::processplatform_core_entity_router(pool);

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/process/ticket/list")
                    .method(axum::http::Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_ne!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_workcompleted_list_route_exists() {
        let pool = build_test_pool();
        let app = crate::processplatform_core_entity_router(pool);

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/process/workcompleted/list")
                    .method(axum::http::Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_ne!(response.status(), StatusCode::NOT_FOUND);
    }

    #[test]
    fn test_work_list_empty_data_action_result_structure() {
        let result: ActionResult<serde_json::Value> = ActionResult::success(serde_json::json!({
            "count": 0,
            "data": []
        }));
        let json = serde_json::to_value(&result).unwrap();
        assert_eq!(json["type"], "success");
        assert_eq!(json["data"]["count"], 0);
        assert_eq!(json["data"]["data"].as_array().unwrap().len(), 0);
    }

    #[tokio::test]
    async fn test_db_error_returns_internal() {
        let mut cfg = deadpool_postgres::tokio_postgres::Config::new();
        cfg.host("invalid-host-that-does-not-exist")
            .port(1)
            .user("invalid")
            .password("invalid")
            .dbname("nonexistent");
        let mgr = Manager::new(cfg, deadpool_postgres::tokio_postgres::NoTls);
        let pool = Pool::builder(mgr).build().unwrap();
        let app = crate::processplatform_core_entity_router(pool);

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/process/work/list")
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
}
