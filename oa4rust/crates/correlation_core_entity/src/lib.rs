use axum::{
    extract::{Extension, Json, Path},
    routing::{delete, get, post},
    Router,
};
use deadpool_postgres::Pool;
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, QueryOrder, QuerySelect, Set};
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

#[axum::debug_handler]
pub async fn create(
    db: Extension<DatabaseConnection>,
    Json(payload): Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let id = uuid::Uuid::new_v4().to_string();
    let source_type = payload.get("sourceType").and_then(|v| v.as_str()).unwrap_or_default().to_string();
    let source_id = payload.get("sourceId").and_then(|v| v.as_str()).unwrap_or_default().to_string();
    let target_type = payload.get("targetType").and_then(|v| v.as_str()).unwrap_or_default().to_string();
    let target_id = payload.get("targetId").and_then(|v| v.as_str()).unwrap_or_default().to_string();
    let weight = payload.get("weight").and_then(|v| v.as_i64()).unwrap_or(0) as i32;

    let active_model = correlation::ActiveModel {
        id: Set(id.clone()),
        source_type: Set(source_type.clone()),
        source_id: Set(source_id.clone()),
        target_type: Set(target_type.clone()),
        target_id: Set(target_id.clone()),
        weight: Set(weight),
        create_time: Set(Some(chrono::Utc::now().naive_utc())),
        deleted_at: Set(None),
    };

    active_model
        .insert(&db.0)
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("id".to_string(), Value::String(id)),
        ("sourceType".to_string(), Value::String(source_type)),
        ("sourceId".to_string(), Value::String(source_id)),
        ("targetType".to_string(), Value::String(target_type)),
        ("targetId".to_string(), Value::String(target_id)),
        ("weight".to_string(), Value::Number(serde_json::Number::from(weight))),
    ])))))
}

pub async fn delete_by_id(
    db: Extension<DatabaseConnection>,
    Path(id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let model = correlation::Entity::find_by_id(&id)
        .one(&db.0)
        .await
        .map_err(|_| AppError::Internal)?;

    match model {
        Some(m) => {
            let active = correlation::ActiveModel {
                id: Set(m.id.clone()),
                source_type: Set(m.source_type.clone()),
                source_id: Set(m.source_id.clone()),
                target_type: Set(m.target_type.clone()),
                target_id: Set(m.target_id.clone()),
                weight: Set(m.weight),
                create_time: Set(m.create_time.clone()),
                deleted_at: Set(Some(chrono::Utc::now().naive_utc())),
            };
            active.update(&db.0).await.map_err(|_| AppError::Internal)?;
            Ok(Json(ActionResult::success(serde_json::json!({"success": true}))))
        }
        None => Ok(Json(ActionResult::error("correlation not found"))),
    }
}

/// 创建关联核心实体路由
/// 注册以下路由：
/// - /jaxrs/correlation/core/entity/list - 关联列表
/// - /jaxrs/correlation/core/entity/list/by/{sourceType}/{sourceId} - 按类型查询关联
/// - /jaxrs/correlation/core/entity/create - 创建关联
/// - /jaxrs/correlation/core/entity/delete/{id} - 删除关联
pub fn correlation_core_entity_router(_pool: Pool) -> Router {
    let db = std::panic::catch_unwind(|| {
        tokio::runtime::Handle::current()
            .block_on(shared::db::create_sea_orm_pool())
    })
    .ok()
    .and_then(|r| r.ok());

    let router = Router::new()
        .route("/jaxrs/correlation/core/entity/list", get(list))
        .route(
            "/jaxrs/correlation/core/entity/list/by/{sourceType}/{sourceId}",
            get(list_by_source),
        )
        .route("/jaxrs/correlation/core/entity/create", post(create))
        .route("/jaxrs/correlation/core/entity/delete/{id}", delete(delete_by_id));
    match db {
        Some(conn) => router.layer(Extension(conn)),
        None => router,
    }
}

#[cfg(test)]
mod tests;

pub fn router(pool: deadpool_postgres::Pool) -> axum::Router {
    crate::correlation_core_entity_router(pool)
}
