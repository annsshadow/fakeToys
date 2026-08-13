#[cfg(test)]
mod tests {
    use axum::body::Body;
    use axum::extract::{Extension, Path, Json};
    use axum::http::{Request, Method, StatusCode};
    use shared::testing::{test_pool, test_sea_orm_pool};
    use tower::util::ServiceExt;

    #[tokio::test]
    async fn test_sync_to_knowledge() {
        let _result = crate::sync_to_knowledge(axum::extract::Extension(shared::testing::test_pool())).await;
    }

    #[tokio::test]
    async fn test_app_list() {
        let _result = crate::app_list(axum::extract::Extension(shared::testing::test_pool())).await;
    }

    #[tokio::test]
    async fn test_model_list() {
        let _result = crate::model_list(axum::extract::Extension(shared::testing::test_pool())).await;
    }

    #[tokio::test]
    async fn test_conversation_list() {
        let _result = crate::conversation_list(axum::extract::Extension(shared::testing::test_pool())).await;
    }

    #[tokio::test]
    async fn test_chat_list_paging() {
        let _result = crate::chat_list_paging(axum::extract::Extension(shared::testing::test_pool()), axum::extract::Path((1i32, 1i32))).await;
    }

    #[tokio::test]
    async fn test_chat_list_completion_paging() {
        let _result = crate::chat_list_completion_paging(axum::extract::Extension(shared::testing::test_pool()), axum::extract::Path(("test-id".to_string(), 1i32, 1i32))).await;
    }

    // SKIPPED: chat_delete requires Session parameter
    #[tokio::test]
    async fn test_config_get() {
        let _result = crate::config_get(axum::extract::Extension(shared::testing::test_pool())).await;
    }

    #[tokio::test]
    async fn test_config_base_config() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/ai/config/base/config")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "config_base_config route should be registered");
    }

    #[tokio::test]
    async fn test_config_list_model_paging() {
        let _result = crate::config_list_model_paging(axum::extract::Extension(shared::testing::test_pool()), axum::extract::Path((1i32, 1i32))).await;
    }

    #[tokio::test]
    async fn test_config_get_model() {
        let _result = crate::config_get_model(axum::extract::Extension(shared::testing::test_pool()), axum::extract::Path("test-id".to_string())).await;
    }

    #[tokio::test]
    async fn test_config_list_mcp_paging() {
        let _result = crate::config_list_mcp_paging(axum::extract::Extension(shared::testing::test_pool()), axum::extract::Path((1i32, 1i32))).await;
    }

    #[tokio::test]
    async fn test_config_get_mcp() {
        let _result = crate::config_get_mcp(axum::extract::Extension(shared::testing::test_pool()), axum::extract::Path("test-id".to_string())).await;
    }

    #[tokio::test]
    async fn test_list_enable_model() {
        let _result = crate::list_enable_model(axum::extract::Extension(shared::testing::test_pool())).await;
    }

    #[tokio::test]
    async fn test_file_get() {
        let _result = crate::file_get(axum::extract::Extension(shared::testing::test_pool()), axum::extract::Path("test-id".to_string())).await;
    }

    #[tokio::test]
    async fn test_file_download() {
        let _result = crate::file_download(axum::extract::Extension(shared::testing::test_pool()), axum::extract::Path("test-id".to_string())).await;
    }

    #[tokio::test]
    async fn test_file_download_scale() {
        let _result = crate::file_download_scale(axum::extract::Extension(shared::testing::test_pool()), axum::extract::Path("test-id".to_string())).await;
    }

    // SKIPPED: file_delete requires Session parameter
    #[tokio::test]
    async fn test_index_cms_doc() {
        let _result = crate::index_cms_doc(axum::extract::Extension(shared::testing::test_pool()), axum::extract::Path("test-id".to_string())).await;
    }

    #[tokio::test]
    async fn test_index_cms_doc_with_app() {
        let _result = crate::index_cms_doc_with_app(axum::extract::Extension(shared::testing::test_pool()), axum::extract::Path("test-id".to_string())).await;
    }

    #[tokio::test]
    async fn test_index_delete() {
        let _result = crate::index_delete(axum::extract::Extension(shared::testing::test_pool()), axum::extract::Path("test-id".to_string())).await;
    }

}