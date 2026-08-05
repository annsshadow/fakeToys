use axum::{Json, Router, routing::get, routing::post};
use serde_json::Value;
use shared::{error::AppError, response::ActionResult};

pub mod routes;

#[derive(Debug, serde::Deserialize)]
pub struct SendRequest {
    pub from: Option<String>,
    pub to: Option<String>,
    pub content: Option<String>,
}

pub async fn send_message(
    axum::extract::Json(req): Json<SendRequest>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("sent".to_string(), Value::Bool(true)),
            ("from".to_string(), Value::String(req.from.unwrap_or_default())),
            ("to".to_string(), Value::String(req.to.unwrap_or_default())),
        ]),
    ))))
}

pub async fn receive_list(
    axum::extract::Path(consume): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let data = vec![
        Value::Object(serde_json::Map::from_iter([
            ("id".to_string(), Value::String("msg-1".to_string())),
            ("consume".to_string(), Value::String(consume)),
            ("status".to_string(), Value::String("unread".to_string())),
        ])),
    ];

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

pub async fn mark_read(
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("id".to_string(), Value::String(id)),
        ("marked_read".to_string(), Value::Bool(true)),
    ])))))
}

pub fn message_assemble_communicate_router() -> Router {
    Router::new()
        .route("/jaxrs/message/assemble/communicate/send", post(send_message))
        .route("/jaxrs/message/assemble/communicate/receive/{consume}", get(receive_list))
        .route("/jaxrs/message/assemble/communicate/mark_read/{id}", post(mark_read))
}

#[cfg(test)]
mod tests;

pub fn router(_pool: deadpool_postgres::Pool) -> axum::Router {
    axum::Router::new()
        .route("/message_assemble_communicate/health", axum::routing::get(|| async { "TODO: message_assemble_communicate - real implementation needed" }))
}