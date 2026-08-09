use axum::{
    extract::Extension,
    routing::get,
    Json, Router,
};
use deadpool_postgres::Pool;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, QueryOrder, QuerySelect};
use serde_json::Value;
use shared::{error::AppError, response::ActionResult};

use crate::entities::{file_file, file_folder};

pub mod entities;
pub mod routes;

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct FolderInfo {
    pub id: String,
    pub name: String,
    pub person: String,
    pub superior: Option<String>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct FileInfo {
    pub id: String,
    pub name: String,
    pub person: String,
    pub reference_type: String,
    pub extension: String,
    pub length: i64,
}

pub async fn folder_list_top(
    Extension(db): Extension<DatabaseConnection>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let models = file_folder::Entity::find()
        .filter(
            file_folder::Column::Superior
                .is_in(vec![""])
                .or(file_folder::Column::Superior.is_null()),
        )
        .order_by_asc(file_folder::Column::Name)
        .limit(50)
        .all(&db)
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = models
        .iter()
        .map(|m| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(m.id.clone())),
                ("name".to_string(), Value::String(m.name.clone())),
                ("person".to_string(), Value::String(m.person.clone())),
                (
                    "superior".to_string(),
                    m.superior
                        .clone()
                        .map(Value::String)
                        .unwrap_or(Value::Null),
                ),
                ("attachmentCount".to_string(), Value::Number(serde_json::Number::from(0))),
                ("size".to_string(), Value::Number(serde_json::Number::from(0))),
                ("folderCount".to_string(), Value::Number(serde_json::Number::from(0))),
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

pub async fn folder_list_with_folder(
    Extension(db): Extension<DatabaseConnection>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let models = file_folder::Entity::find()
        .filter(file_folder::Column::Superior.eq(id))
        .order_by_asc(file_folder::Column::Name)
        .limit(50)
        .all(&db)
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = models
        .iter()
        .map(|m| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(m.id.clone())),
                ("name".to_string(), Value::String(m.name.clone())),
                ("person".to_string(), Value::String(m.person.clone())),
                (
                    "superior".to_string(),
                    m.superior
                        .clone()
                        .map(Value::String)
                        .unwrap_or(Value::Null),
                ),
                ("attachmentCount".to_string(), Value::Number(serde_json::Number::from(0))),
                ("size".to_string(), Value::Number(serde_json::Number::from(0))),
                ("folderCount".to_string(), Value::Number(serde_json::Number::from(0))),
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

pub async fn file_list(
    Extension(db): Extension<DatabaseConnection>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let models = file_file::Entity::find()
        .order_by_asc(file_file::Column::Name)
        .limit(50)
        .all(&db)
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = models
        .iter()
        .map(|m| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(m.id.clone())),
                ("name".to_string(), Value::String(m.name.clone())),
                ("person".to_string(), Value::String(m.person.clone())),
                ("referenceType".to_string(), Value::String(m.reference_type.clone())),
                (
                    "extension".to_string(),
                    serde_json::Value::String(
                        m.extension
                            .clone()
                            .unwrap_or_default(),
                    ),
                ),
                (
                    "length".to_string(),
                    Value::Number(serde_json::Number::from(m.length)),
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

pub async fn complex_top(
    Extension(db): Extension<DatabaseConnection>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let folder_models = file_folder::Entity::find()
        .filter(
            file_folder::Column::Superior
                .is_in(vec![""])
                .or(file_folder::Column::Superior.is_null()),
        )
        .order_by_asc(file_folder::Column::Name)
        .limit(20)
        .all(&db)
        .await
        .map_err(|_| AppError::Internal)?;

    let folder_list: Vec<Value> = folder_models
        .iter()
        .map(|m| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(m.id.clone())),
                ("name".to_string(), Value::String(m.name.clone())),
                ("person".to_string(), Value::String(m.person.clone())),
                (
                    "superior".to_string(),
                    m.superior
                        .clone()
                        .map(Value::String)
                        .unwrap_or(Value::Null),
                ),
                ("attachmentCount".to_string(), Value::Number(serde_json::Number::from(0))),
                ("size".to_string(), Value::Number(serde_json::Number::from(0))),
                ("folderCount".to_string(), Value::Number(serde_json::Number::from(0))),
            ]))
        })
        .collect();

    let file_models = file_file::Entity::find()
        .order_by_asc(file_file::Column::Name)
        .limit(20)
        .all(&db)
        .await
        .map_err(|_| AppError::Internal)?;

    let attachment_list: Vec<Value> = file_models
        .iter()
        .map(|m| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(m.id.clone())),
                ("name".to_string(), Value::String(m.name.clone())),
                ("person".to_string(), Value::String(m.person.clone())),
                ("referenceType".to_string(), Value::String(m.reference_type.clone())),
                (
                    "extension".to_string(),
                    serde_json::Value::String(
                        m.extension
                            .clone()
                            .unwrap_or_default(),
                    ),
                ),
                (
                    "length".to_string(),
                    Value::Number(serde_json::Number::from(m.length)),
                ),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("folderList".to_string(), Value::Array(folder_list)),
            ("attachmentList".to_string(), Value::Array(attachment_list)),
        ]),
    ))))
}

pub fn file_core_entity_router(_pool: Pool) -> Router {
    let db = tokio::runtime::Handle::current()
        .block_on(shared::db::create_sea_orm_pool())
        .expect("failed to create sea-orm connection");

    Router::new()
        .route("/jaxrs/file/core/entity/folder/list/top", get(folder_list_top))
        .route(
            "/jaxrs/file/core/entity/folder/list/{id}",
            get(folder_list_with_folder),
        )
        .route("/jaxrs/file/core/entity/file/list", get(file_list))
        .route("/jaxrs/file/core/entity/complex/top", get(complex_top))
        .layer(Extension(db))
}

#[cfg(test)]
mod tests;

pub fn router(pool: deadpool_postgres::Pool) -> axum::Router {
    crate::file_core_entity_router(pool)
}
