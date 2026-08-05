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

pub fn file_assemble_control_router(pool: Pool) -> axum::Router {
    routes::router(pool)
}

#[axum::debug_handler]
pub async fn get_control_config(
    _pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let data = Value::Object(serde_json::Map::from_iter([
        ("enabled".to_string(), Value::Bool(true)),
        ("defaultStorage".to_string(), Value::String("local".to_string())),
        ("maxUploadSize".to_string(), Value::Number(serde_json::Number::from(104857600i64))),
    ]));

    Ok(Json(ActionResult::success(data)))
}

#[axum::debug_handler]
pub async fn list_storage_pools(
    _pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let data = vec![
        Value::Object(serde_json::Map::from_iter([
            ("id".to_string(), Value::String("local".to_string())),
            ("name".to_string(), Value::String("Local Storage".to_string())),
            ("enabled".to_string(), Value::Bool(true)),
        ])),
        Value::Object(serde_json::Map::from_iter([
            ("id".to_string(), Value::String("minio".to_string())),
            ("name".to_string(), Value::String("MinIO Storage".to_string())),
            ("enabled".to_string(), Value::Bool(false)),
        ])),
    ];

    Ok(Json(ActionResult::success(Value::Array(data))))
}

#[axum::debug_handler]
pub async fn update_control_config(
    _pool: Extension<Pool>,
    body: axum::extract::Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let config = body.0;
    tracing::info!("Updating file assemble control config: {:?}", config);

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("updated".to_string(), Value::Bool(true)),
            ("config".to_string(), config),
        ]),
    ))))
}

#[axum::debug_handler]
pub async fn list_control_categories(
    _pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let categories = vec![
        Value::Object(serde_json::Map::from_iter([
            ("id".to_string(), Value::String("storage".to_string())),
            ("name".to_string(), Value::String("Storage".to_string())),
            ("description".to_string(), Value::String("Storage configuration".to_string())),
        ])),
        Value::Object(serde_json::Map::from_iter([
            ("id".to_string(), Value::String("security".to_string())),
            ("name".to_string(), Value::String("Security".to_string())),
            ("description".to_string(), Value::String("Security settings".to_string())),
        ])),
        Value::Object(serde_json::Map::from_iter([
            ("id".to_string(), Value::String("quota".to_string())),
            ("name".to_string(), Value::String("Quota".to_string())),
            ("description".to_string(), Value::String("Quota management".to_string())),
        ])),
    ];

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(categories.len() as i64))),
            ("data".to_string(), Value::Array(categories)),
        ]),
    ))))
}

pub fn router(_pool: deadpool_postgres::Pool) -> axum::Router {
    axum::Router::new()
        .route("/file_assemble_control/health", axum::routing::get(|| async { "TODO: file_assemble_control - real implementation needed" }))
}