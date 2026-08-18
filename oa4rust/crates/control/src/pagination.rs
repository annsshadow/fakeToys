use axum::Json;
use serde_json::Value;
use shared::response::ActionResult;

/// 游标分页响应封装
pub fn page_result(total: i64, data: Vec<Value>, is_next: bool) -> Json<ActionResult<Value>> {
    let size = data.len() as i64;
    let result = Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(total))),
        ("size".to_string(), Value::Number(serde_json::Number::from(size))),
        ("data".to_string(), Value::Array(data)),
    ]));
    Json(ActionResult {
        data: Some(result),
        r#type: Some("success".to_string()),
        message: None,
        date: None,
        spent: None,
        size: Some(size),
        count: Some(total),
        position: Some(if is_next { "next" } else { "prev" }.to_string()),
        prompt: None,
    })
}
