// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

use std::env;
use std::sync::Arc;

use anyhow::{Context as _, Result};
use mcp_server::tool_bridge::{run_stdio, ToolBridge};
use shared::db::create_pool;
use shared::session::SessionManager;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args: Vec<String> = env::args().collect();

    // Parse transport mode from args.
    //   cargo run --bin mcp_server -- stdio    → stdio mode (default)
    //   cargo run --bin mcp_server -- http 3001 → HTTP mode on port 3001
    let (mode, port) = parse_args(&args);

    dotenvy::dotenv().ok();

    // Always create a real pool so internal tool calls can hit the database.
    // Falls back to a mock pool when DATABASE_URL is not set (e.g., stdio-only
    // discovery scenarios).
    let pool = match create_pool().await {
        Ok(p) => p,
        Err(_) => shared::testing::mock_pool(),
    };

    let session_manager = SessionManager::with_pool(pool.clone());

    let bridge = Arc::new(ToolBridge::new(pool, session_manager).await);

    match mode {
        "http" => {
            // 2026-10 安全审计：默认只绑定回环，避免 0.0.0.0 无认证暴露
            // ToolBridge（直通 DB 的后端工具调用）；确需远端访问时通过
            // MCP_BIND_ADDR 显式配置，并务必同时设置 MCP_API_KEY 门禁。
            let bind_addr = std::env::var("MCP_BIND_ADDR").unwrap_or_else(|_| "127.0.0.1".to_string());
            let api_key = std::env::var("MCP_API_KEY").ok();
            if api_key.is_none()
                && !matches!(
                    bind_addr.as_str(),
                    "127.0.0.1" | "localhost" | "::1"
                )
            {
                tracing::warn!(
                    "MCP server binds to non-loopback address {bind_addr} without MCP_API_KEY: \
                     tool calls are UNAUTHENTICATED. Set MCP_API_KEY to require an X-API-Key header."
                );
            }
            let listen_port = port.unwrap_or(3001);
            let listener = TcpListener::bind(format!("{bind_addr}:{listen_port}"))
                .await
                .with_context(|| {
                    format!(
                        "failed to bind MCP HTTP listener on {bind_addr}:{listen_port}"
                    )
                })?;
            tracing::info!(
                "MCP server listening on http://{bind_addr}:{listen_port} (api_key={})",
                if api_key.is_some() { "required" } else { "open" }
            );

            let mcp_router = axum::Router::new()
                .route(
                    "/mcp",
                    axum::routing::post(mcp_http_handler).with_state(Arc::clone(&bridge)),
                )
                .route(
                    "/mcp",
                    axum::routing::get(|| async {
                        axum::Json(serde_json::json!({"ok": true, "service": "oa4rust-mcp"}))
                    }),
                );

            axum::serve(
                listener,
                mcp_router.into_make_service_with_connect_info::<std::net::SocketAddr>(),
            )
            .await?;
        }
        _ => {
            tracing::info!("MCP server running in stdio mode");
            run_stdio(bridge).await?;
        }
    }

    Ok(())
}

/// Parse CLI arguments.
/// Returns (mode, port) where mode is "stdio" or "http".
fn parse_args(args: &[String]) -> (&str, Option<u16>) {
    let mut mode = "stdio";
    let mut port = None;

    for (i, arg) in args.iter().enumerate() {
        if arg == "--http" {
            mode = "http";
            if i + 1 < args.len() {
                if let Ok(p) = args[i + 1].parse::<u16>() {
                    port = Some(p);
                }
            }
        }
    }

    (mode, port)
}

/// 常量时间字符串比较，避免 API key 校验的时序侧信道。
fn ct_eq(a: &str, b: &str) -> bool {
    if a.len() != b.len() {
        return false;
    }
    let mut diff: u8 = 0;
    for (x, y) in a.bytes().zip(b.bytes()) {
        diff |= x ^ y;
    }
    diff == 0
}

/// 配置了 MCP_API_KEY 时，/mcp 请求必须携带匹配的 X-API-Key，否则 401。
/// key 只在进程启动后首次调用时读取并缓存（OnceLock）。
fn configured_api_key() -> Option<&'static str> {
    static KEY: std::sync::OnceLock<Option<String>> = std::sync::OnceLock::new();
    KEY.get_or_init(|| std::env::var("MCP_API_KEY").ok())
        .as_deref()
}

/// JSON-RPC handler for HTTP transport.
/// The `bridge` state carries the ToolBridge (shared via Arc).
async fn mcp_http_handler(
    axum::extract::State(bridge): axum::extract::State<Arc<ToolBridge>>,
    headers: axum::http::HeaderMap,
    axum::Json(req): axum::Json<serde_json::Value>,
) -> Result<axum::Json<serde_json::Value>, axum::http::StatusCode> {
    use mcp_server::tool_bridge::JsonRpcResponse;

    // 2026-10 安全审计：MCP 工具可直通后端 DB，配置 MCP_API_KEY 后强制鉴权。
    if let Some(key) = configured_api_key() {
        let provided = headers
            .get("x-api-key")
            .and_then(|v| v.to_str().ok())
            .unwrap_or("");
        if !ct_eq(provided, key) {
            return Err(axum::http::StatusCode::UNAUTHORIZED);
        }
    }

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
            let tool_call: mcp_server::tool_bridge::ToolCallParams =
                match serde_json::from_value(params) {
                    Ok(p) => p,
                    Err(e) => {
                        return Ok(
                            axum::Json(
                                JsonRpcResponse::err(id, -32600, format!("invalid params: {}", e))
                                    .into_json_value(),
                            ),
                        );
                    }
                };
            match bridge.call_tool(tool_call).await {
                Ok(resp) => Ok(serde_json::to_value(resp).unwrap_or(serde_json::json!({}))),
                Err(e) => {
                    return Ok(
                        axum::Json(
                            JsonRpcResponse::err(id, e.code, e.message).into_json_value(),
                        ),
                    );
                }
            }
        }
        _ => {
            return Ok(
                axum::Json(
                    JsonRpcResponse::err(id, -32601, format!("method not found: {}", method))
                        .into_json_value(),
                ),
            );
        }
    };

    match result {
        Ok(r) => Ok(axum::Json(JsonRpcResponse::ok(id, r).into_json_value())),
        Err(e) => Ok(axum::Json(JsonRpcResponse::err(id, e.code, e.message).into_json_value())),
    }
}
