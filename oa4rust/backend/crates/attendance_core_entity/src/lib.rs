// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

use axum::{
    extract::{Extension, Json, Path},
    routing::{get, post},
    Json as AxumJson, Router,
};
use sea_orm::{
    ActiveModelTrait, ConnectionTrait, DatabaseConnection, DbBackend, EntityTrait, QueryOrder,
    QuerySelect, Statement,
};
use serde_json::Value;
use shared::{error::AppError, response::ActionResult};

pub mod entities;
pub mod routes;

use entities::{attendance_record, attendance_rule};

/// 安全审计（round4-b82）：attendance 实体 4 把 sea_orm CRUD 真写收"会话存在档"门禁。
/// 无会话（Option<Extension<Session>> = None）→ 401，门置于首个 DB 访问
/// （`ActiveModel::insert/update` / `Entity::find_by_id().one()`）之前，杜绝匿名写
/// attendance_record / attendance_rule。这些是 sea_orm ORM 写，审计尺子"含写"
/// 指标不可见（ORM-blindness，§五 尺子缺陷 28），R7 降但含写不变属预期。
fn require_session(
    session: Option<Extension<shared::session::Session>>,
) -> Result<shared::session::Session, AppError> {
    session.map(|Extension(s)| s).ok_or(AppError::Unauthorized)
}

fn require_actor(session: Option<Extension<shared::session::Session>>) -> Result<String, AppError> {
    require_session(session).map(|s| s.person_unique)
}

/// 管理员判定：与 `shared::middleware::is_admin` 同一条 SQL（unique_id 或 id 命中
/// admin 角色，角色未删且未停用）。本 crate 只有 SeaORM 连接，没有 deadpool Pool，
/// 故就地实现；查询失败按「非管理员」处理（fail-closed）。
async fn is_admin_db(db: &DatabaseConnection, person_unique: &str) -> bool {
    let row = db
        .query_one(Statement::from_sql_and_values(
            DbBackend::Postgres,
            "SELECT EXISTS(SELECT 1 FROM auth_person p \
               LEFT JOIN auth_person_role pr ON pr.person_id = p.id \
               LEFT JOIN auth_role r ON r.id = pr.role_id \
               WHERE (p.unique_id = $1 OR p.id = $1) \
                 AND r.name = 'admin' AND r.deleted_at IS NULL AND r.disable = false) AS is_admin",
            vec![person_unique.into()],
        ))
        .await;
    match row {
        Ok(Some(row)) => row.try_get::<bool>("", "is_admin").unwrap_or(false),
        _ => false,
    }
}

async fn require_admin(db: &DatabaseConnection, person_unique: &str) -> Result<(), AppError> {
    if is_admin_db(db, person_unique).await {
        Ok(())
    } else {
        Err(AppError::Forbidden)
    }
}

