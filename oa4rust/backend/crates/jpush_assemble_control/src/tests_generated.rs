// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

#[cfg(test)]
mod tests {
    #![allow(non_snake_case)]
    use axum::body::Body;
    use axum::http::{Request, StatusCode};

    use tower::util::ServiceExt;

    #[tokio::test]
    async fn test_list_jpushs() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/jpush/list")
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
            assert!(!bytes.is_empty(), "list_jpushs route should be registered");
        }
    }

    #[tokio::test]
    async fn test_get_jpush() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/jpush/get/test-id")
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
            assert!(!bytes.is_empty(), "get_jpush route should be registered");
        }
    }

    // SKIPPED: create_jpush requires Session parameter
    // SKIPPED: save_jpush requires Session parameter
    // SKIPPED: delete_jpush requires Session parameter
    // SKIPPED: device_bind requires Session parameter
    // SKIPPED: get_control_config not accessible
    // SKIPPED: list_control_apps not accessible
    // SKIPPED: update_control_config requires Session parameter
    // SKIPPED: device_admin_unbind_all_person requires Session parameter
    // SKIPPED: device_check_deviceName_deviceType_pushType not accessible
    // SKIPPED: device_config_push_type not accessible
    // SKIPPED: device_list_pushType not accessible
    // SKIPPED: device_unbind_new_deviceName_deviceType_pushType requires Session parameter
    // SKIPPED: device_unbind_deviceName_deviceType requires Session parameter
    // SKIPPED: message_test_send not accessible
    // SKIPPED: message_send requires Session parameter
}
