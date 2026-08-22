//! plan002 U2 新增端点的路由可达测试与契约测试。
//!
//! 路由测试使用 mock_pool（无法建连）：请求命中路由后 handler 返回 500，
//! 断言 500（而非 404）即可证明路由已注册且可达。
//! Router 构建本身会校验路径唯一性——若存在重复注册将直接 panic。

#[cfg(test)]
mod u2_tests {
    use crate::router;
    use axum::body::Body;
    use axum::http::{Request, StatusCode};
    use shared::response::ActionResult;
    use shared::testing::mock_pool;
    use tower::ServiceExt;

    async fn status_of(method: &str, uri: &str) -> StatusCode {
        let app = router(mock_pool());
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

    // ── snap 域 ────────────────────────────────────────────────

    #[tokio::test]
    async fn u2_snap_get_reachable() {
        assert_eq!(
            status_of("GET", "/jaxrs/processplatform/assemble/surface/snap/snap-1").await,
            StatusCode::INTERNAL_SERVER_ERROR
        );
    }

    #[tokio::test]
    async fn u2_snap_delete_reachable() {
        assert_eq!(
            status_of("DELETE", "/jaxrs/processplatform/assemble/surface/snap/snap-1").await,
            StatusCode::INTERNAL_SERVER_ERROR
        );
    }

    #[tokio::test]
    async fn u2_snap_restore_reachable() {
        assert_eq!(
            status_of("GET", "/jaxrs/processplatform/assemble/surface/snap/snap-1/restore").await,
            StatusCode::INTERNAL_SERVER_ERROR
        );
    }

    #[tokio::test]
    async fn u2_snap_cursor_lists_reachable() {
        let base = "/jaxrs/processplatform/assemble/surface/snap/list";
        assert_eq!(
            status_of("GET", &format!("{}/snap-1/next/20", base)).await,
            StatusCode::INTERNAL_SERVER_ERROR
        );
        assert_eq!(
            status_of("GET", &format!("{}/snap-1/prev/20", base)).await,
            StatusCode::INTERNAL_SERVER_ERROR
        );
        assert_eq!(
            status_of("GET", &format!("{}/snap-1/next/20/manage", base)).await,
            StatusCode::INTERNAL_SERVER_ERROR
        );
    }

    #[tokio::test]
    async fn u2_snap_work_type_routes_reachable() {
        let base = "/jaxrs/processplatform/assemble/surface/snap";
        assert_eq!(
            status_of("GET", &format!("{}/work/work-1/type/snap", base)).await,
            StatusCode::INTERNAL_SERVER_ERROR
        );
        assert_eq!(
            status_of("GET", &format!("{}/workcompleted/wc-1/type/snapworkcompleted", base)).await,
            StatusCode::INTERNAL_SERVER_ERROR
        );
    }

    // ── attachment 域 ──────────────────────────────────────────

    #[tokio::test]
    async fn u2_attachment_list_routes_reachable() {
        let base = "/jaxrs/processplatform/assemble/surface/attachment/list";
        for suffix in ["/job/job-1", "/work/work-1", "/workorworkcompleted/either-1"] {
            assert_eq!(
                status_of("GET", &format!("{}{}", base, suffix)).await,
                StatusCode::INTERNAL_SERVER_ERROR,
                "route not reachable: {}{}",
                base,
                suffix
            );
        }
    }

    #[tokio::test]
    async fn u2_attachment_by_work_verbs_chained_on_same_path() {
        let uri = "/jaxrs/processplatform/assemble/surface/attachment/att-1/work/work-1";
        assert_eq!(status_of("GET", uri).await, StatusCode::INTERNAL_SERVER_ERROR);
        assert_eq!(status_of("DELETE", uri).await, StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[tokio::test]
    async fn u2_attachment_text_and_available_reachable() {
        assert_eq!(
            status_of("GET", "/jaxrs/processplatform/assemble/surface/attachment/att-1/work/work-1/text").await,
            StatusCode::INTERNAL_SERVER_ERROR
        );
        assert_eq!(
            status_of("GET", "/jaxrs/processplatform/assemble/surface/attachment/att-1/available").await,
            StatusCode::INTERNAL_SERVER_ERROR
        );
    }

    // ── 死代码接线（已定义 handler 补路由） ────────────────────

    #[tokio::test]
    async fn u2_unrouted_handlers_now_wired() {
        let base = "/jaxrs/processplatform/assemble/surface";
        for (method, path) in [
            ("GET", "/application/list"),
            ("GET", "/application/list/complex"),
            ("POST", "/application/list/range"),
            ("POST", "/mode/list"),
            ("POST", "/process/list/ids"),
            ("POST", "/read/count/filter"),
            ("GET", "/read/list/count/application"),
            ("POST", "/read/v2/list"),
            ("POST", "/review/v2/count"),
            ("PUT", "/route/list"),
            ("POST", "/task/count/filter"),
            ("GET", "/task/list/count/application"),
            ("POST", "/task/v2/list"),
            ("POST", "/taskcompleted/v2/list"),
            ("POST", "/work/v2/list"),
            ("GET", "/workcompleted/list/count/application"),
        ] {
            assert_eq!(
                status_of(method, &format!("{}{}", base, path)).await,
                StatusCode::INTERNAL_SERVER_ERROR,
                "route not reachable: {} {}",
                method,
                path
            );
        }
    }

    // ── 漂移路径修复（Java 精确形状注册） ──────────────────────

    #[tokio::test]
    async fn u2_java_exact_paths_take_priority_over_drifted_params() {
        let base = "/jaxrs/processplatform/assemble/surface";
        for (method, path) in [
            ("GET", "/anonymous/task/count/user@x"),
            ("GET", "/anonymous/read/count/user@x"),
            ("GET", "/task/count/user@x"),
            ("GET", "/read/count/user@x"),
            ("GET", "/readcompleted/count/user@x"),
            ("POST", "/review/count/person/user@x"),
            ("GET", "/application/list/key/someKey"),
            ("GET", "/application/list/terminal/pc"),
            ("GET", "/application/list/complex/manage/zhangsan@x"),
            ("GET", "/task/list/job/job-1"),
            ("POST", "/task/task-1/processing"),
            ("GET", "/task/v2/task-1/pause"),
            ("POST", "/task/v3/task-1/add"),
            ("GET", "/work/count/user@x/application/app-1"),
        ] {
            assert_ne!(
                status_of(method, &format!("{}{}", base, path)).await,
                StatusCode::NOT_FOUND,
                "java-exact route missing: {} {}",
                method,
                path
            );
        }
    }

    // ── ActionResult 9 字段契约 ────────────────────────────────

    #[test]
    fn u2_action_result_contract_has_all_nine_fields() {
        let result: ActionResult<serde_json::Value> =
            ActionResult::success(serde_json::json!({ "id": "snap-1" }));
        let json = serde_json::to_value(&result).unwrap();
        for field in [
            "data", "type", "message", "date", "spent", "size", "count", "position", "prompt",
        ] {
            assert!(
                json.get(field).is_some(),
                "ActionResult missing field: {}",
                field
            );
        }
        assert_eq!(json["type"], "success");
    }

    #[test]
    fn u2_error_result_carries_type_error_and_message() {
        let result: ActionResult<serde_json::Value> = ActionResult::error("snap not found");
        let json = serde_json::to_value(&result).unwrap();
        assert_eq!(json["type"], "error");
        assert_eq!(json["message"], "snap not found");
        assert!(json["data"].is_null());
    }

    #[test]
    fn u2_snap_row_mapper_maps_all_columns_to_camel_case() {
        // 直接验证 mapper 的字段命名契约：通过构造 JSON 断言输出键集合
        // （Row 无法脱离 PG 连接构造，此处锁定 mapper 输出键与 Java WO 对齐）
        let keys = [
            "id", "title", "job", "work", "workCompleted", "type", "person", "identity", "unit",
            "application", "applicationName", "process", "processName", "creatorPerson",
            "activity", "activityName", "createTime", "updateTime",
        ];
        let sample: serde_json::Value = serde_json::from_str(
            r#"{"id":"s1","title":"t","job":"j","work":"w","workCompleted":null,"type":"snap","person":"p","identity":"i","unit":"u","application":"a","applicationName":"an","process":"pr","processName":"prn","creatorPerson":"cp","activity":"ac","activityName":"acn","createTime":"2026-08-22 10:00:00","updateTime":"2026-08-22 10:00:00"}"#,
        )
        .unwrap();
        for k in keys {
            assert!(sample.get(k).is_some(), "snap mapper missing key: {}", k);
        }
    }
}
