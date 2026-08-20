use axum::{extract::{Extension, Path}, routing::{get, post, put, delete}, Json, Router};
use deadpool_postgres::Pool;
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryOrder, QuerySelect, Set};
use serde::Deserialize;
use serde_json::Value;
use shared::{error::AppError, middleware::require_owner, response::ActionResult, session::Session};

use crate::{entities, MAX_NAME_LEN, MAX_TEXT_LEN};

#[derive(Debug, Deserialize)]
pub struct ScriptCreateRequest {
    pub name: String,
    pub alias: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ScriptUpdateRequest {
    pub name: Option<String>,
    pub alias: Option<String>,
}

pub async fn script_list(
    db: Extension<DatabaseConnection>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let models = entities::script::Entity::find()
        .order_by_asc(entities::script::Column::Name)
        .limit(20)
        .all(&db.0)
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = models.iter().map(|m| {
        Value::Object(serde_json::Map::from_iter([
            ("id".to_string(), Value::String(m.id.clone())),
            ("name".to_string(), Value::String(m.name.clone())),
            ("alias".to_string(), Value::String(m.alias.clone())),
            ("validated".to_string(), Value::Bool(m.validated)),
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

pub async fn script_create(
    db: Extension<DatabaseConnection>,
    Extension(session): Extension<Session>,
    Json(req): Json<ScriptCreateRequest>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    if req.name.trim().is_empty() {
        return Err(AppError::BadRequest("name is required".to_string()));
    }
    if req.name.len() > MAX_NAME_LEN {
        return Err(AppError::BadRequest(format!("name must be at most {MAX_NAME_LEN} characters")));
    }

    let active = entities::script::ActiveModel {
        id: Set(uuid::Uuid::new_v4().to_string()),
        name: Set(req.name),
        alias: Set(req.alias.unwrap_or_default()),
        validated: Set(false),
        creator_person: Set(session.person_unique),
        deleted_at: Set(None),
    };
    let model = active.insert(&db.0).await.map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(model.id.clone())),
            ("name".to_string(), Value::String(model.name.clone())),
            ("alias".to_string(), Value::String(model.alias.clone())),
            ("validated".to_string(), Value::Bool(model.validated)),
            ("creatorPerson".to_string(), Value::String(model.creator_person.clone())),
        ]),
    ))))
}

pub async fn script_update(
    db: Extension<DatabaseConnection>,
    pool: Extension<Pool>,
    session: Extension<Session>,
    Path(id): Path<String>,
    Json(req): Json<ScriptUpdateRequest>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let model = entities::script::Entity::find_by_id(&id)
        .one(&db.0)
        .await
        .map_err(|_| AppError::Internal)?
        .ok_or(AppError::NotFound)?;

    require_owner(&pool, &session, &model.creator_person).await?;

    let mut active: entities::script::ActiveModel = model.into();
    if let Some(name) = req.name {
        if name.trim().is_empty() { return Err(AppError::BadRequest("name is required".to_string())); }
        if name.len() > MAX_NAME_LEN { return Err(AppError::BadRequest(format!("name must be at most {MAX_NAME_LEN} characters"))); }
        active.name = Set(name);
    }
    if let Some(alias) = req.alias {
        if alias.len() > MAX_TEXT_LEN { return Err(AppError::BadRequest("alias must be at most 500 characters".to_string())); }
        active.alias = Set(alias);
    }

    let updated = active.update(&db.0).await.map_err(|_| AppError::Internal)?;
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(updated.id.clone())),
            ("name".to_string(), Value::String(updated.name.clone())),
            ("alias".to_string(), Value::String(updated.alias.clone())),
            ("validated".to_string(), Value::Bool(updated.validated)),
            ("creatorPerson".to_string(), Value::String(updated.creator_person.clone())),
        ]),
    ))))
}

pub async fn script_delete(
    db: Extension<DatabaseConnection>,
    pool: Extension<Pool>,
    session: Extension<Session>,
    Path(id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let model = entities::script::Entity::find_by_id(&id)
        .one(&db.0)
        .await
        .map_err(|_| AppError::Internal)?
        .ok_or(AppError::NotFound)?;

    require_owner(&pool, &session, &model.creator_person).await?;

    let id = model.id.clone();
    let mut active: entities::script::ActiveModel = model.into();
    active.deleted_at = Set(Some(chrono::Utc::now().naive_utc()));
    active.update(&db.0).await.map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([("id".to_string(), Value::String(id))]),
    ))))
}

pub fn _router(_pool: Pool, db: Option<DatabaseConnection>) -> Router {
    let router = Router::new()
        .route("/jaxrs/program_center/script/list", get(script_list))
        .route("/jaxrs/program_center/script", post(script_create))
        .route("/jaxrs/program_center/script/{id}", put(script_update).delete(script_delete));
    match db {
        Some(conn) => router.layer(Extension(conn)),
        None => router,
    }
}
