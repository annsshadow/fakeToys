//! plan002 U2 续轮：appdict surface 家族（段序修正）与 data/document 通配族的测试。
//!
//! 三层：
//!   1. 路由可达性：mock_pool 无法建连，请求命中路由后返回 500/415，
//!      断言 ≠404 且 ≠405 即证明路由已按 Java 段序注册且动词正确；
//!      Router 构建本身校验路径唯一性——通配归一化若产生冲突将直接 panic。
//!   2. 门禁单元测试：直接调用写 handler，DB 不可用时门禁 fail-closed
//!      返回 Internal（拒绝放行），证明写端点不存在"未授权直通"。
//!   3. 真库端到端：is_db_available 守卫，验证 data 字段组合键读写删、
//!      mock 动词别名与主动词同语义（含旧 ON CONFLICT 运行期错误的修复）、
//!      派生资源经父文档所有者校验的 IDOR 门禁，以及 surface appdict
//!      根行/子树行的创建-读取-冲突拒绝-删除生命周期。

#[cfg(test)]
mod data_appdict_tests {
    use axum::body::Body;
    use axum::extract::Extension;
    use axum::http::{Request, StatusCode};
    use serde_json::json;
    use shared::error::AppError;
    use shared::session::Session;
    use shared::testing::{is_db_available, mock_pool, test_pool};
    use tower::util::ServiceExt;

    use crate::{
        data_document_id_path0_update,
        surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_data_put,
    };

    const OWNER: &str = "u3-owner";
    const STRANGER: &str = "u3-stranger";
    const DOC: &str = "u3test-doc-data";
    const DOC_MOCK: &str = "u3test-doc-mock";
    const DICT: &str = "u3-dict";
    const APP: &str = "u3-app";

    fn session(person: &str) -> Session {
        let now = chrono::Utc::now().naive_utc();
        Session {
            token: format!("u3-token-{person}"),
            person_unique: person.to_string(),
            created_at: now,
            expires_at: now + chrono::Duration::hours(2),
        }
    }

    // ── 1. 路由可达性 ───────────────────────────────────────────

    async fn status_of(method: &str, uri: &str, body: Option<serde_json::Value>) -> StatusCode {
        let app = crate::router(mock_pool());
        let mut builder = Request::builder().method(method).uri(uri);
        if body.is_some() {
            builder = builder.header("content-type", "application/json");
        }
        let req = match body {
            Some(v) => builder.body(Body::from(v.to_string())).unwrap(),
            None => builder.body(Body::empty()).unwrap(),
        };
        app.oneshot(req).await.unwrap().status()
    }

    fn reachable(status: StatusCode) -> bool {
        status != StatusCode::NOT_FOUND && status != StatusCode::METHOD_NOT_ALLOWED
    }

    #[tokio::test]
    async fn data_wildcard_routes_reachable() {
        // 通配读：深度 0 与深度 3/7 各抽一档；字面 path0 URL 也应被通配捕获
        assert!(reachable(status_of("GET", "/jaxrs/data/document/d-1/anything", None).await));
        assert!(reachable(
            status_of("GET", "/jaxrs/data/document/d-1/a/b/c", None).await
        ));
        assert!(reachable(
            status_of(
                "GET",
                "/jaxrs/data/document/d-1/a/b/c/d/e/f/g/h",
                None
            )
            .await
        ));
        // 字面段名 URL 同样命中通配路由（path0 作为参数值）
        assert!(reachable(status_of("GET", "/jaxrs/data/document/d-1/path0", None).await));
        // 动词别名：mockdeletetoget(GET) / mockputtopost(POST)，静态段优先于参数段
        assert_eq!(
            status_of("GET", "/jaxrs/data/document/d-1/x/mockdeletetoget", None).await,
            StatusCode::INTERNAL_SERVER_ERROR
        );
        assert_ne!(
            status_of("POST", "/jaxrs/data/document/d-1/x/mockputtopost", None).await,
            StatusCode::NOT_FOUND
        );
        assert!(reachable(
            status_of("POST", "/jaxrs/data/document/d-1/array/data", None).await
        ));
    }

    #[tokio::test]
    async fn data_write_methods_aligned() {
        // 基座全动词（Java ActionUpdate/Create/DeleteWithDocument）
        for method in ["PUT", "POST", "DELETE"] {
            let st = status_of(method, "/jaxrs/data/document/d-1", Some(json!({"k": "v"}))).await;
            assert!(reachable(st), "{method} /jaxrs/data/document/d-1 -> {st}");
        }
        // 路径级全动词（深度 0 与深度 2 抽样）
        for uri in [
            "/jaxrs/data/document/d-1/field",
            "/jaxrs/data/document/d-1/a/b/c",
        ] {
            for method in ["PUT", "POST", "DELETE"] {
                let st = status_of(method, uri, Some(json!({"k": "v"}))).await;
                assert!(reachable(st), "{method} {uri} -> {st}");
            }
        }
    }

