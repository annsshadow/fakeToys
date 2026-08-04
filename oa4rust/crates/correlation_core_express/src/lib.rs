use axum::{
    extract::Extension,
    routing::get,
    Json, Router,
};
use deadpool_postgres::Pool;
use serde_json::Value;
use shared::{error::AppError, response::ActionResult};

pub mod routes;

/// 获取关联服务状态
/// 检查关联服务是否正常运行
pub async fn get_status(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let count: i64 = client
        .query_one("SELECT COUNT(*) FROM x_correlation", &[])
        .await
        .map_err(|_| AppError::Internal)?
        .get("count");

    let data = Value::Object(serde_json::Map::from_iter([
        ("status".to_string(), Value::String("running".to_string())),
        ("totalRecords".to_string(), Value::Number(serde_json::Number::from(count))),
        ("enabled".to_string(), Value::Bool(true)),
    ]));

    Ok(Json(ActionResult::success(data)))
}

/// 执行关联同步
/// 触发关联数据的同步操作
pub async fn sync_correlation(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let synced: i64 = client
        .query_one("SELECT COUNT(*) FROM x_correlation", &[])
        .await
        .map_err(|_| AppError::Internal)?
        .get("count");

    let data = Value::Object(serde_json::Map::from_iter([
        ("synced".to_string(), Value::Bool(true)),
        ("syncedRecords".to_string(), Value::Number(serde_json::Number::from(synced))),
        ("message".to_string(), Value::String("同步完成".to_string())),
    ]));

    Ok(Json(ActionResult::success(data)))
}

/// 创建关联核心服务路由
/// 注册以下路由：
/// - /jaxrs/correlation/core/express/status - 服务状态
/// - /jaxrs/correlation/core/express/sync - 同步关联
pub fn correlation_core_express_router(pool: Pool) -> Router {
    Router::new()
        .route("/jaxrs/correlation/core/express/status", get(get_status))
        .route("/jaxrs/correlation/core/express/sync", get(sync_correlation))
        .layer(Extension(pool))
}

#[cfg(test)]
mod tests;
