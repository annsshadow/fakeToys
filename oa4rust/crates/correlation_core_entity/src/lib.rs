use axum::{
    extract::Extension,
    routing::get,
    Json, Router,
};
use deadpool_postgres::Pool;
use serde_json::Value;
use shared::{error::AppError, response::ActionResult};

pub mod routes;

// 关联实体 - 表示两个对象之间的关系
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct Correlation {
    pub id: String,
    pub source_type: String,
    pub source_id: String,
    pub target_type: String,
    pub target_id: String,
    pub weight: i32,
}

/// 获取关联列表
/// 从数据库查询 x_correlation 表
pub async fn list(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT id, source_type, source_id, target_type, target_id, weight FROM x_correlation ORDER BY create_time DESC LIMIT 20",
            &[],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("sourceType".to_string(), Value::String(row.get("source_type"))),
                ("sourceId".to_string(), Value::String(row.get("source_id"))),
                ("targetType".to_string(), Value::String(row.get("target_type"))),
                ("targetId".to_string(), Value::String(row.get("target_id"))),
                ("weight".to_string(), Value::Number(serde_json::Number::from(row.get::<_, i32>("weight")))),
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

/// 按类型查询关联
/// 根据 source_type 和 source_id 查询相关关联
pub async fn list_by_source(
    pool: Extension<Pool>,
    axum::extract::Path((source_type, source_id)): axum::extract::Path<(String, String)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT id, source_type, source_id, target_type, target_id, weight FROM x_correlation WHERE source_type = $1 AND source_id = $2 ORDER BY weight DESC LIMIT 20",
            &[&source_type, &source_id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("sourceType".to_string(), Value::String(row.get("source_type"))),
                ("sourceId".to_string(), Value::String(row.get("source_id"))),
                ("targetType".to_string(), Value::String(row.get("target_type"))),
                ("targetId".to_string(), Value::String(row.get("target_id"))),
                ("weight".to_string(), Value::Number(serde_json::Number::from(row.get::<_, i32>("weight")))),
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

/// 创建关联核心实体路由
/// 注册以下路由：
/// - /jaxrs/correlation/core/entity/list - 关联列表
/// - /jaxrs/correlation/core/entity/list/by/{sourceType}/{sourceId} - 按类型查询关联
pub fn correlation_core_entity_router(pool: Pool) -> Router {
    Router::new()
        .route("/jaxrs/correlation/core/entity/list", get(list))
        .route("/jaxrs/correlation/core/entity/list/by/{sourceType}/{sourceId}", get(list_by_source))
        .layer(Extension(pool))
}

#[cfg(test)]
mod tests;

pub fn router(_pool: deadpool_postgres::Pool) -> axum::Router {
    axum::Router::new()
        .route("/correlation_core_entity/health", axum::routing::get(|| async { "TODO: correlation_core_entity - real implementation needed" }))
}