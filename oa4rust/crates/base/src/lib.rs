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

pub async fn echo_get() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("type".to_string(), Value::String("echo".to_string())),
        ("message".to_string(), Value::String("pong".to_string())),
    ])))))
}

pub async fn cache_detail(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let row = client
        .query_one("SELECT count(*) AS cache_count FROM pg_class WHERE relname LIKE 'cache_%'", &[])
        .await
        .map_err(|_| AppError::Internal)?;

    let count: i64 = row.get("cache_count");

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("status".to_string(), Value::String("running".to_string())),
        ("cacheCount".to_string(), Value::Number(serde_json::Number::from(count))),
    ])))))
}

pub async fn openapi_info() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("version".to_string(), Value::String("3.0.3".to_string())),
        ("title".to_string(), Value::String("OA4Rust API".to_string())),
    ])))))
}

pub fn base_router(pool: Pool) -> Router {
    routes::build_router(pool)
}
