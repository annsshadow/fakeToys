use super::*;
use shared::response::ActionResult;
use serde_json::json;
use axum::body::Body;
use axum::http::{Request, Method, StatusCode};
use deadpool_postgres::{Manager, Pool};
use deadpool_postgres::tokio_postgres::{Config, NoTls};
use tower::util::ServiceExt;



fn build_test_pool() -> deadpool_postgres::Pool {
    deadpool_postgres::Pool::builder(deadpool_postgres::Manager::new(
        deadpool_postgres::tokio_postgres::Config::new(),
        deadpool_postgres::tokio_postgres::NoTls,
    ))
    .build()
    .unwrap()
}
#[test]
fn test_action_result_success_serialization() {
    let result: ActionResult<serde_json::Value> = ActionResult::success(json!({"count": 2, "data": []}));
    let json = serde_json::to_value(&result).unwrap();
    assert_eq!(json["type"], "success");
    assert!(json["data"].is_object());
}

#[tokio::test]
async fn test_get_control_config_route() {
    let pool = build_test_pool();
    let app = crate::router(pool);

    let response = app
        .oneshot(
            Request::builder()
                .uri("/jaxrs/cms_assemble_control/get/control/config")
                .method(Method::GET)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
}

#[tokio::test]
async fn test_post_jaxrs_document_id_view_count() {
    let pool = build_test_pool();
    let app = crate::cms_assemble_control_router(pool);

    let response = app
        .oneshot(
            Request::builder()
                .uri("/jaxrs/document/test-id/view/count")
                .method(Method::POST)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_ne!(response.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn test_get_jaxrs_application_id() {
    let pool = build_test_pool();
    let app = crate::cms_assemble_control_router(pool);

    let response = app
        .oneshot(
            Request::builder()
                .uri("/jaxrs/application/test-id")
                .method(Method::GET)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_ne!(response.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn test_get_jaxrs_document_search() {
    let pool = build_test_pool();
    let app = crate::cms_assemble_control_router(pool);

    let response = app
        .oneshot(
            Request::builder()
                .uri("/jaxrs/cms_assemble_control/document/search")
                .method(Method::GET)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_ne!(response.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn test_get_jaxrs_anonymous_document_id_view() {
    let pool = build_test_pool();
    let app = crate::cms_assemble_control_router(pool);

    let response = app
        .oneshot(
            Request::builder()
                .uri("/jaxrs/anonymous/document/test-id/view")
                .method(Method::GET)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_ne!(response.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn test_get_jaxrs_data_document_id_array_data() {
    let pool = build_test_pool();
    let app = crate::cms_assemble_control_router(pool);

    let response = app
        .oneshot(
            Request::builder()
                .uri("/jaxrs/data/document/test-id/array/data")
                .method(Method::GET)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_ne!(response.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn test_get_jaxrs_data_document_id_mockdeletetoget() {
    let pool = build_test_pool();
    let app = crate::cms_assemble_control_router(pool);

    let response = app
        .oneshot(
            Request::builder()
                .uri("/jaxrs/data/document/test-id/mockdeletetoget")
                .method(Method::GET)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_ne!(response.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn test_post_jaxrs_data_document_id_mockputtopost() {
    let pool = build_test_pool();
    let app = crate::cms_assemble_control_router(pool);

    let response = app
        .oneshot(
            Request::builder()
                .uri("/jaxrs/data/document/test-id/mockputtopost")
                .method(Method::POST)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_ne!(response.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn test_get_jaxrs_data_document_id_path0() {
    let pool = build_test_pool();
    let app = crate::cms_assemble_control_router(pool);

    let response = app
        .oneshot(
            Request::builder()
                .uri("/jaxrs/data/document/test-id/path0")
                .method(Method::GET)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_ne!(response.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn test_get_jaxrs_fileinfo_id() {
    let pool = build_test_pool();
    let app = crate::cms_assemble_control_router(pool);

    let response = app
        .oneshot(
            Request::builder()
                .uri("/jaxrs/fileinfo/test-id")
                .method(Method::GET)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_ne!(response.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn test_get_jaxrs_fileinfo_id_mockdeletetoget() {
    let pool = build_test_pool();
    let app = crate::cms_assemble_control_router(pool);

    let response = app
        .oneshot(
            Request::builder()
                .uri("/jaxrs/fileinfo/test-id/mockdeletetoget")
                .method(Method::GET)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_ne!(response.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn test_get_jaxrs_anonymous_fileinfo_download_document_id() {
    let pool = build_test_pool();
    let app = crate::cms_assemble_control_router(pool);

    let response = app
        .oneshot(
            Request::builder()
                .uri("/jaxrs/anonymous/fileinfo/download/document/test-id")
                .method(Method::GET)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_ne!(response.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn test_get_jaxrs_fileinfo_download_document_id() {
    let pool = build_test_pool();
    let app = crate::cms_assemble_control_router(pool);

    let response = app
        .oneshot(
            Request::builder()
                .uri("/jaxrs/fileinfo/download/document/test-id")
                .method(Method::GET)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_ne!(response.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn test_post_jaxrs_fileinfo_upload_document_docId() {
    let pool = build_test_pool();
    let app = crate::cms_assemble_control_router(pool);

    let response = app
        .oneshot(
            Request::builder()
                .uri("/jaxrs/fileinfo/upload/document/test-id")
                .method(Method::POST)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_ne!(response.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn test_update_control_config_route() {
    let pool = build_test_pool();
    let app = crate::router(pool);

    let req_body = serde_json::to_string(&json!({"enabled": true, "maxCategoryCount": 300})).unwrap();

    let response = app
        .oneshot(
            Request::builder()
                .uri("/jaxrs/cms_assemble_control/update/control/config")
                .method(Method::GET)
                .header("content-type", "application/json")
                .body(Body::from(req_body))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
}

// ─── Integration tests (require PostgreSQL) ───────────────────────────────────

#[tokio::test]
async fn test_document_crud_end_to_end() {
    use shared::testing::{is_db_available, test_pool};

    if !is_db_available().await {
        eprintln!("skipping test_document_crud_end_to_end: DATABASE_URL not reachable");
        return;
    }

    let pool = test_pool();
    let client = pool.get().await.ok();

    let doc_id = "test-doc-crud-001";

    if let Some(c) = &client {
        let _ = c
            .execute(
                "INSERT INTO x_cms_data_document (id, title, content, author_id, status) \
                 VALUES ($1, $2, $3, $4, $5) \
                 ON CONFLICT (id) DO UPDATE SET title = $2, content = $3, author_id = $4, status = $5",
                &[&doc_id, &"Test Title", &"Test Content", &"test-author", &"draft"],
            )
            .await;
    }

    let app = crate::router(pool);

    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .uri(format!("/jaxrs/anonymous/document/{}/view", doc_id))
                .method(Method::GET)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let bytes = axum::body::to_bytes(response.into_body(), 4096).await.unwrap();
    let json: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
    assert_eq!(json["type"], "success");
    assert_eq!(json["data"]["title"], "Test Title");
    assert_eq!(json["data"]["content"], "Test Content");
    assert_eq!(json["data"]["authorId"], "test-author");
}

#[tokio::test]
async fn test_document_soft_delete() {
    use shared::testing::{is_db_available, test_pool};

    if !is_db_available().await {
        eprintln!("skipping test_document_soft_delete: DATABASE_URL not reachable");
        return;
    }

    let pool = test_pool();
    let client = pool.get().await.ok();

    let doc_id = "test-doc-softdelete-001";

    if let Some(c) = &client {
        let _ = c
            .execute(
                "INSERT INTO x_cms_data_document (id, title, content, author_id, status) \
                 VALUES ($1, $2, $3, $4, $5) \
                 ON CONFLICT (id) DO UPDATE SET title = $2, content = $3, author_id = $4, status = $5",
                &[&doc_id, &"Test Title", &"Test Content", &"test-author", &"draft"],
            )
            .await;
    }

    let app = crate::router(pool);

    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .uri(format!("/jaxrs/data/document/{}/mockdeletetoget", doc_id))
                .method(Method::GET)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let bytes = axum::body::to_bytes(response.into_body(), 4096).await.unwrap();
    let json: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
    assert_eq!(json["type"], "success");
    assert_eq!(json["data"]["deleted"], true);

    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .uri(format!("/jaxrs/anonymous/document/{}/view", doc_id))
                .method(Method::GET)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let bytes = axum::body::to_bytes(response.into_body(), 4096).await.unwrap();
    let json: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
    assert_eq!(json["type"], "error");
    assert_eq!(json["message"], "document not found");
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::Body;
    use axum::http::{Request, Method, StatusCode};
    use tower::util::ServiceExt;




    #[tokio::test]
    async fn test_get_jaxrs_application_id() {
        let pool = build_test_pool();
        let app = crate::cms_assemble_control_router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/application/test-id")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_get_jaxrs_cms_assemble_control_get_control_c() {
        let pool = build_test_pool();
        let app = crate::cms_assemble_control_router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/cms_assemble_control/get/control/config")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_get_jaxrs_cms_assemble_control_list_control_() {
        let pool = build_test_pool();
        let app = crate::cms_assemble_control_router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/cms_assemble_control/list/control/sections")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_get_jaxrs_cms_assemble_control_update_contro() {
        let pool = build_test_pool();
        let app = crate::cms_assemble_control_router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/cms_assemble_control/update/control/config")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_get_jaxrs_commend_list_paging_docId() {
        let pool = build_test_pool();
        let app = crate::cms_assemble_control_router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/commend/list/paging/test-id")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_get_jaxrs_queryview_flag_view_definition_que() {
        let pool = build_test_pool();
        let app = crate::cms_assemble_control_router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/queryview/flag/test-id/definition/test-id")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_post_jaxrs_document_id_view_count() {
        let pool = build_test_pool();
        let app = crate::cms_assemble_control_router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/document/test-id/view/count")
                    .method(Method::POST)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
    }
}
