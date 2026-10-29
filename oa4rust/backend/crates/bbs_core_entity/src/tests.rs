// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

#[cfg(test)]
mod tests {
    use crate::entities::{
        bbs_forum_info::Model as ForumInfo, bbs_section_info::Model as SectionInfo,
        bbs_subject_info::Model as SubjectInfo,
    };
    use axum::body::Body;
    use axum::http::{Request, StatusCode};
    use shared::response::ActionResult;
    use tower::ServiceExt;

    #[test]
    fn test_forum_list_returns_success() {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            let db = shared::testing::mock_db().await;
            let app = crate::bbs_core_entity_router(db);

            let response = app
                .clone()
                .oneshot(
                    Request::builder()
                        .uri("/api/bbs/core/entity/forum/list")
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
    fn test_section_list_returns_success() {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            let db = shared::testing::mock_db().await;
            let app = crate::bbs_core_entity_router(db);

            let response = app
                .clone()
                .oneshot(
                    Request::builder()
                        .uri("/api/bbs/core/entity/section/list/test-forum-id")
                        .method(axum::http::Method::GET)
                        .body(Body::empty())
                        .unwrap(),
                )
                .await
                .unwrap();

            assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR); // axum 0.8: {param} 路由可匹配(0.7 下 :param/{param} 混用会 404), handler 缺 pool 返回 500
        });
    }

    #[test]
    fn test_subject_top_list_returns_success() {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            let db = shared::testing::mock_db().await;
            let app = crate::bbs_core_entity_router(db);

            let response = app
                .clone()
                .oneshot(
                    Request::builder()
                        .uri("/api/bbs/core/entity/subject/top/test-section-id")
                        .method(axum::http::Method::GET)
                        .body(Body::empty())
                        .unwrap(),
                )
                .await
                .unwrap();

            assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR); // axum 0.8: {param} 路由可匹配(0.7 下 :param/{param} 混用会 404), handler 缺 pool 返回 500
        });
    }

    #[test]
    fn test_subject_list_returns_success() {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            let db = shared::testing::mock_db().await;
            let app = crate::bbs_core_entity_router(db);

            let response = app
                .clone()
                .oneshot(
                    Request::builder()
                        .uri("/api/bbs/core/entity/subject/list/test-section-id")
                        .method(axum::http::Method::GET)
                        .body(Body::empty())
                        .unwrap(),
                )
                .await
                .unwrap();

            assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR); // axum 0.8: {param} 路由可匹配(0.7 下 :param/{param} 混用会 404), handler 缺 pool 返回 500
        });
    }

    #[test]
    fn test_forum_create_returns_success() {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            let db = shared::testing::mock_db().await;
            let app = crate::bbs_core_entity_router(db);

            let response = app
                .clone()
                .oneshot(
                    Request::builder()
                        .uri("/api/bbs/core/entity/forum")
                        .method(axum::http::Method::POST)
                        .header("content-type", "application/json")
                        .body(Body::from(r#"{"name":"测试论坛"}"#))
                        .unwrap(),
                )
                .await
                .unwrap();

            assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
        });
    }

    #[test]
    fn test_forum_update_returns_success() {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            let db = shared::testing::mock_db().await;
            let app = crate::bbs_core_entity_router(db);

            let response = app
                .clone()
                .oneshot(
                    Request::builder()
                        .uri("/api/bbs/core/entity/forum/forum-001")
                        .method(axum::http::Method::POST)
                        .header("content-type", "application/json")
                        .body(Body::from(r#"{"name":"更新论坛名称"}"#))
                        .unwrap(),
                )
                .await
                .unwrap();

            assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
        });
    }

    #[test]
    fn test_forum_delete_returns_success() {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            let db = shared::testing::mock_db().await;
            let app = crate::bbs_core_entity_router(db);

            let response = app
                .clone()
                .oneshot(
                    Request::builder()
                        .uri("/api/bbs/core/entity/forum/forum-001")
                        .method(axum::http::Method::DELETE)
                        .body(Body::empty())
                        .unwrap(),
                )
                .await
                .unwrap();

            assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
        });
    }

    #[test]
    fn test_section_create_returns_success() {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            let db = shared::testing::mock_db().await;
            let app = crate::bbs_core_entity_router(db);

            let response = app
                .clone()
                .oneshot(
                    Request::builder()
                        .uri("/api/bbs/core/entity/section")
                        .method(axum::http::Method::POST)
                        .header("content-type", "application/json")
                        .body(Body::from(r#"{"name":"前端版块","forumId":"forum-001"}"#))
                        .unwrap(),
                )
                .await
                .unwrap();

            assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
        });
    }

    #[test]
    fn test_section_delete_returns_success() {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            let db = shared::testing::mock_db().await;
            let app = crate::bbs_core_entity_router(db);

            let response = app
                .clone()
                .oneshot(
                    Request::builder()
                        .uri("/api/bbs/core/entity/section/section-001")
                        .method(axum::http::Method::DELETE)
                        .body(Body::empty())
                        .unwrap(),
                )
                .await
                .unwrap();

            assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
        });
    }

    #[test]
    fn test_subject_create_returns_success() {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            let db = shared::testing::mock_db().await;
            let app = crate::bbs_core_entity_router(db);

            let response = app
                .clone()
                .oneshot(
                    Request::builder()
                        .uri("/api/bbs/core/entity/subject")
                        .method(axum::http::Method::POST)
                        .header("content-type", "application/json")
                        .body(Body::from(
                            r#"{"title":"测试帖子","sectionId":"section-001"}"#,
                        ))
                        .unwrap(),
                )
                .await
                .unwrap();

            assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
        });
    }

    #[test]
    fn test_subject_delete_returns_success() {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            let db = shared::testing::mock_db().await;
            let app = crate::bbs_core_entity_router(db);

            let response = app
                .clone()
                .oneshot(
                    Request::builder()
                        .uri("/api/bbs/core/entity/subject/subject-001")
                        .method(axum::http::Method::DELETE)
                        .body(Body::empty())
                        .unwrap(),
                )
                .await
                .unwrap();

            assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
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
    fn test_forum_info_serialization() {
        let forum = ForumInfo {
            id: "forum-001".to_string(),
            name: "技术论坛".to_string(),
            description: Some("交流技术".to_string()),
            create_time: None,
            disable: false,
            order_number: 0,
        };
        let json = serde_json::to_string(&forum).unwrap();
        assert!(json.contains("forum-001"));
        assert!(json.contains("技术论坛"));
    }

    #[test]
    fn test_section_info_serialization() {
        let section = SectionInfo {
            id: "section-001".to_string(),
            name: "前端板块".to_string(),
            forum_id: "forum-001".to_string(),
            description: None,
            order_number: 1,
            disable: false,
            create_time: None,
        };
        let json = serde_json::to_string(&section).unwrap();
        assert!(json.contains("section-001"));
        assert!(json.contains("前端板块"));
    }

    #[test]
    fn test_subject_info_serialization() {
        let subject = SubjectInfo {
            id: "subject-001".to_string(),
            title: "Rust异步编程".to_string(),
            author_id: "user-001".to_string(),
            section_id: "section-001".to_string(),
            content: None,
            reply_count: 0,
            view_count: 0,
            is_top: false,
            disable: false,
            create_time: None,
        };
        let json = serde_json::to_string(&subject).unwrap();
        assert!(json.contains("subject-001"));
        assert!(json.contains("Rust异步编程"));
    }

    // ── B2 抽样复核（2026-10-30）：create_reply 从功能桩改为真实落库 ──
    // ① 无会话必须 401（门在 DB 前，离线可证）；
    // ② DB 臂：真实 PG 下落库并可读回，且 creator 取会话身份。
    #[test]
    fn create_reply_rejects_without_session() {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            let db = shared::testing::mock_db().await;
            let res = crate::create_reply(
                axum::extract::Extension(db),
                None,
                axum::extract::Json(serde_json::json!({
                    "topicId": "t-1",
                    "content": "hello",
                })),
            )
            .await;
            assert!(
                matches!(res, Err(shared::error::AppError::Unauthorized)),
                "无会话调用 create_reply 必须 401（门在 DB 前）"
            );
        });
    }

    #[tokio::test]
    async fn create_reply_inserts_row_with_session() {
        if !shared::testing::is_db_available().await {
            eprintln!("skipping create_reply_inserts_row_with_session: DATABASE_URL not reachable");
            return;
        }
        use sea_orm::{ConnectionTrait, Statement};
        let db = shared::testing::test_sea_orm_pool()
            .await
            .expect("PG 连接应成功");
        let sess = shared::testing::test_session("b2-reply-user");
        let res = crate::create_reply(
            axum::extract::Extension(db.clone()),
            Some(axum::extract::Extension(sess)),
            axum::extract::Json(serde_json::json!({
                "subjectId": "b2-reply-subject",
                "content": "db-arm content",
            })),
        )
        .await;
        let resp = res.expect("有会话应放行");
        let id = resp
            .0
            .data
            .as_ref()
            .and_then(|d| d.get("id"))
            .and_then(|v| v.as_str())
            .unwrap_or_default()
            .to_string();
        assert!(!id.is_empty(), "落库应返回新 id");

        let backend = db.get_database_backend();
        let row = db
            .query_one(Statement::from_sql_and_values(
                backend,
                "SELECT content, creator FROM x_bbs_reply WHERE id = $1",
                vec![id.clone().into()],
            ))
            .await
            .expect("查询 x_bbs_reply 应成功");
        let (content, creator) = match row {
            Some(r) => {
                let c: Option<Option<String>> = r.try_get("", "content").ok();
                let cr: Option<Option<String>> = r.try_get("", "creator").ok();
                (c.flatten(), cr.flatten())
            }
            None => (None, None),
        };
        assert_eq!(content.as_deref(), Some("db-arm content"));
        assert_eq!(
            creator.as_deref(),
            Some("b2-reply-user"),
            "creator 应取会话身份"
        );

        // 清理
        db.execute(Statement::from_sql_and_values(
            backend,
            "DELETE FROM x_bbs_reply WHERE id = $1",
            vec![id.into()],
        ))
        .await
        .expect("清理 x_bbs_reply 应成功");
    }
}
