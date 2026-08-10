use axum::{
    extract::{Extension, Path},
    routing::{get, post, put},
    Json, Router,
};
use deadpool_postgres::Pool;
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, QueryOrder, QuerySelect, Set};
use serde::Deserialize;
use serde_json::Value;
use shared::{error::AppError, middleware::require_owner, response::ActionResult, session::Session};

pub mod entities;
pub mod routes;

/// Maximum length for text input fields to prevent DB overflow.
const MAX_NAME_LEN: usize = 200;
const MAX_TEXT_LEN: usize = 500;
const MAX_LONG_TEXT_LEN: usize = 2000;

// ── Request structs ──────────────────────────────────────────────────────────

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

#[derive(Debug, Deserialize)]
pub struct InvokeCreateRequest {
    pub name: String,
    pub alias: Option<String>,
    pub category: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct InvokeUpdateRequest {
    pub name: Option<String>,
    pub alias: Option<String>,
    pub category: Option<String>,
}

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

// ── Helpers ──────────────────────────────────────────────────────────────────

fn validate_name(name: &str) -> Result<(), AppError> {
    if name.trim().is_empty() {
        return Err(AppError::BadRequest("name is required".to_string()));
    }
    if name.len() > MAX_NAME_LEN {
        return Err(AppError::BadRequest(format!(
            "name must be at most {MAX_NAME_LEN} characters"
        )));
    }
    Ok(())
}

fn validate_text(text: &str, max: usize, field: &str) -> Result<(), AppError> {
    if text.len() > max {
        return Err(AppError::BadRequest(format!(
            "{field} must be at most {max} characters"
        )));
    }
    Ok(())
}

// ── List handlers (with soft-delete filter) ──────────────────────────────────

pub async fn application_list(
    db: Extension<DatabaseConnection>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let models = entities::application::Entity::find()
        .order_by_asc(entities::application::Column::Name)
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
                ("category".to_string(), Value::String(m.category.clone())),
                (
                    "subCategory".to_string(),
                    Value::String(m.sub_category.clone()),
                ),
                ("version".to_string(), Value::String(m.version.clone())),
                ("publisher".to_string(), Value::String(m.publisher.clone())),
                (
                    "creatorPerson".to_string(),
                    Value::String(m.creator_person.clone()),
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

pub async fn script_list(
    db: Extension<DatabaseConnection>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let models = entities::script::Entity::find()
        .order_by_asc(entities::script::Column::Name)
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
                ("alias".to_string(), Value::String(m.alias.clone())),
                ("validated".to_string(), Value::Bool(m.validated)),
                (
                    "creatorPerson".to_string(),
                    Value::String(m.creator_person.clone()),
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

pub async fn invoke_list(
    db: Extension<DatabaseConnection>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let models = entities::cte_invoke::Entity::find()
        .filter(entities::cte_invoke::Column::DeletedAt.is_null())
        .order_by_asc(entities::cte_invoke::Column::Name)
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
                ("alias".to_string(), Value::String(m.alias.clone())),
                ("category".to_string(), Value::String(m.category.clone())),
                ("validated".to_string(), Value::Bool(m.validated)),
                (
                    "creatorPerson".to_string(),
                    Value::String(m.creator_person.clone()),
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

    let data: Vec<Value> = models
        .iter()
        .map(|m| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(m.id.clone())),
                ("name".to_string(), Value::String(m.name.clone())),
                ("type".to_string(), Value::String("agent".to_string())),
                ("alias".to_string(), Value::String(m.alias.clone())),
                (
                    "description".to_string(),
                    Value::String(m.description.clone()),
                ),
                ("validated".to_string(), Value::Bool(m.validated)),
                ("enable".to_string(), Value::Bool(m.enable)),
                ("cron".to_string(), Value::String(m.cron.clone())),
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

    let data: Vec<Value> = models
        .iter()
        .map(|m| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(m.id.clone())),
                ("name".to_string(), Value::String(m.name.clone())),
                (
                    "type".to_string(),
                    Value::String(m.extension.clone().unwrap_or_default()),
                ),
                ("storage".to_string(), Value::String(m.storage.clone())),
                (
                    "length".to_string(),
                    Value::Number(serde_json::Number::from(m.length.unwrap_or(0))),
                ),
                (
                    "description".to_string(),
                    Value::String(m.description.clone()),
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

// ── Application write handlers ──────────────────────────────────────────────

pub async fn application_create(
    db: Extension<DatabaseConnection>,
    Extension(session): Extension<Session>,
    Json(req): Json<ApplicationCreateRequest>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    validate_name(&req.name)?;

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

    let result = Value::Object(serde_json::Map::from_iter([
        ("id".to_string(), Value::String(model.id.clone())),
        ("name".to_string(), Value::String(model.name.clone())),
        ("category".to_string(), Value::String(model.category.clone())),
        (
            "subCategory".to_string(),
            Value::String(model.sub_category.clone()),
        ),
        ("version".to_string(), Value::String(model.version.clone())),
        ("publisher".to_string(), Value::String(model.publisher.clone())),
        (
            "creatorPerson".to_string(),
            Value::String(model.creator_person.clone()),
        ),
    ]));

    Ok(Json(ActionResult::success(result)))
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

    // IDOR check: verify current user owns this record
    require_owner(&pool, &session, &model.creator_person).await?;

    let mut active: entities::application::ActiveModel = model.into();
    if let Some(name) = req.name {
        validate_name(&name)?;
        active.name = Set(name);
    }
    if let Some(category) = req.category {
        validate_text(&category, MAX_TEXT_LEN, "category")?;
        active.category = Set(category);
    }
    if let Some(sub_category) = req.sub_category {
        validate_text(&sub_category, MAX_TEXT_LEN, "sub_category")?;
        active.sub_category = Set(sub_category);
    }
    if let Some(version) = req.version {
        validate_text(&version, MAX_TEXT_LEN, "version")?;
        active.version = Set(version);
    }
    if let Some(publisher) = req.publisher {
        validate_text(&publisher, MAX_TEXT_LEN, "publisher")?;
        active.publisher = Set(publisher);
    }

    let updated = active.update(&db.0).await.map_err(|_| AppError::Internal)?;

    let result = Value::Object(serde_json::Map::from_iter([
        ("id".to_string(), Value::String(updated.id.clone())),
        ("name".to_string(), Value::String(updated.name.clone())),
        ("category".to_string(), Value::String(updated.category.clone())),
        (
            "subCategory".to_string(),
            Value::String(updated.sub_category.clone()),
        ),
        ("version".to_string(), Value::String(updated.version.clone())),
        ("publisher".to_string(), Value::String(updated.publisher.clone())),
        (
            "creatorPerson".to_string(),
            Value::String(updated.creator_person.clone()),
        ),
    ]));

    Ok(Json(ActionResult::success(result)))
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

    // IDOR check: verify current user owns this record
    require_owner(&pool, &session, &model.creator_person).await?;

    // Soft delete
    let mut active: entities::application::ActiveModel = model.into();
    active.deleted_at = Set(Some(chrono::Utc::now().naive_utc()));
    active.update(&db.0).await.map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([("deleted".to_string(), Value::Bool(true))]),
    ))))
}

// ── Script write handlers ────────────────────────────────────────────────────

pub async fn script_create(
    db: Extension<DatabaseConnection>,
    Extension(session): Extension<Session>,
    Json(req): Json<ScriptCreateRequest>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    validate_name(&req.name)?;

    let active = entities::script::ActiveModel {
        id: Set(uuid::Uuid::new_v4().to_string()),
        name: Set(req.name),
        alias: Set(req.alias.unwrap_or_default()),
        validated: Set(false),
        creator_person: Set(session.person_unique),
        deleted_at: Set(None),
    };
    let model = active.insert(&db.0).await.map_err(|_| AppError::Internal)?;

    let result = Value::Object(serde_json::Map::from_iter([
        ("id".to_string(), Value::String(model.id.clone())),
        ("name".to_string(), Value::String(model.name.clone())),
        ("alias".to_string(), Value::String(model.alias.clone())),
        ("validated".to_string(), Value::Bool(model.validated)),
        (
            "creatorPerson".to_string(),
            Value::String(model.creator_person.clone()),
        ),
    ]));

    Ok(Json(ActionResult::success(result)))
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

    // IDOR check
    require_owner(&pool, &session, &model.creator_person).await?;

    let mut active: entities::script::ActiveModel = model.into();
    if let Some(name) = req.name {
        validate_name(&name)?;
        active.name = Set(name);
    }
    if let Some(alias) = req.alias {
        validate_text(&alias, MAX_TEXT_LEN, "alias")?;
        active.alias = Set(alias);
    }

    let updated = active.update(&db.0).await.map_err(|_| AppError::Internal)?;

    let result = Value::Object(serde_json::Map::from_iter([
        ("id".to_string(), Value::String(updated.id.clone())),
        ("name".to_string(), Value::String(updated.name.clone())),
        ("alias".to_string(), Value::String(updated.alias.clone())),
        ("validated".to_string(), Value::Bool(updated.validated)),
        (
            "creatorPerson".to_string(),
            Value::String(updated.creator_person.clone()),
        ),
    ]));

    Ok(Json(ActionResult::success(result)))
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

    // IDOR check
    require_owner(&pool, &session, &model.creator_person).await?;

    // Soft delete
    let mut active: entities::script::ActiveModel = model.into();
    active.deleted_at = Set(Some(chrono::Utc::now().naive_utc()));
    active.update(&db.0).await.map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([("deleted".to_string(), Value::Bool(true))]),
    ))))
}

