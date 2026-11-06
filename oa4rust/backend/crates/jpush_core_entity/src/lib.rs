// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

use axum::{
    extract::Extension, extract::Path, routing::delete, routing::get, routing::post, Json, Router,
};
use sea_orm::{
    ActiveModelTrait, ConnectionTrait, DatabaseConnection, DbBackend, EntityTrait, QueryOrder,
    QuerySelect, Statement, Value as SeaValue,
};
use serde_json::Value;
use shared::{error::AppError, response::ActionResult};

pub mod entities;
pub mod routes;

use entities::{jpush_device, jpush_template};

/// 安全审计（round4-b83）：jpush 实体 2 把 sea_orm CRUD 真写收"会话存在档"门禁。
/// 无会话（Option<Extension<Session>> = None）→ 401，门置于首个 DB 访问
/// （`ActiveModel::insert` / `Entity::find_by_id().one()`）之前，杜绝匿名写
/// `jpush_device` 表。这些是 sea_orm ORM 写，审计尺子"含写"指标不可见
/// （ORM-blindness，§五 尺子缺陷 28），R7 降但含写不变属预期。
fn require_session(
    session: Option<Extension<shared::session::Session>>,
) -> Result<shared::session::Session, AppError> {
    session.map(|Extension(s)| s).ok_or(AppError::Unauthorized)
}

/// 读面归属谓词（批次 7）：`user_id` 命中本人，或管理员；归属为空时仅管理员。
/// `$2` = 会话 person_unique（可重复引用）。
const DEVICE_READ_OWNER_PREDICATE: &str = "(NULLIF(BTRIM(user_id), '') = $2 \
      OR EXISTS(SELECT 1 FROM auth_person p \
                  JOIN auth_person_role pr ON pr.person_id = p.id \
                  JOIN auth_role r ON r.id = pr.role_id \
                 WHERE (p.unique_id = $2 OR p.id = $2) \
                   AND r.name = 'admin' AND r.deleted_at IS NULL AND r.disable = false))";

/// 授权读：谓词进 `SELECT EXISTS(...)`，行不存在或无权都返回 false。
async fn device_read_allowed(
    db: &DatabaseConnection,
    id: &str,
    me: &str,
) -> Result<bool, AppError> {
    let sql = format!(
        "SELECT EXISTS(SELECT 1 FROM x_jpush_device WHERE id = $1 AND {DEVICE_READ_OWNER_PREDICATE}) AS allowed"
    );
    let values: Vec<SeaValue> = vec![id.into(), me.into()];
    let row = db
        .query_one(Statement::from_sql_and_values(
            DbBackend::Postgres,
            sql,
            values,
        ))
        .await
        .map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;
    Ok(row
        .and_then(|r| r.try_get::<bool>("", "allowed").ok())
        .unwrap_or(false))
}

// 推送设备实体
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct PushDevice {
    pub id: String,
    pub user_id: String,
    pub platform: String,
    pub token: String,
}

// 推送模板实体
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct PushTemplate {
    pub id: String,
    pub name: String,
    pub title: String,
    pub content: String,
}

