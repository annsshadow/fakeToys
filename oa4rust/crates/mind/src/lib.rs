use axum::{
    extract::Extension,
    Json, Router,
};
use deadpool_postgres::Pool;
use serde_json::Value;
use shared::{error::AppError, response::ActionResult};

pub mod routes;

#[cfg(test)]
mod tests;

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

pub fn router(_pool: deadpool_postgres::Pool) -> axum::Router {
    axum::Router::new()
        .route("/mind/health", axum::routing::get(|| async { "TODO: mind - real implementation needed" }))
}