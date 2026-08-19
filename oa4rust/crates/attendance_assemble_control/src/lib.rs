use axum::{
    extract::{Extension, Path},
    Json, Router,
};
use deadpool_postgres::Pool;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use shared::{db::dialect, error::AppError, response::ActionResult};

pub mod routes;

#[cfg(test)]
mod tests;
#[cfg(test)]
mod tests_generated;


#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ControlRule {
    pub id: String,
    pub rule_name: String,
    pub rule_type: String,
    pub enabled: bool,
    pub description: Option<String>,
}

pub async fn list_control_rules(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query(
            "SELECT id, rule_name, rule_type, enabled, description FROM x_attendance_assemble_control_rule ORDER BY create_time",
            &[],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("ruleName".to_string(), Value::String(row.get("rule_name"))),
                ("ruleType".to_string(), Value::String(row.get("rule_type"))),
                ("enabled".to_string(), Value::Bool(row.get("enabled"))),
                ("description".to_string(), Value::String(row.get::<_, Option<String>>("description").unwrap_or_default())),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

pub async fn toggle_control_rule(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
    axum::extract::Json(payload): axum::extract::Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let enabled: bool = payload.get("enabled").and_then(|v| v.as_bool()).unwrap_or(false);

    client
        .execute(
            &dialect().format_sql(
                "UPDATE x_attendance_assemble_control_rule SET enabled = $1 WHERE id = $2",
            ),
            &[&enabled, &id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("id".to_string(), Value::String(id)),
        ("enabled".to_string(), Value::Bool(enabled)),
        ("updated".to_string(), Value::Bool(true)),
    ])))))
}

pub fn attendance_assemble_control_router(pool: Pool) -> Router {
    routes::attendance_assemble_control_routes(pool)
}

pub fn router(pool: deadpool_postgres::Pool) -> axum::Router {
    crate::routes::attendance_assemble_control_routes(pool)
}




/// GET /jaxrs/attendance/assemble/control/attendanceadmin/list/all
pub async fn attendanceadmin_list_all(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query(
            "SELECT id, person_id, unit_id, creator, create_time FROM x_attendance_admin ORDER BY create_time DESC",
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
                ("unitId".to_string(), Value::String(row.get("unit_id"))),
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

/// GET /jaxrs/attendance/assemble/control/attendanceadmin/{id}
pub async fn attendanceadmin_id(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let row = client
        .query_opt(
            &dialect().format_sql(
                "SELECT id, person_id, unit_id, creator, create_time FROM x_attendance_admin WHERE id = $1",
            ),
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let result = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("personId".to_string(), Value::String(row.get("person_id"))),
                ("unitId".to_string(), Value::String(row.get("unit_id"))),
                ("creator".to_string(), Value::String(row.get("creator"))),
                ("createTime".to_string(), Value::String(row.get("create_time"))),
            ]));
            Ok(Json(ActionResult::success(result)))
        }
        None => Ok(Json(ActionResult::error("attendance admin not found"))),
    }
}

/// POST /jaxrs/attendance/assemble/control/attendanceappealInfo/appeal/{id}
pub async fn attendanceappealInfo_appeal_id(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
    axum::extract::Json(payload): axum::extract::Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let status = payload
        .get("status")
        .and_then(|v| v.as_str())
        .unwrap_or("appealed");

    let result = client
        .execute(
            "UPDATE x_attendance_appeal_info SET appeal_status = $1, update_time = NOW() WHERE id = $2",
            &[&status, &id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if result == 0 {
        return Ok(Json(ActionResult::error("attendance appeal not found")));
    }

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id.to_string())),
            ("status".to_string(), Value::String(status.to_string())),
            ("updated".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// GET /jaxrs/attendance/assemble/control/attendanceconfig/list
pub async fn attendanceconfig_list(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query(
            "SELECT id, name, value, category, creator, create_time FROM x_attendance_config ORDER BY create_time DESC",
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
                ("value".to_string(), Value::String(row.get("value"))),
                ("category".to_string(), Value::String(row.get::<_, Option<String>>("category").unwrap_or_default())),
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

/// POST /jaxrs/attendance/assemble/control/attendanceconfig/save
pub async fn attendanceconfig_save(
    pool: Extension<Pool>,
    axum::extract::Json(payload): axum::extract::Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let id = payload.get("id").and_then(|v| v.as_str()).unwrap_or_default();
    let name = payload.get("name").and_then(|v| v.as_str()).unwrap_or_default();
    let value = payload.get("value").and_then(|v| v.as_str()).unwrap_or_default();
    let category = payload.get("category").and_then(|v| v.as_str());
    let creator = payload.get("creator").and_then(|v| v.as_str()).unwrap_or("system");

    let existing = client
        .query_opt("SELECT id FROM x_attendance_config WHERE id = $1", &[&id])
        .await
        .map_err(|_| AppError::Internal)?;

    if existing.is_some() {
        client
            .execute(
                "UPDATE x_attendance_config SET name = $1, value = $2, category = $3 WHERE id = $4",
                &[&name, &value, &category, &id],
            )
            .await
            .map_err(|_| AppError::Internal)?;
    } else {
        let new_id = if id.is_empty() {
            uuid::Uuid::new_v4().to_string()
        } else {
            id.to_string()
        };
        client
            .execute(
                "INSERT INTO x_attendance_config (id, name, value, category, creator, create_time) VALUES ($1, $2, $3, $4, $5, NOW())",
                &[&new_id, &name, &value, &category, &creator],
            )
            .await
            .map_err(|_| AppError::Internal)?;
    }

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id.to_string())),
            ("name".to_string(), Value::String(name.to_string())),
            ("value".to_string(), Value::String(value.to_string())),
            ("saved".to_string(), Value::Bool(true)),
        ]),
    ))))
}







/// POST /jaxrs/attendance/assemble/control/attendanceappealInfo/archive/{id}
pub async fn attendanceappealInfo_archive_id(
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

/// POST /jaxrs/attendance/assemble/control/attendanceappealInfo/audit
pub async fn attendanceappealInfo_audit(
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

/// POST /jaxrs/attendance/assemble/control/attendanceappealInfo/check 
pub async fn attendanceappealInfo_check(
    pool: Extension<Pool>,
    Json(payload): Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let id = payload.get("id").and_then(|v| v.as_str()).unwrap_or("").to_string();
    let checked = payload.get("checked").and_then(|v| v.as_bool()).unwrap_or(true);

    let result = client
        .execute(
            "UPDATE x_attendance_appeal_info SET checked = $1, update_time = NOW() WHERE id = $2",
            &[&checked, &id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if result == 0 {
        return Ok(Json(ActionResult::error("attendance appeal not found")));
    }

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
            ("checked".to_string(), Value::Bool(checked)),
        ]),
    ))))
}

