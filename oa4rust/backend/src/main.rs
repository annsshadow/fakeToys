// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

use std::env;
use std::sync::Arc;

use anyhow::Context as _;
use axum::response::IntoResponse;
use axum::Router;
use mcp_server::tool_bridge::ToolBridge;
use openapi::ApiDoc;
use shared::db::create_pool;
use shared::middleware::{
    auth_middleware, authorize_middleware, rate_limit_middleware, security_headers_middleware,
    trace_middleware,
};
use std::sync::OnceLock;

use shared::rate_limit::RateLimiter;
use shared::session::SessionManager;
use tracing_subscriber::EnvFilter;
use utoipa_swagger_ui::{Config, SwaggerUi};

/// 第 44 轮：OpenAPI 文档是静态的（4733 个 path item），此前**每次请求**
/// 都重新 `ApiDoc::openapi().to_json()` 序列化一遍（数百 KB / 数十 ms）。
/// 启动后生成一次缓存；失败（编程错误，静态文档应恒定可生成）也缓存错误，
/// 不会每次请求重复付生成代价。
fn openapi_json() -> Result<&'static str, &'static str> {
    use utoipa::OpenApi;
    static CACHE: OnceLock<Result<String, String>> = OnceLock::new();
    CACHE
        .get_or_init(|| ApiDoc::openapi().to_json().map_err(|e| e.to_string()))
        .as_ref()
        .map(String::as_str)
        .map_err(String::as_str)
}

/// OpenAPI JSON endpoint handler.
async fn openapi_json_handler() -> Result<Vec<u8>, axum::response::Json<serde_json::Value>> {
    match openapi_json() {
        Ok(json) => Ok(json.as_bytes().to_vec()),
        Err(e) => Err(axum::Json(serde_json::json!({"error": e}))),
    }
}

