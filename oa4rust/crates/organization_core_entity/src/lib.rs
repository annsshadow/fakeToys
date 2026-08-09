use axum::{
    extract::Extension,
    routing::get,
    Json, Router,
};
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, QueryOrder, QuerySelect};
use serde_json::Value;
use shared::{error::AppError, response::ActionResult};

pub mod entities;
pub mod routes;

use entities::{
    org_bind, org_custom, org_definition, org_group, org_identity, org_person,
};

pub async fn definition_list(
    db: Extension<DatabaseConnection>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let models = org_definition::Entity::find()
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
        .order_by_asc(org_group::Column::Level)
        .order_by_asc(org_group::Column::Name)
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
                    "parentId".to_string(),
                    m.parent_id
                        .clone()
                        .map(Value::String)
                        .unwrap_or(Value::Null),
                ),
                (
                    "level".to_string(),
                    Value::Number(serde_json::Number::from(m.level)),
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

pub async fn identity_list(
    db: Extension<DatabaseConnection>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let models = org_identity::Entity::find()
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
        .order_by_asc(org_person::Column::Name)
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
                    "mobile".to_string(),
                    m.mobile
                        .clone()
                        .map(Value::String)
                        .unwrap_or(Value::Null),
                ),
                (
                    "email".to_string(),
                    m.email
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

pub async fn custom_list(
    db: Extension<DatabaseConnection>,
    axum::extract::Path(identity_id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let models = org_custom::Entity::find()
        .filter(org_custom::Column::IdentityId.eq(&identity_id))
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
        .order_by_desc(org_bind::Column::CreateTime)
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
                    "identityId".to_string(),
                    Value::String(m.identity_id.clone()),
                ),
                ("groupId".to_string(), Value::String(m.group_id.clone())),
                (
                    "role".to_string(),
                    m.role
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

pub fn organization_core_entity_router(_pool: deadpool_postgres::Pool) -> Router {
    // NOTE: DatabaseConnection must be provided externally via Extension.
    // The pool is kept for backwards compatibility with crates that still use it.
    Router::new()
        .route("/jaxrs/organization/definition/list", get(definition_list))
        .route("/jaxrs/organization/group/list", get(group_list))
        .route("/jaxrs/organization/identity/list", get(identity_list))
        .route("/jaxrs/organization/person/list", get(person_list))
        .route(
            "/jaxrs/organization/custom/list/{identityId}",
            get(custom_list),
        )
        .route("/jaxrs/organization/bind/list", get(bind_list))
}

#[cfg(test)]
mod tests;

pub fn router(pool: deadpool_postgres::Pool) -> axum::Router {
    crate::organization_core_entity_router(pool)
}