/// 获取考勤记录列表
#[allow(non_snake_case)]
pub async fn record_list(
    db: Extension<DatabaseConnection>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let models = attendance_record::Entity::find()
        .order_by_desc(attendance_record::Column::CreateTime)
        .limit(20)
        .all(&db.0)
        .await
        .map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;

    let data: Vec<Value> = models
        .iter()
        .map(|m| {
            Value::Object(serde_json::Map::from_iter(
                [
                    ("id".to_string(), Value::String(m.id.clone())),
                    ("userId".to_string(), Value::String(m.user_id.clone())),
                    (
                        "checkInTime".to_string(),
                        Value::String(m.check_in_time.clone()),
                    ),
                    ("status".to_string(), Value::String(m.status.clone())),
                ]
                .into_iter()
                .chain(
                    m.check_out_time
                        .as_ref()
                        .map(|s| ("checkOutTime".to_string(), Value::String(s.clone()))),
                ),
            ))
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

/// 获取考勤规则列表
#[allow(non_snake_case)]
pub async fn rule_list(
    db: Extension<DatabaseConnection>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let models = attendance_rule::Entity::find()
        .order_by_asc(attendance_rule::Column::Name)
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
                ("startTime".to_string(), Value::String(m.start_time.clone())),
                ("endTime".to_string(), Value::String(m.end_time.clone())),
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

/// 创建考勤记录
#[allow(non_snake_case)]
pub async fn record_create(
    db: Extension<DatabaseConnection>,
    session: Option<Extension<shared::session::Session>>,
    AxumJson(payload): AxumJson<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let actor = require_actor(session)?;
    use attendance_record::ActiveModel;

    // P0：user_id 默认取会话人；请求体只能「不写」或「写自己」。
    // 代建（写他人）仅管理员放行，并用 created_by 留痕实际写入人。
    let requested_owner = payload
        .get("userId")
        .and_then(|v| v.as_str())
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(str::to_string);
    let user_id = match requested_owner {
        None => actor.clone(),
        Some(owner) if owner == actor => actor.clone(),
        Some(owner) => {
            require_admin(&db.0, &actor).await?;
            owner
        }
    };

    let check_in_time = payload
        .get("checkInTime")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string();
    let status = payload
        .get("status")
        .and_then(|v| v.as_str())
        .unwrap_or("normal")
        .to_string();
    let new_id = uuid::Uuid::new_v4().to_string();

    let active = ActiveModel {
        id: sea_orm::ActiveValue::Set(new_id.clone()),
        user_id: sea_orm::ActiveValue::Set(user_id.clone()),
        check_in_time: sea_orm::ActiveValue::Set(check_in_time.clone()),
        check_out_time: sea_orm::ActiveValue::Set(None),
        status: sea_orm::ActiveValue::Set(status.clone()),
        create_time: sea_orm::ActiveValue::Set(None),
        created_by: sea_orm::ActiveValue::Set(Some(actor.clone())),
    };

    active.insert(&db.0).await?;

    let created = attendance_record::Entity::find_by_id(new_id.clone())
        .one(&db.0)
        .await
        .map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?
        .ok_or_else(|| AppError::Internal)?;

    Ok(Json(ActionResult::success(
        serde_json::to_value(created)
            .map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?,
    )))
}

/// 记录归属谓词：本人 OR 管理员。**写进 SQL WHERE**，不做「先查后判」。
///
/// `$1` = 记录 id，`$2` = 会话人（person_unique）。
const RECORD_OWNER_PREDICATE: &str = "id = $1 AND (user_id = $2 OR EXISTS(\
     SELECT 1 FROM auth_person p \
       JOIN auth_person_role pr ON pr.person_id = p.id \
       JOIN auth_role r ON r.id = pr.role_id \
      WHERE (p.unique_id = $2 OR p.id = $2) \
        AND r.name = 'admin' AND r.deleted_at IS NULL AND r.disable = false))";

/// 写语句影响 0 行时区分「不存在」(404) 与「无权」(403)。
/// 变更本身已带 WHERE 条件，此处的读只用于选错误码，不参与授权判定。
async fn ownership_error(db: &DatabaseConnection, id: &str) -> AppError {
    match attendance_record::Entity::find_by_id(id.to_string())
        .one(db)
        .await
    {
        Ok(Some(_)) => AppError::Forbidden,
        Ok(None) => AppError::NotFound,
        Err(e) => AppError::internal_with_source(concat!(file!(), ":", line!()), e),
    }
}

/// 更新考勤记录
#[allow(non_snake_case)]
pub async fn record_update(
    db: Extension<DatabaseConnection>,
    session: Option<Extension<shared::session::Session>>,
    Path(id): Path<String>,
    AxumJson(payload): AxumJson<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let actor = require_actor(session)?;

    let check_out_time = payload
        .get("checkOutTime")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string());
    let status = payload
        .get("status")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string());

    // 归属条件在 WHERE 内，非本人且非管理员时影响 0 行（无 TOCTOU 窗口）。
    let sql = format!(
        "UPDATE x_attendance_record SET check_out_time = $3, status = COALESCE($4, status) \
         WHERE {RECORD_OWNER_PREDICATE} RETURNING id"
    );
    let updated =
        db.0.query_one(Statement::from_sql_and_values(
            DbBackend::Postgres,
            &sql,
            vec![
                id.clone().into(),
                actor.clone().into(),
                check_out_time.into(),
                status.into(),
            ],
        ))
        .await
        .map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;
    if updated.is_none() {
        return Err(ownership_error(&db.0, &id).await);
    }

    let model = attendance_record::Entity::find_by_id(id.clone())
        .one(&db.0)
        .await
        .map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?
        .ok_or(AppError::Internal)?;

    Ok(Json(ActionResult::success(
        serde_json::to_value(model)
            .map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?,
    )))
}

/// 删除考勤记录
#[allow(non_snake_case)]
pub async fn record_delete(
    db: Extension<DatabaseConnection>,
    session: Option<Extension<shared::session::Session>>,
    Path(id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let actor = require_actor(session)?;

    let sql =
        format!("DELETE FROM x_attendance_record WHERE {RECORD_OWNER_PREDICATE} RETURNING id");
    let deleted =
        db.0.query_one(Statement::from_sql_and_values(
            DbBackend::Postgres,
            &sql,
            vec![id.clone().into(), actor.into()],
        ))
        .await
        .map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;
    if deleted.is_none() {
        return Err(ownership_error(&db.0, &id).await);
    }

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
            ("deleted".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// 创建考勤规则
#[allow(non_snake_case)]
pub async fn rule_create(
    db: Extension<DatabaseConnection>,
    session: Option<Extension<shared::session::Session>>,
    AxumJson(payload): AxumJson<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let sess = require_session(session)?;
    // 考勤规则是全局配置（表内无「本人」列），写操作按配置管理口径收口到管理员。
    require_admin(&db.0, &sess.person_unique).await?;
    use attendance_rule::ActiveModel;

    let name = payload
        .get("name")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string();
    let start_time = payload
        .get("startTime")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string();
    let end_time = payload
        .get("EndTime")
        .or_else(|| payload.get("endTime"))
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string();
    let new_id = uuid::Uuid::new_v4().to_string();

    let active = ActiveModel {
        id: sea_orm::ActiveValue::Set(new_id.clone()),
        name: sea_orm::ActiveValue::Set(name.clone()),
        start_time: sea_orm::ActiveValue::Set(start_time.clone()),
        end_time: sea_orm::ActiveValue::Set(end_time.clone()),
        create_time: sea_orm::ActiveValue::Set(None),
        update_time: sea_orm::ActiveValue::Set(None),
    };

    active.insert(&db.0).await?;

    let created = attendance_rule::Entity::find_by_id(new_id.clone())
        .one(&db.0)
        .await
        .map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?
        .ok_or_else(|| AppError::Internal)?;

    Ok(Json(ActionResult::success(
        serde_json::to_value(created)
            .map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?,
    )))
}

/// 更新考勤规则
#[allow(non_snake_case)]
pub async fn rule_update(
    db: Extension<DatabaseConnection>,
    session: Option<Extension<shared::session::Session>>,
    Path(id): Path<String>,
    AxumJson(payload): AxumJson<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let sess = require_session(session)?;
    require_admin(&db.0, &sess.person_unique).await?;
    use attendance_rule::ActiveModel;

    let name = payload
        .get("name")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string());
    let start_time = payload
        .get("startTime")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string());
    let end_time = payload
        .get("endTime")
        .or_else(|| payload.get("EndTime"))
        .and_then(|v| v.as_str())
        .map(|s| s.to_string());

    let model = attendance_rule::Entity::find_by_id(id.clone())
        .one(&db.0)
        .await
        .map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?
        .ok_or_else(|| AppError::NotFound)?;

    let mut active: ActiveModel = model.clone().into();
    active.name = sea_orm::ActiveValue::Set(
        name.or_else(|| Some(model.name.clone()))
            .unwrap_or_default(),
    );
    active.start_time = sea_orm::ActiveValue::Set(
        start_time
            .or_else(|| Some(model.start_time.clone()))
            .unwrap_or_default(),
    );
    active.end_time = sea_orm::ActiveValue::Set(
        end_time
            .or_else(|| Some(model.end_time.clone()))
            .unwrap_or_default(),
    );

    active.update(&db.0).await?;

    let updated = attendance_rule::Entity::find_by_id(id.clone())
        .one(&db.0)
        .await
        .map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?
        .ok_or_else(|| AppError::Internal)?;

    Ok(Json(ActionResult::success(
        serde_json::to_value(updated)
            .map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?,
    )))
}

/// 删除考勤规则
#[allow(non_snake_case)]
pub async fn rule_delete(
    db: Extension<DatabaseConnection>,
    session: Option<Extension<shared::session::Session>>,
    Path(id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let sess = require_session(session)?;
    require_admin(&db.0, &sess.person_unique).await?;
    let deleted = attendance_rule::Entity::delete_by_id(id.clone())
        .exec(&db.0)
        .await
        .map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;

    if deleted.rows_affected == 0 {
        return Ok(Json(ActionResult::error("attendance rule not found")));
    }

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
            (
                "deleted".to_string(),
                Value::Bool(deleted.rows_affected > 0),
            ),
        ]),
    ))))
}