/// 2026-11-15：文档路由（OpenAPI JSON + Swagger UI 页面）。
///
/// 刻意**不用** `SwaggerUi::url("/openapi.json", ApiDoc::openapi())`：那个写法会替
/// 同名路径再注册一条 GET 路由，与下面 `.route("/openapi.json", ...)` 重叠，
/// `Router::merge` 会直接 panic；而且会在启动时第二次生成 4733 个 path item。
/// 改成 `Config::new(["/openapi.json"])` 只把页面指向既有端点：
/// Swagger UI 只挂 `/swagger-ui`、`/swagger-ui/`、`/swagger-ui/{*rest}` 三条，
/// 文档本体仍走 `openapi_json()` 的 OnceLock 缓存。
fn docs_router() -> Router {
    Router::new()
        .route("/openapi.json", axum::routing::get(openapi_json_handler))
        .merge(SwaggerUi::new("/swagger-ui").config(Config::new(["/openapi.json"])))
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env().add_directive("o2server=debug".parse()?))
        .init();

    dotenvy::dotenv().ok();

    let auth_config = shared::config::AuthConfig::from_env()
        .context("invalid browser authentication configuration")?;

    // 安全审计（2026-10）：AUTH_LEGACY_COMPAT 开启后密码验证与 SSO 解密会接受
    // 旧 MD5/DES/3DES 格式（一次性迁移期用途）。生产环境若开启须在日志留痕。
    if auth::password::legacy_compat_from_env() {
        tracing::warn!(
            "AUTH_LEGACY_COMPAT=true: legacy MD5/DES password hashes and 3DES SSO tokens accepted. \
             This is a one-time migration setting and should be disabled once data is migrated."
        );
    }

    let args: Vec<String> = env::args().collect();
    let http_flag = args.iter().any(|a| a == "--http");
    let migrate_only = args.iter().any(|a| a == "--migrate-only");

    let pool = create_pool()
        .await
        .context("failed to create database pool")?;

    // 启动时自行应用数据库迁移（幂等），无需手工执行 SQL。
    let report = shared::migrate::run_migrations(&pool)
        .await
        .context("failed to run database migrations")?;
    tracing::info!(
        "migrations: {} applied, {} already applied (skipped)",
        report.applied.len(),
        report.skipped.len()
    );

    if migrate_only {
        tracing::info!("migrate-only requested; exiting after applying migrations");
        return Ok(());
    }

    let session_manager = SessionManager::with_pool_and_config(pool.clone(), auth_config.clone());
    let rate_limiter = RateLimiter::new();

    // 单实例部署（2026-10-28 决策）：会话为内存 + PostgreSQL 持久化，
    // 限流为进程内计数，不再依赖外部缓存服务。
    tracing::info!(
        "single-instance mode: in-memory session store with PostgreSQL persistence, in-process rate limiting"
    );

    // plan002 U7b: LDAP_SYNC_ENABLE=true 时启动 LDAP 用户自动同步定时 worker（幂等）
    if ldap::sync::init_from_env(pool.clone()) {
        tracing::info!("LDAP user sync worker started (LDAP_SYNC_ENABLE=true)");
    }

    let app =
        oa4rust::create_app(pool.clone(), session_manager.clone(), rate_limiter.clone()).await?;

    // Mount OpenAPI JSON and Swagger UI before other layers（2026-11-15：UI 真正挂上）
    let app = app.merge(docs_router());

    // Optionally mount the MCP HTTP endpoint at /mcp when --http flag is present.
    let security_state = shared::middleware::SecurityState {
        session_manager: session_manager.clone(),
        rate_limiter: rate_limiter.clone(),
        pool: pool.clone(),
    };
    let app = if http_flag {
        let bridge = Arc::new(ToolBridge::new(pool, session_manager).await);
        // /mcp 在 create_app 之后挂载，不继承那里的层；这里单独套同一组请求防护，
        // 保证可选端点与主路由口径一致（R24 body limit + R31 超时/panic）。
        let body_limit = shared::middleware::request_body_limit_from_env()?;
        let timeout = shared::middleware::request_timeout_from_env()?;
        app.merge(mcp_app(bridge, security_state, body_limit, timeout))
    } else {
        app
    };

    // ── 静态文件服务（前端构建产物）─────────────────────────────────────
    // OA4RUST_WEB_DIST 环境变量指定 dist 目录，默认相对于二进制位置
    // （backend/target/{debug,release}/）向上三级到 oa4rust/，再进 frontend/dist/web。
    // SPA 深链回退：/login、/app/** 等前端路由刷新时回 index.html；/api/** 与
    // /ws/** 未知路径保持 404（API 客户端不应收到 HTML）。
    let web_dist =
        env::var("OA4RUST_WEB_DIST").unwrap_or_else(|_| "../../../frontend/dist/web".to_string());
    let web_dist_root = std::path::PathBuf::from(&web_dist);
    // 第 45 轮：index.html 内存缓存（SPA 深链每次请求都可能回读它）。
    // 用 mtime 判失效：构建产物被替换后下一次请求自动重读。
    let index_cache = Arc::new(SpaIndexCache::new(web_dist_root.join("index.html")));
    let spa = tower::service_fn(move |req: axum::http::Request<axum::body::Body>| {
        let web_dist_root = web_dist_root.clone();
        let index_cache = index_cache.clone();
        async move {
            let path = req.uri().path();
            if path.starts_with("/api/") || path.starts_with("/ws/") {
                return Ok::<_, std::convert::Infallible>(
                    axum::http::StatusCode::NOT_FOUND.into_response(),
                );
            }
            // 拒绝目录穿越（逐段精确匹配 ".."）
            if path.split('/').any(|segment| segment == "..") {
                return Ok(axum::http::StatusCode::FORBIDDEN.into_response());
            }
            let rel = path.trim_start_matches('/');
            let candidate = if rel.is_empty() {
                web_dist_root.join("index.html")
            } else {
                web_dist_root.join(rel)
            };
            // 命中静态文件则原样返回；未命中回 index.html（SPA 深链）
            if let Ok(bytes) = tokio::fs::read(&candidate).await {
                let mime = mime_for_path(&candidate);
                let mut resp = axum::http::Response::builder()
                    .header(axum::http::header::CONTENT_TYPE, mime)
                    .body(axum::body::Body::from(bytes))
                    .unwrap();
                // 第 45 轮：Vite 构建产物 `/assets/*` 带内容哈希，不可变。
                // 加 immutable 长缓存后浏览器不再回源验证；其余文件（index.html
                // 本身、public 下的无哈希资源）由 SPA 深链路径处理，不加缓存。
                if path.starts_with("/assets/") {
                    resp.headers_mut().insert(
                        axum::http::header::CACHE_CONTROL,
                        axum::http::HeaderValue::from_static("public, max-age=31536000, immutable"),
                    );
                }
                return Ok(resp);
            }
            // 第 45 轮：index.html 走内存缓存（mtime 失效），不再每次磁盘读。
            match index_cache.read().await {
                Ok(bytes) => Ok(axum::http::Response::builder()
                    .header(axum::http::header::CONTENT_TYPE, "text/html; charset=utf-8")
                    .body(axum::body::Body::from(bytes))
                    .unwrap()),
                Err(_) => Ok(axum::http::StatusCode::NOT_FOUND.into_response()),
            }
        }
    });
    let app = app.fallback_service(spa);
    tracing::info!(web_dist, "static frontend files mounted with SPA fallback");

    // 监听地址默认 0.0.0.0:3000；OA4RUST_BIND_ADDR 供部署与重启回归使用。
    let bind_addr = env::var("OA4RUST_BIND_ADDR").unwrap_or_else(|_| "0.0.0.0:3000".to_string());
    let listener = tokio::net::TcpListener::bind(&bind_addr).await?;
    tracing::info!("listening on {}", listener.local_addr()?);
    // 优雅停机：SIGINT/SIGTERM 后停止接新连接，等在途请求跑完（最多 10s）。
    // 修前直接 `axum::serve(...).await`，部署重启会打断在途事务。
    shared::graceful::serve_router_with_shutdown(
        listener,
        app,
        shared::graceful::DEFAULT_GRACE_PERIOD,
        shared::graceful::shutdown_signal(),
    )
    .await?;

    Ok(())
}

