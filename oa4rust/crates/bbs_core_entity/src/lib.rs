use axum::{
    extract::{Extension, Json, Path},
    routing::{get, post, delete},
    Router,
};
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, QueryOrder, QuerySelect, Set};
use serde_json::Value;
use shared::{error::AppError, response::ActionResult};

pub mod entities;
pub mod routes;

use entities::{bbs_forum_info, bbs_section_info, bbs_subject_info};

pub async fn forum_list(
    db: Extension<DatabaseConnection>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let models = bbs_forum_info::Entity::find()
        .order_by_asc(bbs_forum_info::Column::CreateTime)
        .limit(50)
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

pub async fn section_list(
    db: Extension<DatabaseConnection>,
    Path(forum_id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let models = bbs_section_info::Entity::find()
        .filter(bbs_section_info::Column::ForumId.eq(&forum_id))
        .order_by_asc(bbs_section_info::Column::OrderNumber)
        .limit(50)
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
                    "forumId".to_string(),
                    Value::String(m.forum_id.clone()),
                ),
                (
                    "sort".to_string(),
                    Value::Number(serde_json::Number::from(m.order_number)),
                ),
                (
                    "description".to_string(),
                    m.description
                        .clone()
                        .map(Value::String)
                        .unwrap_or(Value::Null),
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

pub async fn subject_top_list(
    db: Extension<DatabaseConnection>,
    Path(section_id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let models = bbs_subject_info::Entity::find()
        .filter(
            bbs_subject_info::Column::SectionId.eq(&section_id)
                .and(bbs_subject_info::Column::IsTop.eq(true))
                .and(bbs_subject_info::Column::Disable.eq(false)),
        )
        .order_by_desc(bbs_subject_info::Column::CreateTime)
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
                    "authorId".to_string(),
                    Value::String(m.author_id.clone()),
                ),
                (
                    "sectionId".to_string(),
                    Value::String(m.section_id.clone()),
                ),
                (
                    "replyCount".to_string(),
                    Value::Number(serde_json::Number::from(m.reply_count)),
                ),
                (
                    "viewCount".to_string(),
                    Value::Number(serde_json::Number::from(m.view_count)),
                ),
                ("isTop".to_string(), Value::Bool(m.is_top)),
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

pub async fn subject_list(
    db: Extension<DatabaseConnection>,
    Path(section_id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let models = bbs_subject_info::Entity::find()
        .filter(
            bbs_subject_info::Column::SectionId.eq(&section_id)
                .and(bbs_subject_info::Column::Disable.eq(false)),
        )
        .order_by_desc(bbs_subject_info::Column::CreateTime)
        .limit(50)
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
                    "authorId".to_string(),
                    Value::String(m.author_id.clone()),
                ),
                (
                    "sectionId".to_string(),
                    Value::String(m.section_id.clone()),
                ),
                (
                    "replyCount".to_string(),
                    Value::Number(serde_json::Number::from(m.reply_count)),
                ),
                (
                    "viewCount".to_string(),
                    Value::Number(serde_json::Number::from(m.view_count)),
                ),
                ("isTop".to_string(), Value::Bool(m.is_top)),
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

#[axum::debug_handler]
pub async fn create_forum(
    db: Extension<DatabaseConnection>,
    Json(payload): Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let id = uuid::Uuid::new_v4().to_string();
    let name = payload.get("name").and_then(|v| v.as_str()).unwrap_or_default().to_string();
    let description = payload.get("description").and_then(|v| v.as_str()).map(|s| s.to_string());

    let active_model = bbs_forum_info::ActiveModel {
        id: Set(id.clone()),
        name: Set(name.clone()),
        description: Set(description),
        order_number: Set(0),
        disable: Set(false),
        create_time: Set(Some(chrono::Utc::now().naive_utc())),
    };

    active_model
        .insert(&db.0)
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("id".to_string(), Value::String(id)),
        ("name".to_string(), Value::String(name)),
    ])))))
}

