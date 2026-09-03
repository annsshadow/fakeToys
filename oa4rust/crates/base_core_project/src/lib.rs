use axum::{extract::Extension, Json, Router, routing::{get, post}};
use deadpool_postgres::Pool;
use serde_json::Value;
use shared::{error::AppError, response::ActionResult};

pub const JAVA_BASE: &str = '/jaxrs';

#[axum::debug_handler]
pub async fn cache_post(pool: Extension<Pool>, Json(_req): Json<Value>) -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(serde_json::json!({"status": "ok"}))))
}

#[axum::debug_handler]
pub async fn cache_config_flush(pool: Extension<Pool>) -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(serde_json::json!({"status": "flushed"}))))
}

#[axum::debug_handler]
pub async fn cache_commonscript_flush(pool: Extension<Pool>) -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(serde_json::json!({"status": "flushed"}))))
}

#[axum::debug_handler]
pub async fn cache_detail(pool: Extension<Pool>) -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(serde_json::json!({"detail": "cache info"}))))
}

#[axum::debug_handler]
pub async fn echo(pool: Extension<Pool>) -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(serde_json::json!({"message": "pong"}))))
}

#[axum::debug_handler]
pub async fn fireschedule_classname(pool: Extension<Pool>) -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(serde_json::json!({"class": "test"}))))
}

#[axum::debug_handler]
pub async fn openapi(pool: Extension<Pool>) -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(serde_json::json!({"openapi": "3.0"}))))
}

#[axum::debug_handler]
pub async fn sysresource_filepath(pool: Extension<Pool>) -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(serde_json::json!({"path": "/"}))))
}

pub fn base_core_project_router() -> Router {
    Router::new()
        .route("/jaxrs/cache", post(cache_post))
        .route("/jaxrs/cache/config/flush", get(cache_config_flush))
        .route("/jaxrs/cache/commonscript/flush", get(cache_commonscript_flush))
        .route("/jaxrs/cache/detail", get(cache_detail))
        .route("/jaxrs/echo", get(echo))
        .route("/jaxrs/fireschedule/classname/{className}", get(fireschedule_classname))
        .route("/jaxrs/openapi", get(openapi))
        .route("/jaxrs/sysresource/filePath/{filePath}", get(sysresource_filepath))
}

pub fn router(pool: Pool) -> Router {
    base_core_project_router().layer(Extension(pool))
}
