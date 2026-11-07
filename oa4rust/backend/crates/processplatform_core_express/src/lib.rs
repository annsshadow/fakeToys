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

/// 读归属档（与 `work_terminate` 同一把尺）：任一非空归属列等于本人 → 放行；
/// 归属列全空 / 行不存在 → 只允许 admin（fail-closed）。
async fn require_read_owner(
    pool: &Pool,
    session: &shared::session::Session,
    owners: &[Option<String>],
) -> Result<(), AppError> {
    let me = session.person_unique.trim();
    let matched = owners.iter().flatten().any(|owner| {
        let owner = owner.trim();
        !owner.is_empty() && owner.eq_ignore_ascii_case(me)
    });
    if matched {
        return Ok(());
    }
    if shared::middleware::is_admin(pool, &session.person_unique).await {
        Ok(())
    } else {
        Err(AppError::Forbidden)
    }
}

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
    session: Option<Extension<shared::session::Session>>,
    Path(work_id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let Some(Extension(session)) = session else {
        return Err(AppError::Unauthorized);
    };
    let client = shared::db::acquire(&pool).await?;
    let rows = client
        .query(
            "SELECT id, work_status, activity, creator FROM x_work WHERE id = $1",
            &[&work_id],
        )
        .await
        .map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;

    if rows.is_empty() {
        return Ok(Json(ActionResult::error("work not found")));
    }
    require_read_owner(&pool.0, &session, &[rows[0].get("creator")]).await?;

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
    session: Option<Extension<shared::session::Session>>,
    Path(work_id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let Some(Extension(session)) = session else {
        return Err(AppError::Unauthorized);
    };
    let client = shared::db::acquire(&pool).await?;
    let row = client
        .query_opt(
            "SELECT id, title, work_status, activity, creator FROM x_work WHERE id = $1",
            &[&work_id],
        )
        .await
        .map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?
        .ok_or(AppError::NotFound)?;
    require_read_owner(&pool.0, &session, &[row.get("creator")]).await?;

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
    session: Option<Extension<shared::session::Session>>,
    Path(task_id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let Some(Extension(session)) = session else {
        return Err(AppError::Unauthorized);
    };
    let client = shared::db::acquire(&pool).await?;
    let row = client
        .query_opt(
            "SELECT t.id, t.title, t.person, t.activity, w.creator AS work_creator \
             FROM x_task t LEFT JOIN x_work w ON w.id = t.work WHERE t.id = $1",
            &[&task_id],
        )
        .await
        .map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?
        .ok_or(AppError::NotFound)?;
    // 与 B-b4 的 task 口径一致：办件人（person）或发起人（work.creator）任一命中即放行，
    // 双方之外的陌生会话按 ID 不再能读他人待办。
    require_read_owner(
        &pool.0,
        &session,
        &[row.get("person"), row.get("work_creator")],
    )
    .await?;

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
