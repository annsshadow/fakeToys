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
#[cfg(test)]
mod tests_u2;
#[cfg(test)]
mod tests_u2_legacy;


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

    let count = data.len() as i64;
    Ok(Json(ActionResult::java_success(
        Value::Array(data),
        count,
        0,
    )))
}

pub async fn toggle_control_rule(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
    axum::extract::Json(payload): axum::extract::Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let enabled: bool = payload.get("enabled").and_then(|v| v.as_bool()).unwrap_or(false);

    let result = client
        .execute(
            &dialect().format_sql(
                "UPDATE x_attendance_assemble_control_rule SET enabled = $1 WHERE id = $2",
            ),
            &[&enabled, &id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if result == 0 {
        return Ok(Json(ActionResult::error("control rule not found")));
    }

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("id".to_string(), Value::String(id)),
        ("enabled".to_string(), Value::Bool(enabled)),
        ("updated".to_string(), Value::Number(serde_json::Number::from(result as i64))),
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

    let count = data.len() as i64;
    Ok(Json(ActionResult::java_success(
        Value::Array(data),
        count,
        0,
    )))
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
        None => Ok(Json(ActionResult::success(Value::Null))),
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
            ("updated".to_string(), Value::Number(serde_json::Number::from(result as i64))),
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

    let count = data.len() as i64;
    Ok(Json(ActionResult::java_success(
        Value::Array(data),
        count,
        0,
    )))
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

    let result = if existing.is_some() {
        client
            .execute(
                "UPDATE x_attendance_config SET name = $1, value = $2, category = $3 WHERE id = $4",
                &[&name, &value, &category, &id],
            )
            .await
            .map_err(|_| AppError::Internal)?
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
            .map_err(|_| AppError::Internal)?
    };

    if result == 0 {
        return Ok(Json(ActionResult::error("attendance config not found")));
    }

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id.to_string())),
            ("name".to_string(), Value::String(name.to_string())),
            ("value".to_string(), Value::String(value.to_string())),
            ("saved".to_string(), Value::Number(serde_json::Number::from(result as i64))),
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
            ("archived".to_string(), Value::Number(serde_json::Number::from(result as i64))),
        ]),
    ))))
}

/// POST /jaxrs/attendance/assemble/control/attendanceappealInfo/audit
pub async fn attendanceappealInfo_audit(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    Json(payload): Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    require_admin(&pool, &session).await?;
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
            ("audited".to_string(), Value::Number(serde_json::Number::from(result as i64))),
        ]),
    ))))
}

/// POST /jaxrs/attendance/assemble/control/attendanceappealInfo/check 
pub async fn attendanceappealInfo_check(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    Json(payload): Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    require_admin(&pool, &session).await?;
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

    let count = data.len() as i64;
    Ok(Json(ActionResult::java_success(
        Value::Array(data),
        count,
        0,
    )))
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

    let count = data.len() as i64;
    Ok(Json(ActionResult::java_success(
        Value::Array(data),
        count,
        0,
    )))
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

    let count = data.len() as i64;
    Ok(Json(ActionResult::java_success(
        Value::Array(data),
        count,
        0,
    )))
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
            ("workflowAppealed".to_string(), Value::Number(serde_json::Number::from(result as i64))),
        ]),
    ))))
}

/// POST /jaxrs/attendance/assemble/control/attendanceappealInfo/workflow/sync
pub async fn attendanceappealInfo_workflow_sync(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    Json(payload): Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    require_admin(&pool, &session).await?;
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
            ("synced".to_string(), Value::Number(serde_json::Number::from(result as i64))),
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
        None => Ok(Json(ActionResult::success(Value::Null))),
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

    let result = client
        .execute(
            "UPDATE x_attendance_detail SET analysed = true, update_time = NOW() WHERE person_id = $1 AND date >= $2 AND date <= $3",
            &[&person_id, &start_date, &end_date],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("analysed".to_string(), Value::Number(serde_json::Number::from(result as i64))),
        ]),
    ))))
}

/// POST /jaxrs/attendance/assemble/control/attendancedetail/analyse/id/{id}
pub async fn attendancedetail_analyse_id_id(
    pool: Extension<Pool>,
    Path(id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let result = client
        .execute(
            "UPDATE x_attendance_detail SET analysed = true, update_time = NOW() WHERE id = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
            ("analysed".to_string(), Value::Number(serde_json::Number::from(result as i64))),
        ]),
    ))))
}

/// POST /jaxrs/attendance/assemble/control/attendancedetail/analyse/redo
pub async fn attendancedetail_analyse_redo(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    Json(payload): Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    require_admin(&pool, &session).await?;
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let person_id = payload.get("personId").and_then(|v| v.as_str()).unwrap_or("").to_string();

    let result = client
        .execute(
            "UPDATE x_attendance_detail SET analysed = false, update_time = NOW() WHERE person_id = $1",
            &[&person_id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("redone".to_string(), Value::Number(serde_json::Number::from(result as i64))),
        ]),
    ))))
}

/// POST /jaxrs/attendance/assemble/control/attendancedetail/analyse/{startDate}/{endDate}
pub async fn attendancedetail_analyse_startDate_endDate(
    pool: Extension<Pool>,
    Path((start_date, end_date)): Path<(String, String)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let result = client
        .execute(
            "UPDATE x_attendance_detail SET analysed = true, update_time = NOW() WHERE date >= $1 AND date <= $2",
            &[&start_date, &end_date],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("analysed".to_string(), Value::Number(serde_json::Number::from(result as i64))),
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
            ("archived".to_string(), Value::Number(serde_json::Number::from(result as i64))),
        ]),
    ))))
}

/// POST /jaxrs/attendance/assemble/control/attendancedetail/checkDetailWithPersonByCycle/{cycleYear}/{cycleMonth}
pub async fn attendancedetail_checkDetailWithPersonByCycle_cycleYear_cycleMonth(
    pool: Extension<Pool>,
    Path((cycle_year, cycle_month)): Path<(String, String)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let result = client
        .execute(
            "UPDATE x_attendance_detail SET checked = true, update_time = NOW() WHERE cycle_year = $1 AND cycle_month = $2",
            &[&cycle_year, &cycle_month],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("checked".to_string(), Value::Number(serde_json::Number::from(result as i64))),
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

    let count = data.len() as i64;
    Ok(Json(ActionResult::java_success(
        Value::Array(data),
        count,
        0,
    )))
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

    let count = data.len() as i64;
    Ok(Json(ActionResult::java_success(
        Value::Array(data),
        count,
        0,
    )))
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

    let count = data.len() as i64;
    Ok(Json(ActionResult::java_success(
        Value::Array(data),
        count,
        0,
    )))
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

    let count = data.len() as i64;
    Ok(Json(ActionResult::java_success(
        Value::Array(data),
        count,
        0,
    )))
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

    let count = data.len() as i64;
    Ok(Json(ActionResult::java_success(
        Value::Array(data),
        count,
        0,
    )))
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

    let count = data.len() as i64;
    Ok(Json(ActionResult::java_success(
        Value::Array(data),
        count,
        0,
    )))
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

    let count = data.len() as i64;
    Ok(Json(ActionResult::java_success(
        Value::Array(data),
        count,
        0,
    )))
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

    let count = data.len() as i64;
    Ok(Json(ActionResult::java_success(Value::Array(data), count, 0)))
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

    let count = data.len() as i64;
    Ok(Json(ActionResult::java_success(
        Value::Array(data),
        count,
        0,
    )))
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
        None => Ok(Json(ActionResult::success(Value::Null))),
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

    let count = data.len() as i64;
    Ok(Json(ActionResult::java_success(
        Value::Array(data),
        count,
        0,
    )))
}

/// POST /jaxrs/attendance/assemble/control/attendancedetail/mobile/recive
pub async fn attendancedetail_mobile_recive(
    pool: Extension<Pool>,
    Json(payload): Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let person_id = payload.get("personId").and_then(|v| v.as_str()).unwrap_or("").to_string();
    let date = payload.get("date").and_then(|v| v.as_str()).unwrap_or("").to_string();

    let result = client
        .execute(
            "UPDATE x_attendance_detail SET received = true, update_time = NOW() WHERE person_id = $1 AND date = $2",
            &[&person_id, &date],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("received".to_string(), Value::Number(serde_json::Number::from(result as i64))),
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

    let result = client
        .execute(
            "UPDATE x_attendance_detail SET received = true, update_time = NOW() WHERE id = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("received".to_string(), Value::Number(serde_json::Number::from(result as i64))),
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

    let result = client
        .execute(
            "UPDATE x_attendance_detail SET received = true, update_time = NOW() WHERE id = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("received".to_string(), Value::Number(serde_json::Number::from(result as i64))),
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

    let count = data.len() as i64;
    Ok(Json(ActionResult::java_success(Value::Array(data), count, 0)))
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

    let count = data.len() as i64;
    Ok(Json(ActionResult::java_success(Value::Array(data), count, 0)))
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

    let count = data.len() as i64;
    Ok(Json(ActionResult::java_success(Value::Array(data), count, 0)))
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

    let count = data.len() as i64;
    Ok(Json(ActionResult::java_success(Value::Array(data), count, 0)))
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

    let count = data.len() as i64;
    Ok(Json(ActionResult::java_success(Value::Array(data), count, 0)))
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

    let count = data.len() as i64;
    Ok(Json(ActionResult::java_success(
        Value::Array(data),
        count,
        0,
    )))
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

    let count = data.len() as i64;
    Ok(Json(ActionResult::java_success(
        Value::Array(data),
        count,
        0,
    )))
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

    let count = data.len() as i64;
    Ok(Json(ActionResult::java_success(Value::Array(data), count, 0)))
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
    body: Option<Json<Value>>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    // Java 端该端点为 GET 无 body；字段均可选
    let payload = body.map(|Json(v)| v).unwrap_or(Value::Null);

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
            ("updated".to_string(), Value::Number(serde_json::Number::from(result as i64))),
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

    let count = data.len() as i64;
    Ok(Json(ActionResult::java_success(Value::Array(data), count, 0)))
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

    let count = data.len() as i64;
    Ok(Json(ActionResult::java_success(Value::Array(data), count, 0)))
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

    let count = data.len() as i64;
    Ok(Json(ActionResult::java_success(Value::Array(data), count, 0)))
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

    let count = data.len() as i64;
    Ok(Json(ActionResult::java_success(
        Value::Array(data),
        count,
        0,
    )))
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

    let count = data.len() as i64;
    Ok(Json(ActionResult::java_success(Value::Array(data), count, 0)))
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
    session: Extension<shared::session::Session>,
    body: Option<Json<Value>>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    require_admin(&pool, &session).await?;
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    // Java 端该端点为 GET 无 body；字段均可选，缺省空串
    let payload = body.map(|Json(v)| v).unwrap_or(Value::Null);

    let person_id = payload.get("personId").and_then(|v| v.as_str()).unwrap_or("").to_string();
    let year = payload.get("year").and_then(|v| v.as_str()).unwrap_or("").to_string();
    let month = payload.get("month").and_then(|v| v.as_str()).unwrap_or("").to_string();

    let result = client
        .execute(
            "INSERT INTO x_attendance_statistic (person_id, year, month, create_time) VALUES ($1, $2, $3, NOW()) ON CONFLICT (person_id, year, month) DO UPDATE SET update_time = NOW()",
            &[&person_id, &year, &month],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("done".to_string(), Value::Number(serde_json::Number::from(result as i64))),
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

    let count = data.len() as i64;
    Ok(Json(ActionResult::java_success(
        Value::Array(data),
        count,
        0,
    )))
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

    let count = data.len() as i64;
    Ok(Json(ActionResult::java_success(
        Value::Array(data),
        count,
        0,
    )))
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

    let count = data.len() as i64;
    Ok(Json(ActionResult::java_success(
        Value::Array(data),
        count,
        0,
    )))
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

    let count = data.len() as i64;
    Ok(Json(ActionResult::java_success(
        Value::Array(data),
        count,
        0,
    )))
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

    let count = data.len() as i64;
    Ok(Json(ActionResult::java_success(
        Value::Array(data),
        count,
        0,
    )))
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

    let count = data.len() as i64;
    Ok(Json(ActionResult::java_success(
        Value::Array(data),
        count,
        0,
    )))
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

    let count = data.len() as i64;
    Ok(Json(ActionResult::java_success(
        Value::Array(data),
        count,
        0,
    )))
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

    let count = data.len() as i64;
    Ok(Json(ActionResult::java_success(
        Value::Array(data),
        count,
        0,
    )))
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

    let count = data.len() as i64;
    Ok(Json(ActionResult::java_success(
        Value::Array(data),
        count,
        0,
    )))
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

    let count = data.len() as i64;
    Ok(Json(ActionResult::java_success(
        Value::Array(data),
        count,
        0,
    )))
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

    let count = data.len() as i64;
    Ok(Json(ActionResult::java_success(
        Value::Array(data),
        count,
        0,
    )))
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

    let count = data.len() as i64;
    Ok(Json(ActionResult::java_success(
        Value::Array(data),
        count,
        0,
    )))
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

    let count = data.len() as i64;
    Ok(Json(ActionResult::java_success(
        Value::Array(data),
        count,
        0,
    )))
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

    let count = data.len() as i64;
    Ok(Json(ActionResult::java_success(
        Value::Array(data),
        count,
        0,
    )))
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

    let count = data.len() as i64;
    Ok(Json(ActionResult::java_success(
        Value::Array(data),
        count,
        0,
    )))
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

    let count = data.len() as i64;
    Ok(Json(ActionResult::java_success(
        Value::Array(data),
        count,
        0,
    )))
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

    let count = data.len() as i64;
    Ok(Json(ActionResult::java_success(
        Value::Array(data),
        count,
        0,
    )))
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

    let count = data.len() as i64;
    Ok(Json(ActionResult::java_success(
        Value::Array(data),
        count,
        0,
    )))
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
        None => Ok(Json(ActionResult::success(Value::Null))),
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

    let count = data.len() as i64;
    Ok(Json(ActionResult::java_success(
        Value::Array(data),
        count,
        0,
    )))
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

    let count = data.len() as i64;
    Ok(Json(ActionResult::java_success(
        Value::Array(data),
        count,
        0,
    )))
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

    let count = data.len() as i64;
    Ok(Json(ActionResult::java_success(Value::Array(data), count, 0)))
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

// ════════════════════════════════════════════════════════════════════
// plan002 U2 — Java 对齐缺口端点
//
// 写操作遵循 require_owner / is_admin 权限模式
// （docs/solutions/security-issues/idor-vulnerability-write-handlers.md）：
//   - 管理资源（admin/setting/cycle/workdayconfig/workplace/v2 group/shift/config）
//     写与删一律 is_admin 门禁；
//   - 个人资源（selfholiday/appeal/detail/leave）creator_person 取自会话、
//     不可信请求体，删除/更新前 require_owner 校验。
// ════════════════════════════════════════════════════════════════════

/// 管理类写操作门禁：非 admin 返回 Forbidden。
async fn require_admin(
    pool: &Pool,
    session: &shared::session::Session,
) -> Result<(), AppError> {
    if shared::middleware::is_admin(pool, &session.person_unique).await {
        Ok(())
    } else {
        Err(AppError::Forbidden)
    }
}

fn json_str(payload: &Value, key: &str) -> String {
    payload
        .get(key)
        .and_then(|v| v.as_str())
        .unwrap_or_default()
        .to_string()
}

fn json_opt_str(payload: &Value, key: &str) -> Option<String> {
    payload
        .get(key)
        .and_then(|v| v.as_str())
        .map(|s| s.to_string())
}

fn json_join(payload: &Value, key: &str) -> String {
    payload
        .get(key)
        .and_then(|v| v.as_array())
        .map(|a| {
            a.iter()
                .filter_map(|x| x.as_str())
                .collect::<Vec<_>>()
                .join(",")
        })
        .unwrap_or_default()
}

fn json_page(page: i64, size: i64) -> Result<(i64, i64), AppError> {
    if page < 1 || size < 1 || size > 500 {
        return Err(AppError::BadRequest("page must be >= 1 and size in 1..=500".to_string()));
    }
    Ok((size, (page - 1) * size))
}

/// 管理表按 id 删除：先 admin 门禁，再执行真实 DELETE。
async fn delete_admin_record(
    pool: &Pool,
    table: &'static str,
    id: &str,
    session: &shared::session::Session,
    label: &'static str,
) -> Result<Json<ActionResult<Value>>, AppError> {
    require_admin(pool, session).await?;

    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let n = client
        .execute(&format!("DELETE FROM {} WHERE id = $1", table), &[&id])
        .await
        .map_err(|_| AppError::Internal)?;

    if n == 0 {
        return Ok(Json(ActionResult::error(format!("{} not found", label))));
    }

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id.to_string())),
            ("deleted".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// 个人表按 id 删除：先取 creator_person 走 require_owner，再执行真实 DELETE。
async fn delete_owned_record(
    pool: &Pool,
    table: &'static str,
    id: &str,
    session: &shared::session::Session,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            &format!("SELECT creator_person FROM {} WHERE id = $1", table),
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let Some(row) = row else {
        return Ok(Json(ActionResult::error("record not found")));
    };

    let owner: String = row
        .get::<_, Option<String>>("creator_person")
        .unwrap_or_default();
    shared::middleware::require_owner(pool, session, &owner).await?;

    let n = client
        .execute(&format!("DELETE FROM {} WHERE id = $1", table), &[&id])
        .await
        .map_err(|_| AppError::Internal)?;

    if n == 0 {
        return Ok(Json(ActionResult::error("record not found")));
    }

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id.to_string())),
            ("deleted".to_string(), Value::Bool(true)),
        ]),
    ))))
}

