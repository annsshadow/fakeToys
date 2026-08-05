use axum::{
    extract::Extension,
    routing::get,
    Json, Router,
};
use deadpool_postgres::Pool;
use serde_json::Value;
use shared::{error::AppError, response::ActionResult};

pub mod routes;

// 思维导图文件夹实体
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct MindFolder {
    pub id: String,
    pub name: String,
    pub parent_id: Option<String>,
    pub order_number: i32,
}

// 思维导图版本实体
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct MindVersion {
    pub id: String,
    pub mind_id: String,
    pub name: String,
    pub file_version: i32,
}

/// 获取思维导图列表
/// 从数据库查询 x_mind_base_info 表
pub async fn list(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT id, name, folder_id, description, creator FROM x_mind_base_info ORDER BY create_time DESC LIMIT 20",
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
                ("folderId".to_string(), Value::String(row.get("folder_id"))),
                (
                    "description".to_string(),
                    row.get::<_, Option<String>>("description")
                        .map(Value::String)
                        .unwrap_or(Value::Null),
                ),
                ("creator".to_string(), Value::String(row.get("creator"))),
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

/// 获取思维导图文件夹列表
/// 从数据库查询 x_mind_folder_info 表
pub async fn folder_list(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT id, name, parent_id, order_number, description, creator FROM x_mind_folder_info ORDER BY order_number LIMIT 20",
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
                    "parentId".to_string(),
                    row.get::<_, Option<String>>("parent_id")
                        .map(Value::String)
                        .unwrap_or(Value::Null),
                ),
                ("orderNumber".to_string(), Value::Number(serde_json::Number::from(row.get::<_, i32>("order_number")))),
                (
                    "description".to_string(),
                    row.get::<_, Option<String>>("description")
                        .map(Value::String)
                        .unwrap_or(Value::Null),
                ),
                ("creator".to_string(), Value::String(row.get("creator"))),
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

/// 获取思维导图版本列表
/// 从数据库查询 x_mind_version_info 表
pub async fn version_list(
    pool: Extension<Pool>,
    axum::extract::Path(mind_id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT id, mind_id, name, folder_id, description, creator, file_version, create_time FROM x_mind_version_info WHERE mind_id = $1 ORDER BY create_time DESC LIMIT 20",
            &[&mind_id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("mindId".to_string(), Value::String(row.get("mind_id"))),
                ("name".to_string(), Value::String(row.get("name"))),
                ("folderId".to_string(), Value::String(row.get("folder_id"))),
                (
                    "description".to_string(),
                    row.get::<_, Option<String>>("description")
                        .map(Value::String)
                        .unwrap_or(Value::Null),
                ),
                ("creator".to_string(), Value::String(row.get("creator"))),
                ("fileVersion".to_string(), Value::Number(serde_json::Number::from(row.get::<_, i32>("file_version")))),
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

/// 创建思维导图核心实体路由
/// 注册以下路由：
/// - /jaxrs/mind/core/entity/list - 思维导图列表
/// - /jaxrs/mind/core/entity/folder/list - 文件夹列表
/// - /jaxrs/mind/core/entity/version/list/{mindId} - 版本列表
pub fn mind_core_entity_router(pool: Pool) -> Router {
    Router::new()
        .route("/jaxrs/mind/core/entity/list", get(list))
        .route("/jaxrs/mind/core/entity/folder/list", get(folder_list))
        .route("/jaxrs/mind/core/entity/version/list/{mindId}", get(version_list))
        .layer(Extension(pool))
}

#[cfg(test)]
mod tests;

pub fn router(_pool: deadpool_postgres::Pool) -> axum::Router {
    axum::Router::new()
        .route("/mind_core_entity/health", axum::routing::get(|| async { "TODO: mind_core_entity - real implementation needed" }))
}