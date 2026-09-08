use axum::{extract::Extension, Json};
use deadpool_postgres::Pool;
use serde::Deserialize;
use serde_json::Value;

use shared::middleware::rbac::is_admin;
use shared::session::Session;
use shared::{error::AppError, response::ActionResult};
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

/// 只读系统命令白名单。每项必须与可执行文件名完全一致，不含路径或参数。
const ALLOWED_COMMANDS: &[&str] = &["uname", "df", "free", "ps", "uptime"];

/// 允许的参数字符：字母、数字、`-`、`_`、`=`、`/`、`.`、`,`。
/// 路径分隔符保留是为了兼容 `df -h /mount` 等合法用法，其余字符一律拒绝。
const fn is_allowed_arg_char(c: char) -> bool {
    c.is_ascii_alphanumeric() || matches!(c, '-' | '_' | '=' | '/' | '.' | ',')
}

/// 单次命令执行的输出上限。
const MAX_OUTPUT_BYTES: usize = 1 << 20; // 1 MiB

/// 仅基于请求字段进行静态校验的纯函数。
///
/// 不依赖数据库、session 或文件系统，可直接在单元测试中运行。
/// 返回 `(program, validated_args)` 供调用方直接用 `Command::new` 执行。
pub fn validate_command_and_args(
    raw_command: &str,
    raw_args: &[String],
) -> Result<(String, Vec<String>), AppError> {
    // command 必须是白名单中的单一 token；空格会被视为注入尝试。
    if raw_command.is_empty() || raw_command.contains(char::is_whitespace) {
        return Err(AppError::BadRequest(
            "command must be a single token without whitespace".to_string(),
        ));
    }
    if !ALLOWED_COMMANDS.contains(&raw_command) {
        return Err(AppError::BadRequest(format!(
            "command '{}' is not allowed. Allowed: {:?}",
            raw_command, ALLOWED_COMMANDS
        )));
    }

    for arg in raw_args {
        if arg.is_empty() {
            return Err(AppError::BadRequest(
                "empty argument is not allowed".to_string(),
            ));
        }
        // 任何非安全字符都拒绝，避免 `;rm -rf /` 这类注入。
        if !arg.chars().all(is_allowed_arg_char) {
            return Err(AppError::BadRequest(format!(
                "argument '{}' contains forbidden characters",
                arg
            )));
        }
    }

    Ok((raw_command.to_string(), raw_args.to_vec()))
}

#[allow(non_snake_case)]
pub async fn get_status(pool: Extension<Pool>) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT xstatus, xversion, xuptime FROM x_console_status WHERE xid = 'system' LIMIT 1",
            &[],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data = if rows.is_empty() {
        Value::Object(serde_json::Map::from_iter([
            ("status".to_string(), Value::String("running".to_string())),
            ("version".to_string(), Value::String("1.0.0".to_string())),
            (
                "uptime".to_string(),
                Value::Number(serde_json::Number::from(0)),
            ),
        ]))
    } else {
        let row = &rows[0];
        Value::Object(serde_json::Map::from_iter([
            ("status".to_string(), Value::String(row.get("xstatus"))),
            ("version".to_string(), Value::String(row.get("xversion"))),
            (
                "uptime".to_string(),
                Value::Number(serde_json::Number::from(row.get::<_, i64>("xuptime"))),
            ),
        ]))
    };

    Ok(Json(ActionResult::success(data)))
}

#[allow(non_snake_case)]
pub async fn get_logs(
    pool: Extension<Pool>,
    axum::extract::Path(log_type): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT xlevel, xmessage, xtimestamp FROM x_console_log WHERE xtype = $1 ORDER BY xtimestamp DESC LIMIT 100",
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
                (
                    "timestamp".to_string(),
                    Value::String(row.get("xtimestamp")),
                ),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("type".to_string(), Value::String(log_type)),
            (
                "count".to_string(),
                Value::Number(serde_json::Number::from(data.len() as i64)),
            ),
            ("data".to_string(), Value::Array(data)),
        ]),
    ))))
}

#[allow(non_snake_case)]
pub async fn send_message(
    pool: Extension<Pool>,
    axum::extract::Json(req): Json<SendMessageRequest>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let token = req.token.unwrap_or_default();
    let message = req.message.unwrap_or_default();

    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let id = uuid::Uuid::new_v4().to_string();

    let result = client
        .execute(
            "INSERT INTO x_console_message (xid, xtoken, xmessage) VALUES ($1, $2, $3)",
            &[&id, &token, &message],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("sent".to_string(), Value::Bool(result > 0)),
            ("token".to_string(), Value::String(token)),
            ("message".to_string(), Value::String(message)),
        ]),
    ))))
}

#[allow(non_snake_case)]
pub async fn clear_cache(
    pool: Extension<Pool>,
    axum::extract::Path(cache_type): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let result = client
        .execute(
            "DELETE FROM x_console_cache WHERE xtype = $1",
            &[&cache_type],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let now = chrono::Local::now().to_rfc3339();
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("cleared".to_string(), Value::Bool(result > 0)),
            ("type".to_string(), Value::String(cache_type)),
            ("clearedAt".to_string(), Value::String(now)),
        ]),
    ))))
}

