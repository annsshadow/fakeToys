// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

#[cfg(test)]
mod tests {
    #![allow(non_snake_case)]
    use axum::body::Body;
    use axum::http::{Request, StatusCode};

    use tower::util::ServiceExt;

    #[tokio::test]
    async fn test_get_bam_config() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/processplatform/assemble/bam/get/test-id")
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
                "get_bam_config route should be registered"
            );
        }
    }

    // SKIPPED: create_bam requires Session parameter
    #[tokio::test]
    async fn test_list_bams() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/processplatform/assemble/bam/list/test-id")
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
            assert!(!bytes.is_empty(), "list_bams route should be registered");
        }
    }

    // SKIPPED: delete_bam requires Session parameter
    #[tokio::test]
    async fn test_get_bam_status() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/processplatform/assemble/bam/status/test-id")
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
                "get_bam_status route should be registered"
            );
        }
    }

    #[tokio::test]
    async fn test_state_applicationtstubs_trigger() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/processplatform/assemble/bam/state/applicationtstubs/trigger")
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
                "state_applicationtstubs_trigger route should be registered"
            );
        }
    }

    #[tokio::test]
    async fn test_state_category() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/processplatform/assemble/bam/state/category")
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
                "state_category route should be registered"
            );
        }
    }

    #[tokio::test]
    async fn test_state_organization() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/processplatform/assemble/bam/state/organization")
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
                "state_organization route should be registered"
            );
        }
    }

    #[tokio::test]
    async fn test_state_running() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/processplatform/assemble/bam/state/running")
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
                "state_running route should be registered"
            );
        }
    }

    #[tokio::test]
    async fn test_state_summary() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/processplatform/assemble/bam/state/summary")
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
                "state_summary route should be registered"
            );
        }
    }

    #[tokio::test]
    async fn test_bam_stubs_completed_task_by_application() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/processplatform/assemble/bam/period/list/completed/task/applicationstubs")
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
                "bam_stubs_completed_task_by_application route should be registered"
            );
        }
    }

    #[tokio::test]
    async fn test_bam_stubs_completed_task_by_unit() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/processplatform/assemble/bam/period/list/completed/task/unitstubs")
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
                "bam_stubs_completed_task_by_unit route should be registered"
            );
        }
    }

    #[tokio::test]
    async fn test_bam_stubs_completed_work_by_application() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/processplatform/assemble/bam/period/list/completed/work/applicationstubs")
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
                "bam_stubs_completed_work_by_application route should be registered"
            );
        }
    }

    #[tokio::test]
    async fn test_bam_stubs_completed_work_by_unit() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/processplatform/assemble/bam/period/list/completed/work/unitstubs")
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
                "bam_stubs_completed_work_by_unit route should be registered"
            );
        }
    }

    #[tokio::test]
    async fn test_bam_stubs_expired_task_by_application() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/processplatform/assemble/bam/period/list/expired/task/applicationstubs")
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
                "bam_stubs_expired_task_by_application route should be registered"
            );
        }
    }

    #[tokio::test]
    async fn test_bam_stubs_expired_task_by_unit() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/processplatform/assemble/bam/period/list/expired/task/unitstubs")
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
                "bam_stubs_expired_task_by_unit route should be registered"
            );
        }
    }

    #[tokio::test]
    async fn test_bam_stubs_expired_work_by_application() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/processplatform/assemble/bam/period/list/expired/work/applicationstubs")
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
                "bam_stubs_expired_work_by_application route should be registered"
            );
        }
    }

    #[tokio::test]
    async fn test_bam_stubs_expired_work_by_unit() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/processplatform/assemble/bam/period/list/expired/work/unitstubs")
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
                "bam_stubs_expired_work_by_unit route should be registered"
            );
        }
    }

    #[tokio::test]
    async fn test_bam_stubs_start_task_by_application() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri(
                        "/api/processplatform/assemble/bam/period/list/start/task/applicationstubs",
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
                "bam_stubs_start_task_by_application route should be registered"
            );
        }
    }

    #[tokio::test]
    async fn test_bam_stubs_start_task_by_unit() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/processplatform/assemble/bam/period/list/start/task/unitstubs")
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
                "bam_stubs_start_task_by_unit route should be registered"
            );
        }
    }

    #[tokio::test]
    async fn test_bam_stubs_start_work_by_application() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri(
                        "/api/processplatform/assemble/bam/period/list/start/work/applicationstubs",
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
                "bam_stubs_start_work_by_application route should be registered"
            );
        }
    }

    #[tokio::test]
    async fn test_bam_stubs_start_work_by_unit() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/processplatform/assemble/bam/period/list/start/work/unitstubs")
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
                "bam_stubs_start_work_by_unit route should be registered"
            );
        }
    }

    #[tokio::test]
    async fn test_bam_count_completed_task_by_unit() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/processplatform/assemble/bam/period/list/count/completed/task/application/test-id/process/test-id/activity/test-id/by/unit")
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
                "bam_count_completed_task_by_unit route should be registered"
            );
        }
    }

    #[tokio::test]
    async fn test_bam_count_completed_task_total() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/processplatform/assemble/bam/period/list/count/completed/task/application/test-id/process/test-id/activity/test-id/unit/test-id/person/test-id")
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
                "bam_count_completed_task_total route should be registered"
            );
        }
    }

    #[tokio::test]
    async fn test_bam_count_completed_task_by_activity() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/processplatform/assemble/bam/period/list/count/completed/task/application/test-id/process/test-id/unit/test-id/person/test-id/by/activity")
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
                "bam_count_completed_task_by_activity route should be registered"
            );
        }
    }

    #[tokio::test]
    async fn test_bam_count_completed_task_by_process() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/processplatform/assemble/bam/period/list/count/completed/task/application/test-id/unit/test-id/person/test-id/by/process")
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
                "bam_count_completed_task_by_process route should be registered"
            );
        }
    }

    #[tokio::test]
    async fn test_bam_count_completed_task_by_application() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/processplatform/assemble/bam/period/list/count/completed/task/unit/test-id/person/test-id/by/application")
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
                "bam_count_completed_task_by_application route should be registered"
            );
        }
    }

    #[tokio::test]
    async fn test_bam_count_completed_work_by_unit() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/processplatform/assemble/bam/period/list/count/completed/work/application/test-id/process/test-id/by/unit")
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
                "bam_count_completed_work_by_unit route should be registered"
            );
        }
    }

    #[tokio::test]
    async fn test_bam_count_completed_work_total() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/processplatform/assemble/bam/period/list/count/completed/work/application/test-id/process/test-id/unit/test-id/person/test-id")
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
                "bam_count_completed_work_total route should be registered"
            );
        }
    }

    #[tokio::test]
    async fn test_bam_count_completed_work_by_process() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/processplatform/assemble/bam/period/list/count/completed/work/application/test-id/unit/test-id/person/test-id/by/process")
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
                "bam_count_completed_work_by_process route should be registered"
            );
        }
    }

    #[tokio::test]
    async fn test_bam_count_completed_work_by_application() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/processplatform/assemble/bam/period/list/count/completed/work/unit/test-id/person/test-id/by/application")
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
                "bam_count_completed_work_by_application route should be registered"
            );
        }
    }

    #[tokio::test]
    async fn test_bam_count_expired_task_by_unit() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/processplatform/assemble/bam/period/list/count/expired/task/application/test-id/process/test-id/activity/test-id/by/unit")
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
                "bam_count_expired_task_by_unit route should be registered"
            );
        }
    }

    #[tokio::test]
    async fn test_bam_count_expired_task_total() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/processplatform/assemble/bam/period/list/count/expired/task/application/test-id/process/test-id/activity/test-id/unit/test-id/person/test-id")
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
                "bam_count_expired_task_total route should be registered"
            );
        }
    }

    #[tokio::test]
    async fn test_bam_count_expired_task_by_activity() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/processplatform/assemble/bam/period/list/count/expired/task/application/test-id/process/test-id/unit/test-id/person/test-id/by/activity")
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
                "bam_count_expired_task_by_activity route should be registered"
            );
        }
    }

    #[tokio::test]
    async fn test_bam_count_expired_task_by_process() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/processplatform/assemble/bam/period/list/count/expired/task/application/test-id/unit/test-id/person/test-id/by/process")
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
                "bam_count_expired_task_by_process route should be registered"
            );
        }
    }

    #[tokio::test]
    async fn test_bam_count_expired_task_by_application() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/processplatform/assemble/bam/period/list/count/expired/task/unit/test-id/person/test-id/by/application")
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
                "bam_count_expired_task_by_application route should be registered"
            );
        }
    }

    #[tokio::test]
    async fn test_bam_count_expired_work_by_unit() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/processplatform/assemble/bam/period/list/count/expired/work/application/test-id/process/test-id/by/unit")
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
                "bam_count_expired_work_by_unit route should be registered"
            );
        }
    }

    #[tokio::test]
    async fn test_bam_count_expired_work_total() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/processplatform/assemble/bam/period/list/count/expired/work/application/test-id/process/test-id/unit/test-id/person/test-id")
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
                "bam_count_expired_work_total route should be registered"
            );
        }
    }

    #[tokio::test]
    async fn test_bam_count_expired_work_by_process() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/processplatform/assemble/bam/period/list/count/expired/work/application/test-id/unit/test-id/person/test-id/by/process")
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
                "bam_count_expired_work_by_process route should be registered"
            );
        }
    }

    #[tokio::test]
    async fn test_bam_count_expired_work_by_application() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/processplatform/assemble/bam/period/list/count/expired/work/unit/test-id/person/test-id/by/application")
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
                "bam_count_expired_work_by_application route should be registered"
            );
        }
    }

    #[tokio::test]
    async fn test_bam_count_start_task_by_unit() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/processplatform/assemble/bam/period/list/count/start/task/application/test-id/process/test-id/activity/test-id/by/unit")
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
                "bam_count_start_task_by_unit route should be registered"
            );
        }
    }

    #[tokio::test]
    async fn test_bam_count_start_task_total() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/processplatform/assemble/bam/period/list/count/start/task/application/test-id/process/test-id/activity/test-id/unit/test-id/person/test-id")
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
                "bam_count_start_task_total route should be registered"
            );
        }
    }

    #[tokio::test]
    async fn test_bam_count_start_task_by_activity() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/processplatform/assemble/bam/period/list/count/start/task/application/test-id/process/test-id/unit/test-id/person/test-id/by/activity")
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
                "bam_count_start_task_by_activity route should be registered"
            );
        }
    }

    #[tokio::test]
    async fn test_bam_count_start_task_by_process() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/processplatform/assemble/bam/period/list/count/start/task/application/test-id/unit/test-id/person/test-id/by/process")
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
                "bam_count_start_task_by_process route should be registered"
            );
        }
    }

    #[tokio::test]
    async fn test_bam_count_start_task_by_application() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/processplatform/assemble/bam/period/list/count/start/task/unit/test-id/person/test-id/by/application")
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
                "bam_count_start_task_by_application route should be registered"
            );
        }
    }

    #[tokio::test]
    async fn test_bam_count_start_work_by_unit() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/processplatform/assemble/bam/period/list/count/start/work/application/test-id/process/test-id/by/unit")
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
                "bam_count_start_work_by_unit route should be registered"
            );
        }
    }

    #[tokio::test]
    async fn test_bam_count_start_work_total() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/processplatform/assemble/bam/period/list/count/start/work/application/test-id/process/test-id/unit/test-id/person/test-id")
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
                "bam_count_start_work_total route should be registered"
            );
        }
    }

    #[tokio::test]
    async fn test_bam_count_start_work_by_process() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/processplatform/assemble/bam/period/list/count/start/work/application/test-id/unit/test-id/person/test-id/by/process")
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
                "bam_count_start_work_by_process route should be registered"
            );
        }
    }

    #[tokio::test]
    async fn test_bam_count_start_work_by_application() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/processplatform/assemble/bam/period/list/count/start/work/unit/test-id/person/test-id/by/application")
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
                "bam_count_start_work_by_application route should be registered"
            );
        }
    }

    #[tokio::test]
    async fn test_state_category_trigger_all() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/processplatform/assemble/bam/state/category/trigger")
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
                "state_category_trigger_all route should be registered"
            );
        }
    }

    // SKIPPED: period_list_completed_task_application not accessible
    // SKIPPED: period_list_completed_task_unit not accessible
    // SKIPPED: period_list_completed_work_application not accessible
    // SKIPPED: period_list_completed_work_unit not accessible
    // SKIPPED: period_list_count_completed_task_application_applicationId_process_processId_activity_activityId_by_unit not accessible
    // SKIPPED: period_list_count_completed_task_application_applicationId_process_processId_activity_activityId_unit_unit_person_person not accessible
    // SKIPPED: period_list_count_completed_task_application_applicationId_process_processId_unit_unit_person_person_by_activity not accessible
    // SKIPPED: period_list_count_completed_task_application_applicationId_unit_unit_person_person_by_process not accessible
    // SKIPPED: period_list_count_completed_task_unit_unit_person_person_by_application not accessible
    // SKIPPED: period_list_count_completed_work_application_applicationId_process_processId_by_unit not accessible
    // SKIPPED: period_list_count_completed_work_application_applicationId_process_processId_unit_unit_person_person not accessible
    // SKIPPED: period_list_count_completed_work_application_applicationId_unit_unit_person_person_by_process not accessible
    // SKIPPED: period_list_count_completed_work_unit_unit_person_person_by_application not accessible
    // SKIPPED: period_list_count_expired_task_application_applicationId_process_processId_activity_activityId_by_unit not accessible
    // SKIPPED: period_list_count_expired_task_application_applicationId_process_processId_activity_activityId_unit_unit_person_person not accessible
    // SKIPPED: period_list_count_expired_task_application_applicationId_process_processId_unit_unit_person_person_by_activity not accessible
    // SKIPPED: period_list_count_expired_task_application_applicationId_unit_unit_person_person_by_process not accessible
    // SKIPPED: period_list_count_expired_task_unit_unit_person_person_by_application not accessible
    // SKIPPED: period_list_count_expired_work_application_applicationId_process_processId_by_unit not accessible
    // SKIPPED: period_list_count_expired_work_application_applicationId_process_processId_unit_unit_person_person not accessible
    // SKIPPED: period_list_count_expired_work_application_applicationId_unit_unit_person_person_by_process not accessible
    // SKIPPED: period_list_count_expired_work_unit_unit_person_person_by_application not accessible
    // SKIPPED: period_list_count_start_task_application_applicationId_process_processId_activity_activityId_by_unit not accessible
    // SKIPPED: period_list_count_start_task_application_applicationId_process_processId_activity_activityId_unit_unit_person_person not accessible
    // SKIPPED: period_list_count_start_task_application_applicationId_process_processId_unit_unit_person_person_by_activity not accessible
    // SKIPPED: period_list_count_start_task_application_applicationId_unit_unit_person_person_by_process not accessible
    // SKIPPED: period_list_count_start_task_unit_unit_person_person_by_application not accessible
    // SKIPPED: period_list_count_start_work_application_applicationId_process_processId_by_unit not accessible
    // SKIPPED: period_list_count_start_work_application_applicationId_process_processId_unit_unit_person_person not accessible
    // SKIPPED: period_list_count_start_work_application_applicationId_unit_unit_person_person_by_process not accessible
    // SKIPPED: period_list_count_start_work_unit_unit_person_person_by_application not accessible
    // SKIPPED: period_list_expired_task_application not accessible
    // SKIPPED: period_list_expired_task_unit not accessible
    // SKIPPED: period_list_expired_work_application not accessible
    // SKIPPED: period_list_expired_work_unit not accessible
    // SKIPPED: period_list_start_task_application not accessible
    // SKIPPED: period_list_start_task_unit not accessible
    // SKIPPED: period_list_start_work_application not accessible
    // SKIPPED: period_list_start_work_unit not accessible
    // SKIPPED: state_category_trigger requires Session parameter
}
