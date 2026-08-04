use axum::{Json, Router, routing::get, routing::post};
use serde::Deserialize;
use serde_json::Value;
use shared::{error::AppError, response::ActionResult};

pub mod routes;

#[derive(Debug, Deserialize)]
pub struct CreatePortalRequest {
    pub name: Option<String>,
    pub description: Option<String>,
}

pub async fn create_design(
    axum::extract::Json(req): Json<CreatePortalRequest>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("created".to_string(), Value::Bool(true)),
            ("name".to_string(), Value::String(req.name.unwrap_or_default())),
            ("description".to_string(), Value::String(req.description.unwrap_or_default())),
        ]),
    ))))
}

pub async fn get_design(
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
            ("name".to_string(), Value::String("Portal Design".to_string())),
            ("components".to_string(), Value::Array(vec![])),
        ]),
    ))))
}

pub async fn list_designs() -> Result<Json<ActionResult<Value>>, AppError> {
    let data = vec![
        Value::Object(serde_json::Map::from_iter([
            ("id".to_string(), Value::String("design-1".to_string())),
            ("name".to_string(), Value::String("Design 1".to_string())),
        ])),
    ];

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

pub async fn save_design(
    axum::extract::Path(id): axum::extract::Path<String>,
    axum::extract::Json(body): Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
            ("saved".to_string(), Value::Bool(true)),
            ("updated_at".to_string(), Value::String("2024-01-01T00:00:00Z".to_string())),
        ]),
    ))))
}

pub fn portal_assemble_designer_router() -> Router {
    Router::new()
        .route("/jaxrs/portal/assemble/designer/create", post(create_design))
        .route("/jaxrs/portal/assemble/designer/get/:id", get(get_design))
        .route("/jaxrs/portal/assemble/designer/list", get(list_designs))
        .route("/jaxrs/portal/assemble/designer/save/:id", post(save_design))
}

#[cfg(test)]
mod tests;