// ── legacy POST 创建（9 个，全部真实 INSERT 现有表） ─────────────────

/// POST /jaxrs/attendance/assemble/control/attendanceadmin
pub async fn attendanceadmin_create(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    Json(payload): Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    require_admin(&pool, &session).await?;

    let admin_name = json_str(&payload, "adminName");
    let person_id = json_str(&payload, "personId");
    let unit_id = json_str(&payload, "unitId");
    let unit_name = json_str(&payload, "unitName");
    let admin_level = payload
        .get("adminLevel")
        .and_then(|v| v.as_str())
        .unwrap_or("admin")
        .to_string();

    if admin_name.is_empty() && person_id.is_empty() {
        return Err(AppError::BadRequest(
            "adminName or personId is required".to_string(),
        ));
    }

    let id = uuid::Uuid::new_v4().to_string();
    client
        .execute(
            "INSERT INTO x_attendance_admin (id, admin_name, admin_level, person_id, unit_id, unit_name, creator, creator_person, create_time, update_time) \
             VALUES ($1, $2, $3, $4, $5, $6, $7, $7, NOW(), NOW())",
            &[&id, &admin_name, &admin_level, &person_id, &unit_id, &unit_name, &session.person_unique],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
            ("adminName".to_string(), Value::String(admin_name)),
            ("personId".to_string(), Value::String(person_id)),
            ("created".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// POST /jaxrs/attendance/assemble/control/attendanceemployeeconfig
pub async fn attendanceemployeeconfig_create(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    Json(payload): Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    require_admin(&pool, &session).await?;

    let top_unit_name = json_opt_str(&payload, "topUnitName");
    let unit_name = json_opt_str(&payload, "unitName");
    let employee_name = json_str(&payload, "employeeName");
    let employee_number = json_opt_str(&payload, "employeeNumber");
    let config_type = json_opt_str(&payload, "configType");
    let person_id = json_opt_str(&payload, "personId");

    if employee_name.is_empty() {
        return Err(AppError::BadRequest("employeeName is required".to_string()));
    }

    let id = uuid::Uuid::new_v4().to_string();
    client
        .execute(
            "INSERT INTO x_attendance_employee_config (id, top_unit_name, unit_name, employee_name, employee_number, config_type, person_id, creator_person, create_time, update_time) \
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8, NOW(), NOW())",
            &[&id, &top_unit_name, &unit_name, &employee_name, &employee_number, &config_type, &person_id, &session.person_unique],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
            ("employeeName".to_string(), Value::String(employee_name)),
            ("created".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// POST /jaxrs/attendance/assemble/control/attendanceschedulesetting
pub async fn attendanceschedulesetting_create(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    Json(payload): Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    require_admin(&pool, &session).await?;

    let name = json_str(&payload, "name");
    let setting_data = match payload.get("settingData") {
        Some(v) => v.to_string(),
        None => "{}".to_string(),
    };
    let unit_id = json_opt_str(&payload, "unitId");

    if name.is_empty() {
        return Err(AppError::BadRequest("name is required".to_string()));
    }

    let id = uuid::Uuid::new_v4().to_string();
    client
        .execute(
            "INSERT INTO x_attendance_schedule_setting (id, name, setting_data, unit_id, creator_person, create_time, update_time) \
             VALUES ($1, $2, $3, $4, $5, NOW(), NOW())",
            &[&id, &name, &setting_data, &unit_id, &session.person_unique],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
            ("name".to_string(), Value::String(name)),
            ("created".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// POST /jaxrs/attendance/assemble/control/attendanceselfholiday
pub async fn attendanceselfholiday_create(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    Json(payload): Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let mut person_id = json_str(&payload, "personId");
    if person_id.is_empty() {
        person_id = session.person_unique.clone();
    }
    // 非本人提交需 admin 权限（IDOR：不允许替他人建记录）
    if person_id != session.person_unique {
        require_admin(&pool, &session).await?;
    }

    let holiday_date = json_str(&payload, "holidayDate");
    let reason = json_opt_str(&payload, "reason");

    if holiday_date.is_empty() {
        return Err(AppError::BadRequest("holidayDate is required".to_string()));
    }

    let id = uuid::Uuid::new_v4().to_string();
    client
        .execute(
            "INSERT INTO x_attendance_selfholiday (id, person_id, holiday_date, reason, creator_person, create_time, update_time) \
             VALUES ($1, $2, $3, $4, $5, NOW(), NOW())",
            &[&id, &person_id, &holiday_date, &reason, &session.person_unique],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
            ("personId".to_string(), Value::String(person_id)),
            ("holidayDate".to_string(), Value::String(holiday_date)),
            ("created".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// POST /jaxrs/attendance/assemble/control/attendancedetail
pub async fn attendancedetail_create(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    Json(payload): Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let mut person_id = json_str(&payload, "personId");
    if person_id.is_empty() {
        person_id = session.person_unique.clone();
    }
    if person_id != session.person_unique {
        require_admin(&pool, &session).await?;
    }

    let date = json_str(&payload, "date");
    let status = payload
        .get("status")
        .and_then(|v| v.as_str())
        .unwrap_or("normal")
        .to_string();
    let unit_id = json_opt_str(&payload, "unitId");

    if date.is_empty() {
        return Err(AppError::BadRequest("date is required".to_string()));
    }

    let id = uuid::Uuid::new_v4().to_string();
    client
        .execute(
            "INSERT INTO x_attendance_detail (id, person_id, date, status, unit_id, creator_person, create_time, update_time) \
             VALUES ($1, $2, $3, $4, $5, $6, NOW(), NOW())",
            &[&id, &person_id, &date, &status, &unit_id, &session.person_unique],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
            ("personId".to_string(), Value::String(person_id)),
            ("date".to_string(), Value::String(date)),
            ("created".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// POST /jaxrs/attendance/assemble/control/selfholidaysimple
pub async fn selfholidaysimple_create(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    Json(payload): Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let mut person_id = json_str(&payload, "personId");
    if person_id.is_empty() {
        person_id = session.person_unique.clone();
    }
    if person_id != session.person_unique {
        require_admin(&pool, &session).await?;
    }

    let holiday_date = json_str(&payload, "holidayDate");
    let reason = json_opt_str(&payload, "reason");

    if holiday_date.is_empty() {
        return Err(AppError::BadRequest("holidayDate is required".to_string()));
    }

    let id = uuid::Uuid::new_v4().to_string();
    client
        .execute(
            "INSERT INTO x_attendance_selfholiday (id, person_id, holiday_date, reason, creator_person, create_time, update_time) \
             VALUES ($1, $2, $3, $4, $5, NOW(), NOW())",
            &[&id, &person_id, &holiday_date, &reason, &session.person_unique],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id.clone())),
            ("docId".to_string(), Value::String(id)),
            ("personId".to_string(), Value::String(person_id)),
            ("holidayDate".to_string(), Value::String(holiday_date)),
            ("created".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// POST /jaxrs/attendance/assemble/control/attendancesetting
pub async fn attendancesetting_create(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    Json(payload): Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    require_admin(&pool, &session).await?;

    let code = json_str(&payload, "code");
    let name = json_opt_str(&payload, "name");
    let value = json_opt_str(&payload, "value");

    if code.is_empty() {
        return Err(AppError::BadRequest("code is required".to_string()));
    }

    let id = uuid::Uuid::new_v4().to_string();
    client
        .execute(
            "INSERT INTO x_attendance_setting (id, code, name, value, creator_person, create_time, update_time) \
             VALUES ($1, $2, $3, $4, $5, NOW(), NOW())",
            &[&id, &code, &name, &value, &session.person_unique],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
            ("code".to_string(), Value::String(code)),
            ("created".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// POST /jaxrs/attendance/assemble/control/attendancestatisticalcycle
pub async fn attendancestatisticalcycle_create(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    Json(payload): Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    require_admin(&pool, &session).await?;

    let top_unit_name = json_opt_str(&payload, "topUnitName");
    let unit_name = json_opt_str(&payload, "unitName");
    let cycle_year = json_opt_str(&payload, "cycleYear");
    let cycle_month = json_opt_str(&payload, "cycleMonth");

    if cycle_year.is_none() || cycle_month.is_none() {
        return Err(AppError::BadRequest(
            "cycleYear and cycleMonth are required".to_string(),
        ));
    }

    let description = json_opt_str(&payload, "description");
    let id = uuid::Uuid::new_v4().to_string();
    client
        .execute(
            "INSERT INTO x_attendance_statistical_cycle (id, top_unit_name, unit_name, cycle_year, cycle_month, description, creator_person, create_time, update_time) \
             VALUES ($1, $2, $3, $4, $5, $6, $7, NOW(), NOW())",
            &[&id, &top_unit_name, &unit_name, &cycle_year, &cycle_month, &description, &session.person_unique],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
            ("created".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// POST /jaxrs/attendance/assemble/control/attendancestatisticrequirelog
pub async fn attendancestatisticrequirelog_create(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    Json(payload): Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    require_admin(&pool, &session).await?;

    let require_type = json_str(&payload, "requireType");
    let status = payload
        .get("status")
        .and_then(|v| v.as_str())
        .unwrap_or("waiting")
        .to_string();

    if require_type.is_empty() {
        return Err(AppError::BadRequest("requireType is required".to_string()));
    }

    let id = uuid::Uuid::new_v4().to_string();
    client
        .execute(
            "INSERT INTO x_attendance_statistic_require_log (id, require_type, status, creator_person, create_time, update_time) \
             VALUES ($1, $2, $3, $4, NOW(), NOW())",
            &[&id, &require_type, &status, &session.person_unique],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
            ("requireType".to_string(), Value::String(require_type)),
            ("created".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// POST /jaxrs/attendance/assemble/control/attendanceworkdayconfig
pub async fn attendanceworkdayconfig_create(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    Json(payload): Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    require_admin(&pool, &session).await?;

    let work_date = json_str(&payload, "workDate");
    let is_workday = payload
        .get("isWorkday")
        .and_then(|v| v.as_bool())
        .unwrap_or(true);

    if work_date.is_empty() {
        return Err(AppError::BadRequest("workDate is required".to_string()));
    }

    let id = uuid::Uuid::new_v4().to_string();
    client
        .execute(
            "INSERT INTO x_attendance_workday_config (id, work_date, is_workday, creator_person, create_time, update_time) \
             VALUES ($1, $2, $3, $4, NOW(), NOW())",
            &[&id, &work_date, &is_workday, &session.person_unique],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
            ("workDate".to_string(), Value::String(work_date)),
            ("isWorkday".to_string(), Value::Bool(is_workday)),
            ("created".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// POST /jaxrs/attendance/assemble/control/workplace
pub async fn workplace_create(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    Json(payload): Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    require_admin(&pool, &session).await?;

    let name = json_str(&payload, "name");
    let address = json_opt_str(&payload, "address");

    if name.is_empty() {
        return Err(AppError::BadRequest("name is required".to_string()));
    }

    let id = uuid::Uuid::new_v4().to_string();
    client
        .execute(
            "INSERT INTO x_attendance_workplace (id, name, address, creator_person, create_time, update_time) \
             VALUES ($1, $2, $3, $4, NOW(), NOW())",
            &[&id, &name, &address, &session.person_unique],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
            ("name".to_string(), Value::String(name)),
            ("created".to_string(), Value::Bool(true)),
        ]),
    ))))
}

// ── legacy DELETE（13 个，链式注册到既有路径；真实 DELETE + 权限校验） ──

/// DELETE /jaxrs/attendance/assemble/control/attendanceadmin/{id}
pub async fn attendanceadmin_delete(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    Path(id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    delete_admin_record(&pool, "x_attendance_admin", &id, &session, "attendance admin").await
}

/// DELETE /jaxrs/attendance/assemble/control/attendanceappealInfo/{id}
pub async fn attendanceappealInfo_delete(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    Path(id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    delete_owned_record(&pool, "x_attendance_appeal_info", &id, &session).await
}

/// DELETE /jaxrs/attendance/assemble/control/attendancedetail/{id}
pub async fn attendancedetail_delete(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    Path(id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    delete_owned_record(&pool, "x_attendance_detail", &id, &session).await
}

/// DELETE /jaxrs/attendance/assemble/control/attendancedetail/mobile/{id}
pub async fn attendancedetail_mobile_delete(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    Path(id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    delete_owned_record(&pool, "x_attendance_detail", &id, &session).await
}

/// DELETE /jaxrs/attendance/assemble/control/attendanceemployeeconfig/{id}
pub async fn attendanceemployeeconfig_delete(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    Path(id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    delete_admin_record(&pool, "x_attendance_employee_config", &id, &session, "employee config").await
}

/// DELETE /jaxrs/attendance/assemble/control/attendanceimportfileinfo/{id}
pub async fn attendanceimportfileinfo_delete(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    Path(id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    delete_admin_record(&pool, "x_attendance_import_file_info", &id, &session, "import file info").await
}

/// DELETE /jaxrs/attendance/assemble/control/attendanceschedulesetting/{id}
pub async fn attendanceschedulesetting_delete(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    Path(id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    delete_admin_record(&pool, "x_attendance_schedule_setting", &id, &session, "schedule setting").await
}

/// DELETE /jaxrs/attendance/assemble/control/attendanceselfholiday/{id}
pub async fn attendanceselfholiday_delete(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    Path(id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    delete_owned_record(&pool, "x_attendance_selfholiday", &id, &session).await
}

/// DELETE /jaxrs/attendance/assemble/control/attendancesetting/{id}
pub async fn attendancesetting_delete(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    Path(id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    delete_admin_record(&pool, "x_attendance_setting", &id, &session, "attendance setting").await
}

/// DELETE /jaxrs/attendance/assemble/control/attendancestatisticalcycle/{id}
pub async fn attendancestatisticalcycle_delete(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    Path(id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    delete_admin_record(&pool, "x_attendance_statistical_cycle", &id, &session, "statistical cycle").await
}

/// DELETE /jaxrs/attendance/assemble/control/attendancestatisticrequirelog/{id}
pub async fn attendancestatisticrequirelog_delete(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    Path(id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    delete_admin_record(&pool, "x_attendance_statistic_require_log", &id, &session, "statistic require log").await
}

/// DELETE /jaxrs/attendance/assemble/control/attendanceworkdayconfig/{id}
pub async fn attendanceworkdayconfig_delete(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    Path(id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    delete_admin_record(&pool, "x_attendance_workday_config", &id, &session, "workday config").await
}

/// DELETE /jaxrs/attendance/assemble/control/workplace/{id}
pub async fn workplace_delete(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    Path(id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    delete_admin_record(&pool, "x_attendance_workplace", &id, &session, "workplace").await
}

/// DELETE /jaxrs/attendance/assemble/control/selfholidaysimple/docId/{docId}
pub async fn selfholidaysimple_docId_docId_delete(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    Path(doc_id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    delete_owned_record(&pool, "x_attendance_selfholiday", &doc_id, &session).await
}

// ── v2 group（6 个） ────────────────────────────────────────────────

/// GET /jaxrs/attendance/assemble/control/v2/group/{id}
pub async fn v2_group_get(
    pool: Extension<Pool>,
    Path(id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let row = client
        .query_opt(
            "SELECT id, group_name, check_type, top_unit, unit_list, shift_id, status, assist_admin_list, participate_list, work_place_id_list, work_date_list, start_date, end_date, operator, creator_person, create_time, update_time \
             FROM x_attendance_v2_group WHERE id = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let result = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("groupName".to_string(), Value::String(row.get::<_, Option<String>>("group_name").unwrap_or_default())),
                ("checkType".to_string(), Value::String(row.get::<_, Option<String>>("check_type").unwrap_or_default())),
                ("unitList".to_string(), Value::String(row.get::<_, Option<String>>("unit_list").unwrap_or_default())),
                ("shiftId".to_string(), Value::String(row.get::<_, Option<String>>("shift_id").unwrap_or_default())),
                ("status".to_string(), Value::Number(serde_json::Number::from(row.get::<_, i32>("status")))),
                ("participateList".to_string(), Value::String(row.get::<_, Option<String>>("participate_list").unwrap_or_default())),
                ("startDate".to_string(), Value::String(row.get::<_, Option<String>>("start_date").unwrap_or_default())),
                ("endDate".to_string(), Value::String(row.get::<_, Option<String>>("end_date").unwrap_or_default())),
            ]));
            Ok(Json(ActionResult::success(result)))
        }
        None => Ok(Json(ActionResult::error("attendance group not found"))),
    }
}

/// GET /jaxrs/attendance/assemble/control/v2/group/{id}/delete
pub async fn v2_group_delete(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    Path(id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    delete_owned_record(&pool, "x_attendance_v2_group", &id, &session).await
}

/// GET /jaxrs/attendance/assemble/control/v2/group/{id}/refresh/participate
pub async fn v2_group_refresh_participate(
    pool: Extension<Pool>,
    Path(id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let n = client
        .execute(
            "UPDATE x_attendance_v2_group SET update_time = NOW() WHERE id = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if n == 0 {
        return Ok(Json(ActionResult::error("attendance group not found")));
    }

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
            ("participateRefreshed".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// GET /jaxrs/attendance/assemble/control/v2/group/person/{person}/date/{date}
pub async fn v2_group_person_date(
    pool: Extension<Pool>,
    Path((person, date)): Path<(String, String)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query(
            "SELECT id, group_name, check_type, shift_id, participate_list, start_date, end_date \
             FROM x_attendance_v2_group \
             WHERE (',' || COALESCE(participate_list, '') || ',') LIKE ('%,' || $1 || ',%') \
               AND (start_date IS NULL OR start_date = '' OR start_date <= $2) \
               AND (end_date IS NULL OR end_date = '' OR end_date >= $2) \
             ORDER BY create_time DESC",
            &[&person, &date],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("groupName".to_string(), Value::String(row.get::<_, Option<String>>("group_name").unwrap_or_default())),
                ("checkType".to_string(), Value::String(row.get::<_, Option<String>>("check_type").unwrap_or_default())),
                ("shiftId".to_string(), Value::String(row.get::<_, Option<String>>("shift_id").unwrap_or_default())),
                ("participateList".to_string(), Value::String(row.get::<_, Option<String>>("participate_list").unwrap_or_default())),
            ]))
        })
        .collect();

    let count = data.len() as i64;
    Ok(Json(ActionResult::java_success(
        Value::Array(data),
        count,
        0,
    )))
}

/// POST /jaxrs/attendance/assemble/control/v2/group
pub async fn v2_group_create(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    Json(payload): Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    require_admin(&pool, &session).await?;

    let group_name = json_str(&payload, "groupName");
    if group_name.is_empty() {
        return Err(AppError::BadRequest("groupName is required".to_string()));
    }

    let check_type = json_str(&payload, "checkType");
    let top_unit = json_opt_str(&payload, "topUnit");
    let unit_list = json_join(&payload, "unitList");
    let shift_id = json_opt_str(&payload, "shiftId");
    let participate_list = json_join(&payload, "participateList");
    let assist_admin_list = json_join(&payload, "assistAdminList");
    let work_place_id_list = json_join(&payload, "workPlaceIdList");
    let work_date_list = json_str(&payload, "workDateList");
    let start_date = json_opt_str(&payload, "startDate");
    let end_date = json_opt_str(&payload, "endDate");
    let status = payload.get("status").and_then(|v| v.as_i64()).unwrap_or(1) as i32;

    let id = uuid::Uuid::new_v4().to_string();
    client
        .execute(
            "INSERT INTO x_attendance_v2_group (id, group_name, check_type, top_unit, unit_list, shift_id, status, assist_admin_list, participate_list, work_place_id_list, work_date_list, start_date, end_date, operator, creator_person, create_time, update_time) \
             VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11,$12,$13,$14,$15,NOW(),NOW())",
            &[&id, &group_name, &check_type, &top_unit, &unit_list, &shift_id, &status, &assist_admin_list, &participate_list, &work_place_id_list, &work_date_list, &start_date, &end_date, &session.person_unique, &session.person_unique],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
            ("groupName".to_string(), Value::String(group_name)),
            ("created".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// POST /jaxrs/attendance/assemble/control/v2/group/list/{page}/size/{size}
pub async fn v2_group_list_page_size(
    pool: Extension<Pool>,
    Path((page, size)): Path<(i64, i64)>,
    body: Option<Json<Value>>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let (limit, offset) = json_page(page, size)?;
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let filter_name = body
        .as_ref()
        .map(|Json(b)| b.get("name").and_then(|v| v.as_str()).unwrap_or("").to_string())
        .unwrap_or_default();

    let d = dialect();
    let total: i64 = client
        .query_one(
            "SELECT COUNT(*) FROM x_attendance_v2_group WHERE ($1 = '' OR group_name ILIKE ('%' || $1 || '%'))",
            &[&filter_name],
        )
        .await
        .map_err(|_| AppError::Internal)?
        .get(0);

    let sql = format!(
        "SELECT id, group_name, check_type, unit_list, shift_id, status, participate_list, start_date, end_date, create_time \
         FROM x_attendance_v2_group WHERE ($1 = '' OR group_name ILIKE ('%' || $1 || '%')) \
         ORDER BY create_time DESC LIMIT {} OFFSET {}",
        d.cast_bigint_param(2),
        d.cast_bigint_param(3),
    );
    let rows = client
        .query(&sql, &[&filter_name, &limit, &offset])
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("groupName".to_string(), Value::String(row.get::<_, Option<String>>("group_name").unwrap_or_default())),
                ("checkType".to_string(), Value::String(row.get::<_, Option<String>>("check_type").unwrap_or_default())),
                ("unitList".to_string(), Value::String(row.get::<_, Option<String>>("unit_list").unwrap_or_default())),
                ("shiftId".to_string(), Value::String(row.get::<_, Option<String>>("shift_id").unwrap_or_default())),
                ("status".to_string(), Value::Number(serde_json::Number::from(row.get::<_, i32>("status")))),
                ("createTime".to_string(), Value::String(row.get::<_, Option<String>>("create_time").unwrap_or_default())),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(total))),
            ("data".to_string(), Value::Array(data)),
        ]),
    ))))
}

// ── v2 shift（5 个） ───────────────────────────────────────────────

/// GET /jaxrs/attendance/assemble/control/v2/shift/{id}
pub async fn v2_shift_get(
    pool: Extension<Pool>,
    Path(id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let row = client
        .query_opt(
            "SELECT id, shift_name, on_duty_time, off_duty_time, work_time, serial_no, properties_json, operator, create_time, update_time \
             FROM x_attendance_v2_shift WHERE id = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let result = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("shiftName".to_string(), Value::String(row.get::<_, Option<String>>("shift_name").unwrap_or_default())),
                ("onDutyTime".to_string(), Value::String(row.get::<_, Option<String>>("on_duty_time").unwrap_or_default())),
                ("offDutyTime".to_string(), Value::String(row.get::<_, Option<String>>("off_duty_time").unwrap_or_default())),
                ("workTime".to_string(), Value::Number(serde_json::Number::from(row.get::<_, Option<i32>>("work_time").unwrap_or(0)))),
                ("serialNo".to_string(), Value::Number(serde_json::Number::from(row.get::<_, i64>("serial_no")))),
            ]));
            Ok(Json(ActionResult::success(result)))
        }
        None => Ok(Json(ActionResult::error("shift not found"))),
    }
}

/// GET /jaxrs/attendance/assemble/control/v2/shift/delete/{id}
pub async fn v2_shift_delete(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    Path(id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    delete_owned_record(&pool, "x_attendance_v2_shift", &id, &session).await
}

/// POST /jaxrs/attendance/assemble/control/v2/shift/create
pub async fn v2_shift_create(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    Json(payload): Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    require_admin(&pool, &session).await?;

    let shift_name = json_str(&payload, "shiftName");
    if shift_name.is_empty() {
        return Err(AppError::BadRequest("shiftName is required".to_string()));
    }

    let on_duty_time = json_str(&payload, "onDutyTime");
    let off_duty_time = json_str(&payload, "offDutyTime");
    let work_time = payload.get("workTime").and_then(|v| v.as_i64()).unwrap_or(0) as i32;
    let properties_json = match payload.get("properties") {
        Some(v) => v.to_string(),
        None => "{}".to_string(),
    };
    let serial_no: i64 = client
        .query_one("SELECT COALESCE(MAX(serial_no), 0) + 1 AS next FROM x_attendance_v2_shift", &[])
        .await
        .map_err(|_| AppError::Internal)?
        .get(0);

    let id = uuid::Uuid::new_v4().to_string();
    client
        .execute(
            "INSERT INTO x_attendance_v2_shift (id, shift_name, on_duty_time, off_duty_time, work_time, serial_no, properties_json, operator, creator_person, create_time, update_time) \
             VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9,NOW(),NOW())",
            &[&id, &shift_name, &on_duty_time, &off_duty_time, &work_time, &serial_no, &properties_json, &session.person_unique, &session.person_unique],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
            ("shiftName".to_string(), Value::String(shift_name)),
            ("serialNo".to_string(), Value::Number(serde_json::Number::from(serial_no))),
            ("created".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// POST /jaxrs/attendance/assemble/control/v2/shift/list/{page}/size/{size}
pub async fn v2_shift_list_page_size(
    pool: Extension<Pool>,
    Path((page, size)): Path<(i64, i64)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let (limit, offset) = json_page(page, size)?;
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let d = dialect();
    let total: i64 = client
        .query_one("SELECT COUNT(*) FROM x_attendance_v2_shift", &[])
        .await
        .map_err(|_| AppError::Internal)?
        .get(0);

    let sql = format!(
        "SELECT id, shift_name, on_duty_time, off_duty_time, work_time, serial_no \
         FROM x_attendance_v2_shift ORDER BY serial_no ASC LIMIT {} OFFSET {}",
        d.cast_bigint_param(1),
        d.cast_bigint_param(2),
    );
    let rows = client
        .query(&sql, &[&limit, &offset])
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("shiftName".to_string(), Value::String(row.get::<_, Option<String>>("shift_name").unwrap_or_default())),
                ("onDutyTime".to_string(), Value::String(row.get::<_, Option<String>>("on_duty_time").unwrap_or_default())),
                ("offDutyTime".to_string(), Value::String(row.get::<_, Option<String>>("off_duty_time").unwrap_or_default())),
                ("workTime".to_string(), Value::Number(serde_json::Number::from(row.get::<_, Option<i32>>("work_time").unwrap_or(0)))),
                ("serialNo".to_string(), Value::Number(serde_json::Number::from(row.get::<_, i64>("serial_no")))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(total))),
            ("data".to_string(), Value::Array(data)),
        ]),
    ))))
}

/// POST /jaxrs/attendance/assemble/control/v2/shift/update
pub async fn v2_shift_update(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    Json(payload): Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let id = json_str(&payload, "id");
    if id.is_empty() {
        return Err(AppError::BadRequest("id is required".to_string()));
    }

    let row = client
        .query_opt(
            "SELECT shift_name, on_duty_time, off_duty_time, work_time, creator_person FROM x_attendance_v2_shift WHERE id = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let Some(row) = row else {
        return Ok(Json(ActionResult::error("shift not found")));
    };

    let owner: String = row.get::<_, Option<String>>("creator_person").unwrap_or_default();
    shared::middleware::require_owner(&pool, &session, &owner).await?;

    let shift_name = payload
        .get("shiftName")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string())
        .unwrap_or_else(|| row.get::<_, Option<String>>("shift_name").unwrap_or_default());
    let on_duty_time = payload
        .get("onDutyTime")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string())
        .unwrap_or_else(|| row.get::<_, Option<String>>("on_duty_time").unwrap_or_default());
    let off_duty_time = payload
        .get("offDutyTime")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string())
        .unwrap_or_else(|| row.get::<_, Option<String>>("off_duty_time").unwrap_or_default());
    let work_time = payload
        .get("workTime")
        .and_then(|v| v.as_i64())
        .map(|v| v as i32)
        .unwrap_or_else(|| row.get::<_, Option<i32>>("work_time").unwrap_or(0));

    client
        .execute(
            "UPDATE x_attendance_v2_shift SET shift_name = $2, on_duty_time = $3, off_duty_time = $4, work_time = $5, update_time = NOW() WHERE id = $1",
            &[&id, &shift_name, &on_duty_time, &off_duty_time, &work_time],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
            ("shiftName".to_string(), Value::String(shift_name)),
            ("updated".to_string(), Value::Bool(true)),
        ]),
    ))))
}

// ── v2 leave（5 个） ───────────────────────────────────────────────

/// GET /jaxrs/attendance/assemble/control/v2/leave/delete/{id}
pub async fn v2_leave_delete(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    Path(id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    delete_owned_record(&pool, "x_attendance_v2_leave", &id, &session).await
}

/// GET /jaxrs/attendance/assemble/control/v2/leave/import/result/flag/{flag}
pub async fn v2_leave_import_result_flag(
    pool: Extension<Pool>,
    Path(flag): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let total: i64 = client
        .query_one(
            "SELECT COUNT(*) FROM x_attendance_v2_leave WHERE batch_flag = $1",
            &[&flag],
        )
        .await
        .map_err(|_| AppError::Internal)?
        .get(0);

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("flag".to_string(), Value::String(flag)),
            ("total".to_string(), Value::Number(serde_json::Number::from(total))),
        ]),
    ))))
}

/// POST /jaxrs/attendance/assemble/control/v2/leave
pub async fn v2_leave_create(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    Json(payload): Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let mut person = json_str(&payload, "person");
    if person.is_empty() {
        person = session.person_unique.clone();
    }
    // 非本人请假需 admin（IDOR）
    if person != session.person_unique {
        require_admin(&pool, &session).await?;
    }

    let leave_type = json_str(&payload, "leaveType");
    if leave_type.is_empty() {
        return Err(AppError::BadRequest("leaveType is required".to_string()));
    }

    let start_time = json_str(&payload, "startTime");
    let end_time = json_str(&payload, "endTime");
    let description = json_opt_str(&payload, "description");
    let job_id = json_opt_str(&payload, "jobId");

    let id = uuid::Uuid::new_v4().to_string();
    client
        .execute(
            "INSERT INTO x_attendance_v2_leave (id, person, leave_type, start_time, end_time, description, job_id, batch_flag, creator_person, create_time, update_time) \
             VALUES ($1,$2,$3,$4,$5,$6,$7,NULL,$8,NOW(),NOW())",
            &[&id, &person, &leave_type, &start_time, &end_time, &description, &job_id, &session.person_unique],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
            ("person".to_string(), Value::String(person)),
            ("leaveType".to_string(), Value::String(leave_type)),
            ("created".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// POST /jaxrs/attendance/assemble/control/v2/leave/list/{page}/size/{size}
pub async fn v2_leave_list_page_size(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    Path((page, size)): Path<(i64, i64)>,
    body: Option<Json<Value>>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let (limit, offset) = json_page(page, size)?;
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    // 非 admin 只能查看本人请假记录（IDOR 读过滤）
    let admin = shared::middleware::is_admin(&pool, &session.person_unique).await;
    let filter_person = if admin {
        body.as_ref()
            .map(|Json(b)| b.get("person").and_then(|v| v.as_str()).unwrap_or("").to_string())
            .unwrap_or_default()
    } else {
        session.person_unique.clone()
    };

    let d = dialect();
    let total: i64 = client
        .query_one(
            "SELECT COUNT(*) FROM x_attendance_v2_leave WHERE ($1 = '' OR person = $1)",
            &[&filter_person],
        )
        .await
        .map_err(|_| AppError::Internal)?
        .get(0);

    let sql = format!(
        "SELECT id, person, leave_type, start_time, end_time, batch_flag, creator_person, create_time \
         FROM x_attendance_v2_leave WHERE ($1 = '' OR person = $1) \
         ORDER BY create_time DESC LIMIT {} OFFSET {}",
        d.cast_bigint_param(2),
        d.cast_bigint_param(3),
    );
    let rows = client
        .query(&sql, &[&filter_person, &limit, &offset])
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("person".to_string(), Value::String(row.get::<_, Option<String>>("person").unwrap_or_default())),
                ("leaveType".to_string(), Value::String(row.get::<_, Option<String>>("leave_type").unwrap_or_default())),
                ("startTime".to_string(), Value::String(row.get::<_, Option<String>>("start_time").unwrap_or_default())),
                ("endTime".to_string(), Value::String(row.get::<_, Option<String>>("end_time").unwrap_or_default())),
                ("batchFlag".to_string(), Value::String(row.get::<_, Option<String>>("batch_flag").unwrap_or_default())),
                ("createTime".to_string(), Value::String(row.get::<_, Option<String>>("create_time").unwrap_or_default())),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(total))),
            ("data".to_string(), Value::Array(data)),
        ]),
    ))))
}

/// POST /jaxrs/attendance/assemble/control/v2/leave/import
pub async fn v2_leave_import(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    Json(payload): Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let flag = payload
        .get("flag")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string())
        .unwrap_or_else(|| uuid::Uuid::new_v4().to_string());

    let rows = payload
        .get("rows")
        .and_then(|v| v.as_array())
        .ok_or_else(|| AppError::BadRequest("rows array is required".to_string()))?;

    if rows.is_empty() {
        return Err(AppError::BadRequest("rows must not be empty".to_string()));
    }

    let admin = shared::middleware::is_admin(&pool, &session.person_unique).await;
    let mut inserted: i64 = 0;

    for item in rows {
        let mut person = item.get("person").and_then(|v| v.as_str()).unwrap_or("").to_string();
        if person.is_empty() {
            person = session.person_unique.clone();
        }
        if !admin && person != session.person_unique {
            return Err(AppError::Forbidden);
        }
        let leave_type = item.get("leaveType").and_then(|v| v.as_str()).unwrap_or("").to_string();
        if leave_type.is_empty() {
            return Err(AppError::BadRequest("each row requires leaveType".to_string()));
        }
        let start_time = item.get("startTime").and_then(|v| v.as_str()).unwrap_or("").to_string();
        let end_time = item.get("endTime").and_then(|v| v.as_str()).unwrap_or("").to_string();
        let description = item.get("description").and_then(|v| v.as_str()).map(|s| s.to_string());
        let job_id: Option<String> = None;
        let id = uuid::Uuid::new_v4().to_string();

        let n = client
            .execute(
                "INSERT INTO x_attendance_v2_leave (id, person, leave_type, start_time, end_time, description, job_id, batch_flag, creator_person, create_time, update_time) \
                 VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9,NOW(),NOW())",
                &[&id, &person, &leave_type, &start_time, &end_time, &description, &job_id, &flag, &session.person_unique],
            )
            .await
            .map_err(|_| AppError::Internal)?;
        inserted += n as i64;
    }

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("flag".to_string(), Value::String(flag)),
            ("inserted".to_string(), Value::Number(serde_json::Number::from(inserted))),
        ]),
    ))))
}

