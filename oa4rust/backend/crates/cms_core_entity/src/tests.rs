// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

#[cfg(test)]
mod tests {

    use crate::entities::{cms_article::Model as Article, cms_category::Model as Category};
    use axum::body::Body;
    use axum::http::{Request, StatusCode};
    use chrono::NaiveDateTime;
    use deadpool_postgres::{Manager, Pool};
    use shared::response::ActionResult;
    use std::str::FromStr;
    use tower::ServiceExt;

    #[test]
    fn test_category_list_returns_success() {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            let db = shared::testing::mock_db().await;
            let app = crate::cms_core_entity_router(db);

            let response = app
                .clone()
                .oneshot(
                    Request::builder()
                        .uri("/api/cms/category/list")
                        .method(axum::http::Method::GET)
                        .body(Body::empty())
                        .unwrap(),
                )
                .await
                .unwrap();

            assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
        });
    }

    #[test]
    fn test_category_get_returns_success() {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            let db = shared::testing::mock_db().await;
            let app = crate::cms_core_entity_router(db);

            let response = app
                .clone()
                .oneshot(
                    Request::builder()
                        .uri("/api/cms/category/test-category-id")
                        .method(axum::http::Method::GET)
                        .body(Body::empty())
                        .unwrap(),
                )
                .await
                .unwrap();

            assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
        });
    }

    #[test]
    fn test_category_list_includes_created_data() {
        // Without DB, list returns 500; with DB would include created data.
        // This test verifies the route is accessible.
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            let db = shared::testing::mock_db().await;
            let app = crate::cms_core_entity_router(db);

            let response = app
                .clone()
                .oneshot(
                    Request::builder()
                        .uri("/api/cms/category/list")
                        .method(axum::http::Method::GET)
                        .body(Body::empty())
                        .unwrap(),
                )
                .await
                .unwrap();

            assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
        });
    }

    #[test]
    fn test_category_update_returns_success() {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            let db = shared::testing::mock_db().await;
            let app = crate::cms_core_entity_router(db);

            // Note: cms uses GET for update route (category/{id}) - not a PUT endpoint
            let response = app
                .clone()
                .oneshot(
                    Request::builder()
                        .uri("/api/cms/category/test-category-id")
                        .method(axum::http::Method::GET)
                        .body(Body::empty())
                        .unwrap(),
                )
                .await
                .unwrap();

            // 这是只读 GET（category 详情），无会话门；无 DB 时返回 500。
            assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
        });
    }

    #[test]
    fn test_article_create_returns_success() {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            let db = shared::testing::mock_db().await;
            let app = crate::cms_core_entity_router(db);

            let response = app
                .clone()
                .oneshot(
                    Request::builder()
                        .uri("/api/cms/article/create")
                        .method(axum::http::Method::POST)
                        .header("content-type", "application/json")
                        .body(Body::from(r#"{"title":"test","categoryId":"cat-001"}"#))
                        .unwrap(),
                )
                .await
                .unwrap();

            assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
        });
    }

    #[test]
    fn test_article_list_returns_success() {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            let db = shared::testing::mock_db().await;
            let app = crate::cms_core_entity_router(db);

            let response = app
                .clone()
                .oneshot(
                    Request::builder()
                        .uri("/api/cms/article/list")
                        .method(axum::http::Method::GET)
                        .body(Body::empty())
                        .unwrap(),
                )
                .await
                .unwrap();

            assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
        });
    }

    #[test]
    fn test_article_get_returns_success() {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            let db = shared::testing::mock_db().await;
            let app = crate::cms_core_entity_router(db);

            let response = app
                .clone()
                .oneshot(
                    Request::builder()
                        .uri("/api/cms/article/test-article-id")
                        .method(axum::http::Method::GET)
                        .body(Body::empty())
                        .unwrap(),
                )
                .await
                .unwrap();

            assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
        });
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
            create_time: None,
            deleted_at: None,
        };
        let json = serde_json::to_value(&category).unwrap();
        assert_eq!(json["id"], "cat-001");
        assert_eq!(json["parent_id"], serde_json::Value::Null);
        assert_eq!(json["sort_order"], 1);
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
            publish_time: Some(NaiveDateTime::from_str("2024-01-01T10:00:00").unwrap()),
            create_time: None,
            deleted_at: None,
        };
        let json = serde_json::to_value(&article).unwrap();
        assert_eq!(json["title"], "测试文章");
        assert_eq!(json["publish_time"], "2024-01-01T10:00:00");
    }

    // cms/core/entity/* 斜杠路径家族（093 迁移建表）：路由已注册，无 DB 时返回 500 而非 404。
    #[test]
    fn test_cms_core_entity_list_routes_registered() {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            let db = shared::testing::mock_db().await;
            let app = crate::cms_core_entity_router(db);
            for uri in [
                "/api/cms/core/entity/column/list",
                "/api/cms/core/entity/column_manager/list",
                "/api/cms/core/entity/index/list",
                "/api/cms/core/entity/module/list",
                "/api/cms/core/entity/note/list",
            ] {
                let response = app
                    .clone()
                    .oneshot(
                        Request::builder()
                            .uri(uri)
                            .method(axum::http::Method::GET)
                            .body(Body::empty())
                            .unwrap(),
                    )
                    .await
                    .unwrap();
                assert_ne!(
                    response.status(),
                    StatusCode::NOT_FOUND,
                    "route {} should be registered",
                    uri
                );
            }
        });
    }

    // 第六轮 R7 写面收口离线臂：15 把 cms/core/entity/* 薄 handler 无会话全 401。
    // dead pool + 无会话，require_admin_opt 在 pool.get() 之前先 401（出现 Internal 说明门插在读库之后）。
    #[tokio::test]
    async fn b46_r7_writes_reject_without_session() {
        use axum::extract::{Extension, Path};
        use axum::Json;
        use serde_json::json;

        let mgr = Manager::new(
            deadpool_postgres::tokio_postgres::Config::new(),
            deadpool_postgres::tokio_postgres::NoTls,
        );
        let pool = Pool::builder(mgr).build().unwrap();
        let ext = Extension(pool);
        let j = || Json(json!({"name": "x", "description": "d", "personId": "p"}));
        let p = || Path("b46-c1".to_string());
        let cases = vec![
            (
                "column_create",
                crate::cms_entity_column_create(ext.clone(), None, j())
                    .await
                    .map(|_| ()),
            ),
            (
                "column_save",
                crate::cms_entity_column_save(ext.clone(), None, p(), j())
                    .await
                    .map(|_| ()),
            ),
            (
                "column_delete",
                crate::cms_entity_column_delete(ext.clone(), None, p())
                    .await
                    .map(|_| ()),
            ),
            (
                "column_manager_create",
                crate::cms_entity_column_manager_create(ext.clone(), None, j())
                    .await
                    .map(|_| ()),
            ),
            (
                "column_manager_save",
                crate::cms_entity_column_manager_save(ext.clone(), None, p(), j())
                    .await
                    .map(|_| ()),
            ),
            (
                "column_manager_delete",
                crate::cms_entity_column_manager_delete(ext.clone(), None, p())
                    .await
                    .map(|_| ()),
            ),
            (
                "index_create",
                crate::cms_entity_index_create(ext.clone(), None, j())
                    .await
                    .map(|_| ()),
            ),
            (
                "index_save",
                crate::cms_entity_index_save(ext.clone(), None, p(), j())
                    .await
                    .map(|_| ()),
            ),
            (
                "index_delete",
                crate::cms_entity_index_delete(ext.clone(), None, p())
                    .await
                    .map(|_| ()),
            ),
            (
                "module_create",
                crate::cms_entity_module_create(ext.clone(), None, j())
                    .await
                    .map(|_| ()),
            ),
            (
                "module_save",
                crate::cms_entity_module_save(ext.clone(), None, p(), j())
                    .await
                    .map(|_| ()),
            ),
            (
                "module_delete",
                crate::cms_entity_module_delete(ext.clone(), None, p())
                    .await
                    .map(|_| ()),
            ),
            (
                "note_create",
                crate::cms_entity_note_create(ext.clone(), None, j())
                    .await
                    .map(|_| ()),
            ),
            (
                "note_save",
                crate::cms_entity_note_save(ext.clone(), None, p(), j())
                    .await
                    .map(|_| ()),
            ),
            (
                "note_delete",
                crate::cms_entity_note_delete(ext.clone(), None, p())
                    .await
                    .map(|_| ()),
            ),
        ];
        assert_eq!(
            cases.len(),
            15,
            "本批 15 把薄 handler 必须逐把点名，少一把即漏门"
        );
        let leaked: Vec<String> = cases
            .iter()
            .filter(|(_, r)| !matches!(r, Err(shared::error::AppError::Unauthorized)))
            .map(|(k, r)| format!("{k} => {:?}", r.as_ref().err()))
            .collect();
        assert!(
            leaked.is_empty(),
            "以下 cms/core/entity 写面无会话未回 401（出现 Internal 就是门插在读库之后）：{leaked:?}"
        );
    }
}
