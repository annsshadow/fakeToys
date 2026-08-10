use axum::{
    extract::Extension,
    routing::get,
    Json, Router,
};
use sea_orm::{DatabaseConnection, EntityTrait, QueryOrder, QuerySelect};
use serde_json::Value;
use shared::{error::AppError, response::ActionResult};

pub mod entities;
pub mod routes;

// AI 应用实体 - 存储 AI 应用的基本信息
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct AiApp {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub status: String,
}

// AI 模型实体 - 存储 AI 模型配置信息
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct AiModel {
    pub id: String,
    pub name: String,
    pub provider: String,
    pub enabled: bool,
}

// AI 对话实体 - 存储对话记录
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct AiConversation {
    pub id: String,
    pub title: String,
    pub user_id: String,
    pub create_time: String,
}

/// 获取 AI 应用列表
/// 从数据库查询 x_ai_app 表，返回应用列表
pub async fn app_list(
    db: Extension<DatabaseConnection>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let models = entities::ai_app::Entity::find()
        .order_by_desc(entities::ai_app::Column::CreateTime)
        .limit(20)
        .all(&db.0)
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = models
        .iter()
        .map(|m| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(m.id.clone())),
                ("name".to_string(), Value::String(m.name.clone())),
                (
                    "description".to_string(),
                    m.description
                        .clone()
                        .map(Value::String)
                        .unwrap_or(Value::Null),
                ),
                ("status".to_string(), Value::String(m.status.clone())),
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

/// 获取 AI 模型列表
/// 从数据库查询 x_ai_model 表，返回模型列表
pub async fn model_list(
    db: Extension<DatabaseConnection>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let models = entities::ai_model::Entity::find()
        .order_by_asc(entities::ai_model::Column::Name)
        .limit(20)
        .all(&db.0)
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = models
        .iter()
        .map(|m| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(m.id.clone())),
                ("name".to_string(), Value::String(m.name.clone())),
                ("provider".to_string(), Value::String(m.provider.clone())),
                ("enabled".to_string(), Value::Bool(m.enabled)),
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

/// 获取 AI 对话列表
/// 从数据库查询 x_ai_conversation 表，返回对话列表
pub async fn conversation_list(
    db: Extension<DatabaseConnection>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let models = entities::ai_conversation::Entity::find()
        .order_by_desc(entities::ai_conversation::Column::CreateTime)
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
                    "userId".to_string(),
                    Value::String(m.user_id.clone()),
                ),
                (
                    "createTime".to_string(),
                    Value::String(
                        m.create_time
                            .clone()
                            .map(|dt| dt.to_string())
                            .unwrap_or_default(),
                    ),
                ),
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

/// 创建 AI 核心实体路由
/// 注册以下路由：
/// - /jaxrs/ai/core/entity/app/list - 应用列表
/// - /jaxrs/ai/core/entity/model/list - 模型列表
/// - /jaxrs/ai/core/entity/conversation/list - 对话列表
pub fn ai_core_entity_router(_pool: deadpool_postgres::Pool) -> Router {
    let db = std::panic::catch_unwind(|| {
        tokio::runtime::Handle::current()
            .block_on(shared::db::create_sea_orm_pool())
    })
    .ok()
    .and_then(|r| r.ok());

    let router = Router::new()
        .route("/jaxrs/ai/core/entity/app/list", get(app_list))
        .route("/jaxrs/ai/core/entity/model/list", get(model_list))
        .route(
            "/jaxrs/ai/core/entity/conversation/list",
            get(conversation_list),
        );
    match db {
        Some(conn) => router.layer(Extension(conn)),
        None => router,
    }
}

#[cfg(test)]
mod tests;

pub fn router(pool: deadpool_postgres::Pool) -> axum::Router {
    crate::ai_core_entity_router(pool)
}
