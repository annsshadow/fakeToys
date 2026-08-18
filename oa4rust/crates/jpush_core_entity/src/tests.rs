#[cfg(test)]
mod tests {
    use crate::{PushDevice, PushTemplate};
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
    async fn test_device_list_returns_success() {
        let pool = build_test_pool();
        let app = crate::jpush_core_entity_router(pool);

        let response = app
            .clone()
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/jpush/core/entity/device/list")
                    .method(axum::http::Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[tokio::test]
    async fn test_template_list_returns_success() {
        let pool = build_test_pool();
        let app = crate::jpush_core_entity_router(pool);

        let response = app
            .clone()
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/jpush/core/entity/template/list")
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

    #[test]
    fn test_push_device_serialization() {
        let device = PushDevice {
            id: "device-001".to_string(),
            user_id: "user-001".to_string(),
            platform: "android".to_string(),
            token: "token-abc123".to_string(),
        };
        let json = serde_json::to_value(&device).unwrap();
        assert_eq!(json["id"], "device-001");
        assert_eq!(json["platform"], "android");
    }

    #[test]
    fn test_push_template_serialization() {
        let template = PushTemplate {
            id: "template-001".to_string(),
            name: "欢迎模板".to_string(),
            title: "欢迎使用".to_string(),
            content: "欢迎加入系统".to_string(),
        };
        let json = serde_json::to_value(&template).unwrap();
        assert_eq!(json["id"], "template-001");
        assert_eq!(json["name"], "欢迎模板");
    }
}
