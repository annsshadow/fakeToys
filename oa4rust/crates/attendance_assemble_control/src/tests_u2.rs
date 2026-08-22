//! plan002 U2 新增端点的路由可达测试与单元测试。
//!
//! 路由测试使用 mock_pool（无法建连）：请求命中路由后 handler 返回 500，
//! 断言 500（而非 404）即可证明路由已注册且可达。
//! Router 构建本身会校验路径唯一性——若存在重复注册将直接 panic。

#[cfg(test)]
mod u2_tests {
    use crate::attendance_assemble_control_router;
    use axum::body::Body;
    use axum::http::{Request, StatusCode};
    use shared::response::ActionResult;
    use shared::testing::mock_pool;
    use tower::ServiceExt;

    async fn status_of(method: &str, uri: &str) -> StatusCode {
        let app = attendance_assemble_control_router(mock_pool());
        // 不发送 content-type：空 body + json 头会使 Option<Json> 提取返回 400，
        // 掩盖路由可达性断言（无头时 Option<Json> 解析为 None）
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

    // ── v2 config ──────────────────────────────────────────────

    #[tokio::test]
    async fn u2_v2_config_get_reachable() {
        assert_eq!(
            status_of("GET", "/jaxrs/attendance/assemble/control/v2/config").await,
            StatusCode::INTERNAL_SERVER_ERROR
        );
    }

    #[tokio::test]
    async fn u2_v2_config_post_chained_on_same_path() {
        assert_eq!(
            status_of("POST", "/jaxrs/attendance/assemble/control/v2/config").await,
            StatusCode::INTERNAL_SERVER_ERROR
        );
    }

    #[tokio::test]
    async fn u2_v2_config_person_routes_reachable() {
        assert_eq!(
            status_of("GET", "/jaxrs/attendance/assemble/control/v2/config/person").await,
            StatusCode::INTERNAL_SERVER_ERROR
        );
        assert_eq!(
            status_of("POST", "/jaxrs/attendance/assemble/control/v2/config/person").await,
            StatusCode::INTERNAL_SERVER_ERROR
        );
    }

    // ── v2 group ───────────────────────────────────────────────

    #[tokio::test]
    async fn u2_v2_group_create_and_list_reachable() {
        assert_eq!(
            status_of("POST", "/jaxrs/attendance/assemble/control/v2/group").await,
            StatusCode::INTERNAL_SERVER_ERROR
        );
        // 路径含字面量段 size：/list/{page}/size/{size}
        assert_eq!(
            status_of(
                "POST",
                "/jaxrs/attendance/assemble/control/v2/group/list/1/size/20"
            )
            .await,
            StatusCode::INTERNAL_SERVER_ERROR
        );
    }

    #[tokio::test]
    async fn u2_v2_group_static_paths_win_over_param_id() {
        // {id} 与静态段 person 共存：person 路径不得落入 {id} 通配
        assert_eq!(
            status_of(
                "GET",
                "/jaxrs/attendance/assemble/control/v2/group/person/u001/date/2026-08-01"
            )
            .await,
            StatusCode::INTERNAL_SERVER_ERROR
        );
        assert_eq!(
            status_of(
                "GET",
                "/jaxrs/attendance/assemble/control/v2/group/some-id/delete"
            )
            .await,
            StatusCode::INTERNAL_SERVER_ERROR
        );
        assert_eq!(
            status_of(
                "GET",
                "/jaxrs/attendance/assemble/control/v2/group/some-id/refresh/participate"
            )
            .await,
            StatusCode::INTERNAL_SERVER_ERROR
        );
    }

    #[tokio::test]
    async fn u2_v2_group_get_by_id_reachable() {
        assert_eq!(
            status_of(
                "GET",
                "/jaxrs/attendance/assemble/control/v2/group/some-id"
            )
            .await,
            StatusCode::INTERNAL_SERVER_ERROR
        );
    }

    // ── v2 shift ───────────────────────────────────────────────

    #[tokio::test]
    async fn u2_v2_shift_write_and_list_reachable() {
        for uri in [
            "/jaxrs/attendance/assemble/control/v2/shift/create",
            "/jaxrs/attendance/assemble/control/v2/shift/update",
            "/jaxrs/attendance/assemble/control/v2/shift/list/1/size/20",
        ] {
            assert_eq!(status_of("POST", uri).await, StatusCode::INTERNAL_SERVER_ERROR);
        }
    }

    #[tokio::test]
    async fn u2_v2_shift_read_and_delete_reachable() {
        assert_eq!(
            status_of(
                "GET",
                "/jaxrs/attendance/assemble/control/v2/shift/abc"
            )
            .await,
            StatusCode::INTERNAL_SERVER_ERROR
        );
        // 静态段 delete 优先于 {id} 参数段
        assert_eq!(
            status_of(
                "GET",
                "/jaxrs/attendance/assemble/control/v2/shift/delete/abc"
            )
            .await,
            StatusCode::INTERNAL_SERVER_ERROR
        );
    }

    // ── v2 leave ───────────────────────────────────────────────

    #[tokio::test]
    async fn u2_v2_leave_routes_reachable() {
        assert_eq!(
            status_of("POST", "/jaxrs/attendance/assemble/control/v2/leave").await,
            StatusCode::INTERNAL_SERVER_ERROR
        );
        assert_eq!(
            status_of(
                "GET",
                "/jaxrs/attendance/assemble/control/v2/leave/delete/l-1"
            )
            .await,
            StatusCode::INTERNAL_SERVER_ERROR
        );
        assert_eq!(
            status_of(
                "POST",
                "/jaxrs/attendance/assemble/control/v2/leave/list/1/size/20"
            )
            .await,
            StatusCode::INTERNAL_SERVER_ERROR
        );
    }

    #[tokio::test]
    async fn u2_v2_leave_import_and_result_reachable() {
        assert_eq!(
            status_of(
                "POST",
                "/jaxrs/attendance/assemble/control/v2/leave/import"
            )
            .await,
            StatusCode::INTERNAL_SERVER_ERROR
        );
        assert_eq!(
            status_of(
                "GET",
                "/jaxrs/attendance/assemble/control/v2/leave/import/result/flag/flag-x"
            )
            .await,
            StatusCode::INTERNAL_SERVER_ERROR
        );
    }

    // ── v2 record / detail / my ────────────────────────────────

    #[tokio::test]
    async fn u2_v2_record_routes_reachable() {
        assert_eq!(
            status_of(
                "GET",
                "/jaxrs/attendance/assemble/control/v2/record/r-1"
            )
            .await,
            StatusCode::INTERNAL_SERVER_ERROR
        );
        assert_eq!(
            status_of(
                "POST",
                "/jaxrs/attendance/assemble/control/v2/record/list/1/size/20"
            )
            .await,
            StatusCode::INTERNAL_SERVER_ERROR
        );
    }

    #[tokio::test]
    async fn u2_v2_detail_list_and_my_statistic_reachable() {
        assert_eq!(
            status_of(
                "POST",
                "/jaxrs/attendance/assemble/control/v2/detail/list/1/size/50"
            )
            .await,
            StatusCode::INTERNAL_SERVER_ERROR
        );
        assert_eq!(
            status_of(
                "POST",
                "/jaxrs/attendance/assemble/control/v2/my/statistic"
            )
            .await,
            StatusCode::INTERNAL_SERVER_ERROR
        );
    }

    // ── legacy POST 创建 ───────────────────────────────────────

    #[tokio::test]
    async fn u2_legacy_post_creates_reachable() {
        for uri in [
            "/jaxrs/attendance/assemble/control/attendanceadmin",
            "/jaxrs/attendance/assemble/control/attendanceselfholiday",
            "/jaxrs/attendance/assemble/control/workplace",
            "/jaxrs/attendance/assemble/control/selfholidaysimple",
        ] {
            assert_eq!(status_of("POST", uri).await, StatusCode::INTERNAL_SERVER_ERROR);
        }
    }

    // ── legacy DELETE（链式注册到既有 GET 路径） ────────────────

    #[tokio::test]
    async fn u2_legacy_deletes_chained_on_existing_paths() {
        for uri in [
            "/jaxrs/attendance/assemble/control/attendanceadmin/a-1",
            "/jaxrs/attendance/assemble/control/attendancedetail/d-1",
            "/jaxrs/attendance/assemble/control/workplace/w-1",
            "/jaxrs/attendance/assemble/control/selfholidaysimple/docId/doc-1",
        ] {
            assert_eq!(status_of("DELETE", uri).await, StatusCode::INTERNAL_SERVER_ERROR);
        }
    }

    #[tokio::test]
    async fn u2_toggle_path_accepts_both_verbs_without_panic() {
        // 原代码对同一路径注册两次会导致 axum 构建 panic；合并后两个动词都应可达
        assert_ne!(
            status_of(
                "POST",
                "/jaxrs/attendance/assemble/control/rule/r-1/toggle"
            )
            .await,
            StatusCode::NOT_FOUND
        );
        assert_ne!(
            status_of(
                "PUT",
                "/jaxrs/attendance/assemble/control/rule/r-1/toggle"
            )
            .await,
            StatusCode::NOT_FOUND
        );
    }

    #[tokio::test]
    async fn u2_router_sanity_known_legacy_route_still_works() {
        assert_eq!(
            status_of(
                "GET",
                "/jaxrs/attendance/assemble/control/workplace/list/all"
            )
            .await,
            StatusCode::INTERNAL_SERVER_ERROR
        );
    }

    #[tokio::test]
    async fn u2_verb_chains_align_with_java_methods() {
        // Java 端 PUT 承载 filter 查询：链式注册后 PUT 可达（不再 405/404）
        for (method, uri) in [
            (
                "PUT",
                "/jaxrs/attendance/assemble/control/attendancedetail/filter/list",
            ),
            (
                "PUT",
                "/jaxrs/attendance/assemble/control/statisticshow/filter/unitDay/list/x/next/5",
            ),
            (
                "GET",
                "/jaxrs/attendance/assemble/control/statistic/do",
            ),
            (
                "GET",
                "/jaxrs/attendance/assemble/control/attendancesetting/enable/type",
            ),
            (
                "DELETE",
                "/jaxrs/attendance/assemble/control/attendancedetail/mobile/m-1",
            ),
        ] {
            assert_eq!(status_of(method, uri).await, StatusCode::INTERNAL_SERVER_ERROR);
        }
    }

    // ── 纯单元测试 ─────────────────────────────────────────────

    #[test]
    fn u2_month_prefix_from_unix_known_timestamps() {
        assert_eq!(crate::month_prefix_from_unix(0), "1970-01");
        // 2026-08-22 00:00:00 UTC
        assert_eq!(crate::month_prefix_from_unix(1_787_356_800), "2026-08");
        // 2024-02-29 leap day, 00:00:00 UTC
        assert_eq!(crate::month_prefix_from_unix(1_709_164_800), "2024-02");
    }

    #[test]
    fn u2_json_page_validates_bounds() {
        assert_eq!(crate::json_page(1, 20).unwrap(), (20, 0));
        assert_eq!(crate::json_page(3, 50).unwrap(), (50, 100));
        assert!(crate::json_page(0, 10).is_err());
        assert!(crate::json_page(1, 0).is_err());
        assert!(crate::json_page(1, 501).is_err());
    }

    #[test]
    fn u2_json_helpers_extract_fields() {
        let body = serde_json::json!({
            "name": "group-a",
            "unitList": ["u1", "u2"],
            "empty": null
        });
        assert_eq!(crate::json_str(&body, "name"), "group-a");
        assert_eq!(crate::json_str(&body, "missing"), "");
        assert_eq!(crate::json_join(&body, "unitList"), "u1,u2");
        assert_eq!(crate::json_join(&body, "missing"), "");
        assert_eq!(crate::json_opt_str(&body, "empty"), None);
    }

    #[test]
    fn u2_action_result_error_carries_message() {
        let r = ActionResult::<serde_json::Value>::error("attendance group not found");
        assert_eq!(r.r#type.as_deref(), Some("error"));
        assert_eq!(r.message.as_deref(), Some("attendance group not found"));
        assert!(r.data.is_none());
    }
}
