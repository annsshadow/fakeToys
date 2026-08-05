use axum::{
    extract::Extension,
    routing::get,
    Json, Router,
};
use deadpool_postgres::Pool;
use serde_json::Value;
use shared::{error::AppError, response::ActionResult};

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
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT id, name, description, status FROM x_ai_app ORDER BY create_time DESC LIMIT 20",
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
                (
                    "description".to_string(),
                    row.get::<_, Option<String>>("description")
                        .map(Value::String)
                        .unwrap_or(Value::Null),
                ),
                ("status".to_string(), Value::String(row.get("status"))),
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

/// 获取 AI 模型列表
/// 从数据库查询 x_ai_model 表，返回模型列表
pub async fn model_list(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT id, name, provider, enabled FROM x_ai_model ORDER BY name LIMIT 20",
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
                ("provider".to_string(), Value::String(row.get("provider"))),
                ("enabled".to_string(), Value::Bool(row.get("enabled"))),
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

/// 获取 AI 对话列表
/// 从数据库查询 x_ai_conversation 表，返回对话列表
pub async fn conversation_list(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT id, title, user_id, create_time FROM x_ai_conversation ORDER BY create_time DESC LIMIT 20",
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
                ("userId".to_string(), Value::String(row.get("user_id"))),
                ("createTime".to_string(), Value::String(row.get("create_time"))),
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

/// 创建 AI 核心实体路由
/// 注册以下路由：
/// - /jaxrs/ai/core/entity/app/list - 应用列表
/// - /jaxrs/ai/core/entity/model/list - 模型列表
/// - /jaxrs/ai/core/entity/conversation/list - 对话列表
pub fn ai_core_entity_router(pool: Pool) -> Router {
    Router::new()
        .route("/jaxrs/ai/core/entity/app/list", get(app_list))
        .route("/jaxrs/ai/core/entity/model/list", get(model_list))
        .route("/jaxrs/ai/core/entity/conversation/list", get(conversation_list))
        .layer(Extension(pool))
}

#[cfg(test)]
mod tests;

pub fn router(_pool: deadpool_postgres::Pool) -> axum::Router {
    axum::Router::new()
        .route("/ai_core_entity/health", axum::routing::get(|| async { "TODO: ai_core_entity - real implementation needed" }))
}