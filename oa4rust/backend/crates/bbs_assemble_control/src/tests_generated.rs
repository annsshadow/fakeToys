// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

#[cfg(test)]
mod tests {
    #![allow(non_snake_case)]
    use axum::body::Body;
    use axum::http::{Request, StatusCode};

    use tower::util::ServiceExt;

    #[tokio::test]
    async fn test_get_control_config() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/bbs/assemble/control/config")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        if response.status() == StatusCode::NOT_FOUND {
            // Route-presence probe: an unregistered path hits the axum fallback
            // (empty 404 body); a matched handler may legitimately answer 404
            // (e.g. NotFound for missing row) but always with a JSON envelope.
            let bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
                .await
                .unwrap_or_default();
            assert!(
                !bytes.is_empty(),
                "get_control_config route should be registered"
            );
        }
    }

    #[tokio::test]
    async fn test_list_forums() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/bbs/assemble/control/forum/list")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        if response.status() == StatusCode::NOT_FOUND {
            // Route-presence probe: an unregistered path hits the axum fallback
            // (empty 404 body); a matched handler may legitimately answer 404
            // (e.g. NotFound for missing row) but always with a JSON envelope.
            let bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
                .await
                .unwrap_or_default();
            assert!(!bytes.is_empty(), "list_forums route should be registered");
        }
    }

    #[tokio::test]
    async fn test_create_topic() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/bbs/assemble/control/topic/create")
                    .method("POST")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        if response.status() == StatusCode::NOT_FOUND {
            // Route-presence probe: an unregistered path hits the axum fallback
            // (empty 404 body); a matched handler may legitimately answer 404
            // (e.g. NotFound for missing row) but always with a JSON envelope.
            let bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
                .await
                .unwrap_or_default();
            assert!(!bytes.is_empty(), "create_topic route should be registered");
        }
    }

    #[tokio::test]
    async fn test_create_reply() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/bbs/assemble/control/reply/create")
                    .method("POST")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        if response.status() == StatusCode::NOT_FOUND {
            // Route-presence probe: an unregistered path hits the axum fallback
            // (empty 404 body); a matched handler may legitimately answer 404
            // (e.g. NotFound for missing row) but always with a JSON envelope.
            let bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
                .await
                .unwrap_or_default();
            assert!(!bytes.is_empty(), "create_reply route should be registered");
        }
    }

    // SKIPPED: user_info requires Session parameter
    #[tokio::test]
    async fn test_uuid_generate() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/bbs/assemble/control/uuid")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        if response.status() == StatusCode::NOT_FOUND {
            // Route-presence probe: an unregistered path hits the axum fallback
            // (empty 404 body); a matched handler may legitimately answer 404
            // (e.g. NotFound for missing row) but always with a JSON envelope.
            let bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
                .await
                .unwrap_or_default();
            assert!(
                !bytes.is_empty(),
                "uuid_generate route should be registered"
            );
        }
    }

    // SKIPPED: list_control_sections not accessible
    // SKIPPED: section_create not accessible
    // SKIPPED: section_update not accessible
    // SKIPPED: section_delete requires Session parameter
    // SKIPPED: update_control_config not accessible
    // SKIPPED: get_forum not accessible
    // SKIPPED: list_topics_by_forum not accessible
    // SKIPPED: forum_view_all not accessible
    // SKIPPED: forum_id not accessible
    // SKIPPED: mobile_view_all not accessible
    // SKIPPED: permission_replyPublishable_subjectId not accessible
    // SKIPPED: permission_subjectPublishable_sectionId not accessible
    // SKIPPED: reply_filter_list_page_page_count_count not accessible
    // SKIPPED: reply_list_sub_id not accessible
    // SKIPPED: subject_view_id not accessible
    // SKIPPED: subject_top_sectionId not accessible
    // SKIPPED: permission_section_sectionId not accessible
    // SKIPPED: permission_subject_subjectId not accessible
    // SKIPPED: section_viewforum_forumId not accessible
    // SKIPPED: delete_forum requires Session parameter
    // SKIPPED: delete_reply requires Session parameter
    // SKIPPED: delete_subject requires Session parameter
    // SKIPPED: delete_forum_body requires Session parameter
    // SKIPPED: delete_reply_body requires Session parameter
    // SKIPPED: delete_subject_body requires Session parameter
    // SKIPPED: list_reply_filter not accessible
    // SKIPPED: list_topics_creamed not accessible
    // SKIPPED: list_topics_recommended not accessible
    // SKIPPED: list_subjects_filtered not accessible
    // SKIPPED: list_subjects_index not accessible
    // SKIPPED: list_subjects_recommended_index not accessible
    // SKIPPED: login not accessible
    // SKIPPED: logout not accessible
    // SKIPPED: picture_list not accessible
    // SKIPPED: shutup_create requires Session parameter
    // SKIPPED: shutup_delete requires Session parameter
    // SKIPPED: shutup_list not accessible
    // SKIPPED: subject_creamed_list not accessible
    // SKIPPED: subject_filter_list not accessible
    // SKIPPED: subject_filter_listsubjectinfo not accessible
    // SKIPPED: subject_index_list not accessible
    // SKIPPED: subject_search not accessible
    // SKIPPED: subject_statgrade not accessible
    // SKIPPED: topic_creamed_list not accessible
    // SKIPPED: topic_filter_list not accessible
    // SKIPPED: topic_filter_listsubjectinfo not accessible
    // SKIPPED: topic_index_list not accessible
    // SKIPPED: topic_recommended_index not accessible
    // SKIPPED: topic_recommended_list not accessible
    // SKIPPED: topic_search not accessible
    // SKIPPED: user_forum_list not accessible
    // SKIPPED: user_reply_list not accessible
    // SKIPPED: user_role_list not accessible
    // SKIPPED: user_section_list not accessible
    // SKIPPED: user_setting not accessible
    // SKIPPED: user_subject_list not accessible
    // SKIPPED: subjectattach_list not accessible
    // SKIPPED: u2_attachment_download not accessible
    // SKIPPED: u2_attachment_download_stream not accessible
    // SKIPPED: u2_attachment_upload not accessible
    // SKIPPED: u2_attachment_upload_callback not accessible
    // SKIPPED: picture_encode not accessible
    // SKIPPED: picture_section_icon requires Session parameter
    // SKIPPED: section_syn not accessible
    // SKIPPED: u2_subject_get not accessible
    // SKIPPED: u2_subject_save not accessible
    // SKIPPED: u2_subject_accept_reply requires Session parameter
    // SKIPPED: u2_subject_unaccept_reply requires Session parameter
    // SKIPPED: u2_subject_soft_delete requires Session parameter
    // SKIPPED: u2_subject_change_section requires Session parameter
    // SKIPPED: u2_vote_submit not accessible
    // SKIPPED: u2_voterecord_list not accessible
    // SKIPPED: u2_my_subject_list requires Session parameter
    // SKIPPED: u2_my_reply_list requires Session parameter
    // SKIPPED: u2_reply_get not accessible
    // SKIPPED: u2_reply_filter_list not accessible
    // SKIPPED: u2_user_reply_save not accessible
    // SKIPPED: u2_user_reply_accept requires Session parameter
    // SKIPPED: u2_user_reply_delete requires Session parameter
    // SKIPPED: u2_user_forum_save requires Session parameter
    // SKIPPED: u2_user_forum_delete requires Session parameter
    // SKIPPED: u2_section_get not accessible
    // SKIPPED: u2_section_viewsub not accessible
    // SKIPPED: u2_user_section_forum not accessible
    // SKIPPED: u2_user_section_sub not accessible
    // SKIPPED: u2_user_section_save requires Session parameter
    // SKIPPED: u2_user_section_delete requires Session parameter
    // SKIPPED: u2_user_section_delete_force requires Session parameter
    // SKIPPED: u2_role_get not accessible
    // SKIPPED: u2_role_all not accessible
    // SKIPPED: u2_role_save requires Session parameter
    // SKIPPED: u2_role_delete requires Session parameter
    // SKIPPED: u2_role_list_by_forum not accessible
    // SKIPPED: u2_role_list_by_section not accessible
    // SKIPPED: u2_role_bind_object requires Session parameter
    // SKIPPED: u2_role_bind_user requires Session parameter
    // SKIPPED: u2_role_selected_by_code not accessible
    // SKIPPED: u2_role_by_unit not accessible
    // SKIPPED: u2_role_by_user not accessible
    // SKIPPED: u2_permission_root not accessible
    // SKIPPED: u2_permission_admin_forum not accessible
    // SKIPPED: u2_permission_admin_section not accessible
    // SKIPPED: u2_permission_admin_role not accessible
    // SKIPPED: u2_setting_bbs_name not accessible
    // SKIPPED: u2_setting_get not accessible
    // SKIPPED: u2_setting_all not accessible
    // SKIPPED: u2_setting_update requires Session parameter
    // SKIPPED: u2_setting_get_by_code not accessible
    // SKIPPED: u2_userinfo_update_nick requires Session parameter
    // SKIPPED: u2_userinfo_filter not accessible
    // SKIPPED: u2_attachment_get not accessible
    // SKIPPED: u2_attachment_list_by_subject not accessible
    // SKIPPED: u2_attachment_delete requires Session parameter
    // SKIPPED: u2_subjectattach_get not accessible
    // SKIPPED: u2_subjectattach_list not accessible
    // SKIPPED: u2_subjectattach_base64 not accessible
    // SKIPPED: u2_subjectattach_delete requires Session parameter
    // SKIPPED: u2_shutup_get_mine requires Session parameter
    // SKIPPED: u2_shutup_delete_admin requires Session parameter
    // SKIPPED: u2_statgrade not accessible
    // SKIPPED: u2_subject_search_page not accessible
    // SKIPPED: u2_subject_listsubjectinfo_page not accessible

    /// R318 t98 桥接契约：section/create 后 H5 picker 数据源 forum/view/all 必须
    /// 可见同名记录且 forum id 与 section id 一致（topic 按 forum_id 归档）。
    /// live-gated：DB 不可达显式跳过；修复前 forum 无该记录 → 本测试 RED。
    #[tokio::test]
    async fn test_section_create_bridges_to_forum() {
        use shared::testing::{is_db_available, test_pool};
        if !is_db_available().await {
            eprintln!("skipping test_section_create_bridges_to_forum: DB not reachable");
            return;
        }
        let pool = test_pool();
        let marker = format!("r318-bridge-{}", uuid::Uuid::new_v4());
        {
            let c = pool.get().await.unwrap();
            c.execute("DELETE FROM auth_person WHERE unique_id = 'r318-bridge-admin' AND id <> 'r318-bridge-admin'", &[]).await.unwrap();
            c.execute("INSERT INTO auth_person (id, unique_id, name, password_hash, salt, created_at, updated_at) VALUES ('r318-bridge-admin', 'r318-bridge-admin', 'probe', '', '', NOW(), NOW()) ON CONFLICT (id) DO UPDATE SET unique_id = EXCLUDED.unique_id", &[]).await.unwrap();
            c.execute("INSERT INTO auth_role (id, name, created_at, updated_at) SELECT 'r318-bridge-admin-role', 'admin', NOW(), NOW() WHERE NOT EXISTS (SELECT 1 FROM auth_role WHERE name = 'admin')", &[]).await.unwrap();
            c.execute("INSERT INTO auth_person_role (person_id, role_id, unit_id, created_at) SELECT 'r318-bridge-admin', r.id, '', NOW() FROM auth_role r WHERE r.name = 'admin' ON CONFLICT DO NOTHING", &[]).await.unwrap();
        }
        let now = chrono::Utc::now().naive_utc();
        let sess = shared::session::Session {
            token: "r318-bridge-token".to_string(),
            person_unique: "r318-bridge-admin".to_string(),
            created_at: now,
            expires_at: now + chrono::Duration::hours(2),
        };
        let app = crate::router(pool.clone()).layer(axum::extract::Extension(sess));
        let created = app.clone().oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/bbs/assemble/control/section/create")
                .header("content-type", "application/json")
                .body(Body::from(serde_json::json!({ "name": marker }).to_string()))
                .unwrap(),
        ).await.unwrap();
        assert_eq!(created.status(), StatusCode::OK, "section/create must be 200 with admin session");
        let cbytes = axum::body::to_bytes(created.into_body(), usize::MAX).await.unwrap();
        let cbody: serde_json::Value = serde_json::from_slice(&cbytes).unwrap();
        let id = cbody["data"]["id"].as_str().unwrap().to_string();
        let listed = crate::router(pool.clone()).oneshot(
            Request::builder()
                .method("GET")
                .uri("/api/bbs/assemble/control/forum/view/all")
                .body(Body::empty()).unwrap(),
        ).await.unwrap();
        assert_eq!(listed.status(), StatusCode::OK, "forum/view/all must be 200");
        let lbytes = axum::body::to_bytes(listed.into_body(), usize::MAX).await.unwrap();
        let lbody: serde_json::Value = serde_json::from_slice(&lbytes).unwrap();
        let arr = lbody["data"].as_array().expect("forum data must be array");
        let found = arr.iter().find(|f| f["id"].as_str() == Some(id.as_str()));
        assert!(found.is_some(), "RED: section/create must bridge to x_bbs_forum with SAME id; id={} not in forum/view/all (count={})", id, arr.len());
        assert_eq!(found.unwrap()["name"].as_str(), Some(marker.as_str()), "bridged forum name must equal section name");
        let c = pool.get().await.unwrap();
        c.execute("DELETE FROM x_bbs_forum WHERE id = $1", &[&id]).await.unwrap();
        c.execute("DELETE FROM x_bbs_assemble_control_section WHERE id = $1", &[&id]).await.unwrap();
    }

    /// R318 t98 实现不变量（防回归）：桥接必须 (a) 复用 section id 写 x_bbs_forum
    /// 避免重复 forum；(b) 写失败时补偿硬删 section，杜绝半状态。
    #[test]
    fn test_section_forum_bridge_invariants() {
        let src = include_str!("lib.rs");
        let start = src.find("pub async fn section_create").expect("section_create");
        let end = src[start..].find("\npub async fn ").map(|i| start + i).unwrap_or(src.len());
        let func = &src[start..end];
        assert!(func.contains("INSERT INTO x_bbs_forum"), "must bridge to x_bbs_forum");
        assert!(func.contains("&[&id, &bridged_name"), "forum id must reuse section id (no duplicate forum)");
        assert!(func.contains("DELETE FROM x_bbs_assemble_control_section WHERE id = $1"), "must compensate section on bridge failure (no half-state)");
        assert!(func.contains("bridge_result.is_err()"), "must fail loud on bridge error");
    }

    /// R320 t102 对称删除契约：section_delete 成功硬删 section 后，
    /// t98 桥接写入的同 id x_bbs_forum 必须被软删（forum/view-all 不再可见）。
    /// live-gated：DB 不可达显式跳过；修复前 forum 仍可见 → 本测试 RED。
    #[tokio::test]
    async fn test_section_delete_symmetric_bridged_forum_cleanup() {
        use shared::testing::{is_db_available, test_pool};
        if !is_db_available().await {
            eprintln!("skipping test_section_delete_symmetric_bridged_forum_cleanup: DB not reachable");
            return;
        }
        let pool = test_pool();
        let marker = format!("r320-delete-{}", uuid::Uuid::new_v4());
        {
            let c = pool.get().await.unwrap();
            c.execute("DELETE FROM auth_person WHERE unique_id = 'r320-delete-admin' AND id <> 'r320-delete-admin'", &[]).await.unwrap();
            c.execute("INSERT INTO auth_person (id, unique_id, name, password_hash, salt, created_at, updated_at) VALUES ('r320-delete-admin', 'r320-delete-admin', 'probe', '', '', NOW(), NOW()) ON CONFLICT (id) DO UPDATE SET unique_id = EXCLUDED.unique_id", &[]).await.unwrap();
            c.execute("INSERT INTO auth_role (id, name, created_at, updated_at) SELECT 'r320-delete-admin-role', 'admin', NOW(), NOW() WHERE NOT EXISTS (SELECT 1 FROM auth_role WHERE name = 'admin')", &[]).await.unwrap();
            c.execute("INSERT INTO auth_person_role (person_id, role_id, unit_id, created_at) SELECT 'r320-delete-admin', r.id, '', NOW() FROM auth_role r WHERE r.name = 'admin' ON CONFLICT DO NOTHING", &[]).await.unwrap();
        }
        let now = chrono::Utc::now().naive_utc();
        let sess = shared::session::Session {
            token: "r320-delete-token".to_string(),
            person_unique: "r320-delete-admin".to_string(),
            created_at: now,
            expires_at: now + chrono::Duration::hours(2),
        };
        let app = crate::router(pool.clone()).layer(axum::extract::Extension(sess));
        let created = app.clone().oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/bbs/assemble/control/section/create")
                .header("content-type", "application/json")
                .body(Body::from(serde_json::json!({ "name": marker }).to_string()))
                .unwrap(),
        ).await.unwrap();
        assert_eq!(created.status(), StatusCode::OK, "section/create must be 200 with admin session");
        let cbytes = axum::body::to_bytes(created.into_body(), usize::MAX).await.unwrap();
        let cbody: serde_json::Value = serde_json::from_slice(&cbytes).unwrap();
        let id = cbody["data"]["id"].as_str().unwrap().to_string();
        // 前置：t98 桥接有效，forum 对该 id 可见
        let before_resp = crate::router(pool.clone()).oneshot(
            Request::builder().method("GET")
                .uri("/api/bbs/assemble/control/forum/view/all")
                .body(Body::empty()).unwrap(),
        ).await.unwrap();
        assert_eq!(before_resp.status(), StatusCode::OK, "forum/view/all must be 200");
        let bb = axum::body::to_bytes(before_resp.into_body(), usize::MAX).await.unwrap();
        let bv: serde_json::Value = serde_json::from_slice(&bb).unwrap();
        let before = bv["data"].as_array().expect("forum data must be array");
        assert!(before.iter().any(|f| f["id"].as_str() == Some(id.as_str())), "precondition: bridged forum must be visible before delete; id={}", id);
        // 动作：删除 section
        let deleted = app.clone().oneshot(
            Request::builder()
                .method("DELETE")
                .uri(format!("/api/bbs/assemble/control/section/delete/{}", id))
                .body(Body::empty()).unwrap(),
        ).await.unwrap();
        assert_eq!(deleted.status(), StatusCode::OK, "section/delete must be 200 with admin session");
        // 后置：同 id forum 必须不再可见
        let after_resp = crate::router(pool.clone()).oneshot(
            Request::builder().method("GET")
                .uri("/api/bbs/assemble/control/forum/view/all")
                .body(Body::empty()).unwrap(),
        ).await.unwrap();
        assert_eq!(after_resp.status(), StatusCode::OK);
        let ab = axum::body::to_bytes(after_resp.into_body(), usize::MAX).await.unwrap();
        let av: serde_json::Value = serde_json::from_slice(&ab).unwrap();
        let after = av["data"].as_array().expect("forum data must be array");
        assert!(after.iter().all(|f| f["id"].as_str() != Some(id.as_str())), "RED: section/delete must soft-delete bridged x_bbs_forum with SAME id; id={} still visible in forum/view/all", id);
        // 数据库回读：deleted_at 非空
        let c = pool.get().await.unwrap();
        let soft: Option<String> = c.query_opt("SELECT deleted_at::text FROM x_bbs_forum WHERE id = $1", &[&id])
            .await.unwrap().and_then(|r| r.get::<_, Option<String>>(0));
        assert!(soft.is_some(), "bridged forum must have deleted_at set after section/delete");
        c.execute("DELETE FROM x_bbs_forum WHERE id = $1", &[&id]).await.unwrap();
        c.execute("DELETE FROM x_bbs_assemble_control_section WHERE id = $1", &[&id]).await.unwrap();
    }

    /// R320 t102 实现不变量：section_delete 必须 (a) 用同 id 软删 x_bbs_forum
    /// deleted_at=NOW()；(b) DB 错误 fail loud；(c) affected=0 不误判。
    #[test]
    fn test_section_delete_bridge_cleanup_invariants() {
        let src = include_str!("lib.rs");
        let start = src.find("pub async fn section_delete").expect("section_delete");
        let end = src[start..].find("\npub async fn ").map(|i| start + i).unwrap_or(src.len());
        let func = &src[start..end];
        assert!(func.contains("UPDATE x_bbs_forum SET deleted_at = NOW()"), "must soft-delete bridged forum");
        assert!(func.contains("&[&id]"), "must reuse section id (no wrong forum)");
        assert!(func.contains("map_err(|e| AppError::internal_with_source"), "must fail loud on DB error");
    }
}