    #[tokio::test]
    async fn appdict_routes_reachable() {
        // 认证族：Java 段序 {appDictFlag}/appInfo/{appInfoFlag}[/{pathN}/]{data}
        assert_eq!(
            status_of("GET", &format!("/jaxrs/surface/appdict/{DICT}/appInfo/{APP}"), None).await,
            StatusCode::INTERNAL_SERVER_ERROR
        );
        assert_eq!(
            status_of(
                "GET",
                &format!("/jaxrs/surface/appdict/{DICT}/appInfo/{APP}/data"),
                None
            )
            .await,
            StatusCode::INTERNAL_SERVER_ERROR
        );
        for depth_uri in [
            format!("/jaxrs/surface/appdict/{DICT}/appInfo/{APP}/p0/data"),
            format!("/jaxrs/surface/appdict/{DICT}/appInfo/{APP}/p0/p1/p2/p3/data"),
            format!("/jaxrs/surface/appdict/{DICT}/appInfo/{APP}/p0/p1/p2/p3/p4/p5/p6/p7/data"),
        ] {
            assert_eq!(
                status_of("GET", &depth_uri, None).await,
                StatusCode::INTERNAL_SERVER_ERROR,
                "{depth_uri}"
            );
        }
        // 匿名族
        assert_eq!(
            status_of(
                "GET",
                &format!("/jaxrs/anonymous/surface/appdict/{DICT}/appInfo/{APP}/p0/data"),
                None
            )
            .await,
            StatusCode::INTERNAL_SERVER_ERROR
        );
        assert!(reachable(
            status_of(
                "GET",
                &format!("/jaxrs/anonymous/surface/appdict/list/appInfo/{APP}"),
                None
            )
            .await
        ));
    }

    #[tokio::test]
    async fn appdict_write_routes_reachable() {
        let base = format!("/jaxrs/surface/appdict/{DICT}/appInfo/{APP}");
        // 基座 PUT + mockputtopost 别名
        assert!(reachable(status_of("PUT", &base, Some(json!({"dataValue": "v"}))).await));
        assert!(reachable(
            status_of("POST", &format!("{base}/mockputtopost"), Some(json!({"dataValue": "v"}))).await
        ));
        // 路径级 PUT/POST/DELETE + 两类别名（Java 仅深度 ≥2 即 3 段起提供 mockputtopost）
        let leaf = format!("{base}/p0/p1/p2/data");
        for method in ["PUT", "POST"] {
            assert!(reachable(status_of(method, &leaf, Some(json!({"v": 1}))).await), "{method} {leaf}");
        }
        assert!(reachable(status_of("DELETE", &leaf, None).await));
        assert!(reachable(status_of("GET", &format!("{leaf}/mockdeletetoget"), None).await));
        // 深度 1（2 段）不应注册 mockputtopost —— 与 Java 一致无此路由
        assert_eq!(
            status_of(
                "POST",
                &format!("{base}/p0/p1/data/mockputtopost"),
                Some(json!({"v": 1}))
            )
            .await,
            StatusCode::NOT_FOUND,
            "mockputtopost must not exist below depth 2"
        );
        assert!(reachable(
            status_of("POST", &format!("{leaf}/mockputtopost"), Some(json!({"v": 1}))).await
        ));
    }

    // ── 2. 门禁单元测试（mock pool，fail-closed） ──────────────

    #[tokio::test]
    async fn data_field_update_fails_closed_without_db() {
        let r = data_document_id_path0_update(
            Extension(mock_pool()),
            Extension(session(STRANGER)),
            axum::extract::Path([DOC.to_string(), "title".to_string()]),
            axum::extract::Json(json!({"x": 1})),
        )
        .await;
        match r {
            Err(AppError::Internal) => {}
            other => panic!("expected Internal(fail closed), got {:?}", other.map(|_| "ok")),
        }
    }

    #[tokio::test]
    async fn appdict_put_fails_closed_without_db() {
        let r = surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_data_put(
            Extension(mock_pool()),
            Extension(session(STRANGER)),
            axum::extract::Path([DICT.to_string(), APP.to_string(), "p0".to_string()]),
            axum::extract::Json(json!({"dataValue": "v"})),
        )
        .await;
        match r {
            Err(AppError::Internal) => {}
            other => panic!("expected Internal(fail closed), got {:?}", other.map(|_| "ok")),
        }
    }

