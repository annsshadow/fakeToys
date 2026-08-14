#[cfg(test)]
mod tests {
    use axum::body::Body;
    use axum::http::{Request, Method, StatusCode};
    use shared::testing::test_pool;
    use tower::util::ServiceExt;

    #[tokio::test]
    async fn test_list_meeting_controls() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/meeting/assemble/control/list/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "list_meeting_controls route should be registered");
    }

    #[tokio::test]
    async fn test_create_meeting_control() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/meeting/assemble/control/create")
                    .method("POST")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "create_meeting_control route should be registered");
    }

    // SKIPPED: delete_meeting_control not accessible
    #[tokio::test]
    async fn test_building_list() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/meeting/assemble/control/building/list")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "building_list route should be registered");
    }

    #[tokio::test]
    async fn test_building_list_like_pinyin_key() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/meeting/assemble/control/building/list/like/pinyin/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "building_list_like_pinyin_key route should be registered");
    }

    #[tokio::test]
    async fn test_building_list_like_key() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/meeting/assemble/control/building/list/like/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "building_list_like_key route should be registered");
    }

    #[tokio::test]
    async fn test_building_list_pinyininitial_key() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/meeting/assemble/control/building/list/pinyininitial/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "building_list_pinyininitial_key route should be registered");
    }

    // SKIPPED: building_list_start_start_completed_completed not accessible
    // SKIPPED: building_list_start_start_completed_completed_allmeeting not accessible
    // SKIPPED: building_list_start_start_completed_completed_room_room_meeting_meeting not accessible
    #[tokio::test]
    async fn test_building_id() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/meeting/assemble/control/building/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "building_id route should be registered");
    }

    #[tokio::test]
    async fn test_config_system_config() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/meeting/assemble/control/config/system/config")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "config_system_config route should be registered");
    }

    #[tokio::test]
    async fn test_config_system_config_manage() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/meeting/assemble/control/config/system/config/manage")
                    .method("POST")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "config_system_config_manage route should be registered");
    }

    #[tokio::test]
    async fn test_meeting_list_applied_completed() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/meeting/assemble/control/meeting/list/applied/completed")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "meeting_list_applied_completed route should be registered");
    }

    #[tokio::test]
    async fn test_meeting_list_applied_processing() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/meeting/assemble/control/meeting/list/applied/processing")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "meeting_list_applied_processing route should be registered");
    }

    #[tokio::test]
    async fn test_meeting_list_applied_wait() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/meeting/assemble/control/meeting/list/applied/wait")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "meeting_list_applied_wait route should be registered");
    }

    #[tokio::test]
    async fn test_meeting_list_apply_page_size_size() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/meeting/assemble/control/meeting/list/apply/test-id/size/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "meeting_list_apply_page_size_size route should be registered");
    }

    #[tokio::test]
    async fn test_meeting_list_coming_day_count() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/meeting/assemble/control/meeting/list/coming/day/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "meeting_list_coming_day_count route should be registered");
    }

    // SKIPPED: meeting_list_coming_month_count not accessible
    // SKIPPED: meeting_list_forward_monthcount_monthCount not accessible
    // SKIPPED: meeting_list_forward_monthcount_monthCount_all not accessible
    // SKIPPED: meeting_list_invite_page_size_size not accessible
    #[tokio::test]
    async fn test_meeting_list_invited_completed() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/meeting/assemble/control/meeting/list/invited/completed")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "meeting_list_invited_completed route should be registered");
    }

    #[tokio::test]
    async fn test_meeting_list_invited_processing() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/meeting/assemble/control/meeting/list/invited/processing")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "meeting_list_invited_processing route should be registered");
    }

    #[tokio::test]
    async fn test_meeting_list_invited_rejected() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/meeting/assemble/control/meeting/list/invited/rejected")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "meeting_list_invited_rejected route should be registered");
    }

    #[tokio::test]
    async fn test_meeting_list_invited_wait() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/meeting/assemble/control/meeting/list/invited/wait")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "meeting_list_invited_wait route should be registered");
    }

    #[tokio::test]
    async fn test_meeting_list_wait_accept() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/meeting/assemble/control/meeting/list/wait/accept")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "meeting_list_wait_accept route should be registered");
    }

    #[tokio::test]
    async fn test_meeting_list_wait_confirm() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/meeting/assemble/control/meeting/list/wait/confirm")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "meeting_list_wait_confirm route should be registered");
    }

    #[tokio::test]
    async fn test_meeting_list_year_year_month_month() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/meeting/assemble/control/meeting/list/year/test-id/month/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "meeting_list_year_year_month_month route should be registered");
    }

    #[tokio::test]
    async fn test_meeting_list_year_year_month_month_all() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/meeting/assemble/control/meeting/list/year/test-id/month/test-id/all")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "meeting_list_year_year_month_month_all route should be registered");
    }

    #[tokio::test]
    async fn test_meeting_list_year_year_month_month_day_day() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/meeting/assemble/control/meeting/list/year/test-id/month/test-id/day/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "meeting_list_year_year_month_month_day_day route should be registered");
    }

    #[tokio::test]
    async fn test_meeting_list_year_year_month_month_day_day_all() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/meeting/assemble/control/meeting/list/year/test-id/month/test-id/day/test-id/all")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "meeting_list_year_year_month_month_day_day_all route should be registered");
    }

    // SKIPPED: meeting_list_year_year_month_month_day_day_roomId not accessible
    // SKIPPED: meeting_list_id_next_count not accessible
    // SKIPPED: meeting_list_id_prev_count not accessible
    // SKIPPED: meeting_list_page_size_size not accessible
    // SKIPPED: meeting_list_page_size_size_manage not accessible
    #[tokio::test]
    async fn test_meeting_id() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/meeting/assemble/control/meeting/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "meeting_id route should be registered");
    }

    #[tokio::test]
    async fn test_create_meeting() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/meeting/assemble/control/meeting/create")
                    .method("POST")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "create_meeting route should be registered");
    }

    #[tokio::test]
    async fn test_save_meeting() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/meeting/assemble/control/meeting/save/test-id")
                    .method("POST")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "save_meeting route should be registered");
    }

    #[tokio::test]
    async fn test_delete_meeting() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/meeting/assemble/control/meeting/delete/test-id")
                    .method("POST")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "delete_meeting route should be registered");
    }

    #[tokio::test]
    async fn test_meeting_id_accept() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/meeting/assemble/control/meeting/test-id/accept")
                    .method("POST")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "meeting_id_accept route should be registered");
    }

    #[tokio::test]
    async fn test_meeting_id_add_invite() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/meeting/assemble/control/meeting/test-id/add/invite")
                    .method("POST")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "meeting_id_add_invite route should be registered");
    }

    #[tokio::test]
    async fn test_meeting_id_checkin() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/meeting/assemble/control/meeting/test-id/checkin")
                    .method("POST")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "meeting_id_checkin route should be registered");
    }

    // SKIPPED: meeting_id_checkin_code not accessible
    #[tokio::test]
    async fn test_meeting_id_confirm_allow() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/meeting/assemble/control/meeting/test-id/confirm/allow")
                    .method("POST")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "meeting_id_confirm_allow route should be registered");
    }

    #[tokio::test]
    async fn test_meeting_id_confirm_deny() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/meeting/assemble/control/meeting/test-id/confirm/deny")
                    .method("POST")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "meeting_id_confirm_deny route should be registered");
    }

    #[tokio::test]
    async fn test_meeting_id_delete_invite() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/meeting/assemble/control/meeting/test-id/delete/invite")
                    .method("POST")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "meeting_id_delete_invite route should be registered");
    }

    #[tokio::test]
    async fn test_meeting_id_manual_completed() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/meeting/assemble/control/meeting/test-id/manual/completed")
                    .method("POST")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "meeting_id_manual_completed route should be registered");
    }

    #[tokio::test]
    async fn test_meeting_id_modify_completedtime() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/meeting/assemble/control/meeting/test-id/modify/completedtime")
                    .method("POST")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "meeting_id_modify_completedtime route should be registered");
    }

    #[tokio::test]
    async fn test_meeting_id_modify_starttime() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/meeting/assemble/control/meeting/test-id/modify/starttime")
                    .method("POST")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "meeting_id_modify_starttime route should be registered");
    }

    #[tokio::test]
    async fn test_meeting_id_reject() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/meeting/assemble/control/meeting/test-id/reject")
                    .method("POST")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "meeting_id_reject route should be registered");
    }

    #[tokio::test]
    async fn test_openmeeting_list_room() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/meeting/assemble/control/openmeeting/list/room")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "openmeeting_list_room route should be registered");
    }

    #[tokio::test]
    async fn test_room_list() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/meeting/assemble/control/room/list")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "room_list route should be registered");
    }

    // SKIPPED: room_list_like_pinyin_key not accessible
    // SKIPPED: room_list_like_key not accessible
    // SKIPPED: room_list_pinyininitial_key not accessible
    #[tokio::test]
    async fn test_room_id() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/meeting/assemble/control/room/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "room_id route should be registered");
    }

    // SKIPPED: room_id_photo not accessible
}