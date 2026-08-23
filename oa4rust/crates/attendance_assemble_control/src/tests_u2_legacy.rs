//! plan002 U2 legacy 族闭合端点的路由可达测试与单元测试。
//!
//! 路由测试使用 mock_pool（无法建连）：请求命中路由后 handler 返回 500，
//! 断言 500（而非 404）即可证明路由已注册且可达。
//! Router 构建本身会校验路径唯一性——若存在重复注册将直接 panic。

#[cfg(test)]
mod u2_legacy_tests {
    use crate::attendance_assemble_control_router;
    use axum::body::Body;
    use axum::http::{Request, StatusCode};
    use shared::testing::mock_pool;
    use tower::ServiceExt;

    async fn status_of(method: &str, uri: &str) -> StatusCode {
        let app = attendance_assemble_control_router(mock_pool());
        // 不发送 content-type：空 body + json 头会使 Json/Option<Json> 提取失败，
        // 掩盖路由可达性断言
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

    const BASE: &str = "/jaxrs/attendance/assemble/control";

    // ── dingding / qywx 路由可达 ───────────────────────────────

    #[tokio::test]
    async fn u2l_dingding_core_routes_reachable() {
        for (method, uri) in [
            ("DELETE", "/dingding/all"),
            (
                "GET",
                "/dingding/sync/from/2026-08-01/to/2026-08-07/start",
            ),
            ("GET", "/dingding/sync/list"),
            ("PUT", "/dingding/attendance/list/x/next/20"),
        ] {
            assert_eq!(
                status_of(method, &format!("{}{}", BASE, uri)).await,
                StatusCode::INTERNAL_SERVER_ERROR,
                "{} {} not routed",
                method,
                uri
            );
        }
    }

    #[tokio::test]
    async fn u2l_dingding_statistic_triggers_reachable() {
        assert_eq!(
            status_of("GET", &format!("{}/dingding/statistic/person/year/2026/month/08", BASE)).await,
            StatusCode::INTERNAL_SERVER_ERROR
        );
        assert_eq!(
            status_of(
                "GET",
                &format!("{}/dingding/statistic/unit/year/2026/month/08/day/22", BASE)
            )
            .await,
            StatusCode::INTERNAL_SERVER_ERROR
        );
    }

    #[tokio::test]
    async fn u2l_dingdingstatistic_queries_reachable() {
        for uri in [
            "/dingdingstatistic/person/p1/2026/08",
            "/dingdingstatistic/person/unit/u1/2026/08",
            "/dingdingstatistic/unit/u1/2026/08",
        ] {
            assert_eq!(
                status_of("GET", &format!("{}{}", BASE, uri)).await,
                StatusCode::INTERNAL_SERVER_ERROR,
                "{} not routed",
                uri
            );
        }
    }

    #[tokio::test]
    async fn u2l_qywx_routes_reachable() {
        for (method, uri) in [
            ("DELETE", "/qywx/all"),
            ("GET", "/qywx/sync/from/2026-08-01/to/2026-08-07/start"),
            ("GET", "/qywx/sync/list"),
            ("PUT", "/qywx/attendance/list/x/next/20"),
            ("GET", "/qywx/statistic/person/year/2026/month/08"),
            ("GET", "/qywx/statistic/unit/year/2026/month/08/day/22"),
            ("GET", "/qywxstatistic/person/p1/2026/08"),
            ("GET", "/qywxstatistic/person/unit/u1/2026/08"),
            ("GET", "/qywxstatistic/unit/u1/2026/08"),
        ] {
            assert_eq!(
                status_of(method, &format!("{}{}", BASE, uri)).await,
                StatusCode::INTERNAL_SERVER_ERROR,
                "{} {} not routed",
                method,
                uri
            );
        }
    }

    // ── v2 appeal / detail / group 路由可达 ────────────────────

    #[tokio::test]
    async fn u2l_v2_appeal_routes_reachable() {
        for (method, uri) in [
            ("POST", "/v2/appeal/list/1/size/20"),
            ("POST", "/v2/appeal/list/manager/1/size/20"),
            ("GET", "/v2/appeal/ap-1"),
            ("GET", "/v2/appeal/ap-1/manager/status"),
            ("GET", "/v2/appeal/ap-1/start/check"),
            ("GET", "/v2/appeal/ap-1/reset/status"),
        ] {
            assert_eq!(
                status_of(method, &format!("{}{}", BASE, uri)).await,
                StatusCode::INTERNAL_SERVER_ERROR,
                "{} {} not routed",
                method,
                uri
            );
        }
    }

    #[tokio::test]
    async fn u2l_v2_detail_and_group_rebuild_reachable() {
        for (method, uri) in [
            ("GET", "/v2/detail/rebuild/person/p1/date/2026-08-22"),
            ("GET", "/v2/detail/statistic/d-1/list/record"),
            ("POST", "/v2/detail/statistic/filter"),
            ("POST", "/v2/detail/statistic/export/filter"),
            ("GET", "/v2/group/rebuild/detail/group/g-1/date/2026-08-22"),
        ] {
            assert_eq!(
                status_of(method, &format!("{}{}", BASE, uri)).await,
                StatusCode::INTERNAL_SERVER_ERROR,
                "{} {} not routed",
                method,
                uri
            );
        }
    }

    #[tokio::test]
    async fn u2l_v2_groupschedule_static_paths_win_over_params() {
        // config/list 静态段不得被 {id} 或其他参数段吞掉
        for (method, uri) in [
            ("POST", "/v2/groupschedule"),
            ("POST", "/v2/groupschedule/list/filter"),
            ("GET", "/v2/groupschedule/config/group/g-1"),
            ("GET", "/v2/groupschedule/list/group/g-1/month/2026-08"),
        ] {
            assert_eq!(
                status_of(method, &format!("{}{}", BASE, uri)).await,
                StatusCode::INTERNAL_SERVER_ERROR,
                "{} {} not routed",
                method,
                uri
            );
        }
    }

    // ── v2 mobile（含 Java 空格路径）路由可达 ──────────────────

    #[tokio::test]
    async fn u2l_v2_mobile_check_space_path_both_encodings_routed() {
        // Java @Path("check/ from/out")：线上请求以 %20 编码传输（裸空格是非法 Uri 字符），
        // %20 形态注册必须可达；handler 因缺 Session 扩展返回 500 而非 404
        assert_eq!(
            status_of(
                "POST",
                &format!("{}/v2/mobile/check/%20from/out", BASE)
            )
            .await,
            StatusCode::INTERNAL_SERVER_ERROR
        );
    }

    #[tokio::test]
    async fn u2l_v2_mobile_precheck_routed() {
        assert_eq!(
            status_of("GET", &format!("{}/v2/mobile/check/pre", BASE)).await,
            StatusCode::INTERNAL_SERVER_ERROR
        );
        assert_eq!(
            status_of("POST", &format!("{}/v2/mobile/check", BASE)).await,
            StatusCode::INTERNAL_SERVER_ERROR
        );
    }

    // ── v2 my / record / workplace / leave template 路由可达 ───

    #[tokio::test]
    async fn u2l_v2_my_routes_reachable() {
        // version 为纯静态契约（无提取器）：200 即证明已路由
        assert_eq!(
            status_of("GET", &format!("{}/v2/my/version", BASE)).await,
            StatusCode::OK
        );
        for (method, uri) in [
            ("GET", "/v2/my/controls"),
            ("POST", "/v2/my/detail/list"),
        ] {
            assert_eq!(
                status_of(method, &format!("{}{}", BASE, uri)).await,
                StatusCode::INTERNAL_SERVER_ERROR,
                "{} {} not routed",
                method,
                uri
            );
        }
        // rest/date/check 缺 body 时 handler 校验日期返回 400（≠404 证明路由命中）
        assert_eq!(
            status_of("POST", &format!("{}/v2/my/rest/date/check", BASE)).await,
            StatusCode::BAD_REQUEST
        );
    }

    #[tokio::test]
    async fn u2l_v2_record_and_leave_template_routes_reachable() {
        for (method, uri) in [
            ("GET", "/v2/record/delete/people/p1/date/2026-08-22"),
            ("POST", "/v2/record/import"),
            ("POST", "/v2/record/import/daily"),
        ] {
            assert_eq!(
                status_of(method, &format!("{}{}", BASE, uri)).await,
                StatusCode::INTERNAL_SERVER_ERROR,
                "{} {} not routed",
                method,
                uri
            );
        }
        // template 端点为纯静态契约（无提取器）：200 即证明已路由
        assert_eq!(
            status_of("GET", &format!("{}/v2/record/template", BASE)).await,
            StatusCode::OK
        );
        assert_eq!(
            status_of("GET", &format!("{}/v2/leave/template", BASE)).await,
            StatusCode::OK
        );
    }

    #[tokio::test]
    async fn u2l_v2_workplace_routes_reachable() {
        for (method, uri) in [
            ("POST", "/v2/workplace"),
            ("DELETE", "/v2/workplace/w-1"),
            ("GET", "/v2/workplace/list/all"),
        ] {
            assert_eq!(
                status_of(method, &format!("{}{}", BASE, uri)).await,
                StatusCode::INTERNAL_SERVER_ERROR,
                "{} {} not routed",
                method,
                uri
            );
        }
        // list/ids 要求 JSON body：无 content-type 时提取器拒绝 415（≠404 证明路由命中）
        assert_eq!(
            status_of("POST", &format!("{}/v2/workplace/list/ids", BASE)).await,
            StatusCode::UNSUPPORTED_MEDIA_TYPE
        );
    }

    // ── legacy 动词链补齐可达 ─────────────────────────────────

    #[tokio::test]
    async fn u2l_legacy_verb_chains_now_routed() {
        // 上轮缺口：Java GET mobile/my、GET mobilepreview、PUT mobile filter
        // my/mobilepreview 要求 JSON body：GET 无 content-type → 415（≠404 证明路由命中）
        assert_eq!(
            status_of("GET", &format!("{}/attendancedetail/mobile/my", BASE)).await,
            StatusCode::UNSUPPORTED_MEDIA_TYPE
        );
        assert_eq!(
            status_of("GET", &format!("{}/attendancedetail/mobile/mobilepreview", BASE)).await,
            StatusCode::UNSUPPORTED_MEDIA_TYPE
        );
        assert_eq!(
            status_of(
                "PUT",
                &format!("{}/attendancedetail/mobile/filter/list/page/1/count/20", BASE)
            )
            .await,
            StatusCode::INTERNAL_SERVER_ERROR
        );
    }

    // ── 纯单元测试 ─────────────────────────────────────────────

    #[test]
    fn u2l_date_string_from_unix_known_timestamps() {
        assert_eq!(crate::date_string_from_unix(0), "1970-01-01");
        // 2026-08-22 00:00:00 UTC
        assert_eq!(crate::date_string_from_unix(1_787_356_800), "2026-08-22");
        // 2024-02-29 leap day
        assert_eq!(crate::date_string_from_unix(1_709_164_800), "2024-02-29");
        assert_eq!(crate::date_string_from_unix(86_399), "1970-01-01");
        // 次日边界：86400 秒
        assert_eq!(crate::date_string_from_unix(86_400), "1970-01-02");
    }

    #[test]
    fn u2l_normalize_key_trims_and_folds_case() {
        // 归一化查重的基础函数：trim + lowercase
        assert_eq!(crate::normalize_key("  Zhang San \n"), "zhang san");
        assert_eq!(crate::normalize_key("U1@X"), "u1@x");
        assert_eq!(crate::normalize_key(""), "");
        // 归一化等价性：不同书写形式映射到同一键
        let a = crate::normalize_key(" PersonA ");
        let b = crate::normalize_key("persona");
        assert_eq!(a, b);
    }

    #[test]
    fn u2l_leave_template_shape_matches_java_columns() {
        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap();
        let resp = rt.block_on(crate::v2_leave_template()).unwrap();
        let body = resp.0;
        assert_eq!(body.r#type.as_deref(), Some("success"));
        let data = body.data.expect("template data");
        let columns = data["columns"].as_array().expect("columns array");
        assert!(columns.contains(&serde_json::json!("person")));
        assert!(columns.contains(&serde_json::json!("leaveType")));
        assert_eq!(data["fileName"], "请假导入模板.xlsx");
    }

    #[test]
    fn u2l_record_template_and_version_contract() {
        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap();

        let resp = rt.block_on(crate::v2_record_template()).unwrap();
        let data = resp.0.data.expect("record template data");
        assert!(data["columns"].as_array().expect("columns").len() >= 5);

        let resp = rt.block_on(crate::v2_my_version()).unwrap();
        let data = resp.0.data.expect("version data");
        // 对齐 ActionVersion.Wo.version = "2"
        assert_eq!(data["version"], "2");
    }
}
