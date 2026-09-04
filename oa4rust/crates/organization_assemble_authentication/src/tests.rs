#[cfg(test)]
mod tests {
    use axum::body::Body;
    use axum::http::{Request, StatusCode};
    use shared::testing::test_pool;
    use tower::util::ServiceExt;
    use crate::router;

    const TEST_PERSON_ID: &str = "test-person-id";
    const TEST_IDENTITY_ID: &str = "test-identity-id";

    #[tokio::test]
    async fn test_person_id_icon_route_exists() {
        let pool = test_pool();
        let app = router(pool);

        let response = app
            .oneshot(
                Request::builder()
                    .uri(&format!("/jaxrs/organization/assemble/authentication/person/{}/icon", TEST_PERSON_ID))
                    .method(axum::http::Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_ne!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_identity_id_route_exists() {
        let pool = test_pool();
        let app = router(pool);

        let response = app
            .oneshot(
                Request::builder()
                    .uri(&format!("/jaxrs/organization/assemble/authentication/identity/{}", TEST_IDENTITY_ID))
                    .method(axum::http::Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_ne!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_qiyeweixin_login_route_exists() {
        let pool = test_pool();
        let app = router(pool);

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/organization/assemble/authentication/qiyeweixin/login/testcode")
                    .method(axum::http::Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_ne!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_router_builds() {
        let pool = test_pool();
        let _ = router(pool);
    }
}


// ═════════════════════════════════════════════════════════════════════════════
// plan002 U2：x_organization_assemble_authentication 残余契约端点行为测试
//
// 这些测试编码业务意图：
//  1. 验证码登录必须先过一次性验证码，再过密码——两道闸缺一不可；
//  2. safe/logout 必须使本人全部会话失效（而非仅当前 token）；
//  3. mockdeletetoget 等 GET 别名与 Java 契约同义（登出真实生效）；
//  4. 配置驱动端点在未配置时显式报错，不伪造成功。
// ═════════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod u2_contract {
    use crate::{organization_assemble_authentication_router, router};
    use axum::body::Body;
    use axum::http::{Request, StatusCode};
    use serde_json::Value;
    use shared::session::SessionManager;
    use shared::testing::test_pool;
    use tower::util::ServiceExt;

    fn app() -> axum::Router {
        let pool = test_pool();
        let session_manager = SessionManager::with_pool(pool.clone());
        organization_assemble_authentication_router()
            .layer(axum::extract::Extension(session_manager))
            .layer(axum::extract::Extension(pool))
    }

    async fn body_bytes(response: axum::response::Response) -> Value {
        let bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap()
            .to_vec();
        serde_json::from_slice(&bytes).unwrap()
    }

    /// 幂等人员夹具：返回 (unique_id, 明文密码)
    async fn seed_person() -> (String, String) {
        let pool = test_pool();
        let client = pool.get().await.unwrap();
        client.execute(
            "CREATE TABLE IF NOT EXISTS auth_person (
                id VARCHAR(255) PRIMARY KEY,
                unique_id VARCHAR(255) UNIQUE,
                name VARCHAR(255),
                mobile VARCHAR(50),
                email VARCHAR(255),
                icon TEXT,
                password_hash VARCHAR(255) DEFAULT '',
                locked BOOLEAN DEFAULT FALSE,
                deleted_at TIMESTAMP,
                created_at TIMESTAMP DEFAULT NOW(),
                updated_at TIMESTAMP DEFAULT NOW()
             )",
            &[],
        )
        .await
        .unwrap();

        let uid = format!("u2-auth@{}", uuid::Uuid::new_v4());
        let password = "Passw0rd!";
        let hash = auth::password::hash_password(password);
        client.execute(
            "INSERT INTO auth_person (id, unique_id, name, password_hash, locked) \
             VALUES ($1, $2, $2, $3, false)",
            &[&format!("u2-p-{}", uid), &uid, &hash],
        )
        .await
        .unwrap();
        (uid, password.to_string())
    }

    async fn ensure_bind_table() {
        let pool = test_pool();
        let client = pool.get().await.unwrap();
        client.execute(
            "CREATE TABLE IF NOT EXISTS x_org_bind_record (
                id VARCHAR(36) PRIMARY KEY,
                name TEXT NOT NULL,
                message TEXT,
                creator TEXT,
                create_time TIMESTAMP NOT NULL DEFAULT NOW(),
                update_time TIMESTAMP NOT NULL DEFAULT NOW()
             )",
            &[],
        )
        .await
        .unwrap();
    }

    #[tokio::test]
    async fn u2_mode_defaults_all_disabled() {
        let v = body_bytes(app().oneshot(
            Request::builder()
                .uri("/jaxrs/organization/assemble/authentication/authentication/mode")
                .body(Body::empty())
                .unwrap(),
        ).await.unwrap()).await;
        assert_eq!(v["data"]["captchaLogin"], false);
        assert_eq!(v["data"]["codeLogin"], false);
        assert_eq!(v["data"]["twoFactorLogin"], false);
    }

    #[tokio::test]
    async fn u2_captcha_login_requires_valid_captcha_first() {
        use shared::testing::is_db_available;
        if !is_db_available().await {
            eprintln!("skipping u2_captcha_login_requires_valid_captcha_first: DATABASE_URL not reachable");
            return;
        }
        let (uid, password) = seed_person().await;
        let body = serde_json::json!({
            "credential": uid,
            "password": password,
            "captchaId": "bogus-id",
            "captchaAnswer": "0000"
        });
        let v = body_bytes(app().oneshot(
            Request::builder()
                .uri("/jaxrs/organization/assemble/authentication/authentication/captcha")
                .method("POST")
                .header("content-type", "application/json")
                .body(Body::from(body.to_string()))
                .unwrap(),
        ).await.unwrap()).await;
        assert_eq!(v["type"], "error");
        // 验证码无效时不得泄露凭据信息（统一文案）
        assert!(
            v["message"].as_str().unwrap().contains("captcha"),
            "错误应归因于验证码环节"
        );
    }

    #[tokio::test]
    async fn u2_captcha_login_full_flow_issues_session() {
        use shared::testing::is_db_available;
        if !is_db_available().await {
            eprintln!("skipping u2_captcha_login_full_flow_issues_session: DATABASE_URL not reachable");
            return;
        }
        let (uid, password) = seed_person().await;
        // 注入已知答案的验证码
        let captcha_id = captcha_store::captcha_store().insert("7272".to_string());
        let body = serde_json::json!({
            "credential": uid,
            "password": password,
            "captchaId": captcha_id,
            "captchaAnswer": "7272"
        });
        let v = body_bytes(app().oneshot(
            Request::builder()
                .uri("/jaxrs/organization/assemble/authentication/authentication/captcha")
                .method("POST")
                .header("content-type", "application/json")
                .body(Body::from(body.to_string()))
                .unwrap(),
        ).await.unwrap()).await;
        assert_eq!(v["type"], "success", "验证码+密码均正确时应登录成功: {v}");
        assert!(!v["data"]["token"].as_str().unwrap_or("").is_empty());

        // 错误密码：验证码已被消费，重新注入；密码错必须拒绝
        let captcha_id = captcha_store::captcha_store().insert("7373".to_string());
        let bad = serde_json::json!({
            "credential": uid,
            "password": "WrongPass1",
            "captchaId": captcha_id,
            "captchaAnswer": "7373"
        });
        let v = body_bytes(app().oneshot(
            Request::builder()
                .uri("/jaxrs/organization/assemble/authentication/authentication/captcha")
                .method("POST")
                .header("content-type", "application/json")
                .body(Body::from(bad.to_string()))
                .unwrap(),
        ).await.unwrap()).await;
        assert_eq!(v["type"], "error", "密码错误不得放行");
    }

    #[tokio::test]
    async fn u2_safe_logout_revokes_all_sessions_of_person() {
        let pool = test_pool();
        let sm = SessionManager::with_pool(pool.clone());
        sm.init_redis(); // 可用时走 Redis，不可用降级内存/DB
        let uid = "u2-safe-logout@P";
        let t1 = format!("tok-{}", uuid::Uuid::new_v4());
        let t2 = format!("tok-{}", uuid::Uuid::new_v4());
        sm.create_session(uid.to_string(), t1.clone()).await.unwrap();
        sm.create_session(uid.to_string(), t2.clone()).await.unwrap();

        let app = organization_assemble_authentication_router()
            .layer(axum::extract::Extension(sm.clone()))
            .layer(axum::extract::Extension(pool));
        let v = body_bytes(app.oneshot(
            Request::builder()
                .uri("/jaxrs/organization/assemble/authentication/authentication/safe/logout")
                .header("authorization", format!("Bearer {t1}"))
                .body(Body::empty())
                .unwrap(),
        ).await.unwrap()).await;
        assert_eq!(v["data"]["name"], "anonymous");

        // 本人两个会话都必须失效
        assert!(sm.validate_session(&t1).await.is_none(), "t1 应已失效");
        assert!(sm.validate_session(&t2).await.is_none(), "t2 也应失效（安全注销语义）");
    }

    #[tokio::test]
    async fn u2_mock_delete_to_get_logs_out() {
        let pool = test_pool();
        let sm = SessionManager::with_pool(pool.clone());
        let uid = "u2-mdtg@P";
        let token = format!("tok-{}", uuid::Uuid::new_v4());
        sm.create_session(uid.to_string(), token.clone()).await.unwrap();

        let app = organization_assemble_authentication_router()
            .layer(axum::extract::Extension(sm.clone()))
            .layer(axum::extract::Extension(pool));
        let response = app.oneshot(
            Request::builder()
                .uri("/jaxrs/organization/assemble/authentication/authentication/mockdeletetoget")
                .header("authorization", format!("Bearer {token}"))
                .body(Body::empty())
                .unwrap(),
        ).await.unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        let v = body_bytes(response).await;
        assert_eq!(v["type"], "success");
        assert!(sm.validate_session(&token).await.is_none(), "GET 登出应移除会话");
    }

    #[tokio::test]
    async fn u2_sso_encrypt_get_returns_encrypted_payload() {
        let v = body_bytes(app().oneshot(
            Request::builder()
                .uri("/jaxrs/organization/assemble/authentication/sso/encrypt/client/u2c/key/u2key-1234567890-abcdef-/credential/user@P")
                .body(Body::empty())
                .unwrap(),
        ).await.unwrap()).await;
        // 加密逻辑由 auth::sso 提供；此处校验 GET 入口契约可用且非失败
        assert_ne!(v["type"], "error", "SSO 加密入口应正常工作: {v}");
    }

    #[tokio::test]
    async fn u2_dingding_info_unconfigured_errors() {
        std::env::remove_var("DINGDING_CORP_ID");
        std::env::remove_var("DINGDING_AGENT_ID");
        std::env::remove_var("DINGDING_JSAPI_TICKET");
        let v = body_bytes(app().oneshot(
            Request::builder()
                .uri("/jaxrs/organization/assemble/authentication/dingding/info")
                .method("POST")
                .header("content-type", "application/json")
                .body(Body::from(r#"{"url":"https://e.x"}"#))
                .unwrap(),
        ).await.unwrap()).await;
        assert_eq!(v["type"], "error");
        assert!(
            v["message"].as_str().unwrap().contains("not configured"),
            "未配置时显式报错而非伪造数据"
        );
    }

    #[tokio::test]
    async fn u2_bind_list_returns_seeded_rows() {
        use shared::testing::is_db_available;
        if !is_db_available().await {
            eprintln!("skipping u2_bind_list_returns_seeded_rows: DATABASE_URL not reachable");
            return;
        }
        ensure_bind_table().await;
        {
            let pool = test_pool();
            let client = pool.get().await.unwrap();
            client.execute("DELETE FROM x_org_bind_record WHERE id='u2-bind-1'", &[]).await.unwrap();
            client.execute(
                "INSERT INTO x_org_bind_record (id, name, message) VALUES ('u2-bind-1', 'qywx', 'hello')",
                &[],
            ).await.unwrap();
        }
        let v = body_bytes(app().oneshot(
            Request::builder()
                .uri("/jaxrs/organization/assemble/authentication/bind/list")
                .body(Body::empty())
                .unwrap(),
        ).await.unwrap()).await;
        assert_eq!(v["type"], "success");
        let arr = v["data"].as_array().expect("bind/list 应返回数组");
        assert!(arr.iter().any(|r| r["id"] == "u2-bind-1"), "应包含种下的绑定记录");
    }

    #[tokio::test]
    async fn u2_full_router_still_builds() {
        let _ = router(test_pool());
    }

    #[tokio::test]
    async fn u2_test_get_oauth_auth_route() {
        let app = router(test_pool());
        let response = app
            .oneshot(
                axum::http::Request::builder()
                    .uri("/jaxrs/organization/assemble/authentication/oauth/auth?client_id=test")
                    .method(axum::http::Method::GET)
                    .body(axum::body::Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn u2_test_post_oauth_info_route() {
        let app = router(test_pool());
        let body = serde_json::to_string(&serde_json::json!({"clientId": "test"})).unwrap();
        let response = app
            .oneshot(
                axum::http::Request::builder()
                    .uri("/jaxrs/organization/assemble/authentication/oauth/info")
                    .method(axum::http::Method::POST)
                    .header("content-type", "application/json")
                    .body(axum::body::Body::from(body))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn u2_test_post_qiyeweixin_info_sign_route() {
        let app = router(test_pool());
        let body = serde_json::to_string(&serde_json::json!({"nonce": "n", "timestamp": "t"})).unwrap();
        let response = app
            .oneshot(
                axum::http::Request::builder()
                    .uri("/jaxrs/organization/assemble/authentication/qiyeweixin/info/sign")
                    .method(axum::http::Method::POST)
                    .header("content-type", "application/json")
                    .body(axum::body::Body::from(body))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
    }
}
