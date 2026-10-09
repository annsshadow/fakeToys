// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

use axum::body::Body;
use axum::http::{Method, Request, StatusCode};
use serde_json::json;
use shared::response::ActionResult;
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
    let result: ActionResult<serde_json::Value> =
        ActionResult::success(json!({"count": 2, "data": []}));
    let json = serde_json::to_value(&result).unwrap();
    assert_eq!(json["type"], "success");
    assert!(json["data"].is_object());
}

#[test]
fn form_definition_roundtrips_o2oa_fields_without_loss() {
    let definition = json!({
        "pcData": {
            "json": {
                "mode": "PC",
                "moduleList": {"subject": {"type": "Textfield"}},
                "actions": {"save": {"script": "return true;"}},
                "events": {"load": {"code": "init();"}},
                "validation": {"subject": {"required": true}}
            },
            "html": "<div id=\"subject\"></div>"
        },
        "mobileData": {
            "json": {"mode": "Mobile", "moduleList": {}},
            "html": "<div class=\"mobile\"></div>"
        }
    });
    let parsed = crate::FormDefinition::parse(definition.clone()).unwrap();
    let stored = parsed.to_db().unwrap();
    let restored = crate::FormDefinition::from_db(Some(stored)).unwrap();

    assert_eq!(restored.into_value(), definition);
}

