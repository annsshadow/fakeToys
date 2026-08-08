use std::env;
use std::sync::Arc;

use anyhow::Context as _;
use axum::middleware;
use axum::Router;
use mcp_server::tool_bridge::ToolBridge;
use shared::db::create_pool;
use shared::middleware::{
    auth_middleware, authorize_middleware, cors_middleware, rate_limit_middleware, security_headers_middleware,
    trace_middleware, SecurityState,
};
use shared::rate_limit::RateLimiter;
use shared::Pool;
use shared::session::SessionManager;
use tracing_subscriber::EnvFilter;
use express;
use message;
use openapi::ApiDoc;
use portal;
use bbs;
use calendar;
use component;
use file;
use ai;
use attendance;
use correlation;
use general;
use hotpic;
use jpush;
use meeting;
use mind;
use cms_express;
use cms_assemble_control;
use process_express;
use query_express;
use process_designer;
use program_center;
use base;
use query_service;
use process_bam;
use process_surface;
use file_assemble_control;
use ai_assemble_control;
use hotpic_assemble_control;
use organization_assemble_express;
use organization_assemble_control;
use mind_assemble_control;
use attendance_assemble_control;
use general_assemble_control;
use meeting_assemble_control;
use message_assemble_communicate;
use portal_assemble_designer;
use correlation_service_processing;
use portal_assemble_surface;
use processplatform_service_processing;
use bbs_assemble_control;
use calendar_assemble_control;
use component_assemble_control;
use jpush_assemble_control;
use processplatform_core_entity;
use portal_core_entity;
use program_center_core_entity;
use processplatform_core_express;
use query_core_entity;
use general_core_entity;
use organization_core_entity;
use cms_core_entity;
use query_assemble_designer;
use query_assemble_surface;
use console;
use processplatform_assemble_surface;
use bbs_core_entity;
use calendar_core_entity;
use component_core_entity;
use file_core_entity;
use ai_core_entity;
use attendance_core_entity;
use cms_core_express;
use correlation_core_entity;
use correlation_core_express;
use hotpic_core_entity;
use jpush_core_entity;
use meeting_core_entity;
use message_core_entity;
use mind_core_entity;
use organization_core_express;
use processplatform_assemble_bam;
use processplatform_assemble_designer;
use query_core_express;
use query_service_processing;

/// OpenAPI JSON endpoint handler.
async fn openapi_json_handler() -> Result<Vec<u8>, axum::response::Json<serde_json::Value>> {
    use utoipa::OpenApi;
    let json = ApiDoc::openapi().to_json().map_err(|e| axum::Json(serde_json::json!({"error": e.to_string()})))?;
    Ok(json.into_bytes())
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env().add_directive("o2server=debug".parse()?))
        .init();

    dotenvy::dotenv().ok();

    let args: Vec<String> = env::args().collect();
    let http_flag = args.iter().any(|a| a == "--http");

    let pool = create_pool().await.context("failed to create database pool")?;

    let session_manager = SessionManager::with_pool(pool.clone());
    let rate_limiter = RateLimiter::new();

    let app = create_app(pool.clone(), session_manager.clone(), rate_limiter.clone()).await?;

    // Mount OpenAPI JSON and Swagger UI before other layers
    let app = app
        .route("/openapi.json", axum::routing::get(openapi_json_handler));

    // Optionally mount the MCP HTTP endpoint at /mcp when --http flag is present.
    let security_state = shared::middleware::SecurityState {
        session_manager: session_manager.clone(),
        rate_limiter: rate_limiter.clone(),
        pool: pool.clone(),
    };
    let app = if http_flag {
        let bridge = Arc::new(ToolBridge::new(pool, session_manager));
        app.merge(mcp_app(bridge, security_state))
    } else {
        app
    };

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
    use axum::Json;

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
                let tool_call: ToolCallParams =
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
                    Ok(resp) => {
                        Ok(serde_json::to_value(resp)
                            .unwrap_or(serde_json::json!({"content": []})))
                    }
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

    Router::new().route(
        "/mcp",
        post(mcp_handler).with_state(Arc::clone(&bridge)),
    )
    .layer(middleware::from_fn_with_state(security_state.clone(), authorize_middleware))
    .layer(middleware::from_fn_with_state(security_state.clone(), auth_middleware))
    .layer(middleware::from_fn_with_state(security_state.clone(), rate_limit_middleware))
    .layer(middleware::from_fn(security_headers_middleware))
    .layer(middleware::from_fn(trace_middleware))
}

