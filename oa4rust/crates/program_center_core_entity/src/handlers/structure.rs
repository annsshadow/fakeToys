use axum::{extract::{Extension, Path}, routing::{get, post, put, delete}, Json, Router};
use deadpool_postgres::Pool;
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, QueryOrder, QuerySelect, Set};
use serde::Deserialize;
use serde_json::Value;
use shared::{error::AppError, middleware::is_admin, response::ActionResult, session::Session};

use crate::{entities, MAX_NAME_LEN, MAX_TEXT_LEN, MAX_LONG_TEXT_LEN};

#[derive(Debug, Deserialize)]
pub struct StructureCreateRequest {
    pub name: String,
    pub storage: String,
    pub extension: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct StructureUpdateRequest {
    pub name: Option<String>,
    pub storage: Option<String>,
    pub extension: Option<String>,
    pub description: Option<String>,
}

pub async fn structure_list(
    db: Extension<DatabaseConnection>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let models = entities::cte_structure::Entity::find()
        .filter(entities::cte_structure::Column::DeletedAt.is_null())
        .order_by_asc(entities::cte_structure::Column::Name)
        .limit(20)
        .all(&db.0)
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = models.iter().map(|m| {
        Value::Object(serde_json::Map::from_iter([
            ("id".to_string(), Value::String(m.id.clone())),
            ("name".to_string(), Value::String(m.name.clone())),
            ("type".to_string(), Value::String(m.extension.clone().unwrap_or_default())),
            ("storage".to_string(), Value::String(m.storage.clone())),
            ("length".to_string(), Value::Number(serde_json::Number::from(m.length.unwrap_or(0)))),
            ("description".to_string(), Value::String(m.description.clone())),
        ]))
    }).collect();

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
            ("data".to_string(), Value::Array(data)),
        ]),
    ))))
}

pub async fn structure_create(
    db: Extension<DatabaseConnection>,
    Extension(session): Extension<Session>,
    Json(req): Json<StructureCreateRequest>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    if req.name.trim().is_empty() {
        return Err(AppError::BadRequest("name is required".to_string()));
    }
    if req.name.len() > MAX_NAME_LEN {
        return Err(AppError::BadRequest(format!("name must be at most {MAX_NAME_LEN} characters")));
    }
    if req.storage.len() > MAX_TEXT_LEN {
        return Err(AppError::BadRequest("storage must be at most 500 characters".to_string()));
    }
    if let Some(ref desc) = req.description {
        if desc.len() > MAX_LONG_TEXT_LEN {
            return Err(AppError::BadRequest(format!("description must be at most {MAX_LONG_TEXT_LEN} characters")));
        }
    }

    let active = entities::cte_structure::ActiveModel {
        id: Set(uuid::Uuid::new_v4().to_string()),
        name: Set(req.name),
        extension: Set(req.extension),
        storage: Set(req.storage),
        length: Set(None),
        description: Set(req.description.unwrap_or_default()),
        deleted_at: Set(None),
    };
    let model = active.insert(&db.0).await.map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(model.id.clone())),
            ("name".to_string(), Value::String(model.name.clone())),
            ("type".to_string(), Value::String(model.extension.clone().unwrap_or_default())),
            ("storage".to_string(), Value::String(model.storage.clone())),
            ("length".to_string(), Value::Number(serde_json::Number::from(model.length.unwrap_or(0)))),
            ("description".to_string(), Value::String(model.description.clone())),
        ]),
    ))))
}

pub async fn structure_update(
    db: Extension<DatabaseConnection>,
    pool: Extension<Pool>,
    session: Extension<Session>,
    Path(id): Path<String>,
    Json(req): Json<StructureUpdateRequest>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let model = entities::cte_structure::Entity::find_by_id(&id)
        .one(&db.0)
        .await
        .map_err(|_| AppError::Internal)?
        .ok_or(AppError::NotFound)?;

    if !is_admin(&pool, &session.person_unique).await {
        return Ok(Json(ActionResult::error("forbidden")));
    }

    let mut active: entities::cte_structure::ActiveModel = model.into();
    if let Some(name) = req.name {
        if name.trim().is_empty() { return Err(AppError::BadRequest("name is required".to_string())); }
        if name.len() > MAX_NAME_LEN { return Err(AppError::BadRequest(format!("name must be at most {MAX_NAME_LEN} characters"))); }
        active.name = Set(name);
    }
    if let Some(storage) = req.storage {
        if storage.len() > MAX_TEXT_LEN { return Err(AppError::BadRequest("storage must be at most 500 characters".to_string())); }
        active.storage = Set(storage);
    }
    if let Some(extension) = req.extension {
        if extension.len() > MAX_TEXT_LEN { return Err(AppError::BadRequest("extension must be at most 500 characters".to_string())); }
        active.extension = Set(Some(extension));
    }
    if let Some(description) = req.description {
        if description.len() > MAX_LONG_TEXT_LEN { return Err(AppError::BadRequest(format!("description must be at most {MAX_LONG_TEXT_LEN} characters"))); }
        active.description = Set(description);
    }

    let updated = active.update(&db.0).await.map_err(|_| AppError::Internal)?;
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(updated.id.clone())),
            ("name".to_string(), Value::String(updated.name.clone())),
            ("type".to_string(), Value::String(updated.extension.clone().unwrap_or_default())),
            ("storage".to_string(), Value::String(updated.storage.clone())),
            ("length".to_string(), Value::Number(serde_json::Number::from(updated.length.unwrap_or(0)))),
            ("description".to_string(), Value::String(updated.description.clone())),
        ]),
    ))))
}

pub async fn structure_delete(
    db: Extension<DatabaseConnection>,
    pool: Extension<Pool>,
    session: Extension<Session>,
    Path(id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let model = entities::cte_structure::Entity::find_by_id(&id)
        .one(&db.0)
        .await
        .map_err(|_| AppError::Internal)?
        .ok_or(AppError::NotFound)?;

    if !is_admin(&pool, &session.person_unique).await {
        return Ok(Json(ActionResult::error("forbidden")));
    }

    let id = model.id.clone();
    let mut active: entities::cte_structure::ActiveModel = model.into();
    active.deleted_at = Set(Some(chrono::Utc::now().naive_utc()));
    active.update(&db.0).await.map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([("id".to_string(), Value::String(id))]),
    ))))
}

pub fn _router(_pool: Pool, db: Option<DatabaseConnection>) -> Router {
    let router = Router::new()
        .route("/jaxrs/program_center/structure/list", get(structure_list))
        .route("/jaxrs/program_center/structure", post(structure_create))
        .route("/jaxrs/program_center/structure/{id}", put(structure_update).delete(structure_delete));
    match db {
        Some(conn) => router.layer(Extension(conn)),
        None => router,
    }
}
