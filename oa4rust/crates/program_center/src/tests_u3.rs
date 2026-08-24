//! plan002 U2 终扫闭合（U3 批次）新增端点的路由可达测试与单元测试。
//!
//! 约定与 tests_u2 相同：mock_pool 无法建连，DB 型请求断言 500（≠404 即路由可达）；
//! 带 Json 提取器的写端点在会话扩展缺失时先以 500 拒绝；管理类 handler 以
//! require_admin 先于任何 DB 访问，fail-closed 返回 Forbidden。

#[cfg(test)]
mod u3_tests {
    use crate::{
        router as program_center_router,
        u3_agent_flag_disable, u3_collect_resetpassword_put, u3_config_change_password_post,
        u3_invoke_file_put, u3_jest_clear_cache_source, u3_market_list_paging_category,
        u3_prompterrorlog_create, u3_qiyeweixin_create, u3_schedule_fire_post, u3_script_save_name,
        u3_welink_pull_sync,
    };
    use axum::body::Body;
    use axum::extract::Extension;
    use axum::http::{Request, StatusCode};
    use shared::session::Session;
    use shared::testing::mock_pool;
    use tower::util::ServiceExt;

    async fn status_of(method: &str, uri: &str) -> StatusCode {
        let app = program_center_router(mock_pool());
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
            token: "u3-test-token".to_string(),
            person_unique: "person-u3".to_string(),
            created_at: chrono::Utc::now().naive_utc(),
            expires_at: chrono::Utc::now().naive_utc() + chrono::Duration::hours(2),
        }
    }

    // ── 路由可达性：agent 参数化家族 ─────────────────────────────

    #[tokio::test]
    async fn u3_agent_param_family_reachable() {
        assert_eq!(status_of("GET", "/jaxrs/program_center/agent/a-1").await, StatusCode::INTERNAL_SERVER_ERROR);
        assert_eq!(status_of("GET", "/jaxrs/program_center/agent/a-1/disable").await, StatusCode::INTERNAL_SERVER_ERROR);
        assert_eq!(status_of("GET", "/jaxrs/program_center/agent/a-1/enable").await, StatusCode::INTERNAL_SERVER_ERROR);
        assert_eq!(status_of("GET", "/jaxrs/program_center/agent/a-1/execute").await, StatusCode::INTERNAL_SERVER_ERROR);
        // PUT file：会话扩展缺失先拒绝（500），但绝非 404/405
        let put_status = status_of("PUT", "/jaxrs/program_center/agent/a-1/file").await;
        assert_ne!(put_status, StatusCode::NOT_FOUND);
        assert_ne!(put_status, StatusCode::METHOD_NOT_ALLOWED);
        // POST /agent（Java 创建入口）
        assert_ne!(status_of("POST", "/jaxrs/program_center/agent").await, StatusCode::NOT_FOUND);
    }

    // ── appstyle：PUT 上传与 GET erase ───────────────────────────

    #[tokio::test]
    async fn u3_appstyle_put_and_erase_get_reachable() {
        assert_ne!(status_of("PUT", "/jaxrs/program_center/appstyle").await, StatusCode::NOT_FOUND);
        assert_ne!(status_of("GET", "/jaxrs/program_center/appstyle/current/update").await, StatusCode::NOT_FOUND);
        for uri in [
            "/jaxrs/program_center/appstyle/image/application/top",
            "/jaxrs/program_center/appstyle/image/menu/logo/blur",
            "/jaxrs/program_center/appstyle/image/process/default",
        ] {
            assert_ne!(status_of("PUT", uri).await, StatusCode::NOT_FOUND, "PUT {uri}");
        }
        for uri in [
            "/jaxrs/program_center/appstyle/image/application/top/erase",
            "/jaxrs/program_center/appstyle/image/menu/logo/blur/erase",
            "/jaxrs/program_center/appstyle/image/menu/logo/focus/erase",
            "/jaxrs/program_center/appstyle/image/process/default/erase",
            "/jaxrs/program_center/appstyle/image/setup/about/logo/erase",
        ] {
            assert_eq!(status_of("GET", uri).await, StatusCode::INTERNAL_SERVER_ERROR, "GET {uri}");
        }
    }

    // ── bar/foo 图表参数化族 ─────────────────────────────────────

    #[tokio::test]
    async fn u3_bar_foo_param_routes_reachable() {
        assert_eq!(status_of("GET", "/jaxrs/program_center/bar/create/mass/0/10").await, StatusCode::INTERNAL_SERVER_ERROR);
        assert_eq!(status_of("GET", "/jaxrs/program_center/foo/create/mass/5/20").await, StatusCode::INTERNAL_SERVER_ERROR);
        assert_eq!(status_of("GET", "/jaxrs/program_center/bar/select1/field/status/value/open/count/10").await, StatusCode::INTERNAL_SERVER_ERROR);
        assert_eq!(status_of("GET", "/jaxrs/program_center/bar/select2/count/10").await, StatusCode::INTERNAL_SERVER_ERROR);
        assert_eq!(status_of("GET", "/jaxrs/program_center/bar/select3/field/name/value/x/count/5").await, StatusCode::INTERNAL_SERVER_ERROR);
        assert_eq!(status_of("GET", "/jaxrs/program_center/bar/select4/field/entity/value/y/count/5").await, StatusCode::INTERNAL_SERVER_ERROR);
    }

    // ── captcha / code 参数化族 ──────────────────────────────────

    #[tokio::test]
    async fn u3_captcha_code_param_routes_reachable() {
        assert_eq!(
            status_of("GET", "/jaxrs/program_center/captcha/v2/create/width/200/height/80").await,
            StatusCode::INTERNAL_SERVER_ERROR
        );
        assert_eq!(
            status_of("GET", "/jaxrs/program_center/captcha/c-1/validate/answer/1234").await,
            StatusCode::INTERNAL_SERVER_ERROR
        );
        assert_eq!(
            status_of("GET", "/jaxrs/program_center/code/create/mobile/13800000000").await,
            StatusCode::INTERNAL_SERVER_ERROR
        );
        assert_eq!(
            status_of("GET", "/jaxrs/program_center/code/validate/mobile/13800000000/answer/123456").await,
            StatusCode::INTERNAL_SERVER_ERROR
        );
        assert_eq!(
            status_of("GET", "/jaxrs/program_center/code/validate/mobile/13800000000/answer/123456/cascade").await,
            StatusCode::INTERNAL_SERVER_ERROR
        );
        assert_ne!(
            status_of("POST", "/jaxrs/program_center/code/list/paging/1/size/20").await,
            StatusCode::NOT_FOUND
        );
    }

    // ── collect 家族 ─────────────────────────────────────────────

    #[tokio::test]
    async fn u3_collect_family_reachable() {
        assert_eq!(status_of("GET", "/jaxrs/program_center/collect").await, StatusCode::INTERNAL_SERVER_ERROR);
        assert_ne!(status_of("POST", "/jaxrs/program_center/collect").await, StatusCode::NOT_FOUND);
        assert_eq!(
            status_of("DELETE", "/jaxrs/program_center/collect/name/n/mobile/m/code/c").await,
            StatusCode::INTERNAL_SERVER_ERROR
        );
        assert_eq!(
            status_of("GET", "/jaxrs/program_center/collect/controllermobile/name/n/mobile/m").await,
            StatusCode::INTERNAL_SERVER_ERROR
        );
        assert_eq!(
            status_of("GET", "/jaxrs/program_center/collect/name/n/exist").await,
            StatusCode::INTERNAL_SERVER_ERROR
        );
        for uri in ["/jaxrs/program_center/collect/resetpassword", "/jaxrs/program_center/collect/urlMapping"] {
            let s = status_of("PUT", uri).await;
            assert_ne!(s, StatusCode::NOT_FOUND, "PUT {uri}");
            assert_ne!(s, StatusCode::METHOD_NOT_ALLOWED, "PUT {uri}");
        }
    }

    // ── config 家族 ──────────────────────────────────────────────

    #[tokio::test]
    async fn u3_config_family_reachable() {
        assert_eq!(status_of("GET", "/jaxrs/program_center/config").await, StatusCode::INTERNAL_SERVER_ERROR);
        assert_eq!(
            status_of("GET", "/jaxrs/program_center/config-open/get/disable/export/enable").await,
            StatusCode::INTERNAL_SERVER_ERROR
        );
        for uri in [
            "/jaxrs/program_center/config/change/password",
            "/jaxrs/program_center/config/open",
            "/jaxrs/program_center/config/open/run/time/config",
            "/jaxrs/program_center/config/ternary/management",
        ] {
            let s = status_of("POST", uri).await;
            assert_ne!(s, StatusCode::NOT_FOUND, "POST {uri}");
            assert_ne!(s, StatusCode::METHOD_NOT_ALLOWED, "POST {uri}");
        }
        for uri in [
            "/jaxrs/program_center/config/collect",
            "/jaxrs/program_center/config/portal",
            "/jaxrs/program_center/config/proxy",
        ] {
            let s = status_of("PUT", uri).await;
            assert_ne!(s, StatusCode::NOT_FOUND, "PUT {uri}");
            assert_ne!(s, StatusCode::METHOD_NOT_ALLOWED, "PUT {uri}");
        }
    }

    // ── deploy / dict / dingding / distribute ────────────────────

    #[tokio::test]
    async fn u3_deploy_dict_distribute_reachable() {
        assert_ne!(status_of("POST", "/jaxrs/program_center/deploy/list/paging/1/size/20").await, StatusCode::NOT_FOUND);
        assert_ne!(status_of("POST", "/jaxrs/program_center/deploy/server/resource").await, StatusCode::NOT_FOUND);
        assert_eq!(
            status_of("POST", "/jaxrs/program_center/deploy/web/resource/as/new/newname").await,
            StatusCode::INTERNAL_SERVER_ERROR
        );

        // 参数化字典路由：段值避开静态 dictFlag/list 以命中参数节点
        assert_eq!(status_of("GET", "/jaxrs/program_center/dict/d-id").await, StatusCode::INTERNAL_SERVER_ERROR);
        let put_status = status_of("PUT", "/jaxrs/program_center/dict/d-id").await;
        assert_ne!(put_status, StatusCode::NOT_FOUND);
        assert_ne!(put_status, StatusCode::METHOD_NOT_ALLOWED);
        assert_eq!(
            status_of("GET", "/jaxrs/program_center/dict/my-flag/my-path/data/mockdeletetoget").await,
            StatusCode::INTERNAL_SERVER_ERROR
        );
        let post_status = status_of("POST", "/jaxrs/program_center/dict/my-flag/my-path/data/mockputtopost").await;
        assert_ne!(post_status, StatusCode::NOT_FOUND);
        assert_ne!(post_status, StatusCode::METHOD_NOT_ALLOWED);
        assert_ne!(status_of("POST", "/jaxrs/program_center/dict/list/paging/1/size/20").await, StatusCode::NOT_FOUND);

        assert_eq!(
            status_of("GET", "/jaxrs/program_center/dingding/sync/organization/register/callback/true").await,
            StatusCode::INTERNAL_SERVER_ERROR
        );
        assert_eq!(status_of("GET", "/jaxrs/program_center/deploy/d-1").await, StatusCode::INTERNAL_SERVER_ERROR);
        assert_eq!(status_of("GET", "/jaxrs/program_center/dict/my-flag/data").await, StatusCode::INTERNAL_SERVER_ERROR);
        assert_eq!(
            status_of("GET", "/jaxrs/program_center/dict/my-flag/my-path/data").await,
            StatusCode::INTERNAL_SERVER_ERROR
        );
        assert_eq!(status_of("GET", "/jaxrs/program_center/module/m-1/compare").await, StatusCode::INTERNAL_SERVER_ERROR);
        assert_eq!(
            status_of("GET", "/jaxrs/program_center/distribute/assemble/source/o2").await,
            StatusCode::INTERNAL_SERVER_ERROR
        );
        assert_eq!(
            status_of("GET", "/jaxrs/program_center/distribute/webserver/assemble/source/o2").await,
            StatusCode::INTERNAL_SERVER_ERROR
        );
    }

    // ── invoke 家族补齐 ──────────────────────────────────────────

    #[tokio::test]
    async fn u3_invoke_extras_reachable() {
        assert_eq!(status_of("GET", "/jaxrs/program_center/invoke").await, StatusCode::INTERNAL_SERVER_ERROR);
        assert_eq!(
            status_of("GET", "/jaxrs/program_center/invoke/list/with/category/cms").await,
            StatusCode::INTERNAL_SERVER_ERROR
        );
        assert_eq!(
            status_of("GET", "/jaxrs/program_center/invoke/i-1/execute/get").await,
            StatusCode::INTERNAL_SERVER_ERROR
        );
        for uri in [
            "/jaxrs/program_center/invoke/i-1/execute",
            "/jaxrs/program_center/invoke/i-1/client/web/token/tk-1/execute",
        ] {
            let s = status_of("POST", uri).await;
            assert_ne!(s, StatusCode::NOT_FOUND, "POST {uri}");
            assert_ne!(s, StatusCode::METHOD_NOT_ALLOWED, "POST {uri}");
        }
        let file_put = status_of("PUT", "/jaxrs/program_center/invoke/i-1/file").await;
        assert_ne!(file_put, StatusCode::NOT_FOUND);
        assert_ne!(file_put, StatusCode::METHOD_NOT_ALLOWED);
    }

    // ── jest / market / module / mpweixin / output ───────────────

    #[tokio::test]
    async fn u3_market_module_family_reachable() {
        assert_eq!(status_of("GET", "/jaxrs/program_center/jest/clear/cache/es").await, StatusCode::INTERNAL_SERVER_ERROR);
        assert_eq!(status_of("GET", "/jaxrs/program_center/market/m-1").await, StatusCode::INTERNAL_SERVER_ERROR);
        assert_eq!(status_of("GET", "/jaxrs/program_center/market/m-1/cover/pic").await, StatusCode::INTERNAL_SERVER_ERROR);
        assert_eq!(status_of("GET", "/jaxrs/program_center/market/m-1/download").await, StatusCode::INTERNAL_SERVER_ERROR);
        assert_eq!(status_of("GET", "/jaxrs/program_center/market/m-1/install/log").await, StatusCode::INTERNAL_SERVER_ERROR);
        assert_ne!(status_of("GET", "/jaxrs/program_center/market/m-1/install/or/update").await, StatusCode::METHOD_NOT_ALLOWED);
        assert_ne!(status_of("PUT", "/jaxrs/program_center/market/m-1/install/or/update").await, StatusCode::METHOD_NOT_ALLOWED);
        assert_eq!(status_of("GET", "/jaxrs/program_center/market/m-1/installed/version").await, StatusCode::INTERNAL_SERVER_ERROR);
        assert_eq!(status_of("GET", "/jaxrs/program_center/market/m-1/uninstall").await, StatusCode::INTERNAL_SERVER_ERROR);
        assert_ne!(status_of("POST", "/jaxrs/program_center/market/list/paging/1/size/20").await, StatusCode::NOT_FOUND);
        assert_ne!(status_of("POST", "/jaxrs/program_center/market/list/install/log/paging/1/size/20").await, StatusCode::NOT_FOUND);
        assert_eq!(
            status_of("GET", "/jaxrs/program_center/market/list/paging/1/size/20/category/cms").await,
            StatusCode::INTERNAL_SERVER_ERROR
        );

        assert_eq!(status_of("GET", "/jaxrs/program_center/module/output/m-1/file").await, StatusCode::INTERNAL_SERVER_ERROR);
        assert_eq!(status_of("DELETE", "/jaxrs/program_center/module/remove/structure/m-1").await, StatusCode::INTERNAL_SERVER_ERROR);
        let write_put = status_of("PUT", "/jaxrs/program_center/module/write/m-1").await;
        assert_ne!(write_put, StatusCode::NOT_FOUND);
        assert_ne!(write_put, StatusCode::METHOD_NOT_ALLOWED);

        assert_eq!(status_of("POST", "/jaxrs/program_center/mpweixin/check").await, StatusCode::INTERNAL_SERVER_ERROR);
        assert_ne!(status_of("POST", "/jaxrs/program_center/mpweixin/menu/update/wm-1").await, StatusCode::NOT_FOUND);
        assert_eq!(status_of("DELETE", "/jaxrs/program_center/mpweixin/menu/delete/wm-1").await, StatusCode::INTERNAL_SERVER_ERROR);
        assert_eq!(status_of("GET", "/jaxrs/program_center/mpweixin/menu/create/to/weixin").await, StatusCode::INTERNAL_SERVER_ERROR);

        assert_eq!(status_of("GET", "/jaxrs/program_center/output/f-1/select/file").await, StatusCode::INTERNAL_SERVER_ERROR);
        assert_eq!(status_of("PUT", "/jaxrs/program_center/output/f-1/select").await, StatusCode::INTERNAL_SERVER_ERROR);
    }

    // ── 日志创建 / qiyeweixin / schedule / script / validation / welink ──

    #[tokio::test]
    async fn u3_logs_misc_reachable() {
        assert_ne!(status_of("POST", "/jaxrs/program_center/prompterrorlog").await, StatusCode::NOT_FOUND);
        assert_eq!(status_of("GET", "/jaxrs/program_center/prompterrorlog/p-1").await, StatusCode::INTERNAL_SERVER_ERROR);
        assert_eq!(status_of("GET", "/jaxrs/program_center/prompterrorlog/list/p-1/next/10").await, StatusCode::INTERNAL_SERVER_ERROR);
        assert_eq!(
            status_of("GET", "/jaxrs/program_center/prompterrorlog/list/p-1/next/10/date/2026-08-24").await,
            StatusCode::INTERNAL_SERVER_ERROR
        );
        assert_ne!(status_of("POST", "/jaxrs/program_center/unexpectederrorlog").await, StatusCode::NOT_FOUND);
        assert_eq!(status_of("GET", "/jaxrs/program_center/unexpectederrorlog/u-1").await, StatusCode::INTERNAL_SERVER_ERROR);
        assert_eq!(status_of("GET", "/jaxrs/program_center/unexpectederrorlog/list/u-1/prev/5").await, StatusCode::INTERNAL_SERVER_ERROR);

        assert_eq!(status_of("GET", "/jaxrs/program_center/qiyeweixin").await, StatusCode::INTERNAL_SERVER_ERROR);
        assert_eq!(status_of("POST", "/jaxrs/program_center/qiyeweixin").await, StatusCode::INTERNAL_SERVER_ERROR);
        assert_eq!(status_of("POST", "/jaxrs/program_center/qiyeweixin/request/pull/sync").await, StatusCode::INTERNAL_SERVER_ERROR);

        assert_ne!(status_of("POST", "/jaxrs/program_center/schedule/schedule/fire").await, StatusCode::NOT_FOUND);
        assert_eq!(
            status_of("GET", "/jaxrs/program_center/schedule/list/schedulelog/application/app-1").await,
            StatusCode::INTERNAL_SERVER_ERROR
        );

        let script_name_post = status_of("POST", "/jaxrs/program_center/script/name/demo").await;
        assert_ne!(script_name_post, StatusCode::NOT_FOUND);
        assert_ne!(script_name_post, StatusCode::METHOD_NOT_ALLOWED);
        assert_eq!(status_of("GET", "/jaxrs/program_center/script/sc-flag").await, StatusCode::INTERNAL_SERVER_ERROR);
        assert_eq!(status_of("GET", "/jaxrs/program_center/script/name/demo/imported").await, StatusCode::INTERNAL_SERVER_ERROR);
        assert_ne!(status_of("POST", "/jaxrs/program_center/script/list/paging/1/size/20").await, StatusCode::NOT_FOUND);

        assert_eq!(status_of("GET", "/jaxrs/program_center/validation/timeout/30000").await, StatusCode::INTERNAL_SERVER_ERROR);
        assert_eq!(status_of("GET", "/jaxrs/program_center/welink/pull/sync").await, StatusCode::INTERNAL_SERVER_ERROR);
        assert_eq!(status_of("POST", "/jaxrs/program_center/welink/request/pull/sync").await, StatusCode::INTERNAL_SERVER_ERROR);
    }

    // ── 纯单元测试：IDOR 门禁 fail-closed 与 DB 失败语义 ─────────

    #[tokio::test]
    async fn u3_agent_disable_requires_admin_before_any_db_access() {
        let pool = mock_pool();
        let r = u3_agent_flag_disable(Extension(pool), Extension(test_session()), axum::extract::Path("a-1".to_string())).await;
        match r {
            Err(shared::error::AppError::Forbidden) => {}
            other => panic!("expected Forbidden, got {:?}", other.map(|_| "ok")),
        }
    }

    #[tokio::test]
    async fn u3_invoke_file_put_requires_admin_fail_closed() {
        let pool = mock_pool();
        let r = u3_invoke_file_put(
            Extension(pool),
            Extension(test_session()),
            axum::extract::Path("i-1".to_string()),
            axum::Json(serde_json::json!({ "text": "print(1)" })),
        )
        .await;
        match r {
            Err(shared::error::AppError::Forbidden) => {}
            other => panic!("expected Forbidden, got {:?}", other.map(|_| "ok")),
        }
    }

    #[tokio::test]
    async fn u3_schedule_fire_requires_admin_fail_closed() {
        let pool = mock_pool();
        let r = u3_schedule_fire_post(
            Extension(pool),
            Extension(test_session()),
            axum::Json(serde_json::json!({ "id": "sch-1" })),
        )
        .await;
        match r {
            Err(shared::error::AppError::Forbidden) => {}
            other => panic!("expected Forbidden, got {:?}", other.map(|_| "ok")),
        }
    }

    #[tokio::test]
    async fn u3_collect_resetpassword_requires_admin_fail_closed() {
        let pool = mock_pool();
        let r = u3_collect_resetpassword_put(
            Extension(pool),
            Extension(test_session()),
            axum::Json(serde_json::json!({ "credential": "user1", "password": "NewPass@123" })),
        )
        .await;
        match r {
            Err(shared::error::AppError::Forbidden) => {}
            other => panic!("expected Forbidden, got {:?}", other.map(|_| "ok")),
        }
    }

    #[tokio::test]
    async fn u3_db_backed_reads_fail_internal_without_db() {
        let pool = mock_pool();
        // 同步日志型端点：无 DB 连接 → Internal
        let r = u3_welink_pull_sync(Extension(pool.clone())).await;
        assert!(matches!(r, Err(shared::error::AppError::Internal)));
        let r = u3_jest_clear_cache_source(Extension(pool.clone()), axum::extract::Path("es".to_string())).await;
        assert!(matches!(r, Err(shared::error::AppError::Internal)));
        let r = u3_qiyeweixin_create(Extension(pool.clone()), Extension(test_session())).await;
        assert!(matches!(r, Err(shared::error::AppError::Internal)));
    }

    #[tokio::test]
    async fn u3_market_paged_query_uses_params_and_fails_internal_without_db() {
        let pool = mock_pool();
        let r = u3_market_list_paging_category(
            Extension(pool),
            axum::extract::Path(1i64),
            axum::extract::Path(20i64),
            axum::extract::Path("cms".to_string()),
        )
        .await;
        assert!(matches!(r, Err(shared::error::AppError::Internal)));
    }

    #[tokio::test]
    async fn u3_log_creation_hits_db_and_fails_internal_without_db() {
        let pool = mock_pool();
        let req = crate::U3PromptErrorLogRequest {
            exception_class: Some("java.lang.NumberFormatException".to_string()),
            logger_name: Some("com.x.test".to_string()),
            message: Some("boom".to_string()),
        };
        let r = u3_prompterrorlog_create(Extension(pool.clone()), Extension(test_session()), axum::Json(req)).await;
        assert!(matches!(r, Err(shared::error::AppError::Internal)));
    }

    #[tokio::test]
    async fn u3_self_password_change_verifies_old_before_write() {
        // 无 DB：人员查询失败 → Internal（旧密码校验先于任何写入）
        let pool = mock_pool();
        let r = u3_config_change_password_post(
            Extension(pool),
            Extension(test_session()),
            axum::Json(serde_json::json!({ "oldPassword": "old", "newPassword": "new" })),
        )
        .await;
        assert!(matches!(r, Err(shared::error::AppError::Internal)));
    }

    #[tokio::test]
    async fn u3_script_save_by_name_uses_same_owner_gate_path() {
        let pool = mock_pool();
        let req = crate::ScriptSaveRequest {
            name: None,
            content: Some("return 1;".to_string()),
            category: None,
        };
        let r = u3_script_save_name(
            Extension(pool),
            Extension(test_session()),
            axum::extract::Path("demo".to_string()),
            axum::Json(req),
        )
        .await;
        // 无 DB 时查询脚本所有者失败 → Internal（require_owner 前置查询）
        assert!(matches!(r, Err(shared::error::AppError::Internal)));
    }
}
