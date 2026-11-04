// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

#[cfg(test)]
mod tests {
    #![allow(non_snake_case)]
    use axum::body::Body;
    use axum::http::{Request, StatusCode};

    use tower::util::ServiceExt;

    #[tokio::test]
    async fn test_list_meeting_controls() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/meeting/assemble/control/list/test-id")
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
                "list_meeting_controls route should be registered"
            );
        }
    }

    // SKIPPED: create_meeting_control requires Session parameter
    // SKIPPED: delete_meeting_control requires Session parameter
    #[tokio::test]
    async fn test_building_list() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/meeting/assemble/control/building/list")
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
                "building_list route should be registered"
            );
        }
    }

    #[tokio::test]
    async fn test_building_list_like_pinyin_key() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/meeting/assemble/control/building/list/like/pinyin/test-id")
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
                "building_list_like_pinyin_key route should be registered"
            );
        }
    }

    #[tokio::test]
    async fn test_building_list_like_key() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/meeting/assemble/control/building/list/like/test-id")
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
                "building_list_like_key route should be registered"
            );
        }
    }

    #[tokio::test]
    async fn test_building_list_pinyininitial_key() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/meeting/assemble/control/building/list/pinyininitial/test-id")
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
                "building_list_pinyininitial_key route should be registered"
            );
        }
    }

    #[tokio::test]
    async fn test_building_id() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/meeting/assemble/control/building/test-id")
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
            assert!(!bytes.is_empty(), "building_id route should be registered");
        }
    }

    #[tokio::test]
    async fn test_config_system_config() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/meeting/assemble/control/config/system/config")
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
                "config_system_config route should be registered"
            );
        }
    }

    // SKIPPED: config_system_config_manage requires Session parameter
    #[tokio::test]
    async fn test_meeting_list_applied_completed() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/meeting/assemble/control/meeting/list/applied/completed")
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
                "meeting_list_applied_completed route should be registered"
            );
        }
    }

    #[tokio::test]
    async fn test_meeting_list_applied_processing() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/meeting/assemble/control/meeting/list/applied/processing")
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
                "meeting_list_applied_processing route should be registered"
            );
        }
    }

    #[tokio::test]
    async fn test_meeting_list_applied_wait() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/meeting/assemble/control/meeting/list/applied/wait")
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
                "meeting_list_applied_wait route should be registered"
            );
        }
    }

    #[tokio::test]
    async fn test_meeting_list_apply_page_size_size() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/meeting/assemble/control/meeting/list/apply/test-id/size/test-id")
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
                "meeting_list_apply_page_size_size route should be registered"
            );
        }
    }

    #[tokio::test]
    async fn test_meeting_list_coming_day_count() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/meeting/assemble/control/meeting/list/coming/day/test-id")
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
                "meeting_list_coming_day_count route should be registered"
            );
        }
    }

    #[tokio::test]
    async fn test_meeting_list_invited_completed() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/meeting/assemble/control/meeting/list/invited/completed")
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
                "meeting_list_invited_completed route should be registered"
            );
        }
    }

    #[tokio::test]
    async fn test_meeting_list_invited_processing() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/meeting/assemble/control/meeting/list/invited/processing")
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
                "meeting_list_invited_processing route should be registered"
            );
        }
    }

    #[tokio::test]
    async fn test_meeting_list_invited_rejected() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/meeting/assemble/control/meeting/list/invited/rejected")
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
                "meeting_list_invited_rejected route should be registered"
            );
        }
    }

    #[tokio::test]
    async fn test_meeting_list_invited_wait() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/meeting/assemble/control/meeting/list/invited/wait")
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
                "meeting_list_invited_wait route should be registered"
            );
        }
    }

    #[tokio::test]
    async fn test_meeting_list_wait_accept() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/meeting/assemble/control/meeting/list/wait/accept")
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
                "meeting_list_wait_accept route should be registered"
            );
        }
    }

    #[tokio::test]
    async fn test_meeting_list_wait_confirm() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/meeting/assemble/control/meeting/list/wait/confirm")
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
                "meeting_list_wait_confirm route should be registered"
            );
        }
    }

    #[tokio::test]
    async fn test_meeting_list_year_year_month_month() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/meeting/assemble/control/meeting/list/year/test-id/month/test-id")
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
                "meeting_list_year_year_month_month route should be registered"
            );
        }
    }

    #[tokio::test]
    async fn test_meeting_list_year_year_month_month_all() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri(
                        "/api/meeting/assemble/control/meeting/list/year/test-id/month/test-id/all",
                    )
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
                "meeting_list_year_year_month_month_all route should be registered"
            );
        }
    }

    #[tokio::test]
    async fn test_meeting_list_year_year_month_month_day_day() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/meeting/assemble/control/meeting/list/year/test-id/month/test-id/day/test-id")
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
                "meeting_list_year_year_month_month_day_day route should be registered"
            );
        }
    }

    #[tokio::test]
    async fn test_meeting_list_year_year_month_month_day_day_all() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/meeting/assemble/control/meeting/list/year/test-id/month/test-id/day/test-id/all")
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
                "meeting_list_year_year_month_month_day_day_all route should be registered"
            );
        }
    }

    #[tokio::test]
    async fn test_meeting_id() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/meeting/assemble/control/meeting/test-id")
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
            assert!(!bytes.is_empty(), "meeting_id route should be registered");
        }
    }

    // SKIPPED: create_meeting requires Session parameter
    // SKIPPED: save_meeting requires Session parameter
    // SKIPPED: delete_meeting requires Session parameter
    // SKIPPED: meeting_id_accept requires Session parameter
    // SKIPPED: meeting_id_add_invite requires Session parameter
    // SKIPPED: meeting_id_checkin requires Session parameter
    // SKIPPED: meeting_id_confirm_allow requires Session parameter
    // SKIPPED: meeting_id_confirm_deny requires Session parameter
    // SKIPPED: meeting_id_delete_invite requires Session parameter
    // SKIPPED: meeting_id_manual_completed requires Session parameter
    // SKIPPED: meeting_id_modify_completedtime requires Session parameter
    // SKIPPED: meeting_id_modify_starttime requires Session parameter
    // SKIPPED: meeting_id_reject requires Session parameter
    #[tokio::test]
    async fn test_openmeeting_list_room() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/meeting/assemble/control/openmeeting/list/room")
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
                "openmeeting_list_room route should be registered"
            );
        }
    }

    #[tokio::test]
    async fn test_room_list() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/meeting/assemble/control/room/list")
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
            assert!(!bytes.is_empty(), "room_list route should be registered");
        }
    }

    #[tokio::test]
    async fn test_room_id() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/meeting/assemble/control/room/test-id")
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
            assert!(!bytes.is_empty(), "room_id route should be registered");
        }
    }

    // SKIPPED: building_list_start_start_completed_completed not accessible
    // SKIPPED: building_list_start_start_completed_completed_allmeeting not accessible
    // SKIPPED: building_list_start_start_completed_completed_room_room_meeting_meeting not accessible
    // SKIPPED: meeting_list_coming_month_count not accessible
    // SKIPPED: meeting_list_coming_month_count_p2 not accessible
    // SKIPPED: meeting_list_forward_monthcount_monthCount not accessible
    // SKIPPED: meeting_list_forward_monthcount_monthCount_all not accessible
    // SKIPPED: meeting_list_invite_page_size_size not accessible
    // SKIPPED: meeting_list_invite_page_size_size_p3 not accessible
    // SKIPPED: meeting_list_year_year_month_month_day_day_roomId not accessible
    // SKIPPED: meeting_list_year_year_month_month_day_day_roomId_p7 not accessible
    // SKIPPED: meeting_list_id_next_count not accessible
    // SKIPPED: meeting_list_id_next_count_p3 not accessible
    // SKIPPED: meeting_list_id_prev_count not accessible
    // SKIPPED: meeting_list_page_size_size not accessible
    // SKIPPED: meeting_list_page_size_size_manage not accessible
    // SKIPPED: meeting_list_page_size_size_manage_p3 not accessible
    // SKIPPED: meeting_id_checkin_code not accessible
    // SKIPPED: room_list_like_pinyin_key not accessible
    // SKIPPED: room_list_like_key not accessible
    // SKIPPED: room_list_pinyininitial_key not accessible
    // SKIPPED: room_id_photo not accessible
    // SKIPPED: u2_attachment_list_with_meeting not accessible
    // SKIPPED: u2_attachment_get not accessible
    // SKIPPED: u2_attachment_download requires Session parameter
    // SKIPPED: u2_attachment_update requires Session parameter
    // SKIPPED: u2_attachment_update_callback requires Session parameter
    // SKIPPED: u2_attachment_create_from_processplatform requires Session parameter
    // SKIPPED: u2_attachment_list_next not accessible
    // SKIPPED: u2_attachment_list_prev not accessible
    // SKIPPED: u2_attachment_upload requires Session parameter
    // SKIPPED: u2_attachment_upload_callback requires Session parameter
    // SKIPPED: u2_attachment_delete requires Session parameter
    // SKIPPED: u2_building_create requires Session parameter
    // SKIPPED: u2_building_edit requires Session parameter
    // SKIPPED: u2_building_delete requires Session parameter
    // SKIPPED: u2_config_save requires Session parameter
    // SKIPPED: u2_config_manage_get not accessible
    // SKIPPED: u2_meeting_delete_owned requires Session parameter
    // SKIPPED: u2_meeting_modify requires Session parameter
    // SKIPPED: u2_meeting_put_save requires Session parameter
    // SKIPPED: u2_meeting_checkin_get requires Session parameter
    // SKIPPED: u2_openmeeting_get not accessible
    // SKIPPED: u2_room_create requires Session parameter
    // SKIPPED: u2_room_edit requires Session parameter
    // SKIPPED: u2_room_delete requires Session parameter
    // SKIPPED: u2_room_set_photo requires Session parameter
}
