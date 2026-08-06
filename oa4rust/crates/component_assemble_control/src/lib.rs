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
        ("maxComponentCount".to_string(), Value::Number(serde_json::Number::from(500i64))),
        ("allowCustomComponents".to_string(), Value::Bool(true)),
    ]));

    Ok(Json(ActionResult::success(data)))
}

#[axum::debug_handler]
pub async fn list_control_categories(
    _pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let categories = vec![
        Value::Object(serde_json::Map::from_iter([
            ("id".to_string(), Value::String("system".to_string())),
            ("name".to_string(), Value::String("System Components".to_string())),
            ("enabled".to_string(), Value::Bool(true)),
        ])),
        Value::Object(serde_json::Map::from_iter([
            ("id".to_string(), Value::String("custom".to_string())),
            ("name".to_string(), Value::String("Custom Components".to_string())),
            ("enabled".to_string(), Value::Bool(false)),
        ])),
    ];

    Ok(Json(ActionResult::success(Value::Array(categories))))
}

#[axum::debug_handler]
pub async fn update_control_config(
    _pool: Extension<Pool>,
    body: axum::extract::Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let config = body.0;
    tracing::info!("Updating component assemble control config: {:?}", config);

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("updated".to_string(), Value::Bool(true)),
            ("config".to_string(), config),
        ]),
    ))))
}

pub fn component_assemble_control_router(pool: Pool) -> Router {
    routes::router(pool)
}

pub fn router(_pool: deadpool_postgres::Pool) -> axum::Router {
    axum::Router::new()
        .route("/component_assemble_control/health", axum::routing::get(|| async { "TODO: component_assemble_control - real implementation needed" }))
}


/// Stub handler for /jaxrs/component/assemble/control/component/delete/all
/// TODO: Implement real business logic
pub async fn stub_component_assemble_control_component_delete_all() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Null)))
}

/// Stub handler for /jaxrs/component/assemble/control/component/list/all
/// TODO: Implement real business logic
pub async fn stub_component_assemble_control_component_list_all() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Null)))
}

/// Stub handler for /jaxrs/component/assemble/control/component/{flag}
/// TODO: Implement real business logic
pub async fn stub_component_assemble_control_component_flag() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Null)))
}

/// Stub handler for /jaxrs/component/assemble/control/status/list
/// TODO: Implement real business logic
pub async fn stub_component_assemble_control_status_list() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Null)))
}
