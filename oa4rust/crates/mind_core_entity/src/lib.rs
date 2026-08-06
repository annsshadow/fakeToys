use axum::{
    extract::{Extension, Json, Path},
    routing::{get, post, delete},
    Router,
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

#[axum::debug_handler]
pub async fn create_mind(
    pool: Extension<Pool>,
    Json(payload): Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let id = uuid::Uuid::new_v4().to_string();
    let name = payload.get("name").and_then(|v| v.as_str()).unwrap_or_default().to_string();
    let folder_id = payload.get("folderId").and_then(|v| v.as_str()).unwrap_or_default().to_string();
    let description = payload.get("description").and_then(|v| v.as_str()).map(|s| s.to_string());
    let creator = payload.get("creator").and_then(|v| v.as_str()).unwrap_or("system").to_string();

    client
        .execute(
            "INSERT INTO x_mind_base_info (id, name, folder_id, description, creator, create_time) VALUES ($1, $2, $3, $4, $5, NOW())",
            &[&id, &name, &folder_id, &description, &creator],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("id".to_string(), Value::String(id)),
        ("name".to_string(), Value::String(name)),
        ("folderId".to_string(), Value::String(folder_id)),
    ])))))
}

#[axum::debug_handler]
pub async fn update_mind(
    pool: Extension<Pool>,
    Path(id): Path<String>,
    Json(payload): Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_one(
            "SELECT name, folder_id, description FROM x_mind_base_info WHERE id = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::NotFound)?;

    let name = payload.get("name").and_then(|v| v.as_str()).map(|s| s.to_string()).unwrap_or_else(|| row.get("name"));
    let folder_id = payload.get("folderId").and_then(|v| v.as_str()).map(|s| s.to_string()).unwrap_or_else(|| row.get("folder_id"));
    let description = payload.get("description").and_then(|v| v.as_str()).map(|s| s.to_string()).or_else(|| row.get::<_, Option<String>>("description")).unwrap_or_default();

    client
        .execute(
            "UPDATE x_mind_base_info SET name = $1, folder_id = $2, description = $3 WHERE id = $4",
            &[&name, &folder_id, &description, &id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("id".to_string(), Value::String(id)),
        ("updated".to_string(), Value::Bool(true)),
    ])))))
}

#[axum::debug_handler]
pub async fn delete_mind(
    pool: Extension<Pool>,
    Path(id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let result = client
        .execute("DELETE FROM x_mind_base_info WHERE id = $1", &[&id])
        .await
        .map_err(|_| AppError::Internal)?;

    if result == 0 {
        return Ok(Json(ActionResult::error("mind not found")));
    }

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("id".to_string(), Value::String(id)),
        ("deleted".to_string(), Value::Bool(true)),
    ])))))
}

#[axum::debug_handler]
pub async fn create_folder(
    pool: Extension<Pool>,
    Json(payload): Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let id = uuid::Uuid::new_v4().to_string();
    let name = payload.get("name").and_then(|v| v.as_str()).unwrap_or_default().to_string();
    let parent_id = payload.get("parentId").and_then(|v| v.as_str()).map(|s| s.to_string());
    let order_number = payload.get("orderNumber").and_then(|v| v.as_i64()).unwrap_or(0) as i32;
    let description = payload.get("description").and_then(|v| v.as_str()).map(|s| s.to_string());
    let creator = payload.get("creator").and_then(|v| v.as_str()).unwrap_or("system").to_string();

    client
        .execute(
            "INSERT INTO x_mind_folder_info (id, name, parent_id, order_number, description, creator) VALUES ($1, $2, $3, $4, $5, $6)",
            &[&id, &name, &parent_id, &order_number, &description, &creator],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("id".to_string(), Value::String(id)),
        ("name".to_string(), Value::String(name)),
        ("orderNumber".to_string(), Value::Number(serde_json::Number::from(order_number))),
    ])))))
}

#[axum::debug_handler]
pub async fn update_folder(
    pool: Extension<Pool>,
    Path(id): Path<String>,
    Json(payload): Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_one(
            "SELECT name, parent_id, order_number, description FROM x_mind_folder_info WHERE id = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::NotFound)?;

    let name = payload.get("name").and_then(|v| v.as_str()).map(|s| s.to_string()).unwrap_or_else(|| row.get("name"));
    let parent_id = payload.get("parentId").and_then(|v| v.as_str()).map(|s| s.to_string()).or_else(|| row.get::<_, Option<String>>("parent_id")).unwrap_or_default();
    let order_number = payload.get("orderNumber").and_then(|v| v.as_i64()).map(|i| i as i32).unwrap_or_else(|| row.get::<_, i32>("order_number"));
    let description = payload.get("description").and_then(|v| v.as_str()).map(|s| s.to_string()).or_else(|| row.get::<_, Option<String>>("description")).unwrap_or_default();

    client
        .execute(
            "UPDATE x_mind_folder_info SET name = $1, parent_id = $2, order_number = $3, description = $4 WHERE id = $5",
            &[&name, &parent_id, &order_number, &description, &id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("id".to_string(), Value::String(id)),
        ("updated".to_string(), Value::Bool(true)),
    ])))))
}

