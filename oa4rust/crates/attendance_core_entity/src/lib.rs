use axum::{
    extract::{Extension, Path},
    routing::{get, post},
    Json, Router,
};
use deadpool_postgres::Pool;
use serde_json::Value;
use shared::{error::AppError, response::ActionResult};

pub mod routes;

// 考勤记录实体
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct AttendanceRecord {
    pub id: String,
    pub user_id: String,
    pub check_in_time: String,
    pub check_out_time: Option<String>,
    pub status: String,
}

// 考勤规则实体
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct AttendanceRule {
    pub id: String,
    pub name: String,
    pub start_time: String,
    pub end_time: String,
}

/// 获取考勤记录列表
/// 从数据库查询 x_attendance_record 表
pub async fn record_list(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT id, user_id, check_in_time, check_out_time, status FROM x_attendance_record ORDER BY check_in_time DESC LIMIT 20",
            &[],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("userId".to_string(), Value::String(row.get("user_id"))),
                ("checkInTime".to_string(), Value::String(row.get("check_in_time"))),
                (
                    "checkOutTime".to_string(),
                    row.get::<_, Option<String>>("check_out_time")
                        .map(Value::String)
                        .unwrap_or(Value::Null),
                ),
                ("status".to_string(), Value::String(row.get("status"))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
            ("data".to_string(), Value::Array(data)),
        ]),
    ))))
}

/// 获取考勤规则列表
/// 从数据库查询 x_attendance_rule 表
pub async fn rule_list(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT id, name, start_time, end_time FROM x_attendance_rule ORDER BY name LIMIT 20",
            &[],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("name".to_string(), Value::String(row.get("name"))),
                ("startTime".to_string(), Value::String(row.get("start_time"))),
                ("endTime".to_string(), Value::String(row.get("end_time"))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
            ("data".to_string(), Value::Array(data)),
        ]),
    ))))
}

/// 创建考勤核心实体路由
/// 注册以下路由：
/// - /jaxrs/attendance/core/entity/record/list - 考勤记录列表
/// - /jaxrs/attendance/core/entity/rule/list - 考勤规则列表
pub fn attendance_core_entity_router(pool: Pool) -> Router {
    Router::new()
        .route("/jaxrs/attendance/core/entity/record/list", get(record_list))
        .route("/jaxrs/attendance/core/entity/record/create", post(record_create))
        .route("/jaxrs/attendance/core/entity/record/{id}/update", post(record_update))
        .route("/jaxrs/attendance/core/entity/record/{id}/delete", get(record_delete))
        .route("/jaxrs/attendance/core/entity/rule/list", get(rule_list))
        .route("/jaxrs/attendance/core/entity/rule/create", post(rule_create))
        .route("/jaxrs/attendance/core/entity/rule/{id}/update", post(rule_update))
        .route("/jaxrs/attendance/core/entity/rule/{id}/delete", get(rule_delete))
        .layer(Extension(pool))
}

pub async fn record_create(
    pool: Extension<Pool>,
    Json(payload): Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let user_id = payload.get("userId").and_then(|v| v.as_str()).unwrap_or("").to_string();
    let check_in_time = payload.get("checkInTime").and_then(|v| v.as_str()).unwrap_or("").to_string();
    let status = payload.get("status").and_then(|v| v.as_str()).unwrap_or("normal").to_string();

    let new_id = uuid::Uuid::new_v4().to_string();

    client
        .execute(
            "INSERT INTO x_attendance_record (id, user_id, check_in_time, status, create_time) VALUES ($1, $2, $3, $4, NOW())",
            &[&new_id, &user_id, &check_in_time, &status],
        )
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

pub async fn record_update(
    pool: Extension<Pool>,
    Path(id): Path<String>,
    Json(payload): Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let check_out_time = payload.get("checkOutTime").and_then(|v| v.as_str());
    let status = payload.get("status").and_then(|v| v.as_str());

    let result = client
        .execute(
            "UPDATE x_attendance_record SET check_out_time = COALESCE($1, check_out_time), status = COALESCE($2, status), update_time = NOW() WHERE id = $3",
            &[&check_out_time, &status, &id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if result == 0 {
        return Ok(Json(ActionResult::error("attendance record not found")));
    }

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
            ("updated".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn record_delete(
    pool: Extension<Pool>,
    Path(id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let result = client
        .execute("DELETE FROM x_attendance_record WHERE id = $1", &[&id])
        .await
        .map_err(|_| AppError::Internal)?;

    if result == 0 {
        return Ok(Json(ActionResult::error("attendance record not found")));
    }

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
            ("deleted".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn rule_create(
    pool: Extension<Pool>,
    Json(payload): Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let name = payload.get("name").and_then(|v| v.as_str()).unwrap_or("").to_string();
    let start_time = payload.get("startTime").and_then(|v| v.as_str()).unwrap_or("").to_string();
    let end_time = payload.get("endTime").and_then(|v| v.as_str()).unwrap_or("").to_string();

    let new_id = uuid::Uuid::new_v4().to_string();

    client
        .execute(
            "INSERT INTO x_attendance_rule (id, name, start_time, end_time, create_time) VALUES ($1, $2, $3, $4, NOW())",
            &[&new_id, &name, &start_time, &end_time],
        )
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

pub async fn rule_update(
    pool: Extension<Pool>,
    Path(id): Path<String>,
    Json(payload): Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let name = payload.get("name").and_then(|v| v.as_str());
    let start_time = payload.get("startTime").and_then(|v| v.as_str());
    let end_time = payload.get("endTime").and_then(|v| v.as_str());

    let result = client
        .execute(
            "UPDATE x_attendance_rule SET name = COALESCE($1, name), start_time = COALESCE($2, start_time), end_time = COALESCE($3, end_time), update_time = NOW() WHERE id = $4",
            &[&name, &start_time, &end_time, &id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if result == 0 {
        return Ok(Json(ActionResult::error("attendance rule not found")));
    }

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
            ("updated".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn rule_delete(
    pool: Extension<Pool>,
    Path(id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let result = client
        .execute("DELETE FROM x_attendance_rule WHERE id = $1", &[&id])
        .await
        .map_err(|_| AppError::Internal)?;

    if result == 0 {
        return Ok(Json(ActionResult::error("attendance rule not found")));
    }

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
            ("deleted".to_string(), Value::Bool(true)),
        ]),
    ))))
}

#[cfg(test)]
mod tests;

pub fn router(pool: deadpool_postgres::Pool) -> axum::Router {
    crate::attendance_core_entity_router(pool)
}