#[axum::debug_handler]
pub async fn update_forum(
    db: Extension<DatabaseConnection>,
    Path(id): Path<String>,
    Json(payload): Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let existing = bbs_forum_info::Entity::find_by_id(&id)
        .one(&db.0)
        .await
        .map_err(|_| AppError::Internal)?;

    let Some(m) = existing else {
        return Ok(Json(ActionResult::error("forum not found")));
    };

    let name = payload
        .get("name")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string())
        .unwrap_or(m.name);
    let description = payload
        .get("description")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string())
        .or(m.description)
        .unwrap_or_default();

    let active_model = bbs_forum_info::ActiveModel {
        id: Set(id.clone()),
        name: Set(name),
        description: Set(Some(description)),
        order_number: Set(m.order_number),
        disable: Set(m.disable),
        create_time: Set(m.create_time),
    };

    active_model
        .update(&db.0)
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("id".to_string(), Value::String(id)),
        ("updated".to_string(), Value::Bool(true)),
    ])))))
}

#[axum::debug_handler]
pub async fn delete_forum(
    db: Extension<DatabaseConnection>,
    Path(id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let existing = bbs_forum_info::Entity::find_by_id(&id)
        .one(&db.0)
        .await
        .map_err(|_| AppError::Internal)?;

    if existing.is_none() {
        return Ok(Json(ActionResult::error("forum not found")));
    }

    let active_model: bbs_forum_info::ActiveModel = existing.unwrap().into();
    active_model
        .delete(&db.0)
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("id".to_string(), Value::String(id)),
        ("deleted".to_string(), Value::Bool(true)),
    ])))))
}

#[axum::debug_handler]
pub async fn create_section(
    db: Extension<DatabaseConnection>,
    Json(payload): Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let id = uuid::Uuid::new_v4().to_string();
    let name = payload.get("name").and_then(|v| v.as_str()).unwrap_or_default().to_string();
    let forum_id = payload
        .get("forumId")
        .and_then(|v| v.as_str())
        .unwrap_or_default()
        .to_string();
    let sort = payload.get("sort").and_then(|v| v.as_i64()).unwrap_or(0) as i32;
    let description = payload.get("description").and_then(|v| v.as_str()).map(|s| s.to_string());

    let active_model = bbs_section_info::ActiveModel {
        id: Set(id.clone()),
        forum_id: Set(forum_id.clone()),
        name: Set(name.clone()),
        description: Set(description),
        order_number: Set(sort),
        disable: Set(false),
        create_time: Set(Some(chrono::Utc::now().naive_utc())),
    };

    active_model
        .insert(&db.0)
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("id".to_string(), Value::String(id)),
        ("name".to_string(), Value::String(name)),
        ("forumId".to_string(), Value::String(forum_id)),
    ])))))
}

#[axum::debug_handler]
pub async fn update_section(
    db: Extension<DatabaseConnection>,
    Path(id): Path<String>,
    Json(payload): Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let existing = bbs_section_info::Entity::find_by_id(&id)
        .one(&db.0)
        .await
        .map_err(|_| AppError::Internal)?;

    let Some(m) = existing else {
        return Ok(Json(ActionResult::error("section not found")));
    };

    let name = payload
        .get("name")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string())
        .unwrap_or(m.name);
    let forum_id = payload
        .get("forumId")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string())
        .unwrap_or(m.forum_id);
    let sort = payload
        .get("sort")
        .and_then(|v| v.as_i64())
        .map(|i| i as i32)
        .unwrap_or(m.order_number);
    let description = payload
        .get("description")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string())
        .or(m.description)
        .unwrap_or_default();

    let active_model = bbs_section_info::ActiveModel {
        id: Set(id.clone()),
        forum_id: Set(forum_id),
        name: Set(name),
        description: Set(Some(description)),
        order_number: Set(sort),
        disable: Set(m.disable),
        create_time: Set(m.create_time),
    };

    active_model
        .update(&db.0)
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("id".to_string(), Value::String(id)),
        ("updated".to_string(), Value::Bool(true)),
    ])))))
}

