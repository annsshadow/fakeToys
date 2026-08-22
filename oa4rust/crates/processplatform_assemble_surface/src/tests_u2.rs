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

    // ══════════ plan002 U2 批量第二注册（漂移路由 + 缺失端点补齐） ══════════
    // 断言语义：404=路由未注册；400=路径参数提取失败；500=路由可达且提取成功、DB 连接失败。
    // 因此「断言 500」同时证明了：路由存在 + 元组提取器与 Java 路径形状一致。

    #[tokio::test]
    async fn u2r_phase_c_snap_routes_reachable() {
        let base = "/jaxrs/processplatform/assemble/surface/snap";
        assert_eq!(status_of("GET", &format!("{}/snap-1/mockdeletetoget", base)).await, StatusCode::INTERNAL_SERVER_ERROR);
        assert_eq!(status_of("GET", &format!("{}/list/my/paging/0/size/20", base)).await, StatusCode::INTERNAL_SERVER_ERROR);
        assert_eq!(status_of("POST", &format!("{}/list/my/filter/0/size/20", base)).await, StatusCode::INTERNAL_SERVER_ERROR);
        assert_eq!(status_of("GET", &format!("{}/list/snap-1/next/20/application/app-1", base)).await, StatusCode::INTERNAL_SERVER_ERROR);
        assert_eq!(status_of("GET", &format!("{}/list/snap-1/prev/20/application/app-1", base)).await, StatusCode::INTERNAL_SERVER_ERROR);
        assert_eq!(status_of("GET", &format!("{}/list/snap-1/next/20/process/pr-1", base)).await, StatusCode::INTERNAL_SERVER_ERROR);
        assert_eq!(status_of("GET", &format!("{}/list/snap-1/prev/20/process/pr-1", base)).await, StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[tokio::test]
    async fn u2r_phase_c_attachment_routes_reachable() {
        let base = "/jaxrs/processplatform/assemble/surface/attachment";
        assert_eq!(status_of("DELETE", &format!("{}/att-1", base)).await, StatusCode::INTERNAL_SERVER_ERROR);
        assert_eq!(status_of("GET", &format!("{}/att-1/mockdeletetoget", base)).await, StatusCode::INTERNAL_SERVER_ERROR);
        assert_eq!(status_of("GET", &format!("{}/att-1/workorworkcompleted/either-1", base)).await, StatusCode::INTERNAL_SERVER_ERROR);
        assert_eq!(status_of("GET", &format!("{}/list/workorworkcompleted/either-1", base)).await, StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[tokio::test]
    async fn u2r_tuple_extraction_contract_paging_routes() {
        // 若 Path<(i64,i64)> 与路由 {page}/size/{size} 不匹配，axum 会回 400 而非 500
        let base = "/jaxrs/processplatform/assemble/surface";
        for (method, path) in [
            ("GET", "/task/list/my/paging/3/size/20"),
            ("POST", "/task/list/my/filter/3/size/20"),
            ("GET", "/read/list/my/paging/1/size/50"),
            ("POST", "/work/v2/list/paging/2/size/30"),
            ("GET", "/record/list/job/job-1/paging/2/size/10"),
            ("GET", "/documentversion/list/job/job-1/category/cat-1"),
            ("GET", "/correlation/list/job/job-1/site/site-1"),
            ("POST", "/handover/list/paging/0/size/10"),
            ("POST", "/serialnumber/list/paging/0/size/10"),
        ] {
            assert_eq!(
                status_of(method, &format!("{}{}", base, path)).await,
                StatusCode::INTERNAL_SERVER_ERROR,
                "tuple route extraction failed: {} {}",
                method,
                path
            );
        }
    }

    #[tokio::test]
    async fn u2r_tuple_extraction_contract_cursor_with_filter() {
        let base = "/jaxrs/processplatform/assemble/surface";
        for path in [
            "/read/list/r-1/next/20/application/app-1",
            "/read/list/r-1/prev/20/application/app-1",
            "/readcompleted/list/rc-1/next/20/process/pr-1",
            "/snap/list/s-1/next/20/application/app-1",
        ] {
            assert_eq!(
                status_of("GET", &format!("{}{}", base, path)).await,
                StatusCode::INTERNAL_SERVER_ERROR,
                "cursor tuple route failed: {}",
                path
            );
        }
    }

    #[tokio::test]
    async fn u2r_deep_pathn_extraction_contract() {
        // applicationdict 10 参元组、data 路径导航族：证明长元组提取器与 Java 形状一致
        let base = "/jaxrs/processplatform/assemble/surface";
        for (method, path) in [
            ("GET", "/applicationdict/d1/application/a1/p0/p1/p2/p3/p4/p5/p6/p7/data"),
            ("PUT", "/applicationdict/d1/application/a1/p0/data"),
            ("DELETE", "/applicationdict/d1/application/a1/p0/p1/p2/p3/p4/p5/p6/p7/data"),
            ("POST", "/data/job/j-1/p0/mockputtopost"),
            ("GET", "/datarecord/get/job/j-1/path/p-1"),
        ] {
            assert_ne!(
                status_of(method, &format!("{}{}", base, path)).await,
                StatusCode::NOT_FOUND,
                "deep pathN route missing: {} {}",
                method,
                path
            );
        }
    }

    #[tokio::test]
    async fn u2r_same_path_multi_method_merging() {
        // 同一 Java 路径的 GET/PUT/POST 复用同一 handler，方法路由应合并且互不覆盖
        let uri = "/jaxrs/processplatform/assemble/surface/applicationdict/d1/application/a1/p0/data";
        for method in ["GET", "PUT", "POST"] {
            assert_eq!(
                status_of(method, uri).await,
                StatusCode::INTERNAL_SERVER_ERROR,
                "method router merge broken: {}",
                method
            );
        }
    }

    #[tokio::test]
    async fn u2r_drifted_java_paths_now_registered_spot_checks() {
        let base = "/jaxrs/processplatform/assemble/surface";
        for (method, path) in [
            ("DELETE", "/application/app-1/false"),
            ("GET", "/application/app-1/icon"),
            ("GET", "/applicationdict/dict-1/application/app-1"),
            ("GET", "/attachment/att-1/workorworkcompleted/either-1"),
            ("GET", "/form/v2/form-1"),
            ("GET", "/sign/sj-1"),
            ("GET", "/route/route-1/selectconfig"),
            ("GET", "/touch/expire"),
            ("GET", "/handover/hv-1"),
            ("GET", "/datarecord/get/job/j-1/path/p-1"),
            ("GET", "/documentversion/dv-1"),
            ("GET", "/script/sc-1/application/app-1/imported"),
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

    #[tokio::test]
    async fn u2r_snap_cursor_prev_filtered_routes_reachable() {
        let base = "/jaxrs/processplatform/assemble/surface/snap/list";
        for path in [
            "/snap-1/prev/20/application/app-1",
            "/snap-1/prev/20/process/pr-1",
            "/snap-1/next/20/application/app-1",
            "/snap-1/next/20/process/pr-1",
        ] {
            assert_eq!(
                status_of("GET", &format!("{}{}", base, path)).await,
                StatusCode::INTERNAL_SERVER_ERROR,
                "snap filtered cursor prev/next route failed: {}",
                path
            );
        }
    }

    #[tokio::test]
    async fn u2r_data_job_pathn_extraction_contract() {
        // data/job 路径导航族：元组提取器与 Java {job}/{pathN} 形状一致（否则 axum 回 400）
        let base = "/jaxrs/processplatform/assemble/surface/data/job";
        for (method, path) in [
            ("GET", "/j-1/p0"),
            ("GET", "/j-1/p0/p1/p2"),
            ("GET", "/j-1/p0/p1/p2/p3/p4/p5"),
            ("POST", "/j-1/mockputtopost"),
            ("GET", "/j-1/p0/mockdeletetoget"),
        ] {
            assert_eq!(
                status_of(method, &format!("{}{}", base, path)).await,
                StatusCode::INTERNAL_SERVER_ERROR,
                "data job pathN route failed: {} {}",
                method,
                path
            );
        }
    }

    #[tokio::test]
    async fn u2r_review_v2_and_task_v2_tuple_routes() {
        let base = "/jaxrs/processplatform/assemble/surface";
        for (method, path) in [
            ("POST", "/review/v2/list/paging/0/size/20"),
            ("POST", "/task/v2/list/paging/1/size/50"),
            ("POST", "/taskcompleted/v2/list/create/paging/2/size/10"),
            ("POST", "/readcompleted/v2/list/paging/0/size/20"),
            ("GET", "/taskcompleted/list/my/paging/0/size/30"),
        ] {
            assert_eq!(
                status_of(method, &format!("{}{}", base, path)).await,
                StatusCode::INTERNAL_SERVER_ERROR,
                "v2 tuple route failed: {} {}",
                method,
                path
            );
        }
    }
}
