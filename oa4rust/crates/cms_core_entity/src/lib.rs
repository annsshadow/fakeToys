use axum::{
    extract::Extension,
    routing::get,
    Json, Router,
};
use deadpool_postgres::Pool;
use serde_json::Value;
use shared::{error::AppError, response::ActionResult};

pub mod routes;

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct CategoryInfo {
    pub id: String,
    pub name: String,
    pub app_id: String,
    pub parent_id: Option<String>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct AppInfo {
    pub id: String,
    pub name: String,
    pub flag: String,
    pub category: String,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct AppInfoConfig {
    pub id: String,
    pub app_id: String,
    pub key: String,
    pub value: String,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct CategoryExt {
    pub id: String,
    pub category_id: String,
    pub key: String,
    pub value: String,
}

pub async fn category_list(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT id, name, app_id, parent_id FROM x_cms_category ORDER BY name LIMIT 20",
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
                ("appId".to_string(), Value::String(row.get("app_id"))),
                (
                    "parentId".to_string(),
                    row.get::<_, Option<String>>("parent_id")
                        .map(Value::String)
                        .unwrap_or(Value::Null),
                ),
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

pub async fn app_list(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT id, name, flag, category FROM x_cms_app ORDER BY name LIMIT 20",
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
                ("flag".to_string(), Value::String(row.get("flag"))),
                ("category".to_string(), Value::String(row.get("category"))),
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

pub async fn app_config_list(
    pool: Extension<Pool>,
    axum::extract::Path(app_id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT id, app_id, key, value FROM x_cms_app_config WHERE app_id = $1",
            &[&app_id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("appId".to_string(), Value::String(row.get("app_id"))),
                ("key".to_string(), Value::String(row.get("key"))),
                ("value".to_string(), Value::String(row.get("value"))),
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

pub async fn category_ext_list(
    pool: Extension<Pool>,
    axum::extract::Path(category_id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT id, category_id, key, value FROM x_cms_category_ext WHERE category_id = $1",
            &[&category_id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("categoryId".to_string(), Value::String(row.get("category_id"))),
                ("key".to_string(), Value::String(row.get("key"))),
                ("value".to_string(), Value::String(row.get("value"))),
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

pub fn cms_core_entity_router(pool: Pool) -> Router {
    Router::new()
        .route("/jaxrs/cms/category/list", get(category_list))
        .route("/jaxrs/cms/app/list", get(app_list))
        .route("/jaxrs/cms/app/config/list/{appId}", get(app_config_list))
        .route("/jaxrs/cms/category/ext/list/{categoryId}", get(category_ext_list))
        .layer(Extension(pool))
}

#[cfg(test)]
mod tests;