#[test]
fn form_definition_accepts_legacy_json_string_and_serializes_as_object() {
    let input = json!(r#"{"moduleList":{"name":{"type":"Textfield"}}}"#);
    let parsed: crate::FormDefinition = serde_json::from_value(input).unwrap();
    let output = serde_json::to_value(parsed).unwrap();

    assert!(output.is_object());
    assert_eq!(output["moduleList"]["name"]["type"], "Textfield");
}

#[test]
fn form_definition_rejects_malformed_or_wrong_module_list() {
    assert!(crate::FormDefinition::parse(json!("{not-json")).is_err());
    assert!(crate::FormDefinition::parse(json!([])).is_err());
    assert!(crate::FormDefinition::parse(json!({"moduleList": []})).is_err());
    assert!(crate::FormDefinition::parse(json!({
        "pcData": {"json": {"moduleList": []}}
    }))
    .is_err());
}

#[tokio::test]
async fn test_get_control_config_route() {
    let pool = build_test_pool();
    let app = crate::router(pool);

    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/cms_assemble_control/get/control/config")
                .method(Method::GET)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
}

#[tokio::test]
async fn test_post_document_id_view_count() {
    let pool = build_test_pool();
    let app = crate::cms_assemble_control_router(pool);

    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/document/test-id/view/count")
                .method(Method::POST)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_ne!(response.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn test_get_application_id() {
    let pool = build_test_pool();
    let app = crate::cms_assemble_control_router(pool);

    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/application/test-id")
                .method(Method::GET)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_ne!(response.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn test_get_document_search() {
    let pool = build_test_pool();
    let app = crate::cms_assemble_control_router(pool);

    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/cms_assemble_control/document/search")
                .method(Method::GET)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_ne!(response.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn test_get_anonymous_document_id_view() {
    let pool = build_test_pool();
    let app = crate::cms_assemble_control_router(pool);

    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/anonymous/document/test-id/view")
                .method(Method::GET)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_ne!(response.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn test_get_data_document_id_array_data() {
    let pool = build_test_pool();
    let app = crate::cms_assemble_control_router(pool);

    // o2server DataAction：array/data 为 POST（ActionUpdateArrayDataWithDocument）
    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/data/document/test-id/array/data")
                .method(Method::POST)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_ne!(response.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn test_get_data_document_id_mockdeletetoget() {
    let pool = build_test_pool();
    let app = crate::cms_assemble_control_router(pool);

    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/data/document/test-id/mockdeletetoget")
                .method(Method::GET)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_ne!(response.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn test_post_data_document_id_mockputtopost() {
    let pool = build_test_pool();
    let app = crate::cms_assemble_control_router(pool);

    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/data/document/test-id/mockputtopost")
                .method(Method::POST)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_ne!(response.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn test_get_data_document_id_path0() {
    let pool = build_test_pool();
    let app = crate::cms_assemble_control_router(pool);

    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/data/document/test-id/path0")
                .method(Method::GET)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_ne!(response.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn test_get_fileinfo_id() {
    let pool = build_test_pool();
    let app = crate::cms_assemble_control_router(pool);

    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/fileinfo/test-id")
                .method(Method::GET)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_ne!(response.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn test_get_fileinfo_id_mockdeletetoget() {
    let pool = build_test_pool();
    let app = crate::cms_assemble_control_router(pool);

    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/fileinfo/test-id/mockdeletetoget")
                .method(Method::GET)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_ne!(response.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn test_get_anonymous_fileinfo_download_document_id() {
    let pool = build_test_pool();
    let app = crate::cms_assemble_control_router(pool);

    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/anonymous/fileinfo/download/document/test-id")
                .method(Method::GET)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_ne!(response.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn test_get_fileinfo_download_document_id() {
    let pool = build_test_pool();
    let app = crate::cms_assemble_control_router(pool);

    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/fileinfo/download/document/test-id")
                .method(Method::GET)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_ne!(response.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn test_post_fileinfo_upload_document_docId() {
    let pool = build_test_pool();
    let app = crate::cms_assemble_control_router(pool);

    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/fileinfo/upload/document/test-id")
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

    let req_body =
        serde_json::to_string(&json!({"enabled": true, "maxCategoryCount": 300})).unwrap();

    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/cms_assemble_control/update/control/config")
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
                .uri(format!("/api/anonymous/document/{}/view", doc_id))
                .method(Method::GET)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let bytes = axum::body::to_bytes(response.into_body(), 4096)
        .await
        .unwrap();
    let json: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
    assert_eq!(json["type"], "success");
    assert_eq!(json["data"]["title"], "Test Title");
    assert_eq!(json["data"]["content"], "Test Content");
    assert_eq!(json["data"]["authorId"], "test-author");
}

#[tokio::test]
async fn test_document_soft_delete() {
    use shared::session::Session;
    use shared::testing::{is_db_available, test_pool};

    if !is_db_available().await {
        eprintln!("skipping test_document_soft_delete: DATABASE_URL not reachable");
        return;
    }

    // o2server ActionDeleteWithDocument 语义：删除的是文档数据（字段行），
    // 而非文档实体；且需要文档编辑者会话（IDOR 门禁）。
    let owner = "test-doc-softdelete-owner";
    let doc_id = "test-doc-softdelete-001";
    let now = chrono::Utc::now().naive_utc();
    let session = Session {
        token: "test-doc-softdelete-token".to_string(),
        person_unique: owner.to_string(),
        created_at: now,
        expires_at: now + chrono::Duration::hours(2),
    };

    {
        let pool = test_pool();
        let client = pool.get().await.unwrap();
        let _ = client
            .execute(
                "DELETE FROM x_cms_data_document_field WHERE doc_id = $1",
                &[&doc_id],
            )
            .await;
        let _ = client
            .execute(
                "INSERT INTO x_cms_data_document (id, title, creator) VALUES ($1, 'Test Title', $2) \
                 ON CONFLICT (id) DO UPDATE SET title = $2, creator = $2, deleted_at = NULL",
                &[&doc_id, &owner],
            )
            .await;
        client
            .execute(
                "INSERT INTO x_cms_data_document_field (id, doc_id, field_name, field_value) \
                 VALUES ('test-doc-softdelete-field', $1, 'title', 'Test Title')",
                &[&doc_id],
            )
            .await
            .unwrap();
    }

    let app = crate::router(test_pool());
    let req = Request::builder()
        .uri(format!("/api/data/document/{}/mockdeletetoget", doc_id))
        .method(Method::GET)
        .extension(session)
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(req).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let bytes = axum::body::to_bytes(response.into_body(), 4096)
        .await
        .unwrap();
    let json: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
    assert_eq!(json["type"], "success");
    assert!(json["data"]["deleted"].as_i64().unwrap() >= 1);

    // 文档实体仍在，数据字段已被软删
    {
        let client = test_pool().get().await.unwrap();
        let row = client
            .query_opt(
                "SELECT 1 FROM x_cms_data_document WHERE id = $1 AND deleted_at IS NULL",
                &[&doc_id],
            )
            .await
            .unwrap();
        assert!(
            row.is_some(),
            "document entity should survive data deletion"
        );
        let remaining = client
            .query_one(
                "SELECT COUNT(*) AS n FROM x_cms_data_document_field \
                 WHERE doc_id = $1 AND deleted_at IS NULL",
                &[&doc_id],
            )
            .await
            .unwrap();
        let n: i64 = remaining.get("n");
        assert_eq!(n, 0, "data fields should be soft-deleted");
        let _ = client
            .execute("DELETE FROM x_cms_data_document WHERE id = $1", &[&doc_id])
            .await;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::Body;
    use axum::http::{Method, Request, StatusCode};
    use tower::util::ServiceExt;

    #[tokio::test]
    async fn test_get_application_id() {
        let pool = build_test_pool();
        let app = crate::cms_assemble_control_router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/application/test-id")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_get_cms_assemble_control_get_control_c() {
        let pool = build_test_pool();
        let app = crate::cms_assemble_control_router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/cms_assemble_control/get/control/config")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_get_cms_assemble_control_list_control_() {
        let pool = build_test_pool();
        let app = crate::cms_assemble_control_router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/cms_assemble_control/list/control/sections")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_get_cms_assemble_control_update_contro() {
        let pool = build_test_pool();
        let app = crate::cms_assemble_control_router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/cms_assemble_control/update/control/config")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_get_commend_list_paging_docId() {
        let pool = build_test_pool();
        let app = crate::cms_assemble_control_router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/commend/list/paging/test-id")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_get_queryview_flag_view_definition_que() {
        let pool = build_test_pool();
        let app = crate::cms_assemble_control_router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/queryview/flag/test-id/definition/test-id")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_post_document_id_view_count() {
        let pool = build_test_pool();
        let app = crate::cms_assemble_control_router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/document/test-id/view/count")
                    .method(Method::POST)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
    }

    // ── cms/assemble/control/* 斜杠路径家族路由注册 ─────────────────────────
    // 这 4 条是前端 o2server 斜杠口径调用、此前未注册（桌面契约守卫 KNOWN_BACKEND_GAPS）的端点；
    // 现在补齐真实 list handler，此测试锁定"已注册"（非 404），DB 缺数据时返回 500/200 而非 404。
    async fn get_status(app: &axum::Router, uri: &str) -> StatusCode {
        app.clone()
            .oneshot(
                Request::builder()
                    .uri(uri)
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap()
            .status()
    }

    #[tokio::test]
    async fn test_cms_assemble_control_list_routes_registered() {
        let pool = build_test_pool();
        let app = crate::cms_assemble_control_router(pool);
        for uri in [
            "/api/cms/assemble/control/dict/list",
            "/api/cms/assemble/control/form/list",
            "/api/cms/assemble/control/view/list",
            "/api/cms/assemble/control/xform/list",
        ] {
            let status = get_status(&app, uri).await;
            assert_ne!(
                status,
                StatusCode::NOT_FOUND,
                "route {} should be registered",
                uri
            );
        }
    }
}

/// CMS 关键词搜索 LIKE 通配符转义（优化二轮 44）：review_v2_search 等关键词
/// 搜索原 `format!("%{}%", keyword)` 把用户词原样拼进 ILIKE，含 `_`/`%` 会被当
/// 通配符（搜 `_` 命中任意单字=近全表）。转义后须按字面匹配。
#[tokio::test]
async fn cms_keyword_search_treats_underscore_as_literal() {
    if !shared::testing::is_db_available().await {
        eprintln!("skipping cms_keyword_search_treats_underscore_as_literal: DB not reachable");
        return;
    }
    let pool = shared::testing::test_pool();
    let c = pool.get().await.unwrap();
    c.execute(
        "CREATE TABLE IF NOT EXISTS x_cms_comment (id VARCHAR(255) PRIMARY KEY, doc_id VARCHAR(255) NOT NULL, person_id VARCHAR(255), content TEXT, parent_id VARCHAR(255), create_time TIMESTAMP DEFAULT CURRENT_TIMESTAMP, deleted_at TIMESTAMP)",
        &[],
    )
    .await
    .unwrap();
    c.execute(
        "DELETE FROM x_cms_comment WHERE id IN ('u2-cmt-under','u2-cmt-plain')",
        &[],
    )
    .await
    .unwrap();
    // content 'a_c' 含字面下划线；'abc' 不含
    c.execute(
        "INSERT INTO x_cms_comment (id, doc_id, content) VALUES ('u2-cmt-under','u2doc','a_c'), ('u2-cmt-plain','u2doc','abc')",
        &[],
    )
    .await
    .unwrap();

    let params = std::collections::HashMap::from([("keyword".to_string(), "_".to_string())]);
    let resp = crate::review_v2_search(axum::Extension(pool.clone()), axum::extract::Query(params))
        .await
        .unwrap();
    let j = serde_json::to_value(&resp.0).unwrap();
    let ids: Vec<String> = j["data"]
        .as_array()
        .expect("data array")
        .iter()
        .map(|r| r["id"].as_str().unwrap_or_default().to_string())
        .collect();
    assert!(
        ids.contains(&"u2-cmt-under".to_string()),
        "应命中含字面下划线的评论"
    );
    assert!(
        !ids.contains(&"u2-cmt-plain".to_string()),
        "下划线须按字面匹配，不得当通配符命中 'abc'"
    );

    c.execute(
        "DELETE FROM x_cms_comment WHERE id IN ('u2-cmt-under','u2-cmt-plain')",
        &[],
    )
    .await
    .unwrap();
}
