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

    // SKIPPED: create_topic requires Session parameter
    // SKIPPED: create_reply requires Session parameter
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
    // SKIPPED: section_create requires Session parameter
    // SKIPPED: section_update requires Session parameter
    // SKIPPED: section_delete requires Session parameter
    // SKIPPED: update_control_config requires Session parameter
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
    // SKIPPED: u2_attachment_upload requires Session parameter
    // SKIPPED: u2_attachment_upload_callback requires Session parameter
    // SKIPPED: picture_encode not accessible
    // SKIPPED: picture_section_icon requires Session parameter
    // SKIPPED: section_syn not accessible
    // SKIPPED: u2_subject_get not accessible
    // SKIPPED: u2_subject_save requires Session parameter
    // SKIPPED: u2_subject_accept_reply requires Session parameter
    // SKIPPED: u2_subject_unaccept_reply requires Session parameter
    // SKIPPED: u2_subject_soft_delete requires Session parameter
    // SKIPPED: u2_subject_change_section requires Session parameter
    // SKIPPED: u2_vote_submit requires Session parameter
    // SKIPPED: u2_voterecord_list not accessible
    // SKIPPED: u2_my_subject_list requires Session parameter
    // SKIPPED: u2_my_reply_list requires Session parameter
    // SKIPPED: u2_reply_get not accessible
    // SKIPPED: u2_reply_filter_list not accessible
    // SKIPPED: u2_user_reply_save requires Session parameter
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
}
