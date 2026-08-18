#[cfg(test)]
mod tests {
    use crate::routes::process_designer_router;
    use shared::response::ActionResult;
    use serde_json::json;

    #[test]
    fn test_application_list_summary_action_result_format() {
        let result: ActionResult<serde_json::Value> = ActionResult::success(json!({
            "count": 3,
            "data": [
                {"id": "app1", "name": "Test App", "applicationCategory": "DEFAULT"}
            ]
        }));
        let json = serde_json::to_value(&result).unwrap();
        assert_eq!(json["type"], "success");
        assert_eq!(json["data"]["count"], 3);
        assert_eq!(json["data"]["data"][0]["id"], "app1");
    }

    #[test]
    fn test_application_list_action_result_format() {
        let result: ActionResult<serde_json::Value> = ActionResult::success(json!({
            "count": 2,
            "data": [
                {"id": "app1", "name": "App 1", "applicationCategory": "DEFAULT", "creator": "admin"}
            ]
        }));
        let json = serde_json::to_value(&result).unwrap();
        assert_eq!(json["type"], "success");
        assert_eq!(json["data"]["count"], 2);
        assert_eq!(json["data"]["data"][0]["creator"], "admin");
    }

    #[tokio::test]
    async fn test_application_list_route_exists() {
        let pool = deadpool_postgres::Pool::builder(
            deadpool_postgres::Manager::new(
                deadpool_postgres::tokio_postgres::Config::new(),
                deadpool_postgres::tokio_postgres::NoTls,
            ),
        )
        .build()
        .unwrap();

        let app = process_designer_router(pool);

        use axum::body::Body;
        use axum::http::{Request, Method};
        use tower::util::ServiceExt;

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/process/designer/application/list")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_ne!(response.status(), axum::http::StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_application_get_route_exists() {
        let pool = deadpool_postgres::Pool::builder(
            deadpool_postgres::Manager::new(
                deadpool_postgres::tokio_postgres::Config::new(),
                deadpool_postgres::tokio_postgres::NoTls,
            ),
        )
        .build()
        .unwrap();

        let app = process_designer_router(pool);

        use axum::body::Body;
        use axum::http::{Request, Method};
        use tower::util::ServiceExt;

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/process/designer/application/test-id")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_ne!(response.status(), axum::http::StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_application_create_route_exists() {
        let pool = deadpool_postgres::Pool::builder(
            deadpool_postgres::Manager::new(
                deadpool_postgres::tokio_postgres::Config::new(),
                deadpool_postgres::tokio_postgres::NoTls,
            ),
        )
        .build()
        .unwrap();

        let app = process_designer_router(pool);

        use axum::body::Body;
        use axum::http::{Request, Method, header};
        use tower::util::ServiceExt;

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/process/designer/application/create")
                    .method(Method::POST)
                    .header(header::CONTENT_TYPE, "application/json")
                    .body(Body::from(r#"{"name":"test"}"#))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_ne!(response.status(), axum::http::StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_application_update_route_exists() {
        let pool = deadpool_postgres::Pool::builder(
            deadpool_postgres::Manager::new(
                deadpool_postgres::tokio_postgres::Config::new(),
                deadpool_postgres::tokio_postgres::NoTls,
            ),
        )
        .build()
        .unwrap();

        let app = process_designer_router(pool);

        use axum::body::Body;
        use axum::http::{Request, Method, header};
        use tower::util::ServiceExt;

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/process/designer/application/update")
                    .method(Method::POST)
                    .header(header::CONTENT_TYPE, "application/json")
                    .body(Body::from(r#"{"id":"test-id","name":"updated"}"#))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_ne!(response.status(), axum::http::StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_application_remove_route_exists() {
        let pool = deadpool_postgres::Pool::builder(
            deadpool_postgres::Manager::new(
                deadpool_postgres::tokio_postgres::Config::new(),
                deadpool_postgres::tokio_postgres::NoTls,
            ),
        )
        .build()
        .unwrap();

        let app = process_designer_router(pool);

        use axum::body::Body;
        use axum::http::{Request, Method, header};
        use tower::util::ServiceExt;

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/process/designer/application/remove")
                    .method(Method::POST)
                    .header(header::CONTENT_TYPE, "application/json")
                    .body(Body::from(r#"{"id":"test-id"}"#))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_ne!(response.status(), axum::http::StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_db_error_returns_internal() {
        let mut cfg = deadpool_postgres::tokio_postgres::Config::new();
        cfg.host("invalid-host-that-does-not-exist")
            .port(1)
            .user("invalid")
            .password("invalid")
            .dbname("nonexistent");
        let mgr = deadpool_postgres::Manager::new(cfg, deadpool_postgres::tokio_postgres::NoTls);
        let pool = deadpool_postgres::Pool::builder(mgr).build().unwrap();
        let app = process_designer_router(pool);

        use axum::body::Body;
        use axum::http::{Request, Method};
        use tower::util::ServiceExt;

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/process/designer/application/list")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), axum::http::StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[test]
    fn test_application_list_empty_data_action_result_structure() {
        let result: ActionResult<serde_json::Value> = ActionResult::success(json!({
            "count": 0,
            "data": []
        }));
        let json = serde_json::to_value(&result).unwrap();
        assert_eq!(json["type"], "success");
        assert_eq!(json["data"]["count"], 0);
        assert_eq!(json["data"]["data"].as_array().unwrap().len(), 0);
    }
}
