// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

use axum::{extract::Extension, extract::Path, Json, Router};
use deadpool_postgres::Pool;
use serde_json::Value;
use shared::{error::AppError, response::ActionResult};

pub mod routes;

/// 写面会话存在档：无会话先 401，在任何 `pool.get()` 之前（离线可证）。
fn require_session(
    session: Option<Extension<shared::session::Session>>,
) -> Result<shared::session::Session, AppError> {
    session.map(|Extension(s)| s).ok_or(AppError::Unauthorized)
}

/// 读面归属谓词（批次 7）：设备归属列 `user_id` 命中本人，或调用方是管理员；
/// 归属列为空时只有管理员可读（fail-closed）。`$2` = 会话 person_unique。
const DEVICE_READ_OWNER_PREDICATE: &str = "(NULLIF(BTRIM(user_id), '') = $2 \
      OR EXISTS(SELECT 1 FROM auth_person p \
                  JOIN auth_person_role pr ON pr.person_id = p.id \
                  JOIN auth_role r ON r.id = pr.role_id \
                 WHERE (p.unique_id = $2 OR p.id = $2) \
                   AND r.name = 'admin' AND r.deleted_at IS NULL AND r.disable = false))";

pub fn jpush_router(pool: Pool) -> Router {
    routes::jpush_router(pool)
}

#[allow(non_snake_case)]
pub async fn hello() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::String(
        "hello".to_string(),
    ))))
}

#[axum::debug_handler]
#[allow(non_snake_case)]
pub async fn device_list(pool: Extension<Pool>) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = shared::db::acquire(&pool).await?;
    let rows = client
        .query(
            "SELECT id, user_id, platform, token FROM x_jpush_device ORDER BY create_time DESC LIMIT 20",
            &[],
        )
        .await
        .map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("userId".to_string(), Value::String(row.get("user_id"))),
                ("platform".to_string(), Value::String(row.get("platform"))),
                ("token".to_string(), Value::String(row.get("token"))),
            ]))
        })
        .collect();

    let count = data.len() as i64;
    Ok(Json(ActionResult::legacy_success(
        Value::Array(data),
        count,
        0,
    )))
}

#[axum::debug_handler]
#[allow(non_snake_case)]
pub async fn device_get(
    pool: Extension<Pool>,
    session: Option<Extension<shared::session::Session>>,
    Path(id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    // 批次 7：设备详情按 x_jpush_device.user_id 校归属（本人或管理员），
    // 无会话 401 先于任何读库；归属为空时仅管理员（fail-closed）。
    let sess = require_session(session)?;
    let client = shared::db::acquire(&pool).await?;
    let sql = format!(
        "SELECT id, user_id, platform, token FROM x_jpush_device \
         WHERE id = $1 AND {DEVICE_READ_OWNER_PREDICATE}"
    );
    let row = client
        .query_opt(&sql, &[&id, &sess.person_unique])
        .await
        .map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;

    match row {
        Some(row) => Ok(Json(ActionResult::success(Value::Object(
            serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("userId".to_string(), Value::String(row.get("user_id"))),
                ("platform".to_string(), Value::String(row.get("platform"))),
                ("token".to_string(), Value::String(row.get("token"))),
            ]),
        )))),
        None => {
            // 存在性读不参与授权：行存在但无权 → 403，行不存在保留 404。
            let exists: bool = client
                .query_one(
                    "SELECT EXISTS(SELECT 1 FROM x_jpush_device WHERE id = $1) AS row_exists",
                    &[&id],
                )
                .await
                .map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?
                .get("row_exists");
            if exists {
                Err(AppError::Forbidden)
            } else {
                Err(AppError::NotFound)
            }
        }
    }
}

#[axum::debug_handler]
#[allow(non_snake_case)]
pub async fn device_create(
    pool: Extension<Pool>,
    session: Option<Extension<shared::session::Session>>,
    Json(req): Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let sess = require_session(session)?;
    let client = shared::db::acquire(&pool).await?;

    // 推送设备归属以会话登录人为事实源：客户端 userId 可伪造，替他人注册推送设备
    // 即推送劫持面（安全审计 2026-10 二轮）。
    let user_id = sess.person_unique.clone();
    let platform = req
        .get("platform")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string();
    let token = req
        .get("token")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string();

    let id = uuid::Uuid::new_v4().to_string();

    client
        .execute(
            "INSERT INTO x_jpush_device (id, user_id, platform, token, create_time) VALUES ($1, $2, $3, $4, NOW())",
            &[&id, &user_id, &platform, &token],
        )
        .await
        .map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
            ("userId".to_string(), Value::String(user_id)),
            ("platform".to_string(), Value::String(platform)),
            ("token".to_string(), Value::String(token)),
        ]),
    ))))
}

#[axum::debug_handler]
#[allow(non_snake_case)]
pub async fn template_list(pool: Extension<Pool>) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = shared::db::acquire(&pool).await?;
    let rows = client
        .query(
            "SELECT id, name, title, content FROM x_jpush_template ORDER BY name LIMIT 20",
            &[],
        )
        .await
        .map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("name".to_string(), Value::String(row.get("name"))),
                ("title".to_string(), Value::String(row.get("title"))),
                ("content".to_string(), Value::String(row.get("content"))),
            ]))
        })
        .collect();

    let count = data.len() as i64;
    Ok(Json(ActionResult::legacy_success(
        Value::Array(data),
        count,
        0,
    )))
}

#[axum::debug_handler]
#[allow(non_snake_case)]
pub async fn template_get(
    pool: Extension<Pool>,
    Path(id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = shared::db::acquire(&pool).await?;
    let row = client
        .query_opt(
            "SELECT id, name, title, content FROM x_jpush_template WHERE id = $1",
            &[&id],
        )
        .await
        .map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;

    match row {
        Some(row) => Ok(Json(ActionResult::success(Value::Object(
            serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("name".to_string(), Value::String(row.get("name"))),
                ("title".to_string(), Value::String(row.get("title"))),
                ("content".to_string(), Value::String(row.get("content"))),
            ]),
        )))),
        None => Err(AppError::NotFound),
    }
}

pub fn router(pool: deadpool_postgres::Pool) -> axum::Router {
    crate::jpush_router(pool)
}

#[cfg(test)]
mod security_gate_tests;
#[cfg(test)]
mod security_gate_tests_idor_b7;
#[cfg(test)]
#[allow(clippy::module_inception)]
mod tests;
#[cfg(test)]
mod tests_generated;
