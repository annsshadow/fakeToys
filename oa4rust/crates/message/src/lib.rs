use axum::{Json, Router, routing::get, routing::post};
use serde::Deserialize;
use serde_json::Value;

use shared::{error::AppError, response::ActionResult};

#[derive(Debug, Deserialize)]
pub struct ConsumeListRequest {
    pub consume: String,
    pub count: Option<i64>,
}

#[derive(Debug, Deserialize)]
pub struct CreateMessageRequest {
    pub title: Option<String>,
    pub body: Option<String>,
    pub r#type: Option<String>,
    pub consumer: Option<String>,
}

pub async fn consume_list(
    axum::extract::Path((consume, count)): axum::extract::Path<(String, i64)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let data = vec![
        Value::Object(serde_json::Map::from_iter([
            ("id".to_string(), Value::String("msg-1".to_string())),
            ("title".to_string(), Value::String("Test message".to_string())),
            ("consumed".to_string(), Value::Bool(false)),
        ]))
    ];

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

pub async fn update_single(
    axum::extract::Path((id, r#type)): axum::extract::Path<(String, String)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("id".to_string(), Value::String(id)),
        ("type".to_string(), Value::String(r#type)),
        ("updated".to_string(), Value::Bool(true)),
    ])))))
}

pub async fn custom_create(
    axum::extract::Json(req): Json<CreateMessageRequest>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("created".to_string(), Value::Bool(true)),
        ("title".to_string(), Value::String(req.title.unwrap_or_default())),
    ])))))
}

pub fn message_router() -> Router {
    Router::new()
        .route("/jaxrs/message/consume/list/{consume}/count/{count}", get(consume_list))
        .route("/jaxrs/message/consume/{id}/type/{type}", get(update_single))
        .route("/jaxrs/message/custom/create", post(custom_create))
}

pub fn router(_pool: deadpool_postgres::Pool) -> axum::Router {
    axum::Router::new()
        .route("/message/health", axum::routing::get(|| async { "TODO: message - real implementation needed" }))
}