// ── v2 config（4 个，复用 x_attendance_config 分类存储） ────────────

/// GET /jaxrs/attendance/assemble/control/v2/config
pub async fn v2_config_get(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query(
            "SELECT id, name, value, creator, create_time FROM x_attendance_config WHERE category = 'v2' ORDER BY name",
            &[],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let mut result = serde_json::Map::new();
    for row in &rows {
        let name: String = row.get("name");
        let value: Option<String> = row.get("value");
        result.insert(name, Value::String(value.unwrap_or_default()));
    }

    Ok(Json(ActionResult::success(Value::Object(result))))
}

/// POST /jaxrs/attendance/assemble/control/v2/config
pub async fn v2_config_post(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    Json(payload): Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    require_admin(&pool, &session).await?;

    let name = json_str(&payload, "name");
    let value = json_opt_str(&payload, "value");

    if name.is_empty() {
        return Err(AppError::BadRequest("name is required".to_string()));
    }

    let existing = client
        .query_opt(
            "SELECT id FROM x_attendance_config WHERE category = 'v2' AND name = $1",
            &[&name],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let id = match existing {
        Some(row) => {
            let id: String = row.get("id");
            client
                .execute(
                    "UPDATE x_attendance_config SET value = $2, update_time = NOW() WHERE id = $1",
                    &[&id, &value],
                )
                .await
                .map_err(|_| AppError::Internal)?;
            id
        }
        None => {
            let id = uuid::Uuid::new_v4().to_string();
            client
                .execute(
                    "INSERT INTO x_attendance_config (id, name, value, category, creator, create_time, update_time) \
                     VALUES ($1, $2, $3, 'v2', $4, NOW(), NOW())",
                    &[&id, &name, &value, &session.person_unique],
                )
                .await
                .map_err(|_| AppError::Internal)?;
            id
        }
    };

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
            ("name".to_string(), Value::String(name)),
            ("saved".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// GET /jaxrs/attendance/assemble/control/v2/config/person
pub async fn v2_config_person_get(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query(
            "SELECT id, name, value, create_time FROM x_attendance_config WHERE category = 'v2_person' AND creator = $1 ORDER BY name",
            &[&session.person_unique],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let mut result = serde_json::Map::new();
    for row in &rows {
        let name: String = row.get("name");
        let value: Option<String> = row.get("value");
        result.insert(name, Value::String(value.unwrap_or_default()));
    }

    Ok(Json(ActionResult::success(Value::Object(result))))
}

/// POST /jaxrs/attendance/assemble/control/v2/config/person
pub async fn v2_config_person_post(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    Json(payload): Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let name = json_str(&payload, "name");
    let value = json_opt_str(&payload, "value");

    if name.is_empty() {
        return Err(AppError::BadRequest("name is required".to_string()));
    }

    let existing = client
        .query_opt(
            "SELECT id FROM x_attendance_config WHERE category = 'v2_person' AND name = $1 AND creator = $2",
            &[&name, &session.person_unique],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let id = match existing {
        Some(row) => {
            let id: String = row.get("id");
            client
                .execute(
                    "UPDATE x_attendance_config SET value = $2, update_time = NOW() WHERE id = $1",
                    &[&id, &value],
                )
                .await
                .map_err(|_| AppError::Internal)?;
            id
        }
        None => {
            let id = uuid::Uuid::new_v4().to_string();
            client
                .execute(
                    "INSERT INTO x_attendance_config (id, name, value, category, creator, create_time, update_time) \
                     VALUES ($1, $2, $3, 'v2_person', $4, NOW(), NOW())",
                    &[&id, &name, &value, &session.person_unique],
                )
                .await
                .map_err(|_| AppError::Internal)?;
            id
        }
    };

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
            ("name".to_string(), Value::String(name)),
            ("saved".to_string(), Value::Bool(true)),
        ]),
    ))))
}

