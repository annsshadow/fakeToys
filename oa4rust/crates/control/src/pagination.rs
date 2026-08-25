use axum::Json;
use serde_json::Value;
use shared::response::ActionResult;

/// 游标分页响应封装（O2OA v9 Java 兼容形状，plan002 U2 行为对齐）
///
/// Java 实测基准（docs/audits/behavior-compare-first-run.md）：`data` 直接是数组，
/// count=总数、size=本页条数、position 为数字 0；不再嵌套 {count, size, data} 内层对象。
/// is_next 仅保留参数兼容既有调用点——Java 信封的 position 不区分 next/prev。
pub fn page_result(total: i64, data: Vec<Value>, _is_next: bool) -> Json<ActionResult<Value>> {
    let size = data.len() as i64;
    Json(ActionResult::java_success(Value::Array(data), total, size))
}
