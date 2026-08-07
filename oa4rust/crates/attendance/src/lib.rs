use axum::{
    extract::{Extension, Path},
    Json, Router,
};
use deadpool_postgres::Pool;
use serde::Serialize;
use serde_json::Value;
use shared::{error::AppError, response::ActionResult};

pub mod routes;

#[cfg(test)]
mod tests;

// --- Request/Response DTOs ---

#[derive(Debug, Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AdminInfo {
    pub id: String,
    pub unit_name: String,
    pub unit_ou: String,
    pub admin_name: String,
    pub admin: String,
    pub admin_level: String,
}

#[derive(Debug, Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EmployeeConfigInfo {
    pub id: String,
    pub top_unit_name: String,
    pub top_unit_ou: String,
    pub unit_name: String,
    pub unit_ou: String,
    pub employee_name: String,
    pub employee_number: String,
    pub config_type: String,
    pub emp_in_top_unit_time: String,
}

#[derive(Debug, Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StatisticalCycleInfo {
    pub id: String,
    pub top_unit_name: String,
    pub unit_name: String,
    pub cycle_year: String,
    pub cycle_month: String,
    pub cycle_start_date_string: String,
    pub cycle_end_date_string: String,
    pub description: String,
}

// --- Handlers ---

pub async fn list_admins(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query(
            "SELECT id, unit_name, unit_ou, admin_name, admin, admin_level FROM x_attendance_admin ORDER BY create_time",
            &[],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("unitName".to_string(), Value::String(row.get("unit_name"))),
                ("unitOu".to_string(), Value::String(row.get("unit_ou"))),
                ("adminName".to_string(), Value::String(row.get("admin_name"))),
                ("admin".to_string(), Value::String(row.get("admin"))),
                ("adminLevel".to_string(), Value::String(row.get("admin_level"))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

pub async fn list_employee_configs(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query(
            "SELECT id, top_unit_name, top_unit_ou, unit_name, unit_ou, employee_name, employee_number, config_type, emp_in_top_unit_time FROM x_attendance_employee_config ORDER BY create_time",
            &[],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("topUnitName".to_string(), Value::String(row.get("top_unit_name"))),
                ("topUnitOu".to_string(), Value::String(row.get("top_unit_ou"))),
                ("unitName".to_string(), Value::String(row.get("unit_name"))),
                ("unitOu".to_string(), Value::String(row.get("unit_ou"))),
                ("employeeName".to_string(), Value::String(row.get("employee_name"))),
                ("employeeNumber".to_string(), Value::String(row.get("employee_number"))),
                ("configType".to_string(), Value::String(row.get("config_type"))),
                ("empInTopUnitTime".to_string(), Value::String(row.get("emp_in_top_unit_time"))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

pub async fn list_statistical_cycles(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query(
            "SELECT id, top_unit_name, unit_name, cycle_year, cycle_month, cycle_start_date_string, cycle_end_date_string, description FROM x_attendance_statistical_cycle ORDER BY create_time",
            &[],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("topUnitName".to_string(), Value::String(row.get("top_unit_name"))),
                ("unitName".to_string(), Value::String(row.get("unit_name"))),
                ("cycleYear".to_string(), Value::String(row.get("cycle_year"))),
                ("cycleMonth".to_string(), Value::String(row.get("cycle_month"))),
                ("cycleStartDateString".to_string(), Value::String(row.get("cycle_start_date_string"))),
                ("cycleEndDateString".to_string(), Value::String(row.get("cycle_end_date_string"))),
                ("description".to_string(), Value::String(row.get("description"))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

pub fn attendance_router(pool: Pool) -> Router {
    routes::attendance_router(pool)
}

pub fn router(pool: deadpool_postgres::Pool) -> axum::Router {
    crate::attendance_router(pool)
}


pub async fn list_check_in_records(
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

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

pub async fn list_schedule_rules(
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

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

pub async fn list_appeal_records(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query(
            "SELECT id, person_id, appeal_status, creator, create_time FROM x_attendance_appeal_info ORDER BY create_time DESC",
            &[],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("personId".to_string(), Value::String(row.get("person_id"))),
                ("status".to_string(), Value::String(row.get("appeal_status"))),
                ("creator".to_string(), Value::String(row.get("creator"))),
                ("createTime".to_string(), Value::String(row.get("create_time"))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

pub async fn submit_appeal(
    pool: Extension<Pool>,
    Json(payload): Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let person_id = payload.get("personId").and_then(|v| v.as_str()).unwrap_or("").to_string();
    let appeal_date = payload.get("appealDate").and_then(|v| v.as_str()).unwrap_or("").to_string();
    let reason = payload.get("reason").and_then(|v| v.as_str()).unwrap_or("").to_string();
    let creator = payload.get("creator").and_then(|v| v.as_str()).unwrap_or("system").to_string();

    let new_id = uuid::Uuid::new_v4().to_string();

    client
        .execute(
            "INSERT INTO x_attendance_appeal_info (id, person_id, appeal_date, reason, appeal_status, creator, create_time) VALUES ($1, $2, $3, $4, 'appealed', $5, NOW())",
            &[&new_id, &person_id, &appeal_date, &reason, &creator],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(new_id)),
            ("personId".to_string(), Value::String(person_id)),
            ("appealDate".to_string(), Value::String(appeal_date)),
            ("status".to_string(), Value::String("appealed".to_string())),
            ("submitted".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn audit_appeal(
    pool: Extension<Pool>,
    Json(payload): Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let id = payload.get("id").and_then(|v| v.as_str()).unwrap_or("").to_string();
    let audit_status = payload.get("auditStatus").and_then(|v| v.as_str()).unwrap_or("approved");

    let result = client
        .execute(
            "UPDATE x_attendance_appeal_info SET audit_status = $1, update_time = NOW() WHERE id = $2",
            &[&audit_status, &id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if result == 0 {
        return Ok(Json(ActionResult::error("attendance appeal not found")));
    }

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
            ("audited".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn archive_appeal(
    pool: Extension<Pool>,
    Path(id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let result = client
        .execute(
            "UPDATE x_attendance_appeal_info SET archived = true, update_time = NOW() WHERE id = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if result == 0 {
        return Ok(Json(ActionResult::error("attendance appeal not found")));
    }

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
            ("archived".to_string(), Value::Bool(true)),
        ]),
    ))))
}