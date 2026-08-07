#[cfg(test)]
mod tests {
    use crate::{ComponentInfo as Component, component_router};
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
    async fn test_list_all_returns_internal_error() {
        let pool = build_test_pool();
        let app = component_router(pool);

        let response = app
            .clone()
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/component/list/all")
                    .method(axum::http::Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[tokio::test]
    async fn test_get_component_returns_internal_error() {
        let pool = build_test_pool();
        let app = component_router(pool);

        let response = app
            .clone()
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/component/comp-001")
                    .method(axum::http::Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[tokio::test]
    async fn test_count_returns_internal_error() {
        let pool = build_test_pool();
        let app = component_router(pool);

        let response = app
            .clone()
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/component/count")
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
    fn test_component_serialization() {
        let component = Component {
            id: "comp-001".to_string(),
            name: "desktop".to_string(),
            title: "工作台".to_string(),
            r#type: "system".to_string(),
            visible: true,
            order_number: Some(1),
            path: "/desktop".to_string(),
            icon_path: "/icon/desktop.png".to_string(),
        };
        let json = serde_json::to_value(&component).unwrap();
        assert_eq!(json["id"], "comp-001");
        assert_eq!(json["name"], "desktop");
        assert_eq!(json["type"], "system");
    }
}
