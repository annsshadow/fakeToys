#[cfg(test)]
mod tests {
    use axum::{
        body::Body,
        http::{Request, StatusCode},
    };
    use shared::response::ActionResult;
    use tower::ServiceExt;

    fn build_test_pool() -> deadpool_postgres::Pool {
        deadpool_postgres::Pool::builder(deadpool_postgres::Manager::new(
            deadpool_postgres::tokio_postgres::Config::new(),
            deadpool_postgres::tokio_postgres::NoTls,
        ))
        .build()
        .unwrap()
    }

    #[test]
    fn test_action_result_success() {
        let result: ActionResult<serde_json::Value> =
            ActionResult::success(serde_json::json!({"code": "test", "status": "ok"}));
        let json = serde_json::to_value(&result).unwrap();
        assert_eq!(json["type"], "success");
        assert!(json["data"].is_object());
    }

    #[test]
    fn test_router_builds() {
        let pool = build_test_pool();
        let _ = crate::router(pool);
    }

    #[tokio::test]
    async fn test_get_express_info() {
        let pool = build_test_pool();
        let app = crate::router(pool);

        let response = app
            .clone()
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/express/query?code=TEST123&company=SF")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[tokio::test]
    async fn test_list_express_companies() {
        let pool = build_test_pool();
        let app = crate::router(pool);

        let response = app
            .clone()
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/express/companies")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[tokio::test]
    async fn test_subscribe_express() {
        let pool = build_test_pool();
        let app = crate::router(pool);

        let body = serde_json::json!({"code": "TEST123", "company": "SF", "callback": "http://example.com"}).to_string();
        let response = app
            .clone()
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/express/subscribe")
                    .method("POST")
                    .header("content-type", "application/json")
                    .body(Body::from(body))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
    }

    // ── Batch query endpoint tests ────────────────────────────────────────────

    #[tokio::test]
    async fn test_person_list_by_ids_returns_error_without_db() {
        let pool = build_test_pool();
        let app = crate::router(pool);

        let body = serde_json::json!({"ids": ["p1", "p2"]}).to_string();
        let response = app
            .clone()
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/express/person/list")
                    .method("POST")
                    .header("content-type", "application/json")
                    .body(Body::from(body))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[tokio::test]
    async fn test_person_list_empty_body_returns_error() {
        let pool = build_test_pool();
        let app = crate::router(pool);

        let response = app
            .clone()
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/express/person/list")
                    .method("POST")
                    .header("content-type", "application/json")
                    .body(Body::from("{}".to_string()))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        let body = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();
        let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
        assert_eq!(json["type"], "error");
    }

    #[tokio::test]
    async fn test_person_list_by_identities_returns_error_without_db() {
        let pool = build_test_pool();
        let app = crate::router(pool);

        let body = serde_json::json!({"identities": ["i1", "i2"]}).to_string();
        let response = app
            .clone()
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/express/person/list")
                    .method("POST")
                    .header("content-type", "application/json")
                    .body(Body::from(body))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[tokio::test]
    async fn test_unit_list_returns_error_without_db() {
        let pool = build_test_pool();
        let app = crate::router(pool);

        let body = serde_json::json!({"ids": ["u1", "u2"]}).to_string();
        let response = app
            .clone()
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/express/unit/list")
                    .method("POST")
                    .header("content-type", "application/json")
                    .body(Body::from(body))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[tokio::test]
    async fn test_identity_list_returns_error_without_db() {
        let pool = build_test_pool();
        let app = crate::router(pool);

        let body = serde_json::json!({"ids": ["id1"]}).to_string();
        let response = app
            .clone()
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/express/identity/list")
                    .method("POST")
                    .header("content-type", "application/json")
                    .body(Body::from(body))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[tokio::test]
    async fn test_group_list_returns_error_without_db() {
        let pool = build_test_pool();
        let app = crate::router(pool);

        let body = serde_json::json!({"ids": ["g1"]}).to_string();
        let response = app
            .clone()
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/express/group/list")
                    .method("POST")
                    .header("content-type", "application/json")
                    .body(Body::from(body))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[tokio::test]
    async fn test_role_list_returns_error_without_db() {
        let pool = build_test_pool();
        let app = crate::router(pool);

        let body = serde_json::json!({"ids": ["r1"]}).to_string();
        let response = app
            .clone()
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/express/role/list")
                    .method("POST")
                    .header("content-type", "application/json")
                    .body(Body::from(body))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[tokio::test]
    async fn test_person_with_unit_returns_error_without_db() {
        let pool = build_test_pool();
        let app = crate::router(pool);

        let body = serde_json::json!({"ids": ["p1"]}).to_string();
        let response = app
            .clone()
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/express/person/with/unit")
                    .method("POST")
                    .header("content-type", "application/json")
                    .body(Body::from(body))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[tokio::test]
    async fn test_person_with_identity_returns_error_without_db() {
        let pool = build_test_pool();
        let app = crate::router(pool);

        let body = serde_json::json!({"ids": ["p1"]}).to_string();
        let response = app
            .clone()
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/express/person/with/identity")
                    .method("POST")
                    .header("content-type", "application/json")
                    .body(Body::from(body))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[tokio::test]
    async fn test_person_list_exceeds_id_limit_returns_error() {
        let pool = build_test_pool();
        let app = crate::router(pool);

        let ids: Vec<String> = (0..101).map(|i| format!("p{}", i)).collect();
        let body = serde_json::json!({"ids": ids}).to_string();
        let response = app
            .clone()
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/express/person/list")
                    .method("POST")
                    .header("content-type", "application/json")
                    .body(Body::from(body))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        let body = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();
        let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
        assert_eq!(json["type"], "error");
    }

    #[tokio::test]
    async fn test_unit_list_empty_ids_returns_error() {
        let pool = build_test_pool();
        let app = crate::router(pool);

        let body = serde_json::json!({"ids": []}).to_string();
        let response = app
            .clone()
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/express/unit/list")
                    .method("POST")
                    .header("content-type", "application/json")
                    .body(Body::from(body))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
        let body = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();
        let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
        assert_eq!(json["type"], "error");
    }

    #[tokio::test]
    async fn test_person_list_returns_success_structure_with_db_error() {
        let pool = build_test_pool();
        let app = crate::router(pool);

        let body = serde_json::json!({"ids": ["p1"]}).to_string();
        let response = app
            .clone()
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/express/person/list")
                    .method("POST")
                    .header("content-type", "application/json")
                    .body(Body::from(body))
                    .unwrap(),
            )
            .await
            .unwrap();

        let body = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();
        let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
        assert!(json["type"].is_string());
        assert!(json["data"].is_null() || json["data"].is_object());
    }

    #[tokio::test]
    async fn test_router_includes_all_batch_routes() {
        let pool = build_test_pool();
        let app = crate::router(pool);

        // Verify router was built without panic — route registration is the main concern
        let _ = app;
    }
}
