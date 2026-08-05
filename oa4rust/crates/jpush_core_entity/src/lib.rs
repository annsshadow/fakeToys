use axum::{
    extract::Extension,
    routing::get,
    Json, Router,
};
use deadpool_postgres::Pool;
use serde_json::Value;
use shared::{error::AppError, response::ActionResult};

pub mod routes;

// 推送设备实体
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct PushDevice {
    pub id: String,
    pub user_id: String,
    pub platform: String,
    pub token: String,
}

// 推送模板实体
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct PushTemplate {
    pub id: String,
    pub name: String,
    pub title: String,
    pub content: String,
}

/// 获取推送设备列表
/// 从数据库查询 x_jpush_device 表
pub async fn device_list(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT id, user_id, platform, token FROM x_jpush_device ORDER BY create_time DESC LIMIT 20",
            &[],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("userId".to_string(), Value::String(row.get("user_id"))),
                ("platform".to_string(), Value::String(row.get("platform"))),
                ("token".to_string(), Value::String(row.get("token"))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
            ("data".to_string(), Value::Array(data)),
        ]),
    ))))
}

/// 获取推送模板列表
/// 从数据库查询 x_jpush_template 表
pub async fn template_list(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT id, name, title, content FROM x_jpush_template ORDER BY name LIMIT 20",
            &[],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("name".to_string(), Value::String(row.get("name"))),
                ("title".to_string(), Value::String(row.get("title"))),
                ("content".to_string(), Value::String(row.get("content"))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
            ("data".to_string(), Value::Array(data)),
        ]),
    ))))
}

/// 创建推送核心实体路由
/// 注册以下路由：
/// - /jaxrs/jpush/core/entity/device/list - 设备列表
/// - /jaxrs/jpush/core/entity/template/list - 模板列表
pub fn jpush_core_entity_router(pool: Pool) -> Router {
    Router::new()
        .route("/jaxrs/jpush/core/entity/device/list", get(device_list))
        .route("/jaxrs/jpush/core/entity/template/list", get(template_list))
        .layer(Extension(pool))
}

#[cfg(test)]
mod tests;

pub fn router(_pool: deadpool_postgres::Pool) -> axum::Router {
    axum::Router::new()
        .route("/jpush_core_entity/health", axum::routing::get(|| async { "TODO: jpush_core_entity - real implementation needed" }))
}