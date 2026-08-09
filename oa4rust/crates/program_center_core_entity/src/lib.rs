use axum::{
    extract::Extension,
    routing::get,
    Json, Router,
};
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, QueryOrder};
use shared::{
    error::AppError, response::ActionResult,
};
use serde_json::Value;

pub mod entities;
pub mod routes;

pub async fn application_list(
    db: Extension<DatabaseConnection>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let models = entities::application::Entity::find()
        .order_by_asc(entities::application::Column::Name)
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
        .all(&db.0)
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = models
        .iter()
        .map(|m| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(m.id.clone())),
                ("name".to_string(), Value::String(m.name.clone())),
                ("application".to_string(), Value::String(m.id.clone())),
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
                    Value::String(
                        m.extension
                            .clone()
                            .unwrap_or_default(),
                    ),
                ),
                ("storage".to_string(), Value::String(m.storage.clone())),
                (
                    "length".to_string(),
                    m.length
                        .map(|v| Value::Number(serde_json::Number::from(v)))
                        .unwrap_or(Value::Null),
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

pub fn program_center_core_entity_router(_pool: deadpool_postgres::Pool) -> Router {
    let db = tokio::runtime::Handle::current()
        .block_on(shared::db::create_sea_orm_pool())
        .expect("failed to create sea-orm connection");

    Router::new()
        .route("/jaxrs/program_center/application/list", get(application_list))
        .route("/jaxrs/program_center/script/list", get(script_list))
        .route("/jaxrs/program_center/invoke/list", get(invoke_list))
        .route("/jaxrs/program_center/agent/list", get(agent_list))
        .route("/jaxrs/program_center/structure/list", get(structure_list))
        .layer(Extension(db))
}

#[cfg(test)]
pub fn program_center_mock_router(_db: DatabaseConnection) -> Router {
    Router::new()
        .route("/jaxrs/program_center/application/list", get(application_list))
        .route("/jaxrs/program_center/script/list", get(script_list))
        .layer(Extension(_db))
}

pub fn router(pool: deadpool_postgres::Pool) -> axum::Router {
    crate::program_center_core_entity_router(pool)
}

#[cfg(test)]
mod tests;
