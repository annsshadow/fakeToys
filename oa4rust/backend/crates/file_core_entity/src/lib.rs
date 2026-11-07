// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

use axum::{
    extract::{Extension, Path},
    routing::{delete, get, post},
    Json, Router,
};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, ConnectionTrait, DatabaseConnection, DbBackend, EntityTrait,
    QueryFilter, QueryOrder, QuerySelect, Set, Statement,
};
use serde::Deserialize;
use serde_json::Value;
use shared::{error::AppError, response::ActionResult};

use crate::entities::{file_file, file_folder};

/// 安全审计（round4-b78）：file entity 3 把 sea_orm CRUD 真写收"会话存在档"门禁。
/// 无会话（Option<Extension<Session>> = None）→ 401，门置于首个 DB 访问
/// （`ActiveModel::insert` / `Entity::find_by_id().one()`）之前，杜绝匿名写
/// `file_folder` / `file_file`。这些是 sea_orm ORM 写，审计尺子"含写"指标
/// 不可见（ORM-blindness，§五 尺子缺陷 28），R7 降但含写不变属预期。
fn require_session(
    session: Option<Extension<shared::session::Session>>,
) -> Result<shared::session::Session, AppError> {
    session.map(|Extension(s)| s).ok_or(AppError::Unauthorized)
}

/// 归属谓词（`$2` = 会话身份）：`file_folder.person` 为空/不等不命中（fail-closed）；
/// 管理员可维护他人目录。谓词写进 SQL `WHERE`，不做先查后判。
const FOLDER_OWNER_PREDICATE: &str = "id = $1 AND (person = $2 \
  OR EXISTS(\
    SELECT 1 FROM auth_person p \
      JOIN auth_person_role pr ON pr.person_id = p.id \
      JOIN auth_role r ON r.id = pr.role_id \
     WHERE (p.unique_id = $2 OR p.id = $2) \
       AND r.name = 'admin' AND r.deleted_at IS NULL AND r.disable = false))";

/// 0 行命中时区分 403/404。该读**不参与授权**：判定已由带谓词的写语句完成。
async fn ownership_error(db: &DatabaseConnection, id: &str) -> AppError {
    let stmt = Statement::from_sql_and_values(
        DbBackend::Postgres,
        "SELECT EXISTS(SELECT 1 FROM file_folder WHERE id = $1) AS row_exists",
        [id.into()],
    );
    match db.query_one(stmt).await {
        Ok(Some(row)) => match row.try_get::<bool>("", "row_exists") {
            Ok(true) => AppError::Forbidden,
            Ok(false) => AppError::NotFound,
            Err(e) => AppError::internal_with_source(concat!(file!(), ":", line!()), e),
        },
        Ok(None) => AppError::NotFound,
        Err(e) => AppError::internal_with_source(concat!(file!(), ":", line!()), e),
    }
}

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
#[derive(utoipa::ToSchema)]
// ── Request structs ──────────────────────────────────────────────────────────
#[derive(Debug, Deserialize)]
pub struct FolderCreateRequest {
    pub name: String,
    pub person: String,
    pub superior: Option<String>,
}
#[derive(utoipa::ToSchema, Debug, Deserialize)]
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

