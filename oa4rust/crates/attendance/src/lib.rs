use axum::{
    extract::Extension,
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
