// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

#[cfg(test)]
mod tests {
    #![allow(non_snake_case)]
    use axum::body::Body;
    use axum::http::{Request, StatusCode};

    use tower::util::ServiceExt;

    #[tokio::test]
    async fn test_bind() {
        let pool = shared::testing::test_pool();
        let app = crate::test_router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/authentication/bind")
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
            assert!(!bytes.is_empty(), "bind route should be registered");
        }
    }

    #[tokio::test]
    async fn test_captcha_default() {
        let pool = shared::testing::test_pool();
        let app = crate::test_router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/authentication/captcha")
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
                "captcha_default route should be registered"
            );
        }
    }

    #[tokio::test]
    async fn test_verify() {
        let pool = shared::testing::test_pool();
        let app = crate::test_router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/secret/captcha/verify")
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
            assert!(!bytes.is_empty(), "verify route should be registered");
        }
    }

    // SKIPPED: login requires Session parameter
    // SKIPPED: refresh requires Session parameter
    // SKIPPED: logout requires Session parameter
    // SKIPPED: whoami requires Session parameter
    // SKIPPED: code requires Session parameter
    #[tokio::test]
    async fn test_unit_list() {
        let pool = shared::testing::test_pool();
        let app = crate::test_router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/authentication/unit/list")
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
            assert!(!bytes.is_empty(), "unit_list route should be registered");
        }
    }

    #[tokio::test]
    async fn test_role_list() {
        let pool = shared::testing::test_pool();
        let app = crate::test_router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/authentication/role/list")
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
            assert!(!bytes.is_empty(), "role_list route should be registered");
        }
    }

    #[tokio::test]
    async fn test_group_list() {
        let pool = shared::testing::test_pool();
        let app = crate::test_router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/authentication/group/list")
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
            assert!(!bytes.is_empty(), "group_list route should be registered");
        }
    }

    // SKIPPED: mpweixin_login requires Session parameter
    // SKIPPED: mpweixin_bind_code requires Session parameter
    #[tokio::test]
    async fn test_oidc_authorize() {
        let pool = shared::testing::test_pool();
        let app = crate::test_router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/authentication/oidc/authorize")
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
                "oidc_authorize route should be registered"
            );
        }
    }

    // SKIPPED: oidc_callback requires Session parameter
    // SKIPPED: qiyeweixin_login requires Session parameter
    #[tokio::test]
    async fn test_sms_send_handler() {
        let pool = shared::testing::test_pool();
        let app = crate::test_router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/authentication/sms/send")
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
                "sms_send_handler route should be registered"
            );
        }
    }

    #[tokio::test]
    async fn test_sms_verify_handler() {
        let pool = shared::testing::test_pool();
        let app = crate::test_router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/authentication/sms/verify")
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
                "sms_verify_handler route should be registered"
            );
        }
    }

    // SKIPPED: welink_login requires Session parameter
    // SKIPPED: zwdingding_login requires Session parameter
    #[tokio::test]
    async fn test_zwdingding_info() {
        let pool = shared::testing::test_pool();
        let app = crate::test_router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/zhengwudingding/info")
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
                "zwdingding_info route should be registered"
            );
        }
    }

    // SKIPPED: andfx_moa_sso requires Session parameter
    // SKIPPED: bind_confirm requires Session parameter
    // SKIPPED: bind_poll requires Session parameter
    // SKIPPED: captcha_with_size not accessible
    // SKIPPED: check_token requires Session parameter
    // SKIPPED: try_ldap_authenticate not accessible
    // SKIPPED: code_send not accessible
    // SKIPPED: captcha_generate not accessible
    // SKIPPED: captcha_verify not accessible
    // SKIPPED: mpweixin_bind_openid requires Session parameter
    // SKIPPED: mpweixin_test_send requires Session parameter
    // SKIPPED: check_oauth_provider_health not accessible
    // SKIPPED: oauth_list not accessible
    // SKIPPED: oauth_qywx_config not accessible
    // SKIPPED: oauth_dingding_config not accessible
    // SKIPPED: oauth_name_config not accessible
    // SKIPPED: oauth_login_qywx requires Session parameter
    // SKIPPED: oauth_login_dingding requires Session parameter
    // SKIPPED: oauth_login_name requires Session parameter
    // SKIPPED: oauth_bind_name requires Session parameter
    // SKIPPED: verify_password_async not accessible
    // SKIPPED: get not accessible
    // SKIPPED: list not accessible
    // SKIPPED: qiyeweixin_update_person_detail requires Session parameter
    // SKIPPED: qiyeweixin_jssdk_sign not accessible
    // SKIPPED: safe_logout requires Session parameter
    // SKIPPED: sms_send not accessible
    // SKIPPED: sso_post_login requires Session parameter
    // SKIPPED: sso_get_login requires Session parameter
    // SKIPPED: sso_encrypt not accessible
    // SKIPPED: switch_user requires Session parameter
    // SKIPPED: two_factor_login not accessible
}
