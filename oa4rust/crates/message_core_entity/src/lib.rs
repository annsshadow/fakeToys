use axum::{
    extract::{Extension, Path},
    routing::get,
    Json, Router,
};
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder, QuerySelect};
use serde_json::Value;
use shared::{error::AppError, response::ActionResult};

pub mod entities;
pub mod routes;

use entities::message;

/// 获取消息列表
pub async fn list(
    db: Extension<DatabaseConnection>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let models = message::Entity::find()
        .order_by_desc(message::Column::CreateTime)
        .limit(20)
        .all(&db.0)
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = models
        .iter()
        .map(|m| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(m.id.clone())),
                ("title".to_string(), Value::String(m.title.clone())),
                (
                    "body".to_string(),
                    m.body.clone().map(Value::String).unwrap_or(Value::Null),
                ),
                ("type".to_string(), Value::String(m.r#type.clone())),
                ("consumer".to_string(), Value::String(m.consumer.clone())),
                ("isRead".to_string(), Value::Bool(m.is_read)),
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

/// 按消费类型查询消息
pub async fn list_by_consume(
    db: Extension<DatabaseConnection>,
    Path(consume): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let models = message::Entity::find()
        .filter(message::Column::Consumer.eq(&consume))
        .order_by_desc(message::Column::CreateTime)
        .limit(20)
        .all(&db.0)
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = models
        .iter()
        .map(|m| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(m.id.clone())),
                ("title".to_string(), Value::String(m.title.clone())),
                (
                    "body".to_string(),
                    m.body.clone().map(Value::String).unwrap_or(Value::Null),
                ),
                ("type".to_string(), Value::String(m.r#type.clone())),
                ("consumer".to_string(), Value::String(m.consumer.clone())),
                ("isRead".to_string(), Value::Bool(m.is_read)),
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

/// 获取未读消息数量
pub async fn unread_count(
    db: Extension<DatabaseConnection>,
    Path(consume): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let count: u64 = message::Entity::find()
        .filter(message::Column::Consumer.eq(&consume))
        .filter(message::Column::IsRead.eq(false))
        .count(&db.0)
        .await
        .map_err(|_| AppError::Internal)?;

    let data = Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(count))),
        ("consumer".to_string(), Value::String(consume)),
    ]));

    Ok(Json(ActionResult::success(data)))
}

/// 创建消息核心实体路由
pub fn message_core_entity_router(_pool: deadpool_postgres::Pool) -> Router {
    let db = tokio::runtime::Handle::current()
        .block_on(shared::db::create_sea_orm_pool())
        .expect("failed to create sea-orm connection");

    Router::new()
        .route("/jaxrs/message/core/entity/list", get(list))
        .route(
            "/jaxrs/message/core/entity/list/by/{consume}",
            get(list_by_consume),
        )
        .route(
            "/jaxrs/message/core/entity/unread/count/{consume}",
            get(unread_count),
        )
        .layer(Extension(db))
}

#[cfg(test)]
mod tests;

pub fn router(pool: deadpool_postgres::Pool) -> axum::Router {
    crate::message_core_entity_router(pool)
}