/// 请求层防护：与 `create_app` 同口径的超时 + panic 收敛 + body 上限。
///
/// `/mcp` 是在 `create_app` 之后 merge 的独立子应用，主路由的层不会包住它；
/// 修前只有 body limit 与鉴权/限流/日志，漏了 R5 超时与 R11 panic 收敛 ——
/// 工具调用卡在慢库/池等待时会无限挂，panic 会直接断连。
fn apply_mcp_request_guards(
    router: Router,
    timeout: Option<std::time::Duration>,
    body_limit: usize,
) -> Router {
    // 与 create_app 相同顺序：panic 在内层（先收敛），超时在外一层，body 上限在最外。
    let router = shared::panic_guard::apply_panic_guard(router);
    let router = shared::middleware::apply_request_timeout(router, timeout);
    shared::middleware::apply_body_limit(router, body_limit)
}

/// Build the MCP HTTP sub-application mounted at /mcp.
/// Forwards the caller's Authorization header to the internal ToolBridge so
/// existing session-based auth is reused without duplicating business logic.
fn mcp_app(
    bridge: Arc<ToolBridge>,
    security_state: shared::middleware::SecurityState,
    body_limit: usize,
    timeout: Option<std::time::Duration>,
) -> Router {
    use axum::middleware;
    use axum::routing::post;

    async fn mcp_handler(
        axum::extract::State(bridge): axum::extract::State<Arc<ToolBridge>>,
        axum::extract::Json(req): axum::extract::Json<serde_json::Value>,
    ) -> axum::Json<serde_json::Value> {
        use mcp_server::tool_bridge::{JsonRpcResponse, ToolCallParams};

        let method = req.get("method").and_then(|v| v.as_str()).unwrap_or("");
        let id = req.get("id").cloned();

        let result: Result<serde_json::Value, mcp_server::tool_bridge::McpError> = match method {
            "initialize" => {
                let result = serde_json::json!({
                    "protocolVersion": "2024-11-05",
                    "capabilities": {"tools": {"listChanged": false}},
                    "serverInfo": {"name": "oa4rust-mcp", "version": "0.1.0"}
                });
                Ok(result)
            }
            "tools/list" => {
                let tools = bridge.list_tools();
                Ok(serde_json::to_value(tools).unwrap_or(serde_json::json!([])))
            }
            "tools/call" => {
                let params = req.get("params").cloned().unwrap_or(serde_json::json!({}));
                let tool_call: ToolCallParams = match serde_json::from_value(params) {
                    Ok(p) => p,
                    Err(e) => {
                        return axum::Json(
                            JsonRpcResponse::err(id, -32600, format!("invalid params: {}", e))
                                .into_json_value(),
                        );
                    }
                };
                match bridge.call_tool(tool_call).await {
                    Ok(resp) => Ok(
                        serde_json::to_value(resp).unwrap_or(serde_json::json!({"content": []}))
                    ),
                    Err(e) => {
                        return axum::Json(
                            JsonRpcResponse::err(id, e.code, e.message).into_json_value(),
                        );
                    }
                }
            }
            _ => {
                return axum::Json(
                    JsonRpcResponse::err(id, -32601, format!("method not found: {}", method))
                        .into_json_value(),
                );
            }
        };

        match result {
            Ok(r) => axum::Json(JsonRpcResponse::ok(id, r).into_json_value()),
            Err(e) => axum::Json(JsonRpcResponse::err(id, e.code, e.message).into_json_value()),
        }
    }

    apply_mcp_request_guards(
        Router::new()
            .route("/mcp", post(mcp_handler).with_state(Arc::clone(&bridge)))
            .layer(middleware::from_fn_with_state(
                security_state.clone(),
                authorize_middleware,
            ))
            .layer(middleware::from_fn_with_state(
                security_state.clone(),
                auth_middleware,
            ))
            .layer(middleware::from_fn_with_state(
                security_state.clone(),
                shared::middleware::csrf_middleware,
            ))
            .layer(middleware::from_fn_with_state(
                security_state.clone(),
                rate_limit_middleware,
            ))
            .layer(middleware::from_fn(security_headers_middleware))
            .layer(middleware::from_fn(trace_middleware)),
        timeout,
        body_limit,
    )
}