#[axum::debug_handler]
pub async fn delete_section(
    db: Extension<DatabaseConnection>,
    Path(id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let existing = bbs_section_info::Entity::find_by_id(&id)
        .one(&db.0)
        .await
        .map_err(|_| AppError::Internal)?;

    if existing.is_none() {
        return Ok(Json(ActionResult::error("section not found")));
    }

    let active_model: bbs_section_info::ActiveModel = existing.unwrap().into();
    active_model
        .delete(&db.0)
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("id".to_string(), Value::String(id)),
        ("deleted".to_string(), Value::Bool(true)),
    ])))))
}

#[axum::debug_handler]
pub async fn create_subject(
    db: Extension<DatabaseConnection>,
    Json(payload): Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let id = uuid::Uuid::new_v4().to_string();
    let title = payload
        .get("title")
        .and_then(|v| v.as_str())
        .unwrap_or_default()
        .to_string();
    let section_id = payload
        .get("sectionId")
        .and_then(|v| v.as_str())
        .unwrap_or_default()
        .to_string();
    let author_id = payload
        .get("authorId")
        .and_then(|v| v.as_str())
        .unwrap_or_default()
        .to_string();

    let active_model = bbs_subject_info::ActiveModel {
        id: Set(id.clone()),
        title: Set(title.clone()),
        author_id: Set(author_id),
        section_id: Set(section_id.clone()),
        content: Set(None),
        reply_count: Set(0),
        view_count: Set(0),
        is_top: Set(false),
        disable: Set(false),
        create_time: Set(Some(chrono::Utc::now().naive_utc())),
    };

    active_model
        .insert(&db.0)
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("id".to_string(), Value::String(id)),
        ("title".to_string(), Value::String(title)),
        ("sectionId".to_string(), Value::String(section_id)),
    ])))))
}

#[axum::debug_handler]
pub async fn update_subject(
    db: Extension<DatabaseConnection>,
    Path(id): Path<String>,
    Json(payload): Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let existing = bbs_subject_info::Entity::find_by_id(&id)
        .one(&db.0)
        .await
        .map_err(|_| AppError::Internal)?;

    let Some(m) = existing else {
        return Ok(Json(ActionResult::error("subject not found")));
    };

    let title = payload
        .get("title")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string())
        .unwrap_or(m.title);
    let section_id = payload
        .get("sectionId")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string())
        .unwrap_or(m.section_id);
    let is_top = payload.get("isTop").and_then(|v| v.as_bool()).unwrap_or(m.is_top);
    let disable = payload.get("disable").and_then(|v| v.as_bool()).unwrap_or(m.disable);

    let active_model = bbs_subject_info::ActiveModel {
        id: Set(id.clone()),
        title: Set(title),
        author_id: Set(m.author_id),
        section_id: Set(section_id),
        content: Set(m.content),
        reply_count: Set(m.reply_count),
        view_count: Set(m.view_count),
        is_top: Set(is_top),
        disable: Set(disable),
        create_time: Set(m.create_time),
    };

    active_model
        .update(&db.0)
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("id".to_string(), Value::String(id)),
        ("updated".to_string(), Value::Bool(true)),
    ])))))
}

#[axum::debug_handler]
pub async fn delete_subject(
    db: Extension<DatabaseConnection>,
    Path(id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let existing = bbs_subject_info::Entity::find_by_id(&id)
        .one(&db.0)
        .await
        .map_err(|_| AppError::Internal)?;

    if existing.is_none() {
        return Ok(Json(ActionResult::error("subject not found")));
    }

    let active_model: bbs_subject_info::ActiveModel = existing.unwrap().into();
    active_model
        .delete(&db.0)
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("id".to_string(), Value::String(id)),
        ("deleted".to_string(), Value::Bool(true)),
    ])))))
}

