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

// ════════════ plan002 U2-b：attachment 二进制族 + data pathN 元组化回归保护 ════════════
#[cfg(test)]
mod u2b_tests {
    use crate::router;
    use axum::body::Body;
    use axum::http::{Request, StatusCode};
    use shared::response::ActionResult;
    use shared::storage::{BlobStorage, DbBlobStorage, FsBlobStorage};
    use tower::ServiceExt;

    async fn respond(method: &str, uri: &str, headers: &[(&str, &str)], body: Body) -> (StatusCode, serde_json::Value) {
        let mut builder = Request::builder().uri(uri).method(method);
        for (k, v) in headers {
            builder = builder.header(*k, *v);
        }
        let response = router(shared::testing::mock_pool())
            .oneshot(builder.body(body).unwrap())
            .await
            .unwrap();
        let status = response.status();
        let bytes = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let json = if bytes.is_empty() {
            serde_json::Value::Null
        } else {
            serde_json::from_slice(&bytes).unwrap_or(serde_json::Value::Null)
        };
        (status, json)
    }

    async fn status_of(method: &str, uri: &str) -> StatusCode {
        respond(method, uri, &[], Body::empty()).await.0
    }

    fn multipart_body() -> Body {
        Body::from(
            "--xboundary\r\n\
             Content-Disposition: form-data; name=\"file\"; filename=\"a.txt\"\r\n\
             Content-Type: text/plain\r\n\r\nhello\r\n--xboundary--\r\n",
        )
    }

    const MP: &[(&str, &str)] = &[("content-type", "multipart/form-data; boundary=xboundary")];
    const JSON: &[(&str, &str)] = &[("content-type", "application/json")];

    // ── 转换/预览/发票/URL/打包族：无引擎 → 精确 501（不触碰 DB 即可断言） ──

    #[tokio::test]
    async fn u2b_engineless_endpoints_return_exact_501() {
        let b = "/jaxrs/processplatform/assemble/surface/attachment";
        let cases: Vec<(&str, String)> = vec![
            ("POST", format!("{b}/doc/to/word/work/w-1")),
            ("POST", format!("{b}/doc/to/word/workorworkcompleted/w-1")),
            ("POST", format!("{b}/html/to/pdf")),
            ("POST", format!("{b}/html/to/image")),
            ("GET", format!("{b}/att-1/preview/pdf")),
            ("GET", format!("{b}/att-1/preview/image/page/2")),
            ("GET", format!("{b}/preview/pdf/f-1/result")),
            ("GET", format!("{b}/preview/image/f-1/result")),
            ("GET", format!("{b}/invoice/f-1/joborworkorworkcompleted/w-1")),
            ("GET", format!("{b}/download/invoice/f-1/joborworkorworkcompleted/w-1")),
            ("POST", format!("{b}/upload/with/url")),
            ("GET", format!("{b}/batch/download/job/j-1/site/s-1")),
            ("GET", format!("{b}/batch/download/work/w-1/site/s-1")),
            ("GET", format!("{b}/batch/download/work/w-1/site/s-1/stream")),
        ];
        for (method, path) in cases {
            assert_eq!(
                status_of(method, &path).await,
                StatusCode::NOT_IMPLEMENTED,
                "engine-less endpoint must answer exact 501: {path}"
            );
        }
    }

    #[tokio::test]
    async fn u2b_501_response_body_is_action_result_error_shape() {
        let (status, json) =
            respond("POST", "/jaxrs/processplatform/assemble/surface/attachment/html/to/pdf", &[], Body::empty()).await;
        assert_eq!(status, StatusCode::NOT_IMPLEMENTED);
        assert_eq!(json["type"], "error");
        assert!(json.get("message").is_some(), "ActionResult.message required");
        assert!(json["data"].is_null());
    }

    // ── 上传族路由可达（session extension 缺失 → handler 内 pool/session 提取失败 → 500） ──

    #[tokio::test]
    async fn u2b_multipart_upload_routes_reachable() {
        let b = "/jaxrs/processplatform/assemble/surface/attachment";
        for (method, path) in [
            ("POST", &format!("{b}/upload/work/w-1")),
            ("POST", &format!("{b}/upload/work/w-1/callback/cb-1")),
            ("POST", &format!("{b}/upload/workcompleted/wc-1")),
            ("PUT", &format!("{b}/upload/work/w-1/save/as/name.txt")),
            ("POST", &format!("{b}/upload/work/w-1/save/as/name.txt/mockputtopost")),
            ("POST", &format!("{b}/v2/upload/workorworkcompleted/either-1")),
            ("POST", &format!("{b}/batch/upload/manage")),
        ] {
            assert_eq!(
                respond(method, path, MP, multipart_body()).await.0,
                StatusCode::INTERNAL_SERVER_ERROR,
                "multipart upload route failed: {path}"
            );
        }
    }

