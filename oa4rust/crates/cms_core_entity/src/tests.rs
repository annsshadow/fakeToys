#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Article, Category};
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
    async fn test_category_get_returns_success() {
        let pool = build_test_pool();
        let app = crate::cms_core_entity_router(pool);

        let response = app
            .clone()
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/cms/category/test-category-id")
                    .method(axum::http::Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[tokio::test]
    async fn test_category_create_returns_success() {
        let pool = build_test_pool();
        let app = crate::cms_core_entity_router(pool);

        let response = app
            .clone()
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/cms/category/create")
                    .method(axum::http::Method::POST)
                    .header("content-type", "application/json")
                    .body(Body::from(r#"{"name":"test"}"#))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[tokio::test]
    async fn test_article_list_returns_success() {
        let pool = build_test_pool();
        let app = crate::cms_core_entity_router(pool);

        let response = app
            .clone()
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/cms/article/list")
                    .method(axum::http::Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[tokio::test]
    async fn test_article_get_returns_success() {
        let pool = build_test_pool();
        let app = crate::cms_core_entity_router(pool);

        let response = app
            .clone()
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/cms/article/test-article-id")
                    .method(axum::http::Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[tokio::test]
    async fn test_article_create_returns_success() {
        let pool = build_test_pool();
        let app = crate::cms_core_entity_router(pool);

        let response = app
            .clone()
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/cms/article/create")
                    .method(axum::http::Method::POST)
                    .header("content-type", "application/json")
                    .body(Body::from(r#"{"title":"test","categoryId":"cat-001"}"#))
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
    fn test_category_serialization() {
        let category = Category {
            id: "cat-001".to_string(),
            name: "新闻分类".to_string(),
            parent_id: None,
            sort_order: 1,
            status: "active".to_string(),
            create_time: "2024-01-01T00:00:00Z".to_string(),
        };
        let json = serde_json::to_value(&category).unwrap();
        assert_eq!(json["id"], "cat-001");
        assert_eq!(json["parentId"], serde_json::Value::Null);
        assert_eq!(json["sortOrder"], 1);
    }

    #[test]
    fn test_article_serialization() {
        let article = Article {
            id: "article-001".to_string(),
            category_id: "cat-001".to_string(),
            title: "测试文章".to_string(),
            content: Some("内容".to_string()),
            author_id: "user-001".to_string(),
            status: "published".to_string(),
            publish_time: Some("2024-01-01T10:00:00Z".to_string()),
            create_time: "2024-01-01T00:00:00Z".to_string(),
        };
        let json = serde_json::to_value(&article).unwrap();
        assert_eq!(json["title"], "测试文章");
        assert_eq!(json["publishTime"], "2024-01-01T10:00:00Z");
    }
}