    // ── 3. 真库端到端 ──────────────────────────────────────────

    async fn call(
        method: &str,
        uri: &str,
        body: Option<serde_json::Value>,
        sess: Option<Session>,
    ) -> (StatusCode, serde_json::Value) {
        let app = crate::router(test_pool());
        let mut builder = Request::builder().method(method).uri(uri);
        if let Some(s) = &sess {
            builder = builder.extension(s.clone());
        }
        let req = match body {
            Some(v) => builder
                .header("content-type", "application/json")
                .body(Body::from(v.to_string()))
                .unwrap(),
            None => builder.body(Body::empty()).unwrap(),
        };
        let resp = app.oneshot(req).await.unwrap();
        let status = resp.status();
        let bytes = axum::body::to_bytes(resp.into_body(), 1_048_576).await.unwrap();
        let json = if bytes.is_empty() {
            serde_json::Value::Null
        } else {
            serde_json::from_slice(&bytes).unwrap_or(serde_json::Value::Null)
        };
        (status, json)
    }

    async fn seed_doc(id: &str) {
        let client = test_pool().get().await.unwrap();
        let _ = client
            .execute(
                "DELETE FROM x_cms_data_document_field WHERE doc_id = $1",
                &[&id],
            )
            .await;
        let _ = client
            .execute("DELETE FROM x_cms_data_document WHERE id = $1", &[&id])
            .await;
        client
            .execute(
                "INSERT INTO x_cms_data_document (id, title, creator) VALUES ($1, 'U3 Data', $2)",
                &[&id, &OWNER.to_string()],
            )
            .await
            .unwrap();
    }

    async fn drop_doc(id: &str) {
        let client = test_pool().get().await.unwrap();
        let _ = client
            .execute(
                "DELETE FROM x_cms_data_document_field WHERE doc_id = $1",
                &[&id],
            )
            .await;
        let _ = client
            .execute("DELETE FROM x_cms_data_document WHERE id = $1", &[&id])
            .await;
    }

    #[tokio::test]
    async fn data_document_field_lifecycle_real_db() {
        if !is_db_available().await {
            eprintln!("skipping data_document_field_lifecycle_real_db: db not reachable");
            return;
        }
        seed_doc(DOC).await;

        // PUT 基座：顶层 key 全量 upsert（owner 会话）
        let (status, json) = call(
            "PUT",
            &format!("/jaxrs/data/document/{DOC}"),
            Some(json!({"title": "hello", "count": 42})),
            Some(session(OWNER)),
        )
        .await;
        assert_eq!(status, StatusCode::OK, "put base: {json}");
        assert_eq!(json["type"], "success");
        assert_eq!(json["data"]["updated"], 2);

        // GET 一级路径：精确字段匹配
        let (status, json) = call("GET", &format!("/jaxrs/data/document/{DOC}/title"), None, None).await;
        assert_eq!(status, StatusCode::OK);
        assert_eq!(json["data"][0]["fieldName"], "title");
        assert_eq!(json["data"][0]["fieldValue"], "hello");

        // POST 多级路径：组合键 title.deep.body 插入
        let (status, json) = call(
            "POST",
            &format!("/jaxrs/data/document/{DOC}/title/deep/body"),
            Some(json!({"v": 1})),
            Some(session(OWNER)),
        )
        .await;
        assert_eq!(status, StatusCode::OK, "create deep: {json}");
        assert_eq!(json["data"]["created"], true);

        let (_, json) = call("GET", &format!("/jaxrs/data/document/{DOC}/title/deep/body"), None, None).await;
        assert_eq!(json["data"][0]["fieldName"], "title.deep.body");

        // 组合键真实落库
        let row = test_pool()
            .get()
            .await
            .unwrap()
            .query_opt(
                "SELECT 1 FROM x_cms_data_document_field \
                 WHERE doc_id = $1 AND field_name = 'title.deep.body' AND deleted_at IS NULL",
                &[&DOC.to_string()],
            )
            .await
            .unwrap();
        assert!(row.is_some(), "composed key should be persisted");

        // IDOR：非所有者不能写文档数据
        for (method, uri) in [
            ("PUT", format!("/jaxrs/data/document/{DOC}/title")),
            ("DELETE", format!("/jaxrs/data/document/{DOC}/title/deep/body")),
        ] {
            let (status, _) = call(&method, &uri, Some(json!({"x": 1})), Some(session(STRANGER))).await;
            assert_eq!(status, StatusCode::FORBIDDEN, "{method} {uri}");
        }

        // 所有者删除深层路径后不可再读（读端点语义：无匹配行返回空数组）
        let (status, json) = call(
            "DELETE",
            &format!("/jaxrs/data/document/{DOC}/title/deep/body"),
            None,
            Some(session(OWNER)),
        )
        .await;
        assert_eq!(status, StatusCode::OK, "delete deep: {json}");
        assert_eq!(json["data"]["deleted"], true);

        let (status, json) = call("GET", &format!("/jaxrs/data/document/{DOC}/title/deep/body"), None, None).await;
        assert_eq!(status, StatusCode::OK);
        assert_eq!(json["type"], "success");
        assert_eq!(json["data"].as_array().unwrap().len(), 0);

        // 删除真实落库：组合键行已软删
        let row = test_pool()
            .get()
            .await
            .unwrap()
            .query_opt(
                "SELECT 1 FROM x_cms_data_document_field \
                 WHERE doc_id = $1 AND field_name = 'title.deep.body' AND deleted_at IS NOT NULL",
                &[&DOC.to_string()],
            )
            .await
            .unwrap();
        assert!(row.is_some(), "soft delete should be persisted");

        drop_doc(DOC).await;
    }