// ── Invoke write handlers ────────────────────────────────────────────────────

pub async fn invoke_create(
    db: Extension<DatabaseConnection>,
    Extension(session): Extension<Session>,
    Json(req): Json<InvokeCreateRequest>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    validate_name(&req.name)?;

    let active = entities::cte_invoke::ActiveModel {
        id: Set(uuid::Uuid::new_v4().to_string()),
        name: Set(req.name),
        alias: Set(req.alias.unwrap_or_default()),
        category: Set(req.category.unwrap_or_default()),
        validated: Set(false),
        creator_person: Set(session.person_unique),
        deleted_at: Set(None),
    };
    let model = active.insert(&db.0).await.map_err(|_| AppError::Internal)?;

    let result = Value::Object(serde_json::Map::from_iter([
        ("id".to_string(), Value::String(model.id.clone())),
        ("name".to_string(), Value::String(model.name.clone())),
        ("alias".to_string(), Value::String(model.alias.clone())),
        ("category".to_string(), Value::String(model.category.clone())),
        ("validated".to_string(), Value::Bool(model.validated)),
        (
            "creatorPerson".to_string(),
            Value::String(model.creator_person.clone()),
        ),
    ]));

    Ok(Json(ActionResult::success(result)))
}

pub async fn invoke_update(
    db: Extension<DatabaseConnection>,
    pool: Extension<Pool>,
    session: Extension<Session>,
    Path(id): Path<String>,
    Json(req): Json<InvokeUpdateRequest>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let model = entities::cte_invoke::Entity::find_by_id(&id)
        .one(&db.0)
        .await
        .map_err(|_| AppError::Internal)?
        .ok_or(AppError::NotFound)?;

    // IDOR check
    require_owner(&pool, &session, &model.creator_person).await?;

    let mut active: entities::cte_invoke::ActiveModel = model.into();
    if let Some(name) = req.name {
        validate_name(&name)?;
        active.name = Set(name);
    }
    if let Some(alias) = req.alias {
        validate_text(&alias, MAX_TEXT_LEN, "alias")?;
        active.alias = Set(alias);
    }
    if let Some(category) = req.category {
        validate_text(&category, MAX_TEXT_LEN, "category")?;
        active.category = Set(category);
    }

    let updated = active.update(&db.0).await.map_err(|_| AppError::Internal)?;

    let result = Value::Object(serde_json::Map::from_iter([
        ("id".to_string(), Value::String(updated.id.clone())),
        ("name".to_string(), Value::String(updated.name.clone())),
        ("alias".to_string(), Value::String(updated.alias.clone())),
        ("category".to_string(), Value::String(updated.category.clone())),
        ("validated".to_string(), Value::Bool(updated.validated)),
        (
            "creatorPerson".to_string(),
            Value::String(updated.creator_person.clone()),
        ),
    ]));

    Ok(Json(ActionResult::success(result)))
}

