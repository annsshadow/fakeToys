use axum::{
    extract::{Extension, Json},
    routing::{get, post},
    Router,
};
use axum::extract::Path;
use deadpool_postgres::Pool;
use serde_json::Value;
use shared::{error::AppError, response::ActionResult};

pub mod routes;

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct Category {
    pub id: String,
    pub name: String,
    #[serde(rename = "parentId")]
    pub parent_id: Option<String>,
    #[serde(rename = "sortOrder")]
    pub sort_order: i32,
    pub status: String,
    #[serde(rename = "createTime")]
    pub create_time: String,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct Article {
    pub id: String,
    #[serde(rename = "categoryId")]
    pub category_id: String,
    pub title: String,
    pub content: Option<String>,
    #[serde(rename = "authorId")]
    pub author_id: String,
    pub status: String,
    #[serde(rename = "publishTime")]
    pub publish_time: Option<String>,
    #[serde(rename = "createTime")]
    pub create_time: String,
}

pub async fn category_list(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT id, name, parent_id, sort_order, status, create_time FROM CMS_CATEGORY WHERE deleted_at IS NULL ORDER BY sort_order LIMIT 20",
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
                (
                    "sortOrder".to_string(),
                    Value::Number(serde_json::Number::from(row.get::<_, i32>("sort_order"))),
                ),
                ("status".to_string(), Value::String(row.get("status"))),
                (
                    "createTime".to_string(),
                    Value::String(row.get::<_, String>("create_time")),
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

pub async fn category_get(
    pool: Extension<Pool>,
    Path(id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let row = client
        .query_opt(
            "SELECT id, name, parent_id, sort_order, status, create_time FROM CMS_CATEGORY WHERE id = $1 AND deleted_at IS NULL",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let result = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("name".to_string(), Value::String(row.get("name"))),
                (
                    "parentId".to_string(),
                    row.get::<_, Option<String>>("parent_id")
                        .map(Value::String)
                        .unwrap_or(Value::Null),
                ),
                (
                    "sortOrder".to_string(),
                    Value::Number(serde_json::Number::from(row.get::<_, i32>("sort_order"))),
                ),
                ("status".to_string(), Value::String(row.get("status"))),
                (
                    "createTime".to_string(),
                    Value::String(row.get::<_, String>("create_time")),
                ),
            ]));
            Ok(Json(ActionResult::success(result)))
        }
        None => Ok(Json(ActionResult::error("category not found"))),
    }
}

#[axum::debug_handler]
pub async fn category_create(
    pool: Extension<Pool>,
    Json(payload): Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let id = uuid::Uuid::new_v4().to_string();
    let name = payload.get("name").and_then(|v| v.as_str()).unwrap_or_default().to_string();
    let parent_id = payload.get("parentId").and_then(|v| v.as_str()).map(|s| s.to_string());
    let sort_order = payload.get("sortOrder").and_then(|v| v.as_i64()).unwrap_or(0) as i32;
    let status = payload.get("status").and_then(|v| v.as_str()).unwrap_or_else(|| "active").to_string();

    client
        .execute(
            "INSERT INTO CMS_CATEGORY (id, name, parent_id, sort_order, status, create_time) VALUES ($1, $2, $3, $4, $5, NOW())",
            &[&id, &name, &parent_id, &sort_order, &status],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("id".to_string(), Value::String(id)),
        ("name".to_string(), Value::String(name)),
        ("sortOrder".to_string(), Value::Number(serde_json::Number::from(sort_order))),
    ])))))
}

pub async fn article_list(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT id, category_id, title, content, author_id, status, publish_time, create_time FROM CMS_ARTICLE WHERE deleted_at IS NULL ORDER BY create_time DESC LIMIT 20",
            &[],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("categoryId".to_string(), Value::String(row.get("category_id"))),
                ("title".to_string(), Value::String(row.get("title"))),
                (
                    "content".to_string(),
                    row.get::<_, Option<String>>("content")
                        .map(Value::String)
                        .unwrap_or(Value::Null),
                ),
                ("authorId".to_string(), Value::String(row.get("author_id"))),
                ("status".to_string(), Value::String(row.get("status"))),
                (
                    "publishTime".to_string(),
                    row.get::<_, Option<String>>("publish_time")
                        .map(Value::String)
                        .unwrap_or(Value::Null),
                ),
                (
                    "createTime".to_string(),
                    Value::String(row.get::<_, String>("create_time")),
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

pub async fn article_get(
    pool: Extension<Pool>,
    Path(id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let row = client
        .query_opt(
            "SELECT id, category_id, title, content, author_id, status, publish_time, create_time FROM CMS_ARTICLE WHERE id = $1 AND deleted_at IS NULL",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let result = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("categoryId".to_string(), Value::String(row.get("category_id"))),
                ("title".to_string(), Value::String(row.get("title"))),
                (
                    "content".to_string(),
                    row.get::<_, Option<String>>("content")
                        .map(Value::String)
                        .unwrap_or(Value::Null),
                ),
                ("authorId".to_string(), Value::String(row.get("author_id"))),
                ("status".to_string(), Value::String(row.get("status"))),
                (
                    "publishTime".to_string(),
                    row.get::<_, Option<String>>("publish_time")
                        .map(Value::String)
                        .unwrap_or(Value::Null),
                ),
                (
                    "createTime".to_string(),
                    Value::String(row.get::<_, String>("create_time")),
                ),
            ]));
            Ok(Json(ActionResult::success(result)))
        }
        None => Ok(Json(ActionResult::error("article not found"))),
    }
}

#[axum::debug_handler]
pub async fn article_create(
    pool: Extension<Pool>,
    Json(payload): Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let id = uuid::Uuid::new_v4().to_string();
    let category_id = payload.get("categoryId").and_then(|v| v.as_str()).unwrap_or_default().to_string();
    let title = payload.get("title").and_then(|v| v.as_str()).unwrap_or_default().to_string();
    let content = payload.get("content").and_then(|v| v.as_str()).map(|s| s.to_string());
    let author_id = payload.get("authorId").and_then(|v| v.as_str()).unwrap_or_default().to_string();
    let status = payload.get("status").and_then(|v| v.as_str()).unwrap_or_else(|| "draft").to_string();

    client
        .execute(
            "INSERT INTO CMS_ARTICLE (id, category_id, title, content, author_id, status, create_time) VALUES ($1, $2, $3, $4, $5, $6, NOW())",
            &[&id, &category_id, &title, &content, &author_id, &status],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("id".to_string(), Value::String(id)),
        ("title".to_string(), Value::String(title)),
        ("categoryId".to_string(), Value::String(category_id)),
    ])))))
}

pub fn cms_core_entity_router(pool: Pool) -> Router {
    Router::new()
        .route("/jaxrs/cms/category/list", get(category_list))
        .route("/jaxrs/cms/category/{id}", get(category_get))
        .route("/jaxrs/cms/category/create", post(category_create))
        .route("/jaxrs/cms/article/list", get(article_list))
        .route("/jaxrs/cms/article/{id}", get(article_get))
        .route("/jaxrs/cms/article/create", post(article_create))
        .layer(Extension(pool))
}

#[cfg(test)]
mod tests;

pub fn router(pool: deadpool_postgres::Pool) -> axum::Router {
    crate::cms_core_entity_router(pool)
}