/// GET /jaxrs/attendance/assemble/control/attendanceappealInfo/filter/list/{id}/next/{count}
pub async fn attendanceappealInfo_filter_list_id_next_count(
    pool: Extension<Pool>,
    Path((id, count)): Path<(String, i64)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let d = dialect();
    let sql = format!(
        "SELECT id, person_id, appeal_status, creator, create_time FROM x_attendance_appeal_info WHERE id > $1 ORDER BY create_time ASC LIMIT {}",
        d.cast_bigint_param(2),
    );
    let rows = client
        .query(&sql, &[&id, &count])
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("personId".to_string(), Value::String(row.get("person_id"))),
                ("status".to_string(), Value::String(row.get("appeal_status"))),
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

/// GET /jaxrs/attendance/assemble/control/attendanceappealInfo/filter/list/{id}/prev/{count}
pub async fn attendanceappealInfo_filter_list_id_prev_count(
    pool: Extension<Pool>,
    Path((id, count)): Path<(String, i64)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let d = dialect();
    let sql = format!(
        "SELECT id, person_id, appeal_status, creator, create_time FROM x_attendance_appeal_info WHERE id < $1 ORDER BY create_time DESC LIMIT {}",
        d.cast_bigint_param(2),
    );
    let rows = client
        .query(&sql, &[&id, &count])
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("personId".to_string(), Value::String(row.get("person_id"))),
                ("status".to_string(), Value::String(row.get("appeal_status"))),
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

/// GET /jaxrs/attendance/assemble/control/attendanceappealInfo/manager/list/{id}/next/{count}
pub async fn attendanceappealInfo_manager_list_id_next_count(
    pool: Extension<Pool>,
    Path((id, count)): Path<(String, i64)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query(
            "SELECT id, person_id, appeal_status, creator, create_time FROM x_attendance_appeal_info WHERE id > $1 AND creator = $2 ORDER BY create_time ASC LIMIT $3::bigint",
            &[&id, &"manager", &count],
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

/// POST /jaxrs/attendance/assemble/control/attendanceappealInfo/workflow/appeal/{id}
pub async fn attendanceappealInfo_workflow_appeal_id(
    pool: Extension<Pool>,
    Path(id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let result = client
        .execute(
            "UPDATE x_attendance_appeal_info SET workflow_status = 'appealed', update_time = NOW() WHERE id = $1",
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
            ("workflowAppealed".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// POST /jaxrs/attendance/assemble/control/attendanceappealInfo/workflow/sync
pub async fn attendanceappealInfo_workflow_sync(
    pool: Extension<Pool>,
    Json(payload): Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let appeal_id = payload.get("appealId").and_then(|v| v.as_str()).unwrap_or("").to_string();

    let result = client
        .execute(
            "UPDATE x_attendance_appeal_info SET workflow_synced = true, update_time = NOW() WHERE id = $1",
            &[&appeal_id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if result == 0 {
        return Ok(Json(ActionResult::error("attendance appeal not found")));
    }

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("synced".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// GET /jaxrs/attendance/assemble/control/attendanceappealInfo/{id}
pub async fn attendanceappealInfo_id(
    pool: Extension<Pool>,
    Path(id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let row = client
        .query_opt(
            "SELECT id, person_id, appeal_status, creator, create_time FROM x_attendance_appeal_info WHERE id = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let result = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("personId".to_string(), Value::String(row.get("person_id"))),
                ("status".to_string(), Value::String(row.get("appeal_status"))),
                ("creator".to_string(), Value::String(row.get("creator"))),
            ]));
            Ok(Json(ActionResult::success(result)))
        }
        None => Ok(Json(ActionResult::error("attendance appeal not found"))),
    }
}

/// POST /jaxrs/attendance/assemble/control/attendancedetail/analyse
pub async fn attendancedetail_analyse(
    pool: Extension<Pool>,
    Json(payload): Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let person_id = payload.get("personId").and_then(|v| v.as_str()).unwrap_or("").to_string();
    let start_date = payload.get("startDate").and_then(|v| v.as_str()).unwrap_or("").to_string();
    let end_date = payload.get("endDate").and_then(|v| v.as_str()).unwrap_or("").to_string();

    let _ = client
        .execute(
            "UPDATE x_attendance_detail SET analysed = true, update_time = NOW() WHERE person_id = $1 AND date >= $2 AND date <= $3",
            &[&person_id, &start_date, &end_date],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("analysed".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// POST /jaxrs/attendance/assemble/control/attendancedetail/analyse/id/{id}
pub async fn attendancedetail_analyse_id_id(
    pool: Extension<Pool>,
    Path(id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let _ = client
        .execute(
            "UPDATE x_attendance_detail SET analysed = true, update_time = NOW() WHERE id = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
            ("analysed".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// POST /jaxrs/attendance/assemble/control/attendancedetail/analyse/redo
pub async fn attendancedetail_analyse_redo(
    pool: Extension<Pool>,
    Json(payload): Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let person_id = payload.get("personId").and_then(|v| v.as_str()).unwrap_or("").to_string();

    let _ = client
        .execute(
            "UPDATE x_attendance_detail SET analysed = false, update_time = NOW() WHERE person_id = $1",
            &[&person_id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("redone".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// POST /jaxrs/attendance/assemble/control/attendancedetail/analyse/{startDate}/{endDate}
pub async fn attendancedetail_analyse_startDate_endDate(
    pool: Extension<Pool>,
    Path((start_date, end_date)): Path<(String, String)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let _ = client
        .execute(
            "UPDATE x_attendance_detail SET analysed = true, update_time = NOW() WHERE date >= $1 AND date <= $2",
            &[&start_date, &end_date],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("analysed".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// POST /jaxrs/attendance/assemble/control/attendancedetail/archive/{id}
pub async fn attendancedetail_archive_id(
    pool: Extension<Pool>,
    Path(id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let result = client
        .execute(
            "UPDATE x_attendance_detail SET archived = true, update_time = NOW() WHERE id = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if result == 0 {
        return Ok(Json(ActionResult::error("attendance detail not found")));
    }

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
            ("archived".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// POST /jaxrs/attendance/assemble/control/attendancedetail/checkDetailWithPersonByCycle/{cycleYear}/{cycleMonth}
pub async fn attendancedetail_checkDetailWithPersonByCycle_cycleYear_cycleMonth(
    pool: Extension<Pool>,
    Path((cycle_year, cycle_month)): Path<(String, String)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let _ = client
        .execute(
            "UPDATE x_attendance_detail SET checked = true, update_time = NOW() WHERE cycle_year = $1 AND cycle_month = $2",
            &[&cycle_year, &cycle_month],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("checked".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// GET /jaxrs/attendance/assemble/control/attendancedetail/filter/list
pub async fn attendancedetail_filter_list(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query(
            "SELECT id, person_id, date, status, creator FROM x_attendance_detail ORDER BY date DESC",
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
                ("date".to_string(), Value::String(row.get("date"))),
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

/// GET /jaxrs/attendance/assemble/control/attendancedetail/filter/list/topUnit
pub async fn attendancedetail_filter_list_topUnit(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query(
            "SELECT id, person_id, date, status, unit_id FROM x_attendance_detail WHERE unit_id IS NULL ORDER BY date DESC",
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
                ("date".to_string(), Value::String(row.get("date"))),
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

/// GET /jaxrs/attendance/assemble/control/attendancedetail/filter/list/unit
pub async fn attendancedetail_filter_list_unit(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query(
            "SELECT id, person_id, date, status, unit_id FROM x_attendance_detail WHERE unit_id IS NOT NULL ORDER BY date DESC",
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
                ("date".to_string(), Value::String(row.get("date"))),
                ("status".to_string(), Value::String(row.get("status"))),
                ("unitId".to_string(), Value::String(row.get("unit_id"))),
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

/// GET /jaxrs/attendance/assemble/control/attendancedetail/filter/list/user
pub async fn attendancedetail_filter_list_user(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query(
            "SELECT id, person_id, date, status FROM x_attendance_detail ORDER BY date DESC",
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
                ("date".to_string(), Value::String(row.get("date"))),
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

/// GET /jaxrs/attendance/assemble/control/attendancedetail/filter/list/{id}/next/{count}
pub async fn attendancedetail_filter_list_id_next_count(
    pool: Extension<Pool>,
    Path((id, count)): Path<(String, i64)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query(
            "SELECT id, person_id, date, status FROM x_attendance_detail WHERE id > $1 ORDER BY date ASC LIMIT $2::bigint",
            &[&id, &count],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("personId".to_string(), Value::String(row.get("person_id"))),
                ("date".to_string(), Value::String(row.get("date"))),
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

/// GET /jaxrs/attendance/assemble/control/attendancedetail/filter/list/{id}/prev/{count}
pub async fn attendancedetail_filter_list_id_prev_count(
    pool: Extension<Pool>,
    Path((id, count)): Path<(String, i64)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query(
            "SELECT id, person_id, date, status FROM x_attendance_detail WHERE id < $1 ORDER BY date DESC LIMIT $2::bigint",
            &[&id, &count],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("personId".to_string(), Value::String(row.get("person_id"))),
                ("date".to_string(), Value::String(row.get("date"))),
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

/// GET /jaxrs/attendance/assemble/control/attendancedetail/list/persons/nonesign
pub async fn attendancedetail_list_persons_nonesign(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query(
            "SELECT id, person_id, date, status FROM x_attendance_detail WHERE status = 'nonesign' ORDER BY date DESC",
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
                ("date".to_string(), Value::String(row.get("date"))),
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

/// GET /jaxrs/attendance/assemble/control/attendancedetail/list/{file_id}
pub async fn attendancedetail_list_file_id(
    pool: Extension<Pool>,
    Path(file_id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query(
            "SELECT id, person_id, date, status FROM x_attendance_detail WHERE file_id = $1 ORDER BY date DESC",
            &[&file_id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("personId".to_string(), Value::String(row.get("person_id"))),
                ("date".to_string(), Value::String(row.get("date"))),
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

/// GET /jaxrs/attendance/assemble/control/attendancedetail/mobile/filter/list/page/{page}/count/{count}
pub async fn attendancedetail_mobile_filter_list_page_page_count_count(
    pool: Extension<Pool>,
    Path((page, count)): Path<(i64, i64)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let offset = (page - 1) * count;
    let rows = client
        .query(
            "SELECT id, person_id, date, status FROM x_attendance_detail ORDER BY date DESC LIMIT $2::bigint OFFSET $1::bigint",
            &[&offset, &count],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("personId".to_string(), Value::String(row.get("person_id"))),
                ("date".to_string(), Value::String(row.get("date"))),
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

/// GET /jaxrs/attendance/assemble/control/attendancedetail/mobile/mobilepreview
pub async fn attendancedetail_mobile_mobilepreview(
    pool: Extension<Pool>,
    Json(payload): Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let person_id = payload.get("personId").and_then(|v| v.as_str()).unwrap_or("").to_string();
    let date = payload.get("date").and_then(|v| v.as_str()).unwrap_or("").to_string();

    let row = client
        .query_opt(
            "SELECT id, person_id, date, status FROM x_attendance_detail WHERE person_id = $1 AND date = $2 LIMIT 1",
            &[&person_id, &date],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let result = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("personId".to_string(), Value::String(row.get("person_id"))),
                ("date".to_string(), Value::String(row.get("date"))),
                ("status".to_string(), Value::String(row.get("status"))),
            ]));
            Ok(Json(ActionResult::success(result)))
        }
        None => Ok(Json(ActionResult::error("attendance detail not found"))),
    }
}

/// GET /jaxrs/attendance/assemble/control/attendancedetail/mobile/my
pub async fn attendancedetail_mobile_my(
    pool: Extension<Pool>,
    Json(payload): Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let person_id = payload.get("personId").and_then(|v| v.as_str()).unwrap_or("").to_string();

    let rows = client
        .query(
            "SELECT id, person_id, date, status FROM x_attendance_detail WHERE person_id = $1 ORDER BY date DESC",
            &[&person_id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("personId".to_string(), Value::String(row.get("person_id"))),
                ("date".to_string(), Value::String(row.get("date"))),
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

/// POST /jaxrs/attendance/assemble/control/attendancedetail/mobile/recive
pub async fn attendancedetail_mobile_recive(
    pool: Extension<Pool>,
    Json(payload): Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let person_id = payload.get("personId").and_then(|v| v.as_str()).unwrap_or("").to_string();
    let date = payload.get("date").and_then(|v| v.as_str()).unwrap_or("").to_string();

    let _ = client
        .execute(
            "UPDATE x_attendance_detail SET received = true, update_time = NOW() WHERE person_id = $1 AND date = $2",
            &[&person_id, &date],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("received".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// GET /jaxrs/attendance/assemble/control/attendancedetail/mobile/{id}
pub async fn attendancedetail_mobile_id(
    pool: Extension<Pool>,
    Path(id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let row = client
        .query_opt(
            "SELECT id, person_id, date, status FROM x_attendance_detail WHERE id = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let result = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("personId".to_string(), Value::String(row.get("person_id"))),
                ("date".to_string(), Value::String(row.get("date"))),
                ("status".to_string(), Value::String(row.get("status"))),
            ]));
            Ok(Json(ActionResult::success(result)))
        }
        None => Ok(Json(ActionResult::error("attendance detail not found"))),
    }
}

/// POST /jaxrs/attendance/assemble/control/attendancedetail/recive
pub async fn attendancedetail_recive(
    pool: Extension<Pool>,
    Json(payload): Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let id = payload.get("id").and_then(|v| v.as_str()).unwrap_or("").to_string();

    let _ = client
        .execute(
            "UPDATE x_attendance_detail SET received = true, update_time = NOW() WHERE id = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("received".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// POST /jaxrs/attendance/assemble/control/attendancedetail/reciveSingle
pub async fn attendancedetail_reciveSingle(
    pool: Extension<Pool>,
    Json(payload): Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let id = payload.get("id").and_then(|v| v.as_str()).unwrap_or("").to_string();

    let _ = client
        .execute(
            "UPDATE x_attendance_detail SET received = true, update_time = NOW() WHERE id = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("received".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// GET /jaxrs/attendance/assemble/control/attendancedetail/{id}
pub async fn attendancedetail_id(
    pool: Extension<Pool>,
    Path(id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let row = client
        .query_opt(
            "SELECT id, person_id, date, status FROM x_attendance_detail WHERE id = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let result = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("personId".to_string(), Value::String(row.get("person_id"))),
                ("date".to_string(), Value::String(row.get("date"))),
                ("status".to_string(), Value::String(row.get("status"))),
            ]));
            Ok(Json(ActionResult::success(result)))
        }
        None => Ok(Json(ActionResult::error("attendance detail not found"))),
    }
}

/// GET /jaxrs/attendance/assemble/control/attendanceemployeeconfig/list/all
pub async fn attendanceemployeeconfig_list_all(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query(
            "SELECT id, person_id, config_data, creator, create_time FROM x_attendance_employee_config ORDER BY create_time DESC",
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
                ("configData".to_string(), Value::String(row.get("config_data"))),
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

/// GET /jaxrs/attendance/assemble/control/attendanceemployeeconfig/{id}
pub async fn attendanceemployeeconfig_id(
    pool: Extension<Pool>,
    Path(id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let row = client
        .query_opt(
            "SELECT id, person_id, config_data, creator, create_time FROM x_attendance_employee_config WHERE id = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let result = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("personId".to_string(), Value::String(row.get("person_id"))),
                ("configData".to_string(), Value::String(row.get("config_data"))),
            ]));
            Ok(Json(ActionResult::success(result)))
        }
        None => Ok(Json(ActionResult::error("employee config not found"))),
    }
}

/// GET /jaxrs/attendance/assemble/control/attendanceimportfileinfo/list/all
pub async fn attendanceimportfileinfo_list_all(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query(
            "SELECT id, file_name, file_size, creator, create_time FROM x_attendance_import_file_info ORDER BY create_time DESC",
            &[],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("\"fileName\"".to_string(), Value::String(row.get("file_name"))),
                ("\"fileSize\"".to_string(), Value::Number(serde_json::Number::from(row.get::<_, i64>("file_size")))),
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

/// GET /jaxrs/attendance/assemble/control/attendanceimportfileinfo/{id}
pub async fn attendanceimportfileinfo_id(
    pool: Extension<Pool>,
    Path(id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let row = client
        .query_opt(
            "SELECT id, file_name, file_size, creator, create_time FROM x_attendance_import_file_info WHERE id = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let result = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("\"fileName\"".to_string(), Value::String(row.get("file_name"))),
                ("\"fileSize\"".to_string(), Value::Number(serde_json::Number::from(row.get::<_, i64>("file_size")))),
            ]));
            Ok(Json(ActionResult::success(result)))
        }
        None => Ok(Json(ActionResult::error("import file info not found"))),
    }
}

/// GET /jaxrs/attendance/assemble/control/attendanceschedulesetting/list/all
pub async fn attendanceschedulesetting_list_all(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query(
            "SELECT id, name, setting_data, creator, create_time FROM x_attendance_schedule_setting ORDER BY create_time DESC",
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
                ("settingData".to_string(), Value::String(row.get("setting_data"))),
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

/// GET /jaxrs/attendance/assemble/control/attendanceschedulesetting/list/topUnit/{name}
pub async fn attendanceschedulesetting_list_topUnit_name(
    pool: Extension<Pool>,
    Path(name): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query(
            "SELECT id, name, setting_data, creator FROM x_attendance_schedule_setting WHERE unit_id IS NULL ORDER BY create_time DESC",
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
                ("settingData".to_string(), Value::String(row.get("setting_data"))),
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

/// GET /jaxrs/attendance/assemble/control/attendanceschedulesetting/list/unit/{name}
pub async fn attendanceschedulesetting_list_unit_name(
    pool: Extension<Pool>,
    Path(name): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query(
            "SELECT id, name, setting_data, creator, unit_id FROM x_attendance_schedule_setting WHERE unit_id = $1 ORDER BY create_time DESC",
            &[&name],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("name".to_string(), Value::String(row.get("name"))),
                ("settingData".to_string(), Value::String(row.get("setting_data"))),
                ("unitId".to_string(), Value::String(row.get("unit_id"))),
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

/// GET /jaxrs/attendance/assemble/control/attendanceschedulesetting/{id}
pub async fn attendanceschedulesetting_id(
    pool: Extension<Pool>,
    Path(id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let row = client
        .query_opt(
            "SELECT id, name, setting_data, creator, create_time FROM x_attendance_schedule_setting WHERE id = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let result = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("name".to_string(), Value::String(row.get("name"))),
                ("settingData".to_string(), Value::String(row.get("setting_data"))),
            ]));
            Ok(Json(ActionResult::success(result)))
        }
        None => Ok(Json(ActionResult::error("schedule setting not found"))),
    }
}

/// GET /jaxrs/attendance/assemble/control/attendanceselfholiday/filter/list/{id}/next/{count}
pub async fn attendanceselfholiday_filter_list_id_next_count(
    pool: Extension<Pool>,
    Path((id, count)): Path<(String, i64)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query(
            "SELECT id, person_id, holiday_date, reason FROM x_attendance_selfholiday WHERE id > $1 ORDER BY holiday_date ASC LIMIT $2::bigint",
            &[&id, &count],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("personId".to_string(), Value::String(row.get("person_id"))),
                ("holidayDate".to_string(), Value::String(row.get("holiday_date"))),
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

/// GET /jaxrs/attendance/assemble/control/attendanceselfholiday/filter/list/{id}/prev/{count}
pub async fn attendanceselfholiday_filter_list_id_prev_count(
    pool: Extension<Pool>,
    Path((id, count)): Path<(String, i64)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query(
            "SELECT id, person_id, holiday_date, reason FROM x_attendance_selfholiday WHERE id < $1 ORDER BY holiday_date DESC LIMIT $2::bigint",
            &[&id, &count],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("personId".to_string(), Value::String(row.get("person_id"))),
                ("holidayDate".to_string(), Value::String(row.get("holiday_date"))),
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

/// GET /jaxrs/attendance/assemble/control/attendanceselfholiday/list/all
pub async fn attendanceselfholiday_list_all(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query(
            "SELECT id, person_id, holiday_date, reason, creator, create_time FROM x_attendance_selfholiday ORDER BY holiday_date DESC",
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
                ("holidayDate".to_string(), Value::String(row.get("holiday_date"))),
                ("reason".to_string(), Value::String(row.get("reason"))),
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

/// GET /jaxrs/attendance/assemble/control/attendanceselfholiday/{id}
pub async fn attendanceselfholiday_id(
    pool: Extension<Pool>,
    Path(id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let row = client
        .query_opt(
            "SELECT id, person_id, holiday_date, reason, creator, create_time FROM x_attendance_selfholiday WHERE id = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let result = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("personId".to_string(), Value::String(row.get("person_id"))),
                ("holidayDate".to_string(), Value::String(row.get("holiday_date"))),
                ("reason".to_string(), Value::String(row.get("reason"))),
            ]));
            Ok(Json(ActionResult::success(result)))
        }
        None => Ok(Json(ActionResult::error("self holiday not found"))),
    }
}

/// GET /jaxrs/attendance/assemble/control/attendancesetting/code/{code}
pub async fn attendancesetting_code_code(
    pool: Extension<Pool>,
    Path(code): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let row = client
        .query_opt(
            "SELECT id, code, name, value, creator, create_time FROM x_attendance_setting WHERE code = $1",
            &[&code],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let result = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("code".to_string(), Value::String(row.get("code"))),
                ("name".to_string(), Value::String(row.get("name"))),
                ("value".to_string(), Value::String(row.get("value"))),
            ]));
            Ok(Json(ActionResult::success(result)))
        }
        None => Ok(Json(ActionResult::error("setting not found"))),
    }
}

/// POST /jaxrs/attendance/assemble/control/attendancesetting/enable/type
pub async fn attendancesetting_enable_type(
    pool: Extension<Pool>,
    Json(payload): Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let code = payload.get("code").and_then(|v| v.as_str()).unwrap_or("").to_string();
    let enabled = payload.get("enabled").and_then(|v| v.as_bool()).unwrap_or(true);

    let result = client
        .execute(
            "UPDATE x_attendance_setting SET enabled = $1, update_time = NOW() WHERE code = $2",
            &[&enabled, &code],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if result == 0 {
        return Ok(Json(ActionResult::error("setting not found")));
    }

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("code".to_string(), Value::String(code)),
            ("enabled".to_string(), Value::Bool(enabled)),
            ("updated".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// GET /jaxrs/attendance/assemble/control/attendancesetting/list/all
pub async fn attendancesetting_list_all(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query(
            "SELECT id, code, name, value, creator, create_time FROM x_attendance_setting ORDER BY create_time DESC",
            &[],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("code".to_string(), Value::String(row.get("code"))),
                ("name".to_string(), Value::String(row.get("name"))),
                ("value".to_string(), Value::String(row.get("value"))),
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

/// GET /jaxrs/attendance/assemble/control/attendancesetting/{id}
pub async fn attendancesetting_id(
    pool: Extension<Pool>,
    Path(id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let row = client
        .query_opt(
            "SELECT id, code, name, value, creator, create_time FROM x_attendance_setting WHERE id = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let result = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("code".to_string(), Value::String(row.get("code"))),
                ("name".to_string(), Value::String(row.get("name"))),
                ("value".to_string(), Value::String(row.get("value"))),
            ]));
            Ok(Json(ActionResult::success(result)))
        }
        None => Ok(Json(ActionResult::error("setting not found"))),
    }
}

/// GET /jaxrs/attendance/assemble/control/attendancestatisticalcycle/cycleDetail/{year}/{month}
pub async fn attendancestatisticalcycle_cycleDetail_year_month(
    pool: Extension<Pool>,
    Path((year, month)): Path<(String, String)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let row = client
        .query_opt(
            "SELECT id, year, month, cycle_status, creator, create_time FROM x_attendance_statistical_cycle WHERE year = $1 AND month = $2 LIMIT 1",
            &[&year, &month],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let result = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("year".to_string(), Value::String(row.get("year"))),
                ("month".to_string(), Value::String(row.get("month"))),
                ("cycleStatus".to_string(), Value::String(row.get("cycle_status"))),
            ]));
            Ok(Json(ActionResult::success(result)))
        }
        None => Ok(Json(ActionResult::error("statistical cycle not found"))),
    }
}

/// GET /jaxrs/attendance/assemble/control/attendancestatisticalcycle/list/all
pub async fn attendancestatisticalcycle_list_all(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query(
            "SELECT id, year, month, cycle_status, creator, create_time FROM x_attendance_statistical_cycle ORDER BY year DESC, month DESC",
            &[],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("year".to_string(), Value::String(row.get("year"))),
                ("month".to_string(), Value::String(row.get("month"))),
                ("cycleStatus".to_string(), Value::String(row.get("cycle_status"))),
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

/// GET /jaxrs/attendance/assemble/control/attendancestatisticalcycle/{id}
pub async fn attendancestatisticalcycle_id(
    pool: Extension<Pool>,
    Path(id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let row = client
        .query_opt(
            "SELECT id, year, month, cycle_status, creator, create_time FROM x_attendance_statistical_cycle WHERE id = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let result = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("year".to_string(), Value::String(row.get("year"))),
                ("month".to_string(), Value::String(row.get("month"))),
                ("cycleStatus".to_string(), Value::String(row.get("cycle_status"))),
            ]));
            Ok(Json(ActionResult::success(result)))
        }
        None => Ok(Json(ActionResult::error("statistical cycle not found"))),
    }
}

/// GET /jaxrs/attendance/assemble/control/attendancestatisticrequirelog/list/all
pub async fn attendancestatisticrequirelog_list_all(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query(
            "SELECT id, require_type, status, creator, create_time FROM x_attendance_statistic_require_log ORDER BY create_time DESC",
            &[],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("requireType".to_string(), Value::String(row.get("require_type"))),
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

/// GET /jaxrs/attendance/assemble/control/attendancestatisticrequirelog/{id}
pub async fn attendancestatisticrequirelog_id(
    pool: Extension<Pool>,
    Path(id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let row = client
        .query_opt(
            "SELECT id, require_type, status, creator, create_time FROM x_attendance_statistic_require_log WHERE id = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let result = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("requireType".to_string(), Value::String(row.get("require_type"))),
                ("status".to_string(), Value::String(row.get("status"))),
            ]));
            Ok(Json(ActionResult::success(result)))
        }
        None => Ok(Json(ActionResult::error("statistic require log not found"))),
    }
}

/// GET /jaxrs/attendance/assemble/control/attendanceworkdayconfig/filter
pub async fn attendanceworkdayconfig_filter(
    pool: Extension<Pool>,
    Json(payload): Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let start_date = payload.get("startDate").and_then(|v| v.as_str()).unwrap_or("").to_string();
    let end_date = payload.get("endDate").and_then(|v| v.as_str()).unwrap_or("").to_string();

    let rows = client
        .query(
            "SELECT id, work_date, is_workday, creator FROM x_attendance_workday_config WHERE work_date >= $1 AND work_date <= $2 ORDER BY work_date ASC",
            &[&start_date, &end_date],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("workDate".to_string(), Value::String(row.get("work_date"))),
                ("isWorkday".to_string(), Value::Bool(row.get("is_workday"))),
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

/// GET /jaxrs/attendance/assemble/control/attendanceworkdayconfig/list/all
pub async fn attendanceworkdayconfig_list_all(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query(
            "SELECT id, work_date, is_workday, creator, create_time FROM x_attendance_workday_config ORDER BY work_date DESC",
            &[],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("workDate".to_string(), Value::String(row.get("work_date"))),
                ("isWorkday".to_string(), Value::Bool(row.get("is_workday"))),
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

/// GET /jaxrs/attendance/assemble/control/attendanceworkdayconfig/{id}
pub async fn attendanceworkdayconfig_id(
    pool: Extension<Pool>,
    Path(id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let row = client
        .query_opt(
            "SELECT id, work_date, is_workday, creator, create_time FROM x_attendance_workday_config WHERE id = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let result = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("workDate".to_string(), Value::String(row.get("work_date"))),
                ("isWorkday".to_string(), Value::Bool(row.get("is_workday"))),
            ]));
            Ok(Json(ActionResult::success(result)))
        }
        None => Ok(Json(ActionResult::error("workday config not found"))),
    }
}

/// GET /jaxrs/attendance/assemble/control/selfholidaysimple/docId/{docId}
pub async fn selfholidaysimple_docId_docId(
    pool: Extension<Pool>,
    Path(doc_id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let row = client
        .query_opt(
            "SELECT id, person_id, holiday_date, reason FROM x_attendance_selfholiday WHERE doc_id = $1 LIMIT 1",
            &[&doc_id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let result = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("personId".to_string(), Value::String(row.get("person_id"))),
                ("holidayDate".to_string(), Value::String(row.get("holiday_date"))),
                ("reason".to_string(), Value::String(row.get("reason"))),
            ]));
            Ok(Json(ActionResult::success(result)))
        }
        None => Ok(Json(ActionResult::error("self holiday not found"))),
    }
}

/// POST /jaxrs/attendance/assemble/control/statistic/do
pub async fn statistic_do(
    pool: Extension<Pool>,
    Json(payload): Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let person_id = payload.get("personId").and_then(|v| v.as_str()).unwrap_or("").to_string();
    let year = payload.get("year").and_then(|v| v.as_str()).unwrap_or("").to_string();
    let month = payload.get("month").and_then(|v| v.as_str()).unwrap_or("").to_string();

    let _ = client
        .execute(
            "INSERT INTO x_attendance_statistic (person_id, year, month, create_time) VALUES ($1, $2, $3, NOW()) ON CONFLICT (person_id, year, month) DO UPDATE SET update_time = NOW()",
            &[&person_id, &year, &month],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("done".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// GET /jaxrs/attendance/assemble/control/statisticshow/filter/personMonth/list/{id}/next/{count}
pub async fn statisticshow_filter_personMonth_list_id_next_count(
    pool: Extension<Pool>,
    Path((id, count)): Path<(String, i64)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query(
            "SELECT id, person_id, year, month, status FROM x_attendance_statisticshow WHERE id > $1 ORDER BY year DESC, month DESC LIMIT $2::bigint",
            &[&id, &count],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("personId".to_string(), Value::String(row.get("person_id"))),
                ("year".to_string(), Value::String(row.get("year"))),
                ("month".to_string(), Value::String(row.get("month"))),
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

/// GET /jaxrs/attendance/assemble/control/statisticshow/filter/personMonth/list/{id}/prev/{count}
pub async fn statisticshow_filter_personMonth_list_id_prev_count(
    pool: Extension<Pool>,
    Path((id, count)): Path<(String, i64)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query(
            "SELECT id, person_id, year, month, status FROM x_attendance_statisticshow WHERE id < $1 ORDER BY year DESC, month DESC LIMIT $2::bigint",
            &[&id, &count],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("personId".to_string(), Value::String(row.get("person_id"))),
                ("year".to_string(), Value::String(row.get("year"))),
                ("month".to_string(), Value::String(row.get("month"))),
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

/// GET /jaxrs/attendance/assemble/control/statisticshow/filter/topUnitDay/list/{id}/next/{count}
pub async fn statisticshow_filter_topUnitDay_list_id_next_count(
    pool: Extension<Pool>,
    Path((id, count)): Path<(String, i64)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query(
            "SELECT id, unit_id, work_date, status FROM x_attendance_statisticshow WHERE id > $1 AND unit_id IS NULL ORDER BY work_date ASC LIMIT $2::bigint",
            &[&id, &count],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("workDate".to_string(), Value::String(row.get("work_date"))),
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

/// GET /jaxrs/attendance/assemble/control/statisticshow/filter/topUnitDay/list/{id}/prev/{count}
pub async fn statisticshow_filter_topUnitDay_list_id_prev_count(
    pool: Extension<Pool>,
    Path((id, count)): Path<(String, i64)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query(
            "SELECT id, unit_id, work_date, status FROM x_attendance_statisticshow WHERE id < $1 AND unit_id IS NULL ORDER BY work_date DESC LIMIT $2::bigint",
            &[&id, &count],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("workDate".to_string(), Value::String(row.get("work_date"))),
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

/// GET /jaxrs/attendance/assemble/control/statisticshow/filter/topUnitMonth/list/{id}/next/{count}
pub async fn statisticshow_filter_topUnitMonth_list_id_next_count(
    pool: Extension<Pool>,
    Path((id, count)): Path<(String, i64)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query(
            "SELECT id, unit_id, year, month, status FROM x_attendance_statisticshow WHERE id > $1 AND unit_id IS NULL ORDER BY year DESC, month DESC LIMIT $2::bigint",
            &[&id, &count],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("year".to_string(), Value::String(row.get("year"))),
                ("month".to_string(), Value::String(row.get("month"))),
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

/// GET /jaxrs/attendance/assemble/control/statisticshow/filter/topUnitMonth/list/{id}/prev/{count}
pub async fn statisticshow_filter_topUnitMonth_list_id_prev_count(
    pool: Extension<Pool>,
    Path((id, count)): Path<(String, i64)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query(
            "SELECT id, unit_id, year, month, status FROM x_attendance_statisticshow WHERE id < $1 AND unit_id IS NULL ORDER BY year DESC, month DESC LIMIT $2::bigint",
            &[&id, &count],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("year".to_string(), Value::String(row.get("year"))),
                ("month".to_string(), Value::String(row.get("month"))),
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

/// GET /jaxrs/attendance/assemble/control/statisticshow/filter/unitDay/list/{id}/next/{count}
pub async fn statisticshow_filter_unitDay_list_id_next_count(
    pool: Extension<Pool>,
    Path((id, count)): Path<(String, i64)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query(
            "SELECT id, unit_id, work_date, status FROM x_attendance_statisticshow WHERE id > $1 AND unit_id IS NOT NULL ORDER BY work_date ASC LIMIT $2::bigint",
            &[&id, &count],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("unitId".to_string(), Value::String(row.get("unit_id"))),
                ("workDate".to_string(), Value::String(row.get("work_date"))),
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

/// GET /jaxrs/attendance/assemble/control/statisticshow/filter/unitDay/list/{id}/prev/{count}
pub async fn statisticshow_filter_unitDay_list_id_prev_count(
    pool: Extension<Pool>,
    Path((id, count)): Path<(String, i64)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query(
            "SELECT id, unit_id, work_date, status FROM x_attendance_statisticshow WHERE id < $1 AND unit_id IS NOT NULL ORDER BY work_date DESC LIMIT $2::bigint",
            &[&id, &count],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("unitId".to_string(), Value::String(row.get("unit_id"))),
                ("workDate".to_string(), Value::String(row.get("work_date"))),
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

/// GET /jaxrs/attendance/assemble/control/statisticshow/filter/unitMonth/list/{id}/next/{count}
pub async fn statisticshow_filter_unitMonth_list_id_next_count(
    pool: Extension<Pool>,
    Path((id, count)): Path<(String, i64)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query(
            "SELECT id, unit_id, year, month, status FROM x_attendance_statisticshow WHERE id > $1 AND unit_id IS NOT NULL ORDER BY year DESC, month DESC LIMIT $2::bigint",
            &[&id, &count],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("unitId".to_string(), Value::String(row.get("unit_id"))),
                ("year".to_string(), Value::String(row.get("year"))),
                ("month".to_string(), Value::String(row.get("month"))),
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

/// GET /jaxrs/attendance/assemble/control/statisticshow/filter/unitMonth/list/{id}/prev/{count}
pub async fn statisticshow_filter_unitMonth_list_id_prev_count(
    pool: Extension<Pool>,
    Path((id, count)): Path<(String, i64)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query(
            "SELECT id, unit_id, year, month, status FROM x_attendance_statisticshow WHERE id < $1 AND unit_id IS NOT NULL ORDER BY year DESC, month DESC LIMIT $2::bigint",
            &[&id, &count],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("unitId".to_string(), Value::String(row.get("unit_id"))),
                ("year".to_string(), Value::String(row.get("year"))),
                ("month".to_string(), Value::String(row.get("month"))),
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

/// GET /jaxrs/attendance/assemble/control/statisticshow/person/{name}/{year}/{month}
pub async fn statisticshow_person_name_year_month(
    pool: Extension<Pool>,
    Path((name, year, month)): Path<(String, String, String)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let row = client
        .query_opt(
            "SELECT id, person_id, year, month, status FROM x_attendance_statisticshow WHERE person_id = $1 AND year = $2 AND month = $3 LIMIT 1",
            &[&name, &year, &month],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let result = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("personId".to_string(), Value::String(row.get("person_id"))),
                ("year".to_string(), Value::String(row.get("year"))),
                ("month".to_string(), Value::String(row.get("month"))),
                ("status".to_string(), Value::String(row.get("status"))),
            ]));
            Ok(Json(ActionResult::success(result)))
        }
        None => Ok(Json(ActionResult::error("statisticshow not found"))),
    }
}

/// GET /jaxrs/attendance/assemble/control/statisticshow/persons/unit/subnested/{name}/{year}/{month}
pub async fn statisticshow_persons_unit_subnested_name_year_month(
    pool: Extension<Pool>,
    Path((name, year, month)): Path<(String, String, String)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query(
            "SELECT id, person_id, unit_id, year, month, status FROM x_attendance_statisticshow WHERE unit_id = $1 AND year = $2 AND month = $3 ORDER BY person_id ASC",
            &[&name, &year, &month],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("personId".to_string(), Value::String(row.get("person_id"))),
                ("unitId".to_string(), Value::String(row.get("unit_id"))),
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

/// GET /jaxrs/attendance/assemble/control/statisticshow/persons/unit/{name}/{year}/{month}
pub async fn statisticshow_persons_unit_name_year_month(
    pool: Extension<Pool>,
    Path((name, year, month)): Path<(String, String, String)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query(
            "SELECT id, person_id, unit_id, year, month, status FROM x_attendance_statisticshow WHERE unit_id = $1 AND year = $2 AND month = $3 ORDER BY person_id ASC",
            &[&name, &year, &month],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("personId".to_string(), Value::String(row.get("person_id"))),
                ("unitId".to_string(), Value::String(row.get("unit_id"))),
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

/// GET /jaxrs/attendance/assemble/control/statisticshow/topUnit/day/{name}/{year}/{month}
pub async fn statisticshow_topUnit_day_name_year_month(
    pool: Extension<Pool>,
    Path((name, year, month)): Path<(String, String, String)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query(
            "SELECT id, unit_id, work_date, status FROM x_attendance_statisticshow WHERE unit_id IS NULL AND year = $1 AND month = $2 ORDER BY work_date ASC",
            &[&year, &month],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("workDate".to_string(), Value::String(row.get("work_date"))),
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

/// GET /jaxrs/attendance/assemble/control/statisticshow/topUnit/{name}/{year}/{month}
pub async fn statisticshow_topUnit_name_year_month(
    pool: Extension<Pool>,
    Path((name, year, month)): Path<(String, String, String)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query(
            "SELECT id, unit_id, year, month, status FROM x_attendance_statisticshow WHERE unit_id IS NULL AND year = $1 AND month = $2",
            &[&year, &month],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("year".to_string(), Value::String(row.get("year"))),
                ("month".to_string(), Value::String(row.get("month"))),
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

/// GET /jaxrs/attendance/assemble/control/statisticshow/unit/day/topUnit/{name}/{date}
pub async fn statisticshow_unit_day_topUnit_name_date(
    pool: Extension<Pool>,
    Path((name, date)): Path<(String, String)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query(
            "SELECT id, unit_id, work_date, status FROM x_attendance_statisticshow WHERE unit_id = $1 AND work_date = $2 AND unit_id IS NOT NULL",
            &[&name, &date],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("workDate".to_string(), Value::String(row.get("work_date"))),
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

/// GET /jaxrs/attendance/assemble/control/statisticshow/unit/day/{name}/{date}
pub async fn statisticshow_unit_day_name_date(
    pool: Extension<Pool>,
    Path((name, date)): Path<(String, String)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query(
            "SELECT id, unit_id, work_date, status FROM x_attendance_statisticshow WHERE unit_id = $1 AND work_date = $2 ORDER BY work_date ASC",
            &[&name, &date],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("unitId".to_string(), Value::String(row.get("unit_id"))),
                ("workDate".to_string(), Value::String(row.get("work_date"))),
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

/// GET /jaxrs/attendance/assemble/control/statisticshow/unit/day/{name}/{year}/{month}
pub async fn statisticshow_unit_day_name_year_month(
    pool: Extension<Pool>,
    Path((name, year, month)): Path<(String, String, String)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query(
            "SELECT id, unit_id, work_date, status FROM x_attendance_statisticshow WHERE unit_id = $1 AND year = $2 AND month = $3 ORDER BY work_date ASC",
            &[&name, &year, &month],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("workDate".to_string(), Value::String(row.get("work_date"))),
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

/// GET /jaxrs/attendance/assemble/control/statisticshow/unit/subnested/{name}/{year}/{month}
pub async fn statisticshow_unit_subnested_name_year_month(
    pool: Extension<Pool>,
    Path((name, year, month)): Path<(String, String, String)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query(
            "SELECT id, person_id, unit_id, year, month, status FROM x_attendance_statisticshow WHERE unit_id = $1 AND year = $2 AND month = $3 ORDER BY person_id ASC",
            &[&name, &year, &month],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("personId".to_string(), Value::String(row.get("person_id"))),
                ("unitId".to_string(), Value::String(row.get("unit_id"))),
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

/// GET /jaxrs/attendance/assemble/control/statisticshow/unit/sum/{name}/{year}/{month}
pub async fn statisticshow_unit_sum_name_year_month(
    pool: Extension<Pool>,
    Path((name, year, month)): Path<(String, String, String)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let row = client
        .query_opt(
            "SELECT id, unit_id, year, month, SUM(status) as total FROM x_attendance_statisticshow WHERE unit_id = $1 AND year = $2 AND month = $3 GROUP BY id, unit_id, year, month LIMIT 1",
            &[&name, &year, &month],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let result = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("unitId".to_string(), Value::String(row.get("unit_id"))),
                ("total".to_string(), Value::Number(serde_json::Number::from(row.get::<_, i64>("total")))),
            ]));
            Ok(Json(ActionResult::success(result)))
        }
        None => Ok(Json(ActionResult::error("statisticshow not found"))),
    }
}

/// GET /jaxrs/attendance/assemble/control/statisticshow/unit/topUnit/{name}/{year}/{month}
pub async fn statisticshow_unit_topUnit_name_year_month(
    pool: Extension<Pool>,
    Path((name, year, month)): Path<(String, String, String)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query(
            "SELECT id, unit_id, year, month, status FROM x_attendance_statisticshow WHERE unit_id = $1 AND year = $2 AND month = $3 ORDER BY year DESC, month DESC",
            &[&name, &year, &month],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("unitId".to_string(), Value::String(row.get("unit_id"))),
                ("year".to_string(), Value::String(row.get("year"))),
                ("month".to_string(), Value::String(row.get("month"))),
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

/// GET /jaxrs/attendance/assemble/control/statisticshow/unit/{name}/{year}/{month}
pub async fn statisticshow_unit_name_year_month(
    pool: Extension<Pool>,
    Path((name, year, month)): Path<(String, String, String)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query(
            "SELECT id, unit_id, year, month, status FROM x_attendance_statisticshow WHERE unit_id = $1 AND year = $2 AND month = $3 ORDER BY year DESC, month DESC",
            &[&name, &year, &month],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("unitId".to_string(), Value::String(row.get("unit_id"))),
                ("year".to_string(), Value::String(row.get("year"))),
                ("month".to_string(), Value::String(row.get("month"))),
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

/// GET /jaxrs/attendance/assemble/control/uuid/random
pub async fn uuid_random() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("uuid".to_string(), Value::String(uuid::Uuid::new_v4().to_string())),
        ]),
    ))))
}

/// GET /jaxrs/attendance/assemble/control/workplace/list/all
pub async fn workplace_list_all(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query(
            "SELECT id, name, address, creator, create_time FROM x_attendance_workplace ORDER BY create_time DESC",
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
                ("address".to_string(), Value::String(row.get("address"))),
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

/// GET /jaxrs/attendance/assemble/control/workplace/{id}
pub async fn workplace_id(
    pool: Extension<Pool>,
    Path(id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let row = client
        .query_opt(
            "SELECT id, name, address, creator, create_time FROM x_attendance_workplace WHERE id = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let result = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("name".to_string(), Value::String(row.get("name"))),
                ("address".to_string(), Value::String(row.get("address"))),
            ]));
            Ok(Json(ActionResult::success(result)))
        }
        None => Ok(Json(ActionResult::error("workplace not found"))),
    }
}
