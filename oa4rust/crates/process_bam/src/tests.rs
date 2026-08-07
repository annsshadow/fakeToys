#[cfg(test)]
mod tests {
    use super::*;
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
            ActionResult::success(serde_json::json!({"totalProcesses": 128}));
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
    async fn test_state_summary() {
        let pool = build_test_pool();
        let app = crate::router(pool);

        let response = app
            .clone()
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/process/state/summary")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();
        let json: serde_json::Value = serde_json::from_slice(&body).unwrap();

        assert_eq!(json.get("type").and_then(|v| v.as_str()), Some("success"));
        assert!(json.get("data").is_some());
        let data = json.get("data").unwrap();
        assert_eq!(data.get("totalProcesses").and_then(|v| v.as_i64()), Some(128));
    }

    #[tokio::test]
    async fn test_state_running() {
        let pool = build_test_pool();
        let app = crate::router(pool);

        let response = app
            .clone()
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/process/state/running")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();
        let json: serde_json::Value = serde_json::from_slice(&body).unwrap();

        assert_eq!(json.get("type").and_then(|v| v.as_str()), Some("success"));
        assert!(json.get("data").is_some());
        let data = json.get("data").unwrap();
        assert_eq!(data.get("runningCount").and_then(|v| v.as_i64()), Some(42));
    }

    #[tokio::test]
    async fn test_state_organization() {
        let pool = build_test_pool();
        let app = crate::router(pool);

        let response = app
            .clone()
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/process/state/organization")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();
        let json: serde_json::Value = serde_json::from_slice(&body).unwrap();

        assert_eq!(json.get("type").and_then(|v| v.as_str()), Some("success"));
        assert!(json.get("data").is_some());
        let data = json.get("data").unwrap();
        let orgs = data.get("organizations").unwrap().as_array().unwrap();
        assert_eq!(orgs.len(), 2);
        assert_eq!(orgs[0].get("name").and_then(|v| v.as_str()), Some("研发部"));
    }
}