/// SPA 静态服务的最小 mime 映射（按扩展名）
fn mime_for_path(path: &std::path::Path) -> &'static str {
    match path
        .extension()
        .and_then(|ext| ext.to_str())
        .unwrap_or("")
        .to_ascii_lowercase()
        .as_str()
    {
        "html" => "text/html; charset=utf-8",
        "js" | "mjs" => "text/javascript; charset=utf-8",
        "css" => "text/css; charset=utf-8",
        "json" => "application/json; charset=utf-8",
        "svg" => "image/svg+xml",
        "png" => "image/png",
        "jpg" | "jpeg" => "image/jpeg",
        "gif" => "image/gif",
        "webp" => "image/webp",
        "ico" => "image/x-icon",
        "woff" => "font/woff",
        "woff2" => "font/woff2",
        "ttf" => "font/ttf",
        "wasm" => "application/wasm",
        "map" => "application/json; charset=utf-8",
        "txt" => "text/plain; charset=utf-8",
        "webmanifest" => "application/manifest+json",
        _ => "application/octet-stream",
    }
}

/// 第 45 轮：SPA 入口页的内存缓存。`read()` 返回缓存字节，
/// 文件的 mtime 变化（构建产物被替换）时自动重读。
/// `state` 用类型别名收敛（clippy type_complexity）：
/// `(mtime, 缓存字节)` 的锁保护可选项，包装在 Arc 里供闭包共享。
type SpaIndexCacheState = Arc<tokio::sync::Mutex<Option<(std::time::SystemTime, Vec<u8>)>>>;

