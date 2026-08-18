use axum::{extract::{Extension, Path}, routing::{get, post, put, delete}, Json, Router};
use deadpool_postgres::Pool;
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryOrder, QuerySelect, Set};
use serde::Deserialize;
use serde_json::Value;
use shared::{error::AppError, middleware::require_owner, response::ActionResult, session::Session};

use crate::{entities, MAX_NAME_LEN};

#[derive(Debug, Deserialize)]
pub struct ApplicationCreateRequest {
    pub name: String,
    pub category: Option<String>,
    pub sub_category: Option<String>,
    pub version: Option<String>,
    pub publisher: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ApplicationUpdateRequest {
    pub name: Option<String>,
    pub category: Option<String>,
    pub sub_category: Option<String>,
    pub version: Option<String>,
    pub publisher: Option<String>,
}

pub async fn application_list(
    db: Extension<DatabaseConnection>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let models = entities::application::Entity::find()
        .order_by_asc(entities::application::Column::Name)
        .limit(20)
        .all(&db.0)
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = models.iter().map(|m| {
        Value::Object(serde_json::Map::from_iter([
            ("id".to_string(), Value::String(m.id.clone())),
            ("name".to_string(), Value::String(m.name.clone())),
            ("category".to_string(), Value::String(m.category.clone())),
            ("subCategory".to_string(), Value::String(m.sub_category.clone())),
            ("version".to_string(), Value::String(m.version.clone())),
            ("publisher".to_string(), Value::String(m.publisher.clone())),
            ("creatorPerson".to_string(), Value::String(m.creator_person.clone())),
        ]))
    }).collect();

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
            ("data".to_string(), Value::Array(data)),
        ]),
    ))))
}

pub async fn application_create(
    db: Extension<DatabaseConnection>,
    Extension(session): Extension<Session>,
    Json(req): Json<ApplicationCreateRequest>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    if req.name.trim().is_empty() {
        return Err(AppError::BadRequest("name is required".to_string()));
    }
    if req.name.len() > MAX_NAME_LEN {
        return Err(AppError::BadRequest(format!("name must be at most {MAX_NAME_LEN} characters")));
    }

    let active = entities::application::ActiveModel {
        id: Set(uuid::Uuid::new_v4().to_string()),
        name: Set(req.name),
        category: Set(req.category.unwrap_or_default()),
        sub_category: Set(req.sub_category.unwrap_or_default()),
        version: Set(req.version.unwrap_or_default()),
        publisher: Set(req.publisher.unwrap_or_default()),
        creator_person: Set(session.person_unique),
        deleted_at: Set(None),
    };
    let model = active.insert(&db.0).await.map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(model.id.clone())),
            ("name".to_string(), Value::String(model.name.clone())),
            ("category".to_string(), Value::String(model.category.clone())),
            ("subCategory".to_string(), Value::String(model.sub_category.clone())),
            ("version".to_string(), Value::String(model.version.clone())),
            ("publisher".to_string(), Value::String(model.publisher.clone())),
            ("creatorPerson".to_string(), Value::String(model.creator_person.clone())),
        ]),
    ))))
}

pub async fn application_update(
    db: Extension<DatabaseConnection>,
    pool: Extension<Pool>,
    session: Extension<Session>,
    Path(id): Path<String>,
    Json(req): Json<ApplicationUpdateRequest>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let model = entities::application::Entity::find_by_id(&id)
        .one(&db.0)
        .await
        .map_err(|_| AppError::Internal)?
        .ok_or(AppError::NotFound)?;

    require_owner(&pool, &session, &model.creator_person).await?;

    let mut active: entities::application::ActiveModel = model.into();
    if let Some(name) = req.name {
        if name.trim().is_empty() { return Err(AppError::BadRequest("name is required".to_string())); }
        if name.len() > MAX_NAME_LEN { return Err(AppError::BadRequest(format!("name must be at most {MAX_NAME_LEN} characters"))); }
        active.name = Set(name);
    }
    if let Some(v) = req.category { active.category = Set(v); }
    if let Some(v) = req.sub_category { active.sub_category = Set(v); }
    if let Some(v) = req.version { active.version = Set(v); }
    if let Some(v) = req.publisher { active.publisher = Set(v); }

    let updated = active.update(&db.0).await.map_err(|_| AppError::Internal)?;
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(updated.id.clone())),
            ("name".to_string(), Value::String(updated.name.clone())),
            ("category".to_string(), Value::String(updated.category.clone())),
            ("subCategory".to_string(), Value::String(updated.sub_category.clone())),
            ("version".to_string(), Value::String(updated.version.clone())),
            ("publisher".to_string(), Value::String(updated.publisher.clone())),
            ("creatorPerson".to_string(), Value::String(updated.creator_person.clone())),
        ]),
    ))))
}

pub async fn application_delete(
    db: Extension<DatabaseConnection>,
    pool: Extension<Pool>,
    session: Extension<Session>,
    Path(id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let model = entities::application::Entity::find_by_id(&id)
        .one(&db.0)
        .await
        .map_err(|_| AppError::Internal)?
        .ok_or(AppError::NotFound)?;

    require_owner(&pool, &session, &model.creator_person).await?;

    let mut active: entities::application::ActiveModel = model.into();
    active.deleted_at = Set(Some(chrono::Utc::now().naive_utc()));
    active.update(&db.0).await.map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([("deleted".to_string(), Value::Bool(true))]),
    ))))
}

pub fn _router(_pool: Pool, db: Option<DatabaseConnection>) -> Router {
    let router = Router::new()
        .route("/jaxrs/program_center/application/list", get(application_list))
        .route("/jaxrs/program_center/application", post(application_create))
        .route("/jaxrs/program_center/application/{id}", put(application_update).delete(application_delete));
    match db {
        Some(conn) => router.layer(Extension(conn)),
        None => router,
    }
}
