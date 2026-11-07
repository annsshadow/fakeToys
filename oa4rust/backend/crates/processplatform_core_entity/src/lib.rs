// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

use axum::{
    extract::{Extension, Path},
    routing::get,
    Json, Router,
};
use sea_orm::{
    ColumnTrait, ConnectionTrait, DatabaseConnection, DbBackend, EntityTrait, QueryFilter,
    QueryOrder, QuerySelect, Statement,
};
use serde_json::Value;
use shared::{error::AppError, response::ActionResult};

pub mod entities;
pub mod routes;

use entities::{pp_task, pp_ticket, pp_work, pp_work_completed};

/// 本 crate 的路由只注入 `DatabaseConnection`（没有 deadpool 池），管理员判定
/// 直接用 SeaORM 连接查同一套 `auth_person`/`auth_role` 口径。
async fn db_is_admin(db: &DatabaseConnection, person: &str) -> Result<bool, AppError> {
    let stmt = Statement::from_sql_and_values(
        DbBackend::Postgres,
        "SELECT EXISTS(SELECT 1 FROM auth_person p \
           LEFT JOIN auth_person_role pr ON pr.person_id = p.id \
           LEFT JOIN auth_role r ON r.id = pr.role_id \
          WHERE (p.unique_id = $1 OR p.id = $1) \
            AND r.name = 'admin' AND r.deleted_at IS NULL AND r.disable = false) AS is_admin",
        [person.into()],
    );
    let row = db
        .query_one(stmt)
        .await
        .map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;
    Ok(row
        .map(|row| row.try_get::<bool>("", "is_admin").unwrap_or(false))
        .unwrap_or(false))
}