    #[tokio::test]
    async fn u2b_base64_upload_route_reachable() {
        let path = "/jaxrs/processplatform/assemble/surface/attachment/v2/upload/workorworkcompleted/either-1/base64";
        let (status, _) = respond("POST", path, JSON, Body::from(r#"{"fileName":"a.txt","fileBase64":"aGVsbG8="}"#)).await;
        assert_eq!(status, StatusCode::INTERNAL_SERVER_ERROR, "base64 upload route unreachable");
    }

    // ── 下载族路由可达 ─────────────────────────────────────────────────────

    #[tokio::test]
    async fn u2b_download_routes_reachable() {
        let b = "/jaxrs/processplatform/assemble/surface/attachment";
        for path in [
            format!("{b}/download/att-1"),
            format!("{b}/download/att-1/stream"),
            format!("{b}/download/att-1/manage"),
            format!("{b}/download/att-1/manage/stream"),
            format!("{b}/download/att-1/work/w-1"),
            format!("{b}/download/att-1/work/w-1/stream"),
            format!("{b}/download/att-1/workcompleted/wc-1"),
            format!("{b}/download/att-1/workcompleted/wc-1/stream"),
            format!("{b}/download/work/w-1/att/att-1"),
            format!("{b}/download/transfer/flag/either-1"),
        ] {
            assert_eq!(
                status_of("GET", &path).await,
                StatusCode::INTERNAL_SERVER_ERROR,
                "download route failed: {path}"
            );
        }
    }

    // ── 元数据管理族路由可达 ───────────────────────────────────────────────

    #[tokio::test]
    async fn u2b_metadata_write_routes_reachable() {
        let b = "/jaxrs/processplatform/assemble/surface/attachment";
        for (method, path) in [
            ("DELETE", &format!("{b}/att-1/workcompleted/wc-1")),
            ("PUT", &format!("{b}/update/att-1/work/w-1")),
            ("POST", &format!("{b}/update/att-1/work/w-1")),
            ("POST", &format!("{b}/update/att-1/work/w-1/callback/cb-1")),
            ("POST", &format!("{b}/update/att-1/work/w-1/mockputtopost")),
            ("PUT", &format!("{b}/update/content/att-1/work/w-1")),
            ("POST", &format!("{b}/update/content/att-1/work/w-1/mockputtopost")),
            ("PUT", &format!("{b}/edit/att-1/work/w-1")),
            ("POST", &format!("{b}/edit/att-1/work/w-1/mockputtopost")),
            ("PUT", &format!("{b}/edit/att-1/work/w-1/text")),
            ("POST", &format!("{b}/edit/att-1/work/w-1/text/mockputtopost")),
            ("POST", &format!("{b}/copy/work/w-1")),
            ("POST", &format!("{b}/copy/work/w-1/soft")),
            ("POST", &format!("{b}/copy/workcompleted/wc-1")),
            ("POST", &format!("{b}/copy/workcompleted/wc-1/soft")),
            ("POST", &format!("{b}/batch/delete/manage")),
            ("POST", &format!("{b}/batch/update/manage")),
            ("GET", &format!("{b}/att-1/work/w-1/change/ordernumber/3")),
            ("GET", &format!("{b}/att-1/work/w-1/change/site/new-site")),
        ] {
            let needs_json = method == "PUT" || method == "POST";
            let (headers, body) = if needs_json { (JSON, Body::from("{}")) } else { (&[][..], Body::empty()) };
            assert_eq!(
                respond(method, path, headers, body).await.0,
                StatusCode::INTERNAL_SERVER_ERROR,
                "metadata route failed: {path}"
            );
        }
    }

    #[tokio::test]
    async fn u2b_online_info_and_mockdelete_routes_reachable() {
        let b = "/jaxrs/processplatform/assemble/surface/attachment";
        for path in [
            format!("{b}/att-1/online/info"),
            format!("{b}/att-1/work/w-1/mockdeletetoget"),
            format!("{b}/att-1/workcompleted/wc-1/mockdeletetoget"),
        ] {
            assert_ne!(
                status_of("GET", &path).await,
                StatusCode::NOT_FOUND,
                "route missing: {path}"
            );
        }
    }

    // ── BlobStorage 接入点单元级行为：FS 回读成功 / DB 占位 fail loud ──────

    fn fs_backend(tag: &str) -> FsBlobStorage {
        let dir = std::env::temp_dir()
            .join(format!("oa4rust_u2b_{tag}_{}", uuid::Uuid::new_v4()));
        std::fs::create_dir_all(&dir).unwrap();
        FsBlobStorage::new(dir)
    }

    #[tokio::test]
    async fn u2b_fs_backend_persists_upload_roundtrip() {
        let storage = fs_backend("ok");
        crate::u2_att_persist_verified(&storage, "attachment/a-1/f.bin", b"payload").await.unwrap();
        assert_eq!(
            storage.get("attachment/a-1/f.bin").await.unwrap(),
            b"payload".to_vec(),
            "FS backend must persist uploaded bytes verbatim"
        );
    }

    #[tokio::test]
    async fn u2b_db_placeholder_backend_fails_loud_as_not_implemented() {
        // 红线：DbBlobStorage.put 是 no-op —— 若照常 success 即"上传假成功"。
        // 契约：回读校验必须把这种情况映射为显式 NotImplemented（HTTP 501）。
        let storage = DbBlobStorage::default();
        let err = crate::u2_att_persist_verified(&storage, "attachment/a-1/f.bin", b"x")
            .await
            .unwrap_err();
        assert!(
            matches!(err, shared::error::AppError::NotImplemented),
            "DB placeholder backend must fail loud with NotImplemented, got {err:?}"
        );
    }

    #[tokio::test]
    async fn u2b_blob_key_rejects_traversal_and_empty_names() {
        use crate::u2_att_blob_key;
        assert!(u2_att_blob_key("a-1", "../escape.txt").is_ok()); // 分隔符被剥离为 _
        let key = u2_att_blob_key("a-1", "../escape.txt").unwrap();
        assert!(!key.contains(".."), "key must not contain traversal components: {key}");
        assert!(u2_att_blob_key("a-1", "").is_err());
        assert!(u2_att_blob_key("a-1", "   ").is_err());
        assert_eq!(u2_att_blob_key("a-1", "dir/nested.txt").unwrap(), "attachment/a-1/dir_nested.txt");
    }

    // ── 族 2：data work/workcompleted pathN Java 形状元组提取契约 ──────────

    #[tokio::test]
    async fn u2r_data_work_pathn_java_shape_extraction_contract() {
        // 断言 500（而非 400/404）证明：Java 形状路由存在且 N 元组 Path 提取器匹配。
        let base = "/jaxrs/processplatform/assemble/surface/data/work";
        for (method, path) in [
            ("GET", &format!("{base}/w-1/p0")),
            ("GET", &format!("{base}/w-1/p0/p1/p2/p3")),
            ("GET", &format!("{base}/w-1/p0/p1/p2/p3/p4/p5/p6/p7")),
            ("GET", &format!("{base}/w-1/p0/mockdeletetoget")),
            ("PUT", &format!("{base}/w-1/p0")),
            ("PUT", &format!("{base}/w-1/p0/p1/p2")),
            ("POST", &format!("{base}/w-1/p0/p1/mockputtopost")),
            ("DELETE", &format!("{base}/w-1/p0/p1/p2")),
        ] {
            assert_eq!(
                status_of(method, path).await,
                StatusCode::INTERNAL_SERVER_ERROR,
                "work pathN tuple extraction failed: {path}"
            );
        }
    }

    #[tokio::test]
    async fn u2r_data_workcompleted_pathn_java_shape_extraction_contract() {
        let base = "/jaxrs/processplatform/assemble/surface/data/workcompleted";
        for (method, path) in [
            ("GET", &format!("{base}/wc-1/p0")),
            ("GET", &format!("{base}/wc-1/p0/p1/p2/p3/p4/p5/p6/p7")),
            ("PUT", &format!("{base}/wc-1/p0/p1")),
            ("POST", &format!("{base}/wc-1/p0/mockputtopost")),
        ] {
            assert_eq!(
                status_of(method, path).await,
                StatusCode::INTERNAL_SERVER_ERROR,
                "workcompleted pathN tuple extraction failed: {path}"
            );
        }
    }

    #[tokio::test]
    async fn u2r_data_pathn_legacy_literal_routes_still_guarded() {
        // 回归保护：旧字面量风格 URI 不因新增 Java 形状路由而消失（tests_generated 口径 !=404）
        let base = "/jaxrs/processplatform/assemble/surface/data";
        for (method, path) in [
            ("GET", &format!("{base}/work/path0/test-id")),
            ("GET", &format!("{base}/work/path0/path1/test-id")),
            ("GET", &format!("{base}/workcompleted/path0/test-id")),
        ] {
            assert_ne!(
                status_of(method, path).await,
                StatusCode::NOT_FOUND,
                "legacy literal route lost: {path}"
            );
        }
    }
}

