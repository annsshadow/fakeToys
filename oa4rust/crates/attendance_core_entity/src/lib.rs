use axum::{
    extract::{Extension, Json, Path},
    routing::{get, post},
    Json as AxumJson, Router,
};
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryOrder, QuerySelect};
use serde_json::Value;
use shared::{error::AppError, response::ActionResult};

pub mod entities;
pub mod routes;

use entities::{attendance_record, attendance_rule};

/// 获取考勤记录列表
pub async fn record_list(
    db: Extension<DatabaseConnection>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let models = attendance_record::Entity::find()
        .order_by_desc(attendance_record::Column::CreateTime)
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
                ("checkInTime".to_string(), Value::String(m.check_in_time.clone())),
                (
                    "checkOutTime".to_string(),
                    m.check_out_time
                        .clone()
                        .map(Value::String)
                        .unwrap_or(Value::Null),
                ),
                ("status".to_string(), Value::String(m.status.clone())),
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

/// 获取考勤规则列表
pub async fn rule_list(
    db: Extension<DatabaseConnection>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let models = attendance_rule::Entity::find()
        .order_by_asc(attendance_rule::Column::Name)
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
pub async fn record_create(
    db: Extension<DatabaseConnection>,
    AxumJson(payload): AxumJson<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    use attendance_record::ActiveModel;

    let user_id = payload.get("userId").and_then(|v| v.as_str()).unwrap_or("").to_string();
    let check_in_time = payload
        .get("checkInTime")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string();
    let status = payload.get("status").and_then(|v| v.as_str()).unwrap_or("normal").to_string();
    let new_id = uuid::Uuid::new_v4().to_string();

    let active = ActiveModel {
        id: sea_orm::ActiveValue::Set(new_id.clone()),
        user_id: sea_orm::ActiveValue::Set(user_id.clone()),
        check_in_time: sea_orm::ActiveValue::Set(check_in_time.clone()),
        check_out_time: sea_orm::ActiveValue::Set(None),
        status: sea_orm::ActiveValue::Set(status.clone()),
        create_time: sea_orm::ActiveValue::Set(None),
    };

    active
        .insert(&db.0)
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(new_id)),
            ("userId".to_string(), Value::String(user_id)),
            ("checkInTime".to_string(), Value::String(check_in_time)),
            ("status".to_string(), Value::String(status)),
            ("created".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// 更新考勤记录
pub async fn record_update(
    db: Extension<DatabaseConnection>,
    Path(id): Path<String>,
    AxumJson(payload): AxumJson<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    use attendance_record::ActiveModel;

    let check_out_time = payload.get("checkOutTime").and_then(|v| v.as_str()).map(|s| s.to_string());
    let status = payload.get("status").and_then(|v| v.as_str()).map(|s| s.to_string());

    let model = attendance_record::Entity::find_by_id(id.clone())
        .one(&db.0)
        .await
        .map_err(|_| AppError::Internal)?
        .ok_or_else(|| AppError::NotFound)?;

    let mut active: ActiveModel = model.clone().into();
    active.check_out_time = sea_orm::ActiveValue::Set(check_out_time);
    let status_val = status.clone().unwrap_or(model.status.clone());
    active.status = sea_orm::ActiveValue::Set(status_val);

    active
        .update(&db.0)
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
            ("updated".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// 删除考勤记录
pub async fn record_delete(
    db: Extension<DatabaseConnection>,
    Path(id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let deleted = attendance_record::Entity::delete_by_id(id.clone())
        .exec(&db.0)
        .await
        .map_err(|_| AppError::Internal)?;

    if deleted.rows_affected == 0 {
        return Ok(Json(ActionResult::error("attendance record not found")));
    }

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
            ("deleted".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// 创建考勤规则
pub async fn rule_create(
    db: Extension<DatabaseConnection>,
    AxumJson(payload): AxumJson<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    use attendance_rule::ActiveModel;

    let name = payload.get("name").and_then(|v| v.as_str()).unwrap_or("").to_string();
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

    active
        .insert(&db.0)
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(new_id)),
            ("name".to_string(), Value::String(name)),
            ("startTime".to_string(), Value::String(start_time)),
            ("endTime".to_string(), Value::String(end_time)),
            ("created".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// 更新考勤规则
pub async fn rule_update(
    db: Extension<DatabaseConnection>,
    Path(id): Path<String>,
    AxumJson(payload): AxumJson<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    use attendance_rule::ActiveModel;

    let name = payload.get("name").and_then(|v| v.as_str()).map(|s| s.to_string());
    let start_time = payload.get("startTime").and_then(|v| v.as_str()).map(|s| s.to_string());
    let end_time = payload
        .get("endTime")
        .or_else(|| payload.get("EndTime"))
        .and_then(|v| v.as_str())
        .map(|s| s.to_string());

    let model = attendance_rule::Entity::find_by_id(id.clone())
        .one(&db.0)
        .await
        .map_err(|_| AppError::Internal)?
        .ok_or_else(|| AppError::NotFound)?;

    let mut active: ActiveModel = model.clone().into();
    active.name = sea_orm::ActiveValue::Set(
        name.or_else(|| Some(model.name.clone())).unwrap_or_default(),
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

    active
        .update(&db.0)
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
            ("updated".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// 删除考勤规则
pub async fn rule_delete(
    db: Extension<DatabaseConnection>,
    Path(id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let deleted = attendance_rule::Entity::delete_by_id(id.clone())
        .exec(&db.0)
        .await
        .map_err(|_| AppError::Internal)?;

    if deleted.rows_affected == 0 {
        return Ok(Json(ActionResult::error("attendance rule not found")));
    }

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
            ("deleted".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// 创建考勤核心实体路由
pub fn attendance_core_entity_router(_pool: deadpool_postgres::Pool) -> Router {
    // 尝试创建数据库连接，测试环境中可能没有活跃的tokio runtime
    let db = std::panic::catch_unwind(|| {
        tokio::runtime::Handle::current()
            .block_on(shared::db::create_sea_orm_pool())
    })
    .ok()
    .and_then(|r| r.ok());

    let router = Router::new()
        .route(
            "/jaxrs/attendance/core/entity/record/list",
            get(record_list),
        )
        .route(
            "/jaxrs/attendance/core/entity/record/create",
            post(record_create),
        )
        .route(
            "/jaxrs/attendance/core/entity/record/{id}/update",
            post(record_update),
        )
        .route(
            "/jaxrs/attendance/core/entity/record/{id}/delete",
            get(record_delete),
        )
        .route(
            "/jaxrs/attendance/core/entity/rule/list",
            get(rule_list),
        )
        .route(
            "/jaxrs/attendance/core/entity/rule/create",
            post(rule_create),
        )
        .route(
            "/jaxrs/attendance/core/entity/rule/{id}/update",
            post(rule_update),
        )
        .route(
            "/jaxrs/attendance/core/entity/rule/{id}/delete",
            get(rule_delete),
        );

    match db {
        Some(conn) => router.layer(Extension(conn)),
        None => router,
    }
}

#[cfg(test)]
mod tests;
#[cfg(test)]
mod tests_generated;


pub fn router(pool: deadpool_postgres::Pool) -> axum::Router {
    crate::attendance_core_entity_router(pool)
}
