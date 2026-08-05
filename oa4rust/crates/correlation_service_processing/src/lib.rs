use axum::{Json, Router, routing::get, routing::post};
use serde::Deserialize;
use serde_json::Value;
use shared::{error::AppError, response::ActionResult};

pub mod routes;

#[derive(Debug, Deserialize)]
pub struct LinkRequest {
    pub source_type: Option<String>,
    pub source_id: Option<String>,
    pub target_type: Option<String>,
    pub target_id: Option<String>,
}

pub async fn link_service(
    axum::extract::Json(req): Json<LinkRequest>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("linked".to_string(), Value::Bool(true)),
            ("source_type".to_string(), Value::String(req.source_type.unwrap_or_default())),
            ("source_id".to_string(), Value::String(req.source_id.unwrap_or_default())),
            ("target_type".to_string(), Value::String(req.target_type.unwrap_or_default())),
            ("target_id".to_string(), Value::String(req.target_id.unwrap_or_default())),
        ]),
    ))))
}

pub async fn get_link(
    axum::extract::Path((source_type, source_id)): axum::extract::Path<(String, String)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("sourceType".to_string(), Value::String(source_type)),
            ("sourceId".to_string(), Value::String(source_id)),
            ("targetType".to_string(), Value::String("unknown".to_string())),
            ("targetId".to_string(), Value::String("none".to_string())),
        ]),
    ))))
}

pub async fn list_links(
    axum::extract::Path(source_type): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let data = vec![
        Value::Object(serde_json::Map::from_iter([
            ("sourceType".to_string(), Value::String(source_type.clone())),
            ("sourceId".to_string(), Value::String("src-1".to_string())),
            ("targetType".to_string(), Value::String("process".to_string())),
            ("targetId".to_string(), Value::String("proc-1".to_string())),
        ])),
    ];

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

pub async fn unlink_service(
    axum::extract::Path((source_type, source_id, target_type, target_id)): axum::extract::Path<(String, String, String, String)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("unlinked".to_string(), Value::Bool(true)),
            ("sourceType".to_string(), Value::String(source_type)),
            ("sourceId".to_string(), Value::String(source_id)),
            ("targetType".to_string(), Value::String(target_type)),
            ("targetId".to_string(), Value::String(target_id)),
        ]),
    ))))
}

pub fn correlation_service_processing_router() -> Router {
    Router::new()
        .route("/jaxrs/correlation/service/processing/list/{sourceType}", get(list_links))
        .route("/jaxrs/correlation/service/processing/link/{sourceType}/{sourceId}", get(get_link))
        .route("/jaxrs/correlation/service/processing/link", post(link_service))
        .route("/jaxrs/correlation/service/processing/unlink/{sourceType}/{sourceId}/{targetType}/{targetId}", post(unlink_service))
}

#[cfg(test)]
mod tests;

pub fn router(_pool: deadpool_postgres::Pool) -> axum::Router {
    axum::Router::new()
        .route("/correlation_service_processing/health", axum::routing::get(|| async { "TODO: correlation_service_processing - real implementation needed" }))
}