use axum::{
    extract::Extension,
    Json, Router, routing::get, routing::post,
};
use deadpool_postgres::Pool;
use serde::Deserialize;
use serde_json::Value;
use uuid::Uuid;

use shared::{error::AppError, response::ActionResult};
use shared::session::Session;
use shared::middleware::rbac::is_admin;
use std::ops::Deref;
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

/// 只读系统命令白名单
const ALLOWED_COMMANDS: &[&str] = &["uname", "df", "free", "ps", "uptime"];

/// shell 元字符黑名单
const FORBIDDEN_CHARS: &[char] = &[';', '|', '&', '`', '$', '(', ')'];

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
    session: Extension<Session>,
    Json(payload): Json<ExecuteCommandRequest>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let command = payload.command.as_deref().ok_or(AppError::BadRequest("command is required".to_string()))?;
    let args = payload.args.unwrap_or_default();

    // RBAC: 仅 Admin 可执行命令
    // pool 是 Extension<Pool>，可以通过 Deref 转换为 &Pool
    if !is_admin(pool.deref(), &session.person_unique).await {
        return Err(AppError::Forbidden);
    }

    // 检查命令是否在白名单中
    let base_cmd = command.split_whitespace().next().unwrap_or(command);
    if !ALLOWED_COMMANDS.contains(&base_cmd) {
        return Err(AppError::BadRequest(format!(
            "command '{}' is not allowed. Allowed: {:?}",
            base_cmd, ALLOWED_COMMANDS
        )));
    }

    // 检查 shell 元字符（禁止注入）
    let full_input = format!("{} {}", command, args.join(" "));
    if full_input.chars().any(|c| FORBIDDEN_CHARS.contains(&c)) {
        return Err(AppError::BadRequest("forbidden shell metacharacters detected".to_string()));
    }

    // 执行命令（同步，非 async）
    let output = std::process::Command::new("sh")
        .arg("-c")
        .arg(&full_input)
        .output()
        .map_err(|e| AppError::BadRequest(format!("failed to execute command: {}", e)))?;

    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).to_string();
    let exit_code = output.status.code().unwrap_or(-1);

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("command".to_string(), Value::String(command.to_string())),
            ("args".to_string(), Value::Array(args.into_iter().map(Value::String).collect())),
            ("output".to_string(), Value::String(stdout)),
            ("stderr".to_string(), Value::String(stderr)),
            ("exitCode".to_string(), Value::Number(serde_json::Number::from(exit_code))),
        ]),
    ))))
}

pub async fn get_system_info() -> Result<Json<ActionResult<Value>>, AppError> {
    let mut sys = sysinfo::System::new_all();
    sys.refresh_all();

    let os_name = std::env::consts::OS;
    let arch = std::env::consts::ARCH;
    let cpu_cores = sys.cpus().len() as i64;

    let total_mem = sys.total_memory();
    let mem_gb = total_mem as f64 / (1024.0 * 1024.0 * 1024.0);
    let mem_str = format!("{:.1}GB", mem_gb);

    // 获取磁盘大小（sysinfo 0.33 无 disks() API，使用占位值）
    let disk_str = "unknown".to_string();

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("os".to_string(), Value::String(os_name.to_string())),
            ("arch".to_string(), Value::String(arch.to_string())),
            ("cpuCores".to_string(), Value::Number(serde_json::Number::from(cpu_cores))),
            ("memory".to_string(), Value::String(mem_str)),
            ("disk".to_string(), Value::String(disk_str)),
        ]),
    ))))
}

pub fn router(pool: deadpool_postgres::Pool) -> axum::Router {
    routes::router(pool)
}
