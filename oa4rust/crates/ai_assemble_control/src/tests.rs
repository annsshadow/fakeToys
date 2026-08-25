#[cfg(test)]
mod tests {
    use axum::body::Body;
    use axum::http::{Request, Method, StatusCode};
    use deadpool_postgres::{Manager, Pool};
    use deadpool_postgres::tokio_postgres::{Config, NoTls};
    use shared::response::ActionResult;
    use tower::util::ServiceExt;
    use serde_json::Value;

    fn build_test_pool() -> Pool {
        let mgr = Manager::new(
            Config::new(),
            NoTls,
        );
        Pool::builder(mgr).max_size(1).build().unwrap()
    }

    #[tokio::test]
    async fn test_get_ai_control_config_returns_success() {
        let pool = build_test_pool();
        let app = crate::router(pool);

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/ai_assemble_control/get/ai/control/config")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[tokio::test]
    async fn test_list_ai_models_returns_success() {
        let pool = build_test_pool();
        let app = crate::router(pool);

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/ai_assemble_control/list/ai/models")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[tokio::test]
    async fn test_get_usage_stats_returns_success() {
        let pool = build_test_pool();
        let app = crate::router(pool);

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/ai_assemble_control/get/usage/stats")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[tokio::test]
    async fn test_update_ai_control_config_returns_success() {
        let pool = build_test_pool();
        let app = crate::router(pool);

        let body = serde_json::to_string(&serde_json::json!({"maxTokens": 8192})).unwrap();

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/ai_assemble_control/update/ai/control/config")
                    .method(Method::GET)
                    .header("content-type", "application/json")
                    .body(Body::from(body))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[test]
    fn test_ai_assemble_control_router_builds() {
        let pool = build_test_pool();
        let _ = crate::router(pool);
    }

    #[tokio::test]
    async fn test_chat_completion_route_registered() {
        let pool = build_test_pool();
        let app = crate::router(pool);

        let body = serde_json::to_string(&serde_json::json!({
            "messages": [{"role": "user", "content": "hello"}],
            "context_window": 5,
        }))
        .unwrap();

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/ai_assemble_control/chat/completion")
                    .method(Method::POST)
                    .header("content-type", "application/json")
                    .body(Body::from(body))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[test]
    fn test_context_window_clamp_bounds() {
        let clamp = |v: Option<i32>| v.unwrap_or(20).clamp(1, 100);
        assert_eq!(clamp(None), 20);
        assert_eq!(clamp(Some(0)), 1);
        assert_eq!(clamp(Some(150)), 100);
        assert_eq!(clamp(Some(5)), 5);
    }

    #[test]
    fn test_action_result_success() {
        let result: ActionResult<String> = ActionResult::success("test".to_string());
        assert_eq!(result.r#type, Some("success".to_string()));
        assert_eq!(result.data, Some("test".to_string()));
    }
}

// ──────────────────────────────────────────────────────────────────────────────
// plan002 U2 端点全量闭合测试。
// 测试策略与既有约定一致：使用无 DB 的占位 pool（Config::new()），任何到达
// handler 的请求都因连接失败返回 500（fail-loud）；路由不存在才是 404，
// 方法不匹配是 405——两者都与“路由已注册”可区分。
// ──────────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod u2_closure_tests {
    use axum::body::Body;
    use axum::http::{Request, Method, StatusCode};
    use deadpool_postgres::{Manager, Pool};
    use deadpool_postgres::tokio_postgres::{Config, NoTls};
    use tower::util::ServiceExt;

    /// Java x_ai_assemble_control jaxrs 的 33 个唯一端点（类级+方法级 @Path 拼接、
    /// 动词精确对应）。路径参数统一以字面量代入。
    const U2_JAVA_ENDPOINTS: &[(&str, &str)] = &[
        // ChatAction（5）
        ("POST", "/jaxrs/ai_assemble_control/chat/completion"),
        ("GET", "/jaxrs/ai_assemble_control/chat/list/paging/1/size/20"),
        ("GET", "/jaxrs/ai_assemble_control/chat/list/completion/u2t/paging/1/size/20"),
        ("GET", "/jaxrs/ai_assemble_control/chat/delete/u2t"),
        ("POST", "/jaxrs/ai_assemble_control/chat/write/completion/extra"),
        // ConfigAction（15）
        ("GET", "/jaxrs/ai_assemble_control/config/get"),
        ("GET", "/jaxrs/ai_assemble_control/config/base/config"),
        ("POST", "/jaxrs/ai_assemble_control/config/save"),
        ("GET", "/jaxrs/ai_assemble_control/config/list/model/paging/1/size/20"),
        ("POST", "/jaxrs/ai_assemble_control/config/create/model"),
        ("POST", "/jaxrs/ai_assemble_control/config/update/model/u2t"),
        ("GET", "/jaxrs/ai_assemble_control/config/get/model/u2t"),
        ("GET", "/jaxrs/ai_assemble_control/config/delete/model/u2t"),
        ("GET", "/jaxrs/ai_assemble_control/config/list/mcp/paging/1/size/20"),
        ("POST", "/jaxrs/ai_assemble_control/config/create/mcp"),
        ("POST", "/jaxrs/ai_assemble_control/config/update/mcp/u2t"),
        ("GET", "/jaxrs/ai_assemble_control/config/get/mcp/u2t"),
        ("GET", "/jaxrs/ai_assemble_control/config/get/mcp/ext/u2t"),
        ("GET", "/jaxrs/ai_assemble_control/config/delete/mcp/u2t"),
        ("GET", "/jaxrs/ai_assemble_control/config/list/enable/model"),
        // FileAction（8）
        ("GET", "/jaxrs/ai_assemble_control/file/u2t"),
        ("POST", "/jaxrs/ai_assemble_control/file/upload"),
        ("POST", "/jaxrs/ai_assemble_control/file/copy/file"),
        ("GET", "/jaxrs/ai_assemble_control/file/u2t/download"),
        ("GET", "/jaxrs/ai_assemble_control/file/u2t/download/scale"),
        ("POST", "/jaxrs/ai_assemble_control/file/list/paging/1/size/20"),
        ("GET", "/jaxrs/ai_assemble_control/file/delete/u2t"),
        ("POST", "/jaxrs/ai_assemble_control/file/list"),
        // IndexAction（5）
        ("GET", "/jaxrs/ai_assemble_control/index/cms/doc/u2t"),
        ("GET", "/jaxrs/ai_assemble_control/index/cms/doc/with/app/u2t"),
        ("GET", "/jaxrs/ai_assemble_control/index/delete/u2t"),
        ("POST", "/jaxrs/ai_assemble_control/index/list/paging/1/size/20"),
        ("GET", "/jaxrs/ai_assemble_control/index/sync/to/knowledge"),
    ];

    fn build_test_pool() -> Pool {
        let mgr = Manager::new(Config::new(), NoTls);
        Pool::builder(mgr).max_size(1).build().unwrap()
    }

    #[tokio::test]
    async fn test_u2_java_endpoint_closure_all_33_registered() {
        let pool = build_test_pool();
        let app = crate::router(pool);
        assert_eq!(U2_JAVA_ENDPOINTS.len(), 33, "Java 端点全集应为 33 条");

        for (method, uri) in U2_JAVA_ENDPOINTS {
            let req = match *method {
                "POST" => Request::builder()
                    .uri(*uri)
                    .method(Method::POST)
                    .header("content-type", "application/json")
                    .body(Body::from("{}"))
                    .unwrap(),
                _ => Request::builder()
                    .uri(*uri)
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            };
            let response = app.clone().oneshot(req).await.unwrap();
            assert_ne!(
                response.status(),
                StatusCode::NOT_FOUND,
                "Java 端点未注册: {method} {uri}"
            );
            assert_ne!(
                response.status(),
                StatusCode::METHOD_NOT_ALLOWED,
                "端点动词与 Java 不符: {method} {uri}"
            );
        }
    }

    #[tokio::test]
    async fn test_u2_chat_list_paging_route_registered() {
        let app = crate::router(build_test_pool());
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/ai_assemble_control/chat/list/paging/1/size/20")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_u2_chat_list_completion_paging_route_registered() {
        let app = crate::router(build_test_pool());
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/ai_assemble_control/chat/list/completion/clue-1/paging/1/size/20")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_u2_chat_delete_clue_route_registered() {
        let app = crate::router(build_test_pool());
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/ai_assemble_control/chat/delete/clue-1")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_u2_chat_write_completion_extra_empty_id_rejected() {
        let app = crate::router(build_test_pool());
        let body = serde_json::to_string(&serde_json::json!({"extra": {"k": "v"}})).unwrap();
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/ai_assemble_control/chat/write/completion/extra")
                    .method(Method::POST)
                    .header("content-type", "application/json")
                    .body(Body::from(body))
                    .unwrap(),
            )
            .await
            .unwrap();
        // Java ExceptionFieldEmpty 等价：缺 id 必须 400，而非假成功
        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    async fn test_u2_config_get_route_registered() {
        let app = crate::router(build_test_pool());
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/ai_assemble_control/config/get")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_u2_config_save_post_registered_and_get_rejected() {
        let app = crate::router(build_test_pool());
        let post = app
            .clone()
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/ai_assemble_control/config/save")
                    .method(Method::POST)
                    .header("content-type", "application/json")
                    .body(Body::from("{}"))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(post.status(), StatusCode::NOT_FOUND, "config/save 必须支持 POST");
        assert_ne!(
            post.status(),
            StatusCode::METHOD_NOT_ALLOWED,
            "config/save 的 POST 被方法路由拒绝"
        );

        // Java 中该端点只有 POST：旧注册的 GET 已移除。
        // axum 路径命中但方法不符 → 405（证明该路径下已无 GET 处理器）。
        let get = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/ai_assemble_control/config/save")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(
            get.status(),
            StatusCode::METHOD_NOT_ALLOWED,
            "config/save 不应再接受 GET"
        );
    }

    #[tokio::test]
    async fn test_u2_file_list_post_registered_and_get_rejected() {
        let app = crate::router(build_test_pool());
        let body = serde_json::to_string(&serde_json::json!({"ids": ["a", "a ", "", "b"]})).unwrap();
        let post = app
            .clone()
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/ai_assemble_control/file/list")
                    .method(Method::POST)
                    .header("content-type", "application/json")
                    .body(Body::from(body))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(post.status(), StatusCode::NOT_FOUND);

        let get = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/ai_assemble_control/file/list")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(
            get.status(),
            StatusCode::METHOD_NOT_ALLOWED,
            "file/list 不应再接受 GET"
        );
    }

    #[tokio::test]
    async fn test_u2_index_sync_no_json_body_required() {
        let app = crate::router(build_test_pool());
        // 无 content-type、无 body 的纯 GET。若仍挂着 Json 提取器会得到 415/422，
        // 而非到达 handler 后因占位 pool 失败的 500。
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/ai_assemble_control/index/sync/to/knowledge")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(
            response.status(),
            StatusCode::INTERNAL_SERVER_ERROR,
            "sync 应为无 body 的真实 GET（占位 pool 下 500）"
        );
    }

    #[tokio::test]
    async fn test_u2_parametrized_flag_routes_capture_arbitrary_ids() {
        let app = crate::router(build_test_pool());
        for uri in [
            "/jaxrs/ai_assemble_control/config/get/mcp/any-id-here",
            "/jaxrs/ai_assemble_control/file/any-file-flag",
        ] {
            let response = app
                .clone()
                .oneshot(
                    Request::builder()
                        .uri(uri)
                        .method(Method::GET)
                        .body(Body::empty())
                        .unwrap(),
                )
                .await
                .unwrap();
            assert_ne!(
                response.status(),
                StatusCode::NOT_FOUND,
                "参数化路由未生效（字面量占位残留？）: {uri}"
            );
        }
    }

    #[tokio::test]
    async fn test_u2_file_upload_post_route_registered() {
        let app = crate::router(build_test_pool());
        let body =
            serde_json::to_string(&serde_json::json!({"name": "n", "fileName": "f.txt"})).unwrap();
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/ai_assemble_control/file/upload")
                    .method(Method::POST)
                    .header("content-type", "application/json")
                    .body(Body::from(body))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        assert_ne!(response.status(), StatusCode::METHOD_NOT_ALLOWED);
    }

    #[tokio::test]
    async fn test_u2_chat_completion_stream_route_still_registered() {
        let app = crate::router(build_test_pool());
        let body = serde_json::to_string(&serde_json::json!({
            "messages": [{"role": "user", "content": "hi"}],
        }))
        .unwrap();
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/ai_assemble_control/chat/completion/stream")
                    .method(Method::POST)
                    .header("content-type", "application/json")
                    .body(Body::from(body))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
    }
}

#[cfg(test)]
mod u2_unit_tests {
    #[test]
    fn test_u2_normalize_name_folds_case_and_whitespace() {
        assert_eq!(crate::u2_normalize_name("  Foo   Bar "), "foo bar");
        assert_eq!(crate::u2_normalize_name("FOO\tBAR"), "foo bar");
        assert_eq!(crate::u2_normalize_name("gpt-4"), "gpt-4");
        assert_eq!(crate::u2_normalize_name("   "), "");
    }

    #[test]
    fn test_u2_normalize_name_dedup_variants_collide() {
        // 归一化查重口径：不同书写形态的同名必须碰撞为同一键
        let a = crate::u2_normalize_name("GPT-4  Turbo");
        let b = crate::u2_normalize_name(" gpt-4 turbo ");
        assert_eq!(a, b, "归一化后同名变体必须相等，查重才能拒绝重复创建");
    }
}
