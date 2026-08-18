#[cfg(test)]
mod tests {
    use axum::body::Body;
    use axum::http::{Request, Method, StatusCode};
    use shared::testing::test_pool;
    use tower::util::ServiceExt;

    #[tokio::test]
    async fn test_get_control_config() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/bbs/assemble/control/config")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "get_control_config route should be registered");
    }

    #[tokio::test]
    async fn test_list_control_sections() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/bbs/assemble/control/section/list")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "list_control_sections route should be registered");
    }

    #[tokio::test]
    async fn test_update_control_config() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/bbs/assemble/control/update/control/config")
                    .method("POST")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "update_control_config route should be registered");
    }

    #[tokio::test]
    async fn test_list_forums() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/bbs/assemble/control/forum/list")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "list_forums route should be registered");
    }

    #[tokio::test]
    async fn test_get_forum() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/bbs/assemble/control/forum/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "get_forum route should be registered");
    }

    #[tokio::test]
    async fn test_create_topic() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/bbs/assemble/control/topic/create")
                    .method("POST")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "create_topic route should be registered");
    }

    #[tokio::test]
    async fn test_list_topics_by_forum() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/bbs/assemble/control/topic/list/forum/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "list_topics_by_forum route should be registered");
    }

    #[tokio::test]
    async fn test_create_reply() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/bbs/assemble/control/reply/create")
                    .method("POST")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "create_reply route should be registered");
    }

    #[tokio::test]
    async fn test_forum_view_all() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/bbs/assemble/control/forum/view/all")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "forum_view_all route should be registered");
    }

    // SKIPPED: forum_id not accessible
    // SKIPPED: mobile_view_all not accessible
    // SKIPPED: permission_replyPublishable_subjectId not accessible
    // SKIPPED: permission_subjectPublishable_sectionId not accessible
    // SKIPPED: reply_filter_list_page_page_count_count not accessible
    #[tokio::test]
    async fn test_reply_list_sub_id() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/bbs/assemble/control/reply/list/sub/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "reply_list_sub_id route should be registered");
    }

    #[tokio::test]
    async fn test_subject_view_id() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/bbs/assemble/control/subject/view/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "subject_view_id route should be registered");
    }

    #[tokio::test]
    async fn test_subject_top_sectionId() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/bbs/assemble/control/subject/top/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "subject_top_sectionId route should be registered");
    }

    #[tokio::test]
    async fn test_permission_section_sectionId() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/bbs/assemble/control/permission/section/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "permission_section_sectionId route should be registered");
    }

    #[tokio::test]
    async fn test_permission_subject_subjectId() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/bbs/assemble/control/permission/subject/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "permission_subject_subjectId route should be registered");
    }

    #[tokio::test]
    async fn test_section_viewforum_forumId() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/bbs/assemble/control/section/viewforum/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "section_viewforum_forumId route should be registered");
    }

    // SKIPPED: delete_forum not accessible
    // SKIPPED: delete_reply not accessible
    // SKIPPED: delete_subject not accessible
    // SKIPPED: list_reply_filter not accessible
    // SKIPPED: list_topics_creamed not accessible
    // SKIPPED: list_topics_recommended not accessible
    // SKIPPED: list_subjects_filtered not accessible
    // SKIPPED: list_subjects_index not accessible
    // SKIPPED: list_subjects_recommended_index not accessible
    // SKIPPED: login not accessible
    // SKIPPED: logout not accessible
    // SKIPPED: picture_list not accessible
    #[tokio::test]
    async fn test_shutup_create() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/bbs/assemble/control/shutup/create")
                    .method("POST")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "shutup_create route should be registered");
    }

    // SKIPPED: shutup_delete not accessible
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
    // SKIPPED: user_info not accessible
    // SKIPPED: user_reply_list not accessible
    // SKIPPED: user_role_list not accessible
    // SKIPPED: user_section_list not accessible
    // SKIPPED: user_setting not accessible
    // SKIPPED: user_subject_list not accessible
    #[tokio::test]
    async fn test_uuid_generate() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/bbs/assemble/control/uuid")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "uuid_generate route should be registered");
    }

    // SKIPPED: subjectattach_list not accessible
}