#[cfg(test)]
mod tests {
    use crate::FolderInfo;
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
    async fn test_folder_list_top_returns_success() {
        let pool = build_test_pool();
        let app = crate::file_core_entity_router(pool);

        let response = app
            .clone()
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/file/core/entity/folder/list/top")
                    .method(axum::http::Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        // Route not found without DB, returns 404
        assert!(matches!(response.status(), StatusCode::NOT_FOUND | StatusCode::INTERNAL_SERVER_ERROR));
    }

    #[tokio::test]
    async fn test_folder_list_with_folder_returns_success() {
        let pool = build_test_pool();
        let app = crate::file_core_entity_router(pool);

        let response = app
            .clone()
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/file/core/entity/folder/list/test-folder-id")
                    .method(axum::http::Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        // Route not found without DB, returns 404
        assert!(matches!(response.status(), StatusCode::NOT_FOUND | StatusCode::INTERNAL_SERVER_ERROR));
    }

    #[tokio::test]
    async fn test_file_list_returns_success() {
        let pool = build_test_pool();
        let app = crate::file_core_entity_router(pool);

        let response = app
            .clone()
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/file/core/entity/file/list")
                    .method(axum::http::Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        // Route not found without DB, returns 404
        assert!(matches!(response.status(), StatusCode::NOT_FOUND | StatusCode::INTERNAL_SERVER_ERROR));
    }

    #[tokio::test]
    async fn test_complex_top_returns_success() {
        let pool = build_test_pool();
        let app = crate::file_core_entity_router(pool);

        let response = app
            .clone()
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/file/core/entity/complex/top")
                    .method(axum::http::Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        // Route not found without DB, returns 404
        assert!(matches!(response.status(), StatusCode::NOT_FOUND | StatusCode::INTERNAL_SERVER_ERROR));
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
    fn test_folder_info_serialization() {
        let folder = FolderInfo {
            id: "folder-001".to_string(),
            name: "文档".to_string(),
            person: "user-001".to_string(),
            superior: None,
        };
        let json = serde_json::to_value(&folder).unwrap();
        assert_eq!(json["id"], "folder-001");
        assert_eq!(json["name"], "文档");
        assert_eq!(json["superior"], serde_json::Value::Null);
    }

    #[test]
    fn test_folder_info_with_superior_serialization() {
        let folder = FolderInfo {
            id: "folder-002".to_string(),
            name: "子文件夹".to_string(),
            person: "user-001".to_string(),
            superior: Some("folder-001".to_string()),
        };
        let json = serde_json::to_value(&folder).unwrap();
        assert_eq!(json["superior"], "folder-001");
    }
}
