use axum::{Json, Router, routing::get};
use serde_json::Value;

use shared::{error::AppError, response::ActionResult};

pub async fn view_list() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(0_i64))),
        ("data".to_string(), Value::Array(vec![])),
    ])))))
}

pub fn cms_control_router() -> Router {
    Router::new().route("/jaxrs/cms/view/list/all", get(view_list))
}
