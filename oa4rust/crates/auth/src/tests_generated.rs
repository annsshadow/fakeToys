#[cfg(test)]
mod tests {
    use axum::body::Body;
    use axum::http::{Request, Method, StatusCode};
    use shared::testing::test_pool;
    use tower::util::ServiceExt;

    // SKIPPED: andfx_moa_sso requires Session parameter
    #[tokio::test]
    async fn test_bind() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/authentication/bind")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "bind route should be registered");
    }

    // SKIPPED: bind_confirm requires Session parameter
    // SKIPPED: bind_poll requires Session parameter
    #[tokio::test]
    async fn test_captcha_default() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/authentication/captcha")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "captcha_default route should be registered");
    }

    // SKIPPED: captcha_with_size not accessible
    #[tokio::test]
    async fn test_verify() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/secret/captcha/verify")
                    .method("POST")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "verify route should be registered");
    }

    // SKIPPED: check_token requires Session parameter
    // SKIPPED: try_ldap_authenticate not accessible
    // SKIPPED: login requires Session parameter
    // SKIPPED: refresh requires Session parameter
    // SKIPPED: logout requires Session parameter
    // SKIPPED: whoami requires Session parameter
    // SKIPPED: code_send not accessible
    // SKIPPED: code requires Session parameter
    #[tokio::test]
    async fn test_unit_list() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/authentication/unit/list")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "unit_list route should be registered");
    }

    #[tokio::test]
    async fn test_role_list() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/authentication/role/list")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "role_list route should be registered");
    }

    #[tokio::test]
    async fn test_group_list() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/authentication/group/list")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "group_list route should be registered");
    }

    // SKIPPED: captcha_generate not accessible
    // SKIPPED: captcha_verify not accessible
    // SKIPPED: mpweixin_login requires Session parameter
    // SKIPPED: mpweixin_bind_code requires Session parameter
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
    // SKIPPED: oidc_authorize not accessible
    // SKIPPED: oidc_callback requires Session parameter
    // SKIPPED: get not accessible
    // SKIPPED: list not accessible
    // SKIPPED: qiyeweixin_login requires Session parameter
    // SKIPPED: qiyeweixin_update_person_detail requires Session parameter
    #[tokio::test]
    async fn test_qiyeweixin_jssdk_sign() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/qiyeweixin/jssdk/sign/info")
                    .method("POST")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "qiyeweixin_jssdk_sign route should be registered");
    }

    // SKIPPED: safe_logout requires Session parameter
    // SKIPPED: sms_send not accessible
    #[tokio::test]
    async fn test_sms_send_handler() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/authentication/sms/send")
                    .method("POST")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "sms_send_handler route should be registered");
    }

    #[tokio::test]
    async fn test_sms_verify_handler() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/authentication/sms/verify")
                    .method("POST")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "sms_verify_handler route should be registered");
    }

    // SKIPPED: sso_post_login requires Session parameter
    // SKIPPED: sso_get_login requires Session parameter
    // SKIPPED: sso_encrypt not accessible
    // SKIPPED: switch_user requires Session parameter
    // SKIPPED: two_factor_login not accessible
    // SKIPPED: welink_login requires Session parameter
    // SKIPPED: zwdingding_login requires Session parameter
    #[tokio::test]
    async fn test_zwdingding_info() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/zhengwudingding/info")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "zwdingding_info route should be registered");
    }

}