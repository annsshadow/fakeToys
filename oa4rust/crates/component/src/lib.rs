use axum::{
    extract::Extension, extract::Path,
    Json,
};
use deadpool_postgres::Pool;
use serde_json::Value;
use shared::{error::AppError, response::ActionResult};

#[derive(Debug, serde::Serialize, Clone)]
pub struct ComponentInfo {
    pub id: String,
    pub name: String,
    pub title: String,
    pub r#type: String,
    pub visible: bool,
    pub order_number: Option<i32>,
    pub path: String,
    pub icon_path: String,
}

pub async fn list_all(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT id, name, title, type, visible, order_number, path, icon_path FROM x_component WHERE deleted_at IS NULL ORDER BY order_number ASC LIMIT 50",
            &[],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            let mut map = serde_json::Map::new();
            map.insert("id".to_string(), Value::String(row.get("id")));
            map.insert("name".to_string(), Value::String(row.get("name")));
            map.insert("title".to_string(), Value::String(row.get("title")));
            map.insert("type".to_string(), Value::String(row.get("type")));
            map.insert("visible".to_string(), Value::Bool(row.get("visible")));
            if let Some(v) = row.get::<_, Option<i32>>("order_number") {
                map.insert("orderNumber".to_string(), serde_json::Number::from(v).into());
            }
            map.insert("path".to_string(), Value::String(row.get("path")));
            map.insert("iconPath".to_string(), Value::String(row.get("icon_path")));
            Value::Object(map)
        })
        .collect();

    let count = data.len() as i64;
    Ok(Json(ActionResult::java_success(Value::Array(data), count, 0)))
}

pub async fn get_component(
    pool: Extension<Pool>,
    Path(id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    eprintln!("DEBUG get_component called with id={}", id);
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT id, name, title, type, visible, order_number, path, icon_path FROM x_component WHERE (id = $1 OR name = $1) AND deleted_at IS NULL",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let mut map = serde_json::Map::new();
            map.insert("id".to_string(), Value::String(row.get("id")));
            map.insert("name".to_string(), Value::String(row.get("name")));
            map.insert("title".to_string(), Value::String(row.get("title")));
            map.insert("type".to_string(), Value::String(row.get("type")));
            map.insert("visible".to_string(), Value::Bool(row.get("visible")));
            if let Some(v) = row.get::<_, Option<i32>>("order_number") {
                map.insert("orderNumber".to_string(), serde_json::Number::from(v).into());
            }
            map.insert("path".to_string(), Value::String(row.get("path")));
            map.insert("iconPath".to_string(), Value::String(row.get("icon_path")));
            Ok(Json(ActionResult::success(Value::Object(map))))
        }
        None => Err(AppError::NotFound),
    }
}

pub async fn count(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_one(
            "SELECT COUNT(*) as cnt FROM x_component WHERE deleted_at IS NULL",
            &[],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    let count: i64 = row.get("cnt");

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(count))),
    ])))))
}

pub mod routes;

pub use routes::component_router;

#[cfg(test)]
mod tests;
#[cfg(test)]
mod tests_generated;


pub fn router(pool: deadpool_postgres::Pool) -> axum::Router {
    crate::component_router(pool)
}
