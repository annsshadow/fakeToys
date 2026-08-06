use axum::{
    extract::Extension,
    routing::get,
    Json, Router,
};
use deadpool_postgres::Pool;
use serde_json::Value;
use shared::{error::AppError, response::ActionResult};

pub mod routes;

// 消息实体
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct Message {
    pub id: String,
    pub title: String,
    pub body: Option<String>,
    pub r#type: String,
    pub consumer: String,
    pub is_read: bool,
}

/// 获取消息列表
/// 从数据库查询 x_message 表
pub async fn list(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT id, title, body, type, consumer, is_read FROM x_message ORDER BY create_time DESC LIMIT 20",
            &[],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("title".to_string(), Value::String(row.get("title"))),
                (
                    "body".to_string(),
                    row.get::<_, Option<String>>("body")
                        .map(Value::String)
                        .unwrap_or(Value::Null),
                ),
                ("type".to_string(), Value::String(row.get("type"))),
                ("consumer".to_string(), Value::String(row.get("consumer"))),
                ("isRead".to_string(), Value::Bool(row.get("is_read"))),
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

/// 按消费类型查询消息
/// 根据消费类型筛选消息
pub async fn list_by_consume(
    pool: Extension<Pool>,
    axum::extract::Path(consume): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT id, title, body, type, consumer, is_read FROM x_message WHERE consumer = $1 ORDER BY create_time DESC LIMIT 20",
            &[&consume],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("title".to_string(), Value::String(row.get("title"))),
                (
                    "body".to_string(),
                    row.get::<_, Option<String>>("body")
                        .map(Value::String)
                        .unwrap_or(Value::Null),
                ),
                ("type".to_string(), Value::String(row.get("type"))),
                ("consumer".to_string(), Value::String(row.get("consumer"))),
                ("isRead".to_string(), Value::Bool(row.get("is_read"))),
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

/// 获取未读消息数量
/// 统计指定消费类型的未读消息数量
pub async fn unread_count(
    pool: Extension<Pool>,
    axum::extract::Path(consume): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let count: i64 = client
        .query_one(
            "SELECT COUNT(*) FROM x_message WHERE consumer = $1 AND is_read = false",
            &[&consume],
        )
        .await
        .map_err(|_| AppError::Internal)?
        .get("count");

    let data = Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(count))),
        ("consumer".to_string(), Value::String(consume)),
    ]));

    Ok(Json(ActionResult::success(data)))
}

/// 创建消息核心实体路由
/// 注册以下路由：
/// - /jaxrs/message/core/entity/list - 消息列表
/// - /jaxrs/message/core/entity/list/by/{consume} - 按类型查询
/// - /jaxrs/message/core/entity/unread/count/{consume} - 未读数量
pub fn message_core_entity_router(pool: Pool) -> Router {
    Router::new()
        .route("/jaxrs/message/core/entity/list", get(list))
        .route("/jaxrs/message/core/entity/list/by/{consume}", get(list_by_consume))
        .route("/jaxrs/message/core/entity/unread/count/{consume}", get(unread_count))
        .layer(Extension(pool))
}

#[cfg(test)]
mod tests;

pub fn router(pool: deadpool_postgres::Pool) -> axum::Router {
    routes::router(pool)
}