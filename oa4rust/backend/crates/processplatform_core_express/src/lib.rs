// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

use axum::{
    extract::{Extension, Path},
    routing::get,
    Json, Router,
};
use deadpool_postgres::Pool;
use serde_json::Value;
use shared::{error::AppError, response::ActionResult};

pub mod routes;

#[allow(non_snake_case)]
pub async fn work_terminate(
    pool: Extension<Pool>,
    session: Option<Extension<shared::session::Session>>,
    Path(work_id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    // 四轮 R4-6：此前该写既不取会话也不校归属，任一登录用户（甚至中间件缺位时）
    // 按 ID 即可终止任意 work。终止他人 work 与 meeting/work 其余状态写同族，走属主门。
    let Some(Extension(session)) = session else {
        return Err(AppError::Unauthorized);
    };
    let client = shared::db::acquire(&pool).await?;
    let creator: Option<String> = client
        .query_opt("SELECT creator FROM x_work WHERE id = $1", &[&work_id])
        .await
        .map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?
        .and_then(|row| row.get::<_, Option<String>>("creator"));
    // 归属门：发起人非空 → 本人或 admin（`require_owner`，admin 旁路）。
    // 发起人为空 / 行不存在 → **仅 admin**：空归属不是「无需校验」，否则
    // creator=NULL 的历史行会对任何登录用户开放终止（审计第六轮实证）。
    let creator = creator
        .map(|c| c.trim().to_string())
        .filter(|c| !c.is_empty());
    match creator {
        Some(creator) => {
            shared::middleware::require_owner(&pool.0, &session, &creator).await?;
        }
        None => {
            if !shared::middleware::is_admin(&pool.0, &session.person_unique).await {
                return Err(AppError::Forbidden);
            }
        }
    }
    client
        .execute(
            "UPDATE x_work SET work_status = $1 WHERE id = $2",
            &[&"terminated", &work_id],
        )
        .await
        .map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(work_id)),
            (
                "workStatus".to_string(),
                Value::String("terminated".to_string()),
            ),
            ("result".to_string(), Value::String("ok".to_string())),
        ]),
    ))))
}

#[allow(non_snake_case)]
pub async fn work_retract(
    pool: Extension<Pool>,
    Path(work_id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = shared::db::acquire(&pool).await?;
    let rows = client
        .query(
            "SELECT id, work_status, activity FROM x_work WHERE id = $1",
            &[&work_id],
        )
        .await
        .map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;

    if rows.is_empty() {
        return Ok(Json(ActionResult::error("work not found")));
    }

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(work_id)),
            (
                "workStatus".to_string(),
                Value::String("retracted".to_string()),
            ),
            (
                "previousStatus".to_string(),
                Value::String(rows[0].get("work_status")),
            ),
        ]),
    ))))
}

#[allow(non_snake_case)]
pub async fn work_processing(
    pool: Extension<Pool>,
    Path(work_id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = shared::db::acquire(&pool).await?;
    let row = client
        .query_opt(
            "SELECT id, title, work_status, activity FROM x_work WHERE id = $1",
            &[&work_id],
        )
        .await
        .map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?
        .ok_or(AppError::NotFound)?;

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(row.get("id"))),
            ("title".to_string(), Value::String(row.get("title"))),
            (
                "workStatus".to_string(),
                Value::String(
                    row.get::<_, Option<String>>("work_status")
                        .unwrap_or_default(),
                ),
            ),
            (
                "activity".to_string(),
                Value::String(row.get::<_, Option<String>>("activity").unwrap_or_default()),
            ),
        ]),
    ))))
}

#[allow(non_snake_case)]
pub async fn task_processing(
    pool: Extension<Pool>,
    Path(task_id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = shared::db::acquire(&pool).await?;
    let row = client
        .query_opt(
            "SELECT id, title, person, activity FROM x_task WHERE id = $1",
            &[&task_id],
        )
        .await
        .map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?
        .ok_or(AppError::NotFound)?;

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(row.get("id"))),
            (
                "title".to_string(),
                Value::String(row.get::<_, Option<String>>("title").unwrap_or_default()),
            ),
            (
                "person".to_string(),
                Value::String(row.get::<_, Option<String>>("person").unwrap_or_default()),
            ),
            (
                "activity".to_string(),
                Value::String(row.get::<_, Option<String>>("activity").unwrap_or_default()),
            ),
        ]),
    ))))
}

#[allow(non_snake_case)]
pub async fn work_count_with_person(
    pool: Extension<Pool>,
    Path(person_id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = shared::db::acquire(&pool).await?;
    let row = client
        .query_one(
            "SELECT COUNT(*) as count FROM x_task WHERE person = $1 AND work IS NOT NULL",
            &[&person_id],
        )
        .await
        .map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;

    let count: i64 = row.get("count");

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("personId".to_string(), Value::String(person_id)),
            (
                "count".to_string(),
                Value::Number(serde_json::Number::from(count)),
            ),
        ]),
    ))))
}

#[allow(non_snake_case)]
pub async fn task_count_with_person(
    pool: Extension<Pool>,
    Path(person_id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = shared::db::acquire(&pool).await?;
    let row = client
        .query_one(
            "SELECT COUNT(*) as count FROM x_task WHERE person = $1",
            &[&person_id],
        )
        .await
        .map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;

    let count: i64 = row.get("count");

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("personId".to_string(), Value::String(person_id)),
            (
                "count".to_string(),
                Value::Number(serde_json::Number::from(count)),
            ),
        ]),
    ))))
}

pub fn processplatform_core_express_router(pool: Pool) -> Router {
    Router::new()
        .route(
            "/api/processplatform/work/terminate/{id}",
            get(work_terminate),
        )
        .route("/api/processplatform/work/retract/{id}", get(work_retract))
        .route(
            "/api/processplatform/work/processing/{id}",
            get(work_processing),
        )
        .route(
            "/api/processplatform/task/processing/{id}",
            get(task_processing),
        )
        .route(
            "/api/processplatform/work/count/with/person/{id}",
            get(work_count_with_person),
        )
        .route(
            "/api/processplatform/task/count/with/person/{id}",
            get(task_count_with_person),
        )
        .layer(Extension(pool))
}

pub fn router(pool: deadpool_postgres::Pool) -> axum::Router {
    crate::processplatform_core_express_router(pool)
}

#[cfg(test)]
mod security_gate_tests;
#[cfg(test)]
mod tests;
#[cfg(test)]
mod tests_generated;
#[cfg(test)]
mod tests_manual;