/// 构建完整应用 Router（供集成测试使用）。
pub async fn create_app(
    pool: Pool,
    session_manager: SessionManager,
    rate_limiter: RateLimiter,
) -> anyhow::Result<Router> {
    let security_state = SecurityState {
        session_manager: session_manager.clone(),
        rate_limiter: rate_limiter.clone(),
        pool: pool.clone(),
    };

    let app = Router::new()
        .merge(shared::router::router())
        .merge(auth::router(pool.clone(), rate_limiter.clone(), session_manager.clone()))
        .merge(personal::router(pool.clone(), session_manager.clone()))
        .merge(cms_control::cms_control_router(pool.clone()))
        .merge(control::control_router(pool.clone()))
        .merge(personal_extend::personal_extend_router(pool.clone(), session_manager))
        .merge(program_init::program_init_router(pool.clone()))
        .merge(express::router(pool.clone()))
        .merge(message::router(pool.clone()))
        .merge(portal::router(pool.clone()))
        .merge(bbs::router(pool.clone()))
        .merge(calendar::router(pool.clone()))
        .merge(component::router(pool.clone()))
        .merge(file::router(pool.clone()))
        .merge(ai::router(pool.clone()))
        .merge(attendance::router(pool.clone()))
        .merge(correlation::router(pool.clone()))
        .merge(general::router(pool.clone()))
        .merge(hotpic::router(pool.clone()))
        .merge(jpush::router(pool.clone()))
        .merge(meeting::router(pool.clone()))
        .merge(mind::router(pool.clone()))
        .merge(cms_express::router(pool.clone()))
        .merge(cms_assemble_control::router(pool.clone()))
        .merge(process_express::router(pool.clone()))
        .merge(query_express::router(pool.clone()))
        .merge(process_designer::router(pool.clone()))
        .merge(program_center::router(pool.clone()))
        .merge(base::router(pool.clone()))
        .merge(query_service::router(pool.clone()))
        .merge(process_bam::router(pool.clone()))
        .merge(process_surface::router(pool.clone()))
        .merge(file_assemble_control::router(pool.clone()))
        .merge(ai_assemble_control::router(pool.clone()))
        .merge(hotpic_assemble_control::router(pool.clone()))
        .merge(organization_assemble_express::router(pool.clone()))
        .merge(organization_assemble_control::router(pool.clone()))
        .merge(mind_assemble_control::router(pool.clone()))
        .merge(attendance_assemble_control::router(pool.clone()))
        .merge(general_assemble_control::router(pool.clone()))
        .merge(meeting_assemble_control::router(pool.clone()))
        .merge(message_assemble_communicate::router(pool.clone()))
        .merge(portal_assemble_designer::router(pool.clone()))
        .merge(correlation_service_processing::router(pool.clone()))
        .merge(portal_assemble_surface::router(pool.clone()))
        .merge(processplatform_service_processing::router(pool.clone()))
        .merge(bbs_assemble_control::router(pool.clone()))
        .merge(calendar_assemble_control::router(pool.clone()))
        .merge(component_assemble_control::router(pool.clone()))
        .merge(jpush_assemble_control::router(pool.clone()))
        .merge(processplatform_core_entity::router(pool.clone()))
        .merge(portal_core_entity::router(pool.clone()))
        .merge(program_center_core_entity::router(pool.clone()))
        .merge(processplatform_core_express::router(pool.clone()))
        .merge(query_core_entity::router(pool.clone()))
        .merge(general_core_entity::router(pool.clone()))
        .merge(organization_core_entity::router(pool.clone()))
        .merge(cms_core_entity::router(pool.clone()))
        .merge(query_assemble_designer::router(pool.clone()))
        .merge(query_assemble_surface::router(pool.clone()))
        .merge(console::router(pool.clone()))
        .merge(processplatform_assemble_surface::router(pool.clone()))
        .merge(bbs_core_entity::router(pool.clone()))
        .merge(calendar_core_entity::router(pool.clone()))
        .merge(component_core_entity::router(pool.clone()))
        .merge(file_core_entity::router(pool.clone()))
        .merge(ai_core_entity::router(pool.clone()))
        .merge(attendance_core_entity::router(pool.clone()))
        .merge(cms_core_express::router(pool.clone()))
        .merge(correlation_core_entity::router(pool.clone()))
        .merge(correlation_core_express::router(pool.clone()))
        .merge(hotpic_core_entity::router(pool.clone()))
        .merge(jpush_core_entity::router(pool.clone()))
        .merge(meeting_core_entity::router(pool.clone()))
        .merge(message_core_entity::router(pool.clone()))
        .merge(mind_core_entity::router(pool.clone()))
        .merge(organization_core_express::router(pool.clone()))
        .merge(processplatform_assemble_bam::router(pool.clone()))
        .merge(processplatform_assemble_designer::router(pool.clone()))
        .merge(query_core_express::router(pool.clone()))
        .merge(query_service_processing::router(pool.clone()));

    let app = app
        .layer(middleware::from_fn_with_state(security_state.clone(), authorize_middleware))
        .layer(middleware::from_fn_with_state(security_state.clone(), auth_middleware))
        .layer(middleware::from_fn_with_state(security_state.clone(), rate_limit_middleware))
        .layer(cors_middleware())
        .layer(middleware::from_fn(security_headers_middleware))
        .layer(middleware::from_fn(trace_middleware));

    Ok(app)
}
