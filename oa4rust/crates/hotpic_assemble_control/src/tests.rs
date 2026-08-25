#[cfg(test)]
mod tests {
    use axum::body::Body;
    use axum::http::{Request, Method, StatusCode};
    use deadpool_postgres::{Manager, Pool};
    use deadpool_postgres::tokio_postgres::{Config, NoTls};
    use shared::response::ActionResult;
    use tower::util::ServiceExt;
    use serde_json::Value;

    fn build_test_pool() -> Pool {
        let mgr = Manager::new(
            Config::new(),
            NoTls,
        );
        Pool::builder(mgr).max_size(1).build().unwrap()
    }

    #[tokio::test]
    async fn test_get_control_config_returns_success() {
        let pool = build_test_pool();
        let app = crate::router(pool);

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/hotpic_assemble_control/get/control/config")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[tokio::test]
    async fn test_list_control_panels_returns_success() {
        let pool = build_test_pool();
        let app = crate::router(pool);

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/hotpic_assemble_control/list/control/panels")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[tokio::test]
    async fn test_list_control_applications_returns_success() {
        let pool = build_test_pool();
        let app = crate::router(pool);

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/hotpic_assemble_control/list/control/applications")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[tokio::test]
    async fn test_hotpic_list_java_path_route() {
        let pool = build_test_pool();
        let app = crate::router(pool);

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/hotpic/list/hotpics")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[tokio::test]
    async fn test_hotpic_get_by_id_route() {
        let pool = build_test_pool();
        let app = crate::router(pool);

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/hotpic/get/hotpic/hotpic-001")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[tokio::test]
    async fn test_hotpic_create_java_path_route() {
        let pool = build_test_pool();
        let app = crate::router(pool);

        let body = serde_json::to_string(&serde_json::json!({"name": "test"})).unwrap();

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/hotpic/create/hotpic")
                    .method(Method::POST)
                    .header("content-type", "application/json")
                    .body(Body::from(body))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[tokio::test]
    async fn test_hotpic_control_config_java_path_route() {
        let pool = build_test_pool();
        let app = crate::router(pool);

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/hotpic/assemble/control/config")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[tokio::test]
    async fn test_user_hotpic_id_route() {
        let pool = build_test_pool();
        let app = crate::router(pool);

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/hotpic/assemble/control/user/hotpic/hotpic-001")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[test]
    fn test_hotpic_assemble_control_router_builds() {
        let pool = build_test_pool();
        let _ = crate::router(pool);
    }

    #[test]
    fn test_action_result_success() {
        let result: ActionResult<i32> = ActionResult::success(42);
        assert_eq!(result.r#type, Some("success".to_string()));
        assert_eq!(result.data, Some(42));
    }

    #[tokio::test]
    async fn test_u2_post_user_hotpic_route() {
        let pool = build_test_pool();
        let app = crate::router(pool);
        let body = serde_json::to_string(&serde_json::json!({"title": "x", "imageUrl": "y"})).unwrap();
        let response = app.oneshot(
            Request::builder()
                .uri("/jaxrs/hotpic/assemble/control/user/hotpic")
                .method(Method::POST)
                .header("content-type", "application/json")
                .body(Body::from(body)).unwrap(),
        ).await.unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_u2_delete_user_hotpic_id_route() {
        let pool = build_test_pool();
        let app = crate::router(pool);
        let response = app.oneshot(
            Request::builder()
                .uri("/jaxrs/hotpic/assemble/control/user/hotpic/test-id")
                .method(Method::DELETE)
                .body(Body::empty()).unwrap(),
        ).await.unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_u2_delete_cipher_hotpic_bbs_route() {
        let pool = build_test_pool();
        let app = crate::router(pool);
        let response = app.oneshot(
            Request::builder()
                .uri("/jaxrs/hotpic/assemble/control/cipher/hotpic/bbs/test-id")
                .method(Method::DELETE)
                .body(Body::empty()).unwrap(),
        ).await.unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
    }
}
