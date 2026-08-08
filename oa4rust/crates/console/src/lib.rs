use axum::{
    extract::Extension,
    Json, Router, routing::get, routing::post,
};
use deadpool_postgres::Pool;
use serde::Deserialize;
use serde_json::Value;
use uuid::Uuid;

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

pub async fn get_status(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query("SELECT xstatus, xversion, xuptime FROM X.CONSOLE_STATUS WHERE xid = 'system' LIMIT 1", &[])
        .await
        .map_err(|_| AppError::Internal)?;

    let data = if rows.is_empty() {
        Value::Object(serde_json::Map::from_iter([
            ("status".to_string(), Value::String("running".to_string())),
            ("version".to_string(), Value::String("1.0.0".to_string())),
            ("uptime".to_string(), Value::Number(serde_json::Number::from(0))),
        ]))
    } else {
        let row = &rows[0];
        Value::Object(serde_json::Map::from_iter([
            ("status".to_string(), Value::String(row.get("xstatus"))),
            ("version".to_string(), Value::String(row.get("xversion"))),
            ("uptime".to_string(), Value::Number(serde_json::Number::from(row.get::<_, i64>("xuptime")))),
        ]))
    };

    Ok(Json(ActionResult::success(data)))
}

pub async fn get_logs(
    pool: Extension<Pool>,
    axum::extract::Path(log_type): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT xlevel, xmessage, xtimestamp FROM X.CONSOLE_LOG WHERE xtype = $1 ORDER BY xtimestamp DESC LIMIT 100",
            &[&log_type],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("level".to_string(), Value::String(row.get("xlevel"))),
                ("message".to_string(), Value::String(row.get("xmessage"))),
                ("timestamp".to_string(), Value::String(row.get("xtimestamp"))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("type".to_string(), Value::String(log_type)),
            ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
            ("data".to_string(), Value::Array(data)),
        ]),
    ))))
}

pub async fn send_message(
    pool: Extension<Pool>,
    axum::extract::Json(req): Json<SendMessageRequest>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let token = req.token.unwrap_or_default();
    let message = req.message.unwrap_or_default();

    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let id = uuid::Uuid::new_v4().to_string();

    client
        .execute(
            "INSERT INTO X.CONSOLE_MESSAGE (xid, xtoken, xmessage) VALUES ($1, $2, $3)",
            &[&id, &token, &message],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("sent".to_string(), Value::Bool(true)),
            ("token".to_string(), Value::String(token)),
            ("message".to_string(), Value::String(message)),
        ]),
    ))))
}

pub async fn clear_cache(
    pool: Extension<Pool>,
    axum::extract::Path(cache_type): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    client
        .execute("DELETE FROM X.CONSOLE_CACHE WHERE xtype = $1", &[&cache_type])
        .await
        .map_err(|_| AppError::Internal)?;

    let now = chrono::Local::now().to_rfc3339();
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("cleared".to_string(), Value::Bool(true)),
            ("type".to_string(), Value::String(cache_type)),
            ("clearedAt".to_string(), Value::String(now)),
        ]),
    ))))
}

pub async fn get_metric(
    pool: Extension<Pool>,
    axum::extract::Path(metric_name): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT xvalue, xunit FROM X.CONSOLE_METRIC WHERE xname = $1 LIMIT 1",
            &[&metric_name],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let (value, unit) = if rows.is_empty() {
        (42, "count".to_string())
    } else {
        let row = &rows[0];
        (row.get("xvalue"), row.get("xunit"))
    };

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("name".to_string(), Value::String(metric_name)),
            ("value".to_string(), Value::Number(serde_json::Number::from(value))),
            ("unit".to_string(), Value::String(unit)),
        ]),
    ))))
}

pub async fn execute_command(
    pool: Extension<Pool>,
    Json(payload): Json<ExecuteCommandRequest>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let command = payload.command.unwrap_or_default();
    let args = payload.args.unwrap_or_default();

    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let id = uuid::Uuid::new_v4().to_string();

    client
        .execute(
            "INSERT INTO X.CONSOLE_COMMAND_LOG (xid, xcommand, xargs) VALUES ($1, $2, $3)",
            &[&id, &command, &args.join(",")],
        )
        .await
        .map_err(|_| AppError::Internal)?;

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

pub fn router(pool: deadpool_postgres::Pool) -> axum::Router {
    routes::router(pool)
}
