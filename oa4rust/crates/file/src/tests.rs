#[cfg(test)]
mod tests {
    use crate::{
        complex_top, file_download, router as file_router, folder_create, folder_list_top,
        folder_list_with_folder, upload_file_record,
    };
    use axum::extract::{Extension, Json};
    use shared::{
        error::AppError,
        response::ActionResult,
    };
    use serde_json::Value;
    use shared::testing::test_pool;

    #[test]
    fn test_action_result_success_serialization() {
        let result: ActionResult<serde_json::Value> =
            ActionResult::success(serde_json::json!({"count": 2, "data": []}));

        let json = serde_json::to_value(&result).unwrap();
        assert_eq!(json["type"], "success");
        assert!(json["data"].is_object());
    }

    #[test]
    fn test_file_router_builds() {
        let pool = test_pool();
        let _ = file_router(pool);
    }

    #[tokio::test]
    #[ignore = "requires a running PostgreSQL server"]
    async fn test_folder_list_top_with_db() {
        let pool = test_pool();
        let result: Result<Json<ActionResult<Value>>, AppError> =
            folder_list_top(Extension(pool)).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    #[ignore = "requires a running PostgreSQL server"]
    async fn test_complex_top_with_db() {
        let pool = test_pool();
        let result: Result<Json<ActionResult<Value>>, AppError> =
            complex_top(Extension(pool)).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    #[ignore = "requires a running PostgreSQL server"]
    async fn test_folder_create_empty_name_returns_error() {
        let pool = Extension(test_pool());
        let body = serde_json::json!({"name": "", "superior": null});
        let result = folder_create(pool, axum::extract::Json(body)).await;

        assert!(result.is_err());
        match result {
            Err(AppError::BadRequest(_)) => {}
            _ => panic!("expected BadRequest error"),
        }
    }

    #[tokio::test]
    #[ignore = "requires a running PostgreSQL server"]
    async fn test_file_upload_oversized_returns_error() {
        let pool = Extension(test_pool());
        let large_data = vec![0u8; (5 * 1024 * 1024 + 1) as usize];
        let mime = "application/pdf".to_string();
        let result = upload_file_record(pool, large_data, mime, None, None, None, None, None).await;

        assert!(result.is_err());
        match result {
            Err(AppError::BadRequest(_)) => {}
            _ => panic!("expected BadRequest error"),
        }
    }

    #[tokio::test]
    #[ignore = "requires a running PostgreSQL server"]
    async fn test_file_upload_disallowed_mime_returns_error() {
        let pool = Extension(test_pool());
        let data = vec![0u8; 100];
        let result = upload_file_record(pool, data, "application/zip".to_string(), None, None, None, None, None).await;

        assert!(result.is_err());
        match result {
            Err(AppError::BadRequest(_)) => {}
            _ => panic!("expected BadRequest error"),
        }
    }

    #[tokio::test]
    #[ignore = "requires a running PostgreSQL server"]
    async fn test_folder_list_with_folder_with_db() {
        let pool = test_pool();
        let result: Result<Json<ActionResult<Value>>, AppError> =
            folder_list_with_folder(Extension(pool), axum::extract::Path("test".to_string())).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    #[ignore = "requires a running PostgreSQL server"]
    async fn test_file_download_with_db() {
        let pool = test_pool();
        let result: Result<Json<ActionResult<Value>>, AppError> =
            file_download(Extension(pool), axum::extract::Path("test".to_string())).await;

        assert!(result.is_ok());
    }
}
