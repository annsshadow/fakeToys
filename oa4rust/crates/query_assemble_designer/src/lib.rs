use axum::{Json, Router, routing::get, routing::post};
use serde::Deserialize;
use serde_json::Value;
use shared::{error::AppError, response::ActionResult};

pub mod routes;

#[derive(Debug, Deserialize)]
pub struct CreateDesignerRequest {
    pub name: Option<String>,
    pub query: Option<String>,
    pub category: Option<String>,
}

pub async fn get_designer(
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
            ("name".to_string(), Value::String("Query Designer".to_string())),
            ("query".to_string(), Value::String("".to_string())),
            ("category".to_string(), Value::String("default".to_string())),
        ]),
    ))))
}

pub async fn create_designer(
    axum::extract::Json(req): Json<CreateDesignerRequest>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("created".to_string(), Value::Bool(true)),
            ("id".to_string(), Value::String("designer-1".to_string())),
            ("name".to_string(), Value::String(req.name.unwrap_or_default())),
            ("query".to_string(), Value::String(req.query.unwrap_or_default())),
            ("category".to_string(), Value::String(req.category.unwrap_or_default())),
        ]),
    ))))
}

pub async fn list_designers(
    axum::extract::Path(category): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let data = vec![
        Value::Object(serde_json::Map::from_iter([
            ("id".to_string(), Value::String("designer-1".to_string())),
            ("name".to_string(), Value::String("Designer 1".to_string())),
            ("category".to_string(), Value::String(category)),
        ])),
    ];

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

pub async fn save_designer(
    axum::extract::Path(id): axum::extract::Path<String>,
    axum::extract::Json(req): Json<CreateDesignerRequest>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
            ("saved".to_string(), Value::Bool(true)),
            ("name".to_string(), Value::String(req.name.unwrap_or_default())),
            ("query".to_string(), Value::String(req.query.unwrap_or_default())),
        ]),
    ))))
}

pub async fn delete_designer(
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
            ("deleted".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub fn query_assemble_designer_router() -> Router {
    Router::new()
        .route("/jaxrs/query/assemble/designer/get/{id}", get(get_designer))
        .route("/jaxrs/query/assemble/designer/create", post(create_designer))
        .route("/jaxrs/query/assemble/designer/list/{category}", get(list_designers))
        .route("/jaxrs/query/assemble/designer/save/{id}", post(save_designer))
        .route("/jaxrs/query/assemble/designer/delete/{id}", post(delete_designer))
}

#[cfg(test)]
mod tests;

pub fn router(_pool: deadpool_postgres::Pool) -> axum::Router {
    axum::Router::new()
        .route("/query_assemble_designer/health", axum::routing::get(|| async { "TODO: query_assemble_designer - real implementation needed" }))
}