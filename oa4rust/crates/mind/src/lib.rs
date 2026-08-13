use axum::{
    extract::{Extension, Json, Path},
    Router,
};
use deadpool_postgres::Pool;
use serde_json::Value;
use shared::{error::AppError, response::ActionResult};

pub mod routes;

#[cfg(test)]
mod tests;
#[cfg(test)]
mod tests_generated;


pub fn mind_router() -> Router {
    Router::new()
        .merge(routes::mind_routes())
}

#[axum::debug_handler]
pub async fn get_mind_with_id(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let row = client
        .query_one(
            "SELECT id, name, folder_id, icon, description, creator, creator_unit, shared, file_version FROM mind_base_info WHERE id = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::NotFound)?;

    let result = ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("id".to_string(), Value::String(row.get("id"))),
        ("name".to_string(), Value::String(row.get("name"))),
        ("folderId".to_string(), Value::String(row.get("folder_id"))),
        ("icon".to_string(), Value::String(row.get::<_, Option<String>>("icon").unwrap_or_default())),
        ("description".to_string(), Value::String(row.get::<_, Option<String>>("description").unwrap_or_default())),
        ("creator".to_string(), Value::String(row.get("creator"))),
        ("creatorUnit".to_string(), Value::String(row.get("creator_unit"))),
        ("shared".to_string(), Value::Bool(row.get("shared"))),
        ("fileVersion".to_string(), Value::Number(serde_json::Number::from(row.get::<_, i32>("file_version")))),
    ])));

    Ok(Json(result))
}

#[axum::debug_handler]
pub async fn list_my_folders(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT id, name, parent_id, order_number, description, creator, creator_unit FROM mind_folder_info ORDER BY order_number",
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
                ("parentId".to_string(), Value::String(row.get("parent_id"))),
                ("orderNumber".to_string(), Value::Number(serde_json::Number::from(row.get::<_, i32>("order_number")))),
                ("description".to_string(), Value::String(row.get::<_, Option<String>>("description").unwrap_or_default())),
                ("creator".to_string(), Value::String(row.get("creator"))),
                ("creatorUnit".to_string(), Value::String(row.get("creator_unit"))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

#[axum::debug_handler]
pub async fn list_versions_with_mind_id(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT id, mind_id, name, folder_id, description, creator, creator_unit, file_version, shared, create_time, update_time FROM mind_version_info WHERE mind_id = $1 ORDER BY create_time DESC",
            &[&id],
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
                ("description".to_string(), Value::String(row.get::<_, Option<String>>("description").unwrap_or_default())),
                ("creator".to_string(), Value::String(row.get("creator"))),
                ("creatorUnit".to_string(), Value::String(row.get("creator_unit"))),
                ("fileVersion".to_string(), Value::Number(serde_json::Number::from(row.get::<_, i32>("file_version")))),
                ("shared".to_string(), Value::Bool(row.get("shared"))),
                ("createTime".to_string(), Value::String(row.get::<_, String>("create_time"))),
                ("updateTime".to_string(), Value::String(row.get::<_, String>("update_time"))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
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
    let icon = payload.get("icon").and_then(|v| v.as_str()).map(|s| s.to_string());
    let description = payload.get("description").and_then(|v| v.as_str()).map(|s| s.to_string());
    let creator = payload.get("creator").and_then(|v| v.as_str()).unwrap_or("system").to_string();
    let creator_unit = payload.get("creatorUnit").and_then(|v| v.as_str()).unwrap_or_default().to_string();
    let shared = payload.get("shared").and_then(|v| v.as_bool()).unwrap_or(false);
    let file_version = payload.get("fileVersion").and_then(|v| v.as_i64()).unwrap_or(1) as i32;

    client
        .execute(
            "INSERT INTO mind_base_info (id, name, folder_id, icon, description, creator, creator_unit, shared, file_version, create_time, update_time) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, NOW(), NOW())",
            &[&id, &name, &folder_id, &icon, &description, &creator, &creator_unit, &shared, &file_version],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("id".to_string(), Value::String(id)),
        ("name".to_string(), Value::String(name)),
        ("folderId".to_string(), Value::String(folder_id)),
        ("shared".to_string(), Value::Bool(shared)),
        ("fileVersion".to_string(), Value::Number(serde_json::Number::from(file_version))),
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
            "SELECT name, folder_id, icon, description, shared, file_version FROM mind_base_info WHERE id = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::NotFound)?;

    let name = payload.get("name").and_then(|v| v.as_str()).map(|s| s.to_string()).unwrap_or_else(|| row.get("name"));
    let folder_id = payload.get("folderId").and_then(|v| v.as_str()).map(|s| s.to_string()).unwrap_or_else(|| row.get("folder_id"));
    let icon = payload.get("icon").and_then(|v| v.as_str()).map(|s| s.to_string()).or_else(|| row.get::<_, Option<String>>("icon")).unwrap_or_default();
    let description = payload.get("description").and_then(|v| v.as_str()).map(|s| s.to_string()).or_else(|| row.get::<_, Option<String>>("description")).unwrap_or_default();
    let shared = payload.get("shared").and_then(|v| v.as_bool()).unwrap_or_else(|| row.get("shared"));
    let file_version = payload.get("fileVersion").and_then(|v| v.as_i64()).map(|i| i as i32).unwrap_or_else(|| row.get::<_, i32>("file_version"));

    client
        .execute(
            "UPDATE mind_base_info SET name = $1, folder_id = $2, icon = $3, description = $4, shared = $5, file_version = $6, update_time = NOW() WHERE id = $7",
            &[&name, &folder_id, &icon, &description, &shared, &file_version, &id],
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
        .execute("DELETE FROM mind_base_info WHERE id = $1", &[&id])
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
    let creator_unit = payload.get("creatorUnit").and_then(|v| v.as_str()).unwrap_or_default().to_string();

    client
        .execute(
            "INSERT INTO mind_folder_info (id, name, parent_id, order_number, description, creator, creator_unit) VALUES ($1, $2, $3, $4, $5, $6, $7)",
            &[&id, &name, &parent_id, &order_number, &description, &creator, &creator_unit],
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
            "SELECT name, parent_id, order_number, description FROM mind_folder_info WHERE id = $1",
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
            "UPDATE mind_folder_info SET name = $1, parent_id = $2, order_number = $3, description = $4 WHERE id = $5",
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
        .execute("DELETE FROM mind_folder_info WHERE id = $1", &[&id])
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
            "INSERT INTO mind_version_info (id, mind_id, name, folder_id, description, creator, creator_unit, file_version, shared, create_time, update_time) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, NOW(), NOW())",
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

pub fn router(pool: deadpool_postgres::Pool) -> axum::Router {
    routes::mind_routes().layer(axum::extract::Extension(pool))
}