pub async fn invoke_delete(
    db: Extension<DatabaseConnection>,
    pool: Extension<Pool>,
    session: Extension<Session>,
    Path(id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let model = entities::cte_invoke::Entity::find_by_id(&id)
        .one(&db.0)
        .await
        .map_err(|_| AppError::Internal)?
        .ok_or(AppError::NotFound)?;

    // IDOR check
    require_owner(&pool, &session, &model.creator_person).await?;

    let mut active: entities::cte_invoke::ActiveModel = model.into();
    active.deleted_at = Set(Some(chrono::Utc::now().naive_utc()));
    active.update(&db.0).await.map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([("deleted".to_string(), Value::Bool(true))]),
    ))))
}

// ── Agent write handlers ─────────────────────────────────────────────────────

pub async fn agent_create(
    db: Extension<DatabaseConnection>,
    Extension(session): Extension<Session>,
    Json(req): Json<AgentCreateRequest>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    validate_name(&req.name)?;

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

    let result = Value::Object(serde_json::Map::from_iter([
        ("id".to_string(), Value::String(model.id.clone())),
        ("name".to_string(), Value::String(model.name.clone())),
        ("type".to_string(), Value::String("agent".to_string())),
        ("alias".to_string(), Value::String(model.alias.clone())),
        (
            "description".to_string(),
            Value::String(model.description.clone()),
        ),
        ("validated".to_string(), Value::Bool(model.validated)),
        ("enable".to_string(), Value::Bool(model.enable)),
        ("cron".to_string(), Value::String(model.cron.clone())),
    ]));

    Ok(Json(ActionResult::success(result)))
}

