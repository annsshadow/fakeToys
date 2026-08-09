use axum::{
    extract::{Extension, Path},
    routing::{delete, get, post},
    Json, Router,
};
use deadpool_postgres::Pool;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, QueryOrder,
    QuerySelect, Set,
};
use serde::Deserialize;
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

// ── Request structs ──────────────────────────────────────────────────────────

#[derive(Debug, Deserialize)]
pub struct FolderCreateRequest {
    pub name: String,
    pub person: String,
    pub superior: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct FileCreateRequest {
    pub name: String,
    pub person: String,
    pub reference_type: String,
    pub reference_id: Option<String>,
    pub extension: Option<String>,
    pub length: Option<i64>,
    pub mime_type: Option<String>,
}

// ── List handlers (with soft-delete filter) ──────────────────────────────────

pub async fn folder_list_top(
    Extension(db): Extension<DatabaseConnection>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let models = file_folder::Entity::find()
        .filter(
            file_folder::Column::Superior
                .is_in(vec![""])
                .or(file_folder::Column::Superior.is_null())
                .and(file_folder::Column::DeletedAt.is_null()),
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
        .filter(
            file_folder::Column::Superior.eq(id)
                .and(file_folder::Column::DeletedAt.is_null()),
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

pub async fn file_list(
    Extension(db): Extension<DatabaseConnection>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let models = file_file::Entity::find()
        .filter(file_file::Column::DeletedAt.is_null())
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
                .or(file_folder::Column::Superior.is_null())
                .and(file_folder::Column::DeletedAt.is_null()),
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
        .filter(file_file::Column::DeletedAt.is_null())
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

// ── Folder create handler ───────────────────────────────────────────────────

pub async fn folder_create(
    Extension(db): Extension<DatabaseConnection>,
    Json(req): Json<FolderCreateRequest>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    if req.name.trim().is_empty() || req.person.trim().is_empty() {
        return Ok(Json(ActionResult::error("name and person are required")));
    }

    let active = file_folder::ActiveModel {
        id: Set(uuid::Uuid::new_v4().to_string()),
        name: Set(req.name),
        person: Set(req.person),
        superior: Set(req.superior),
        create_time: Set(None),
        update_time: Set(None),
        deleted_at: Set(None),
    };
    let model = active.insert(&db).await.map_err(|_| AppError::Internal)?;

    let result = Value::Object(serde_json::Map::from_iter([
        ("id".to_string(), Value::String(model.id.clone())),
        ("name".to_string(), Value::String(model.name.clone())),
        ("person".to_string(), Value::String(model.person.clone())),
        (
            "superior".to_string(),
            model
                .superior
                .clone()
                .map(Value::String)
                .unwrap_or(Value::Null),
        ),
    ]));

    Ok(Json(ActionResult::success(result)))
}

// ── Folder delete handler ───────────────────────────────────────────────────

pub async fn folder_delete(
    Extension(db): Extension<DatabaseConnection>,
    Path(id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let model = file_folder::Entity::find_by_id(&id)
        .one(&db)
        .await
        .map_err(|_| AppError::Internal)?
        .ok_or(AppError::NotFound)?;

    let mut active: file_folder::ActiveModel = model.into();
    active.deleted_at = Set(Some(chrono::Utc::now().naive_utc()));
    active.update(&db).await.map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([("deleted".to_string(), Value::Bool(true))]),
    ))))
}

// ── File create handler ─────────────────────────────────────────────────────

pub async fn file_create(
    Extension(db): Extension<DatabaseConnection>,
    Json(req): Json<FileCreateRequest>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    if req.name.trim().is_empty()
        || req.person.trim().is_empty()
        || req.reference_type.trim().is_empty()
    {
        return Ok(Json(ActionResult::error(
            "name, person and reference_type are required",
        )));
    }

    let active = file_file::ActiveModel {
        id: Set(uuid::Uuid::new_v4().to_string()),
        name: Set(req.name),
        person: Set(req.person),
        reference_id: Set(req.reference_id),
        reference_type: Set(req.reference_type),
        extension: Set(req.extension),
        length: Set(req.length.unwrap_or(0)),
        mime_type: Set(req.mime_type),
        create_time: Set(None),
        update_time: Set(None),
        deleted_at: Set(None),
    };
    let model = active.insert(&db).await.map_err(|_| AppError::Internal)?;

    let result = Value::Object(serde_json::Map::from_iter([
        ("id".to_string(), Value::String(model.id.clone())),
        ("name".to_string(), Value::String(model.name.clone())),
        ("person".to_string(), Value::String(model.person.clone())),
        ("referenceType".to_string(), Value::String(model.reference_type.clone())),
        (
            "extension".to_string(),
            serde_json::Value::String(
                model
                    .extension
                    .clone()
                    .unwrap_or_default(),
            ),
        ),
        ("length".to_string(), Value::Number(serde_json::Number::from(model.length))),
    ]));

    Ok(Json(ActionResult::success(result)))
}

// ── Router ───────────────────────────────────────────────────────────────────

pub fn file_core_entity_router(_pool: Pool) -> Router {
    let db = std::panic::catch_unwind(|| {
        tokio::runtime::Handle::current()
            .block_on(shared::db::create_sea_orm_pool())
    })
    .ok()
    .and_then(|r| r.ok());

    let router = Router::new()
        // folder
        .route("/jaxrs/file/core/entity/folder/list/top", get(folder_list_top))
        .route(
            "/jaxrs/file/core/entity/folder/list/{id}",
            get(folder_list_with_folder),
        )
        .route(
            "/jaxrs/file/core/entity/folder",
            post(folder_create),
        )
        .route(
            "/jaxrs/file/core/entity/folder/{id}",
            delete(folder_delete),
        )
        // file
        .route("/jaxrs/file/core/entity/file/list", get(file_list))
        .route("/jaxrs/file/core/entity/file", post(file_create))
        // complex
        .route("/jaxrs/file/core/entity/complex/top", get(complex_top));
    match db {
        Some(conn) => router.layer(Extension(conn)),
        None => router,
    }
}

#[cfg(test)]
mod tests;

pub fn router(pool: deadpool_postgres::Pool) -> axum::Router {
    crate::file_core_entity_router(pool)
}
