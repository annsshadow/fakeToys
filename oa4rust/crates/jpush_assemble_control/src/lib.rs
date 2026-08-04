use axum::{
    extract::Extension,
    Json,
    Router,
};
use deadpool_postgres::Pool;
use serde_json::Value;
use shared::{error::AppError, response::ActionResult};

pub mod routes;

#[cfg(test)]
mod tests;

#[axum::debug_handler]
pub async fn get_control_config(
    _pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let data = Value::Object(serde_json::Map::from_iter([
        ("enabled".to_string(), Value::Bool(true)),
        ("defaultAppKey".to_string(), Value::String("default".to_string())),
        ("maxPushCount".to_string(), Value::Number(serde_json::Number::from(10000i64))),
    ]));

    Ok(Json(ActionResult::success(data)))
}

#[axum::debug_handler]
pub async fn list_control_apps(
    _pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let apps = vec![
        Value::Object(serde_json::Map::from_iter([
            ("id".to_string(), Value::String("default".to_string())),
            ("name".to_string(), Value::String("Default App".to_string())),
            ("enabled".to_string(), Value::Bool(true)),
        ])),
    ];

    Ok(Json(ActionResult::success(Value::Array(apps))))
}

#[axum::debug_handler]
pub async fn update_control_config(
    _pool: Extension<Pool>,
    body: axum::extract::Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let config = body.0;
    tracing::info!("Updating jpush assemble control config: {:?}", config);

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("updated".to_string(), Value::Bool(true)),
            ("config".to_string(), config),
        ]),
    ))))
}

pub fn jpush_assemble_control_router(pool: Pool) -> Router {
    routes::router(pool)
}
