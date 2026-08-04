use axum::{
    extract::Extension,
    Json, Router,
};
use deadpool_postgres::Pool;
use serde_json::Value;
use shared::{error::AppError, response::ActionResult};

pub mod routes;

#[cfg(test)]
mod tests;

/// Echo 接口（健康检查）
///
/// 返回固定的 `{"type":"echo","message":"pong"}` 响应，用于验证服务是否正常运行。
///
/// # 返回
/// - `Ok(Json<ActionResult<Value>>)`: 成功响应，内容为 pong 消息
pub async fn echo_get() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("type".to_string(), Value::String("echo".to_string())),
        ("message".to_string(), Value::String("pong".to_string())),
    ])))))
}

/// 查询数据库缓存表数量
///
/// 统计 PostgreSQL 中所有以 `cache_` 开头的表数量，用于监控缓存状态。
///
/// # 参数
/// - `pool`: 数据库连接池
///
/// # 返回
/// - `Ok(Json<ActionResult<Value>>)`: 包含 `status`（"running"）和 `cacheCount`（缓存表数量）
pub async fn cache_detail(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let row = client
        .query_one("SELECT count(*) AS cache_count FROM pg_class WHERE relname LIKE 'cache_%'", &[])
        .await
        .map_err(|_| AppError::Internal)?;

    let count: i64 = row.get("cache_count");

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("status".to_string(), Value::String("running".to_string())),
        ("cacheCount".to_string(), Value::Number(serde_json::Number::from(count))),
    ])))))
}

/// 获取 OpenAPI 基础信息
///
/// 返回当前 API 的版本号和标题，供 API 文档工具（如 Swagger）使用。
///
/// # 返回
/// - `Ok(Json<ActionResult<Value>>)`: 包含 `version`（"3.0.3"）和 `title`（"OA4Rust API"）
pub async fn openapi_info() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("version".to_string(), Value::String("3.0.3".to_string())),
        ("title".to_string(), Value::String("OA4Rust API".to_string())),
    ])))))
}

/// 构建基础模块路由
///
/// 委托给 `routes::build_router` 构建完整路由树。
///
/// # 参数
/// - `pool`: 数据库连接池
///
/// # 返回
/// - `Router`: Axum 路由实例
pub fn base_router(pool: Pool) -> Router {
    routes::build_router(pool)
}