/// 创建考勤核心实体路由
pub fn attendance_core_entity_router(db: DatabaseConnection) -> Router {
    // 尝试创建数据库连接，测试环境中可能没有活跃的tokio runtime
    Router::new()
        .route("/api/attendance/core/entity/record/list", get(record_list))
        .route(
            "/api/attendance/core/entity/record/create",
            post(record_create),
        )
        .route(
            "/api/attendance/core/entity/record/{id}/update",
            post(record_update),
        )
        .route(
            "/api/attendance/core/entity/record/{id}/delete",
            post(record_delete),
        )
        .route("/api/attendance/core/entity/rule/list", get(rule_list))
        .route("/api/attendance/core/entity/rule/create", post(rule_create))
        .route(
            "/api/attendance/core/entity/rule/{id}/update",
            post(rule_update),
        )
        .route(
            "/api/attendance/core/entity/rule/{id}/delete",
            post(rule_delete),
        )
        .layer(Extension(db))
}

#[cfg(test)]
mod security_gate_tests_b82;
#[cfg(test)]
#[allow(clippy::module_inception)]
mod tests;
#[cfg(test)]
mod tests_generated;
#[cfg(test)]
mod tests_p0_ownership;

pub fn router(db: DatabaseConnection) -> axum::Router {
    crate::attendance_core_entity_router(db)
}