#[axum::debug_handler]
pub async fn delete_folder(
    pool: Extension<Pool>,
    Path(id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let result = client
        .execute("DELETE FROM x_mind_folder_info WHERE id = $1", &[&id])
        .await
        .map_err(|_| AppError::Internal)?;

    if result == 0 {
        return Ok(Json(ActionResult::error("folder not found")));
    }

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("id".to_string(), Value::String(id)),
        ("deleted".to_string(), Value::Bool(true)),
    ])))))
}

#[axum::debug_handler]
pub async fn create_version(
    pool: Extension<Pool>,
    Json(payload): Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let id = uuid::Uuid::new_v4().to_string();
    let mind_id = payload.get("mindId").and_then(|v| v.as_str()).unwrap_or_default().to_string();
    let name = payload.get("name").and_then(|v| v.as_str()).unwrap_or_default().to_string();
    let folder_id = payload.get("folderId").and_then(|v| v.as_str()).unwrap_or_default().to_string();
    let description = payload.get("description").and_then(|v| v.as_str()).map(|s| s.to_string());
    let creator = payload.get("creator").and_then(|v| v.as_str()).unwrap_or("system").to_string();
    let creator_unit = payload.get("creatorUnit").and_then(|v| v.as_str()).unwrap_or_default().to_string();
    let file_version = payload.get("fileVersion").and_then(|v| v.as_i64()).unwrap_or(1) as i32;
    let shared = payload.get("shared").and_then(|v| v.as_bool()).unwrap_or(false);

    client
        .execute(
            "INSERT INTO x_mind_version_info (id, mind_id, name, folder_id, description, creator, creator_unit, file_version, shared, create_time, update_time) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, NOW(), NOW())",
            &[&id, &mind_id, &name, &folder_id, &description, &creator, &creator_unit, &file_version, &shared],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("id".to_string(), Value::String(id)),
        ("mindId".to_string(), Value::String(mind_id)),
        ("name".to_string(), Value::String(name)),
        ("fileVersion".to_string(), Value::Number(serde_json::Number::from(file_version))),
    ])))))
}

/// 创建思维导图核心实体路由
/// 注册以下路由：
/// - /jaxrs/mind/core/entity/list - 思维导图列表
/// - /jaxrs/mind/core/entity/folder/list - 文件夹列表
/// - /jaxrs/mind/core/entity/version/list/{mindId} - 版本列表
/// - /jaxrs/mind/core/entity/mind - 创建思维导图
/// - /jaxrs/mind/core/entity/mind/{id} - 更新/删除思维导图
/// - /jaxrs/mind/core/entity/folder - 创建文件夹
/// - /jaxrs/mind/core/entity/folder/{id} - 更新/删除文件夹
/// - /jaxrs/mind/core/entity/version - 创建版本
pub fn mind_core_entity_router(pool: Pool) -> Router {
    Router::new()
        .route("/jaxrs/mind/core/entity/list", get(list))
        .route("/jaxrs/mind/core/entity/folder/list", get(folder_list))
        .route("/jaxrs/mind/core/entity/version/list/{mindId}", get(version_list))
        .route("/jaxrs/mind/core/entity/mind", post(create_mind))
        .route("/jaxrs/mind/core/entity/mind/{id}", post(update_mind))
        .route("/jaxrs/mind/core/entity/mind/{id}", delete(delete_mind))
        .route("/jaxrs/mind/core/entity/folder", post(create_folder))
        .route("/jaxrs/mind/core/entity/folder/{id}", post(update_folder))
        .route("/jaxrs/mind/core/entity/folder/{id}", delete(delete_folder))
        .route("/jaxrs/mind/core/entity/version", post(create_version))
        .layer(Extension(pool))
}

#[cfg(test)]
mod tests;

pub fn router(_pool: deadpool_postgres::Pool) -> axum::Router {
    axum::Router::new()
        .route("/mind_core_entity/health", axum::routing::get(|| async { "TODO: mind_core_entity - real implementation needed" }))
}