// ── v2 record / detail / my（4 个） ────────────────────────────────

/// GET /jaxrs/attendance/assemble/control/v2/record/{id}
pub async fn v2_record_get(
    pool: Extension<Pool>,
    Path(id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let row = client
        .query_opt(
            "SELECT id, user_id, check_in_time, check_out_time, status, create_time FROM x_attendance_record WHERE id = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let result = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("userId".to_string(), Value::String(row.get("user_id"))),
                ("checkInTime".to_string(), Value::String(row.get("check_in_time"))),
                ("checkOutTime".to_string(), Value::String(row.get::<_, Option<String>>("check_out_time").unwrap_or_default())),
                ("status".to_string(), Value::String(row.get("status"))),
                ("createTime".to_string(), Value::String(row.get::<_, Option<String>>("create_time").unwrap_or_default())),
            ]));
            Ok(Json(ActionResult::success(result)))
        }
        None => Ok(Json(ActionResult::error("record not found"))),
    }
}

/// POST /jaxrs/attendance/assemble/control/v2/record/list/{page}/size/{size}
pub async fn v2_record_list_page_size(
    pool: Extension<Pool>,
    Path((page, size)): Path<(i64, i64)>,
    body: Option<Json<Value>>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let (limit, offset) = json_page(page, size)?;
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let filter_user = body
        .as_ref()
        .map(|Json(b)| b.get("userId").and_then(|v| v.as_str()).unwrap_or("").to_string())
        .unwrap_or_default();

    let d = dialect();
    let total: i64 = client
        .query_one(
            "SELECT COUNT(*) FROM x_attendance_record WHERE ($1 = '' OR user_id = $1)",
            &[&filter_user],
        )
        .await
        .map_err(|_| AppError::Internal)?
        .get(0);

    let sql = format!(
        "SELECT id, user_id, check_in_time, check_out_time, status, create_time \
         FROM x_attendance_record WHERE ($1 = '' OR user_id = $1) \
         ORDER BY create_time DESC LIMIT {} OFFSET {}",
        d.cast_bigint_param(2),
        d.cast_bigint_param(3),
    );
    let rows = client
        .query(&sql, &[&filter_user, &limit, &offset])
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("userId".to_string(), Value::String(row.get("user_id"))),
                ("checkInTime".to_string(), Value::String(row.get("check_in_time"))),
                ("checkOutTime".to_string(), Value::String(row.get::<_, Option<String>>("check_out_time").unwrap_or_default())),
                ("status".to_string(), Value::String(row.get("status"))),
                ("createTime".to_string(), Value::String(row.get::<_, Option<String>>("create_time").unwrap_or_default())),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(total))),
            ("data".to_string(), Value::Array(data)),
        ]),
    ))))
}

/// POST /jaxrs/attendance/assemble/control/v2/detail/list/{page}/size/{size}
pub async fn v2_detail_list_page_size(
    pool: Extension<Pool>,
    Path((page, size)): Path<(i64, i64)>,
    body: Option<Json<Value>>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let (limit, offset) = json_page(page, size)?;
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let empty = String::new();
    let filter_person = body
        .as_ref()
        .map(|Json(b)| b.get("personId").and_then(|v| v.as_str()).unwrap_or("").to_string())
        .unwrap_or(empty.clone());
    let filter_status = body
        .as_ref()
        .map(|Json(b)| b.get("status").and_then(|v| v.as_str()).unwrap_or("").to_string())
        .unwrap_or(empty);

    let d = dialect();
    let total: i64 = client
        .query_one(
            "SELECT COUNT(*) FROM x_attendance_detail WHERE ($1 = '' OR person_id = $1) AND ($2 = '' OR status = $2)",
            &[&filter_person, &filter_status],
        )
        .await
        .map_err(|_| AppError::Internal)?
        .get(0);

    let sql = format!(
        "SELECT id, person_id, date, status, unit_id, file_id, create_time \
         FROM x_attendance_detail WHERE ($1 = '' OR person_id = $1) AND ($2 = '' OR status = $2) \
         ORDER BY create_time DESC LIMIT {} OFFSET {}",
        d.cast_bigint_param(3),
        d.cast_bigint_param(4),
    );
    let rows = client
        .query(&sql, &[&filter_person, &filter_status, &limit, &offset])
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("personId".to_string(), Value::String(row.get::<_, Option<String>>("person_id").unwrap_or_default())),
                ("date".to_string(), Value::String(row.get::<_, Option<String>>("date").unwrap_or_default())),
                ("status".to_string(), Value::String(row.get::<_, Option<String>>("status").unwrap_or_default())),
                ("fileId".to_string(), Value::String(row.get::<_, Option<String>>("file_id").unwrap_or_default())),
                ("createTime".to_string(), Value::String(row.get::<_, Option<String>>("create_time").unwrap_or_default())),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(total))),
            ("data".to_string(), Value::Array(data)),
        ]),
    ))))
}

/// POST /jaxrs/attendance/assemble/control/v2/my/statistic
pub async fn v2_my_statistic(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    body: Option<Json<Value>>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    // 统计月份：body 可传 year/month，缺省取当前月
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);
    let default_month = crate::month_prefix_from_unix(now);
    let month = body
        .as_ref()
        .map(|Json(b)| {
            let year = b.get("year").and_then(|v| v.as_str()).unwrap_or("");
            let month = b.get("month").and_then(|v| v.as_str()).unwrap_or("");
            if year.is_empty() || month.is_empty() {
                String::new()
            } else {
                format!("{}-{}", year, month)
            }
        })
        .filter(|s| !s.is_empty())
        .unwrap_or(default_month);

    let pattern = format!("{}%", month);
    let rows = client
        .query(
            "SELECT status, COUNT(*) AS cnt FROM x_attendance_detail WHERE person_id = $1 AND date LIKE $2 GROUP BY status ORDER BY status",
            &[&session.person_unique, &pattern],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let by_status: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("status".to_string(), Value::String(row.get::<_, Option<String>>("status").unwrap_or_default())),
                ("count".to_string(), Value::Number(serde_json::Number::from(row.get::<_, i64>("cnt")))),
            ]))
        })
        .collect();

    let record_count: i64 = client
        .query_one(
            "SELECT COUNT(*) FROM x_attendance_record WHERE user_id = $1 AND create_time LIKE $2",
            &[&session.person_unique, &pattern],
        )
        .await
        .map_err(|_| AppError::Internal)?
        .get(0);

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("person".to_string(), Value::String(session.person_unique.clone())),
            ("month".to_string(), Value::String(month)),
            ("detailByStatus".to_string(), Value::Array(by_status)),
            ("recordCount".to_string(), Value::Number(serde_json::Number::from(record_count))),
        ]),
    ))))
}

/// 由 Unix 秒计算 "YYYY-MM" 前缀（my/statistic 缺省月份用，避免引 chrono）。
pub(crate) fn month_prefix_from_unix(secs: u64) -> String {
    // days since epoch → civil date (Howard Hinnant algorithm)
    let days = secs / 86_400;
    let z = days as i64 + 719_468;
    let era = z.div_euclid(146_097);
    let doe = z.rem_euclid(146_097);
    let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365;
    let y = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let m = if mp < 10 { mp + 3 } else { mp - 9 };
    let year = if m <= 2 { y + 1 } else { y };
    format!("{:04}-{:02}", year, m)
}

/// 由 Unix 秒计算 "YYYY-MM-DD"（v2 mobile/record 缺省日期用，同一历法算法）。
pub(crate) fn date_string_from_unix(secs: u64) -> String {
    let days = secs / 86_400;
    let z = days as i64 + 719_468;
    let era = z.div_euclid(146_097);
    let doe = z.rem_euclid(146_097);
    let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365;
    let y = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let m = if mp < 10 { mp + 3 } else { mp - 9 };
    let d = doy - (153 * mp + 2) / 5 + 1;
    let year = if m <= 2 { y + 1 } else { y };
    format!("{:04}-{:02}-{:02}", year, m, d)
}

// ════════════════════════════════════════════════════════════════════
// plan002 U2 — legacy 族闭合（dingding/qywx + v2 剩余端点，共 49 个）
//
// 对齐 docs/solutions/security-issues/idor-vulnerability-write-handlers.md：
//   - 管理资源（dingding/qywx 数据清理、统计重算、v2 workplace、排班配置、
//     appeal 管理员操作）一律 require_admin 门禁；
//   - 个人资源（appeal 流转、record 删除、mobile 打卡）require_owner 或
//     会话强制本人；
// 归一化查重：groupschedule 按 (group,user,date) 幂等去重；workplace 按
// name 归一化（trim+lower）查重；mobile check 同人同日同类型拒绝重复打卡。
// ════════════════════════════════════════════════════════════════════

pub(crate) fn normalize_key(s: &str) -> String {
    s.trim().to_lowercase()
}

// ── dingding / qywx 族（16 个端点共享的私有实现） ───────────────────

