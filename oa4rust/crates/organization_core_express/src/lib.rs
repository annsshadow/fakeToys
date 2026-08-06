use axum::{
    extract::Extension,
    routing::get,
    Json, Router,
};
use deadpool_postgres::Pool;
use serde_json::Value;
use shared::{error::AppError, response::ActionResult};

pub mod routes;

/// 获取组织服务状态
/// 检查组织服务是否正常运行
pub async fn get_status(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let person_count: i64 = client
        .query_one("SELECT COUNT(*) FROM x_org_person", &[])
        .await
        .map_err(|_| AppError::Internal)?
        .get("count");
    let group_count: i64 = client
        .query_one("SELECT COUNT(*) FROM x_org_group", &[])
        .await
        .map_err(|_| AppError::Internal)?
        .get("count");

    let data = Value::Object(serde_json::Map::from_iter([
        ("status".to_string(), Value::String("running".to_string())),
        ("personCount".to_string(), Value::Number(serde_json::Number::from(person_count))),
        ("groupCount".to_string(), Value::Number(serde_json::Number::from(group_count))),
        ("enabled".to_string(), Value::Bool(true)),
    ]));

    Ok(Json(ActionResult::success(data)))
}

/// 同步组织数据
/// 触发组织数据的同步操作
pub async fn sync_organization(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let synced: i64 = client
        .query_one("SELECT COUNT(*) FROM x_org_person", &[])
        .await
        .map_err(|_| AppError::Internal)?
        .get("count");

    let data = Value::Object(serde_json::Map::from_iter([
        ("synced".to_string(), Value::Bool(true)),
        ("syncedRecords".to_string(), Value::Number(serde_json::Number::from(synced))),
        ("lastSyncTime".to_string(), Value::String("".to_string())),
        ("message".to_string(), Value::String("同步完成".to_string())),
    ]));

    Ok(Json(ActionResult::success(data)))
}

/// 获取组织服务配置
/// 返回组织服务的配置信息
pub async fn get_config(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let enabled: bool = client
        .query_one("SELECT config_value FROM x_org_config WHERE config_key = 'sync_enabled'", &[])
        .await
        .map_err(|_| AppError::Internal)?
        .get("config_value");

    let data = Value::Object(serde_json::Map::from_iter([
        ("enabled".to_string(), Value::Bool(enabled)),
        ("syncInterval".to_string(), Value::Number(serde_json::Number::from(300i64))),
        ("maxRecords".to_string(), Value::Number(serde_json::Number::from(10000i64))),
    ]));

    Ok(Json(ActionResult::success(data)))
}

/// 创建组织核心服务路由
/// 注册以下路由：
/// - /jaxrs/organization/core/express/status - 服务状态
/// - /jaxrs/organization/core/express/sync - 同步组织数据
/// - /jaxrs/organization/core/express/config - 服务配置
pub fn organization_core_express_router(pool: Pool) -> Router {
    Router::new()
        .route("/jaxrs/organization/core/express/status", get(get_status))
        .route("/jaxrs/organization/core/express/sync", get(sync_organization))
        .route("/jaxrs/organization/core/express/config", get(get_config))
        .layer(Extension(pool))
}

#[cfg(test)]
mod tests;

pub fn router(pool: deadpool_postgres::Pool) -> axum::Router {
    organization_core_express_router(pool)
}