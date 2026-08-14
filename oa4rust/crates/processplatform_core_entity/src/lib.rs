use axum::{
    extract::{Extension, Path},
    routing::get,
    Json, Router,
};
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, QueryOrder, QuerySelect};
use serde_json::Value;
use shared::{error::AppError, response::ActionResult};

pub mod entities;
pub mod routes;

use entities::{pp_work, pp_task, pp_ticket, pp_work_completed};

pub async fn work_list(
    db: Extension<DatabaseConnection>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let models = pp_work::Entity::find()
        .filter(pp_work::Column::DeletedAt.is_null())
        .order_by_desc(pp_work::Column::CreateTime)
        .limit(20)
        .all(&db.0)
        .await
        .map_err(|_| AppError::Internal)?;

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
                        .unwrap_or(Value::Null)
                }),
                (
                    "createTime".to_string(),
                    Value::String(
                        m.create_time
                            .clone()
                            .map(|dt| dt.to_string())
                            .unwrap_or_default(),
                    ),
                ),
                (
                    "updateTime".to_string(),
                    Value::String(
                        m.update_time
                            .clone()
                            .map(|dt| dt.to_string())
                            .unwrap_or_default(),
                    ),
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

pub async fn work_get(
    db: Extension<DatabaseConnection>,
    Path(id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let model = pp_work::Entity::find()
        .filter(pp_work::Column::Id.eq(&id))
        .filter(pp_work::Column::DeletedAt.is_null())
        .one(&db.0)
        .await
        .map_err(|_| AppError::Internal)?
        .ok_or_else(|| AppError::NotFound)?;

    let result = Value::Object(serde_json::Map::from_iter([
        ("id".to_string(), Value::String(model.id.clone())),
        ("title".to_string(), Value::String(model.title.clone())),
        ("creatorId".to_string(), Value::String(model.creator_id.clone())),
        ("status".to_string(), Value::String(model.status.clone())),
        ("formData".to_string(), {
            let fd: Option<String> = model.form_data.clone();
            fd.and_then(|s| serde_json::from_str(&s).ok())
                .unwrap_or(Value::Null)
        }),
        (
            "createTime".to_string(),
            Value::String(
                model
                    .create_time
                    .clone()
                    .map(|dt| dt.to_string())
                    .unwrap_or_default(),
            ),
        ),
        (
            "updateTime".to_string(),
            Value::String(
                model
                    .update_time
                    .clone()
                    .map(|dt| dt.to_string())
                    .unwrap_or_default(),
            ),
        ),
    ]));

    Ok(Json(ActionResult::success(result)))
}

pub async fn task_list(
    db: Extension<DatabaseConnection>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let models = pp_task::Entity::find()
        .filter(pp_task::Column::DeletedAt.is_null())
        .order_by_desc(pp_task::Column::CreateTime)
        .limit(20)
        .all(&db.0)
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = models
        .iter()
        .map(|m| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(m.id.clone())),
                ("workId".to_string(), Value::String(m.work_id.clone())),
                ("title".to_string(), Value::String(m.title.clone())),
                ("assigneeId".to_string(), Value::String(m.assignee_id.clone())),
                ("status".to_string(), Value::String(m.status.clone())),
                (
                    "createTime".to_string(),
                    Value::String(
                        m.create_time
                            .clone()
                            .map(|dt| dt.to_string())
                            .unwrap_or_default(),
                    ),
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

pub async fn task_get(
    db: Extension<DatabaseConnection>,
    Path(id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let model = pp_task::Entity::find()
        .filter(pp_task::Column::Id.eq(&id))
        .filter(pp_task::Column::DeletedAt.is_null())
        .one(&db.0)
        .await
        .map_err(|_| AppError::Internal)?
        .ok_or_else(|| AppError::NotFound)?;

    let result = Value::Object(serde_json::Map::from_iter([
        ("id".to_string(), Value::String(model.id.clone())),
        ("workId".to_string(), Value::String(model.work_id.clone())),
        ("title".to_string(), Value::String(model.title.clone())),
        ("assigneeId".to_string(), Value::String(model.assignee_id.clone())),
        ("status".to_string(), Value::String(model.status.clone())),
        (
            "createTime".to_string(),
            Value::String(
                model
                    .create_time
                    .clone()
                    .map(|dt| dt.to_string())
                    .unwrap_or_default(),
            ),
        ),
    ]));

    Ok(Json(ActionResult::success(result)))
}

pub async fn ticket_list(
    db: Extension<DatabaseConnection>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let models = pp_ticket::Entity::find()
        .filter(pp_ticket::Column::DeletedAt.is_null())
        .order_by_desc(pp_ticket::Column::CreateTime)
        .limit(20)
        .all(&db.0)
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = models
        .iter()
        .map(|m| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(m.id.clone())),
                ("workId".to_string(), Value::String(m.work_id.clone())),
                ("title".to_string(), Value::String(m.title.clone())),
                (
                    "description".to_string(),
                    m.description
                        .clone()
                        .map(Value::String)
                        .unwrap_or(Value::Null),
                ),
                ("status".to_string(), Value::String(m.status.clone())),
                (
                    "createTime".to_string(),
                    Value::String(
                        m.create_time
                            .clone()
                            .map(|dt| dt.to_string())
                            .unwrap_or_default(),
                    ),
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

pub async fn workcompleted_list(
    db: Extension<DatabaseConnection>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let models = pp_work_completed::Entity::find()
        .filter(pp_work_completed::Column::DeletedAt.is_null())
        .order_by_desc(pp_work_completed::Column::CompleteTime)
        .limit(20)
        .all(&db.0)
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = models
        .iter()
        .map(|m| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(m.id.clone())),
                ("workId".to_string(), Value::String(m.work_id.clone())),
                ("result".to_string(), Value::String(m.result.clone())),
                (
                    "completeTime".to_string(),
                    Value::String(
                        m.complete_time
                            .clone()
                            .map(|dt| dt.to_string())
                            .unwrap_or_default(),
                    ),
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

pub fn processplatform_core_entity_router(_pool: deadpool_postgres::Pool) -> Router {
    Router::new()
        .route("/jaxrs/process/work/list", get(work_list))
        .route("/jaxrs/process/work/{id}", get(work_get))
        .route("/jaxrs/process/task/list", get(task_list))
        .route("/jaxrs/process/task/{id}", get(task_get))
        .route("/jaxrs/process/ticket/list", get(ticket_list))
        .route(
            "/jaxrs/process/workcompleted/list",
            get(workcompleted_list),
        )
}

#[cfg(test)]
mod tests;
#[cfg(test)]
mod tests_generated;


pub fn router(pool: deadpool_postgres::Pool) -> axum::Router {
    crate::processplatform_core_entity_router(pool)
}
