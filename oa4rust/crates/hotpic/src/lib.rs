use axum::{
    extract::Extension, extract::Path,
    Json, Router,
};
use deadpool_postgres::Pool;
use serde_json::Value;
use shared::{error::AppError, response::ActionResult};

pub mod routes;

#[axum::debug_handler]
pub async fn exists_check() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("allExists".to_string(), Value::Bool(true)),
        ("count".to_string(), Value::Number(serde_json::Number::from(0_i64))),
    ])))))
}

#[axum::debug_handler]
pub async fn get_by_id(
    _pool: Extension<Pool>,
    Path(id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    if id.is_empty() {
        return Err(AppError::BadRequest("id cannot be empty".to_string()));
    }

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("id".to_string(), Value::String(id)),
        ("title".to_string(), Value::String("HotPic".to_string())),
        ("base64".to_string(), Value::String("placeholder-base64-data".to_string())),
    ])))))
}

#[axum::debug_handler]
pub async fn list_by_application_and_info_id(
    _pool: Extension<Pool>,
    Path((application, info_id)): Path<(String, String)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let data = vec![
        Value::Object(serde_json::Map::from_iter([
            ("id".to_string(), Value::String(format!("{}-{}", application, info_id))),
            ("application".to_string(), Value::String(application)),
            ("infoId".to_string(), Value::String(info_id)),
            ("title".to_string(), Value::String("HotPic Title".to_string())),
        ]))
    ];

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

pub fn hotpic_router(pool: Pool) -> Router {
    Router::new()
        .route("/jaxrs/hotpic/user/hotpic/exists/check", axum::routing::get(exists_check))
        .route("/jaxrs/hotpic/user/hotpic/{id}", axum::routing::get(get_by_id))
        .route("/jaxrs/hotpic/user/hotpic/{application}/{infoId}", axum::routing::get(list_by_application_and_info_id))
        .layer(Extension(pool))
}

#[cfg(test)]
mod tests;

pub fn router(_pool: deadpool_postgres::Pool) -> axum::Router {
    axum::Router::new()
        .route("/hotpic/health", axum::routing::get(|| async { "TODO: hotpic - real implementation needed" }))
}