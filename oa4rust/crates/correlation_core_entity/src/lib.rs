use axum::{
    extract::Extension,
    routing::get,
    Json, Router,
};
use deadpool_postgres::Pool;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, QueryOrder, QuerySelect};
use serde_json::Value;
use shared::{error::AppError, response::ActionResult};

pub mod entities;
pub mod routes;

use entities::correlation;

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
/// 从数据库查询 x_corr_c_correlation 表
pub async fn list(
    db: Extension<DatabaseConnection>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let models = correlation::Entity::find()
        .order_by_desc(correlation::Column::CreateTime)
        .limit(20)
        .all(&db.0)
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = models
        .iter()
        .map(|m| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(m.id.clone())),
                ("sourceType".to_string(), Value::String(m.source_type.clone())),
                ("sourceId".to_string(), Value::String(m.source_id.clone())),
                ("targetType".to_string(), Value::String(m.target_type.clone())),
                ("targetId".to_string(), Value::String(m.target_id.clone())),
                ("weight".to_string(), Value::Number(serde_json::Number::from(m.weight))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            (
                "count".to_string(),
                Value::Number(serde_json::Number::from(data.len() as i64)),
            ),
            ("data".to_string(), Value::Array(data)),
        ]),
    ))))
}

/// 按类型查询关联
/// 根据 source_type 和 source_id 查询相关关联
pub async fn list_by_source(
    db: Extension<DatabaseConnection>,
    axum::extract::Path((source_type, source_id)): axum::extract::Path<(String, String)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let models = correlation::Entity::find()
        .filter(
            correlation::Column::SourceType.eq(&source_type)
                .and(correlation::Column::SourceId.eq(&source_id)),
        )
        .order_by_desc(correlation::Column::Weight)
        .limit(20)
        .all(&db.0)
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = models
        .iter()
        .map(|m| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(m.id.clone())),
                ("sourceType".to_string(), Value::String(m.source_type.clone())),
                ("sourceId".to_string(), Value::String(m.source_id.clone())),
                ("targetType".to_string(), Value::String(m.target_type.clone())),
                ("targetId".to_string(), Value::String(m.target_id.clone())),
                ("weight".to_string(), Value::Number(serde_json::Number::from(m.weight))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            (
                "count".to_string(),
                Value::Number(serde_json::Number::from(data.len() as i64)),
            ),
            ("data".to_string(), Value::Array(data)),
        ]),
    ))))
}

/// 创建关联核心实体路由
/// 注册以下路由：
/// - /jaxrs/correlation/core/entity/list - 关联列表
/// - /jaxrs/correlation/core/entity/list/by/{sourceType}/{sourceId} - 按类型查询关联
pub fn correlation_core_entity_router(_pool: Pool) -> Router {
    let db = tokio::runtime::Handle::current()
        .block_on(shared::db::create_sea_orm_pool())
        .expect("failed to create sea-orm connection");

    Router::new()
        .route("/jaxrs/correlation/core/entity/list", get(list))
        .route(
            "/jaxrs/correlation/core/entity/list/by/{sourceType}/{sourceId}",
            get(list_by_source),
        )
        .layer(Extension(db))
}

#[cfg(test)]
mod tests;

pub fn router(pool: deadpool_postgres::Pool) -> axum::Router {
    crate::correlation_core_entity_router(pool)
}
