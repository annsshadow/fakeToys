#[cfg(test)]
mod tests {
    use crate::{
        complex_top, file_download, file_router, folder_create, folder_list_top,
        folder_list_with_folder, upload_file_record,
    };
    use axum::extract::{Extension, Json};
    use deadpool_postgres::tokio_postgres::{Config, NoTls};
    use deadpool_postgres::{Manager, Pool};
    use shared::{
        error::AppError,
        response::ActionResult,
    };
    use serde_json::Value;

    fn mock_pool() -> deadpool_postgres::Pool {
        let mgr = Manager::new(Config::new(), NoTls);
        Pool::builder(mgr).max_size(1).build().unwrap()
    }

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
        let pool = Pool::builder(Manager::new(
            deadpool_postgres::tokio_postgres::Config::new(),
            deadpool_postgres::tokio_postgres::NoTls,
        ))
        .build()
        .unwrap();

        let _ = file_router(pool);
    }

    #[test]
    fn test_folder_list_top_returns_error_without_db() {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            let pool = Pool::builder(Manager::new(
                deadpool_postgres::tokio_postgres::Config::new(),
                deadpool_postgres::tokio_postgres::NoTls,
            ))
            .build()
            .unwrap();

            let result: Result<Json<ActionResult<Value>>, AppError> =
                folder_list_top(Extension(pool)).await;

            match result {
                Ok(_) => panic!("expected error without DB"),
                Err(AppError::Internal) => {}
                Err(_) => panic!("expected Internal error"),
            }
        });
    }

    #[test]
    fn test_complex_top_returns_error_without_db() {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            let pool = Pool::builder(Manager::new(
                deadpool_postgres::tokio_postgres::Config::new(),
                deadpool_postgres::tokio_postgres::NoTls,
            ))
            .build()
            .unwrap();

            let result: Result<Json<ActionResult<Value>>, AppError> =
                complex_top(Extension(pool)).await;

            match result {
                Ok(_) => panic!("expected error without DB"),
                Err(AppError::Internal) => {}
                Err(_) => panic!("expected Internal error"),
            }
        });
    }

    #[tokio::test]
    async fn test_folder_create_empty_name_returns_error() {
        let pool = Extension(mock_pool());
        let body = serde_json::json!({"name": "", "superior": null});
        let result = folder_create(pool, axum::extract::Json(body)).await;

        assert!(result.is_err());
        match result {
            Err(AppError::BadRequest(_)) => {}
            _ => panic!("expected BadRequest error"),
        }
    }

    #[tokio::test]
    async fn test_file_upload_oversized_returns_error() {
        let pool = Extension(mock_pool());
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
    async fn test_file_upload_disallowed_mime_returns_error() {
        let pool = Extension(mock_pool());
        let data = vec![0u8; 100];
        let result = upload_file_record(pool, data, "application/zip".to_string(), None, None, None, None, None).await;

        assert!(result.is_err());
        match result {
            Err(AppError::BadRequest(_)) => {}
            _ => panic!("expected BadRequest error"),
        }
    }

    #[test]
    fn test_folder_list_with_folder_returns_error_without_db() {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            let pool = Pool::builder(Manager::new(
                deadpool_postgres::tokio_postgres::Config::new(),
                deadpool_postgres::tokio_postgres::NoTls,
            ))
            .build()
            .unwrap();

            let result: Result<Json<ActionResult<Value>>, AppError> =
                folder_list_with_folder(Extension(pool), axum::extract::Path("test".to_string())).await;

            match result {
                Ok(_) => panic!("expected error without DB"),
                Err(AppError::Internal) => {}
                Err(_) => panic!("expected Internal error"),
            }
        });
    }

    #[test]
    fn test_file_download_returns_error_without_db() {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            let pool = Pool::builder(Manager::new(
                deadpool_postgres::tokio_postgres::Config::new(),
                deadpool_postgres::tokio_postgres::NoTls,
            ))
            .build()
            .unwrap();

            let result: Result<Json<ActionResult<Value>>, AppError> =
                file_download(Extension(pool), axum::extract::Path("test".to_string())).await;

            match result {
                Ok(_) => panic!("expected error without DB"),
                Err(AppError::Internal) => {}
                Err(_) => panic!("expected Internal error"),
            }
        });
    }
}
