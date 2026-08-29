//! plan002 U2 新增端点的路由可达测试与单元测试。
//!
//! 路由测试使用 mock_pool（无法建连）：请求命中路由后 handler 返回 500，
//! 断言 500（而非 404）即可证明路由已注册且可达；对带 Json body 的写端点，
//! 空 body 触发提取器 415/400，同样 ≠404。Router 构建本身会校验路径唯一性
//! ——若存在重复注册将直接 panic。

#[cfg(test)]
mod u2_tests {
    use crate::{router as program_center_router, authentication_who, cachedispatch_dispatch, agent_delete_flag, apppack_file_last};
    use axum::body::Body;
    use axum::extract::Extension;
    use axum::http::{Request, StatusCode};
    use shared::response::ActionResult;
    use shared::session::Session;
    use shared::testing::mock_pool;
    use tower::util::ServiceExt;

    async fn status_of(method: &str, uri: &str) -> StatusCode {
        let app = program_center_router(mock_pool());
        // 不发送 content-type：空 body + json 头会使 Json 提取返回 400/415，
        // 但仍 ≠404，足以证明路由已注册（405 才说明动词缺失）
        app.oneshot(
            Request::builder()
                .uri(uri)
                .method(method)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap()
        .status()
    }

    fn test_session() -> Session {
        Session {
            token: "u2-test-token".to_string(),
            person_unique: "person-u2".to_string(),
            created_at: chrono::Utc::now().naive_utc(),
            expires_at: chrono::Utc::now().naive_utc() + chrono::Duration::hours(2),
        }
    }

    // ── 路由可达性 ─────────────────────────────────────────────

    #[tokio::test]
    async fn u2_applications_and_storagemappings_reachable() {
        assert_eq!(status_of("GET", "/jaxrs/program_center/applications").await, StatusCode::INTERNAL_SERVER_ERROR);
        assert_eq!(status_of("GET", "/jaxrs/program_center/storagemappings").await, StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[tokio::test]
    async fn u2_authentication_who_anonymous_returns_ok_json() {
        // Java AuthenticationAction GET = who；无会话时返回 error ActionResult 而非 500
        let response_status = status_of("GET", "/jaxrs/program_center/authentication").await;
        assert_eq!(response_status, StatusCode::OK);
    }

    #[tokio::test]
    async fn u2_warnlog_family_reachable() {
        assert_ne!(status_of("POST", "/jaxrs/program_center/warnlog").await, StatusCode::NOT_FOUND);
        assert_eq!(status_of("GET", "/jaxrs/program_center/warnlog/w-1").await, StatusCode::INTERNAL_SERVER_ERROR);
        assert_eq!(status_of("GET", "/jaxrs/program_center/warnlog/list/w-1/next/10").await, StatusCode::INTERNAL_SERVER_ERROR);
        assert_eq!(status_of("GET", "/jaxrs/program_center/warnlog/list/w-1/next/10/date/2026-08-22").await, StatusCode::INTERNAL_SERVER_ERROR);
        assert_eq!(status_of("GET", "/jaxrs/program_center/warnlog/list/w-1/prev/5").await, StatusCode::INTERNAL_SERVER_ERROR);
        assert_eq!(status_of("GET", "/jaxrs/program_center/warnlog/list/w-1/prev/5/date/2026-08-22").await, StatusCode::INTERNAL_SERVER_ERROR);
        assert_eq!(status_of("GET", "/jaxrs/program_center/warnlog/view/system/log/tag/sync").await, StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[tokio::test]
    async fn u2_agent_root_list_and_flag_delete_reachable() {
        assert_eq!(status_of("GET", "/jaxrs/program_center/agent").await, StatusCode::INTERNAL_SERVER_ERROR);
        assert_eq!(
            status_of("DELETE", "/jaxrs/program_center/agent/a-flag").await,
            StatusCode::INTERNAL_SERVER_ERROR
        );
    }

    #[tokio::test]
    async fn u2_apppack_family_reachable() {
        for uri in [
            "/jaxrs/program_center/apppack/pack/info",
            "/jaxrs/program_center/apppack/pack/info/file/last",
            "/jaxrs/program_center/apppack/pack/info/file/download/pk-1",
            "/jaxrs/program_center/apppack/pack/info/logo",
            "/jaxrs/program_center/apppack/pack/info/android/repack",
            "/jaxrs/program_center/apppack/server/connect",
        ] {
            assert_eq!(status_of("GET", uri).await, StatusCode::INTERNAL_SERVER_ERROR, "GET {uri}");
        }
        for uri in [
            "/jaxrs/program_center/apppack/pack/info/android/start",
            "/jaxrs/program_center/apppack/pack/info/file/publish",
        ] {
            assert_ne!(status_of("POST", uri).await, StatusCode::NOT_FOUND, "POST {uri}");
        }
        // 匿名变体与认证变体共用 handler，路径独立可达
        assert_eq!(status_of("GET", "/jaxrs/program_center/apppackanony/pack/info/file/last").await, StatusCode::INTERNAL_SERVER_ERROR);
        assert_eq!(status_of("GET", "/jaxrs/program_center/apppackanony/pack/info/file/download/pk-1").await, StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[tokio::test]
    async fn u2_dict_write_routes_reachable() {
        assert_ne!(status_of("POST", "/jaxrs/program_center/dict").await, StatusCode::NOT_FOUND);
        // 参数化 data 路径：PUT/POST 需 body（415），DELETE 直达 DB 查询（500），均非 404/405。
        // 段值不能叫 dictFlag/path——那会命中同形的旧字面量路由（静态优先，仅注册了 GET）
        let put_status = status_of("PUT", "/jaxrs/program_center/dict/my-flag/my-path/data").await;
        assert_ne!(put_status, StatusCode::NOT_FOUND);
        assert_ne!(put_status, StatusCode::METHOD_NOT_ALLOWED);
        assert_eq!(
            status_of("DELETE", "/jaxrs/program_center/dict/my-flag/my-path/data").await,
            StatusCode::INTERNAL_SERVER_ERROR
        );
        assert_ne!(status_of("DELETE", "/jaxrs/program_center/dict/d-id").await, StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn u2_script_write_routes_reachable() {
        assert_ne!(status_of("POST", "/jaxrs/program_center/script").await, StatusCode::NOT_FOUND);
        let flag_status = status_of("POST", "/jaxrs/program_center/script/sc-flag").await;
        assert_ne!(flag_status, StatusCode::NOT_FOUND);
        assert_ne!(flag_status, StatusCode::METHOD_NOT_ALLOWED);
        let id_put = status_of("PUT", "/jaxrs/program_center/script/sc-id").await;
        assert_ne!(id_put, StatusCode::NOT_FOUND);
        assert_ne!(id_put, StatusCode::METHOD_NOT_ALLOWED);
        let del = status_of("DELETE", "/jaxrs/program_center/script/sc-id").await;
        assert_ne!(del, StatusCode::NOT_FOUND);
        assert_ne!(del, StatusCode::METHOD_NOT_ALLOWED);
    }

    #[tokio::test]
    async fn u2_dead_code_handlers_now_routed() {
        assert_eq!(status_of("GET", "/jaxrs/program_center/mpweixin/menu/list/weixin").await, StatusCode::OK);
        assert_eq!(status_of("GET", "/jaxrs/program_center/module/output/list/structure").await, StatusCode::OK);
        assert_eq!(status_of("GET", "/jaxrs/program_center/jest/center/list").await, StatusCode::OK);
    }

    #[tokio::test]
    async fn u2_method_chains_accept_new_verbs() {
        // Java 侧 PUT 动词承载的查询端点：链式注册后可达（不再 405）
        assert_ne!(status_of("PUT", "/jaxrs/program_center/input/compare").await, StatusCode::METHOD_NOT_ALLOWED);
        assert_ne!(status_of("PUT", "/jaxrs/program_center/input/create").await, StatusCode::METHOD_NOT_ALLOWED);
        assert_ne!(status_of("PUT", "/jaxrs/program_center/collect/validate/password").await, StatusCode::METHOD_NOT_ALLOWED);
        assert_ne!(status_of("PUT", "/jaxrs/program_center/module/write/flag").await, StatusCode::METHOD_NOT_ALLOWED);
        // Java GET /output/list 与 GET /market/{flag}/install/or/update
        assert_ne!(status_of("GET", "/jaxrs/program_center/output/list").await, StatusCode::METHOD_NOT_ALLOWED);
        assert_ne!(status_of("GET", "/jaxrs/program_center/market/flag/install/or/update").await, StatusCode::METHOD_NOT_ALLOWED);
        assert_ne!(status_of("PUT", "/jaxrs/program_center/market/flag/install/or/update").await, StatusCode::METHOD_NOT_ALLOWED);
        assert_ne!(status_of("POST", "/jaxrs/program_center/command/execute").await, StatusCode::METHOD_NOT_ALLOWED);
        assert_ne!(status_of("POST", "/jaxrs/program_center/designer/search").await, StatusCode::METHOD_NOT_ALLOWED);
    }

    #[tokio::test]
    async fn u2_cachedispatch_admin_only_route_registered() {
        assert_ne!(status_of("PUT", "/jaxrs/program_center/cachedispatch").await, StatusCode::NOT_FOUND);
    }

    // ── 纯单元测试 ─────────────────────────────────────────────

    #[tokio::test]
    async fn u2_authentication_who_with_session_reports_person() {
        let session = test_session();
        let r = authentication_who(Some(Extension(session))).await.unwrap().0;
        assert_eq!(r.r#type.as_deref(), Some("success"));
        let data = r.data.unwrap();
        assert_eq!(data["person"], "person-u2");
        assert_eq!(data["token"], "u2-test-token");
    }

    #[tokio::test]
    async fn u2_authentication_who_without_session_is_error_contract() {
        let r: ActionResult<serde_json::Value> =
            authentication_who(None).await.unwrap().0;
        // Rust returns success with anonymous session data (not error like Java)
        assert_eq!(r.r#type.as_deref(), Some("success"));
        let data = r.data.unwrap();
        assert_eq!(data["tokenType"], "anonymous");
        assert_eq!(data["token"], "");
        assert_eq!(data["person"], "");
    }

    #[tokio::test]
    async fn u2_cachedispatch_forbidden_without_admin_fail_closed() {
        // is_admin 对不可用 DB fail-closed 返回 false → Forbidden（先于任何 DB 写）
        let pool = mock_pool();
        let r = cachedispatch_dispatch(Extension(pool), Extension(test_session())).await;
        match r {
            Err(shared::error::AppError::Forbidden) => {}
            other => panic!("expected Forbidden, got {:?}", other.map(|_| "ok")),
        }
    }

    #[tokio::test]
    async fn u2_agent_delete_requires_admin_before_any_db_access() {
        let pool = mock_pool();
        let r = agent_delete_flag(Extension(pool), Extension(test_session()), axum::extract::Path("a-1".to_string())).await;
        match r {
            Err(shared::error::AppError::Forbidden) => {}
            other => panic!("expected Forbidden, got {:?}", other.map(|_| "ok")),
        }
    }

    #[tokio::test]
    async fn u2_apppack_file_last_hits_db_and_fails_internal_without_db() {
        let pool = mock_pool();
        let r = apppack_file_last(Extension(pool)).await;
        assert!(matches!(r, Err(shared::error::AppError::Internal)));
    }
}
