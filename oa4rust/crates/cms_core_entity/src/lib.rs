use axum::{
    extract::{Extension, Json, Path},
    routing::{get, post},
    Router,
};
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, QueryOrder, QuerySelect, Set};
use serde_json::Value;
use shared::{error::AppError, response::ActionResult};

pub mod entities;
pub mod routes;

use entities::{cms_article, cms_category};

pub async fn category_list(
    db: Extension<DatabaseConnection>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let models = cms_category::Entity::find()
        .filter(cms_category::Column::DeletedAt.is_null())
        .order_by_asc(cms_category::Column::SortOrder)
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
                    "\"parentId\"".to_string(),
                    m.parent_id
                        .clone()
                        .map(Value::String)
                        .unwrap_or(Value::Null),
                ),
                (
                    "sortOrder".to_string(),
                    Value::Number(serde_json::Number::from(m.sort_order)),
                ),
                ("status".to_string(), Value::String(m.status.clone())),
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

pub async fn category_get(
    db: Extension<DatabaseConnection>,
    Path(id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let model = cms_category::Entity::find_by_id(&id)
        .filter(cms_category::Column::DeletedAt.is_null())
        .one(&db.0)
        .await
        .map_err(|_| AppError::Internal)?;

    match model {
        Some(m) => {
            let result = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(m.id.clone())),
                ("name".to_string(), Value::String(m.name.clone())),
                (
                    "\"parentId\"".to_string(),
                    m.parent_id
                        .clone()
                        .map(Value::String)
                        .unwrap_or(Value::Null),
                ),
                (
                    "sortOrder".to_string(),
                    Value::Number(serde_json::Number::from(m.sort_order)),
                ),
                ("status".to_string(), Value::String(m.status.clone())),
                (
                    "createTime".to_string(),
                    Value::String(
                        m.create_time
                            .clone()
                            .map(|dt| dt.to_string())
                            .unwrap_or_default(),
                    ),
                ),
            ]));
            Ok(Json(ActionResult::success(result)))
        }
        None => Ok(Json(ActionResult::error("category not found"))),
    }
}

#[axum::debug_handler]
pub async fn category_create(
    db: Extension<DatabaseConnection>,
    Json(payload): Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let id = uuid::Uuid::new_v4().to_string();
    let name = payload.get("name").and_then(|v| v.as_str()).unwrap_or_default().to_string();
    let parent_id = payload.get("\"parentId\"").and_then(|v| v.as_str()).map(|s| s.to_string());
    let sort_order = payload.get("sortOrder").and_then(|v| v.as_i64()).unwrap_or(0) as i32;
    let status = payload
        .get("status")
        .and_then(|v| v.as_str())
        .unwrap_or_else(|| "active")
        .to_string();
    let _create_time = chrono::Utc::now();

    let active_model = cms_category::ActiveModel {
        id: Set(id.clone()),
        name: Set(name.clone()),
        parent_id: Set(parent_id),
        sort_order: Set(sort_order),
        status: Set(status.clone()),
        create_time: Set(Some(chrono::Utc::now().naive_utc())),
        deleted_at: Set(None),
    };

    active_model
        .insert(&db.0)
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("id".to_string(), Value::String(id)),
        ("name".to_string(), Value::String(name)),
        ("sortOrder".to_string(), Value::Number(serde_json::Number::from(sort_order))),
    ])))))
}

pub async fn article_list(
    db: Extension<DatabaseConnection>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let models = cms_article::Entity::find()
        .filter(cms_article::Column::DeletedAt.is_null())
        .order_by_desc(cms_article::Column::CreateTime)
        .limit(20)
        .all(&db.0)
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = models
        .iter()
        .map(|m| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(m.id.clone())),
                (
                    "categoryId".to_string(),
                    Value::String(m.category_id.clone()),
                ),
                ("title".to_string(), Value::String(m.title.clone())),
                (
                    "content".to_string(),
                    m.content
                        .clone()
                        .map(Value::String)
                        .unwrap_or(Value::Null),
                ),
                (
                    "authorId".to_string(),
                    Value::String(m.author_id.clone()),
                ),
                ("status".to_string(), Value::String(m.status.clone())),
                (
                    "publishTime".to_string(),
                    m.publish_time
                        .clone()
                        .map(|dt| Value::String(dt.to_string()))
                        .unwrap_or(Value::Null),
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

pub async fn article_get(
    db: Extension<DatabaseConnection>,
    Path(id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let model = cms_article::Entity::find_by_id(&id)
        .filter(cms_article::Column::DeletedAt.is_null())
        .one(&db.0)
        .await
        .map_err(|_| AppError::Internal)?;

    match model {
        Some(m) => {
            let result = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(m.id.clone())),
                (
                    "categoryId".to_string(),
                    Value::String(m.category_id.clone()),
                ),
                ("title".to_string(), Value::String(m.title.clone())),
                (
                    "content".to_string(),
                    m.content
                        .clone()
                        .map(Value::String)
                        .unwrap_or(Value::Null),
                ),
                (
                    "authorId".to_string(),
                    Value::String(m.author_id.clone()),
                ),
                ("status".to_string(), Value::String(m.status.clone())),
                (
                    "publishTime".to_string(),
                    m.publish_time
                        .clone()
                        .map(|dt| Value::String(dt.to_string()))
                        .unwrap_or(Value::Null),
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
            ]));
            Ok(Json(ActionResult::success(result)))
        }
        None => Ok(Json(ActionResult::error("article not found"))),
    }
}

#[axum::debug_handler]
pub async fn article_create(
    db: Extension<DatabaseConnection>,
    Json(payload): Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let id = uuid::Uuid::new_v4().to_string();
    let category_id = payload
        .get("categoryId")
        .and_then(|v| v.as_str())
        .unwrap_or_default()
        .to_string();
    let title = payload
        .get("title")
        .and_then(|v| v.as_str())
        .unwrap_or_default()
        .to_string();
    let content = payload.get("content").and_then(|v| v.as_str()).map(|s| s.to_string());
    let author_id = payload
        .get("authorId")
        .and_then(|v| v.as_str())
        .unwrap_or_default()
        .to_string();
    let status = payload
        .get("status")
        .and_then(|v| v.as_str())
        .unwrap_or_else(|| "draft")
        .to_string();

    let active_model = cms_article::ActiveModel {
        id: Set(id.clone()),
        category_id: Set(category_id.clone()),
        title: Set(title.clone()),
        content: Set(content),
        author_id: Set(author_id),
        status: Set(status),
        publish_time: Set(None),
        create_time: Set(Some(chrono::Utc::now().naive_utc())),
        deleted_at: Set(None),
    };

    active_model
        .insert(&db.0)
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("id".to_string(), Value::String(id)),
        ("title".to_string(), Value::String(title)),
        ("categoryId".to_string(), Value::String(category_id)),
    ])))))
}

pub fn cms_core_entity_router(_pool: deadpool_postgres::Pool) -> Router {
    Router::new()
        .route("/jaxrs/cms/category/list", get(category_list))
        .route("/jaxrs/cms/category/{id}", get(category_get))
        .route("/jaxrs/cms/category/create", post(category_create))
        .route("/jaxrs/cms/article/list", get(article_list))
        .route("/jaxrs/cms/article/{id}", get(article_get))
        .route("/jaxrs/cms/article/create", post(article_create))
}

#[cfg(test)]
mod tests;
#[cfg(test)]
mod tests_generated;


pub fn router(pool: deadpool_postgres::Pool) -> axum::Router {
    crate::cms_core_entity_router(pool)
}