#[allow(non_snake_case)]
pub async fn work_list(
    db: Extension<DatabaseConnection>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let models = pp_work::Entity::find()
        .filter(pp_work::Column::DeletedAt.is_null())
        .order_by_desc(pp_work::Column::CreateTime)
        .limit(20)
        .all(&db.0)
        .await
        .map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;

    let data: Vec<Value> = models
        .iter()
        .map(|m| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(m.id.clone())),
                ("title".to_string(), Value::String(m.title.clone())),
                ("creatorId".to_string(), Value::String(m.creator_id.clone())),
                ("status".to_string(), Value::String(m.status.clone())),
                ("formData".to_string(), {
                    let fd: Option<String> = m.form_data.clone();
                    fd.and_then(|s| serde_json::from_str(&s).ok())
                        .unwrap_or_default()
                }),
                (
                    "createTime".to_string(),
                    Value::String(m.create_time.map(|dt| dt.to_string()).unwrap_or_default()),
                ),
                (
                    "updateTime".to_string(),
                    Value::String(m.update_time.map(|dt| dt.to_string()).unwrap_or_default()),
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
pub async fn work_get(
    db: Extension<DatabaseConnection>,
    session: Option<Extension<shared::session::Session>>,
    Path(id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let Some(Extension(sess)) = session else {
        return Err(AppError::Unauthorized);
    };
    let model = pp_work::Entity::find()
        .filter(pp_work::Column::Id.eq(&id))
        .filter(pp_work::Column::DeletedAt.is_null())
        .one(&db.0)
        .await
        .map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?
        .ok_or_else(|| AppError::NotFound)?;

    // 读归属档：发起人非空 → 本人或 admin；为空 → 仅 admin（fail-closed）。
    let creator = model.creator_id.trim();
    let is_owner = !creator.is_empty() && creator.eq_ignore_ascii_case(sess.person_unique.trim());
    if !is_owner && !db_is_admin(&db.0, &sess.person_unique).await? {
        return Err(AppError::Forbidden);
    }

    let result = Value::Object(serde_json::Map::from_iter([
        ("id".to_string(), Value::String(model.id.clone())),
        ("title".to_string(), Value::String(model.title.clone())),
        (
            "creatorId".to_string(),
            Value::String(model.creator_id.clone()),
        ),
        ("status".to_string(), Value::String(model.status.clone())),
        ("formData".to_string(), {
            let fd: Option<String> = model.form_data.clone();
            fd.and_then(|s| serde_json::from_str(&s).ok())
                .unwrap_or_default()
        }),
        (
            "createTime".to_string(),
            Value::String(
                model
                    .create_time
                    .map(|dt| dt.to_string())
                    .unwrap_or_default(),
            ),
        ),
        (
            "updateTime".to_string(),
            Value::String(
                model
                    .update_time
                    .map(|dt| dt.to_string())
                    .unwrap_or_default(),
            ),
        ),
    ]));

    Ok(Json(ActionResult::success(result)))
}

#[allow(non_snake_case)]
pub async fn task_list(
    db: Extension<DatabaseConnection>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let models = pp_task::Entity::find()
        .filter(pp_task::Column::DeletedAt.is_null())
        .order_by_desc(pp_task::Column::CreateTime)
        .limit(20)
        .all(&db.0)
        .await
        .map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;

    let data: Vec<Value> = models
        .iter()
        .map(|m| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(m.id.clone())),
                ("workId".to_string(), Value::String(m.work_id.clone())),
                ("title".to_string(), Value::String(m.title.clone())),
                (
                    "assigneeId".to_string(),
                    Value::String(m.assignee_id.clone()),
                ),
                ("status".to_string(), Value::String(m.status.clone())),
                (
                    "createTime".to_string(),
                    Value::String(m.create_time.map(|dt| dt.to_string()).unwrap_or_default()),
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
pub async fn task_get(
    db: Extension<DatabaseConnection>,
    Path(id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let model = pp_task::Entity::find()
        .filter(pp_task::Column::Id.eq(&id))
        .filter(pp_task::Column::DeletedAt.is_null())
        .one(&db.0)
        .await
        .map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?
        .ok_or_else(|| AppError::NotFound)?;

    let result = Value::Object(serde_json::Map::from_iter([
        ("id".to_string(), Value::String(model.id.clone())),
        ("workId".to_string(), Value::String(model.work_id.clone())),
        ("title".to_string(), Value::String(model.title.clone())),
        (
            "assigneeId".to_string(),
            Value::String(model.assignee_id.clone()),
        ),
        ("status".to_string(), Value::String(model.status.clone())),
        (
            "createTime".to_string(),
            Value::String(
                model
                    .create_time
                    .map(|dt| dt.to_string())
                    .unwrap_or_default(),
            ),
        ),
    ]));

    Ok(Json(ActionResult::success(result)))
}

#[allow(non_snake_case)]
pub async fn ticket_list(
    db: Extension<DatabaseConnection>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let models = pp_ticket::Entity::find()
        .filter(pp_ticket::Column::DeletedAt.is_null())
        .order_by_desc(pp_ticket::Column::CreateTime)
        .limit(20)
        .all(&db.0)
        .await
        .map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;

    let data: Vec<Value> = models
        .iter()
        .map(|m| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(m.id.clone())),
                ("workId".to_string(), Value::String(m.work_id.clone())),
                ("title".to_string(), Value::String(m.title.clone())),
                (
                    "description".to_string(),
                    m.description.clone().map(Value::String).unwrap_or_default(),
                ),
                ("status".to_string(), Value::String(m.status.clone())),
                (
                    "createTime".to_string(),
                    Value::String(m.create_time.map(|dt| dt.to_string()).unwrap_or_default()),
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
pub async fn workcompleted_list(
    db: Extension<DatabaseConnection>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let models = pp_work_completed::Entity::find()
        .filter(pp_work_completed::Column::DeletedAt.is_null())
        .order_by_desc(pp_work_completed::Column::CompleteTime)
        .limit(20)
        .all(&db.0)
        .await
        .map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;

    let data: Vec<Value> = models
        .iter()
        .map(|m| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(m.id.clone())),
                ("workId".to_string(), Value::String(m.work_id.clone())),
                ("result".to_string(), Value::String(m.result.clone())),
                (
                    "completeTime".to_string(),
                    Value::String(m.complete_time.map(|dt| dt.to_string()).unwrap_or_default()),
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

pub fn processplatform_core_entity_router(db: DatabaseConnection) -> Router {
    Router::new()
        .route("/api/process/work/list", get(work_list))
        .route("/api/process/work/{id}", get(work_get))
        .route("/api/process/task/list", get(task_list))
        .route("/api/process/task/{id}", get(task_get))
        .route("/api/process/ticket/list", get(ticket_list))
        .route("/api/process/workcompleted/list", get(workcompleted_list))
        .layer(Extension(db))
}

#[cfg(test)]
#[allow(clippy::module_inception)]
mod tests;
#[cfg(test)]
mod tests_generated;

pub fn router(db: DatabaseConnection) -> axum::Router {
    crate::processplatform_core_entity_router(db)
}
