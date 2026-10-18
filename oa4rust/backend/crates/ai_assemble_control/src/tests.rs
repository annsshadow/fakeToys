// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

#[cfg(test)]
mod tests {
    use axum::body::Body;
    use axum::http::{Method, Request, StatusCode};
    use deadpool_postgres::tokio_postgres::{Config, NoTls};
    use deadpool_postgres::{Manager, Pool};
    use shared::response::ActionResult;
    use tower::util::ServiceExt;

    fn build_test_pool() -> Pool {
        let mgr = Manager::new(Config::new(), NoTls);
        Pool::builder(mgr).max_size(1).build().unwrap()
    }

    #[tokio::test]
    async fn test_get_ai_control_config_returns_success() {
        let pool = build_test_pool();
        let app = crate::router(pool);

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/ai_assemble_control/get/ai/control/config")
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
                    .uri("/api/ai_assemble_control/list/ai/models")
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
                    .uri("/api/ai_assemble_control/get/usage/stats")
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
                    .uri("/api/ai_assemble_control/update/ai/control/config")
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
                    .uri("/api/ai_assemble_control/chat/completion")
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
    use axum::http::{Method, Request, StatusCode};
    use deadpool_postgres::tokio_postgres::{Config, NoTls};
    use deadpool_postgres::{Manager, Pool};
    use tower::util::ServiceExt;

    /// o2server x_ai_assemble_control o2server 的 33 个唯一端点（类级+方法级 路径拼接、
    /// 动词精确对应）。路径参数统一以字面量代入。
    const U2_LEGACY_ENDPOINTS: &[(&str, &str)] = &[
        // ChatAction（5）
        ("POST", "/api/ai_assemble_control/chat/completion"),
        ("GET", "/api/ai_assemble_control/chat/list/paging/1/size/20"),
        (
            "GET",
            "/api/ai_assemble_control/chat/list/completion/u2t/paging/1/size/20",
        ),
        ("GET", "/api/ai_assemble_control/chat/delete/u2t"),
        (
            "POST",
            "/api/ai_assemble_control/chat/write/completion/extra",
        ),
        // ConfigAction（15）
        ("GET", "/api/ai_assemble_control/config/get"),
        ("GET", "/api/ai_assemble_control/config/base/config"),
        ("POST", "/api/ai_assemble_control/config/save"),
        (
            "GET",
            "/api/ai_assemble_control/config/list/model/paging/1/size/20",
        ),
        ("POST", "/api/ai_assemble_control/config/create/model"),
        ("POST", "/api/ai_assemble_control/config/update/model/u2t"),
        ("GET", "/api/ai_assemble_control/config/get/model/u2t"),
        ("GET", "/api/ai_assemble_control/config/delete/model/u2t"),
        (
            "GET",
            "/api/ai_assemble_control/config/list/mcp/paging/1/size/20",
        ),
        ("POST", "/api/ai_assemble_control/config/create/mcp"),
        ("POST", "/api/ai_assemble_control/config/update/mcp/u2t"),
        ("GET", "/api/ai_assemble_control/config/get/mcp/u2t"),
        ("GET", "/api/ai_assemble_control/config/get/mcp/ext/u2t"),
        ("GET", "/api/ai_assemble_control/config/delete/mcp/u2t"),
        ("GET", "/api/ai_assemble_control/config/list/enable/model"),
        // FileAction（8）
        ("GET", "/api/ai_assemble_control/file/u2t"),
        ("POST", "/api/ai_assemble_control/file/upload"),
        ("POST", "/api/ai_assemble_control/file/copy/file"),
        ("GET", "/api/ai_assemble_control/file/u2t/download"),
        ("GET", "/api/ai_assemble_control/file/u2t/download/scale"),
        (
            "POST",
            "/api/ai_assemble_control/file/list/paging/1/size/20",
        ),
        ("GET", "/api/ai_assemble_control/file/delete/u2t"),
        ("POST", "/api/ai_assemble_control/file/list"),
        // IndexAction（5）
        ("GET", "/api/ai_assemble_control/index/cms/doc/u2t"),
        ("GET", "/api/ai_assemble_control/index/cms/doc/with/app/u2t"),
        ("GET", "/api/ai_assemble_control/index/delete/u2t"),
        (
            "POST",
            "/api/ai_assemble_control/index/list/paging/1/size/20",
        ),
        ("GET", "/api/ai_assemble_control/index/sync/to/knowledge"),
    ];

    fn build_test_pool() -> Pool {
        let mgr = Manager::new(Config::new(), NoTls);
        Pool::builder(mgr).max_size(1).build().unwrap()
    }

    #[tokio::test]
    async fn test_u2_legacy_endpoint_closure_all_33_registered() {
        let pool = build_test_pool();
        let app = crate::router(pool);
        assert_eq!(U2_LEGACY_ENDPOINTS.len(), 33, "o2server 端点全集应为 33 条");

        for (method, uri) in U2_LEGACY_ENDPOINTS {
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
                "o2server 端点未注册: {method} {uri}"
            );
            assert_ne!(
                response.status(),
                StatusCode::METHOD_NOT_ALLOWED,
                "端点动词与 o2server 不符: {method} {uri}"
            );
        }
    }