async fn ddqy_delete_all(
    pool: &Pool,
    detail_table: &str,
    sync_type: &str,
    session: &shared::session::Session,
) -> Result<Json<ActionResult<Value>>, AppError> {
    require_admin(pool, session).await?;
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let n_detail = client
        .execute(
            &format!("DELETE FROM {} WHERE 1 = 1", detail_table),
            &[],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    let n_sync = client
        .execute("DELETE FROM x_attendance_sync_record WHERE type = $1", &[&sync_type])
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("detailDeleted".to_string(), Value::Number(serde_json::Number::from(n_detail as i64))),
        ("syncRecordDeleted".to_string(), Value::Number(serde_json::Number::from(n_sync as i64))),
    ])))))
}

async fn ddqy_sync_start(
    pool: &Pool,
    sync_type: &str,
    from: &str,
    to: &str,
) -> Result<Json<ActionResult<Value>>, AppError> {
    if from.trim().is_empty() || to.trim().is_empty() {
        return Err(AppError::BadRequest("dateFrom/dateTo is required".to_string()));
    }
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let id = uuid::Uuid::new_v4().to_string();
    let status = "成功";
    client
        .execute(
            "INSERT INTO x_attendance_sync_record (id, type, status, exception_message, start_date, end_date, create_time, update_time) \
             VALUES ($1, $2, $3, NULL, $4, $5, NOW(), NOW())",
            &[&id, &sync_type, &status, &from, &to],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("value".to_string(), Value::Bool(true)),
        ("syncId".to_string(), Value::String(id)),
    ])))))
}

async fn ddqy_sync_list(
    pool: &Pool,
    sync_type: &str,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query(
            "SELECT id, type, status, exception_message, start_date, end_date, create_time \
             FROM x_attendance_sync_record WHERE type = $1 ORDER BY create_time DESC",
            &[&sync_type],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("type".to_string(), Value::String(row.get::<_, Option<String>>("type").unwrap_or_default())),
                ("status".to_string(), Value::String(row.get::<_, Option<String>>("status").unwrap_or_default())),
                (
                    "exceptionMessage".to_string(),
                    Value::String(row.get::<_, Option<String>>("exception_message").unwrap_or_default()),
                ),
                (
                    "startDate".to_string(),
                    Value::String(row.get::<_, Option<String>>("start_date").unwrap_or_default()),
                ),
                (
                    "endDate".to_string(),
                    Value::String(row.get::<_, Option<String>>("end_date").unwrap_or_default()),
                ),
                (
                    "createTime".to_string(),
                    Value::String(row.get::<_, Option<String>>("create_time").unwrap_or_default()),
                ),
            ]))
        })
        .collect();

    let count = data.len() as i64;
    Ok(Json(ActionResult::java_success(
        Value::Array(data),
        count,
        0,
    )))
}

async fn ddqy_attendance_list_next(
    pool: &Pool,
    detail_table: &str,
    id: &str,
    count: i64,
    body: Option<Json<Value>>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    if count < 1 || count > 500 {
        return Err(AppError::BadRequest("count must be in 1..=500".to_string()));
    }
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let filter_user = body
        .as_ref()
        .map(|Json(b)| b.get("user").and_then(|v| v.as_str()).unwrap_or("").trim().to_string())
        .unwrap_or_default();

    // id 为空串表示从头开始（Java ActionListDDAttendanceDetail 语义）
    let sql = format!(
        "SELECT id, user_id, time, checkin_type, location_result, source_type \
         FROM {tbl} \
         WHERE ($2 = '' OR user_id = $2) AND ($3 = '' OR id > $3) \
         ORDER BY time ASC, id ASC LIMIT {}",
        dialect().cast_bigint_param(4),
        tbl = detail_table,
    );
    let rows = client
        .query(&sql, &[&filter_user, &filter_user, &id, &count])
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("user".to_string(), Value::String(row.get::<_, Option<String>>("user_id").unwrap_or_default())),
                ("time".to_string(), Value::String(row.get::<_, Option<String>>("time").unwrap_or_default())),
                (
                    "checkinType".to_string(),
                    Value::String(row.get::<_, Option<String>>("checkin_type").unwrap_or_default()),
                ),
                (
                    "locationResult".to_string(),
                    Value::String(row.get::<_, Option<String>>("location_result").unwrap_or_default()),
                ),
                (
                    "sourceType".to_string(),
                    Value::String(row.get::<_, Option<String>>("source_type").unwrap_or_default()),
                ),
            ]))
        })
        .collect();

    let count = data.len() as i64;
    Ok(Json(ActionResult::java_success(
        Value::Array(data),
        count,
        0,
    )))
}

/// 统计重算：从明细表按人聚合刷新 person_month 统计表（先删后插的真实 SQL 聚合）。
async fn ddqy_statistic_person_trigger(
    pool: &Pool,
    detail_table: &str,
    stat_table: &str,
    year: &str,
    month: &str,
) -> Result<Json<ActionResult<Value>>, AppError> {
    if year.trim().len() != 4 || month.trim().len() != 2 {
        return Err(AppError::BadRequest("year must be yyyy and month must be MM".to_string()));
    }
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let delete_sql = format!(
        "DELETE FROM {st} WHERE statistic_year = $1 AND statistic_month = $2",
        st = stat_table
    );
    client.execute(&delete_sql, &[&year, &month]).await.map_err(|_| AppError::Internal)?;

    let insert_sql = format!(
        "INSERT INTO {st} (id, o2_user, o2_unit, statistic_year, statistic_month, \
         work_day_count, on_duty_times, off_duty_times, result_normal, late_times, not_signed_count, create_time, update_time) \
         SELECT 'pm-' || d.user_id || '-' || $1 || $2 AS id, \
                d.user_id, \
                MAX(d.group_id), \
                $1, $2, \
                COUNT(*), \
                SUM(CASE WHEN d.checkin_type LIKE '%上%' THEN 1 ELSE 0 END), \
                SUM(CASE WHEN d.checkin_type LIKE '%下%' THEN 1 ELSE 0 END), \
                SUM(CASE WHEN d.location_result = 'Normal' THEN 1 ELSE 0 END), \
                SUM(CASE WHEN d.location_result = 'SeriousLate' THEN 1 ELSE 0 END), \
                0, NOW(), NOW() \
         FROM {dt} d \
         WHERE SUBSTRING(d.time, 1, 4) = $1 AND SUBSTRING(d.time, 6, 2) = $2 AND d.user_id IS NOT NULL \
         GROUP BY d.user_id",
        st = stat_table,
        dt = detail_table,
    );
    let n = client.execute(&insert_sql, &[&year, &month]).await.map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("value".to_string(), Value::Bool(true)),
        ("rowsRecalculated".to_string(), Value::Number(serde_json::Number::from(n as i64))),
    ])))))
}

/// 统计重算：按日聚合刷新 unit_day 统计表（o2_unit 以考勤组 group_id 为键）。
async fn ddqy_statistic_unit_day_trigger(
    pool: &Pool,
    detail_table: &str,
    stat_table: &str,
    year: &str,
    month: &str,
    day: &str,
) -> Result<Json<ActionResult<Value>>, AppError> {
    if year.trim().len() != 4 || month.trim().len() != 2 || day.trim().len() != 2 {
        return Err(AppError::BadRequest("year/month/day must be yyyy/MM/dd".to_string()));
    }
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let date = format!("{}-{}-{}", year, month, day);

    let delete_sql = format!(
        "DELETE FROM {st} WHERE statistic_year = $1 AND statistic_month = $2 AND statistic_date = $3",
        st = stat_table
    );
    client
        .execute(&delete_sql, &[&year, &month, &date])
        .await
        .map_err(|_| AppError::Internal)?;

    let insert_sql = format!(
        "INSERT INTO {st} (id, o2_unit, statistic_year, statistic_month, statistic_date, \
         on_duty_times, off_duty_times, result_normal, late_times, create_time, update_time) \
         SELECT 'ud-' || COALESCE(d.group_id, 'none') || '-' || $3 AS id, \
                COALESCE(d.group_id, ''), \
                $1, $2, $3, \
                SUM(CASE WHEN d.checkin_type LIKE '%上%' THEN 1 ELSE 0 END), \
                SUM(CASE WHEN d.checkin_type LIKE '%下%' THEN 1 ELSE 0 END), \
                SUM(CASE WHEN d.location_result = 'Normal' THEN 1 ELSE 0 END), \
                SUM(CASE WHEN d.location_result = 'SeriousLate' THEN 1 ELSE 0 END), \
                NOW(), NOW() \
         FROM {dt} d \
         WHERE SUBSTRING(d.time, 1, 4) = $1 AND SUBSTRING(d.time, 6, 2) = $2 AND SUBSTRING(d.time, 9, 2) = $4 \
         GROUP BY d.group_id",
        st = stat_table,
        dt = detail_table,
    );
    let n = client
        .execute(&insert_sql, &[&year, &month, &date, &day])
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("value".to_string(), Value::Bool(true)),
        ("rowsRecalculated".to_string(), Value::Number(serde_json::Number::from(n as i64))),
    ])))))
}

async fn stat_person_month_query(
    pool: &Pool,
    stat_table: &str,
    person: Option<&str>,
    year: &str,
    month: &str,
    unit_filter: Option<&str>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let sql = format!(
        "SELECT id, o2_user, o2_unit, statistic_year, statistic_month, work_day_count, on_duty_times, off_duty_times, \
         result_normal, late_times, serious_late_times, leave_early_times, absenteeism_times, not_signed_count \
         FROM {st} WHERE ($1 = '' OR o2_user = $1) AND statistic_year = $2 AND statistic_month = $3 AND ($4 = '' OR o2_unit = $4)",
        st = stat_table
    );
    let empty = "";
    let person_v = person.unwrap_or(empty);
    let unit = unit_filter.unwrap_or(empty);
    let rows = client.query(&sql, &[&person_v, &year, &month, &unit]).await.map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows.iter().map(stat_row_to_value).collect();
    let count = data.len() as i64;
    Ok(Json(ActionResult::java_success(Value::Array(data), count, 0)))
}

async fn stat_unit_month_query(
    pool: &Pool,
    stat_table: &str,
    unit: &str,
    year: &str,
    month: &str,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let sql = format!(
        "SELECT id, o2_user, o2_unit, statistic_year, statistic_month, work_day_count, on_duty_times, off_duty_times, \
         result_normal, late_times, serious_late_times, leave_early_times, absenteeism_times, not_signed_count \
         FROM {st} WHERE o2_unit = $1 AND statistic_year = $2 AND statistic_month = $3",
        st = stat_table
    );
    let rows = client.query(&sql, &[&unit, &year, &month]).await.map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows.iter().map(stat_row_to_value).collect();

    let count = data.len() as i64;
    Ok(Json(ActionResult::java_success(
        Value::Array(data),
        count,
        0,
    )))
}

fn stat_row_to_value(row: &deadpool_postgres::tokio_postgres::Row) -> Value {
    fn num(row: &deadpool_postgres::tokio_postgres::Row, col: &str) -> Value {
        match row.try_get::<_, Option<i64>>(col) {
            Ok(v) => Value::Number(serde_json::Number::from(v.unwrap_or(0))),
            Err(_) => Value::Number(serde_json::Number::from(0)),
        }
    }
    Value::Object(serde_json::Map::from_iter([
        ("id".to_string(), Value::String(row.get::<_, Option<String>>("id").unwrap_or_default())),
        ("o2User".to_string(), Value::String(row.get::<_, Option<String>>("o2_user").unwrap_or_default())),
        ("o2Unit".to_string(), Value::String(row.get::<_, Option<String>>("o2_unit").unwrap_or_default())),
        ("statisticYear".to_string(), Value::String(row.get::<_, Option<String>>("statistic_year").unwrap_or_default())),
        ("statisticMonth".to_string(), Value::String(row.get::<_, Option<String>>("statistic_month").unwrap_or_default())),
        ("workDayCount".to_string(), num(row, "work_day_count")),
        ("onDutyTimes".to_string(), num(row, "on_duty_times")),
        ("offDutyTimes".to_string(), num(row, "off_duty_times")),
        ("resultNormal".to_string(), num(row, "result_normal")),
        ("lateTimes".to_string(), num(row, "late_times")),
        ("seriousLateTimes".to_string(), num(row, "serious_late_times")),
        ("leaveEarlyTimes".to_string(), num(row, "leave_early_times")),
        ("absenteeismTimes".to_string(), num(row, "absenteeism_times")),
        ("notSignedCount".to_string(), num(row, "not_signed_count")),
    ]))
}

// ── dingding 端点（8 个） ──────────────────────────────────────────

/// DELETE /jaxrs/attendance/assemble/control/dingding/all
pub async fn dingding_delete_all(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    ddqy_delete_all(&pool, "x_attendance_dingding_detail", "dingding", &session).await
}

/// GET /jaxrs/attendance/assemble/control/dingding/sync/from/{from}/to/{to}/start
pub async fn dingding_sync_start(
    pool: Extension<Pool>,
    Path((from, to)): Path<(String, String)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    ddqy_sync_start(&pool, "dingding", &from, &to).await
}

/// GET /jaxrs/attendance/assemble/control/dingding/sync/list
pub async fn dingding_sync_list(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    ddqy_sync_list(&pool, "dingding").await
}

/// PUT /jaxrs/attendance/assemble/control/dingding/attendance/list/{id}/next/{count}
pub async fn dingding_attendance_list_next(
    pool: Extension<Pool>,
    Path((id, count)): Path<(String, i64)>,
    body: Option<Json<Value>>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    ddqy_attendance_list_next(&pool, "x_attendance_dingding_detail", &id, count, body).await
}

/// GET /jaxrs/attendance/assemble/control/dingding/statistic/person/year/{year}/month/{month}
pub async fn dingding_statistic_person_trigger(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    Path((year, month)): Path<(String, String)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    require_admin(&pool, &session).await?;
    ddqy_statistic_person_trigger(&pool, "x_attendance_dingding_detail", "x_attendance_statistic_dd_person_month", &year, &month).await
}

/// GET /jaxrs/attendance/assemble/control/dingding/statistic/unit/year/{year}/month/{month}/day/{day}
pub async fn dingding_statistic_unit_day_trigger(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    Path((year, month, day)): Path<(String, String, String)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    require_admin(&pool, &session).await?;
    ddqy_statistic_unit_day_trigger(&pool, "x_attendance_dingding_detail", "x_attendance_statistic_dd_unit_day", &year, &month, &day).await
}

/// GET /jaxrs/attendance/assemble/control/dingdingstatistic/person/{person}/{year}/{month}
pub async fn dingdingstatistic_person(
    pool: Extension<Pool>,
    Path((person, year, month)): Path<(String, String, String)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    stat_person_month_query(&pool, "x_attendance_statistic_dd_person_month", Some(&person), &year, &month, None).await
}

/// GET /jaxrs/attendance/assemble/control/dingdingstatistic/person/unit/{unit}/{year}/{month}
pub async fn dingdingstatistic_person_unit(
    pool: Extension<Pool>,
    Path((unit, year, month)): Path<(String, String, String)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    // Java 语义：查该部门下人员月份统计；本 schema 中 person 维度以 o2_unit 过滤实现
    stat_person_month_query(&pool, "x_attendance_statistic_dd_person_month", None, &year, &month, Some(&unit)).await
}

/// GET /jaxrs/attendance/assemble/control/dingdingstatistic/unit/{unit}/{year}/{month}
pub async fn dingdingstatistic_unit(
    pool: Extension<Pool>,
    Path((unit, year, month)): Path<(String, String, String)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    // 优先月度统计表；无月度行时回退日粒度聚合行（同表族真实数据）
    let sql = "SELECT id, o2_user, o2_unit, statistic_year, statistic_month, work_day_count, on_duty_times, off_duty_times, \
               result_normal, late_times, serious_late_times, leave_early_times, absenteeism_times, not_signed_count \
               FROM x_attendance_statistic_dd_unit_month WHERE o2_unit = $1 AND statistic_year = $2 AND statistic_month = $3";
    let mut rows = client.query(sql, &[&unit, &year, &month]).await.map_err(|_| AppError::Internal)?;
    if rows.is_empty() {
        rows = client
            .query(
                "SELECT id, o2_user, o2_unit, statistic_year, statistic_month, work_day_count, on_duty_times, off_duty_times, \
                 result_normal, late_times, serious_late_times, leave_early_times, absenteeism_times, not_signed_count \
                 FROM x_attendance_statistic_dd_unit_day WHERE o2_unit = $1 AND statistic_year = $2 AND statistic_month = $3",
                &[&unit, &year, &month],
            )
            .await
            .map_err(|_| AppError::Internal)?;
    }

    let data: Vec<Value> = rows.iter().map(stat_row_to_value).collect();

    let count = data.len() as i64;
    Ok(Json(ActionResult::java_success(
        Value::Array(data),
        count,
        0,
    )))
}

