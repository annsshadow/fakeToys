// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

#[cfg(test)]
mod tests {
    #![allow(non_snake_case)]
    use axum::body::Body;
    use axum::http::{Request, StatusCode};

    use tower::util::ServiceExt;

    #[tokio::test]
    async fn test_unread_count() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/message/unread/count")
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
            assert!(!bytes.is_empty(), "unread_count route should be registered");
        }
    }

    #[tokio::test]
    async fn test_send_message() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/message/assemble/communicate/send")
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
            assert!(!bytes.is_empty(), "send_message route should be registered");
        }
    }

    #[tokio::test]
    async fn test_receive_list() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/message/assemble/communicate/receive/test-id")
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
            assert!(!bytes.is_empty(), "receive_list route should be registered");
        }
    }

    #[tokio::test]
    async fn test_mark_read() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/message/assemble/communicate/mark_read/test-id")
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
            assert!(!bytes.is_empty(), "mark_read route should be registered");
        }
    }

    #[tokio::test]
    async fn test_consume_list_consume_count_count() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/message/assemble/communicate/consume/list/test-id/count/test-id")
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
                "consume_list_consume_count_count route should be registered"
            );
        }
    }

    #[tokio::test]
    async fn test_consume_list_consume_currentperson_count_count() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/message/assemble/communicate/consume/list/test-id/currentperson/count/test-id")
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
                "consume_list_consume_currentperson_count_count route should be registered"
            );
        }
    }

    #[tokio::test]
    async fn test_consume_list_consume_person_person_count_count() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/message/assemble/communicate/consume/list/test-id/person/test-id/count/test-id")
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
                "consume_list_consume_person_person_count_count route should be registered"
            );
        }
    }

    #[tokio::test]
    async fn test_consume_type_type() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/message/assemble/communicate/consume/type/test-id")
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
                "consume_type_type route should be registered"
            );
        }
    }

    #[tokio::test]
    async fn test_consume_type_type_mockputtopost() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/message/assemble/communicate/consume/type/test-id/mockputtopost")
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
            assert!(
                !bytes.is_empty(),
                "consume_type_type_mockputtopost route should be registered"
            );
        }
    }

    #[tokio::test]
    async fn test_consume_id_type_type() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/message/assemble/communicate/consume/test-id/type/test-id")
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
                "consume_id_type_type route should be registered"
            );
        }
    }

    #[tokio::test]
    async fn test_im_conversation() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/message/assemble/communicate/im/conversation")
                    .method("PUT")
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
                "im_conversation route should be registered"
            );
        }
    }

    #[tokio::test]
    async fn test_im_conversation_business_businessId() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/message/assemble/communicate/im/conversation/business/test-id")
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
                "im_conversation_business_businessId route should be registered"
            );
        }
    }

    #[tokio::test]
    async fn test_im_conversation_list_my() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/message/assemble/communicate/im/conversation/list/my")
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
                "im_conversation_list_my route should be registered"
            );
        }
    }

    #[tokio::test]
    async fn test_im_conversation_list_with_person() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/message/assemble/communicate/im/conversation/list/with/person")
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
            assert!(
                !bytes.is_empty(),
                "im_conversation_list_with_person route should be registered"
            );
        }
    }

    #[tokio::test]
    async fn test_im_conversation_mockputtopost() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/message/assemble/communicate/im/conversation/mockputtopost")
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
            assert!(
                !bytes.is_empty(),
                "im_conversation_mockputtopost route should be registered"
            );
        }
    }

    #[tokio::test]
    async fn test_im_conversation_id() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/message/assemble/communicate/im/conversation/test-id")
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
                "im_conversation_id route should be registered"
            );
        }
    }

    #[tokio::test]
    async fn test_im_conversation_id_group() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/message/assemble/communicate/im/conversation/test-id/group")
                    .method("DELETE")
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
                "im_conversation_id_group route should be registered"
            );
        }
    }

    #[tokio::test]
    async fn test_im_manager_config_post() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/message/assemble/communicate/im/manager/config")
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
            assert!(
                !bytes.is_empty(),
                "im_manager_config_post route should be registered"
            );
        }
    }

    // SKIPPED: im_conversation_update requires Session parameter
    #[tokio::test]
    async fn test_im_conversation_id_group_mockdeletetoget() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/message/assemble/communicate/im/conversation/test-id/group/mockdeletetoget")
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
                "im_conversation_id_group_mockdeletetoget route should be registered"
            );
        }
    }

    // SKIPPED: im_conversation_id_group_quit_self requires Session parameter
    #[tokio::test]
    async fn test_im_conversation_id_icon() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/message/assemble/communicate/im/conversation/test-id/icon")
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
                "im_conversation_id_icon route should be registered"
            );
        }
    }

    // SKIPPED: im_conversation_id_read requires Session parameter
    #[tokio::test]
    async fn test_im_conversation_id_read_mockputtopost() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/message/assemble/communicate/im/conversation/test-id/read/mockputtopost")
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
            assert!(
                !bytes.is_empty(),
                "im_conversation_id_read_mockputtopost route should be registered"
            );
        }
    }

    #[tokio::test]
    async fn test_im_conversation_id_single() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/message/assemble/communicate/im/conversation/test-id/single")
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
                "im_conversation_id_single route should be registered"
            );
        }
    }

    #[tokio::test]
    async fn test_im_conversation_id_top_cancel() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/message/assemble/communicate/im/conversation/test-id/top/cancel")
                    .method("PUT")
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
                "im_conversation_id_top_cancel route should be registered"
            );
        }
    }

    #[tokio::test]
    async fn test_im_conversation_id_top_cancel_mockputtopost() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/message/assemble/communicate/im/conversation/test-id/top/cancel/mockputtopost")
                    .method("PUT")
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
                "im_conversation_id_top_cancel_mockputtopost route should be registered"
            );
        }
    }

    #[tokio::test]
    async fn test_im_conversation_id_top_set() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/message/assemble/communicate/im/conversation/test-id/top/set")
                    .method("PUT")
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
                "im_conversation_id_top_set route should be registered"
            );
        }
    }

    #[tokio::test]
    async fn test_im_conversation_id_top_set_mockputtopost() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/message/assemble/communicate/im/conversation/test-id/top/set/mockputtopost")
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
            assert!(
                !bytes.is_empty(),
                "im_conversation_id_top_set_mockputtopost route should be registered"
            );
        }
    }

    #[tokio::test]
    async fn test_im_manager_config() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/message/assemble/communicate/im/manager/config")
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
                "im_manager_config route should be registered"
            );
        }
    }

    #[tokio::test]
    async fn test_im_msg() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/message/assemble/communicate/im/msg")
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
            assert!(!bytes.is_empty(), "im_msg route should be registered");
        }
    }

    #[tokio::test]
    async fn test_im_msg_clear() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/message/assemble/communicate/im/msg/clear")
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
            assert!(!bytes.is_empty(), "im_msg_clear route should be registered");
        }
    }

    #[tokio::test]
    async fn test_im_msg_collection() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/message/assemble/communicate/im/msg/collection")
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
            assert!(
                !bytes.is_empty(),
                "im_msg_collection route should be registered"
            );
        }
    }

    #[tokio::test]
    async fn test_im_msg_collection_list_page_size_size() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/message/assemble/communicate/im/msg/collection/list/test-id/size/test-id")
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
            assert!(
                !bytes.is_empty(),
                "im_msg_collection_list_page_size_size route should be registered"
            );
        }
    }

    #[tokio::test]
    async fn test_im_msg_collection_remove() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/message/assemble/communicate/im/msg/collection/remove")
                    .method("DELETE")
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
                "im_msg_collection_remove route should be registered"
            );
        }
    }

    #[tokio::test]
    async fn test_im_msg_download_id() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/message/assemble/communicate/im/msg/download/test-id")
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
                "im_msg_download_id route should be registered"
            );
        }
    }

    #[tokio::test]
    async fn test_im_msg_download_id_image_width_width_height_height() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/message/assemble/communicate/im/msg/download/test-id/image/width/test-id/height/test-id")
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
                "im_msg_download_id_image_width_width_height_height route should be registered"
            );
        }
    }

    #[tokio::test]
    async fn test_im_msg_list_object() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/message/assemble/communicate/im/msg/list/object")
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
            assert!(
                !bytes.is_empty(),
                "im_msg_list_object route should be registered"
            );
        }
    }

    #[tokio::test]
    async fn test_im_msg_list_page_size_size() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/message/assemble/communicate/im/msg/list/test-id/size/test-id")
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
            assert!(
                !bytes.is_empty(),
                "im_msg_list_page_size_size route should be registered"
            );
        }
    }

    #[tokio::test]
    async fn test_im_msg_revoke_id() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/message/assemble/communicate/im/msg/revoke/test-id")
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
                "im_msg_revoke_id route should be registered"
            );
        }
    }

    #[tokio::test]
    async fn test_im_msg_upload_conversationId_type_type() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/message/assemble/communicate/im/msg/upload/test-id/type/test-id")
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
            assert!(
                !bytes.is_empty(),
                "im_msg_upload_conversationId_type_type route should be registered"
            );
        }
    }

    #[tokio::test]
    async fn test_instant_currentperson_consumed() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/message/assemble/communicate/instant/currentperson/consumed")
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
                "instant_currentperson_consumed route should be registered"
            );
        }
    }

    #[tokio::test]
    async fn test_instant_currentperson_consumed_all() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/message/assemble/communicate/instant/currentperson/consumed/all")
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
                "instant_currentperson_consumed_all route should be registered"
            );
        }
    }

    #[tokio::test]
    async fn test_instant_currentperson_consumed_mockputtopost() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/message/assemble/communicate/instant/currentperson/consumed/mockputtopost")
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
            assert!(
                !bytes.is_empty(),
                "instant_currentperson_consumed_mockputtopost route should be registered"
            );
        }
    }

    #[tokio::test]
    async fn test_instant_list_currentperson_consumed_count_count_asc() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/message/assemble/communicate/instant/list/currentperson/consumed/count/test-id/asc")
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
                "instant_list_currentperson_consumed_count_count_asc route should be registered"
            );
        }
    }

    #[tokio::test]
    async fn test_instant_list_currentperson_consumed_count_count_desc() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/message/assemble/communicate/instant/list/currentperson/consumed/count/test-id/desc")
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
                "instant_list_currentperson_consumed_count_count_desc route should be registered"
            );
        }
    }

    #[tokio::test]
    async fn test_instant_list_currentperson_count_count_asc() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/message/assemble/communicate/instant/list/currentperson/count/test-id/asc")
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
                "instant_list_currentperson_count_count_asc route should be registered"
            );
        }
    }

    #[tokio::test]
    async fn test_instant_list_currentperson_count_count_desc() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/message/assemble/communicate/instant/list/currentperson/count/test-id/desc")
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
                "instant_list_currentperson_count_count_desc route should be registered"
            );
        }
    }

    #[tokio::test]
    async fn test_instant_list_currentperson_noim_count_count_desc() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/message/assemble/communicate/instant/list/currentperson/noim/count/test-id/desc")
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
                "instant_list_currentperson_noim_count_count_desc route should be registered"
            );
        }
    }

    #[tokio::test]
    async fn test_instant_list_currentperson_not_consumed_count_count_asc() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/message/assemble/communicate/instant/list/currentperson/not/consumed/count/test-id/asc")
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
            assert!(!bytes.is_empty(),
                "instant_list_currentperson_not_consumed_count_count_asc route should be registered");
        }
    }

    #[tokio::test]
    async fn test_instant_list_currentperson_not_consumed_count_count_desc() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/message/assemble/communicate/instant/list/currentperson/not/consumed/count/test-id/desc")
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
            assert!(!bytes.is_empty(),
                "instant_list_currentperson_not_consumed_count_count_desc route should be registered");
        }
    }

    #[tokio::test]
    async fn test_instant_list_id_next_count() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/message/assemble/communicate/instant/list/test-id/next/test-id")
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
                "instant_list_id_next_count route should be registered"
            );
        }
    }

    #[tokio::test]
    async fn test_instant_list_id_prev_count() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/message/assemble/communicate/instant/list/test-id/prev/test-id")
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
                "instant_list_id_prev_count route should be registered"
            );
        }
    }

    #[tokio::test]
    async fn test_mass_enable_type() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/message/assemble/communicate/mass/enable/type")
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
            assert!(
                !bytes.is_empty(),
                "mass_enable_type route should be registered"
            );
        }
    }

    #[tokio::test]
    async fn test_mass_list_id_next_count() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/message/assemble/communicate/mass/list/test-id/next/test-id")
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
                "mass_list_id_next_count route should be registered"
            );
        }
    }

    #[tokio::test]
    async fn test_mass_list_id_prev_count() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/message/assemble/communicate/mass/list/test-id/prev/test-id")
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
                "mass_list_id_prev_count route should be registered"
            );
        }
    }

    #[tokio::test]
    async fn test_mass_id() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/message/assemble/communicate/mass/test-id")
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
            assert!(!bytes.is_empty(), "mass_id route should be registered");
        }
    }

    #[tokio::test]
    async fn test_message_custom_create() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/message/assemble/communicate/message/custom/create")
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
            assert!(
                !bytes.is_empty(),
                "message_custom_create route should be registered"
            );
        }
    }

    #[tokio::test]
    async fn test_message_list_paging_page_size_size() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/message/assemble/communicate/message/list/paging/test-id/size/test-id")
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
            assert!(
                !bytes.is_empty(),
                "message_list_paging_page_size_size route should be registered"
            );
        }
    }

    #[tokio::test]
    async fn test_connector_create() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/message/assemble/communicate/connector")
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
            assert!(
                !bytes.is_empty(),
                "connector_create route should be registered"
            );
        }
    }

    #[tokio::test]
    async fn test_ws_create() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/message/assemble/communicate/ws")
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
            assert!(!bytes.is_empty(), "ws_create route should be registered");
        }
    }

    #[tokio::test]
    async fn test_ws_count_person() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/message/assemble/communicate/ws/count/person")
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
                "ws_count_person route should be registered"
            );
        }
    }

    #[tokio::test]
    async fn test_ws_list_person_current_node() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/message/assemble/communicate/ws/list/person/current/node")
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
                "ws_list_person_current_node route should be registered"
            );
        }
    }

    #[tokio::test]
    async fn test_ws_list_person() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/message/assemble/communicate/ws/list/person")
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
                "ws_list_person route should be registered"
            );
        }
    }

    // SKIPPED: mass_create requires Session parameter
    #[tokio::test]
    async fn test_mass_enable_type_get() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/message/assemble/communicate/mass/enable/type")
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
                "mass_enable_type_get route should be registered"
            );
        }
    }

    /// R318 M1 行为回归：非 admin 会话人创建不带 sender 的消息后，本人分页列表必须可见。
    /// 写读同源：缺省 sender 应落会话人 person_unique；修复前落 "system" 不匹配读谓词。
    #[tokio::test]
    async fn r318_m1_custom_create_default_sender_is_self_visible() {
        if !shared::testing::is_db_available().await {
            eprintln!("skipping r318_m1_custom_create_default_sender_is_self_visible: DB not reachable");
            return;
        }
        let pool = shared::testing::test_pool();
        let c = pool.get().await.unwrap();
        c.execute(
            "CREATE TABLE IF NOT EXISTS x_message (id TEXT, conversation_id TEXT, content TEXT, sender TEXT, type TEXT, create_time TEXT)",
            &[],
        ).await.unwrap();
        c.execute(
            "CREATE TABLE IF NOT EXISTS x_message_conversation (id TEXT, name TEXT, type TEXT, create_time TEXT)",
            &[],
        ).await.unwrap();
        c.execute(
            "ALTER TABLE x_message_conversation ADD COLUMN IF NOT EXISTS last_message_time TEXT",
            &[],
        ).await.unwrap();
        c.execute(
            "CREATE TABLE IF NOT EXISTS x_message_conversation_member (id TEXT, conversation_id TEXT, person_id TEXT, join_time TEXT, create_time TEXT)",
            &[],
        ).await.unwrap();

        let conv_id = "r318-m1-sender-conv-001";
        let actor = "r318-m1-sender-dave";
        let marker = "r318-m1-default-sender-marker";
        c.execute("DELETE FROM x_message WHERE conversation_id = $1", &[&conv_id]).await.unwrap();
        c.execute("DELETE FROM x_message_conversation_member WHERE conversation_id = $1", &[&conv_id]).await.unwrap();
        c.execute("DELETE FROM x_message_conversation WHERE id = $1", &[&conv_id]).await.unwrap();
        c.execute(
            "INSERT INTO x_message_conversation (id, name, type) VALUES ($1, 'conv', 'single')",
            &[&conv_id],
        ).await.unwrap();
        c.execute(
            "INSERT INTO x_message_conversation_member (id, conversation_id, person_id) VALUES ($1, $2, $3)",
            &[&"r318-m1-member-001".to_string(), &conv_id.to_string(), &actor.to_string()],
        ).await.unwrap();

        let app = crate::router(pool.clone())
            .layer(axum::extract::Extension(shared::testing::test_session(actor)));
        let create_resp = app.clone()
            .oneshot(
                Request::builder()
                    .uri("/api/message/assemble/communicate/message/custom/create")
                    .method(axum::http::Method::POST)
                    .header("content-type", "application/json")
                    .body(Body::from(serde_json::to_vec(&serde_json::json!({
                        "conversationId": conv_id,
                        "content": marker
                    })).unwrap()))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(create_resp.status(), StatusCode::OK, "custom/create 应成功");
        let create_bytes = axum::body::to_bytes(create_resp.into_body(), 65536).await.unwrap();
        let create_json: serde_json::Value = serde_json::from_slice(&create_bytes).unwrap();
        assert_eq!(create_json["type"], "success", "创建响应应为成功信封: {create_json}");

        let list_resp = app
            .oneshot(
                Request::builder()
                    .uri("/api/message/assemble/communicate/message/list/paging/1/size/50")
                    .method(axum::http::Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(list_resp.status(), StatusCode::OK);
        let list_bytes = axum::body::to_bytes(list_resp.into_body(), 65536).await.unwrap();
        let list_json: serde_json::Value = serde_json::from_slice(&list_bytes).unwrap();
        let visible = list_json["data"].as_array()
            .map(|a| a.iter().any(|m| m["content"].as_str() == Some(marker)))
            .unwrap_or(false);
        assert!(
            visible,
            "缺省 sender 消息应被创建人本人列表可见（修复前落 system 不匹配读谓词）: {list_json}"
        );

        c.execute("DELETE FROM x_message WHERE conversation_id = $1", &[&conv_id]).await.unwrap();
        c.execute("DELETE FROM x_message_conversation_member WHERE conversation_id = $1", &[&conv_id]).await.unwrap();
        c.execute("DELETE FROM x_message_conversation WHERE id = $1", &[&conv_id]).await.unwrap();
    }

    /// R318 M1 行为回归：显式 sender 覆盖仍保留原值（缺省策略不得吞掉调用方意图）。
    #[tokio::test]
    async fn r318_m1_custom_create_explicit_sender_is_preserved() {
        if !shared::testing::is_db_available().await {
            eprintln!("skipping r318_m1_custom_create_explicit_sender_is_preserved: DB not reachable");
            return;
        }
        let pool = shared::testing::test_pool();
        let c = pool.get().await.unwrap();
        c.execute(
            "CREATE TABLE IF NOT EXISTS x_message_conversation_member (id TEXT, conversation_id TEXT, person_id TEXT, join_time TEXT, create_time TEXT)",
            &[],
        ).await.unwrap();
        let conv_id = "r318-m1-explicit-conv-001";
        let actor = "r318-m1-sender-dave";
        let explicit_sender = "r318-m1-explicit-other";
        c.execute("DELETE FROM x_message WHERE conversation_id = $1", &[&conv_id]).await.unwrap();
        c.execute("DELETE FROM x_message_conversation_member WHERE conversation_id = $1", &[&conv_id]).await.unwrap();
        c.execute("DELETE FROM x_message_conversation WHERE id = $1", &[&conv_id]).await.unwrap();
        c.execute(
            "INSERT INTO x_message_conversation (id, name, type) VALUES ($1, 'conv', 'single')",
            &[&conv_id],
        ).await.unwrap();
        c.execute(
            "INSERT INTO x_message_conversation_member (id, conversation_id, person_id) VALUES ($1, $2, $3)",
            &[&"r318-m1-member-002".to_string(), &conv_id.to_string(), &actor.to_string()],
        ).await.unwrap();

        let app = crate::router(pool.clone())
            .layer(axum::extract::Extension(shared::testing::test_session(actor)));
        let resp = app
            .oneshot(
                Request::builder()
                    .uri("/api/message/assemble/communicate/message/custom/create")
                    .method(axum::http::Method::POST)
                    .header("content-type", "application/json")
                    .body(Body::from(serde_json::to_vec(&serde_json::json!({
                        "conversationId": conv_id,
                        "content": "r318-m1-explicit-marker",
                        "sender": explicit_sender
                    })).unwrap()))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(resp.status(), StatusCode::OK, "custom/create 显式 sender 应成功");
        let stored: String = c.query_one(
            "SELECT sender FROM x_message WHERE conversation_id = $1 AND content = 'r318-m1-explicit-marker' ORDER BY create_time DESC LIMIT 1",
            &[&conv_id],
        ).await.unwrap().get("sender");
        assert_eq!(stored, explicit_sender, "显式 sender 必须保留");

        c.execute("DELETE FROM x_message WHERE conversation_id = $1", &[&conv_id]).await.unwrap();
        c.execute("DELETE FROM x_message_conversation_member WHERE conversation_id = $1", &[&conv_id]).await.unwrap();
        c.execute("DELETE FROM x_message_conversation WHERE id = $1", &[&conv_id]).await.unwrap();
    }

    // SKIPPED: mass_id_mockdeletetoget requires Session parameter
    // SKIPPED: im_conversation_id_single_delete_virtual requires Session parameter
    // SKIPPED: instant_currentperson_consumed_put requires Session parameter
    // SKIPPED: im_conversation_id_single_mockdeletetoget requires Session parameter
}