    #[tokio::test]
    async fn data_mock_aliases_roundtrip_real_db() {
        if !is_db_available().await {
            eprintln!("skipping data_mock_aliases_roundtrip_real_db: db not reachable");
            return;
        }
        seed_doc(DOC_MOCK).await;

        // POST 基座 mockputtopost：与 PUT 同语义（顶层 key upsert）。
        // 该用例同时验证旧实现对无唯一约束列使用 ON CONFLICT 的运行期错误已被修复。
        let (status, json) = call(
            "POST",
            &format!("/jaxrs/data/document/{DOC_MOCK}/mockputtopost"),
            Some(json!({"k1": "v1"})),
            Some(session(OWNER)),
        )
        .await;
        assert_eq!(status, StatusCode::OK, "mock put base: {json}");
        assert_eq!(json["data"]["updated"], 1);

        let (_, json) = call("GET", &format!("/jaxrs/data/document/{DOC_MOCK}/k1"), None, None).await;
        assert_eq!(json["data"][0]["fieldValue"], "v1");

        // 路径级 mockputtopost：沿用 fieldValue 契约更新同一字段
        let (status, json) = call(
            "POST",
            &format!("/jaxrs/data/document/{DOC_MOCK}/k1/mockputtopost"),
            Some(json!({"fieldValue": "v2"})),
            Some(session(OWNER)),
        )
        .await;
        assert_eq!(status, StatusCode::OK, "mock put path0: {json}");
        assert_eq!(json["data"]["updated"], true);

        let (_, json) = call("GET", &format!("/jaxrs/data/document/{DOC_MOCK}/k1"), None, None).await;
        assert_eq!(json["data"][0]["fieldValue"], "v2");

        // 基座 mockdeletetoget：删除的是文档数据（字段行），而非文档实体
        let (status, json) = call(
            "GET",
            &format!("/jaxrs/data/document/{DOC_MOCK}/mockdeletetoget"),
            None,
            Some(session(OWNER)),
        )
        .await;
        assert_eq!(status, StatusCode::OK, "mock delete: {json}");
        assert!(json["data"]["deleted"].as_i64().unwrap() >= 1);

        let client = test_pool().get().await.unwrap();
        let doc_alive = client
            .query_opt(
                "SELECT 1 FROM x_cms_data_document WHERE id = $1 AND deleted_at IS NULL",
                &[&DOC_MOCK.to_string()],
            )
            .await
            .unwrap();
        assert!(doc_alive.is_some(), "document entity must survive data deletion");
        let remaining = client
            .query_one(
                "SELECT COUNT(*) AS n FROM x_cms_data_document_field \
                 WHERE doc_id = $1 AND deleted_at IS NULL",
                &[&DOC_MOCK.to_string()],
            )
            .await
            .unwrap();
        let n: i64 = remaining.get("n");
        assert_eq!(n, 0, "all data fields should be soft-deleted");

        drop_doc(DOC_MOCK).await;
    }

    async fn clean_dict() {
        let client = test_pool().get().await.unwrap();
        // 双向清理：防御历史运行可能留下的 flag 互换行
        let _ = client
            .execute(
                "DELETE FROM x_cms_surface_appdict \
                 WHERE app_dict_flag = ANY($1) OR app_info_flag = ANY($1)",
                &[&vec![DICT.to_string(), APP.to_string()]],
            )
            .await;
    }

