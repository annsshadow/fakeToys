use axum::{Json, Router};
use serde_json::Value;
use shared::{error::AppError, response::ActionResult};
use uuid::Uuid;

pub mod routes;

#[cfg(test)]
mod tests;

pub fn cms_express_router() -> Router {
    routes::cms_express_router()
}

#[axum::debug_handler]
pub async fn uuid_random() -> Result<Json<ActionResult<Value>>, AppError> {
    let uuid = Uuid::new_v4().to_string();
    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("uuid".to_string(), Value::String(uuid)),
    ])))))
}

#[axum::debug_handler]
pub async fn template_form_list() -> Result<Json<ActionResult<Value>>, AppError> {
    let data = vec![
        Value::Object(serde_json::Map::from_iter([
            ("id".to_string(), Value::String("template-001".to_string())),
            ("name".to_string(), Value::String("默认表单模板".to_string())),
            ("category".to_string(), Value::String("通用".to_string())),
        ])),
        Value::Object(serde_json::Map::from_iter([
            ("id".to_string(), Value::String("template-002".to_string())),
            ("name".to_string(), Value::String("审批表单模板".to_string())),
            ("category".to_string(), Value::String("流程".to_string())),
        ])),
    ];

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

#[axum::debug_handler]
pub async fn view_list_all() -> Result<Json<ActionResult<Value>>, AppError> {
    let data = vec![
        Value::Object(serde_json::Map::from_iter([
            ("id".to_string(), Value::String("view-001".to_string())),
            ("name".to_string(), Value::String("默认视图".to_string())),
            ("appId".to_string(), Value::String("app-001".to_string())),
        ])),
        Value::Object(serde_json::Map::from_iter([
            ("id".to_string(), Value::String("view-002".to_string())),
            ("name".to_string(), Value::String("审批视图".to_string())),
            ("appId".to_string(), Value::String("app-002".to_string())),
        ])),
    ];

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}
