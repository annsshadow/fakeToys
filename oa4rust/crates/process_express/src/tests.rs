#[cfg(test)]
mod tests {
    use crate::routes::process_express_router;
    use shared::response::ActionResult;
    use serde_json::json;

    #[test]
    fn test_task_count_action_result_format() {
        let result: ActionResult<serde_json::Value> = ActionResult::success(json!({
            "credential": "test_user",
            "count": 5
        }));
        let json = serde_json::to_value(&result).unwrap();
        assert_eq!(json["type"], "success");
        assert_eq!(json["data"]["credential"], "test_user");
        assert_eq!(json["data"]["count"], 5);
    }

    #[test]
    fn test_application_list_action_result_format() {
        let result: ActionResult<serde_json::Value> = ActionResult::success(json!({
            "count": 2,
            "data": [
                {"id": "1", "name": "Test Unit"}
            ]
        }));
        let json = serde_json::to_value(&result).unwrap();
        assert_eq!(json["type"], "success");
        assert_eq!(json["data"]["count"], 2);
        assert_eq!(json["data"]["data"][0]["id"], "1");
    }

    #[tokio::test]
    async fn test_task_count_route_exists() {
        let pool = deadpool_postgres::Pool::builder(
            deadpool_postgres::Manager::new(
                deadpool_postgres::tokio_postgres::Config::new(),
                deadpool_postgres::tokio_postgres::NoTls,
            ),
        )
        .build()
        .unwrap();

        let app = process_express_router(pool);

        use axum::body::Body;
        use axum::http::{Request, Method};
        use tower::util::ServiceExt;

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/process/task/count/test_user")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), axum::http::StatusCode::OK);
    }

    #[tokio::test]
    async fn test_read_count_route_exists() {
        let pool = deadpool_postgres::Pool::builder(
            deadpool_postgres::Manager::new(
                deadpool_postgres::tokio_postgres::Config::new(),
                deadpool_postgres::tokio_postgres::NoTls,
            ),
        )
        .build()
        .unwrap();

        let app = process_express_router(pool);

        use axum::body::Body;
        use axum::http::{Request, Method};
        use tower::util::ServiceExt;

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/process/read/count/test_user")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), axum::http::StatusCode::OK);
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

        let app = process_express_router(pool);

        use axum::body::Body;
        use axum::http::{Request, Method};
        use tower::util::ServiceExt;

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/process/application/list")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), axum::http::StatusCode::OK);
    }
}
