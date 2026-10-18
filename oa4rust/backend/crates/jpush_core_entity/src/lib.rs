// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

use axum::{
    extract::Extension, extract::Path, routing::delete, routing::get, routing::post, Json, Router,
};
use deadpool_postgres::Pool;
use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, QueryOrder, QuerySelect};
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
        .map_err(|_| AppError::Internal)?;

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
    Path(id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let model = jpush_device::Entity::find_by_id(&id)
        .one(&db.0)
        .await
        .map_err(|_| AppError::Internal)?;

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
        .map_err(|_| AppError::Internal)?;

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
        .map_err(|_| AppError::Internal)?;

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
        .map_err(|_| AppError::Internal)?;

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
pub fn jpush_core_entity_router(_pool: Pool) -> Router {
    Router::new()
        .route("/api/jpush/core/entity/device/list", get(device_list))
        .route("/api/jpush/core/entity/device/{id}", get(device_get))
        .route("/api/jpush/core/entity/device/{id}", delete(device_delete))
        .route("/api/jpush/core/entity/device/create", post(device_create))
        .route("/api/jpush/core/entity/template/list", get(template_list))
        .route("/api/jpush/core/entity/template/{id}", get(template_get))
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
        .map_err(|_| AppError::Internal)?;
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([(
            "deleted".to_string(),
            Value::Bool(result.rows_affected > 0),
        )]),
    ))))
}

#[cfg(test)]
#[allow(clippy::module_inception)]
mod tests;
#[cfg(test)]
mod tests_generated;
#[cfg(test)]
mod security_gate_tests_b83;

pub fn router(pool: deadpool_postgres::Pool) -> axum::Router {
    crate::jpush_core_entity_router(pool)
}
