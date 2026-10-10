// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

#[cfg(test)]
mod tests {
    use crate::{
        complex_top, file_download, folder_create, folder_list_top, folder_list_with_folder,
        router as file_router, upload_file_record,
    };
    use axum::extract::{Extension, Json};
    use serde_json::Value;
    use shared::testing::test_pool;
    use shared::{error::AppError, response::ActionResult};

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

    // 优化二轮 52：complex_top「最新/置顶」展示原漏 `deleted_at IS NULL`，软删的
    // 文件/文件夹会出现在 top 展示里。修复后须排除软删行。
    #[tokio::test]
    #[ignore = "requires a running PostgreSQL server"]
    async fn test_complex_top_excludes_soft_deleted_rows() {
        let pool = test_pool();
        let c = pool.get().await.unwrap();
        // 清理 + 播种：软删行（极早 name 确保会进 top10）与未删行
        for stmt in [
            "DELETE FROM FILE_FILE WHERE id IN ('u2-cx-live','u2-cx-dead')",
            "DELETE FROM FILE_FOLDER WHERE id IN ('u2-cx-flive','u2-cx-fdead')",
        ] {
            c.execute(stmt, &[]).await.unwrap();
        }
        c.execute(
            "INSERT INTO FILE_FILE (id, name, person, reference_type, extension, length, deleted_at) \
             VALUES ('u2-cx-live','00zx-live','u2p','file','png',10,NULL), \
                    ('u2-cx-dead','00zx-dead','u2p','file','png',10,NOW())",
            &[],
        )
        .await
        .unwrap();
        c.execute(
            "INSERT INTO FILE_FOLDER (id, name, person, superior, deleted_at) \
             VALUES ('u2-cx-flive','00zf-live','u2p',NULL,NULL), \
                    ('u2-cx-fdead','00zf-dead','u2p',NULL,NOW())",
            &[],
        )
        .await
        .unwrap();

        let result = complex_top(Extension(pool.clone())).await;
        assert!(result.is_ok());
        let j = serde_json::to_value(&result.unwrap().0).unwrap();
        let files: Vec<&str> = j["data"]["attachmentList"]
            .as_array()
            .map(|a| a.iter().filter_map(|e| e["id"].as_str()).collect())
            .unwrap_or_default();
        let folders: Vec<&str> = j["data"]["folderList"]
            .as_array()
            .map(|a| a.iter().filter_map(|e| e["id"].as_str()).collect())
            .unwrap_or_default();

        assert!(files.contains(&"u2-cx-live"), "未删文件应出现在 top 展示");
        assert!(
            !files.contains(&"u2-cx-dead"),
            "软删文件不得出现在 top 展示（修复前会泄漏）"
        );
        assert!(
            folders.contains(&"u2-cx-flive"),
            "未删文件夹应出现在 top 展示"
        );
        assert!(
            !folders.contains(&"u2-cx-fdead"),
            "软删文件夹不得出现在 top 展示（修复前会泄漏）"
        );

        // 清理
        c.execute(
            "DELETE FROM FILE_FILE WHERE id IN ('u2-cx-live','u2-cx-dead')",
            &[],
        )
        .await
        .unwrap();
        c.execute(
            "DELETE FROM FILE_FOLDER WHERE id IN ('u2-cx-flive','u2-cx-fdead')",
            &[],
        )
        .await
        .unwrap();
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
        let result = upload_file_record(
            pool,
            data,
            "application/zip".to_string(),
            None,
            None,
            None,
            None,
            None,
        )
        .await;

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
