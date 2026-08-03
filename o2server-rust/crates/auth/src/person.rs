use axum::extract::{Extension, Path, Query};
use axum::Json;
use serde_json::Value;
use shared::response::ActionResult;

pub async fn get(
    Path(flag): Path<String>,
) -> Json<ActionResult<Value>> {
    let result = ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("flag".to_string(), Value::String(flag)),
        ("unique".to_string(), Value::String("admin".to_string())),
        ("name".to_string(), Value::String("admin".to_string())),
        ("mobile".to_string(), Value::Null),
        ("email".to_string(), Value::Null),
    ])));
    Json(result)
}

pub async fn list(
    Query(_params): Query<std::collections::HashMap<String, String>>,
) -> Json<ActionResult<Value>> {
    let result = ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(0))),
        ("size".to_string(), Value::Number(serde_json::Number::from(20))),
        ("data".to_string(), Value::Array(vec![])),
    ])));
    Json(result)
}
