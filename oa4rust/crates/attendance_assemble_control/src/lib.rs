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
pub struct ControlRule {
    pub id: String,
    pub rule_name: String,
    pub rule_type: String,
    pub enabled: bool,
    pub description: Option<String>,
}

pub async fn list_control_rules(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query(
            "SELECT id, rule_name, rule_type, enabled, description FROM x_attendance_assemble_control_rule ORDER BY create_time",
            &[],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("ruleName".to_string(), Value::String(row.get("rule_name"))),
                ("ruleType".to_string(), Value::String(row.get("rule_type"))),
                ("enabled".to_string(), Value::Bool(row.get("enabled"))),
                ("description".to_string(), Value::String(row.get::<_, Option<String>>("description").unwrap_or_default())),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

pub async fn toggle_control_rule(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
    axum::extract::Json(payload): axum::extract::Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let enabled: bool = payload.get("enabled").and_then(|v| v.as_bool()).unwrap_or(false);

    client
        .execute(
            "UPDATE x_attendance_assemble_control_rule SET enabled = $1 WHERE id = $2",
            &[&enabled, &id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("id".to_string(), Value::String(id)),
        ("enabled".to_string(), Value::Bool(enabled)),
        ("updated".to_string(), Value::Bool(true)),
    ])))))
}

pub fn attendance_assemble_control_router(pool: Pool) -> Router {
    routes::attendance_assemble_control_routes(pool)
}

pub fn router(_pool: deadpool_postgres::Pool) -> axum::Router {
    axum::Router::new()
        .route("/attendance_assemble_control/health", axum::routing::get(|| async { "TODO: attendance_assemble_control - real implementation needed" }))
}