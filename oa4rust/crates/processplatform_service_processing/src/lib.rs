use axum::{Json, Router, routing::get, routing::post};
use serde::Deserialize;
use serde_json::Value;
use shared::{error::AppError, response::ActionResult};

pub mod routes;

#[derive(Debug, Deserialize)]
pub struct CreateProcessRequest {
    pub name: Option<String>,
    pub description: Option<String>,
    pub category: Option<String>,
}

pub async fn get_process(
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
            ("name".to_string(), Value::String("Process".to_string())),
            ("status".to_string(), Value::String("active".to_string())),
        ]),
    ))))
}

pub async fn create_process(
    axum::extract::Json(req): Json<CreateProcessRequest>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("created".to_string(), Value::Bool(true)),
            ("name".to_string(), Value::String(req.name.unwrap_or_default())),
            ("description".to_string(), Value::String(req.description.unwrap_or_default())),
            ("category".to_string(), Value::String(req.category.unwrap_or_default())),
        ]),
    ))))
}

pub async fn list_processes(
    axum::extract::Path(category): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let data = vec![
        Value::Object(serde_json::Map::from_iter([
            ("id".to_string(), Value::String("proc-1".to_string())),
            ("name".to_string(), Value::String("Process 1".to_string())),
            ("category".to_string(), Value::String(category)),
        ])),
    ];

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

pub async fn execute_process(
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
            ("executed".to_string(), Value::Bool(true)),
            ("execution_id".to_string(), Value::String("exec-1".to_string())),
            ("status".to_string(), Value::String("running".to_string())),
        ]),
    ))))
}

pub async fn get_process_instance(
    axum::extract::Path(execution_id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("executionId".to_string(), Value::String(execution_id)),
            ("status".to_string(), Value::String("running".to_string())),
            ("current_node".to_string(), Value::String("start".to_string())),
        ]),
    ))))
}

pub async fn cancel_process_instance(
    axum::extract::Path(execution_id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("executionId".to_string(), Value::String(execution_id)),
            ("cancelled".to_string(), Value::Bool(true)),
            ("cancelled_at".to_string(), Value::String("2024-01-01T00:00:00Z".to_string())),
        ]),
    ))))
}

pub fn processplatform_service_processing_router() -> Router {
    Router::new()
        .route("/jaxrs/processplatform/service/processing/get/{id}", get(get_process))
        .route("/jaxrs/processplatform/service/processing/create", post(create_process))
        .route("/jaxrs/processplatform/service/processing/list/{category}", get(list_processes))
        .route("/jaxrs/processplatform/service/processing/execute/{id}", post(execute_process))
        .route("/jaxrs/processplatform/service/processing/instance/{executionId}", get(get_process_instance))
        .route("/jaxrs/processplatform/service/processing/cancel/{executionId}", post(cancel_process_instance))
}

#[cfg(test)]
mod tests;

pub fn router(_pool: deadpool_postgres::Pool) -> axum::Router {
    axum::Router::new()
        .route("/processplatform_service_processing/health", axum::routing::get(|| async { "TODO: processplatform_service_processing - real implementation needed" }))
}