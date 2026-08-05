use axum::{
    extract::Extension,
    routing::get,
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
        .route("/jaxrs/attendance/core/entity/rule/list", get(rule_list))
        .layer(Extension(pool))
}

#[cfg(test)]
mod tests;

pub fn router(_pool: deadpool_postgres::Pool) -> axum::Router {
    axum::Router::new()
        .route("/attendance_core_entity/health", axum::routing::get(|| async { "TODO: attendance_core_entity - real implementation needed" }))
}