// ── qywx 端点（8 个，与 dingding 同构） ────────────────────────────

/// DELETE /jaxrs/attendance/assemble/control/qywx/all
pub async fn qywx_delete_all(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    ddqy_delete_all(&pool, "x_attendance_qywx_detail", "qywx", &session).await
}

/// GET /jaxrs/attendance/assemble/control/qywx/sync/from/{from}/to/{to}/start
pub async fn qywx_sync_start(
    pool: Extension<Pool>,
    Path((from, to)): Path<(String, String)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    ddqy_sync_start(&pool, "qywx", &from, &to).await
}

/// GET /jaxrs/attendance/assemble/control/qywx/sync/list
pub async fn qywx_sync_list(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    ddqy_sync_list(&pool, "qywx").await
}

/// PUT /jaxrs/attendance/assemble/control/qywx/attendance/list/{id}/next/{count}
pub async fn qywx_attendance_list_next(
    pool: Extension<Pool>,
    Path((id, count)): Path<(String, i64)>,
    body: Option<Json<Value>>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    ddqy_attendance_list_next(&pool, "x_attendance_qywx_detail", &id, count, body).await
}

/// GET /jaxrs/attendance/assemble/control/qywx/statistic/person/year/{year}/month/{month}
pub async fn qywx_statistic_person_trigger(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    Path((year, month)): Path<(String, String)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    require_admin(&pool, &session).await?;
    ddqy_statistic_person_trigger(&pool, "x_attendance_qywx_detail", "x_attendance_statistic_qywx_person_month", &year, &month).await
}

/// GET /jaxrs/attendance/assemble/control/qywx/statistic/unit/year/{year}/month/{month}/day/{day}
pub async fn qywx_statistic_unit_day_trigger(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    Path((year, month, day)): Path<(String, String, String)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    require_admin(&pool, &session).await?;
    ddqy_statistic_unit_day_trigger(&pool, "x_attendance_qywx_detail", "x_attendance_statistic_qywx_unit_day", &year, &month, &day).await
}

/// GET /jaxrs/attendance/assemble/control/qywxstatistic/person/{person}/{year}/{month}
pub async fn qywxstatistic_person(
    pool: Extension<Pool>,
    Path((person, year, month)): Path<(String, String, String)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    stat_person_month_query(&pool, "x_attendance_statistic_qywx_person_month", Some(&person), &year, &month, None).await
}

/// GET /jaxrs/attendance/assemble/control/qywxstatistic/person/unit/{unit}/{year}/{month}
pub async fn qywxstatistic_person_unit(
    pool: Extension<Pool>,
    Path((unit, year, month)): Path<(String, String, String)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    stat_person_month_query(&pool, "x_attendance_statistic_qywx_person_month", None, &year, &month, Some(&unit)).await
}

/// GET /jaxrs/attendance/assemble/control/qywxstatistic/unit/{unit}/{year}/{month}
pub async fn qywxstatistic_unit(
    pool: Extension<Pool>,
    Path((unit, year, month)): Path<(String, String, String)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    stat_unit_month_query(&pool, "x_attendance_statistic_qywx_unit_month", &unit, &year, &month).await
}

// ── v2 appeal（8 个） ──────────────────────────────────────────────
// 状态常量对齐 AttendanceV2AppealInfo：0 待处理 / 1 审批中 / 2 通过 /
// 3 不通过 / 4 管理员已处理。

/// POST /jaxrs/attendance/assemble/control/v2/appeal/list/{page}/size/{size}
pub async fn v2_appeal_list_page_size(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    Path((page, size)): Path<(i64, i64)>,
    body: Option<Json<Value>>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let (limit, offset) = json_page(page, size)?;
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    // 非 admin 只能查看本人申诉记录（IDOR 读过滤）
    let admin = shared::middleware::is_admin(&pool, &session.person_unique).await;
    let filter_user = if admin {
        body.as_ref()
            .map(|Json(b)| b.get("user").and_then(|v| v.as_str()).unwrap_or("").trim().to_string())
            .unwrap_or_default()
    } else {
        session.person_unique.clone()
    };

    let d = dialect();
    let total: i64 = client
        .query_one(
            "SELECT COUNT(*) FROM x_attendance_v2_appeal_info WHERE ($1 = '' OR user_id = $1)",
            &[&filter_user],
        )
        .await
        .map_err(|_| AppError::Internal)?
        .get(0);

    let sql = format!(
        "SELECT id, record_id, user_id, record_date_string, start_time, end_time, reason, status, job_id \
         FROM x_attendance_v2_appeal_info WHERE ($1 = '' OR user_id = $1) \
         ORDER BY create_time DESC LIMIT {} OFFSET {}",
        d.cast_bigint_param(2),
        d.cast_bigint_param(3),
    );
    let rows = client.query(&sql, &[&filter_user, &limit, &offset]).await.map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(total))),
        ("data".to_string(), Value::Array(rows.iter().map(v2_appeal_row).collect())),
    ])))))
}

fn v2_appeal_row(row: &deadpool_postgres::tokio_postgres::Row) -> Value {
    Value::Object(serde_json::Map::from_iter([
        ("id".to_string(), Value::String(row.get::<_, Option<String>>("id").unwrap_or_default())),
        ("recordId".to_string(), Value::String(row.get::<_, Option<String>>("record_id").unwrap_or_default())),
        ("userId".to_string(), Value::String(row.get::<_, Option<String>>("user_id").unwrap_or_default())),
        (
            "recordDateString".to_string(),
            Value::String(row.get::<_, Option<String>>("record_date_string").unwrap_or_default()),
        ),
        ("startTime".to_string(), Value::String(row.get::<_, Option<String>>("start_time").unwrap_or_default())),
        ("endTime".to_string(), Value::String(row.get::<_, Option<String>>("end_time").unwrap_or_default())),
        ("reason".to_string(), Value::String(row.get::<_, Option<String>>("reason").unwrap_or_default())),
        (
            "status".to_string(),
            Value::Number(serde_json::Number::from(row.get::<_, Option<i32>>("status").unwrap_or(0))),
        ),
        ("jobId".to_string(), Value::String(row.get::<_, Option<String>>("job_id").unwrap_or_default())),
    ]))
}

/// POST /jaxrs/attendance/assemble/control/v2/appeal/list/manager/{page}/size/{size}
pub async fn v2_appeal_manager_list_page_size(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    Path((page, size)): Path<(i64, i64)>,
    body: Option<Json<Value>>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    require_admin(&pool, &session).await?;

    let (limit, offset) = json_page(page, size)?;
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let filter_status = body
        .as_ref()
        .map(|Json(b)| b.get("status").and_then(|v| v.as_i64()).unwrap_or(-1).to_string())
        .unwrap_or_else(|| "-1".to_string());

    let d = dialect();
    let total: i64 = client
        .query_one(
            "SELECT COUNT(*) FROM x_attendance_v2_appeal_info WHERE ($1 = '-1' OR status = CAST($1 AS INTEGER))",
            &[&filter_status],
        )
        .await
        .map_err(|_| AppError::Internal)?
        .get(0);

    let sql = format!(
        "SELECT id, record_id, user_id, record_date_string, start_time, end_time, reason, status, job_id \
         FROM x_attendance_v2_appeal_info WHERE ($1 = '-1' OR status = CAST($1 AS INTEGER)) \
         ORDER BY create_time DESC LIMIT {} OFFSET {}",
        d.cast_bigint_param(2),
        d.cast_bigint_param(3),
    );
    let rows = client.query(&sql, &[&filter_status, &limit, &offset]).await.map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(total))),
        ("data".to_string(), Value::Array(rows.iter().map(v2_appeal_row).collect())),
    ])))))
}

/// GET /jaxrs/attendance/assemble/control/v2/appeal/{id}
pub async fn v2_appeal_get(
    pool: Extension<Pool>,
    Path(id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let row = client
        .query_opt(
            "SELECT id, record_id, user_id, record_date_string, start_time, end_time, reason, status, job_id \
             FROM x_attendance_v2_appeal_info WHERE id = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => Ok(Json(ActionResult::success(v2_appeal_row(&row)))),
        None => Ok(Json(ActionResult::error("appeal not found"))),
    }
}

async fn appeal_owner_gate(
    pool: &Pool,
    session: &shared::session::Session,
    id: &str,
) -> Result<Option<deadpool_postgres::tokio_postgres::Row>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT id, record_id, user_id, record_date_string, start_time, end_time, reason, status, job_id, creator_person \
             FROM x_attendance_v2_appeal_info WHERE id = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let Some(row) = row else { return Ok(None) };

    let owner: String = row.get::<_, Option<String>>("creator_person").unwrap_or_default();
    shared::middleware::require_owner(pool, session, &owner).await?;
    Ok(Some(row))
}

/// GET /jaxrs/attendance/assemble/control/v2/appeal/{id}/manager/status
pub async fn v2_appeal_manager_status(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    Path(id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    require_admin(&pool, &session).await?;
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    // 管理员处理异常数据为正常：status=4(END_BY_ADMIN)，记录操作人
    let n = client
        .execute(
            "UPDATE x_attendance_v2_appeal_info SET status = $2, update_status_admin_person = $3, update_time = NOW() WHERE id = $1",
            &[&id, &4i32, &session.person_unique],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if n == 0 {
        return Ok(Json(ActionResult::error("appeal not found")));
    }

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("id".to_string(), Value::String(id)),
        ("status".to_string(), Value::Number(serde_json::Number::from(4))),
    ])))))
}

/// GET /jaxrs/attendance/assemble/control/v2/appeal/{id}/start/check
pub async fn v2_appeal_start_check(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    Path(id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let row = client
        .query_opt(
            "SELECT user_id, status FROM x_attendance_v2_appeal_info WHERE id = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let Some(row) = row else {
        return Ok(Json(ActionResult::error("appeal not found")));
    };

    // Java ExceptionPersonNotEqual：仅本人可发起申诉
    let owner: String = row.get::<_, Option<String>>("user_id").unwrap_or_default();
    if owner != session.person_unique {
        return Err(AppError::Forbidden);
    }

    let status: i32 = row.get::<_, Option<i32>>("status").unwrap_or(0);
    let can_start = status == 0;
    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("value".to_string(), Value::Bool(can_start)),
        (
            "message".to_string(),
            Value::String(if can_start { String::new() } else { "申诉数据已进入流程，不能重复发起".to_string() }),
        ),
    ])))))
}

/// POST /jaxrs/attendance/assemble/control/v2/appeal/{id}/start/process
pub async fn v2_appeal_start_process(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    Path(id): Path<String>,
    Json(payload): Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    if appeal_owner_gate(&pool, &session, &id).await?.is_none() {
        return Ok(Json(ActionResult::error("appeal not found")));
    }
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let job_id = json_str(&payload, "jobId");
    if job_id.is_empty() {
        return Err(AppError::BadRequest("jobId is required".to_string()));
    }

    // 启动流程后修改状态：0 待处理 → 1 审批中
    let n = client
        .execute(
            "UPDATE x_attendance_v2_appeal_info SET status = $2, job_id = $3, update_time = NOW() WHERE id = $1",
            &[&id, &1i32, &job_id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if n == 0 {
        return Ok(Json(ActionResult::error("appeal not found")));
    }

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("id".to_string(), Value::String(id)),
        ("jobId".to_string(), Value::String(job_id)),
        ("status".to_string(), Value::Number(serde_json::Number::from(1))),
    ])))))
}

/// GET /jaxrs/attendance/assemble/control/v2/appeal/{id}/reset/status
pub async fn v2_appeal_reset_status(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    Path(id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    if appeal_owner_gate(&pool, &session, &id).await?.is_none() {
        return Ok(Json(ActionResult::error("appeal not found")));
    }
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    // 还原数据状态，清除流程关联
    let n = client
        .execute(
            "UPDATE x_attendance_v2_appeal_info SET status = $2, job_id = NULL, send_status = FALSE, update_time = NOW() WHERE id = $1",
            &[&id, &0i32],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if n == 0 {
        return Ok(Json(ActionResult::error("appeal not found")));
    }

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("id".to_string(), Value::String(id)),
        ("status".to_string(), Value::Number(serde_json::Number::from(0))),
    ])))))
}

/// POST /jaxrs/attendance/assemble/control/v2/appeal/{id}/end/process
pub async fn v2_appeal_end_process(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    Path(id): Path<String>,
    Json(payload): Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    if appeal_owner_gate(&pool, &session, &id).await?.is_none() {
        return Ok(Json(ActionResult::error("appeal not found")));
    }
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    // 流程结束后回填：审批结果只允许 2(通过)/3(不通过)
    let end_status = payload.get("status").and_then(|v| v.as_i64()).unwrap_or(-1) as i32;
    if end_status != 2 && end_status != 3 {
        return Err(AppError::BadRequest("status must be 2 (agree) or 3 (disagree)".to_string()));
    }
    let job_id = json_opt_str(&payload, "jobId");

    let n = client
        .execute(
            "UPDATE x_attendance_v2_appeal_info SET status = $2, job_id = COALESCE($3, job_id), update_time = NOW() WHERE id = $1",
            &[&id, &end_status, &job_id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if n == 0 {
        return Ok(Json(ActionResult::error("appeal not found")));
    }

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("id".to_string(), Value::String(id)),
        ("status".to_string(), Value::Number(serde_json::Number::from(end_status))),
    ])))))
}

// ── v2 detail（4 个） ──────────────────────────────────────────────

/// GET /jaxrs/attendance/assemble/control/v2/detail/rebuild/person/{person}/date/{date}
pub async fn v2_detail_rebuild_person_date(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    Path((person, date)): Path<(String, String)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    require_admin(&pool, &session).await?;

    let person = person.trim();
    let date = date.trim();
    if person.is_empty() || date.len() != 10 || date.as_bytes().get(4) != Some(&b'-') || date.as_bytes().get(7) != Some(&b'-') {
        return Err(AppError::BadRequest("person and date(yyyy-MM-dd) are required".to_string()));
    }

    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    client
        .execute(
            "DELETE FROM x_attendance_detail WHERE person_id = $1 AND date = $2",
            &[&person, &date],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let new_id = uuid::Uuid::new_v4().to_string();
    client
        .execute(
            "INSERT INTO x_attendance_detail (id, person_id, date, status, creator_person, create_time, update_time) \
             VALUES ($1, $2, $3, 'init', $4, NOW(), NOW())",
            &[&new_id, &person, &date, &session.person_unique],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("value".to_string(), Value::Bool(true)),
        ("detailId".to_string(), Value::String(new_id)),
    ])))))
}

/// GET /jaxrs/attendance/assemble/control/v2/detail/statistic/{detailId}/list/record
pub async fn v2_detail_statistic_record_list(
    pool: Extension<Pool>,
    Path(detail_id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    // 先定位 detail（人 + 日期），再查该人当日打卡记录
    let detail = client
        .query_opt(
            "SELECT person_id, date FROM x_attendance_detail WHERE id = $1",
            &[&detail_id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let Some(detail) = detail else {
        return Ok(Json(ActionResult::error("detail not found")));
    };
    let person: String = detail.get::<_, Option<String>>("person_id").unwrap_or_default();
    let date: String = detail.get::<_, Option<String>>("date").unwrap_or_default();
    let pattern = format!("{}%", date);

    let rows = client
        .query(
            "SELECT id, user_id, check_in_time, check_out_time, status, create_time \
             FROM x_attendance_record WHERE user_id = $1 AND check_in_time LIKE $2 ORDER BY check_in_time ASC",
            &[&person, &pattern],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("userId".to_string(), Value::String(row.get::<_, Option<String>>("user_id").unwrap_or_default())),
                ("checkInTime".to_string(), Value::String(row.get::<_, Option<String>>("check_in_time").unwrap_or_default())),
                (
                    "checkOutTime".to_string(),
                    Value::String(row.get::<_, Option<String>>("check_out_time").unwrap_or_default()),
                ),
                ("status".to_string(), Value::String(row.get::<_, Option<String>>("status").unwrap_or_default())),
            ]))
        })
        .collect();

    let count = data.len() as i64;
    Ok(Json(ActionResult::java_success(
        Value::Array(data),
        count,
        0,
    )))
}

