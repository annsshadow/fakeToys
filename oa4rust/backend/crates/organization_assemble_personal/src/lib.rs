// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

use axum::{
    extract::{Extension, Path},
    routing::{get, post},
    Json, Router,
};
use deadpool_postgres::Pool;
use serde_json::Value;
use shared::{error::AppError, response::ActionResult};

pub const API_BASE: &str = "/api/organization_assemble_personal";
pub mod routes;

/// 安全审计（round4-b75）：个人自定义字段更新真写收"会话存在档"门禁。
/// 无会话（Option<Extension<Session>> = None）→ 401，门置于首个 DB 访问
/// （`pool.get()`）之前，杜绝匿名改写 `x_org_custom.field_value`。
/// 注：UPDATE 仅按 id 过滤、无属主/归属条件（IDOR 收紧残留，列 §五）。
fn require_session(
    session: Option<Extension<shared::session::Session>>,
) -> Result<shared::session::Session, AppError> {
    session.map(|Extension(s)| s).ok_or(AppError::Unauthorized)
}

#[cfg(test)]
mod security_gate_tests_b75;
#[cfg(test)]
#[allow(clippy::module_inception)]
mod tests;
#[cfg(test)]
mod tests_generated;

#[cfg(test)]
mod tests_manual;
// 安全审计（读 IDOR 批次 2）：个人设置/角色的读归属「本人 OR 管理员」。
// 归属谓词直接进 SQL 的 WHERE，先授权后取数。
pub(crate) const PERSON_SETTING_OWNER_PREDICATE: &str =
    "id = $1 AND (id = $2 OR unique_id = $2 OR EXISTS(\
     SELECT 1 FROM auth_person p \
       JOIN auth_person_role pr ON pr.person_id = p.id \
       JOIN auth_role r ON r.id = pr.role_id \
      WHERE (p.unique_id = $2 OR p.id = $2) \
        AND r.name = 'admin' AND r.deleted_at IS NULL AND r.disable = false))";

pub(crate) const PERSON_ROLE_OWNER_PREDICATE: &str = "m.person_id = $1 AND ($1 = $2 OR EXISTS(\
     SELECT 1 FROM auth_person p \
       JOIN auth_person_role pr ON pr.person_id = p.id \
       JOIN auth_role r ON r.id = pr.role_id \
      WHERE (p.unique_id = $2 OR p.id = $2) \
        AND r.name = 'admin' AND r.deleted_at IS NULL AND r.disable = false))";

#[axum::debug_handler]
pub async fn user_setting(
    pool: Extension<Pool>,
    session: Option<Extension<shared::session::Session>>,
    Path(id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let actor = require_session(session)?.person_unique;
    let client = shared::db::acquire(&pool).await?;

    let row = client
        .query_opt(
            &format!(
                "SELECT mobile, email, icon, theme, lang FROM auth_person \
                  WHERE {PERSON_SETTING_OWNER_PREDICATE} LIMIT 1"
            ),
            &[&id, &actor],
        )
        .await
        .map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;

    let Some(row) = row else {
        // 授权已在主查询 WHERE 内；这里只决定错误码：行存在但无权读 → 403。
        let exists = client
            .query_one(
                "SELECT EXISTS(SELECT 1 FROM auth_person WHERE id = $1) AS row_exists",
                &[&id],
            )
            .await
            .map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?
            .get::<_, bool>("row_exists");
        return Err(if exists {
            AppError::Forbidden
        } else {
            AppError::NotFound
        });
    };

    let mut map = serde_json::Map::new();
    if let Some(v) = row.get::<_, Option<String>>("mobile") {
        map.insert("mobile".to_string(), Value::String(v));
    }
    if let Some(v) = row.get::<_, Option<String>>("email") {
        map.insert("email".to_string(), Value::String(v));
    }
    if let Some(v) = row.get::<_, Option<String>>("icon") {
        map.insert("icon".to_string(), Value::String(v));
    }
    if let Some(v) = row.get::<_, Option<String>>("theme") {
        map.insert("theme".to_string(), Value::String(v));
    }
    if let Some(v) = row.get::<_, Option<String>>("lang") {
        map.insert("lang".to_string(), Value::String(v));
    }
    let data = Value::Object(map);

    Ok(Json(ActionResult::success(data)))
}

#[axum::debug_handler]
pub async fn user_role_list(
    pool: Extension<Pool>,
    session: Option<Extension<shared::session::Session>>,
    Path(id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let actor = require_session(session)?.person_unique;
    let client = shared::db::acquire(&pool).await?;

    let rows = client
        .query(
            &format!(
                "SELECT r.id, r.name FROM x_org_group_member m \
                  JOIN x_org_role r ON m.role_id = r.id \
                 WHERE {PERSON_ROLE_OWNER_PREDICATE}"
            ),
            &[&id, &actor],
        )
        .await
        .map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;

    if rows.is_empty() {
        // 授权已在主查询 WHERE 内；成员关系存在但无权读 → 403。
        let exists = client
            .query_one(
                "SELECT EXISTS(SELECT 1 FROM x_org_group_member WHERE person_id = $1) AS row_exists",
                &[&id],
            )
            .await
            .map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?
            .get::<_, bool>("row_exists");
        if exists {
            return Err(AppError::Forbidden);
        }
    }

    let roles: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                (
                    "id".to_string(),
                    Value::String(row.get::<_, Option<String>>("id").unwrap_or_default()),
                ),
                (
                    "name".to_string(),
                    Value::String(row.get::<_, Option<String>>("name").unwrap_or_default()),
                ),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([("roles".to_string(), Value::Array(roles))]),
    ))))
}

pub fn organization_assemble_personal_router() -> Router {
    Router::new()
        .route(
            "/api/organization/assemble/personal/{id}/setting",
            get(user_setting),
        )
        .route(
            "/api/organization/assemble/personal/{id}/role/list",
            get(user_role_list),
        )
        .route(
            "/api/organization/assemble/personal/custom/{id}/mockputtopost",
            post(custom_mockputtopost),
        )
}

pub async fn custom_mockputtopost(
    pool: Extension<Pool>,
    session: Option<Extension<shared::session::Session>>,
    Path(id): Path<String>,
    Json(req): Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    require_session(session)?;
    let client = shared::db::acquire(&pool).await?;
    let value = req.get("fieldValue").and_then(|v| v.as_str()).unwrap_or("");
    let n = client
        .execute(
            "UPDATE x_org_custom SET field_value = $2 WHERE id = $1 AND deleted_at IS NULL",
            &[&id, &value],
        )
        .await
        .map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;
    Ok(Json(ActionResult::success(serde_json::json!({
        "id": id, "updated": n
    }))))
}

pub fn router(pool: Pool) -> Router {
    routes::router(pool)
}
