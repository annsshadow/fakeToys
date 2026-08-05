use axum::{
    extract::Extension,
    Json, Router,
};
use deadpool_postgres::Pool;
use serde_json::Value;
use shared::{error::AppError, response::ActionResult};

pub mod routes;

#[cfg(test)]
mod tests;

pub async fn get_control_config(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let row = client
        .query_one(
            "SELECT id, config_data, creator, create_time FROM x_mind_assemble_control_config LIMIT 1",
            &[],
        )
        .await;

    let data = match row {
        Ok(r) => serde_json::Map::from_iter([
            ("id".to_string(), Value::String(r.get("id"))),
            ("configData".to_string(), Value::String(r.get("config_data"))),
            ("creator".to_string(), Value::String(r.get("creator"))),
            ("createTime".to_string(), Value::String(r.get("create_time"))),
        ]),
        Err(_) => serde_json::Map::from_iter([
            ("id".to_string(), Value::String(String::new())),
            ("configData".to_string(), Value::Null),
            ("creator".to_string(), Value::String(String::new())),
            ("createTime".to_string(), Value::String(String::new())),
        ]),
    };

    Ok(Json(ActionResult::success(Value::Object(data))))
}

pub async fn update_control_config(
    pool: Extension<Pool>,
    axum::extract::Json(payload): axum::extract::Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let config_id = payload.get("id").and_then(|v| v.as_str()).unwrap_or("");
    let config_data = payload.get("configData").and_then(|v| v.as_str()).unwrap_or("");

    if config_id.is_empty() {
        return Ok(Json(ActionResult::error("id is required")));
    }

    client
        .execute(
            "UPDATE x_mind_assemble_control_config SET config_data = $1 WHERE id = $2",
            &[&config_data, &config_id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("id".to_string(), Value::String(config_id.to_string())),
        ("updated".to_string(), Value::Bool(true)),
    ])))))
}

pub fn mind_assemble_control_router(pool: Pool) -> Router {
    routes::mind_assemble_control_routes(pool)
}

pub fn router(_pool: deadpool_postgres::Pool) -> axum::Router {
    axum::Router::new()
        .route("/mind_assemble_control/health", axum::routing::get(|| async { "TODO: mind_assemble_control - real implementation needed" }))
}