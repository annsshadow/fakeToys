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

    let bridge = Arc::new(ToolBridge::new(pool, session_manager));

    match mode {
        "http" => {
            let listen_port = port.unwrap_or(3001);
            let listener = TcpListener::bind(format!("0.0.0.0:{}", listen_port))
                .await
                .with_context(|| format!("failed to bind MCP HTTP listener on {}", listen_port))?;
            tracing::info!("MCP server listening on http://0.0.0.0:{}", listen_port);

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

/// JSON-RPC handler for HTTP transport.
/// The `bridge` state carries the ToolBridge (shared via Arc).
async fn mcp_http_handler(
    axum::extract::State(bridge): axum::extract::State<Arc<ToolBridge>>,
    axum::Json(req): axum::Json<serde_json::Value>,
) -> axum::Json<serde_json::Value> {
    use mcp_server::tool_bridge::{JsonRpcRequest, JsonRpcResponse};

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
                        return axum::Json(JsonRpcResponse::err(
                            id,
                            -32600,
                            format!("invalid params: {}", e),
                        )
                        .into_json_value());
                    }
                };
            match bridge.call_tool(tool_call).await {
                Ok(resp) => Ok(serde_json::to_value(resp).unwrap_or(serde_json::json!({}))),
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
