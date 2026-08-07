use axum::{
    extract::Extension,
    routing::get,
    Json, Router,
};
use deadpool_postgres::Pool;
use serde_json::Value;
use shared::{error::AppError, response::ActionResult};

pub mod routes;

// 热图实体
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct HotPic {
    pub id: String,
    pub application: String,
    pub info_id: String,
    pub title: String,
    pub base64: String,
}

/// 获取热图列表
/// 从数据库查询 x_hotpic 表
pub async fn list(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT id, application, info_id, title FROM x_hotpic ORDER BY create_time DESC LIMIT 20",
            &[],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("application".to_string(), Value::String(row.get("application"))),
                ("infoId".to_string(), Value::String(row.get("info_id"))),
                ("title".to_string(), Value::String(row.get("title"))),
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

/// 根据应用和信息ID获取热图
/// 查询 x_hotpic 表中指定应用和信息ID的热图
pub async fn list_by_app_and_info(
    pool: Extension<Pool>,
    axum::extract::Path((application, info_id)): axum::extract::Path<(String, String)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT id, application, info_id, title FROM x_hotpic WHERE application = $1 AND info_id = $2 ORDER BY create_time DESC LIMIT 20",
            &[&application, &info_id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("application".to_string(), Value::String(row.get("application"))),
                ("infoId".to_string(), Value::String(row.get("info_id"))),
                ("title".to_string(), Value::String(row.get("title"))),
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

/// 检查热图是否存在
/// 验证指定应用和信息ID的热图是否存在
pub async fn exists_check(
    pool: Extension<Pool>,
    axum::extract::Path((application, info_id)): axum::extract::Path<(String, String)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let count: i64 = client
        .query_one(
            "SELECT COUNT(*) FROM x_hotpic WHERE application = $1 AND info_id = $2",
            &[&application, &info_id],
        )
        .await
        .map_err(|_| AppError::Internal)?
        .get("count");

    let data = Value::Object(serde_json::Map::from_iter([
        ("allExists".to_string(), Value::Bool(count > 0)),
        ("count".to_string(), Value::Number(serde_json::Number::from(count))),
    ]));

    Ok(Json(ActionResult::success(data)))
}

/// 创建热图核心实体路由
/// 注册以下路由：
/// - /jaxrs/hotpic/core/entity/list - 热图列表
/// - /jaxrs/hotpic/core/entity/list/by/{application}/{infoId} - 按条件查询
/// - /jaxrs/hotpic/core/entity/exists/check/{application}/{infoId} - 检查存在
pub fn hotpic_core_entity_router(pool: Pool) -> Router {
    Router::new()
        .route("/jaxrs/hotpic/core/entity/list", get(list))
        .route("/jaxrs/hotpic/core/entity/list/by/{application}/{infoId}", get(list_by_app_and_info))
        .route("/jaxrs/hotpic/core/entity/exists/check/{application}/{infoId}", get(exists_check))
        .layer(Extension(pool))
}

#[cfg(test)]
mod tests;

pub fn router(pool: deadpool_postgres::Pool) -> axum::Router {
    crate::hotpic_core_entity_router(pool)
}
