#[cfg(test)]
mod tests {
    use super::*;
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
    async fn test_work_list_returns_success() {
        let pool = build_test_pool();
        let app = processplatform_core_entity_router(pool);

        let response = app
            .clone()
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/work/list")
                    .method(axum::http::Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_task_list_returns_success() {
        let pool = build_test_pool();
        let app = processplatform_core_entity_router(pool);

        let response = app
            .clone()
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/task/list")
                    .method(axum::http::Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_work_completed_list_returns_success() {
        let pool = build_test_pool();
        let app = processplatform_core_entity_router(pool);

        let response = app
            .clone()
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/workcompleted/list")
                    .method(axum::http::Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_ticket_list_returns_success() {
        let pool = build_test_pool();
        let app = processplatform_core_entity_router(pool);

        let response = app
            .clone()
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/ticket/list")
                    .method(axum::http::Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
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
    fn test_work_list_action_result_structure() {
        let result: ActionResult<serde_json::Value> = ActionResult::success(serde_json::json!({
            "count": 2,
            "data": [
                {"id": "work-1", "title": "测试工作", "workStatus": "running"}
            ]
        }));
        let json = serde_json::to_value(&result).unwrap();
        assert_eq!(json["type"], "success");
        let data = json["data"].as_object().unwrap();
        assert_eq!(data["count"], 2);
        assert!(data["data"].is_array());
    }
}
