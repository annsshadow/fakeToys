use axum::{
    extract::Extension,
    Json,
};
use deadpool_postgres::Pool;
use serde_json::Value;
use shared::{error::AppError, response::ActionResult};

pub mod routes;

#[cfg(test)]
mod tests;

pub fn ai_assemble_control_router(pool: Pool) -> axum::Router {
    routes::router(pool)
}

#[axum::debug_handler]
pub async fn get_ai_control_config(
    _pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let data = Value::Object(serde_json::Map::from_iter([
        ("defaultModel".to_string(), Value::String("gpt-4".to_string())),
        ("temperature".to_string(), Value::Number(serde_json::Number::from_f64(0.7).unwrap())),
        ("maxTokens".to_string(), Value::Number(serde_json::Number::from(4096i64))),
        ("enabled".to_string(), Value::Bool(true)),
    ]));

    Ok(Json(ActionResult::success(data)))
}

#[axum::debug_handler]
pub async fn list_ai_models(
    _pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let models = vec![
        Value::Object(serde_json::Map::from_iter([
            ("id".to_string(), Value::String("gpt-4".to_string())),
            ("name".to_string(), Value::String("GPT-4".to_string())),
            ("enabled".to_string(), Value::Bool(true)),
            ("contextWindow".to_string(), Value::Number(serde_json::Number::from(8192i64))),
        ])),
        Value::Object(serde_json::Map::from_iter([
            ("id".to_string(), Value::String("gpt-3.5-turbo".to_string())),
            ("name".to_string(), Value::String("GPT-3.5 Turbo".to_string())),
            ("enabled".to_string(), Value::Bool(true)),
            ("contextWindow".to_string(), Value::Number(serde_json::Number::from(4096i64))),
        ])),
        Value::Object(serde_json::Map::from_iter([
            ("id".to_string(), Value::String("claude-3-sonnet".to_string())),
            ("name".to_string(), Value::String("Claude 3 Sonnet".to_string())),
            ("enabled".to_string(), Value::Bool(true)),
            ("contextWindow".to_string(), Value::Number(serde_json::Number::from(200000i64))),
        ])),
    ];

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(models.len() as i64))),
            ("data".to_string(), Value::Array(models)),
        ]),
    ))))
}

#[axum::debug_handler]
pub async fn update_ai_control_config(
    _pool: Extension<Pool>,
    body: axum::extract::Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let config = body.0;
    tracing::info!("Updating AI assemble control config: {:?}", config);

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("updated".to_string(), Value::Bool(true)),
            ("config".to_string(), config),
        ]),
    ))))
}

#[axum::debug_handler]
pub async fn get_usage_stats(
    _pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let data = Value::Object(serde_json::Map::from_iter([
        ("totalRequests".to_string(), Value::Number(serde_json::Number::from(0i64))),
        ("totalTokens".to_string(), Value::Number(serde_json::Number::from(0i64))),
        ("costThisMonth".to_string(), Value::Number(serde_json::Number::from_f64(0.0).unwrap())),
    ]));

    Ok(Json(ActionResult::success(data)))
}

pub fn router(_pool: deadpool_postgres::Pool) -> axum::Router {
    axum::Router::new()
        .route("/ai_assemble_control/health", axum::routing::get(|| async { "TODO: ai_assemble_control - real implementation needed" }))
}