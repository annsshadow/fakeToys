use axum::{Json, Router, routing::get, routing::post};
use serde::Deserialize;
use serde_json::Value;
use shared::{error::AppError, response::ActionResult};

pub mod routes;

#[derive(Debug, Deserialize)]
pub struct CreateSurfaceRequest {
    pub name: Option<String>,
    pub template: Option<String>,
    pub category: Option<String>,
}

pub async fn get_surface(
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
            ("name".to_string(), Value::String("Process Platform Surface".to_string())),
            ("html".to_string(), Value::String("<div></div>".to_string())),
            ("category".to_string(), Value::String("processplatform".to_string())),
        ]),
    ))))
}

pub async fn create_surface(
    axum::extract::Json(req): Json<CreateSurfaceRequest>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("created".to_string(), Value::Bool(true)),
            ("name".to_string(), Value::String(req.name.unwrap_or_default())),
            ("template".to_string(), Value::String(req.template.unwrap_or_default())),
            ("category".to_string(), Value::String(req.category.unwrap_or_default())),
        ]),
    ))))
}

pub async fn list_surfaces(
    axum::extract::Path(category): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let data = vec![
        Value::Object(serde_json::Map::from_iter([
            ("id".to_string(), Value::String("surface-1".to_string())),
            ("name".to_string(), Value::String("Process Platform Surface 1".to_string())),
            ("category".to_string(), Value::String(category)),
        ])),
    ];

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

pub async fn preview_surface(
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id.clone())),
            ("preview_url".to_string(), Value::String(format!("/preview/{}", id))),
            ("html".to_string(), Value::String("<div>Process Platform Preview</div>".to_string())),
        ]),
    ))))
}

pub async fn publish_surface(
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
            ("published".to_string(), Value::Bool(true)),
            ("published_at".to_string(), Value::String("2024-01-01T00:00:00Z".to_string())),
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

pub fn processplatform_assemble_surface_router() -> Router {
    Router::new()
        .route("/jaxrs/processplatform/assemble/surface/get/:id", get(get_surface))
        .route("/jaxrs/processplatform/assemble/surface/create", post(create_surface))
        .route("/jaxrs/processplatform/assemble/surface/list/:category", get(list_surfaces))
        .route("/jaxrs/processplatform/assemble/surface/preview/:id", get(preview_surface))
        .route("/jaxrs/processplatform/assemble/surface/publish/:id", post(publish_surface))
        .route("/jaxrs/processplatform/assemble/surface/delete/:id", post(delete_surface))
}

#[cfg(test)]
mod tests;
