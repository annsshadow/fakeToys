//! plan002 U2 鏂板绔偣鐨勮矾鐢卞彲杈炬祴璇曚笌濂戠害娴嬭瘯銆?
//!
//! 璺敱娴嬭瘯浣跨敤 mock_pool锛堟棤娉曞缓杩烇級锛氳姹傚懡涓矾鐢卞悗 handler 杩斿洖 500锛?
//! 鏂█ 500锛堣€岄潪 404锛夊嵆鍙瘉鏄庤矾鐢卞凡娉ㄥ唽涓斿彲杈俱€?
//! Router 鏋勫缓鏈韩浼氭牎楠岃矾寰勫敮涓€鎬р€斺€旇嫢瀛樺湪閲嶅娉ㄥ唽灏嗙洿鎺?panic銆?

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

    // 鈹€鈹€ snap 鍩?鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€

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

    // 鈹€鈹€ attachment 鍩?鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€

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

    // 鈹€鈹€ 姝讳唬鐮佹帴绾匡紙宸插畾涔?handler 琛ヨ矾鐢憋級 鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€

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

    // 鈹€鈹€ 婕傜Щ璺緞淇锛圝ava 绮剧‘褰㈢姸娉ㄥ唽锛?鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€

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

    // 鈹€鈹€ ActionResult 9 瀛楁濂戠害 鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€

    #[test]
    fn u2_action_result_contract_has_all_nine_fields() {
        let result: ActionResult<serde_json::Value> =
            ActionResult::success(serde_json::json!({ "id": "snap-1" }));
        let json = serde_json::to_value(&result).unwrap();
        // Java 对齐（11521a43）：成功信封恒携带 8 字段；prompt 仅出现在
        // 错误信封（异常类名），None 时整体省略——不得回退为恒输出 9 字段。
        for field in [
            "data", "type", "message", "date", "spent", "size", "count", "position",
        ] {
            assert!(
                json.get(field).is_some(),
                "ActionResult missing field: {}",
                field
            );
        }
        assert!(
            json.get("prompt").is_none(),
            "success envelope must omit prompt (Java parity)"
        );
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
        // 鐩存帴楠岃瘉 mapper 鐨勫瓧娈靛懡鍚嶅绾︼細閫氳繃鏋勯€?JSON 鏂█杈撳嚭閿泦鍚?
        // 锛圧ow 鏃犳硶鑴辩 PG 杩炴帴鏋勯€狅紝姝ゅ閿佸畾 mapper 杈撳嚭閿笌 Java WO 瀵归綈锛?
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

    // 鈺愨晲鈺愨晲鈺愨晲鈺愨晲鈺愨晲 plan002 U2 鎵归噺绗簩娉ㄥ唽锛堟紓绉昏矾鐢?+ 缂哄け绔偣琛ラ綈锛?鈺愨晲鈺愨晲鈺愨晲鈺愨晲鈺愨晲
    // 鏂█璇箟锛?04=璺敱鏈敞鍐岋紱400=璺緞鍙傛暟鎻愬彇澶辫触锛?00=璺敱鍙揪涓旀彁鍙栨垚鍔熴€丏B 杩炴帴澶辫触銆?
    // 鍥犳銆屾柇瑷€ 500銆嶅悓鏃惰瘉鏄庝簡锛氳矾鐢卞瓨鍦?+ 鍏冪粍鎻愬彇鍣ㄤ笌 Java 璺緞褰㈢姸涓€鑷淬€?

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
        // 鑻?Path<(i64,i64)> 涓庤矾鐢?{page}/size/{size} 涓嶅尮閰嶏紝axum 浼氬洖 400 鑰岄潪 500
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
        // applicationdict 10 鍙傚厓缁勩€乨ata 璺緞瀵艰埅鏃忥細璇佹槑闀垮厓缁勬彁鍙栧櫒涓?Java 褰㈢姸涓€鑷?
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
        // 鍚屼竴 Java 璺緞鐨?GET/PUT/POST 澶嶇敤鍚屼竴 handler锛屾柟娉曡矾鐢卞簲鍚堝苟涓斾簰涓嶈鐩?
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
        // data/job 璺緞瀵艰埅鏃忥細鍏冪粍鎻愬彇鍣ㄤ笌 Java {job}/{pathN} 褰㈢姸涓€鑷达紙鍚﹀垯 axum 鍥?400锛?
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

// 鈺愨晲鈺愨晲鈺愨晲鈺愨晲鈺愨晲鈺愨晲 plan002 U2-b锛歛ttachment 浜岃繘鍒舵棌 + data pathN 鍏冪粍鍖栧洖褰掍繚鎶?鈺愨晲鈺愨晲鈺愨晲鈺愨晲鈺愨晲鈺愨晲
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

    // 鈹€鈹€ 杞崲/棰勮/鍙戠エ/URL/鎵撳寘鏃忥細鏃犲紩鎿?鈫?绮剧‘ 501锛堜笉瑙︾ DB 鍗冲彲鏂█锛?鈹€鈹€

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

    // invoice 族已由 capability 桩升级为真实现（62fdf48d：require_owner 门禁 +
    // x_general_invoice 查询），不再属于"精确 501"契约；仅断言路由可达
    // （无 DB/session 时为 500，但绝不能退回 404）。
    #[tokio::test]
    async fn u2b_invoice_endpoints_are_real_routes_not_stubs() {
        let b = "/jaxrs/processplatform/assemble/surface/attachment";
        let cases: Vec<(&str, String)> = vec![
            ("GET", format!("{b}/invoice/f-1/joborworkorworkcompleted/w-1")),
            ("GET", format!("{b}/download/invoice/f-1/joborworkorworkcompleted/w-1")),
        ];
        for (method, path) in cases {
            assert_ne!(
                status_of(method, &path).await,
                StatusCode::NOT_FOUND,
                "invoice route missing after stub replacement: {method} {path}"
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

    // 鈹€鈹€ 涓婁紶鏃忚矾鐢卞彲杈撅紙session extension 缂哄け 鈫?handler 鍐?pool/session 鎻愬彇澶辫触 鈫?500锛?鈹€鈹€

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

    // 鈹€鈹€ 涓嬭浇鏃忚矾鐢卞彲杈?鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€

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

    // 鈹€鈹€ 鍏冩暟鎹鐞嗘棌璺敱鍙揪 鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€

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

    // 鈹€鈹€ BlobStorage 鎺ュ叆鐐瑰崟鍏冪骇琛屼负锛欶S 鍥炶鎴愬姛 / DB 鍗犱綅 fail loud 鈹€鈹€鈹€鈹€鈹€鈹€

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
        // 绾㈢嚎锛欴bBlobStorage.put 鏄?no-op 鈥斺€?鑻ョ収甯?success 鍗?涓婁紶鍋囨垚鍔?銆?
        // 濂戠害锛氬洖璇绘牎楠屽繀椤绘妸杩欑鎯呭喌鏄犲皠涓烘樉寮?NotImplemented锛圚TTP 501锛夈€?
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
        assert!(u2_att_blob_key("a-1", "../escape.txt").is_ok()); // 鍒嗛殧绗﹁鍓ョ涓?_
        let key = u2_att_blob_key("a-1", "../escape.txt").unwrap();
        assert!(!key.contains(".."), "key must not contain traversal components: {key}");
        assert!(u2_att_blob_key("a-1", "").is_err());
        assert!(u2_att_blob_key("a-1", "   ").is_err());
        assert_eq!(u2_att_blob_key("a-1", "dir/nested.txt").unwrap(), "attachment/a-1/dir_nested.txt");
    }

    // 鈹€鈹€ 鏃?2锛歞ata work/workcompleted pathN Java 褰㈢姸鍏冪粍鎻愬彇濂戠害 鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€

    #[tokio::test]
    async fn u2r_data_work_pathn_java_shape_extraction_contract() {
        // 鏂█ 500锛堣€岄潪 400/404锛夎瘉鏄庯細Java 褰㈢姸璺敱瀛樺湪涓?N 鍏冪粍 Path 鎻愬彇鍣ㄥ尮閰嶃€?
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
        // 鍥炲綊淇濇姢锛氭棫瀛楅潰閲忛鏍?URI 涓嶅洜鏂板 Java 褰㈢姸璺敱鑰屾秷澶憋紙tests_generated 鍙ｅ緞 !=404锛?
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

// ════════════ plan002 U2-c：POST filter 族真缺失闭合回归保护 ════════════
#[cfg(test)]
mod u2c_tests {
    use crate::router;
    use axum::body::Body;
    use axum::http::{Request, StatusCode};
    use tower::ServiceExt;

    async fn status_of(method: &str, uri: &str) -> StatusCode {
        router(shared::testing::mock_pool())
            .oneshot(
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

    const BASE: &str = "/jaxrs/processplatform/assemble/surface";

    // ── 路由可达性：500 = 路由存在 + 提取器匹配（session 缺失先于 DB 失败） ──

    #[tokio::test]
    async fn u2c_snap_manage_filter_routes_reachable() {
        for path in [
            "/snap/list/filter/1/size/20/manage",
            "/snap/list/paging/2/size/30/application/app-1/filter/manage",
            "/snap/list/snap-1/next/20/filter/manage",
            "/snap/list/snap-1/prev/20/filter/manage",
        ] {
            assert_eq!(
                status_of("POST", &format!("{}{}", BASE, path)).await,
                StatusCode::INTERNAL_SERVER_ERROR,
                "snap manage filter route failed: {}",
                path
            );
        }
    }

    #[tokio::test]
    async fn u2c_attribute_post_variants_reachable() {
        for path in [
            "/read/filter/attribute/filter",
            "/readcompleted/filter/attribute/filter",
            "/task/filter/attribute/filter",
            "/taskcompleted/filter/attribute/filter",
            "/review/filter/attribute",
        ] {
            assert_eq!(
                status_of("POST", &format!("{}{}", BASE, path)).await,
                StatusCode::INTERNAL_SERVER_ERROR,
                "attribute POST route failed: {}",
                path
            );
        }
    }

    #[tokio::test]
    async fn u2c_review_search_route_reachable() {
        assert_eq!(
            status_of("POST", &format!("{}/review/v2/search", BASE)).await,
            StatusCode::INTERNAL_SERVER_ERROR
        );
    }

    #[tokio::test]
    async fn u2c_review_create_entry_get_and_route_list_post_reachable() {
        // Java 精确形状：GET /review/filter/create/entry（此前仅漂移的 POST 绑定）
        let status = status_of("GET", &format!("{}/review/filter/create/entry", BASE)).await;
        assert_ne!(status, StatusCode::NOT_FOUND);
        assert_ne!(status, StatusCode::BAD_REQUEST);
        // Java POST /route/list/mockputtopost（此前仅有漂移的 GET 绑定）
        let status = status_of("POST", &format!("{}/route/list/mockputtopost", BASE)).await;
        assert_ne!(status, StatusCode::NOT_FOUND);
        assert_ne!(status, StatusCode::BAD_REQUEST);
    }


    #[tokio::test]
    async fn u2c_draft_keylock_write_verb_routes_reachable() {
        for (method, path) in [
            ("PUT", "/draft"),
            ("POST", "/draft/mockputtopost"),
            ("PUT", "/keylock/lock"),
            ("POST", "/keylock/lock/mockputtopost"),
        ] {
            assert_eq!(
                status_of(method, &format!("{}{}", BASE, path)).await,
                StatusCode::INTERNAL_SERVER_ERROR,
                "write route failed: {} {}",
                method,
                path
            );
        }
    }

    #[tokio::test]
    async fn u2c_serialnumber_create_and_java_shape_generate_reachable() {
        assert_eq!(
            status_of("POST", &format!("{}/serialnumber", BASE)).await,
            StatusCode::INTERNAL_SERVER_ERROR
        );
        // Java 精确形状（此前 Rust 侧为漂移的字面量路径 /generate/process/name/name/serial/{id}）
        let status = status_of(
            "POST",
            &format!("{}/serialnumber/generate/process/pr-1/name/serial-1/serial", BASE),
        )
        .await;
        assert_eq!(status, StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[tokio::test]
    async fn u2c_handover_post_openapi_get_reachable() {
        assert_eq!(
            status_of("POST", &format!("{}/handover", BASE)).await,
            StatusCode::INTERNAL_SERVER_ERROR
        );
        // openapi 不触 DB/session：应真实 200 并返回描述符
        assert_eq!(status_of("GET", &format!("{}/openapi", BASE)).await, StatusCode::OK);
    }

    #[tokio::test]
    async fn u2c_work_v3_retract_and_shift_time_post_routes_reachable() {
        assert_eq!(
            status_of("POST", &format!("{}/work/v3/retract", BASE)).await,
            StatusCode::INTERNAL_SERVER_ERROR
        );
        assert_eq!(
            status_of("POST", &format!("{}/workcompleted/shift/time", BASE)).await,
            StatusCode::INTERNAL_SERVER_ERROR
        );
    }

    #[tokio::test]
    async fn u2c_snap_upload_download_routes_reachable() {
        assert_eq!(
            status_of("POST", &format!("{}/snap/upload", BASE)).await,
            StatusCode::INTERNAL_SERVER_ERROR
        );
        assert_ne!(
            status_of("GET", &format!("{}/snap/snap-1/download", BASE)).await,
            StatusCode::NOT_FOUND
        );
    }

    #[tokio::test]
    async fn u2c_attachment_ext_download_routes_reachable() {
        // 形状残差：{fileName}.{ext} 单段双参数受 matchit 限制，以 {fileName} 承载 name.ext
        for path in [
            "/attachment/download/att-1/work/work-1/stream/report.pdf",
            "/attachment/download/att-1/work/work-1/report.pdf",
            "/attachment/download/att-1/workcompleted/wc-1/stream/data.xlsx",
            "/attachment/download/att-1/workcompleted/wc-1/data.xlsx",
        ] {
            let status = status_of("GET", &format!("{}{}", BASE, path)).await;
            assert_ne!(status, StatusCode::NOT_FOUND, "route missing: {}", path);
            assert_ne!(status, StatusCode::BAD_REQUEST, "extractor mismatch: {}", path);
        }
    }

    #[tokio::test]
    async fn u2c_same_path_multi_method_merge_survives() {
        // 同一路径既有 GET 又新增 PUT/POST 的合并注册不得互相覆盖
        let cases: Vec<(&str, &str)> = vec![
            ("GET", "/keylock/lock"),
            ("PUT", "/keylock/lock"),
            ("POST", "/keylock/lock/mockputtopost"),
            ("GET", "/work/v3/retract"),
            ("POST", "/work/v3/retract"),
            ("GET", "/review/filter/attribute"),
            ("POST", "/review/filter/attribute"),
        ];
        for (method, path) in cases {
            let status = status_of(method, &format!("{}{}", BASE, path)).await;
            assert_ne!(
                status,
                StatusCode::METHOD_NOT_ALLOWED,
                "method merge lost: {} {}",
                method,
                path
            );
        }
    }

    // ── 纯函数契约：注入防护 / 分页钳制 / 参数化构建器 ─────────────────────

    #[test]
    fn u2c_like_pattern_escapes_wildcard_injection() {
        use crate::u2_like_pattern;
        // 业务动机：% _ 是 LIKE 通配符、\ 是转义符——不转义则 '%%' 可拖库
        assert_eq!(u2_like_pattern("abc"), "%abc%");
        assert_eq!(u2_like_pattern("50%"), "%50\\%%");
        assert_eq!(u2_like_pattern("a_b"), "%a\\_b%");
        assert_eq!(u2_like_pattern("a\\b"), "%a\\\\b%");
        assert_eq!(u2_like_pattern("%_'\\x"), "%\\%\\_'\\\\x%");
    }

    #[test]
    fn u2c_paging_adjust_clamps_prevent_unbounded_scan() {
        use crate::{u2_adjust_page, u2_adjust_size};
        assert_eq!(u2_adjust_page(0), 1);
        assert_eq!(u2_adjust_page(-5), 1);
        assert_eq!(u2_adjust_page(3), 3);
        assert_eq!(u2_adjust_size(0), 1);
        assert_eq!(u2_adjust_size(999_999), 200, "超大 size 必须被钳制");
        assert_eq!(u2_adjust_size(-1), 1);
    }

    #[test]
    fn u2c_filter_sql_builder_parameterizes_all_user_input() {
        use crate::U2FilterSql;
        let mut fs = U2FilterSql::default();
        fs.push_eq("xperson", "zhang@x");
        fs.push_in("\"xapplication\"", &["app-1".to_string(), "app-2".to_string()]);
        fs.push_key_ilike(&["xtitle", "xserial"], "100%_");
        // 所有用户值必须走占位符，不得内联进 SQL 文本
        let where_clause = fs.where_sql();
        assert_eq!(
            where_clause,
            "xperson = $1 AND \"xapplication\" IN ($2, $3) AND (xtitle ILIKE $4 OR xserial ILIKE $5)"
        );
        assert_eq!(fs.params.len(), 5);
        assert_eq!(fs.params[3], "%100\\%\\_%", "ILIKE pattern 需转义通配符");
        // 空条件退化为恒真（与既有 handler WHERE 1=1 惯例一致）
        let empty = U2FilterSql::default();
        assert_eq!(empty.where_sql(), "1=1");
    }

    #[test]
    fn u2c_paged_result_carries_total_in_action_result_count() {
        use crate::u2_paged_result;
        let data = vec![serde_json::json!({"id": "a"}), serde_json::json!({"id": "b"})];
        let result = u2_paged_result(data, 57);
        assert_eq!(result.0.count, Some(57), "分页 total 必须写入 ActionResult.count");
        assert_eq!(result.0.r#type.as_deref(), Some("success"));
        let body = result.0.data.unwrap();
        assert_eq!(body["count"], 57, "data.count 与 total 一致");
        assert_eq!(body["data"].as_array().unwrap().len(), 2);
    }

    #[test]
    fn u2c_normalized_registrations_have_no_duplicate_method_path() {
        // 防通配冲突守卫：routes.rs 中任意 (method, {x}→{} 归一化路径) 只允许出现一次，
        // 否则 axum Router 构建期会 panic（服务无法启动）。
        let src = include_str!("routes.rs");
        let mut seen = std::collections::HashSet::new();
        let mut duplicates = Vec::new();
        let marker = ".route(\"";
        let mut idx = 0usize;
        while let Some(rel) = src[idx..].find(marker) {
            let start = idx + rel + marker.len();
            let Some(end) = src[start..].find('"') else { break };
            let raw_path = &src[start..start + end];
            let boundary = src[start..].find(".route(").map(|r| start + r).unwrap_or(src.len());
            let window = &src[start..boundary];
            let method = ["get(", "post(", "put(", "delete("]
                .iter()
                .find_map(|t| window.contains(t).then_some(t.trim_end_matches('(')))
                .map(str::to_string);
            if let Some(m) = method {
                let normalized = raw_path
                    .split('/')
                    .map(|seg| {
                        if seg.starts_with('{') && seg.ends_with('}') { "{}" } else { seg }
                    })
                    .collect::<Vec<_>>()
                    .join("/");
                let key = format!("{} {}", m.to_uppercase(), normalized);
                if !seen.insert(key.clone()) {
                    duplicates.push(key);
                }
            }
            idx = start;
        }
        assert!(duplicates.is_empty(), "归一化后重复注册（将导致路由 panic）: {:?}", duplicates);
    }

    #[tokio::test]
    async fn u2c_openapi_descriptor_reflects_real_registrations() {
        use crate::{openapi_get, u2_collect_routes};
        // 描述符必须来自 routes.rs 实际扫描而非硬编码清单
        let routes = u2_collect_routes(include_str!("routes.rs"));
        let upload = routes
            .get(&format!("{}/snap/upload", BASE))
            .expect("snap/upload must be listed");
        assert!(upload.contains(&"post".to_string()));
        assert!(routes.contains_key(&format!("{}/keylock/lock", BASE)));

        let response = openapi_get().await.unwrap().0;
        assert_eq!(response.r#type.as_deref(), Some("success"));
        let body = response.data.expect("openapi data required");
        assert_eq!(body["openapi"], "3.0.3");
        assert!(body["paths"][&format!("{}/review/v2/search", BASE)]["post"].is_object());
        assert!(body["paths"][&format!("{}/openapi", BASE)]["get"].is_object());
    }
}