#[cfg(test)]
mod tests {
    use axum::body::Body;
    use axum::http::{Request, Method, StatusCode};
    use serde_json::Value;
    use tower::util::ServiceExt;

    use crate::{router as program_center_router, modules_all};
    use shared::error::AppError;
    use shared::response::ActionResult;

    #[test]
    fn test_action_result_success_structure() {
        let result: ActionResult<serde_json::Value> =
            ActionResult::success(serde_json::json!({"key": "value"}));
        assert_eq!(result.r#type, Some("success".to_string()));
        assert!(result.data.is_some());
        // Java 成功信封实测恒填空串 message（Gson 对齐，见 shared::response）
        assert_eq!(result.message, Some(String::new()));
        assert_eq!(result.count, Some(0));
    }

    #[test]
    fn test_action_result_error_structure() {
        let result: ActionResult<String> = ActionResult::error("something went wrong");
        assert_eq!(result.r#type, Some("error".to_string()));
        assert_eq!(result.message, Some("something went wrong".to_string()));
        assert!(result.data.is_none());
    }

    #[test]
    fn test_modules_all_returns_error_without_db() {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            let pool = build_dummy_pool().await;
            let result = modules_all(axum::extract::Extension(pool)).await;
            match result {
                Ok(_) => panic!("expected error without DB"),
                Err(AppError::Internal) => {}
                Err(_) => panic!("expected Internal error"),
            }
        });
    }

    #[test]
    fn test_program_center_router_routes_registered() {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            let pool = build_dummy_pool().await;
            let app = program_center_router(pool);

            let response = app
                .oneshot(
                    Request::builder()
                        .uri("/jaxrs/program/applications")
                        .body(Body::empty())
                        .unwrap(),
                )
                .await
                .unwrap();

            assert_ne!(response.status(), StatusCode::NOT_FOUND,
                "route /jaxrs/program/applications should be registered");
        });
    }

    #[test]
    fn test_action_result_serialization() {
        let result: ActionResult<i32> = ActionResult::success(42);
        let json = serde_json::to_value(&result).unwrap();
        assert_eq!(json["type"], "success");
        assert_eq!(json["data"], 42);
        // Java 成功信封实测 message 为空串而非缺省
        assert_eq!(json["message"], Value::String(String::new()));
    }

    async fn build_dummy_pool() -> deadpool_postgres::Pool {
        let mgr = deadpool_postgres::Manager::new(
            deadpool_postgres::tokio_postgres::Config::new(),
            deadpool_postgres::tokio_postgres::NoTls,
        );
        deadpool_postgres::Pool::builder(mgr).build().unwrap()
    }

    #[tokio::test]
    async fn test_get_jaxrs_program_applications() {
        let pool = build_dummy_pool().await;
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/program/applications")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_get_jaxrs_program_appstyle_current_style() {
        let pool = build_dummy_pool().await;
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/program/appstyle/current/style")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_get_jaxrs_program_datastructure_modules_all() {
        let pool = build_dummy_pool().await;
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/program/datastructure/modules/all")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_post_jaxrs_program_center_agent_create() {
        let pool = build_dummy_pool().await;
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/program_center/agent/create")
                    .method(Method::POST)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_post_jaxrs_program_center_agent_save_id() {
        let pool = build_dummy_pool().await;
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/program_center/agent/save/test-id")
                    .method(Method::POST)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_post_jaxrs_program_center_application_create() {
        let pool = build_dummy_pool().await;
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/program_center/application/create")
                    .method(Method::POST)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_post_jaxrs_program_center_application_save_id() {
        let pool = build_dummy_pool().await;
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/program_center/application/save/test-id")
                    .method(Method::POST)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
    }


}