/// 获取推送设备列表
/// 从数据库查询 x_jpush_device 表
pub async fn device_list(
    db: Extension<DatabaseConnection>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let models = jpush_device::Entity::find()
        .order_by_desc(jpush_device::Column::CreateTime)
        .limit(20)
        .all(&db.0)
        .await
        .map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;

    let data: Vec<Value> = models
        .iter()
        .map(|m| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(m.id.clone())),
                ("userId".to_string(), Value::String(m.user_id.clone())),
                ("platform".to_string(), Value::String(m.platform.clone())),
                ("token".to_string(), Value::String(m.token.clone())),
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

/// 获取推送设备详情
pub async fn device_get(
    db: Extension<DatabaseConnection>,
    session: Option<Extension<shared::session::Session>>,
    Path(id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    // 批次 7：设备详情按 `user_id` 校归属（本人或管理员），无会话 401 先于读库。
    let sess = require_session(session)?;
    let allowed = device_read_allowed(&db.0, &id, &sess.person_unique).await?;
    if !allowed {
        // 存在性读不参与授权：行存在但无权 → 403，行不存在保留 404。
        let exists = jpush_device::Entity::find_by_id(&id)
            .one(&db.0)
            .await
            .map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?
            .is_some();
        return if exists {
            Err(AppError::Forbidden)
        } else {
            Err(AppError::NotFound)
        };
    }
    let model = jpush_device::Entity::find_by_id(&id)
        .one(&db.0)
        .await
        .map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;

    match model {
        Some(m) => Ok(Json(ActionResult::success(Value::Object(
            serde_json::Map::from_iter([
                ("id".to_string(), Value::String(m.id.clone())),
                ("userId".to_string(), Value::String(m.user_id.clone())),
                ("platform".to_string(), Value::String(m.platform.clone())),
                ("token".to_string(), Value::String(m.token.clone())),
            ]),
        )))),
        None => Err(AppError::NotFound),
    }
}

/// 创建推送设备
pub async fn device_create(
    db: Extension<DatabaseConnection>,
    session: Option<Extension<shared::session::Session>>,
    Json(req): Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    require_session(session)?;
    let id = uuid::Uuid::new_v4().to_string();
    let user_id = req
        .get("userId")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string();
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

    let active_model = jpush_device::ActiveModel {
        id: sea_orm::ActiveValue::Set(id.clone()),
        user_id: sea_orm::ActiveValue::Set(user_id.clone()),
        platform: sea_orm::ActiveValue::Set(platform.clone()),
        token: sea_orm::ActiveValue::Set(token.clone()),
        create_time: sea_orm::ActiveValue::NotSet,
    };

    active_model
        .insert(&db.0)
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

/// 获取推送模板列表
/// 从数据库查询 x_jpush_template 表
pub async fn template_list(
    db: Extension<DatabaseConnection>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let models = jpush_template::Entity::find()
        .order_by_asc(jpush_template::Column::Name)
        .limit(20)
        .all(&db.0)
        .await
        .map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;

    let data: Vec<Value> = models
        .iter()
        .map(|m| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(m.id.clone())),
                ("name".to_string(), Value::String(m.name.clone())),
                ("title".to_string(), Value::String(m.title.clone())),
                ("content".to_string(), Value::String(m.content.clone())),
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

/// 获取推送模板详情
pub async fn template_get(
    db: Extension<DatabaseConnection>,
    Path(id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let model = jpush_template::Entity::find_by_id(&id)
        .one(&db.0)
        .await
        .map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;

    match model {
        Some(m) => Ok(Json(ActionResult::success(Value::Object(
            serde_json::Map::from_iter([
                ("id".to_string(), Value::String(m.id.clone())),
                ("name".to_string(), Value::String(m.name.clone())),
                ("title".to_string(), Value::String(m.title.clone())),
                ("content".to_string(), Value::String(m.content.clone())),
            ]),
        )))),
        None => Err(AppError::NotFound),
    }
}

/// 创建推送核心实体路由
/// 注册以下路由：
/// - /api/jpush/core/entity/device/list - 设备列表
/// - /api/jpush/core/entity/device/{id} - 设备详情
/// - /api/jpush/core/entity/device/create - 创建设备
/// - /api/jpush/core/entity/template/list - 模板列表
/// - /api/jpush/core/entity/template/{id} - 模板详情
pub fn jpush_core_entity_router(db: DatabaseConnection) -> Router {
    Router::new()
        .route("/api/jpush/core/entity/device/list", get(device_list))
        .route("/api/jpush/core/entity/device/{id}", get(device_get))
        .route("/api/jpush/core/entity/device/{id}", delete(device_delete))
        .route("/api/jpush/core/entity/device/create", post(device_create))
        .route("/api/jpush/core/entity/template/list", get(template_list))
        .route("/api/jpush/core/entity/template/{id}", get(template_get))
        .layer(Extension(db))
}

/// DELETE /api/jpush/core/entity/device/{id} — 删除推送设备
#[allow(non_snake_case)]
pub async fn device_delete(
    db: Extension<DatabaseConnection>,
    session: Option<Extension<shared::session::Session>>,
    Path(id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    require_session(session)?;
    let result = jpush_device::Entity::delete_by_id(&id)
        .exec(&db.0)
        .await
        .map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([(
            "deleted".to_string(),
            Value::Bool(result.rows_affected > 0),
        )]),
    ))))
}

#[cfg(test)]
mod security_gate_tests_b83;
#[cfg(test)]
mod security_gate_tests_idor_b7;
#[cfg(test)]
#[allow(clippy::module_inception)]
mod tests;
#[cfg(test)]
mod tests_generated;

pub fn router(db: DatabaseConnection) -> axum::Router {
    crate::jpush_core_entity_router(db)
}
