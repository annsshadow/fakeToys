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
    async fn test_application_list_summary_route_exists() {
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
                    .uri("/jaxrs/process/application/list/summary")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), axum::http::StatusCode::OK);
    }

    #[tokio::test]
    async fn test_designer_route_route_exists() {
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
                    .uri("/jaxrs/process/designer/route/test-route-id")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), axum::http::StatusCode::OK);
    }
}
