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
use shared::rate_limit::RateLimiter;
use shared::session::SessionManager;
use tracing_subscriber::EnvFilter;

/// OpenAPI JSON endpoint handler.
async fn openapi_json_handler() -> Result<Vec<u8>, axum::response::Json<serde_json::Value>> {
    use utoipa::OpenApi;
    let json = ApiDoc::openapi()
        .to_json()
        .map_err(|e| axum::Json(serde_json::json!({"error": e.to_string()})))?;
    Ok(json.into_bytes())
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env().add_directive("o2server=debug".parse()?))
        .init();

    dotenvy::dotenv().ok();

    let auth_config = shared::config::AuthConfig::from_env()
        .context("invalid browser authentication configuration")?;

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

    // Phase B-U-B2: Redis 为默认 session 存储，不可达时降级为内存+DB 模式
    let redis_available = session_manager.init_redis() && rate_limiter.init_redis();
    if redis_available {
        tracing::info!("Redis backend initialized for session store and rate limiter");
    } else {
        tracing::warn!(
            "Redis unreachable; session store and rate limiter using in-memory fallback"
        );
    }

    // plan002 U7b: LDAP_SYNC_ENABLE=true 时启动 LDAP 用户自动同步定时 worker（幂等）
    if ldap::sync::init_from_env(pool.clone()) {
        tracing::info!("LDAP user sync worker started (LDAP_SYNC_ENABLE=true)");
    }

    let app =
        oa4rust::create_app(pool.clone(), session_manager.clone(), rate_limiter.clone()).await?;

    // Mount OpenAPI JSON and Swagger UI before other layers
    let app = app.route("/openapi.json", axum::routing::get(openapi_json_handler));

    // Optionally mount the MCP HTTP endpoint at /mcp when --http flag is present.
    let security_state = shared::middleware::SecurityState {
        session_manager: session_manager.clone(),
        rate_limiter: rate_limiter.clone(),
        pool: pool.clone(),
    };
    let app = if http_flag {
        let bridge = Arc::new(ToolBridge::new(pool, session_manager).await);
        app.merge(mcp_app(bridge, security_state))
    } else {
        app
    };

    // ── 静态文件服务（前端构建产物）─────────────────────────────────────
    // OA4RUST_WEB_DIST 环境变量指定 dist 目录，默认相对于二进制位置向上两级再进 dist/web。
    // SPA 深链回退：/login、/app/** 等前端路由刷新时回 index.html；/api/** 与
    // /ws/** 未知路径保持 404（API 客户端不应收到 HTML）。
    let web_dist = env::var("OA4RUST_WEB_DIST").unwrap_or_else(|_| "../../dist/web".to_string());
    let web_dist_root = std::path::PathBuf::from(&web_dist);
    let spa = tower::service_fn(move |req: axum::http::Request<axum::body::Body>| {
        let web_dist_root = web_dist_root.clone();
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
                return Ok(axum::http::Response::builder()
                    .header(axum::http::header::CONTENT_TYPE, mime)
                    .body(axum::body::Body::from(bytes))
                    .unwrap());
            }
            match tokio::fs::read(web_dist_root.join("index.html")).await {
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

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    tracing::info!("listening on {}", listener.local_addr()?);
    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<std::net::SocketAddr>(),
    )
    .await?;

    Ok(())
}

/// Build the MCP HTTP sub-application mounted at /mcp.
/// Forwards the caller's Authorization header to the internal ToolBridge so
/// existing session-based auth is reused without duplicating business logic.
fn mcp_app(bridge: Arc<ToolBridge>, security_state: shared::middleware::SecurityState) -> Router {
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
        .layer(middleware::from_fn(trace_middleware))
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