#[allow(non_snake_case)]
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
        .map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;

    let data: Vec<Value> = models
        .iter()
        .map(|m| {
            let mut entries = vec![
                ("id".to_string(), Value::String(m.id.clone())),
                ("name".to_string(), Value::String(m.name.clone())),
                ("person".to_string(), Value::String(m.person.clone())),
                (
                    "attachmentCount".to_string(),
                    Value::Number(serde_json::Number::from(0)),
                ),
                (
                    "size".to_string(),
                    Value::Number(serde_json::Number::from(0)),
                ),
                (
                    "folderCount".to_string(),
                    Value::Number(serde_json::Number::from(0)),
                ),
            ];
            if let Some(superior) = m.superior.clone() {
                entries.insert(3, ("superior".to_string(), Value::String(superior)));
            }
            Value::Object(serde_json::Map::from_iter(entries))
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

#[allow(non_snake_case)]
pub async fn folder_list_with_folder(
    Extension(db): Extension<DatabaseConnection>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let models = file_folder::Entity::find()
        .filter(
            file_folder::Column::Superior
                .eq(id)
                .and(file_folder::Column::DeletedAt.is_null()),
        )
        .order_by_asc(file_folder::Column::Name)
        .limit(50)
        .all(&db)
        .await
        .map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;

    let data: Vec<Value> = models
        .iter()
        .map(|m| {
            let mut entries = vec![
                ("id".to_string(), Value::String(m.id.clone())),
                ("name".to_string(), Value::String(m.name.clone())),
                ("person".to_string(), Value::String(m.person.clone())),
                (
                    "attachmentCount".to_string(),
                    Value::Number(serde_json::Number::from(0)),
                ),
                (
                    "size".to_string(),
                    Value::Number(serde_json::Number::from(0)),
                ),
                (
                    "folderCount".to_string(),
                    Value::Number(serde_json::Number::from(0)),
                ),
            ];
            if let Some(superior) = m.superior.clone() {
                entries.insert(3, ("superior".to_string(), Value::String(superior)));
            }
            Value::Object(serde_json::Map::from_iter(entries))
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

#[allow(non_snake_case)]
pub async fn file_list(
    Extension(db): Extension<DatabaseConnection>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let models = file_file::Entity::find()
        .filter(file_file::Column::DeletedAt.is_null())
        .order_by_asc(file_file::Column::Name)
        .limit(50)
        .all(&db)
        .await
        .map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;

    let data: Vec<Value> = models
        .iter()
        .map(|m| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(m.id.clone())),
                ("name".to_string(), Value::String(m.name.clone())),
                ("person".to_string(), Value::String(m.person.clone())),
                (
                    "referenceType".to_string(),
                    Value::String(m.reference_type.clone()),
                ),
                (
                    "extension".to_string(),
                    serde_json::Value::String(m.extension.clone().unwrap_or_default()),
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

#[allow(non_snake_case)]
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
        .map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;

    let folder_list: Vec<Value> = folder_models
        .iter()
        .map(|m| {
            let mut entries = vec![
                ("id".to_string(), Value::String(m.id.clone())),
                ("name".to_string(), Value::String(m.name.clone())),
                ("person".to_string(), Value::String(m.person.clone())),
                (
                    "attachmentCount".to_string(),
                    Value::Number(serde_json::Number::from(0)),
                ),
                (
                    "size".to_string(),
                    Value::Number(serde_json::Number::from(0)),
                ),
                (
                    "folderCount".to_string(),
                    Value::Number(serde_json::Number::from(0)),
                ),
            ];
            if let Some(superior) = m.superior.clone() {
                entries.insert(3, ("superior".to_string(), Value::String(superior)));
            }
            Value::Object(serde_json::Map::from_iter(entries))
        })
        .collect();

    let file_models = file_file::Entity::find()
        .filter(file_file::Column::DeletedAt.is_null())
        .order_by_asc(file_file::Column::Name)
        .limit(20)
        .all(&db)
        .await
        .map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;

    let attachment_list: Vec<Value> = file_models
        .iter()
        .map(|m| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(m.id.clone())),
                ("name".to_string(), Value::String(m.name.clone())),
                ("person".to_string(), Value::String(m.person.clone())),
                (
                    "referenceType".to_string(),
                    Value::String(m.reference_type.clone()),
                ),
                (
                    "extension".to_string(),
                    serde_json::Value::String(m.extension.clone().unwrap_or_default()),
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

#[allow(non_snake_case)]
pub async fn folder_create(
    Extension(db): Extension<DatabaseConnection>,
    session: Option<Extension<shared::session::Session>>,
    Json(req): Json<FolderCreateRequest>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let session = require_session(session)?;
    if req.name.trim().is_empty() || req.person.trim().is_empty() {
        return Ok(Json(ActionResult::error("name and person are required")));
    }
    // 冒名目录写入直接拒绝：person 必须等于会话人（§9.3 W1 `person` 口径）。
    if req.person.trim() != session.person_unique {
        return Err(AppError::Forbidden);
    }

    let person = session.person_unique.clone();
    let active = file_folder::ActiveModel {
        id: Set(uuid::Uuid::new_v4().to_string()),
        name: Set(req.name),
        person: Set(person),
        superior: Set(req.superior),
        create_time: Set(None),
        update_time: Set(None),
        deleted_at: Set(None),
    };
    let model = active.insert(&db).await?;

    let mut entries = vec![
        ("id".to_string(), Value::String(model.id.clone())),
        ("name".to_string(), Value::String(model.name.clone())),
        ("person".to_string(), Value::String(model.person.clone())),
    ];
    if let Some(superior) = model.superior.clone() {
        entries.push(("superior".to_string(), Value::String(superior)));
    }
    let result = Value::Object(serde_json::Map::from_iter(entries));

    Ok(Json(ActionResult::success(result)))
}

// ── Folder delete handler ───────────────────────────────────────────────────

#[allow(non_snake_case)]
pub async fn folder_delete(
    Extension(db): Extension<DatabaseConnection>,
    session: Option<Extension<shared::session::Session>>,
    Path(id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let session = require_session(session)?;
    let actor = session.person_unique;
    let sql = format!(
        "UPDATE file_folder SET deleted_at = (NOW() AT TIME ZONE 'UTC') \
         WHERE ({FOLDER_OWNER_PREDICATE}) RETURNING id"
    );
    let stmt =
        Statement::from_sql_and_values(DbBackend::Postgres, sql, [id.clone().into(), actor.into()]);
    let res = db
        .execute(stmt)
        .await
        .map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;
    if res.rows_affected() == 0 {
        return Err(ownership_error(&db, &id).await);
    }

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([("deleted".to_string(), Value::Bool(true))]),
    ))))
}

// ── File create handler ─────────────────────────────────────────────────────

#[allow(non_snake_case)]
pub async fn file_create(
    Extension(db): Extension<DatabaseConnection>,
    session: Option<Extension<shared::session::Session>>,
    Json(req): Json<FileCreateRequest>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    require_session(session)?;
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
    let model = active.insert(&db).await?;

    let result = Value::Object(serde_json::Map::from_iter([
        ("id".to_string(), Value::String(model.id.clone())),
        ("name".to_string(), Value::String(model.name.clone())),
        ("person".to_string(), Value::String(model.person.clone())),
        (
            "referenceType".to_string(),
            Value::String(model.reference_type.clone()),
        ),
        (
            "extension".to_string(),
            serde_json::Value::String(model.extension.clone().unwrap_or_default()),
        ),
        (
            "length".to_string(),
            Value::Number(serde_json::Number::from(model.length)),
        ),
    ]));

    Ok(Json(ActionResult::success(result)))
}

// ── Router ───────────────────────────────────────────────────────────────────

pub fn file_core_entity_router(db: DatabaseConnection) -> Router {
    Router::new()
        // folder
        .route(
            "/api/file/core/entity/folder/list/top",
            get(folder_list_top),
        )
        .route(
            "/api/file/core/entity/folder/list/{id}",
            get(folder_list_with_folder),
        )
        .route("/api/file/core/entity/folder", post(folder_create))
        .route("/api/file/core/entity/folder/{id}", delete(folder_delete))
        // file
        .route("/api/file/core/entity/file/list", get(file_list))
        .route("/api/file/core/entity/file", post(file_create))
        // complex
        .route("/api/file/core/entity/complex/top", get(complex_top))
        // B3：handler 需要 Extension<DatabaseConnection>，此前 router 忽略 pool 参数
        // 且不挂该 extension，任何请求都在提取期 500（门禁永远执行不到）。
        .layer(Extension(db))
}

#[cfg(test)]
mod security_gate_tests_b78;
#[cfg(test)]
mod security_gate_tests_idor_b5;
#[cfg(test)]
#[allow(clippy::module_inception)]
mod tests;
#[cfg(test)]
mod tests_generated;

pub fn router(db: DatabaseConnection) -> axum::Router {
    crate::file_core_entity_router(db)
}
