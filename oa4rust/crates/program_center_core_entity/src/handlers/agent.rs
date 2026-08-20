use axum::{extract::{Extension, Path}, routing::{get, post, put, delete}, Json, Router};
use deadpool_postgres::Pool;
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, QueryOrder, QuerySelect, Set};
use serde::Deserialize;
use serde_json::Value;
use shared::{error::AppError, middleware::is_admin, response::ActionResult, session::Session};

use crate::{entities, MAX_NAME_LEN, MAX_TEXT_LEN, MAX_LONG_TEXT_LEN};

#[derive(Debug, Deserialize)]
pub struct AgentCreateRequest {
    pub name: String,
    pub alias: Option<String>,
    pub description: Option<String>,
    pub cron: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct AgentUpdateRequest {
    pub name: Option<String>,
    pub alias: Option<String>,
    pub description: Option<String>,
    pub cron: Option<String>,
}

pub async fn agent_list(
    db: Extension<DatabaseConnection>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let models = entities::cte_agent::Entity::find()
        .filter(entities::cte_agent::Column::DeletedAt.is_null())
        .order_by_asc(entities::cte_agent::Column::Name)
        .limit(20)
        .all(&db.0)
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = models.iter().map(|m| {
        Value::Object(serde_json::Map::from_iter([
            ("id".to_string(), Value::String(m.id.clone())),
            ("name".to_string(), Value::String(m.name.clone())),
            ("type".to_string(), Value::String("agent".to_string())),
            ("alias".to_string(), Value::String(m.alias.clone())),
            ("description".to_string(), Value::String(m.description.clone())),
            ("validated".to_string(), Value::Bool(m.validated)),
            ("enable".to_string(), Value::Bool(m.enable)),
            ("cron".to_string(), Value::String(m.cron.clone())),
        ]))
    }).collect();

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
            ("data".to_string(), Value::Array(data)),
        ]),
    ))))
}

pub async fn agent_create(
    db: Extension<DatabaseConnection>,
    Extension(session): Extension<Session>,
    Json(req): Json<AgentCreateRequest>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    if req.name.trim().is_empty() {
        return Err(AppError::BadRequest("name is required".to_string()));
    }
    if req.name.len() > MAX_NAME_LEN {
        return Err(AppError::BadRequest(format!("name must be at most {MAX_NAME_LEN} characters")));
    }

    let active = entities::cte_agent::ActiveModel {
        id: Set(uuid::Uuid::new_v4().to_string()),
        name: Set(req.name),
        alias: Set(req.alias.unwrap_or_default()),
        description: Set(req.description.clone().unwrap_or_default()),
        validated: Set(false),
        enable: Set(false),
        cron: Set(req.cron.unwrap_or_default()),
        deleted_at: Set(None),
    };
    let model = active.insert(&db.0).await.map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(model.id.clone())),
            ("name".to_string(), Value::String(model.name.clone())),
            ("type".to_string(), Value::String("agent".to_string())),
            ("alias".to_string(), Value::String(model.alias.clone())),
            ("description".to_string(), Value::String(model.description.clone())),
            ("validated".to_string(), Value::Bool(model.validated)),
            ("enable".to_string(), Value::Bool(model.enable)),
            ("cron".to_string(), Value::String(model.cron.clone())),
        ]),
    ))))
}

pub async fn agent_update(
    db: Extension<DatabaseConnection>,
    pool: Extension<Pool>,
    session: Extension<Session>,
    Path(id): Path<String>,
    Json(req): Json<AgentUpdateRequest>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let model = entities::cte_agent::Entity::find_by_id(&id)
        .one(&db.0)
        .await
        .map_err(|_| AppError::Internal)?
        .ok_or(AppError::NotFound)?;

    if !is_admin(&pool, &session.person_unique).await {
        return Ok(Json(ActionResult::error("forbidden")));
    }

    let mut active: entities::cte_agent::ActiveModel = model.into();
    if let Some(name) = req.name {
        if name.trim().is_empty() { return Err(AppError::BadRequest("name is required".to_string())); }
        if name.len() > MAX_NAME_LEN { return Err(AppError::BadRequest(format!("name must be at most {MAX_NAME_LEN} characters"))); }
        active.name = Set(name);
    }
    if let Some(alias) = req.alias {
        if alias.len() > MAX_TEXT_LEN { return Err(AppError::BadRequest("alias must be at most 500 characters".to_string())); }
        active.alias = Set(alias);
    }
    if let Some(description) = req.description {
        if description.len() > MAX_LONG_TEXT_LEN { return Err(AppError::BadRequest(format!("description must be at most {MAX_LONG_TEXT_LEN} characters"))); }
        active.description = Set(description);
    }
    if let Some(cron) = req.cron {
        if cron.len() > MAX_TEXT_LEN { return Err(AppError::BadRequest("cron must be at most 500 characters".to_string())); }
        active.cron = Set(cron);
    }

    let updated = active.update(&db.0).await.map_err(|_| AppError::Internal)?;
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(updated.id.clone())),
            ("name".to_string(), Value::String(updated.name.clone())),
            ("type".to_string(), Value::String("agent".to_string())),
            ("alias".to_string(), Value::String(updated.alias.clone())),
            ("description".to_string(), Value::String(updated.description.clone())),
            ("validated".to_string(), Value::Bool(updated.validated)),
            ("enable".to_string(), Value::Bool(updated.enable)),
            ("cron".to_string(), Value::String(updated.cron.clone())),
        ]),
    ))))
}

pub async fn agent_delete(
    db: Extension<DatabaseConnection>,
    pool: Extension<Pool>,
    session: Extension<Session>,
    Path(id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let model = entities::cte_agent::Entity::find_by_id(&id)
        .one(&db.0)
        .await
        .map_err(|_| AppError::Internal)?
        .ok_or(AppError::NotFound)?;

    if !is_admin(&pool, &session.person_unique).await {
        return Ok(Json(ActionResult::error("forbidden")));
    }

    let id = model.id.clone();
    let mut active: entities::cte_agent::ActiveModel = model.into();
    active.deleted_at = Set(Some(chrono::Utc::now().naive_utc()));
    active.update(&db.0).await.map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([("id".to_string(), Value::String(id))]),
    ))))
}

pub fn _router(_pool: Pool, db: Option<DatabaseConnection>) -> Router {
    let router = Router::new()
        .route("/jaxrs/program_center/agent/list", get(agent_list))
        .route("/jaxrs/program_center/agent", post(agent_create))
        .route("/jaxrs/program_center/agent/{id}", put(agent_update).delete(agent_delete));
    match db {
        Some(conn) => router.layer(Extension(conn)),
        None => router,
    }
}