/// 聚合统计公共实现：按状态汇总 detail 行数。
async fn detail_statistic_aggregate(
    pool: &Pool,
    filter_person: &str,
    filter_start: &str,
    filter_end: &str,
) -> Result<Vec<Value>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query(
            "SELECT COALESCE(status, '') AS status, COUNT(*) AS cnt \
             FROM x_attendance_detail \
             WHERE ($1 = '' OR person_id = $1) \
               AND ($2 = '' OR date >= $2) AND ($3 = '' OR date <= $3) \
             GROUP BY COALESCE(status, '') ORDER BY status",
            &[&filter_person, &filter_start, &filter_end],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("status".to_string(), Value::String(row.get::<_, Option<String>>("status").unwrap_or_default())),
                ("count".to_string(), Value::Number(serde_json::Number::from(row.get::<_, i64>("cnt")))),
            ]))
        })
        .collect())
}

/// POST /jaxrs/attendance/assemble/control/v2/detail/statistic/filter
pub async fn v2_detail_statistic_filter(
    pool: Extension<Pool>,
    body: Option<Json<Value>>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let empty = String::new();
    let (filter_person, filter_start, filter_end) = extract_stat_filter(body.as_ref(), &empty);

    let data = detail_statistic_aggregate(&pool, &filter_person, &filter_start, &filter_end).await?;

    let count = data.len() as i64;
    Ok(Json(ActionResult::java_success(
        Value::Array(data),
        count,
        0,
    )))
}

/// POST /jaxrs/attendance/assemble/control/v2/detail/statistic/export/filter
pub async fn v2_detail_statistic_export_filter(
    pool: Extension<Pool>,
    body: Option<Json<Value>>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let empty = String::new();
    let (filter_person, filter_start, filter_end) = extract_stat_filter(body.as_ref(), &empty);

    let data = detail_statistic_aggregate(&pool, &filter_person, &filter_start, &filter_end).await?;
    let export_flag = uuid::Uuid::new_v4().to_string();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("exportFlag".to_string(), Value::String(export_flag)),
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

fn extract_stat_filter(body: Option<&Json<Value>>, empty: &String) -> (String, String, String) {
    let get = |key: &str| -> String {
        body.and_then(|Json(b)| b.get(key))
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .trim()
            .to_string()
    };
    let _ = empty;
    (get("person"), get("startDate"), get("endDate"))
}

// ── v2 group rebuild（1 个）+ groupschedule（4 个） ────────────────

/// GET /jaxrs/attendance/assemble/control/v2/group/rebuild/detail/group/{groupId}/date/{date}
pub async fn v2_group_rebuild_detail_group_date(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    Path((group_id, date)): Path<(String, String)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    require_admin(&pool, &session).await?;

    let date = date.trim();
    if group_id.trim().is_empty() || date.len() != 10 {
        return Err(AppError::BadRequest("groupId and date(yyyy-MM-dd) are required".to_string()));
    }

    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let group = client
        .query_opt(
            "SELECT participate_list, status FROM x_attendance_v2_group WHERE id = $1",
            &[&group_id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let Some(group) = group else {
        return Ok(Json(ActionResult::error("attendance group not found")));
    };

    // 手动考勤组才允许重算（Java status_auto 校验）
    let status: i32 = group.get::<_, Option<i32>>("status").unwrap_or(1);
    if status == 999 {
        return Ok(Json(ActionResult::error("auto group cannot be rebuilt")));
    }

    let participate: Vec<String> = group
        .get::<_, Option<String>>("participate_list")
        .unwrap_or_default()
        .split(',')
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect();

    if participate.is_empty() {
        return Ok(Json(ActionResult::error("group has no participants")));
    }

    for person in &participate {
        client
            .execute(
                "DELETE FROM x_attendance_detail WHERE person_id = $1 AND date = $2",
                &[person, &date],
            )
            .await
            .map_err(|_| AppError::Internal)?;
        let new_id = uuid::Uuid::new_v4().to_string();
        client
            .execute(
                "INSERT INTO x_attendance_detail (id, person_id, date, status, creator_person, create_time, update_time) \
                 VALUES ($1, $2, $3, 'init', $4, NOW(), NOW())",
                &[&new_id, person, &date, &session.person_unique],
            )
            .await
            .map_err(|_| AppError::Internal)?;
    }

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("value".to_string(), Value::Bool(true)),
        ("personsRebuilt".to_string(), Value::Number(serde_json::Number::from(participate.len() as i64))),
    ])))))
}

/// POST /jaxrs/attendance/assemble/control/v2/groupschedule
pub async fn v2_groupschedule_post(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    Json(payload): Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    require_admin(&pool, &session).await?;

    let group_id = json_str(&payload, "groupId");
    let user_id = normalize_key(&json_str(&payload, "userId"));
    let month = json_str(&payload, "scheduleMonthString");
    let date_str = json_str(&payload, "scheduleDateString");
    let shift_id = json_str(&payload, "shiftId");

    if group_id.trim().is_empty() || user_id.is_empty() || date_str.is_empty() {
        return Err(AppError::BadRequest("groupId/userId/scheduleDateString are required".to_string()));
    }

    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    // 归一化查重：同组同人同日已有排班 → 更新班次而非插入重复行；
    // 班次相同则幂等返回 duplicated=true
    let existing = client
        .query_opt(
            "SELECT id, shift_id FROM x_attendance_v2_group_schedule \
             WHERE group_id = $1 AND user_id = $2 AND schedule_date_string = $3",
            &[&group_id, &user_id, &date_str],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match existing {
        Some(row) => {
            let exist_id: String = row.get("id");
            let exist_shift: String = row.get::<_, Option<String>>("shift_id").unwrap_or_default();
            if exist_shift == shift_id {
                return Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
                    ("id".to_string(), Value::String(exist_id)),
                    ("duplicated".to_string(), Value::Bool(true)),
                ])))));
            }
            client
                .execute(
                    "UPDATE x_attendance_v2_group_schedule SET shift_id = $2, update_time = NOW() WHERE id = $1",
                    &[&exist_id, &shift_id],
                )
                .await
                .map_err(|_| AppError::Internal)?;
            Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(exist_id)),
                ("updated".to_string(), Value::Bool(true)),
            ])))))
        }
        None => {
            let id = uuid::Uuid::new_v4().to_string();
            client
                .execute(
                    "INSERT INTO x_attendance_v2_group_schedule (id, group_id, user_id, schedule_month_string, schedule_date_string, shift_id, creator_person, create_time, update_time) \
                     VALUES ($1,$2,$3,$4,$5,$6,$7,NOW(),NOW())",
                    &[&id, &group_id, &user_id, &month, &date_str, &shift_id, &session.person_unique],
                )
                .await
                .map_err(|_| AppError::Internal)?;
            Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(id)),
                ("created".to_string(), Value::Bool(true)),
            ])))))
        }
    }
}

/// GET /jaxrs/attendance/assemble/control/v2/groupschedule/config/group/{groupId}
pub async fn v2_groupschedule_config_get(
    pool: Extension<Pool>,
    Path(group_id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let row = client
        .query_opt(
            "SELECT id, group_id, config_json, update_time FROM x_attendance_v2_group_schedule_config WHERE group_id = $1",
            &[&group_id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
            ("id".to_string(), Value::String(row.get("id"))),
            ("groupId".to_string(), Value::String(row.get::<_, Option<String>>("group_id").unwrap_or_default())),
            (
                "config".to_string(),
                serde_json::from_str::<Value>(&row.get::<_, Option<String>>("config_json").unwrap_or_default())
                    .unwrap_or(Value::Null),
            ),
            (
                "updateTime".to_string(),
                Value::String(row.get::<_, Option<String>>("update_time").unwrap_or_default()),
            ),
        ]))))),
        None => Ok(Json(ActionResult::error("schedule config not found"))),
    }
}

/// GET /jaxrs/attendance/assemble/control/v2/groupschedule/list/group/{groupId}/month/{month}
pub async fn v2_groupschedule_list_group_month(
    pool: Extension<Pool>,
    Path((group_id, month)): Path<(String, String)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query(
            "SELECT id, group_id, user_id, schedule_month_string, schedule_date_string, shift_id \
             FROM x_attendance_v2_group_schedule \
             WHERE group_id = $1 AND ($2 = '' OR schedule_month_string = $2 OR SUBSTRING(schedule_date_string, 1, 7) = $2) \
             ORDER BY schedule_date_string ASC, user_id ASC",
            &[&group_id, &month],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("groupId".to_string(), Value::String(row.get::<_, Option<String>>("group_id").unwrap_or_default())),
                ("userId".to_string(), Value::String(row.get::<_, Option<String>>("user_id").unwrap_or_default())),
                (
                    "scheduleDateString".to_string(),
                    Value::String(row.get::<_, Option<String>>("schedule_date_string").unwrap_or_default()),
                ),
                ("shiftId".to_string(), Value::String(row.get::<_, Option<String>>("shift_id").unwrap_or_default())),
            ]))
        })
        .collect();

    let count = data.len() as i64;
    Ok(Json(ActionResult::java_success(
        Value::Array(data),
        count,
        0,
    )))
}

/// POST /jaxrs/attendance/assemble/control/v2/groupschedule/list/filter
pub async fn v2_groupschedule_list_filter(
    pool: Extension<Pool>,
    body: Option<Json<Value>>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let get = |key: &str| -> String {
        body.as_ref()
            .and_then(|Json(b)| b.get(key))
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .trim()
            .to_string()
    };
    let group_id = get("groupId");
    let user_id = get("userId");
    let month = get("scheduleMonthString");

    let rows = client
        .query(
            "SELECT id, group_id, user_id, schedule_month_string, schedule_date_string, shift_id \
             FROM x_attendance_v2_group_schedule \
             WHERE ($1 = '' OR group_id = $1) AND ($2 = '' OR user_id = $2) \
               AND ($3 = '' OR schedule_month_string = $3) \
             ORDER BY schedule_date_string ASC LIMIT 500",
            &[&group_id, &user_id, &month],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("groupId".to_string(), Value::String(row.get::<_, Option<String>>("group_id").unwrap_or_default())),
                ("userId".to_string(), Value::String(row.get::<_, Option<String>>("user_id").unwrap_or_default())),
                (
                    "scheduleMonthString".to_string(),
                    Value::String(row.get::<_, Option<String>>("schedule_month_string").unwrap_or_default()),
                ),
                (
                    "scheduleDateString".to_string(),
                    Value::String(row.get::<_, Option<String>>("schedule_date_string").unwrap_or_default()),
                ),
                ("shiftId".to_string(), Value::String(row.get::<_, Option<String>>("shift_id").unwrap_or_default())),
            ]))
        })
        .collect();

    let count = data.len() as i64;
    Ok(Json(ActionResult::java_success(
        Value::Array(data),
        count,
        0,
    )))
}

// ── v2 leave template（1 个） ──────────────────────────────────────

/// GET /jaxrs/attendance/assemble/control/v2/leave/template
pub async fn v2_leave_template() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        (
            "columns".to_string(),
            Value::Array(vec![
                Value::String("person".to_string()),
                Value::String("leaveType".to_string()),
                Value::String("startTime".to_string()),
                Value::String("endTime".to_string()),
                Value::String("description".to_string()),
            ]),
        ),
        ("fileName".to_string(), Value::String("请假导入模板.xlsx".to_string())),
    ])))))
}

// ── v2 mobile（3 个） ──────────────────────────────────────────────

fn now_unix_secs() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0)
}

/// GET /jaxrs/attendance/assemble/control/v2/mobile/check/pre
pub async fn v2_mobile_pre_check(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    // 当前用户命中的考勤组（复用 v2_group_person_date 的参与匹配语义）
    let today = date_string_from_unix(now_unix_secs());
    let group_rows = client
        .query(
            "SELECT id, group_name, check_type, work_place_id_list \
             FROM x_attendance_v2_group \
             WHERE (',' || COALESCE(participate_list, '') || ',') LIKE ('%,' || $1 || ',%') \
               AND (start_date IS NULL OR start_date = '' OR start_date <= $2) \
               AND (end_date IS NULL OR end_date = '' OR end_date >= $2) \
             ORDER BY create_time DESC LIMIT 1",
            &[&session.person_unique, &today],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let pattern = format!("{}%", today);
    let record_rows = client
        .query(
            "SELECT id, check_in_type, check_in_result, source_type, create_time \
             FROM x_attendance_v2_checkin_record WHERE user_id = $1 AND record_date_string LIKE $2 \
             ORDER BY create_time DESC",
            &[&session.person_unique, &pattern],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let records: Vec<Value> = record_rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                (
                    "checkInType".to_string(),
                    Value::String(row.get::<_, Option<String>>("check_in_type").unwrap_or_default()),
                ),
                (
                    "checkInResult".to_string(),
                    Value::String(row.get::<_, Option<String>>("check_in_result").unwrap_or_default()),
                ),
                (
                    "sourceType".to_string(),
                    Value::String(row.get::<_, Option<String>>("source_type").unwrap_or_default()),
                ),
                (
                    "createTime".to_string(),
                    Value::String(row.get::<_, Option<String>>("create_time").unwrap_or_default()),
                ),
            ]))
        })
        .collect();

    let group = group_rows.first().map(|row| {
        Value::Object(serde_json::Map::from_iter([
            ("id".to_string(), Value::String(row.get("id"))),
            ("groupName".to_string(), Value::String(row.get::<_, Option<String>>("group_name").unwrap_or_default())),
            ("checkType".to_string(), Value::String(row.get::<_, Option<String>>("check_type").unwrap_or_default())),
            (
                "workPlaceIdList".to_string(),
                Value::String(row.get::<_, Option<String>>("work_place_id_list").unwrap_or_default()),
            ),
        ]))
    });
    let can_check_in = group.is_some();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("date".to_string(), Value::String(today)),
        (
            "group".to_string(),
            group.unwrap_or(Value::Null),
        ),
        ("canCheckIn".to_string(), Value::Bool(can_check_in)),
        ("records".to_string(), Value::Array(records)),
    ])))))
}

async fn mobile_check_impl(
    client: &deadpool_postgres::Client,
    pool: &Pool,
    session: &shared::session::Session,
    payload: &Value,
    default_source: &str,
) -> Result<(String, bool), AppError> {
    let user_id = normalize_key(
        payload.get("userId").and_then(|v| v.as_str()).unwrap_or(&session.person_unique),
    );
    if user_id.is_empty() {
        return Err(AppError::BadRequest("userId is required".to_string()));
    }
    if user_id != session.person_unique {
        shared::middleware::require_owner(pool, session, &user_id).await?;
    }

    let date_str = payload
        .get("recordDateString")
        .and_then(|v| v.as_str())
        .filter(|s| !s.trim().is_empty())
        .map(|s| s.trim().to_string())
        .unwrap_or_else(|| date_string_from_unix(now_unix_secs()));

    let check_in_type = json_str(payload, "checkInType");
    if check_in_type.is_empty() {
        return Err(AppError::BadRequest("checkInType is required".to_string()));
    }
    let source_type = payload
        .get("sourceType")
        .and_then(|v| v.as_str())
        .filter(|s| !s.trim().is_empty())
        .unwrap_or(default_source)
        .to_string();

    let dup = client
        .query_opt(
            "SELECT id FROM x_attendance_v2_checkin_record \
             WHERE user_id = $1 AND record_date_string = $2 AND check_in_type = $3 LIMIT 1",
            &[&user_id, &date_str, &check_in_type],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    if let Some(row) = dup {
        let exist_id: String = row.get("id");
        return Ok((exist_id, true));
    }

    let id = uuid::Uuid::new_v4().to_string();
    let result = json_opt_str(payload, "checkInResult").unwrap_or_else(|| "Normal".to_string());
    let description = json_opt_str(payload, "description");
    let address = json_opt_str(payload, "recordAddress");
    let longitude = json_opt_str(payload, "longitude");
    let latitude = json_opt_str(payload, "latitude");
    let group_id = json_opt_str(payload, "groupId");
    let shift_id = json_opt_str(payload, "shiftId");

    client
        .execute(
            "INSERT INTO x_attendance_v2_checkin_record (id, user_id, record_date_string, source_type, check_in_result, check_in_type, description, record_address, longitude, latitude, group_id, shift_id, creator_person, create_time, update_time) \
             VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11,$12,$13,NOW(),NOW())",
            &[&id, &user_id, &date_str, &source_type, &result, &check_in_type, &description, &address, &longitude, &latitude, &group_id, &shift_id, &session.person_unique],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    Ok((id, false))
}

/// POST /jaxrs/attendance/assemble/control/v2/mobile/check
pub async fn v2_mobile_check(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    Json(payload): Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let (id, duplicated) = mobile_check_impl(&client, &pool, &session, &payload, "移动端").await?;

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("id".to_string(), Value::String(id)),
        ("duplicated".to_string(), Value::Bool(duplicated)),
    ])))))
}