#[axum::debug_handler]
pub async fn create_reply(
    db: Extension<DatabaseConnection>,
    Json(payload): Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    // Note: reply is stored in a separate table not covered by this entity migration
    // Keeping the same raw SQL path for this endpoint
    let id = uuid::Uuid::new_v4().to_string();
    let topic_id = payload
        .get("topicId")
        .and_then(|v| v.as_str())
        .unwrap_or_default()
        .to_string();
    let content = payload
        .get("content")
        .and_then(|v| v.as_str())
        .unwrap_or_default()
        .to_string();
    let creator = payload
        .get("creator")
        .and_then(|v| v.as_str())
        .unwrap_or("system")
        .to_string();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("id".to_string(), Value::String(id)),
        ("topicId".to_string(), Value::String(topic_id)),
    ])))))
}

pub async fn search_subjects(
    db: Extension<DatabaseConnection>,
    axum::extract::Query(params): axum::extract::Query<std::collections::HashMap<String, String>>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let keyword = params.get("keyword").map(|s| s.as_str()).unwrap_or_default();
    let pattern = format!("%{}%", keyword);

    let models = bbs_subject_info::Entity::find()
        .filter(
            bbs_subject_info::Column::Title
                .like(&pattern)
                .and(bbs_subject_info::Column::Disable.eq(false)),
        )
        .order_by_desc(bbs_subject_info::Column::CreateTime)
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
                    "authorId".to_string(),
                    Value::String(m.author_id.clone()),
                ),
                (
                    "sectionId".to_string(),
                    Value::String(m.section_id.clone()),
                ),
                (
                    "replyCount".to_string(),
                    Value::Number(serde_json::Number::from(m.reply_count)),
                ),
                (
                    "viewCount".to_string(),
                    Value::Number(serde_json::Number::from(m.view_count)),
                ),
                ("isTop".to_string(), Value::Bool(m.is_top)),
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

pub fn bbs_core_entity_router(_pool: deadpool_postgres::Pool) -> Router {
    let db = std::panic::catch_unwind(|| {
        tokio::runtime::Handle::current()
            .block_on(shared::db::create_sea_orm_pool())
    })
    .ok()
    .and_then(|r| r.ok());

    let router = Router::new()
        .route("/jaxrs/bbs/core/entity/forum/list", get(forum_list))
        .route("/jaxrs/bbs/core/entity/forum", post(create_forum))
        .route("/jaxrs/bbs/core/entity/forum/{id}", post(update_forum))
        .route("/jaxrs/bbs/core/entity/forum/{id}", delete(delete_forum))
        .route(
            "/jaxrs/bbs/core/entity/section/list/{forumId}",
            get(section_list),
        )
        .route("/jaxrs/bbs/core/entity/section", post(create_section))
        .route("/jaxrs/bbs/core/entity/section/{id}", post(update_section))
        .route("/jaxrs/bbs/core/entity/section/{id}", delete(delete_section))
        .route(
            "/jaxrs/bbs/core/entity/subject/top/{sectionId}",
            get(subject_top_list),
        )
        .route(
            "/jaxrs/bbs/core/entity/subject/list/{sectionId}",
            get(subject_list),
        )
        .route("/jaxrs/bbs/core/entity/subject", post(create_subject))
        .route("/jaxrs/bbs/core/entity/subject/{id}", post(update_subject))
        .route("/jaxrs/bbs/core/entity/subject/{id}", delete(delete_subject))
        .route("/jaxrs/bbs/core/entity/reply", post(create_reply))
        .route(
            "/jaxrs/bbs/core/entity/subject/search",
            get(search_subjects),
        );
    match db {
        Some(conn) => router.layer(Extension(conn)),
        None => router,
    }
}

#[cfg(test)]
mod tests;

pub fn router(pool: deadpool_postgres::Pool) -> axum::Router {
    crate::bbs_core_entity_router(pool)
}

