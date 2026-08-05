use axum::{Json, Router, routing::get, routing::post};
use serde::Deserialize;
use serde_json::Value;
use shared::{error::AppError, response::ActionResult};

pub mod routes;

#[derive(Debug, Deserialize)]
pub struct CreateSurfaceRequest {
    pub name: Option<String>,
    pub query: Option<String>,
    pub template: Option<String>,
}

pub async fn get_surface(
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
            ("name".to_string(), Value::String("Query Surface".to_string())),
            ("query".to_string(), Value::String("".to_string())),
            ("template".to_string(), Value::String("default".to_string())),
        ]),
    ))))
}

pub async fn create_surface(
    axum::extract::Json(req): Json<CreateSurfaceRequest>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("created".to_string(), Value::Bool(true)),
            ("id".to_string(), Value::String("surface-1".to_string())),
            ("name".to_string(), Value::String(req.name.unwrap_or_default())),
            ("query".to_string(), Value::String(req.query.unwrap_or_default())),
            ("template".to_string(), Value::String(req.template.unwrap_or_default())),
        ]),
    ))))
}

pub async fn list_surfaces(
    axum::extract::Path(category): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let data = vec![
        Value::Object(serde_json::Map::from_iter([
            ("id".to_string(), Value::String("surface-1".to_string())),
            ("name".to_string(), Value::String("Surface 1".to_string())),
            ("category".to_string(), Value::String(category)),
        ])),
    ];

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

pub async fn save_surface(
    axum::extract::Path(id): axum::extract::Path<String>,
    axum::extract::Json(req): Json<CreateSurfaceRequest>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
            ("saved".to_string(), Value::Bool(true)),
            ("name".to_string(), Value::String(req.name.unwrap_or_default())),
            ("query".to_string(), Value::String(req.query.unwrap_or_default())),
            ("template".to_string(), Value::String(req.template.unwrap_or_default())),
        ]),
    ))))
}

pub async fn delete_surface(
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
            ("deleted".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn preview_surface(
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id.clone())),
            ("preview_url".to_string(), Value::String(format!("/preview/query/{}", id))),
            ("html".to_string(), Value::String("<div>Query Preview</div>".to_string())),
        ]),
    ))))
}

pub fn query_assemble_surface_router() -> Router {
    Router::new()
        .route("/jaxrs/query/assemble/surface/get/{id}", get(get_surface))
        .route("/jaxrs/query/assemble/surface/create", post(create_surface))
        .route("/jaxrs/query/assemble/surface/list/{category}", get(list_surfaces))
        .route("/jaxrs/query/assemble/surface/save/{id}", post(save_surface))
        .route("/jaxrs/query/assemble/surface/delete/{id}", post(delete_surface))
        .route("/jaxrs/query/assemble/surface/preview/{id}", get(preview_surface))
}

#[cfg(test)]
mod tests;