struct SpaIndexCache {
    path: std::path::PathBuf,
    state: SpaIndexCacheState,
}

impl SpaIndexCache {
    fn new(path: std::path::PathBuf) -> Self {
        Self {
            path,
            state: Arc::new(tokio::sync::Mutex::new(None)),
        }
    }

    async fn read(&self) -> std::io::Result<Vec<u8>> {
        let mut guard = self.state.lock().await;
        let mtime = tokio::fs::metadata(&self.path).await?.modified()?;
        if let Some((cached_mtime, bytes)) = guard.as_ref() {
            if *cached_mtime == mtime {
                return Ok(bytes.clone());
            }
        }
        let bytes = tokio::fs::read(&self.path).await?;
        *guard = Some((mtime, bytes.clone()));
        Ok(bytes)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn openapi_json_is_stable_and_contains_paths() {
        // 第 44 轮：缓存后的两次读取必须返回同一份静态文档（OnceLock）。
        let first = openapi_json().expect("openapi json 应可生成");
        let second = openapi_json().expect("openapi json 再次生成");
        assert_eq!(first, second, "缓存命中必须返回同一份文档");
        // raw string 写法：避免 quoted_key_guard 误报转义引号键（O3）
        assert!(first.contains(r#""paths""#), "OpenAPI 文档应包含 paths");
        assert!(first.len() > 1000, "文档应包含全部路由（不应是空文档）");
    }
}

#[cfg(test)]
mod spa_cache_tests {
    use super::*;

    #[tokio::test]
    async fn index_cache_serves_cached_bytes_until_mtime_changes() {
        let dir = std::env::temp_dir().join(format!("oa4rust-spa-test-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let path = dir.join("index.html");
        std::fs::write(&path, "<html>v1</html>").unwrap();

        let cache = SpaIndexCache::new(path.clone());
        let first = cache.read().await.unwrap();
        let second = cache.read().await.unwrap();
        assert_eq!(first, b"<html>v1</html>");
        assert_eq!(second, b"<html>v1</html>", "mtime 不变应命中缓存");

        // 改文件内容（mtime 变化）→ 下次读取必须拿到新内容。
        std::fs::write(&path, "<html>v2</html>").unwrap();
        let third = cache.read().await.unwrap();
        assert_eq!(third, b"<html>v2</html>", "mtime 变化后必须重读磁盘");

        let _ = std::fs::remove_dir_all(&dir);
    }
}

/// R31：/mcp 子应用必须和主路由一样有超时 / panic 收敛 / body 上限。
/// 这里用与 `mcp_app` 相同的 `apply_mcp_request_guards` 组装一组测试路由，
/// 直接验证这三层真的生效。
#[cfg(test)]
mod mcp_guard_tests {
    use super::apply_mcp_request_guards;
    use axum::body::Body;
    use axum::http::{Request, StatusCode};
    use axum::routing::{get, post};
    use axum::Router;
    use std::time::{Duration, Instant};
    use tower::ServiceExt;

    async fn mcp_boom() -> &'static str {
        panic!("mcp panic")
    }

    fn guarded_router(timeout: Option<Duration>, body_limit: usize) -> Router {
        apply_mcp_request_guards(
            Router::new()
                .route(
                    "/mcp/slow",
                    get(|| async {
                        tokio::time::sleep(Duration::from_millis(500)).await;
                        "slow"
                    }),
                )
                .route("/mcp/boom", get(mcp_boom))
                .route("/mcp/ok", get(|| async { "ok" }))
                .route("/mcp/echo", post(|body: String| async move { body })),
            timeout,
            body_limit,
        )
    }

    #[tokio::test]
    async fn mcp_slow_handler_is_cut_off_with_408() {
        let app = guarded_router(Some(Duration::from_millis(50)), 2048);
        let started = Instant::now();
        let resp = app
            .oneshot(
                Request::builder()
                    .uri("/mcp/slow")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(resp.status(), StatusCode::REQUEST_TIMEOUT);
        assert!(
            started.elapsed() < Duration::from_millis(300),
            "超时应在上界附近返回，实际 {:?}",
            started.elapsed()
        );
    }

    #[tokio::test]
    async fn mcp_panic_is_500_envelope_and_service_survives() {
        let app = guarded_router(None, 2048);
        let resp = app
            .clone()
            .oneshot(
                Request::builder()
                    .uri("/mcp/boom")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(resp.status(), StatusCode::INTERNAL_SERVER_ERROR);
        let raw = axum::body::to_bytes(resp.into_body(), 4096).await.unwrap();
        let body: serde_json::Value = serde_json::from_slice(&raw).unwrap();
        assert_eq!(body["type"], "error");
        assert_eq!(body["message"], "internal server error");

        // panic 之后服务仍然可用（不是断连/带崩进程）。
        let resp = app
            .oneshot(
                Request::builder()
                    .uri("/mcp/ok")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn mcp_oversized_body_is_413() {
        let app = guarded_router(None, 16);
        let resp = app
            .oneshot(
                Request::post("/mcp/echo")
                    .header("content-length", "1024")
                    .body(Body::from("x".repeat(1024)))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(resp.status(), StatusCode::PAYLOAD_TOO_LARGE);
    }
}

#[cfg(test)]
mod docs_router_tests {
    use super::docs_router;
    use axum::body::{to_bytes, Body};
    use axum::http::{Request, StatusCode};
    use tower::ServiceExt;
    use utoipa_swagger_ui::{Config, SwaggerUi};

    async fn get(path: &str) -> (StatusCode, String) {
        let resp = docs_router()
            .oneshot(Request::builder().uri(path).body(Body::empty()).unwrap())
            .await
            .unwrap();
        let status = resp.status();
        let bytes = to_bytes(resp.into_body(), 4 * 1024 * 1024).await.unwrap();
        (status, String::from_utf8_lossy(&bytes).into_owned())
    }

    #[tokio::test]
    async fn swagger_ui_page_is_served_and_points_at_the_cached_openapi_endpoint() {
        let (redirect_status, _) = get("/swagger-ui").await;
        assert_eq!(redirect_status, StatusCode::SEE_OTHER, "裸路径应补斜杠");

        let (status, body) = get("/swagger-ui/").await;
        assert_eq!(status, StatusCode::OK);
        assert!(body.contains("swagger-ui"), "页面应含 Swagger UI 挂载点");

        let (css_status, _) = get("/swagger-ui/swagger-ui.css").await;
        assert_eq!(css_status, StatusCode::OK, "vendored 静态资源应随页面可取");

        let (init_status, init) = get("/swagger-ui/swagger-initializer.js").await;
        assert_eq!(init_status, StatusCode::OK);
        assert!(
            init.contains("/openapi.json"),
            "初始化脚本应指向既有的缓存端点，而不是第二份文档路由"
        );
    }

    #[tokio::test]
    async fn openapi_json_endpoint_is_still_served_from_the_once_cache() {
        let (status, body) = get("/openapi.json").await;
        assert_eq!(status, StatusCode::OK);
        assert!(body.contains(r#""paths""#));
        assert!(body.len() > 1000, "文档不应被 Swagger UI 换成空壳");
    }

    #[tokio::test]
    async fn swagger_ui_alone_registers_no_openapi_route() {
        // 反证臂：只用 SwaggerUi 自己那份 Router 时不应出现 /openapi.json。
        // 若这里变成 200，说明 `.url()` 形状被误用，merge 时会与 docs_router 的
        // .route 重叠而 panic。
        let standalone: axum::Router = axum::Router::from(
            SwaggerUi::new("/swagger-ui").config(Config::new(["/openapi.json"])),
        );
        let resp = standalone
            .oneshot(
                Request::builder()
                    .uri("/openapi.json")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(resp.status(), StatusCode::NOT_FOUND);
    }
}
