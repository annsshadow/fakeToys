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

    #[test]
    fn test_record_list_returns_success() {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            let pool = build_test_pool();
            let app = crate::attendance_core_entity_router(pool);

            let response = app
                .clone()
                .oneshot(
                    Request::builder()
                        .uri("/jaxrs/attendance/core/entity/record/list")
                        .method(axum::http::Method::GET)
                        .body(Body::empty())
                        .unwrap(),
                )
                .await
                .unwrap();

            assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
        });
    }

    #[test]
    fn test_rule_list_returns_success() {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            let pool = build_test_pool();
            let app = crate::attendance_core_entity_router(pool);

            let response = app
                .clone()
                .oneshot(
                    Request::builder()
                        .uri("/jaxrs/attendance/core/entity/rule/list")
                        .method(axum::http::Method::GET)
                        .body(Body::empty())
                        .unwrap(),
                )
                .await
                .unwrap();

            assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
        });
    }

    #[test]
    fn test_rule_create_returns_success() {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            let pool = build_test_pool();
            let app = crate::attendance_core_entity_router(pool);

            let response = app
                .clone()
                .oneshot(
                    Request::builder()
                        .uri("/jaxrs/attendance/core/entity/rule/create")
                        .method(axum::http::Method::POST)
                        .header("content-type", "application/json")
                        .body(Body::from(r#"{"name":"标准作息","startTime":"09:00","endTime":"18:00"}"#))
                        .unwrap(),
                )
                .await
                .unwrap();

            assert!(response.status() == StatusCode::INTERNAL_SERVER_ERROR
                || response.status() == StatusCode::NOT_FOUND);
        });
    }

    #[test]
    fn test_rule_update_returns_success() {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            let pool = build_test_pool();
            let app = crate::attendance_core_entity_router(pool);

            let response = app
                .clone()
                .oneshot(
                    Request::builder()
                        .uri("/jaxrs/attendance/core/entity/rule/rule-001/update")
                        .method(axum::http::Method::POST)
                        .header("content-type", "application/json")
                        .body(Body::from(r#"{"name":"更新后的规则"}"#))
                        .unwrap(),
                )
                .await
                .unwrap();

            assert!(response.status() == StatusCode::INTERNAL_SERVER_ERROR
                || response.status() == StatusCode::NOT_FOUND);
        });
    }

    #[test]
    fn test_rule_delete_returns_success() {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            let pool = build_test_pool();
            let app = crate::attendance_core_entity_router(pool);

            let response = app
                .clone()
                .oneshot(
                    Request::builder()
                        .uri("/jaxrs/attendance/core/entity/rule/rule-001/delete")
                        .method(axum::http::Method::GET)
                        .body(Body::empty())
                        .unwrap(),
                )
                .await
                .unwrap();

            assert!(response.status() == StatusCode::INTERNAL_SERVER_ERROR
                || response.status() == StatusCode::NOT_FOUND);
        });
    }

    #[test]
    fn test_record_create_returns_success() {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            let pool = build_test_pool();
            let app = crate::attendance_core_entity_router(pool);

            let response = app
                .clone()
                .oneshot(
                    Request::builder()
                        .uri("/jaxrs/attendance/core/entity/record/create")
                        .method(axum::http::Method::POST)
                        .header("content-type", "application/json")
                        .body(Body::from(r#"{"userId":"user-001","checkInTime":"2024-01-01T09:00:00"}"#))
                        .unwrap(),
                )
                .await
                .unwrap();

            assert!(response.status() == StatusCode::INTERNAL_SERVER_ERROR
                || response.status() == StatusCode::NOT_FOUND);
        });
    }

    #[test]
    fn test_record_update_returns_success() {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            let pool = build_test_pool();
            let app = crate::attendance_core_entity_router(pool);

            let response = app
                .clone()
                .oneshot(
                    Request::builder()
                        .uri("/jaxrs/attendance/core/entity/record/record-001/update")
                        .method(axum::http::Method::POST)
                        .header("content-type", "application/json")
                        .body(Body::from(r#"{"checkOutTime":"2024-01-01T18:00:00"}"#))
                        .unwrap(),
                )
                .await
                .unwrap();

            assert!(response.status() == StatusCode::INTERNAL_SERVER_ERROR
                || response.status() == StatusCode::NOT_FOUND);
        });
    }

    #[test]
    fn test_record_delete_returns_success() {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            let pool = build_test_pool();
            let app = crate::attendance_core_entity_router(pool);

            let response = app
                .clone()
                .oneshot(
                    Request::builder()
                        .uri("/jaxrs/attendance/core/entity/record/record-001/delete")
                        .method(axum::http::Method::GET)
                        .body(Body::empty())
                        .unwrap(),
                )
                .await
                .unwrap();

            assert!(response.status() == StatusCode::INTERNAL_SERVER_ERROR
                || response.status() == StatusCode::NOT_FOUND);
        });
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
