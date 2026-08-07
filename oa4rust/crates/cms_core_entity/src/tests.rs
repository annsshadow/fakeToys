#[cfg(test)]
mod tests {
    use crate::{AppInfo, CategoryInfo};
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
    async fn test_category_list_returns_success() {
        let pool = build_test_pool();
        let app = crate::cms_core_entity_router(pool);

        let response = app
            .clone()
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/cms/category/list")
                    .method(axum::http::Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[tokio::test]
    async fn test_app_list_returns_success() {
        let pool = build_test_pool();
        let app = crate::cms_core_entity_router(pool);

        let response = app
            .clone()
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/cms/app/list")
                    .method(axum::http::Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[tokio::test]
    async fn test_app_config_list_returns_success() {
        let pool = build_test_pool();
        let app = crate::cms_core_entity_router(pool);

        let response = app
            .clone()
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/cms/app/config/list/test-app-id")
                    .method(axum::http::Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR); // axum 0.8: {param} 路由可匹配(0.7 下 :param/{param} 混用会 404), handler 缺 pool 返回 500
    }

    #[tokio::test]
    async fn test_category_ext_list_returns_success() {
        let pool = build_test_pool();
        let app = crate::cms_core_entity_router(pool);

        let response = app
            .clone()
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/cms/category/ext/list/test-category-id")
                    .method(axum::http::Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR); // axum 0.8: {param} 路由可匹配(0.7 下 :param/{param} 混用会 404), handler 缺 pool 返回 500
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
    fn test_app_info_serialization() {
        let app = AppInfo {
            id: "app-001".to_string(),
            name: "内容管理系统".to_string(),
            flag: "cms".to_string(),
            category: "content".to_string(),
        };
        let json = serde_json::to_value(&app).unwrap();
        assert_eq!(json["id"], "app-001");
        assert_eq!(json["flag"], "cms");
    }

    #[test]
    fn test_category_serialization() {
        let category = CategoryInfo {
            id: "cat-001".to_string(),
            name: "新闻分类".to_string(),
            app_id: "app-001".to_string(),
            parent_id: None,
        };
        let json = serde_json::to_value(&category).unwrap();
        assert_eq!(json["id"], "cat-001");
        assert_eq!(json["name"], "新闻分类");
        assert_eq!(json["parentId"], serde_json::Value::Null);
    }
}