pub async fn agent_update(
    db: Extension<DatabaseConnection>,
    _pool: Extension<Pool>,
    _session: Extension<Session>,
    Path(id): Path<String>,
    Json(req): Json<AgentUpdateRequest>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let model = entities::cte_agent::Entity::find_by_id(&id)
        .one(&db.0)
        .await
        .map_err(|_| AppError::Internal)?
        .ok_or(AppError::NotFound)?;

    // cte_agent has no creator_person field; admin check is handled by RBAC middleware
    let mut active: entities::cte_agent::ActiveModel = model.into();
    if let Some(name) = req.name {
        validate_name(&name)?;
        active.name = Set(name);
    }
    if let Some(alias) = req.alias {
        validate_text(&alias, MAX_TEXT_LEN, "alias")?;
        active.alias = Set(alias);
    }
    if let Some(description) = req.description {
        validate_text(&description, MAX_LONG_TEXT_LEN, "description")?;
        active.description = Set(description);
    }
    if let Some(cron) = req.cron {
        validate_text(&cron, MAX_TEXT_LEN, "cron")?;
        active.cron = Set(cron);
    }

    let updated = active.update(&db.0).await.map_err(|_| AppError::Internal)?;

    let result = Value::Object(serde_json::Map::from_iter([
        ("id".to_string(), Value::String(updated.id.clone())),
        ("name".to_string(), Value::String(updated.name.clone())),
        ("type".to_string(), Value::String("agent".to_string())),
        ("alias".to_string(), Value::String(updated.alias.clone())),
        (
            "description".to_string(),
            Value::String(updated.description.clone()),
        ),
        ("validated".to_string(), Value::Bool(updated.validated)),
        ("enable".to_string(), Value::Bool(updated.enable)),
        ("cron".to_string(), Value::String(updated.cron.clone())),
    ]));

    Ok(Json(ActionResult::success(result)))
}

pub async fn agent_delete(
    db: Extension<DatabaseConnection>,
    _pool: Extension<Pool>,
    _session: Extension<Session>,
    Path(id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let model = entities::cte_agent::Entity::find_by_id(&id)
        .one(&db.0)
        .await
        .map_err(|_| AppError::Internal)?
        .ok_or(AppError::NotFound)?;

    // cte_agent has no creator_person; admin check handled by RBAC middleware
    let mut active: entities::cte_agent::ActiveModel = model.into();
    active.deleted_at = Set(Some(chrono::Utc::now().naive_utc()));
    active.update(&db.0).await.map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([("deleted".to_string(), Value::Bool(true))]),
    ))))
}

// ── Structure write handlers ─────────────────────────────────────────────────