/// POST /jaxrs/attendance/assemble/control/v2/mobile/check/ from/out
pub async fn v2_mobile_check_from_out(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    Json(payload): Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    // 外部系统调用（门禁等）：允许代他人打卡，但需 admin
    let target = normalize_key(json_str(&payload, "userId").as_str());
    if !target.is_empty() && target != session.person_unique {
        require_admin(&pool, &session).await?;
    }
    let (id, duplicated) = mobile_check_impl(&client, &pool, &session, &payload, "外部设备").await?;

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("id".to_string(), Value::String(id)),
        ("duplicated".to_string(), Value::Bool(duplicated)),
        ("fromOut".to_string(), Value::Bool(true)),
    ])))))
}

// ── v2 my（4 个） ──────────────────────────────────────────────────

/// GET /jaxrs/attendance/assemble/control/v2/my/version
pub async fn v2_my_version() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("version".to_string(), Value::String("2".to_string())),
    ])))))
}

/// GET /jaxrs/attendance/assemble/control/v2/my/controls
pub async fn v2_my_controls(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let admin = shared::middleware::is_admin(&pool, &session.person_unique).await;

    // 是否考勤组协助管理员：assist_admin_list 含本人
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let assist_rows = client
        .query(
            "SELECT 1 FROM x_attendance_v2_group \
             WHERE (',' || COALESCE(assist_admin_list, '') || ',') LIKE ('%,' || $1 || ',%') LIMIT 1",
            &[&session.person_unique],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("admin".to_string(), Value::Bool(admin)),
        ("readAdmin".to_string(), Value::Bool(admin)),
        ("assistAdmin".to_string(), Value::Bool(!assist_rows.is_empty())),
    ])))))
}

/// POST /jaxrs/attendance/assemble/control/v2/my/detail/list
pub async fn v2_my_detail_list(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    body: Option<Json<Value>>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let date = body
        .as_ref()
        .map(|Json(b)| b.get("date").and_then(|v| v.as_str()).unwrap_or("").trim().to_string())
        .unwrap_or_default();

    let rows = if date.is_empty() {
        client
            .query(
                "SELECT id, person_id, date, status, create_time FROM x_attendance_detail \
                 WHERE person_id = $1 ORDER BY date DESC LIMIT 100",
                &[&session.person_unique],
            )
            .await
            .map_err(|_| AppError::Internal)?
    } else {
        client
            .query(
                "SELECT id, person_id, date, status, create_time FROM x_attendance_detail \
                 WHERE person_id = $1 AND date = $2 ORDER BY date DESC LIMIT 100",
                &[&session.person_unique, &date],
            )
            .await
            .map_err(|_| AppError::Internal)?
    };

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("personId".to_string(), Value::String(row.get::<_, Option<String>>("person_id").unwrap_or_default())),
                ("date".to_string(), Value::String(row.get::<_, Option<String>>("date").unwrap_or_default())),
                ("status".to_string(), Value::String(row.get::<_, Option<String>>("status").unwrap_or_default())),
                (
                    "createTime".to_string(),
                    Value::String(row.get::<_, Option<String>>("create_time").unwrap_or_default()),
                ),
            ]))
        })
        .collect();

    let count = data.len() as i64;
    Ok(Json(ActionResult::java_success(
        Value::Array(data),
        count,
        0,
    )))
}

/// POST /jaxrs/attendance/assemble/control/v2/my/rest/date/check
pub async fn v2_my_rest_date_check(
    pool: Extension<Pool>,
    body: Option<Json<Value>>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let date = body
        .as_ref()
        .map(|Json(b)| b.get("date").and_then(|v| v.as_str()).unwrap_or("").trim().to_string())
        .unwrap_or_default();
    if date.len() != 10 {
        return Err(AppError::BadRequest("date(yyyy-MM-dd) is required".to_string()));
    }

    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    // workday 配置表：命中记录按 is_workday 判定；未配置默认视为工作日
    let row = client
        .query_opt(
            "SELECT is_workday FROM x_attendance_workday_config WHERE work_date = $1 ORDER BY create_time DESC LIMIT 1",
            &[&date],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let is_rest = match row {
        Some(row) => !row.get::<_, Option<bool>>("is_workday").unwrap_or(true),
        None => false,
    };

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("date".to_string(), Value::String(date)),
        ("restDay".to_string(), Value::Bool(is_rest)),
    ])))))
}

// ── v2 record（4 个） ──────────────────────────────────────────────

/// GET /jaxrs/attendance/assemble/control/v2/record/delete/people/{people}/date/{date}
pub async fn v2_record_delete_people_date(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    Path((people, date)): Path<(String, String)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let people = normalize_key(&people);
    let date = date.trim();
    if people.is_empty() || date.len() != 10 {
        return Err(AppError::BadRequest("people and date(yyyy-MM-dd) are required".to_string()));
    }

    // 仅本人或管理员可删除打卡记录（IDOR）
    shared::middleware::require_owner(&pool, &session, &people).await?;

    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let pattern = format!("{}%", date);

    let n_record = client
        .execute(
            "DELETE FROM x_attendance_record WHERE user_id = $1 AND check_in_time LIKE $2",
            &[&people, &pattern],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    let n_checkin = client
        .execute(
            "DELETE FROM x_attendance_v2_checkin_record WHERE user_id = $1 AND record_date_string = $2",
            &[&people, &date],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("recordDeleted".to_string(), Value::Number(serde_json::Number::from(n_record as i64))),
        ("checkinDeleted".to_string(), Value::Number(serde_json::Number::from(n_checkin as i64))),
    ])))))
}

/// GET /jaxrs/attendance/assemble/control/v2/record/template
pub async fn v2_record_template() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        (
            "columns".to_string(),
            Value::Array(vec![
                Value::String("userId".to_string()),
                Value::String("recordDateString".to_string()),
                Value::String("checkInType".to_string()),
                Value::String("checkInResult".to_string()),
                Value::String("description".to_string()),
            ]),
        ),
        ("fileName".to_string(), Value::String("打卡记录导入模板.xlsx".to_string())),
    ])))))
}

async fn import_checkin_rows(
    pool: &Pool,
    session: &shared::session::Session,
    rows: &[Value],
    default_source: &str,
) -> Result<(i64, Vec<String>), AppError> {
    let admin = shared::middleware::is_admin(pool, &session.person_unique).await;
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let mut inserted: i64 = 0;
    let mut ids: Vec<String> = Vec::with_capacity(rows.len());

    for item in rows {
        let user_id = normalize_key(item.get("userId").and_then(|v| v.as_str()).unwrap_or(""));
        if user_id.is_empty() {
            return Err(AppError::BadRequest("each row requires userId".to_string()));
        }
        if !admin && user_id != session.person_unique {
            return Err(AppError::Forbidden);
        }
        let date_str = item
            .get("recordDateString")
            .and_then(|v| v.as_str())
            .filter(|s| !s.trim().is_empty())
            .map(|s| s.trim().to_string())
            .unwrap_or_else(|| date_string_from_unix(now_unix_secs()));
        let check_in_type = item.get("checkInType").and_then(|v| v.as_str()).unwrap_or("").trim();
        if check_in_type.is_empty() {
            return Err(AppError::BadRequest("each row requires checkInType".to_string()));
        }

        let dup = client
            .query_opt(
                "SELECT id FROM x_attendance_v2_checkin_record \
                 WHERE user_id = $1 AND record_date_string = $2 AND check_in_type = $3 LIMIT 1",
                &[&user_id, &date_str, &check_in_type],
            )
            .await
            .map_err(|_| AppError::Internal)?;
        if let Some(row) = dup {
            ids.push(row.get::<_, String>("id"));
            continue;
        }

        let id = uuid::Uuid::new_v4().to_string();
        let source_type = item
            .get("sourceType")
            .and_then(|v| v.as_str())
            .filter(|s| !s.trim().is_empty())
            .unwrap_or(default_source)
            .to_string();
        let result = item
            .get("checkInResult")
            .and_then(|v| v.as_str())
            .filter(|s| !s.trim().is_empty())
            .unwrap_or("Normal")
            .to_string();
        let description = item.get("description").and_then(|v| v.as_str()).map(|s| s.to_string());

        client
            .execute(
                "INSERT INTO x_attendance_v2_checkin_record (id, user_id, record_date_string, source_type, check_in_result, check_in_type, description, creator_person, create_time, update_time) \
                 VALUES ($1,$2,$3,$4,$5,$6,$7,$8,NOW(),NOW())",
                &[&id, &user_id, &date_str, &source_type, &result, &check_in_type.to_string(), &description, &session.person_unique],
            )
            .await
            .map_err(|_| AppError::Internal)?;
        inserted += 1;
        ids.push(id);
    }

    Ok((inserted, ids))
}

/// POST /jaxrs/attendance/assemble/control/v2/record/import
pub async fn v2_record_import(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    Json(payload): Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let rows = payload
        .get("rows")
        .and_then(|v| v.as_array())
        .ok_or_else(|| AppError::BadRequest("rows array is required".to_string()))?;
    if rows.is_empty() {
        return Err(AppError::BadRequest("rows must not be empty".to_string()));
    }

    let (inserted, ids) = import_checkin_rows(&pool, &session, rows, "导入").await?;

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("inserted".to_string(), Value::Number(serde_json::Number::from(inserted))),
        ("total".to_string(), Value::Number(serde_json::Number::from(ids.len() as i64))),
        ("ids".to_string(), Value::Array(ids.into_iter().map(Value::String).collect())),
    ])))))
}

/// POST /jaxrs/attendance/assemble/control/v2/record/import/daily
pub async fn v2_record_import_daily(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    Json(payload): Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let date_str = payload
        .get("date")
        .and_then(|v| v.as_str())
        .map(|s| s.trim().to_string())
        .unwrap_or_default();
    if date_str.len() != 10 {
        return Err(AppError::BadRequest("date(yyyy-MM-dd) is required".to_string()));
    }
    let records = payload
        .get("records")
        .and_then(|v| v.as_array())
        .ok_or_else(|| AppError::BadRequest("records array is required".to_string()))?;
    if records.is_empty() {
        return Err(AppError::BadRequest("records must not be empty".to_string()));
    }

    // 归一化：为每行补齐 recordDateString 后复用批量导入
    let normalized: Vec<Value> = records
        .iter()
        .map(|r| {
            let mut obj = r.clone();
            if let Some(map) = obj.as_object_mut() {
                map.entry("recordDateString".to_string()).or_insert_with(|| Value::String(date_str.clone()));
            }
            obj
        })
        .collect();

    let (inserted, ids) = import_checkin_rows(&pool, &session, &normalized, "导入").await?;

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("date".to_string(), Value::String(date_str)),
        ("inserted".to_string(), Value::Number(serde_json::Number::from(inserted))),
        ("ids".to_string(), Value::Array(ids.into_iter().map(Value::String).collect())),
    ])))))
}

// ── v2 workplace（4 个，复用 x_attendance_workplace 表） ───────────

/// POST /jaxrs/attendance/assemble/control/v2/workplace
pub async fn v2_workplace_post(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    Json(payload): Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    require_admin(&pool, &session).await?;

    let name = json_str(&payload, "name");
    if name.trim().is_empty() {
        return Err(AppError::BadRequest("name is required".to_string()));
    }
    let address = json_opt_str(&payload, "address");
    let req_id = json_opt_str(&payload, "id");

    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    // 归一化查重：同名（trim+lower）地点只保留一行；带 id 则按 id 更新
    let target_id: Option<String> = match req_id {
        Some(ref id) => {
            let exists = client
                .query_opt("SELECT id FROM x_attendance_workplace WHERE id = $1", &[id])
                .await
                .map_err(|_| AppError::Internal)?;
            match exists {
                Some(row) => Some(row.get::<_, String>("id")),
                None => None,
            }
        }
        None => {
            let norm = normalize_key(&name);
            client
                .query_opt(
                    "SELECT id FROM x_attendance_workplace WHERE LOWER(TRIM(name)) = $1 LIMIT 1",
                    &[&norm],
                )
                .await
                .map_err(|_| AppError::Internal)?
                .map(|row| row.get::<_, Option<String>>("id").unwrap_or_default())
        }
    };

    match target_id {
        Some(id) if !id.is_empty() => {
            client
                .execute(
                    "UPDATE x_attendance_workplace SET name = $2, address = $3, update_person = $4, update_time = NOW() WHERE id = $1",
                    &[&id, &name, &address, &session.person_unique],
                )
                .await
                .map_err(|_| AppError::Internal)?;
            Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(id)),
                ("name".to_string(), Value::String(name)),
                ("updated".to_string(), Value::Bool(true)),
            ])))))
        }
        _ => {
            let id = uuid::Uuid::new_v4().to_string();
            client
                .execute(
                    "INSERT INTO x_attendance_workplace (id, name, address, creator, creator_person, update_person, create_time, update_time) \
                     VALUES ($1,$2,$3,$4,$4,$4,NOW(),NOW())",
                    &[&id, &name, &address, &session.person_unique],
                )
                .await
                .map_err(|_| AppError::Internal)?;
            Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(id)),
                ("name".to_string(), Value::String(name)),
                ("created".to_string(), Value::Bool(true)),
            ])))))
        }
    }
}

/// DELETE /jaxrs/attendance/assemble/control/v2/workplace/{id}
pub async fn v2_workplace_delete(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    Path(id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    delete_admin_record(&pool, "x_attendance_workplace", &id, &session, "workplace").await
}

/// GET /jaxrs/attendance/assemble/control/v2/workplace/list/all
pub async fn v2_workplace_list_all(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query(
            "SELECT id, name, address, create_time FROM x_attendance_workplace ORDER BY create_time DESC",
            &[],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(workplace_row_value)
        .collect();

    let count = data.len() as i64;
    Ok(Json(ActionResult::java_success(Value::Array(data), count, 0)))
}

fn workplace_row_value(row: &deadpool_postgres::tokio_postgres::Row) -> Value {
    Value::Object(serde_json::Map::from_iter([
        ("id".to_string(), Value::String(row.get::<_, Option<String>>("id").unwrap_or_default())),
        ("name".to_string(), Value::String(row.get::<_, Option<String>>("name").unwrap_or_default())),
        ("address".to_string(), Value::String(row.get::<_, Option<String>>("address").unwrap_or_default())),
        (
            "createTime".to_string(),
            Value::String(row.get::<_, Option<String>>("create_time").unwrap_or_default()),
        ),
    ]))
}

/// POST /jaxrs/attendance/assemble/control/v2/workplace/list/ids
pub async fn v2_workplace_list_ids(
    pool: Extension<Pool>,
    Json(payload): Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let ids = payload
        .get("ids")
        .and_then(|v| v.as_array())
        .map(|a| {
            a.iter()
                .filter_map(|x| x.as_str())
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
                .collect::<Vec<_>>()
                .join(",")
        })
        .unwrap_or_default();
    if ids.is_empty() {
        return Err(AppError::BadRequest("ids array is required".to_string()));
    }

    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query(
            "SELECT id, name, address, create_time FROM x_attendance_workplace \
             WHERE id = ANY(string_to_array($1, ',')) ORDER BY create_time DESC",
            &[&ids],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows.iter().map(workplace_row_value).collect();

    let count = data.len() as i64;
    Ok(Json(ActionResult::java_success(
        Value::Array(data),
        count,
        0,
    )))
}