#[allow(non_snake_case)]
pub async fn get_metric(
    pool: Extension<Pool>,
    axum::extract::Path(metric_name): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT xvalue, xunit FROM x_console_metric WHERE xname = $1 LIMIT 1",
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
            (
                "value".to_string(),
                Value::Number(serde_json::Number::from(value)),
            ),
            ("unit".to_string(), Value::String(unit)),
        ]),
    ))))
}

#[allow(non_snake_case)]
pub async fn execute_command(
    pool: Extension<Pool>,
    session: Extension<Session>,
    Json(payload): Json<ExecuteCommandRequest>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let command = payload
        .command
        .as_deref()
        .ok_or(AppError::BadRequest("command is required".to_string()))?;
    let args = payload.args.unwrap_or_default();

    // RBAC: 仅 Admin 可执行命令
    if !is_admin(pool.deref(), &session.person_unique).await {
        return Err(AppError::Forbidden);
    }

    // 纯校验阶段：不依赖数据库、session 或文件系统。
    let (program, validated_args) = validate_command_and_args(command, &args)?;

    // 执行命令。白名单内的系统命令（uname/df/free/ps/uptime）毫秒级完成，
    // 同步阻塞 Tokio worker 可接受；后续如需真正异步化再单独重构。
    let output = std::process::Command::new(&program)
        .args(&validated_args)
        .output()
        .map_err(|e| AppError::BadRequest(format!("failed to execute command: {}", e)))?;

    let stdout =
        String::from_utf8_lossy(&output.stdout[..output.stdout.len().min(MAX_OUTPUT_BYTES)])
            .to_string();
    let stderr =
        String::from_utf8_lossy(&output.stderr[..output.stderr.len().min(MAX_OUTPUT_BYTES)])
            .to_string();
    let exit_code = output.status.code().unwrap_or(-1);

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("command".to_string(), Value::String(program)),
            (
                "args".to_string(),
                Value::Array(validated_args.into_iter().map(Value::String).collect()),
            ),
            ("output".to_string(), Value::String(stdout)),
            ("stderr".to_string(), Value::String(stderr)),
            (
                "exitCode".to_string(),
                Value::Number(serde_json::Number::from(exit_code)),
            ),
        ]),
    ))))
}

#[allow(non_snake_case)]
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
            (
                "cpuCores".to_string(),
                Value::Number(serde_json::Number::from(cpu_cores)),
            ),
            ("memory".to_string(), Value::String(mem_str)),
            ("disk".to_string(), Value::String(disk_str)),
        ]),
    ))))
}

pub fn router(pool: deadpool_postgres::Pool) -> axum::Router {
    routes::router(pool)
}

#[cfg(test)]
mod tests_generated;

#[cfg(test)]
mod validation_tests {
    use super::*;

    #[test]
    fn allow_valid_command_with_safe_args() {
        let (prog, args) = validate_command_and_args("uname", &["-a".into()]).unwrap();
        assert_eq!(prog, "uname");
        assert_eq!(args, vec!["-a"]);
    }

    #[test]
    fn allow_command_without_args() {
        let (prog, args) = validate_command_and_args("ps", &[]).unwrap();
        assert_eq!(prog, "ps");
        assert!(args.is_empty());
    }

    #[test]
    fn reject_whitespace_in_command() {
        let err = validate_command_and_args("uname -a", &[]).unwrap_err();
        assert_eq!(
            err.to_string(),
            "bad request: command must be a single token without whitespace"
        );
    }

    #[test]
    fn reject_unknown_command() {
        let err = validate_command_and_args("rm", &[]).unwrap_err();
        assert!(err.to_string().contains("not allowed"));
    }

    #[test]
    fn reject_shell_metacharacters_in_args() {
        for payload in [
            ";rm",
            "|cat",
            "&ls",
            "`id`",
            "$(whoami)",
            "$HOME",
            "foo;bar",
            "a|b",
        ] {
            let err = validate_command_and_args("df", &[payload.into()]).unwrap_err();
            assert!(
                err.to_string().contains("forbidden characters"),
                "payload '{}' should be rejected but got: {}",
                payload,
                err
            );
        }
    }

    #[test]
    fn accept_path_like_args() {
        // Forward-slash paths like /tmp, /etc/passwd are valid for df/ps.
        let (_prog, args) =
            validate_command_and_args("df", &["/etc/passwd".into(), "/tmp".into()]).unwrap();
        assert_eq!(args, vec!["/etc/passwd", "/tmp"]);
    }

    #[test]
    fn accept_forward_slash_path_args() {
        let (_prog, args) = validate_command_and_args("df", &["/tmp".into(), "-h".into()]).unwrap();
        assert_eq!(args, vec!["/tmp", "-h"]);
    }
}