pub async fn structure_create(
    db: Extension<DatabaseConnection>,
    Extension(session): Extension<Session>,
    Json(req): Json<StructureCreateRequest>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    validate_name(&req.name)?;
    validate_text(&req.storage, MAX_TEXT_LEN, "storage")?;
    if let Some(ref desc) = req.description {
        validate_text(desc, MAX_LONG_TEXT_LEN, "description")?;
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

    let result = Value::Object(serde_json::Map::from_iter([
        ("id".to_string(), Value::String(model.id.clone())),
        ("name".to_string(), Value::String(model.name.clone())),
        (
            "type".to_string(),
            Value::String(model.extension.clone().unwrap_or_default()),
        ),
        ("storage".to_string(), Value::String(model.storage.clone())),
        (
            "length".to_string(),
            Value::Number(serde_json::Number::from(model.length.unwrap_or(0))),
        ),
        (
            "description".to_string(),
            Value::String(model.description.clone()),
        ),
    ]));

    Ok(Json(ActionResult::success(result)))
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

    // cte_structure has no creator_person; admin check handled by RBAC middleware
    let mut active: entities::cte_structure::ActiveModel = model.into();
    if let Some(name) = req.name {
        validate_name(&name)?;
        active.name = Set(name);
    }
    if let Some(storage) = req.storage {
        validate_text(&storage, MAX_TEXT_LEN, "storage")?;
        active.storage = Set(storage);
    }
    if let Some(extension) = req.extension {
        validate_text(&extension, MAX_TEXT_LEN, "extension")?;
        active.extension = Set(Some(extension));
    }
    if let Some(description) = req.description {
        validate_text(&description, MAX_LONG_TEXT_LEN, "description")?;
        active.description = Set(description);
    }

    let updated = active.update(&db.0).await.map_err(|_| AppError::Internal)?;

    let result = Value::Object(serde_json::Map::from_iter([
        ("id".to_string(), Value::String(updated.id.clone())),
        ("name".to_string(), Value::String(updated.name.clone())),
        (
            "type".to_string(),
            Value::String(updated.extension.clone().unwrap_or_default()),
        ),
        ("storage".to_string(), Value::String(updated.storage.clone())),
        (
            "length".to_string(),
            Value::Number(serde_json::Number::from(updated.length.unwrap_or(0))),
        ),
        (
            "description".to_string(),
            Value::String(updated.description.clone()),
        ),
    ]));

    Ok(Json(ActionResult::success(result)))
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

    // cte_structure has no creator_person; admin check handled by RBAC middleware
    let mut active: entities::cte_structure::ActiveModel = model.into();
    active.deleted_at = Set(Some(chrono::Utc::now().naive_utc()));
    active.update(&db.0).await.map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([("deleted".to_string(), Value::Bool(true))]),
    ))))
}

// ── Router ───────────────────────────────────────────────────────────────────

pub fn program_center_core_entity_router(pool: deadpool_postgres::Pool) -> Router {
    let db = std::panic::catch_unwind(|| {
        tokio::runtime::Handle::current()
            .block_on(shared::db::create_sea_orm_pool())
    })
    .ok()
    .and_then(|r| r.ok());

    let router = Router::new()
        // Application
        .route("/jaxrs/program_center/application/list", get(application_list))
        .route("/jaxrs/program_center/application", post(application_create))
        .route(
            "/jaxrs/program_center/application/{id}",
            put(application_update).delete(application_delete),
        )
        // Script
        .route("/jaxrs/program_center/script/list", get(script_list))
        .route("/jaxrs/program_center/script", post(script_create))
        .route(
            "/jaxrs/program_center/script/{id}",
            put(script_update).delete(script_delete),
        )
        // Invoke
        .route("/jaxrs/program_center/invoke/list", get(invoke_list))
        .route("/jaxrs/program_center/invoke", post(invoke_create))
        .route(
            "/jaxrs/program_center/invoke/{id}",
            put(invoke_update).delete(invoke_delete),
        )
        // Agent
        .route("/jaxrs/program_center/agent/list", get(agent_list))
        .route("/jaxrs/program_center/agent", post(agent_create))
        .route(
            "/jaxrs/program_center/agent/{id}",
            put(agent_update).delete(agent_delete),
        )
        // Structure
        .route("/jaxrs/program_center/structure/list", get(structure_list))
        .route("/jaxrs/program_center/structure", post(structure_create))
        .route(
            "/jaxrs/program_center/structure/{id}",
            put(structure_update).delete(structure_delete),
        );
    match db {
        Some(conn) => router.layer(Extension(conn)),
        None => router,
    }
}

#[cfg(test)]
pub fn program_center_mock_router(_db: DatabaseConnection) -> Router {
    Router::new()
        .route("/jaxrs/program_center/application/list", get(application_list))
        .route("/jaxrs/program_center/script/list", get(script_list))
        .route("/jaxrs/program_center/invoke/list", get(invoke_list))
        .route("/jaxrs/program_center/agent/list", get(agent_list))
        .route("/jaxrs/program_center/structure/list", get(structure_list))
        .layer(Extension(_db))
}

pub fn router(pool: deadpool_postgres::Pool) -> axum::Router {
    crate::program_center_core_entity_router(pool)
}

#[cfg(test)]
mod tests;
