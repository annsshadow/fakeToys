use axum::{
    extract::Extension,
    Json,
};
use deadpool_postgres::Pool;
use serde_json::Value;
use shared::{error::AppError, response::ActionResult};

mod routes;

#[cfg(test)]
mod tests;

pub fn ai_router(pool: Pool) -> axum::Router {
    routes::ai_router(pool)
}

#[axum::debug_handler]
pub async fn config_get(
    _pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let data = Value::Object(serde_json::Map::from_iter([
        ("config".to_string(), Value::String("base".to_string())),
        ("version".to_string(), Value::String("1.0.0".to_string())),
        ("enabled".to_string(), Value::Bool(true)),
    ]));

    Ok(Json(ActionResult::success(data)))
}

#[axum::debug_handler]
pub async fn list_enable_model(
    _pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let data = vec![
        Value::Object(serde_json::Map::from_iter([
            ("flag".to_string(), Value::String("gpt-4".to_string())),
            ("name".to_string(), Value::String("GPT-4".to_string())),
            ("enable".to_string(), Value::Bool(true)),
        ])),
        Value::Object(serde_json::Map::from_iter([
            ("flag".to_string(), Value::String("claude-3".to_string())),
            ("name".to_string(), Value::String("Claude 3".to_string())),
            ("enable".to_string(), Value::Bool(true)),
        ])),
    ];

    Ok(Json(ActionResult::success(Value::Array(data))))
}

#[axum::debug_handler]
pub async fn sync_to_knowledge(
    _pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let data = Value::Object(serde_json::Map::from_iter([
        ("synced".to_string(), Value::Bool(true)),
        ("count".to_string(), Value::Number(serde_json::Number::from(0i64))),
        ("message".to_string(), Value::String("sync completed".to_string())),
    ]));

    Ok(Json(ActionResult::success(data)))
}
