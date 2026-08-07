use axum::{
    extract::Extension,
    Json,
};
use deadpool_postgres::Pool;
use serde_json::Value;
use shared::{error::AppError, response::ActionResult};

mod routes;

#[axum::debug_handler]
pub async fn state_summary(
    _pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let data = Value::Object(serde_json::Map::from_iter([
        ("totalProcesses".to_string(), Value::Number(serde_json::Number::from(128))),
        ("running".to_string(), Value::Number(serde_json::Number::from(42))),
        ("completed".to_string(), Value::Number(serde_json::Number::from(81))),
        ("expired".to_string(), Value::Number(serde_json::Number::from(5))),
    ]));
    Ok(Json(ActionResult::success(data)))
}

#[axum::debug_handler]
pub async fn state_running(
    _pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let data = Value::Object(serde_json::Map::from_iter([
        ("runningCount".to_string(), Value::Number(serde_json::Number::from(42))),
        (
            "applications".to_string(),
            Value::Array(vec![
                Value::String("OA审批".to_string()),
                Value::String("报销流程".to_string()),
                Value::String("请假流程".to_string()),
            ]),
        ),
    ]));
    Ok(Json(ActionResult::success(data)))
}

#[axum::debug_handler]
pub async fn state_organization(
    _pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let data = Value::Object(serde_json::Map::from_iter([
        (
            "organizations".to_string(),
            Value::Array(vec![
                Value::Object(serde_json::Map::from_iter([
                    ("id".to_string(), Value::String("org-001".to_string())),
                    ("name".to_string(), Value::String("研发部".to_string())),
                    ("count".to_string(), Value::Number(serde_json::Number::from(15))),
                ])),
                Value::Object(serde_json::Map::from_iter([
                    ("id".to_string(), Value::String("org-002".to_string())),
                    ("name".to_string(), Value::String("产品部".to_string())),
                    ("count".to_string(), Value::Number(serde_json::Number::from(8))),
                ])),
            ]),
        ),
    ]));
    Ok(Json(ActionResult::success(data)))
}

pub fn process_bam_router(pool: Pool) -> axum::Router {
    routes::process_bam_router(pool)
}

pub fn router(pool: deadpool_postgres::Pool) -> axum::Router {
    crate::process_bam_router(pool)
}

#[cfg(test)]
mod tests;
