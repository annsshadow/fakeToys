use axum::{
    extract::Extension,
    Json, Router,
};
use deadpool_postgres::Pool;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use shared::{error::AppError, response::ActionResult};

pub mod routes;

#[cfg(test)]
mod tests;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MeetingControl {
    pub id: String,
    pub meeting_id: String,
    pub control_type: String,
    pub enabled: bool,
    pub config: Option<String>,
}

pub async fn list_meeting_controls(
    pool: Extension<Pool>,
    axum::extract::Path(meeting_id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query(
            "SELECT id, meeting_id, control_type, enabled, config FROM x_meeting_assemble_control WHERE meeting_id = $1 ORDER BY create_time",
            &[&meeting_id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("meetingId".to_string(), Value::String(row.get("meeting_id"))),
                ("controlType".to_string(), Value::String(row.get("control_type"))),
                ("enabled".to_string(), Value::Bool(row.get("enabled"))),
                ("config".to_string(), Value::String(row.get::<_, Option<String>>("config").unwrap_or_default())),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("meetingId".to_string(), Value::String(meeting_id)),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

pub async fn create_meeting_control(
    pool: Extension<Pool>,
    axum::extract::Json(payload): axum::extract::Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let meeting_id = payload.get("meetingId").and_then(|v| v.as_str()).ok_or(AppError::BadRequest("meetingId is required".to_string()))?;
    let control_type = payload.get("controlType").and_then(|v| v.as_str()).ok_or(AppError::BadRequest("controlType is required".to_string()))?;
    let enabled: bool = payload.get("enabled").and_then(|v| v.as_bool()).unwrap_or(true);
    let config = payload.get("config").and_then(|v| v.as_str()).map(|s| s.to_string());

    let id: String = uuid::Uuid::new_v4().to_string();

    client
        .execute(
            "INSERT INTO x_meeting_assemble_control (id, meeting_id, control_type, enabled, config) VALUES ($1, $2, $3, $4, $5)",
            &[&id, &meeting_id, &control_type, &enabled, &config],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("id".to_string(), Value::String(id)),
        ("meetingId".to_string(), Value::String(meeting_id.to_string())),
        ("controlType".to_string(), Value::String(control_type.to_string())),
        ("enabled".to_string(), Value::Bool(enabled)),
    ])))))
}

pub async fn delete_meeting_control(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let count = client
        .execute(
            "DELETE FROM x_meeting_assemble_control WHERE id = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("id".to_string(), Value::String(id)),
        ("deleted".to_string(), Value::Bool(count > 0)),
    ])))))
}

pub fn meeting_assemble_control_router(pool: Pool) -> Router {
    routes::meeting_assemble_control_routes(pool)
}

pub fn router(_pool: deadpool_postgres::Pool) -> axum::Router {
    axum::Router::new()
        .route("/meeting_assemble_control/health", axum::routing::get(|| async { "TODO: meeting_assemble_control - real implementation needed" }))
}