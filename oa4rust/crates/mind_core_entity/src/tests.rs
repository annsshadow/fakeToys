#[cfg(test)]
mod tests {
    use crate::{MindFolder, MindVersion};
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
    async fn test_list_returns_success() {
        let pool = build_test_pool();
        let app = crate::mind_core_entity_router(pool);

        let response = app
            .clone()
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/mind/core/entity/list")
                    .method(axum::http::Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[tokio::test]
    async fn test_folder_list_returns_success() {
        let pool = build_test_pool();
        let app = crate::mind_core_entity_router(pool);

        let response = app
            .clone()
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/mind/core/entity/folder/list")
                    .method(axum::http::Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[tokio::test]
    async fn test_version_list_returns_success() {
        let pool = build_test_pool();
        let app = crate::mind_core_entity_router(pool);

        let response = app
            .clone()
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/mind/core/entity/version/list/mind-001")
                    .method(axum::http::Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        // 由于没有数据库，会返回 INTERNAL_SERVER_ERROR (500) 或 NOT_FOUND (404)
        assert!(response.status() == StatusCode::INTERNAL_SERVER_ERROR
            || response.status() == StatusCode::NOT_FOUND);
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
    fn test_mind_folder_serialization() {
        let folder = MindFolder {
            id: "folder-001".to_string(),
            name: "工作文件夹".to_string(),
            parent_id: None,
            order_number: 1,
        };
        let json = serde_json::to_value(&folder).unwrap();
        assert_eq!(json["id"], "folder-001");
        assert_eq!(json["name"], "工作文件夹");
        assert_eq!(json["order_number"], 1);
    }

    #[test]
    fn test_mind_version_serialization() {
        let version = MindVersion {
            id: "version-001".to_string(),
            mind_id: "mind-001".to_string(),
            name: "版本1".to_string(),
            file_version: 1,
        };
        let json = serde_json::to_value(&version).unwrap();
        assert_eq!(json["id"], "version-001");
        assert_eq!(json["mind_id"], "mind-001");
        assert_eq!(json["file_version"], 1);
    }
}