    #[tokio::test]
    async fn test_u2_chat_list_paging_route_registered() {
        let app = crate::router(build_test_pool());
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/ai_assemble_control/chat/list/paging/1/size/20")
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
                    .uri("/api/ai_assemble_control/chat/list/completion/clue-1/paging/1/size/20")
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
                    .uri("/api/ai_assemble_control/chat/delete/clue-1")
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
                    .uri("/api/ai_assemble_control/chat/write/completion/extra")
                    .method(Method::POST)
                    .header("content-type", "application/json")
                    .body(Body::from(body))
                    .unwrap(),
            )
            .await
            .unwrap();
        // o2server ExceptionFieldEmpty 等价：缺 id 必须 400，而非假成功
        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    async fn test_u2_config_get_route_registered() {
        let app = crate::router(build_test_pool());
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/ai_assemble_control/config/get")
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
                    .uri("/api/ai_assemble_control/config/save")
                    .method(Method::POST)
                    .header("content-type", "application/json")
                    .body(Body::from("{}"))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(
            post.status(),
            StatusCode::NOT_FOUND,
            "config/save 必须支持 POST"
        );
        assert_ne!(
            post.status(),
            StatusCode::METHOD_NOT_ALLOWED,
            "config/save 的 POST 被方法路由拒绝"
        );

        // o2server 中该端点只有 POST：旧注册的 GET 已移除。
        // axum 路径命中但方法不符 → 405（证明该路径下已无 GET 处理器）。
        let get = app
            .oneshot(
                Request::builder()
                    .uri("/api/ai_assemble_control/config/save")
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
        let body =
            serde_json::to_string(&serde_json::json!({"ids": ["a", "a ", "", "b"]})).unwrap();
        let post = app
            .clone()
            .oneshot(
                Request::builder()
                    .uri("/api/ai_assemble_control/file/list")
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
                    .uri("/api/ai_assemble_control/file/list")
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
        // 注：index_sync_to_knowledge 已收 admin 门（无会话先于读库回 401），
        // 401 同样证明它是无 body 的真实 GET（需 body 会在到达门之前 415/422）。
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/ai_assemble_control/index/sync/to/knowledge")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(
            response.status(),
            StatusCode::UNAUTHORIZED,
            "sync 应为无 body 的真实 GET（收 admin 门后无会话回 401）"
        );
    }

    #[tokio::test]
    async fn test_u2_parametrized_flag_routes_capture_arbitrary_ids() {
        let app = crate::router(build_test_pool());
        for uri in [
            "/api/ai_assemble_control/config/get/mcp/any-id-here",
            "/api/ai_assemble_control/file/any-file-flag",
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
                    .uri("/api/ai_assemble_control/file/upload")
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
                    .uri("/api/ai_assemble_control/chat/completion/stream")
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

    // 回归（四轮审计 GET 写面队列 · ai_assemble_control 4 把）：
    // file_delete_flag / index_delete_flag（按 id 删、无归属门）、index_sync_to_knowledge
    // （批量维护写）、chat_delete_clue_id（原原始串比较改 require_owner）。
    #[tokio::test]
    async fn test_ai_get_writes_owner_admin_gate() {
        // 无会话离线臂：四把全 401（先于读库；出现 Internal 即门插在读库之后）。
        let dead = {
            let mgr = deadpool_postgres::Manager::new(
                deadpool_postgres::tokio_postgres::Config::new(),
                deadpool_postgres::tokio_postgres::NoTls,
            );
            deadpool_postgres::Pool::builder(mgr)
                .max_size(1)
                .build()
                .unwrap()
        };
        let nosess: Vec<(&str, Result<_, shared::error::AppError>)> = vec![
            (
                "file_delete_flag",
                crate::file_delete_flag(
                    axum::extract::Extension(dead.clone()),
                    None,
                    axum::extract::Path("f".to_string()),
                )
                .await,
            ),
            (
                "index_delete_flag",
                crate::index_delete_flag(
                    axum::extract::Extension(dead.clone()),
                    None,
                    axum::extract::Path("i".to_string()),
                )
                .await,
            ),
            (
                "index_sync_to_knowledge",
                crate::index_sync_to_knowledge(axum::extract::Extension(dead.clone()), None).await,
            ),
            (
                "chat_delete_clue_id",
                crate::chat_delete_clue_id(
                    axum::extract::Extension(dead.clone()),
                    None,
                    axum::extract::Path("c".to_string()),
                )
                .await,
            ),
        ];
        for (name, r) in nosess {
            assert!(
                matches!(r, Err(shared::error::AppError::Unauthorized)),
                "{name} 无会话必须 401，实得 {:?}",
                r.as_ref().err()
            );
        }

        if !shared::testing::is_db_available().await {
            eprintln!(
                "skipping test_ai_get_writes_owner_admin_gate(real-db arm): DATABASE_URL not reachable"
            );
            return;
        }
        let pool = shared::testing::test_pool();
        const OWNER: &str = "ai-gate-owner";
        const OTHER: &str = "ai-gate-other";
        const ADMIN: &str = "ai-gate-admin";
        const ROLE: &str = "ai-gate-role-admin";

        let sess = |p: &str| Some(axum::extract::Extension(shared::testing::test_session(p)));
        async fn exists(pool: &deadpool_postgres::Pool, sql: &str, key: &str) -> i64 {
            pool.get()
                .await
                .unwrap()
                .query_one(sql, &[&key.to_string()])
                .await
                .unwrap()
                .get::<_, i64>(0)
        }

        // 造 admin + 归属行。
        {
            let c = pool.get().await.unwrap();
            c.execute(
                "INSERT INTO auth_role (id, name, disable) VALUES ($1,'admin',false) \
                 ON CONFLICT (id) DO UPDATE SET name='admin', disable=false",
                &[&ROLE.to_string()],
            )
            .await
            .unwrap();
            c.execute(
                "INSERT INTO auth_person (id, unique_id, name, password_hash) \
                 VALUES ($1,$1,'ai gate admin','x') ON CONFLICT (id) DO NOTHING",
                &[&ADMIN.to_string()],
            )
            .await
            .unwrap();
            c.execute(
                "INSERT INTO auth_person_role (person_id, role_id, unit_id) \
                 VALUES ($1,$2,'ai-gate-unit') ON CONFLICT DO NOTHING",
                &[&ADMIN.to_string(), &ROLE.to_string()],
            )
            .await
            .unwrap();
            for id in ["ai-gate-file", "ai-gate-file-owner", "ai-gate-file-admin"] {
                // x_ai_file.id 无唯一约束，ON CONFLICT 报 E42P10；改用 DELETE+INSERT 幂等造数。
                c.execute("DELETE FROM x_ai_file WHERE id = $1", &[&id.to_string()])
                    .await
                    .unwrap();
                c.execute(
                    "INSERT INTO x_ai_file (id, name, file_name, creator, enabled, create_time) \
                     VALUES ($1,$1,$1,$2,true,NOW())",
                    &[&id.to_string(), &OWNER.to_string()],
                )
                .await
                .unwrap();
            }
            for id in ["ai-gate-index", "ai-gate-index-owner"] {
                // x_ai_index.id 无唯一约束，ON CONFLICT 报 E42P10；改用 DELETE+INSERT 幂等造数。
                c.execute("DELETE FROM x_ai_index WHERE id = $1", &[&id.to_string()])
                    .await
                    .unwrap();
                c.execute(
                    "INSERT INTO x_ai_index (id, creator, create_time) VALUES ($1,$2,NOW())",
                    &[&id.to_string(), &OWNER.to_string()],
                )
                .await
                .unwrap();
            }
            for id in ["ai-gate-chat", "ai-gate-chat-owner"] {
                // x_ai_conversation 现需非 null title（schema 漂移）；DELETE+INSERT 幂等造数。
                c.execute("DELETE FROM x_ai_conversation WHERE id = $1", &[&id.to_string()])
                    .await
                    .unwrap();
                c.execute(
                    "INSERT INTO x_ai_conversation (id, title, user_id, create_time) \
                     VALUES ($1,$1,$2,NOW())",
                    &[&id.to_string(), &OWNER.to_string()],
                )
                .await
                .unwrap();
            }
        }

        // 非 admin / 非属主 → 403，且不落笔。
        assert!(
            matches!(
                crate::file_delete_flag(axum::extract::Extension(pool.clone()), sess(OTHER), axum::extract::Path("ai-gate-file".to_string())).await,
                Err(shared::error::AppError::Forbidden)
            ),
            "file_delete_flag 他人必须 403"
        );
        assert!(
            matches!(
                crate::index_delete_flag(axum::extract::Extension(pool.clone()), sess(OTHER), axum::extract::Path("ai-gate-index".to_string())).await,
                Err(shared::error::AppError::Forbidden)
            ),
            "index_delete_flag 他人必须 403"
        );
        assert!(
            matches!(
                crate::chat_delete_clue_id(axum::extract::Extension(pool.clone()), sess(OTHER), axum::extract::Path("ai-gate-chat".to_string())).await,
                Err(shared::error::AppError::Forbidden)
            ),
            "chat_delete_clue_id 他人必须 403"
        );
        assert!(
            matches!(
                crate::index_sync_to_knowledge(axum::extract::Extension(pool.clone()), sess(OTHER)).await,
                Err(shared::error::AppError::Forbidden)
            ),
            "index_sync_to_knowledge 非 admin 必须 403"
        );
        // 403 侧不落笔。
        assert_eq!(exists(&pool, "SELECT COUNT(*) FROM x_ai_file WHERE id=$1", "ai-gate-file").await, 1);
        assert_eq!(exists(&pool, "SELECT COUNT(*) FROM x_ai_index WHERE id=$1", "ai-gate-index").await, 1);
        assert_eq!(exists(&pool, "SELECT COUNT(*) FROM x_ai_conversation WHERE id=$1 AND deleted_at IS NULL", "ai-gate-chat").await, 1);

        // 属主放行：file / index 真删、chat 软删。
        assert!(crate::file_delete_flag(axum::extract::Extension(pool.clone()), sess(OWNER), axum::extract::Path("ai-gate-file-owner".to_string())).await.is_ok(), "owner file_delete 应放行");
        assert!(crate::index_delete_flag(axum::extract::Extension(pool.clone()), sess(OWNER), axum::extract::Path("ai-gate-index-owner".to_string())).await.is_ok(), "owner index_delete 应放行");
        assert!(crate::chat_delete_clue_id(axum::extract::Extension(pool.clone()), sess(OWNER), axum::extract::Path("ai-gate-chat-owner".to_string())).await.is_ok(), "owner chat_delete 应放行");
        assert_eq!(exists(&pool, "SELECT COUNT(*) FROM x_ai_file WHERE id=$1", "ai-gate-file-owner").await, 0, "owner 删后文件应消失");
        assert_eq!(exists(&pool, "SELECT COUNT(*) FROM x_ai_index WHERE id=$1", "ai-gate-index-owner").await, 0, "owner 删后索引应消失");
        assert_eq!(exists(&pool, "SELECT COUNT(*) FROM x_ai_conversation WHERE id=$1 AND deleted_at IS NULL", "ai-gate-chat-owner").await, 0, "owner 删后会话应软删");

        // admin 放行：owner-gated 那把走 admin 旁路删自己名下的行；admin-gated 那把跑批量。
        assert!(crate::file_delete_flag(axum::extract::Extension(pool.clone()), sess(ADMIN), axum::extract::Path("ai-gate-file-admin".to_string())).await.is_ok(), "admin file_delete 应放行（admin 旁路）");
        assert!(crate::index_sync_to_knowledge(axum::extract::Extension(pool.clone()), sess(ADMIN)).await.is_ok(), "admin index_sync 应放行");

        // 收口：清造数，零残留。
        let c = pool.get().await.unwrap();
        c.execute("DELETE FROM x_ai_file WHERE id LIKE 'ai-gate-%'", &[]).await.unwrap();
        c.execute("DELETE FROM x_ai_index WHERE id LIKE 'ai-gate-%'", &[]).await.unwrap();
        c.execute("DELETE FROM x_ai_conversation WHERE id LIKE 'ai-gate-%'", &[]).await.unwrap();
        c.execute("DELETE FROM auth_person_role WHERE person_id=$1", &[&ADMIN.to_string()]).await.unwrap();
        c.execute("DELETE FROM auth_person WHERE id=$1", &[&ADMIN.to_string()]).await.unwrap();
        c.execute("DELETE FROM auth_role WHERE id=$1", &[&ROLE.to_string()]).await.unwrap();
    }
}
