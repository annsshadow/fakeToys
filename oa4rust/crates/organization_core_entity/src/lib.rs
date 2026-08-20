use axum::{
    extract::{Extension, Path},
    routing::{get, post, put},
    Json, Router,
};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, QueryOrder,
    QuerySelect, Set,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use shared::{error::AppError, response::{option_to_json, ActionResult}};

pub mod entities;
pub mod routes;

use entities::{
    org_bind, org_custom, org_definition, org_group, org_identity, org_person,
};

// ── Request structs ──────────────────────────────────────────────────────────

#[derive(Debug, Deserialize, Serialize)]
pub struct DefinitionCreateRequest {
    pub name: String,
    pub category: String,
    pub type_: String,
}

#[derive(Debug, Deserialize)]
pub struct DefinitionUpdateRequest {
    pub name: Option<String>,
    pub category: Option<String>,
    pub type_: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct GroupCreateRequest {
    pub name: String,
    pub parent_id: Option<String>,
    pub level: i32,
}

#[derive(Debug, Deserialize)]
pub struct GroupUpdateRequest {
    pub name: Option<String>,
    pub parent_id: Option<String>,
    pub level: Option<i32>,
}

#[derive(Debug, Deserialize)]
pub struct IdentityCreateRequest {
    pub person_id: String,
    pub name: String,
    pub type_: String,
}

#[derive(Debug, Deserialize)]
pub struct IdentityUpdateRequest {
    pub person_id: Option<String>,
    pub name: Option<String>,
    pub type_: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PersonCreateRequest {
    pub name: String,
    pub mobile: Option<String>,
    pub email: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct PersonUpdateRequest {
    pub name: Option<String>,
    pub mobile: Option<String>,
    pub email: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CustomCreateRequest {
    pub identity_id: String,
    pub field_name: String,
    pub field_value: String,
}

#[derive(Debug, Deserialize)]
pub struct CustomUpdateRequest {
    pub field_name: Option<String>,
    pub field_value: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct BindCreateRequest {
    pub identity_id: String,
    pub group_id: String,
    pub role: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct BindUpdateRequest {
    pub identity_id: Option<String>,
    pub group_id: Option<String>,
    pub role: Option<String>,
}

// ── List handlers (with soft-delete filter) ──────────────────────────────────

pub async fn definition_list(
    db: Extension<DatabaseConnection>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let models = org_definition::Entity::find()
        .filter(org_definition::Column::DeletedAt.is_null())
        .order_by_asc(org_definition::Column::Name)
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
                ("type".to_string(), Value::String(m.type_.clone())),
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

pub async fn group_list(
    db: Extension<DatabaseConnection>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let models = org_group::Entity::find()
        .filter(org_group::Column::DeletedAt.is_null())
        .order_by_asc(org_group::Column::Level)
        .order_by_asc(org_group::Column::Name)
        .limit(20)
        .all(&db.0)
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = models
        .iter()
        .map(|m| {
            let mut map = serde_json::Map::new();
            map.insert("id".to_string(), Value::String(m.id.clone()));
            map.insert("name".to_string(), Value::String(m.name.clone()));
            if let Some(val) = option_to_json(m.parent_id.clone().map(|s| Value::String(s))) {
                map.insert("parentId".to_string(), val);
            }
            map.insert("level".to_string(), Value::Number(serde_json::Number::from(m.level)));
            Value::Object(map)
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

pub async fn identity_list(
    db: Extension<DatabaseConnection>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let models = org_identity::Entity::find()
        .filter(org_identity::Column::DeletedAt.is_null())
        .order_by_asc(org_identity::Column::Name)
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
                    "personId".to_string(),
                    Value::String(m.person_id.clone()),
                ),
                ("name".to_string(), Value::String(m.name.clone())),
                ("type".to_string(), Value::String(m.type_.clone())),
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

pub async fn person_list(
    db: Extension<DatabaseConnection>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let models = org_person::Entity::find()
        .filter(org_person::Column::DeletedAt.is_null())
        .order_by_asc(org_person::Column::Name)
        .limit(20)
        .all(&db.0)
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = models
        .iter()
        .map(|m| {
            let mut map = serde_json::Map::new();
            map.insert("id".to_string(), Value::String(m.id.clone()));
            map.insert("name".to_string(), Value::String(m.name.clone()));
            if let Some(val) = option_to_json(m.mobile.clone().map(|s| Value::String(s))) {
                map.insert("mobile".to_string(), val);
            }
            if let Some(val) = option_to_json(m.email.clone().map(|s| Value::String(s))) {
                map.insert("email".to_string(), val);
            }
            Value::Object(map)
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

pub async fn custom_list(
    db: Extension<DatabaseConnection>,
    axum::extract::Path(identity_id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let models = org_custom::Entity::find()
        .filter(
            org_custom::Column::IdentityId.eq(&identity_id)
                .and(org_custom::Column::DeletedAt.is_null()),
        )
        .all(&db.0)
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = models
        .iter()
        .map(|m| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(m.id.clone())),
                (
                    "identityId".to_string(),
                    Value::String(m.identity_id.clone()),
                ),
                (
                    "fieldName".to_string(),
                    Value::String(m.field_name.clone()),
                ),
                (
                    "fieldValue".to_string(),
                    Value::String(m.field_value.clone()),
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

pub async fn bind_list(
    db: Extension<DatabaseConnection>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let models = org_bind::Entity::find()
        .filter(org_bind::Column::DeletedAt.is_null())
        .order_by_desc(org_bind::Column::CreateTime)
        .limit(20)
        .all(&db.0)
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = models
        .iter()
        .map(|m| {
            let mut map = serde_json::Map::new();
            map.insert("id".to_string(), Value::String(m.id.clone()));
            map.insert("identityId".to_string(), Value::String(m.identity_id.clone()));
            map.insert("groupId".to_string(), Value::String(m.group_id.clone()));
            if let Some(val) = option_to_json(m.role.clone().map(|s| Value::String(s))) {
                map.insert("role".to_string(), val);
            }
            Value::Object(map)
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

// ── Definition write handlers ───────────────────────────────────────────────

pub async fn definition_create(
    db: Extension<DatabaseConnection>,
    Json(req): Json<DefinitionCreateRequest>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    if req.name.trim().is_empty()
        || req.category.trim().is_empty()
        || req.type_.trim().is_empty()
    {
        return Ok(Json(ActionResult::error(
            "name, category and type are required",
        )));
    }

    let active = org_definition::ActiveModel {
        id: Set(uuid::Uuid::new_v4().to_string()),
        name: Set(req.name),
        category: Set(req.category),
        type_: Set(req.type_),
        deleted_at: Set(None),
    };
    let model = active.insert(&db.0).await.map_err(|_| AppError::Internal)?;

    let result = Value::Object(serde_json::Map::from_iter([
        ("id".to_string(), Value::String(model.id.clone())),
        ("name".to_string(), Value::String(model.name.clone())),
        ("category".to_string(), Value::String(model.category.clone())),
        ("type".to_string(), Value::String(model.type_.clone())),
    ]));

    Ok(Json(ActionResult::success(result)))
}

pub async fn definition_update(
    db: Extension<DatabaseConnection>,
    Path(id): Path<String>,
    Json(req): Json<DefinitionUpdateRequest>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let model = org_definition::Entity::find_by_id(&id)
        .one(&db.0)
        .await
        .map_err(|_| AppError::Internal)?
        .ok_or(AppError::NotFound)?;

    let mut active: org_definition::ActiveModel = model.into();
    if let Some(name) = req.name {
        active.name = Set(name);
    }
    if let Some(category) = req.category {
        active.category = Set(category);
    }
    if let Some(type_) = req.type_ {
        active.type_ = Set(type_);
    }

    let updated = active.update(&db.0).await.map_err(|_| AppError::Internal)?;

    let result = Value::Object(serde_json::Map::from_iter([
        ("id".to_string(), Value::String(updated.id.clone())),
        ("name".to_string(), Value::String(updated.name.clone())),
        ("category".to_string(), Value::String(updated.category.clone())),
        ("type".to_string(), Value::String(updated.type_.clone())),
    ]));

    Ok(Json(ActionResult::success(result)))
}

pub async fn definition_delete(
    db: Extension<DatabaseConnection>,
    Path(id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let model = org_definition::Entity::find_by_id(&id)
        .one(&db.0)
        .await
        .map_err(|_| AppError::Internal)?
        .ok_or(AppError::NotFound)?;

    let id = model.id.clone();
    let mut active: org_definition::ActiveModel = model.into();
    active.deleted_at = Set(Some(chrono::Utc::now().naive_utc()));
    active.update(&db.0).await.map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([("id".to_string(), Value::String(id))]),
    ))))
}

// ── Group write handlers ─────────────────────────────────────────────────────

pub async fn group_create(
    db: Extension<DatabaseConnection>,
    Json(req): Json<GroupCreateRequest>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    if req.name.trim().is_empty() {
        return Ok(Json(ActionResult::error("name is required")));
    }

    let active = org_group::ActiveModel {
        id: Set(uuid::Uuid::new_v4().to_string()),
        name: Set(req.name),
        parent_id: Set(req.parent_id),
        level: Set(req.level),
        deleted_at: Set(None),
    };
    let model = active.insert(&db.0).await.map_err(|_| AppError::Internal)?;

    let mut map = serde_json::Map::new();
    map.insert("id".to_string(), Value::String(model.id.clone()));
    map.insert("name".to_string(), Value::String(model.name.clone()));
    if let Some(val) = option_to_json(model.parent_id.clone().map(|s| Value::String(s))) {
        map.insert("parentId".to_string(), val);
    }
    map.insert("level".to_string(), Value::Number(serde_json::Number::from(model.level)));
    let result = Value::Object(map);

    Ok(Json(ActionResult::success(result)))
}

pub async fn group_update(
    db: Extension<DatabaseConnection>,
    Path(id): Path<String>,
    Json(req): Json<GroupUpdateRequest>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let model = org_group::Entity::find_by_id(&id)
        .one(&db.0)
        .await
        .map_err(|_| AppError::Internal)?
        .ok_or(AppError::NotFound)?;

    let mut active: org_group::ActiveModel = model.into();
    if let Some(name) = req.name {
        active.name = Set(name);
    }
    if let Some(parent_id) = req.parent_id {
        active.parent_id = Set(Some(parent_id));
    }
    if let Some(level) = req.level {
        active.level = Set(level);
    }

    let updated = active.update(&db.0).await.map_err(|_| AppError::Internal)?;

    let mut map = serde_json::Map::new();
    map.insert("id".to_string(), Value::String(updated.id.clone()));
    map.insert("name".to_string(), Value::String(updated.name.clone()));
    if let Some(val) = option_to_json(updated.parent_id.clone().map(|s| Value::String(s))) {
        map.insert("parentId".to_string(), val);
    }
    map.insert("level".to_string(), Value::Number(serde_json::Number::from(updated.level)));
    let result = Value::Object(map);

    Ok(Json(ActionResult::success(result)))
}

pub async fn group_delete(
    db: Extension<DatabaseConnection>,
    Path(id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let model = org_group::Entity::find_by_id(&id)
        .one(&db.0)
        .await
        .map_err(|_| AppError::Internal)?
        .ok_or(AppError::NotFound)?;

    let id = model.id.clone();
    let mut active: org_group::ActiveModel = model.into();
    active.deleted_at = Set(Some(chrono::Utc::now().naive_utc()));
    active.update(&db.0).await.map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([("id".to_string(), Value::String(id))]),
    ))))
}

// ── Identity write handlers ─────────────────────────────────────────────────

pub async fn identity_create(
    db: Extension<DatabaseConnection>,
    Json(req): Json<IdentityCreateRequest>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    if req.person_id.trim().is_empty()
        || req.name.trim().is_empty()
        || req.type_.trim().is_empty()
    {
        return Ok(Json(ActionResult::error(
            "person_id, name and type are required",
        )));
    }

    let active = org_identity::ActiveModel {
        id: Set(uuid::Uuid::new_v4().to_string()),
        person_id: Set(req.person_id),
        name: Set(req.name),
        type_: Set(req.type_),
        deleted_at: Set(None),
    };
    let model = active.insert(&db.0).await.map_err(|_| AppError::Internal)?;

    let result = Value::Object(serde_json::Map::from_iter([
        ("id".to_string(), Value::String(model.id.clone())),
        (
            "personId".to_string(),
            Value::String(model.person_id.clone()),
        ),
        ("name".to_string(), Value::String(model.name.clone())),
        ("type".to_string(), Value::String(model.type_.clone())),
    ]));

    Ok(Json(ActionResult::success(result)))
}

pub async fn identity_update(
    db: Extension<DatabaseConnection>,
    Path(id): Path<String>,
    Json(req): Json<IdentityUpdateRequest>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let model = org_identity::Entity::find_by_id(&id)
        .one(&db.0)
        .await
        .map_err(|_| AppError::Internal)?
        .ok_or(AppError::NotFound)?;

    let mut active: org_identity::ActiveModel = model.into();
    if let Some(person_id) = req.person_id {
        active.person_id = Set(person_id);
    }
    if let Some(name) = req.name {
        active.name = Set(name);
    }
    if let Some(type_) = req.type_ {
        active.type_ = Set(type_);
    }

    let updated = active.update(&db.0).await.map_err(|_| AppError::Internal)?;

    let result = Value::Object(serde_json::Map::from_iter([
        ("id".to_string(), Value::String(updated.id.clone())),
        (
            "personId".to_string(),
            Value::String(updated.person_id.clone()),
        ),
        ("name".to_string(), Value::String(updated.name.clone())),
        ("type".to_string(), Value::String(updated.type_.clone())),
    ]));

    Ok(Json(ActionResult::success(result)))
}

pub async fn identity_delete(
    db: Extension<DatabaseConnection>,
    Path(id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let model = org_identity::Entity::find_by_id(&id)
        .one(&db.0)
        .await
        .map_err(|_| AppError::Internal)?
        .ok_or(AppError::NotFound)?;

    let id = model.id.clone();
    let mut active: org_identity::ActiveModel = model.into();
    active.deleted_at = Set(Some(chrono::Utc::now().naive_utc()));
    active.update(&db.0).await.map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([("id".to_string(), Value::String(id))]),
    ))))
}

// ── Person write handlers ───────────────────────────────────────────────────

pub async fn person_create(
    db: Extension<DatabaseConnection>,
    Json(req): Json<PersonCreateRequest>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    if req.name.trim().is_empty() {
        return Ok(Json(ActionResult::error("name is required")));
    }

    let active = org_person::ActiveModel {
        id: Set(uuid::Uuid::new_v4().to_string()),
        name: Set(req.name),
        mobile: Set(req.mobile),
        email: Set(req.email),
        deleted_at: Set(None),
    };
    let model = active.insert(&db.0).await.map_err(|_| AppError::Internal)?;

    let mut map = serde_json::Map::new();
    map.insert("id".to_string(), Value::String(model.id.clone()));
    map.insert("name".to_string(), Value::String(model.name.clone()));
    if let Some(val) = option_to_json(model.mobile.clone().map(|s| Value::String(s))) {
        map.insert("mobile".to_string(), val);
    }
    if let Some(val) = option_to_json(model.email.clone().map(|s| Value::String(s))) {
        map.insert("email".to_string(), val);
    }
    let result = Value::Object(map);

    Ok(Json(ActionResult::success(result)))
}

pub async fn person_update(
    db: Extension<DatabaseConnection>,
    Path(id): Path<String>,
    Json(req): Json<PersonUpdateRequest>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let model = org_person::Entity::find_by_id(&id)
        .one(&db.0)
        .await
        .map_err(|_| AppError::Internal)?
        .ok_or(AppError::NotFound)?;

    let mut active: org_person::ActiveModel = model.into();
    if let Some(name) = req.name {
        active.name = Set(name);
    }
    if let Some(mobile) = req.mobile {
        active.mobile = Set(Some(mobile));
    }
    if let Some(email) = req.email {
        active.email = Set(Some(email));
    }

    let updated = active.update(&db.0).await.map_err(|_| AppError::Internal)?;

    let mut map = serde_json::Map::new();
    map.insert("id".to_string(), Value::String(updated.id.clone()));
    map.insert("name".to_string(), Value::String(updated.name.clone()));
    if let Some(val) = option_to_json(updated.mobile.clone().map(|s| Value::String(s))) {
        map.insert("mobile".to_string(), val);
    }
    if let Some(val) = option_to_json(updated.email.clone().map(|s| Value::String(s))) {
        map.insert("email".to_string(), val);
    }
    let result = Value::Object(map);

    Ok(Json(ActionResult::success(result)))
}

pub async fn person_delete(
    db: Extension<DatabaseConnection>,
    Path(id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let model = org_person::Entity::find_by_id(&id)
        .one(&db.0)
        .await
        .map_err(|_| AppError::Internal)?
        .ok_or(AppError::NotFound)?;

    let id = model.id.clone();
    let mut active: org_person::ActiveModel = model.into();
    active.deleted_at = Set(Some(chrono::Utc::now().naive_utc()));
    active.update(&db.0).await.map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([("id".to_string(), Value::String(id))]),
    ))))
}

// ── Custom write handlers ───────────────────────────────────────────────────

pub async fn custom_create(
    db: Extension<DatabaseConnection>,
    Json(req): Json<CustomCreateRequest>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    if req.identity_id.trim().is_empty()
        || req.field_name.trim().is_empty()
        || req.field_value.trim().is_empty()
    {
        return Ok(Json(ActionResult::error(
            "identity_id, field_name and field_value are required",
        )));
    }

    let active = org_custom::ActiveModel {
        id: Set(uuid::Uuid::new_v4().to_string()),
        identity_id: Set(req.identity_id),
        field_name: Set(req.field_name),
        field_value: Set(req.field_value),
        deleted_at: Set(None),
    };
    let model = active.insert(&db.0).await.map_err(|_| AppError::Internal)?;

    let result = Value::Object(serde_json::Map::from_iter([
        ("id".to_string(), Value::String(model.id.clone())),
        (
            "identityId".to_string(),
            Value::String(model.identity_id.clone()),
        ),
        (
            "fieldName".to_string(),
            Value::String(model.field_name.clone()),
        ),
        (
            "fieldValue".to_string(),
            Value::String(model.field_value.clone()),
        ),
    ]));

    Ok(Json(ActionResult::success(result)))
}

pub async fn custom_update(
    db: Extension<DatabaseConnection>,
    Path(id): Path<String>,
    Json(req): Json<CustomUpdateRequest>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let model = org_custom::Entity::find_by_id(&id)
        .one(&db.0)
        .await
        .map_err(|_| AppError::Internal)?
        .ok_or(AppError::NotFound)?;

    let mut active: org_custom::ActiveModel = model.into();
    if let Some(field_name) = req.field_name {
        active.field_name = Set(field_name);
    }
    if let Some(field_value) = req.field_value {
        active.field_value = Set(field_value);
    }

    let updated = active.update(&db.0).await.map_err(|_| AppError::Internal)?;

    let result = Value::Object(serde_json::Map::from_iter([
        ("id".to_string(), Value::String(updated.id.clone())),
        (
            "identityId".to_string(),
            Value::String(updated.identity_id.clone()),
        ),
        (
            "fieldName".to_string(),
            Value::String(updated.field_name.clone()),
        ),
        (
            "fieldValue".to_string(),
            Value::String(updated.field_value.clone()),
        ),
    ]));

    Ok(Json(ActionResult::success(result)))
}

pub async fn custom_delete(
    db: Extension<DatabaseConnection>,
    Path(id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let model = org_custom::Entity::find_by_id(&id)
        .one(&db.0)
        .await
        .map_err(|_| AppError::Internal)?
        .ok_or(AppError::NotFound)?;

    let id = model.id.clone();
    let mut active: org_custom::ActiveModel = model.into();
    active.deleted_at = Set(Some(chrono::Utc::now().naive_utc()));
    active.update(&db.0).await.map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([("id".to_string(), Value::String(id))]),
    ))))
}

// ── Bind write handlers ─────────────────────────────────────────────────────

pub async fn bind_create(
    db: Extension<DatabaseConnection>,
    Json(req): Json<BindCreateRequest>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    if req.identity_id.trim().is_empty() || req.group_id.trim().is_empty() {
        return Ok(Json(ActionResult::error(
            "identity_id and group_id are required",
        )));
    }

    let active = org_bind::ActiveModel {
        id: Set(uuid::Uuid::new_v4().to_string()),
        identity_id: Set(req.identity_id),
        group_id: Set(req.group_id),
        role: Set(req.role),
        create_time: Set(None),
        deleted_at: Set(None),
    };
    let model = active.insert(&db.0).await.map_err(|_| AppError::Internal)?;

    let mut map = serde_json::Map::new();
    map.insert("id".to_string(), Value::String(model.id.clone()));
    map.insert("identityId".to_string(), Value::String(model.identity_id.clone()));
    map.insert("groupId".to_string(), Value::String(model.group_id.clone()));
    if let Some(val) = option_to_json(model.role.clone().map(|s| Value::String(s))) {
        map.insert("role".to_string(), val);
    }
    let result = Value::Object(map);

    Ok(Json(ActionResult::success(result)))
}

pub async fn bind_update(
    db: Extension<DatabaseConnection>,
    Path(id): Path<String>,
    Json(req): Json<BindUpdateRequest>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let model = org_bind::Entity::find_by_id(&id)
        .one(&db.0)
        .await
        .map_err(|_| AppError::Internal)?
        .ok_or(AppError::NotFound)?;

    let mut active: org_bind::ActiveModel = model.into();
    if let Some(identity_id) = req.identity_id {
        active.identity_id = Set(identity_id);
    }
    if let Some(group_id) = req.group_id {
        active.group_id = Set(group_id);
    }
    if let Some(role) = req.role {
        active.role = Set(Some(role));
    }

    let updated = active.update(&db.0).await.map_err(|_| AppError::Internal)?;

    let mut map = serde_json::Map::new();
    map.insert("id".to_string(), Value::String(updated.id.clone()));
    map.insert("identityId".to_string(), Value::String(updated.identity_id.clone()));
    map.insert("groupId".to_string(), Value::String(updated.group_id.clone()));
    if let Some(val) = option_to_json(updated.role.clone().map(|s| Value::String(s))) {
        map.insert("role".to_string(), val);
    }
    let result = Value::Object(map);

    Ok(Json(ActionResult::success(result)))
}

pub async fn bind_delete(
    db: Extension<DatabaseConnection>,
    Path(id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let model = org_bind::Entity::find_by_id(&id)
        .one(&db.0)
        .await
        .map_err(|_| AppError::Internal)?
        .ok_or(AppError::NotFound)?;

    let id = model.id.clone();
    let mut active: org_bind::ActiveModel = model.into();
    active.deleted_at = Set(Some(chrono::Utc::now().naive_utc()));
    active.update(&db.0).await.map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([("id".to_string(), Value::String(id))]),
    ))))
}

// ── Router ───────────────────────────────────────────────────────────────────

pub fn organization_core_entity_router(_pool: deadpool_postgres::Pool) -> Router {
    // NOTE: DatabaseConnection must be provided externally via Extension.
    // The pool is kept for backwards compatibility with crates that still use it.
    Router::new()
        // definition
        .route("/jaxrs/organization/definition/list", get(definition_list))
        .route("/jaxrs/organization/definition", post(definition_create))
        .route(
            "/jaxrs/organization/definition/{id}",
            put(definition_update).delete(definition_delete),
        )
        // group
        .route("/jaxrs/organization/group/list", get(group_list))
        .route("/jaxrs/organization/group", post(group_create))
        .route("/jaxrs/organization/group/{id}", put(group_update).delete(group_delete))
        // identity
        .route("/jaxrs/organization/identity/list", get(identity_list))
        .route(
            "/jaxrs/organization/identity",
            post(identity_create),
        )
        .route(
            "/jaxrs/organization/identity/{id}",
            put(identity_update).delete(identity_delete),
        )
        // person
        .route("/jaxrs/organization/person/list", get(person_list))
        .route("/jaxrs/organization/person", post(person_create))
        .route(
            "/jaxrs/organization/person/{id}",
            put(person_update).delete(person_delete),
        )
        // custom
        .route(
            "/jaxrs/organization/custom/list/{identityId}",
            get(custom_list),
        )
        .route("/jaxrs/organization/custom", post(custom_create))
        .route(
            "/jaxrs/organization/custom/{id}",
            put(custom_update).delete(custom_delete),
        )
        // bind
        .route("/jaxrs/organization/bind/list", get(bind_list))
        .route("/jaxrs/organization/bind", post(bind_create))
        .route("/jaxrs/organization/bind/{id}", put(bind_update).delete(bind_delete))
}

#[cfg(test)]
mod tests;
#[cfg(test)]
mod tests_generated;


pub fn router(pool: deadpool_postgres::Pool) -> axum::Router {
    crate::organization_core_entity_router(pool)
}