    #[tokio::test]
    async fn surface_appdict_lifecycle_real_db() {
        if !is_db_available().await {
            eprintln!("skipping surface_appdict_lifecycle_real_db: db not reachable");
            return;
        }
        clean_dict().await;
        let base = format!("/jaxrs/surface/appdict/{DICT}/appInfo/{APP}");

        // 空字典允许创建：owner PUT 根行
        let (status, json) = call(
            "PUT",
            &base,
            Some(json!({"dataValue": "root-v1"})),
            Some(session(OWNER)),
        )
        .await;
        assert_eq!(status, StatusCode::OK, "dict put root: {json}");
        assert_eq!(json["data"]["created"], true);

        // 根行落库且 creator 为会话用户
        let root = test_pool()
            .get()
            .await
            .unwrap()
            .query_one(
                "SELECT creator, data_value FROM x_cms_surface_appdict \
                 WHERE app_dict_flag = $1 AND app_info_flag = $2 \
                   AND cardinality(path_levels) = 0 AND deleted_at IS NULL",
                &[&DICT.to_string(), &APP.to_string()],
            )
            .await
            .unwrap();
        let creator: String = root.get::<_, Option<String>>("creator").unwrap_or_default();
        assert_eq!(creator, OWNER);
        assert_eq!(root.get::<_, String>("data_value"), "root-v1");

        // 再次 PUT：upsert 更新根行而不新增
        let (status, json) = call(
            "PUT",
            &base,
            Some(json!({"dataValue": "root-v2"})),
            Some(session(OWNER)),
        )
        .await;
        assert_eq!(status, StatusCode::OK);
        assert_eq!(json["data"]["updated"], true);
        let count = test_pool()
            .get()
            .await
            .unwrap()
            .query_one(
                "SELECT COUNT(*) AS n FROM x_cms_surface_appdict \
                 WHERE app_dict_flag = $1 AND app_info_flag = $2 AND deleted_at IS NULL",
                &[&DICT.to_string(), &APP.to_string()],
            )
            .await
            .unwrap();
        let n: i64 = count.get("n");
        assert_eq!(n, 1, "upsert must not duplicate root row");

        // 子路径创建 + 前缀读（子树语义：根行不出现在 p0 读中）
        let leaf_url = format!("{base}/p0/data");
        let (status, json) = call(
            "PUT",
            &leaf_url,
            Some(json!({"leaf": true})),
            Some(session(OWNER)),
        )
        .await;
        assert_eq!(status, StatusCode::OK, "child create: {json}");
        assert_eq!(json["data"]["created"], true);

        let (status, json) = call("GET", &leaf_url, None, None).await;
        assert_eq!(status, StatusCode::OK);
        let arr = json["data"].as_array().unwrap();
        assert_eq!(arr.len(), 1, "prefix read should return exactly the child row: {json}");
        assert_eq!(arr[0]["pathLevels"], json!(["p0"]));
        assert_eq!(arr[0]["dataValue"], "{\"leaf\":true}");

        // POST 重复创建同一精确路径 → already exists（不覆盖既有行）
        let (status, json) = call(
            "POST",
            &leaf_url,
            Some(json!({"leaf": false})),
            Some(session(OWNER)),
        )
        .await;
        assert_eq!(status, StatusCode::OK);
        assert_eq!(json["type"], "error");
        assert_eq!(json["message"], "appdict data already exists");

        // IDOR：字典已有行且属他人 → 非所有者写操作一律 403
        for (method, uri) in [
            ("PUT", base.clone()),
            ("PUT", leaf_url.clone()),
            ("DELETE", leaf_url.clone()),
        ] {
            let body = if method == "DELETE" { None } else { Some(json!({"x": 1})) };
            let (status, _) = call(&method, &uri, body, Some(session(STRANGER))).await;
            assert_eq!(status, StatusCode::FORBIDDEN, "{method} {uri}");
        }

        // 所有者删除子路径后子树读为空，基座读仅剩根行
        let (status, json) = call("DELETE", &leaf_url, None, Some(session(OWNER))).await;
        assert_eq!(status, StatusCode::OK, "child delete: {json}");
        assert_eq!(json["data"]["deleted"], true);

        let (_, json) = call("GET", &leaf_url, None, None).await;
        assert_eq!(json["data"].as_array().unwrap().len(), 0);
        let (_, json) = call("GET", &format!("{base}/data"), None, None).await;
        let arr = json["data"].as_array().unwrap();
        assert_eq!(arr.len(), 1);
        assert_eq!(arr[0]["pathLevels"], json!([]));

        clean_dict().await;
    }
}
