use axum::{Json, Router, routing::get, routing::post};
use serde::Deserialize;
use serde_json::Value;
use shared::{error::AppError, response::ActionResult};

pub mod routes;

#[derive(Debug, Deserialize)]
pub struct SendMessageRequest {
    pub token: Option<String>,
    pub message: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ExecuteCommandRequest {
    pub command: Option<String>,
    pub args: Option<Vec<String>>,
}

pub async fn get_status() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("status".to_string(), Value::String("running".to_string())),
            ("version".to_string(), Value::String("1.0.0".to_string())),
            ("uptime".to_string(), Value::Number(serde_json::Number::from(0))),
        ]),
    ))))
}

pub async fn get_logs(
    axum::extract::Path(log_type): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let data = vec![
        Value::Object(serde_json::Map::from_iter([
            ("level".to_string(), Value::String("info".to_string())),
            ("message".to_string(), Value::String("Log entry 1".to_string())),
            ("timestamp".to_string(), Value::String("2024-01-01T00:00:00Z".to_string())),
        ])),
    ];

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("type".to_string(), Value::String(log_type)),
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

pub async fn send_message(
    axum::extract::Json(req): Json<SendMessageRequest>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("sent".to_string(), Value::Bool(true)),
            ("token".to_string(), Value::String(req.token.unwrap_or_default())),
            ("message".to_string(), Value::String(req.message.unwrap_or_default())),
        ]),
    ))))
}

pub async fn clear_cache(
    axum::extract::Path(cache_type): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("cleared".to_string(), Value::Bool(true)),
            ("type".to_string(), Value::String(cache_type)),
            ("clearedAt".to_string(), Value::String("2024-01-01T00:00:00Z".to_string())),
        ]),
    ))))
}

pub async fn get_metric(
    axum::extract::Path(metric_name): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("name".to_string(), Value::String(metric_name)),
            ("value".to_string(), Value::Number(serde_json::Number::from(42))),
            ("unit".to_string(), Value::String("count".to_string())),
        ]),
    ))))
}

pub async fn execute_command(
    Json(payload): Json<ExecuteCommandRequest>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let command = payload.command.unwrap_or_default();
    let args = payload.args.unwrap_or_default();

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("command".to_string(), Value::String(command)),
            ("args".to_string(), Value::Array(args.into_iter().map(Value::String).collect())),
            ("output".to_string(), Value::String("command executed".to_string())),
            ("exitCode".to_string(), Value::Number(serde_json::Number::from(0))),
        ]),
    ))))
}

pub async fn get_system_info() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("os".to_string(), Value::String("linux".to_string())),
            ("arch".to_string(), Value::String("x86_64".to_string())),
            ("cpuCores".to_string(), Value::Number(serde_json::Number::from(4))),
            ("memory".to_string(), Value::String("8GB".to_string())),
            ("disk".to_string(), Value::String("256GB".to_string())),
        ]),
    ))))
}

pub fn console_router() -> Router {
    Router::new()
        .route("/jaxrs/console/status", get(get_status))
        .route("/jaxrs/console/logs/{type}", get(get_logs))
        .route("/jaxrs/console/send/message", post(send_message))
        .route("/jaxrs/console/cache/clear/{type}", post(clear_cache))
        .route("/jaxrs/console/metric/{name}", get(get_metric))
        .route("/jaxrs/console/command/execute", post(execute_command))
        .route("/jaxrs/console/system/info", get(get_system_info))
}

#[cfg(test)]
mod tests;

pub fn router(pool: deadpool_postgres::Pool) -> axum::Router {
    console_router().layer(axum::extract::Extension(pool))
}
