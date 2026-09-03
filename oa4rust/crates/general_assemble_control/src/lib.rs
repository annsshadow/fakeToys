use axum::{
    extract::{Extension, Json, Path},
    Router,
    routing::{get, post},
};
use deadpool_postgres::Pool;
use serde_json::Value;
use shared::error::AppError;
use shared::response::ActionResult;

pub const JAVA_BASE: &str = "/jaxrs/general_assemble_control";
pub mod routes;

#[cfg(test)]
mod tests;
#[cfg(test)]
mod tests_generated;


pub async fn get_general_control_status(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let row = client
        .query_one(
            "SELECT id, system_name, maintenance_mode, allow_registration, version FROM x_general_assemble_control_config LIMIT 1",
            &[],
        )
        .await;

    let data = match row {
        Ok(r) => serde_json::Map::from_iter([
            ("id".to_string(), Value::String(r.get("id"))),
            ("systemName".to_string(), Value::String(r.get("system_name"))),
            ("\"maintenanceMode\"".to_string(), Value::Bool(r.get("maintenance_mode"))),
            ("\"allowRegistration\"".to_string(), Value::Bool(r.get("allow_registration"))),
            ("version".to_string(), Value::String(r.get("version"))),
        ]),
        Err(_) => serde_json::Map::from_iter([
            ("id".to_string(), Value::String(String::new())),
            ("systemName".to_string(), Value::String(String::new())),
            ("\"maintenanceMode\"".to_string(), Value::Bool(false)),
            ("\"allowRegistration\"".to_string(), Value::Bool(false)),
            ("version".to_string(), Value::String(String::new())),
        ]),
    };

    Ok(Json(ActionResult::success(Value::Object(data))))
}

pub async fn update_general_control_status(
    pool: Extension<Pool>,
    axum::extract::Json(payload): axum::extract::Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let maintenance_mode: bool = payload.get("\"maintenanceMode\"").and_then(|v| v.as_bool()).unwrap_or(false);
    let allow_registration: bool = payload.get("\"allowRegistration\"").and_then(|v| v.as_bool()).unwrap_or(true);

    let result = client
        .execute(
            "UPDATE x_general_assemble_control_config SET maintenance_mode = $1, allow_registration = $2 WHERE id = 'global'",
            &[&maintenance_mode, &allow_registration],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("\"maintenanceMode\"".to_string(), Value::Bool(maintenance_mode)),
        ("\"allowRegistration\"".to_string(), Value::Bool(allow_registration)),
        ("updated".to_string(), Value::Bool(result > 0)),
    ])))))
}

pub async fn get_module_permissions(
    pool: Extension<Pool>,
    axum::extract::Path(module): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query(
            "SELECT id, module_name, user_id, can_view, can_edit, can_delete FROM x_general_assemble_control_permission WHERE module_name = $1",
            &[&module],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("moduleName".to_string(), Value::String(row.get("module_name"))),
                ("\"userId\"".to_string(), Value::String(row.get("user_id"))),
                ("canView".to_string(), Value::Bool(row.get("can_view"))),
                ("canEdit".to_string(), Value::Bool(row.get("can_edit"))),
                ("canDelete".to_string(), Value::Bool(row.get("can_delete"))),
            ]))
        })
        .collect();

    let count = data.len() as i64;
    Ok(Json(ActionResult::java_success(Value::Array(data), count, 0)))
}

pub fn general_assemble_control_router(pool: Pool) -> Router {
    routes::general_assemble_control_routes(pool)
}

pub fn router(pool: deadpool_postgres::Pool) -> axum::Router {
    crate::routes::general_assemble_control_routes(pool)
}


// ---- attendscope handlers ----

#[derive(Debug, serde::Deserialize)]
pub struct AttendScopeCreateRequest {
    pub name: Option<String>,
    pub unit_id: Option<String>,
}

pub async fn attendscope_list(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query(
            "SELECT id, name, unit_id, creator, create_time FROM x_general_attend_scope ORDER BY create_time DESC",
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
                ("unitId".to_string(), Value::String(row.get("unit_id"))),
                ("creator".to_string(), Value::String(row.get("creator"))),
                ("createTime".to_string(), Value::String(row.get("create_time"))),
            ]))
        })
        .collect();

    let count = data.len() as i64;
    Ok(Json(ActionResult::java_success(Value::Array(data), count, 0)))
}

pub async fn attendscope_get(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let row = client
        .query_opt(
            "SELECT id, name, unit_id, creator, create_time FROM x_general_attend_scope WHERE id = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let result = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("name".to_string(), Value::String(row.get("name"))),
                ("unitId".to_string(), Value::String(row.get("unit_id"))),
                ("creator".to_string(), Value::String(row.get("creator"))),
                ("createTime".to_string(), Value::String(row.get("create_time"))),
            ]));
            Ok(Json(ActionResult::success(result)))
        }
        None => Ok(Json(ActionResult::error("attend scope not found"))),
    }
}

pub async fn attendscope_create(
    pool: Extension<Pool>,
    axum::extract::Json(req): Json<AttendScopeCreateRequest>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let id = uuid::Uuid::new_v4().to_string();
    let name = req.name.unwrap_or_default();
    let unit_id = req.unit_id.unwrap_or_default();
    let creator = "system";

    client
        .execute(
            "INSERT INTO x_general_attend_scope (id, name, unit_id, creator, create_time) VALUES ($1, $2, $3, $4, NOW())",
            &[&id, &name, &unit_id, &creator],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let result = Value::Object(serde_json::Map::from_iter([
        ("id".to_string(), Value::String(id)),
        ("name".to_string(), Value::String(name)),
        ("unitId".to_string(), Value::String(unit_id)),
        ("creator".to_string(), Value::String(creator.to_string())),
    ]));

    Ok(Json(ActionResult::success(result)))
}

pub async fn attendscope_save(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
    axum::extract::Json(req): Json<AttendScopeCreateRequest>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let name = req.name.unwrap_or_default();
    let unit_id = req.unit_id.unwrap_or_default();

    let result = client
        .execute(
            "UPDATE x_general_attend_scope SET name = $1, unit_id = $2 WHERE id = $3",
            &[&name, &unit_id, &id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if result == 0 {
        return Ok(Json(ActionResult::error("attend scope not found")));
    }

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
            ("saved".to_string(), Value::Bool(result > 0)),
            ("name".to_string(), Value::String(name)),
            ("unitId".to_string(), Value::String(unit_id)),
        ]),
    ))))
}

pub async fn attendscope_delete(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let result = client
        .execute(
            "DELETE FROM x_general_attend_scope WHERE id = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if result == 0 {
        return Ok(Json(ActionResult::error("attend scope not found")));
    }

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
            ("deleted".to_string(), Value::Bool(result > 0)),
        ]),
    ))))
}


pub async fn area_list(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query(
            "SELECT id, name, parent_id, level, province, city, district, creator, create_time FROM x_general_assemble_area ORDER BY create_time DESC",
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
                ("\"parentId\"".to_string(), Value::String(row.get("parent_id"))),
                ("level".to_string(), Value::String(row.get("level"))),
                ("province".to_string(), Value::String(row.get("province"))),
                ("city".to_string(), Value::String(row.get("city"))),
                ("district".to_string(), Value::String(row.get("district"))),
                ("creator".to_string(), Value::String(row.get("creator"))),
                ("createTime".to_string(), Value::String(row.get("create_time"))),
            ]))
        })
        .collect();

    let count = data.len() as i64;
    Ok(Json(ActionResult::java_success(Value::Array(data), count, 0)))
}

pub async fn area_list_province_province(
    pool: Extension<Pool>,
    axum::extract::Path(province): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query(
            "SELECT id, name, parent_id, level, province, city, district, creator, create_time FROM x_general_assemble_area WHERE province = $1 ORDER BY create_time DESC",
            &[&province],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("name".to_string(), Value::String(row.get("name"))),
                ("\"parentId\"".to_string(), Value::String(row.get("parent_id"))),
                ("level".to_string(), Value::String(row.get("level"))),
                ("province".to_string(), Value::String(row.get("province"))),
                ("city".to_string(), Value::String(row.get("city"))),
                ("district".to_string(), Value::String(row.get("district"))),
                ("creator".to_string(), Value::String(row.get("creator"))),
                ("createTime".to_string(), Value::String(row.get("create_time"))),
            ]))
        })
        .collect();

    let count = data.len() as i64;
    Ok(Json(ActionResult::java_success(Value::Array(data), count, 0)))
}

pub async fn area_list_province_province_city_city(
    pool: Extension<Pool>,
    axum::extract::Path(province): axum::extract::Path<String>,
    axum::extract::Path(city): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query(
            "SELECT id, name, parent_id, level, province, city, district, creator, create_time FROM x_general_assemble_area WHERE province = $1 AND city = $2 ORDER BY create_time DESC",
            &[&province, &city],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("name".to_string(), Value::String(row.get("name"))),
                ("\"parentId\"".to_string(), Value::String(row.get("parent_id"))),
                ("level".to_string(), Value::String(row.get("level"))),
                ("province".to_string(), Value::String(row.get("province"))),
                ("city".to_string(), Value::String(row.get("city"))),
                ("district".to_string(), Value::String(row.get("district"))),
                ("creator".to_string(), Value::String(row.get("creator"))),
                ("createTime".to_string(), Value::String(row.get("create_time"))),
            ]))
        })
        .collect();

    let count = data.len() as i64;
    Ok(Json(ActionResult::java_success(Value::Array(data), count, 0)))
}

pub async fn area_list_province_province_city_city_district_district(
    pool: Extension<Pool>,
    axum::extract::Path(province): axum::extract::Path<String>,
    axum::extract::Path(city): axum::extract::Path<String>,
    axum::extract::Path(district): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query(
            "SELECT id, name, parent_id, level, province, city, district, creator, create_time FROM x_general_assemble_area WHERE province = $1 AND city = $2 AND district = $3 ORDER BY create_time DESC",
            &[&province, &city, &district],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("name".to_string(), Value::String(row.get("name"))),
                ("\"parentId\"".to_string(), Value::String(row.get("parent_id"))),
                ("level".to_string(), Value::String(row.get("level"))),
                ("province".to_string(), Value::String(row.get("province"))),
                ("city".to_string(), Value::String(row.get("city"))),
                ("district".to_string(), Value::String(row.get("district"))),
                ("creator".to_string(), Value::String(row.get("creator"))),
                ("createTime".to_string(), Value::String(row.get("create_time"))),
            ]))
        })
        .collect();

    let count = data.len() as i64;
    Ok(Json(ActionResult::java_success(Value::Array(data), count, 0)))
}

// ---- area CRUD ----

pub async fn area_create(
    pool: Extension<Pool>,
    axum::extract::Json(payload): axum::extract::Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let id = uuid::Uuid::new_v4().to_string();
    let name = payload.get("name").and_then(|v| v.as_str()).unwrap_or_default().to_string();
    let parent_id = payload.get("\"parentId\"").and_then(|v| v.as_str()).unwrap_or_default().to_string();
    let level = payload.get("level").and_then(|v| v.as_str()).unwrap_or_default().to_string();
    let province = payload.get("province").and_then(|v| v.as_str()).unwrap_or_default().to_string();
    let city = payload.get("city").and_then(|v| v.as_str()).unwrap_or_default().to_string();
    let district = payload.get("district").and_then(|v| v.as_str()).unwrap_or_default().to_string();
    let creator = payload.get("creator").and_then(|v| v.as_str()).unwrap_or("system").to_string();

    client
        .execute(
            "INSERT INTO x_general_assemble_area (id, name, parent_id, level, province, city, district, creator, create_time) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, NOW())",
            &[&id, &name, &parent_id, &level, &province, &city, &district, &creator],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("id".to_string(), Value::String(id)),
        ("name".to_string(), Value::String(name)),
        ("\"parentId\"".to_string(), Value::String(parent_id)),
        ("level".to_string(), Value::String(level)),
        ("province".to_string(), Value::String(province)),
        ("city".to_string(), Value::String(city)),
        ("district".to_string(), Value::String(district)),
    ])))))
}

pub async fn area_get(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let row = client
        .query_opt(
            "SELECT id, name, parent_id, level, province, city, district, creator, create_time FROM x_general_assemble_area WHERE id = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let result = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("name".to_string(), Value::String(row.get("name"))),
                ("\"parentId\"".to_string(), Value::String(row.get("parent_id"))),
                ("level".to_string(), Value::String(row.get("level"))),
                ("province".to_string(), Value::String(row.get("province"))),
                ("city".to_string(), Value::String(row.get("city"))),
                ("district".to_string(), Value::String(row.get("district"))),
                ("creator".to_string(), Value::String(row.get("creator"))),
                ("createTime".to_string(), Value::String(row.get("create_time"))),
            ]));
            Ok(Json(ActionResult::success(result)))
        }
        None => Ok(Json(ActionResult::error("area not found"))),
    }
}

pub async fn area_update(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
    axum::extract::Json(payload): axum::extract::Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let name = payload.get("name").and_then(|v| v.as_str()).unwrap_or_default().to_string();
    let parent_id = payload.get("\"parentId\"").and_then(|v| v.as_str()).unwrap_or_default().to_string();
    let level = payload.get("level").and_then(|v| v.as_str()).unwrap_or_default().to_string();
    let province = payload.get("province").and_then(|v| v.as_str()).unwrap_or_default().to_string();
    let city = payload.get("city").and_then(|v| v.as_str()).unwrap_or_default().to_string();
    let district = payload.get("district").and_then(|v| v.as_str()).unwrap_or_default().to_string();

    let result = client
        .execute(
            "UPDATE x_general_assemble_area SET name = $1, parent_id = $2, level = $3, province = $4, city = $5, district = $6 WHERE id = $7",
            &[&name, &parent_id, &level, &province, &city, &district, &id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if result == 0 {
        return Ok(Json(ActionResult::error("area not found")));
    }

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("id".to_string(), Value::String(id)),
        ("saved".to_string(), Value::Bool(result > 0)),
        ("name".to_string(), Value::String(name)),
    ])))))
}

pub async fn area_delete(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let result = client
        .execute("DELETE FROM x_general_assemble_area WHERE id = $1", &[&id])
        .await
        .map_err(|_| AppError::Internal)?;

    if result == 0 {
        return Ok(Json(ActionResult::error("area not found")));
    }

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("id".to_string(), Value::String(id)),
        ("deleted".to_string(), Value::Bool(result > 0)),
    ])))))
}

pub async fn ecnet_check(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query(
            "SELECT id, name, value, creator, create_time FROM x_general_assemble_ecnet_config ORDER BY create_time DESC",
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
                ("creator".to_string(), Value::String(row.get("creator"))),
                ("createTime".to_string(), Value::String(row.get("create_time"))),
            ]))
        })
        .collect();

    let count = data.len() as i64;
    Ok(Json(ActionResult::java_success(Value::Array(data), count, 0)))
}

pub async fn excel_excelName_excelName(
    pool: Extension<Pool>,
    axum::extract::Path(excel_name): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let row = client
        .query_opt(
            "SELECT id, name, excel_name, creator, create_time FROM x_general_assemble_excel WHERE excel_name = $1",
            &[&excel_name],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let result = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("name".to_string(), Value::String(row.get("name"))),
                ("\"excelName\"".to_string(), Value::String(row.get("excel_name"))),
                ("creator".to_string(), Value::String(row.get("creator"))),
                ("createTime".to_string(), Value::String(row.get("create_time"))),
            ]));
            Ok(Json(ActionResult::success(result)))
        }
        None => Ok(Json(ActionResult::error("excel not found"))),
    }
}

pub async fn excel_excelName_excelName_sheetList(
    pool: Extension<Pool>,
    axum::extract::Path(excel_name): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query(
            "SELECT id, sheet_name, excel_id, creator, create_time FROM x_general_assemble_excel_sheet WHERE excel_id = (SELECT id FROM x_general_assemble_excel WHERE excel_name = $1 LIMIT 1) ORDER BY create_time",
            &[&excel_name],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("sheetName".to_string(), Value::String(row.get("sheet_name"))),
                ("excelId".to_string(), Value::String(row.get("excel_id"))),
                ("creator".to_string(), Value::String(row.get("creator"))),
                ("createTime".to_string(), Value::String(row.get("create_time"))),
            ]))
        })
        .collect();

    let count = data.len() as i64;
    Ok(Json(ActionResult::java_success(Value::Array(data), count, 0)))
}

pub async fn excel_result_flag(
    pool: Extension<Pool>,
    axum::extract::Path(flag): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let row = client
        .query_opt(
            "SELECT id, flag, result, creator, create_time FROM x_general_assemble_excel_result WHERE flag = $1",
            &[&flag],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let result = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("flag".to_string(), Value::String(row.get("flag"))),
                ("result".to_string(), Value::String(row.get("result"))),
                ("creator".to_string(), Value::String(row.get("creator"))),
                ("createTime".to_string(), Value::String(row.get("create_time"))),
            ]));
            Ok(Json(ActionResult::success(result)))
        }
        None => Ok(Json(ActionResult::error("result not found"))),
    }
}

pub async fn excel_upload(
    pool: Extension<Pool>,
    axum::extract::Json(payload): axum::extract::Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let id = uuid::Uuid::new_v4().to_string();
    let name = payload.get("name").and_then(|v| v.as_str()).unwrap_or("").to_string();
    let excel_name = payload.get("\"excelName\"").and_then(|v| v.as_str()).unwrap_or("").to_string();
    let creator = payload.get("creator").and_then(|v| v.as_str()).unwrap_or("system").to_string();
    let flag = uuid::Uuid::new_v4().to_string();

    client
        .execute(
            "INSERT INTO x_general_assemble_excel (id, name, excel_name, creator, create_time, flag) VALUES ($1, $2, $3, $4, NOW(), $5)",
            &[&id, &name, &excel_name, &creator, &flag],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("id".to_string(), Value::String(id)),
        ("name".to_string(), Value::String(name)),
        ("\"excelName\"".to_string(), Value::String(excel_name)),
        ("flag".to_string(), Value::String(flag)),
    ])))))
}

pub async fn excel_upload_with_url(
    pool: Extension<Pool>,
    axum::extract::Json(payload): axum::extract::Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let id = uuid::Uuid::new_v4().to_string();
    let name = payload.get("name").and_then(|v| v.as_str()).unwrap_or("").to_string();
    let excel_name = payload.get("\"excelName\"").and_then(|v| v.as_str()).unwrap_or("").to_string();
    let creator = payload.get("creator").and_then(|v| v.as_str()).unwrap_or("system").to_string();
    let flag = uuid::Uuid::new_v4().to_string();

    client
        .execute(
            "INSERT INTO x_general_assemble_excel (id, name, excel_name, creator, create_time, flag) VALUES ($1, $2, $3, $4, NOW(), $5)",
            &[&id, &name, &excel_name, &creator, &flag],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("id".to_string(), Value::String(id)),
        ("name".to_string(), Value::String(name)),
        ("\"excelName\"".to_string(), Value::String(excel_name)),
        ("flag".to_string(), Value::String(flag)),
    ])))))
}

pub async fn generalfile_download_flag_flag(
    pool: Extension<Pool>,
    axum::extract::Path(flag): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let row = client
        .query_opt(
            "SELECT id, name, flag, content, size, creator, create_time FROM x_general_assemble_general_file WHERE flag = $1",
            &[&flag],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let result = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("name".to_string(), Value::String(row.get("name"))),
                ("flag".to_string(), Value::String(row.get("flag"))),
                ("size".to_string(), Value::String(row.get::<_, i64>("size").to_string())),
                ("creator".to_string(), Value::String(row.get("creator"))),
                ("createTime".to_string(), Value::String(row.get("create_time"))),
            ]));
            Ok(Json(ActionResult::success(result)))
        }
        None => Ok(Json(ActionResult::error("file not found"))),
    }
}

pub async fn generalfile_flag_flag(
    pool: Extension<Pool>,
    axum::extract::Path(flag): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let row = client
        .query_opt(
            "SELECT id, name, flag, content, size, creator, create_time FROM x_general_assemble_general_file WHERE flag = $1",
            &[&flag],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let result = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("name".to_string(), Value::String(row.get("name"))),
                ("flag".to_string(), Value::String(row.get("flag"))),
                ("size".to_string(), Value::String(row.get::<_, i64>("size").to_string())),
                ("creator".to_string(), Value::String(row.get("creator"))),
                ("createTime".to_string(), Value::String(row.get("create_time"))),
            ]));
            Ok(Json(ActionResult::success(result)))
        }
        None => Ok(Json(ActionResult::error("file not found"))),
    }
}

pub async fn generalfile_flag_flag_binary_base64(
    pool: Extension<Pool>,
    axum::extract::Path(flag): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let row = client
        .query_opt(
            "SELECT id, name, flag, content, size, creator, create_time FROM x_general_assemble_general_file WHERE flag = $1",
            &[&flag],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let content: Option<String> = row.get("content");
            let result = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("name".to_string(), Value::String(row.get("name"))),
                ("flag".to_string(), Value::String(row.get("flag"))),
                ("content".to_string(), Value::String(content.unwrap_or_default())),
                ("size".to_string(), Value::String(row.get::<_, i64>("size").to_string())),
                ("creator".to_string(), Value::String(row.get("creator"))),
                ("createTime".to_string(), Value::String(row.get("create_time"))),
            ]));
            Ok(Json(ActionResult::success(result)))
        }
        None => Ok(Json(ActionResult::error("file not found"))),
    }
}

pub async fn invoice_create(
    pool: Extension<Pool>,
    axum::extract::Json(payload): axum::extract::Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let id = uuid::Uuid::new_v4().to_string();
    let name = payload.get("name").and_then(|v| v.as_str()).unwrap_or("").to_string();
    let flag = uuid::Uuid::new_v4().to_string();
    let status = payload.get("status").and_then(|v| v.as_str()).unwrap_or("draft").to_string();
    let creator = payload.get("creator").and_then(|v| v.as_str()).unwrap_or("system").to_string();

    client
        .execute(
            "INSERT INTO x_general_assemble_invoice (id, name, flag, status, creator, create_time) VALUES ($1, $2, $3, $4, $5, NOW())",
            &[&id, &name, &flag, &status, &creator],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("id".to_string(), Value::String(id)),
        ("name".to_string(), Value::String(name)),
        ("flag".to_string(), Value::String(flag)),
        ("status".to_string(), Value::String(status)),
        ("creator".to_string(), Value::String(creator)),
    ])))))
}

pub async fn invoice_delete_id(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let result = client
        .execute("DELETE FROM x_general_assemble_invoice WHERE id = $1", &[&id])
        .await
        .map_err(|_| AppError::Internal)?;

    if result == 0 {
        return Ok(Json(ActionResult::error("invoice not found")));
    }

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("id".to_string(), Value::String(id)),
        ("deleted".to_string(), Value::Bool(result > 0)),
    ])))))
}

pub async fn invoice_download_flag_flag(
    pool: Extension<Pool>,
    axum::extract::Path(flag): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let row = client
        .query_opt(
            "SELECT id, name, flag, status, creator, create_time FROM x_general_assemble_invoice WHERE flag = $1",
            &[&flag],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let result = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("name".to_string(), Value::String(row.get("name"))),
                ("flag".to_string(), Value::String(row.get("flag"))),
                ("status".to_string(), Value::String(row.get("status"))),
                ("creator".to_string(), Value::String(row.get("creator"))),
                ("createTime".to_string(), Value::String(row.get("create_time"))),
            ]));
            Ok(Json(ActionResult::success(result)))
        }
        None => Ok(Json(ActionResult::error("invoice not found"))),
    }
}

pub async fn invoice_get_id(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let row = client
        .query_opt(
            "SELECT id, name, flag, status, creator, create_time FROM x_general_assemble_invoice WHERE id = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let result = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("name".to_string(), Value::String(row.get("name"))),
                ("flag".to_string(), Value::String(row.get("flag"))),
                ("status".to_string(), Value::String(row.get("status"))),
                ("creator".to_string(), Value::String(row.get("creator"))),
                ("createTime".to_string(), Value::String(row.get("create_time"))),
            ]));
            Ok(Json(ActionResult::success(result)))
        }
        None => Ok(Json(ActionResult::error("invoice not found"))),
    }
}

pub async fn invoice_list_paging_page_size_size(
    pool: Extension<Pool>,
    axum::extract::Path(page): axum::extract::Path<i32>,
    axum::extract::Path(size): axum::extract::Path<i32>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let offset = ((page.max(1) - 1) * size) as i64;

    let total_row = client
        .query_one("SELECT COUNT(*) as cnt FROM x_general_assemble_invoice", &[])
        .await
        .map_err(|_| AppError::Internal)?;
    let total: i64 = total_row.get("cnt");

    let rows = client
        .query(
            "SELECT id, name, flag, status, creator, create_time FROM x_general_assemble_invoice ORDER BY create_time DESC LIMIT $1::int OFFSET $2::int",
            &[&size, &offset],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("name".to_string(), Value::String(row.get("name"))),
                ("flag".to_string(), Value::String(row.get("flag"))),
                ("status".to_string(), Value::String(row.get("status"))),
                ("creator".to_string(), Value::String(row.get("creator"))),
                ("createTime".to_string(), Value::String(row.get("create_time"))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::java_success(Value::Array(data), total, size as i64)))
}

pub async fn invoice_update_apply_status_id(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
    axum::extract::Json(payload): axum::extract::Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let status = payload.get("status").and_then(|v| v.as_str()).unwrap_or("").to_string();

    let result = client
        .execute(
            "UPDATE x_general_assemble_invoice SET status = $1 WHERE id = $2",
            &[&status, &id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if result == 0 {
        return Ok(Json(ActionResult::error("invoice not found")));
    }

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("id".to_string(), Value::String(id)),
        ("status".to_string(), Value::String(status)),
        ("updated".to_string(), Value::Bool(result > 0)),
    ])))))
}

pub async fn invoice_update_id(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
    axum::extract::Json(payload): axum::extract::Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let name = payload.get("name").and_then(|v| v.as_str()).unwrap_or_default().to_string();
    let status = payload.get("status").and_then(|v| v.as_str()).unwrap_or_default().to_string();

    let result = client
        .execute(
            "UPDATE x_general_assemble_invoice SET name = $1, status = $2 WHERE id = $3",
            &[&name, &status, &id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if result == 0 {
        return Ok(Json(ActionResult::error("invoice not found")));
    }

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("id".to_string(), Value::String(id)),
        ("name".to_string(), Value::String(name)),
        ("status".to_string(), Value::String(status)),
        ("updated".to_string(), Value::Bool(result > 0)),
    ])))))
}

pub async fn invoice_upload(
    pool: Extension<Pool>,
    axum::extract::Json(payload): axum::extract::Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let id = uuid::Uuid::new_v4().to_string();
    let name = payload.get("name").and_then(|v| v.as_str()).unwrap_or("").to_string();
    let flag = uuid::Uuid::new_v4().to_string();
    let creator = payload.get("creator").and_then(|v| v.as_str()).unwrap_or("system").to_string();

    client
        .execute(
            "INSERT INTO x_general_assemble_invoice (id, name, flag, status, creator, create_time) VALUES ($1, $2, $3, 'draft', $4, NOW())",
            &[&id, &name, &flag, &creator],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("id".to_string(), Value::String(id)),
        ("name".to_string(), Value::String(name)),
        ("flag".to_string(), Value::String(flag)),
        ("status".to_string(), Value::String("draft".to_string())),
    ])))))
}

pub async fn invoice_upload_for_create(
    pool: Extension<Pool>,
    axum::extract::Json(payload): axum::extract::Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let id = uuid::Uuid::new_v4().to_string();
    let name = payload.get("name").and_then(|v| v.as_str()).unwrap_or("").to_string();
    let flag = uuid::Uuid::new_v4().to_string();
    let creator = payload.get("creator").and_then(|v| v.as_str()).unwrap_or("system").to_string();

    client
        .execute(
            "INSERT INTO x_general_assemble_invoice (id, name, flag, status, creator, create_time) VALUES ($1, $2, $3, 'draft', $4, NOW())",
            &[&id, &name, &flag, &creator],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("id".to_string(), Value::String(id)),
        ("name".to_string(), Value::String(name)),
        ("flag".to_string(), Value::String(flag)),
        ("status".to_string(), Value::String("draft".to_string())),
    ])))))
}

pub async fn invoice_upload_with_url(
    pool: Extension<Pool>,
    axum::extract::Json(payload): axum::extract::Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let id = uuid::Uuid::new_v4().to_string();
    let name = payload.get("name").and_then(|v| v.as_str()).unwrap_or("").to_string();
    let flag = uuid::Uuid::new_v4().to_string();
    let creator = payload.get("creator").and_then(|v| v.as_str()).unwrap_or("system").to_string();

    client
        .execute(
            "INSERT INTO x_general_assemble_invoice (id, name, flag, status, creator, create_time) VALUES ($1, $2, $3, 'draft', $4, NOW())",
            &[&id, &name, &flag, &creator],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("id".to_string(), Value::String(id)),
        ("name".to_string(), Value::String(name)),
        ("flag".to_string(), Value::String(flag)),
        ("status".to_string(), Value::String("draft".to_string())),
    ])))))
}

pub async fn office_html_to_word(
    pool: Extension<Pool>,
    axum::extract::Json(payload): axum::extract::Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let id = uuid::Uuid::new_v4().to_string();
    let html_content = payload.get("\"htmlContent\"").and_then(|v| v.as_str()).unwrap_or("").to_string();
    let creator = payload.get("creator").and_then(|v| v.as_str()).unwrap_or("system").to_string();
    let word_flag = uuid::Uuid::new_v4().to_string();

    client
        .execute(
            "INSERT INTO x_general_assemble_office (id, html_content, word_flag, creator, create_time) VALUES ($1, $2, $3, $4, NOW())",
            &[&id, &html_content, &word_flag, &creator],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("id".to_string(), Value::String(id)),
        ("wordFlag".to_string(), Value::String(word_flag)),
    ])))))
}

pub async fn office_html_to_word_result_flag(
    pool: Extension<Pool>,
    axum::extract::Path(flag): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let row = client
        .query_opt(
            "SELECT id, word_flag, html_content, creator, create_time FROM x_general_assemble_office WHERE word_flag = $1",
            &[&flag],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let result = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("wordFlag".to_string(), Value::String(row.get("word_flag"))),
                ("\"htmlContent\"".to_string(), Value::String(row.get("html_content"))),
                ("creator".to_string(), Value::String(row.get("creator"))),
                ("createTime".to_string(), Value::String(row.get("create_time"))),
            ]));
            Ok(Json(ActionResult::success(result)))
        }
        None => Ok(Json(ActionResult::error("conversion result not found"))),
    }
}

pub async fn qrcode_width_width_height_height_text_text(
    pool: Extension<Pool>,
    axum::extract::Path(width): axum::extract::Path<u32>,
    axum::extract::Path(height): axum::extract::Path<u32>,
    axum::extract::Path(text): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let id = uuid::Uuid::new_v4().to_string();
    let content = text.clone();
    let creator = "system";

    client
        .execute(
            "INSERT INTO x_general_assemble_qrcode (id, width, height, text, content, creator, create_time) VALUES ($1, $2, $3, $4, $5, $6, NOW())",
            &[&id, &(width as i32), &(height as i32), &text, &content, &creator],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("id".to_string(), Value::String(id)),
        ("width".to_string(), Value::Number(serde_json::Number::from(width))),
        ("height".to_string(), Value::Number(serde_json::Number::from(height))),
        ("text".to_string(), Value::String(text)),
    ])))))
}

// ---- qrcode CRUD ----

pub async fn qrcode_list(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query(
            "SELECT id, width, height, text, content, creator, create_time FROM x_general_assemble_qrcode ORDER BY create_time DESC",
            &[],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("width".to_string(), Value::Number(serde_json::Number::from(row.get::<_, i32>("width")))),
                ("height".to_string(), Value::Number(serde_json::Number::from(row.get::<_, i32>("height")))),
                ("text".to_string(), Value::String(row.get("text"))),
                ("creator".to_string(), Value::String(row.get("creator"))),
                ("createTime".to_string(), Value::String(row.get("create_time"))),
            ]))
        })
        .collect();

    let count = data.len() as i64;
    Ok(Json(ActionResult::java_success(Value::Array(data), count, 0)))
}

pub async fn qrcode_get(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let row = client
        .query_opt(
            "SELECT id, width, height, text, content, creator, create_time FROM x_general_assemble_qrcode WHERE id = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let result = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("width".to_string(), Value::Number(serde_json::Number::from(row.get::<_, i32>("width")))),
                ("height".to_string(), Value::Number(serde_json::Number::from(row.get::<_, i32>("height")))),
                ("text".to_string(), Value::String(row.get("text"))),
                ("content".to_string(), Value::String(row.get("content"))),
                ("creator".to_string(), Value::String(row.get("creator"))),
                ("createTime".to_string(), Value::String(row.get("create_time"))),
            ]));
            Ok(Json(ActionResult::success(result)))
        }
        None => Ok(Json(ActionResult::error("qrcode not found"))),
    }
}

pub async fn qrcode_delete(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let result = client
        .execute("DELETE FROM x_general_assemble_qrcode WHERE id = $1", &[&id])
        .await
        .map_err(|_| AppError::Internal)?;

    if result == 0 {
        return Ok(Json(ActionResult::error("qrcode not found")));
    }

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("id".to_string(), Value::String(id)),
        ("deleted".to_string(), Value::Bool(result > 0)),
    ])))))
}

pub async fn securityclearance_enable(
    pool: Extension<Pool>,
    axum::extract::Json(payload): axum::extract::Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let enabled = payload.get("enabled").and_then(|v| v.as_bool()).unwrap_or(false);
    let name = payload.get("name").and_then(|v| v.as_str()).unwrap_or("").to_string();

    let row = client
        .query_opt(
            "SELECT id FROM x_general_assemble_security_clearance WHERE name = $1 LIMIT 1",
            &[&name],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let id: String = row.get("id");
            client
                .execute(
                    "UPDATE x_general_assemble_security_clearance SET enabled = $1 WHERE id = $2",
                    &[&enabled, &id],
                )
                .await
                .map_err(|_| AppError::Internal)?;
            Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(id)),
                ("name".to_string(), Value::String(name)),
                ("enabled".to_string(), Value::Bool(enabled)),
            ])))))
        }
        None => {
            let id = uuid::Uuid::new_v4().to_string();
            client
                .execute(
                    "INSERT INTO x_general_assemble_security_clearance (id, name, enabled, creator, create_time) VALUES ($1, $2, $3, 'system', NOW())",
                    &[&id, &name, &enabled],
                )
                .await
                .map_err(|_| AppError::Internal)?;
            Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(id)),
                ("name".to_string(), Value::String(name)),
                ("enabled".to_string(), Value::Bool(enabled)),
            ])))))
        }
    }
}

pub async fn securityclearance_object(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query(
            "SELECT id, name, type, enabled, subject, creator, create_time FROM x_general_assemble_security_clearance WHERE type = 'object' ORDER BY create_time DESC",
            &[],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let mut map = serde_json::Map::new();
    for row in &rows {
        let name: String = row.get("name");
        let id: String = row.get("id");
        map.insert(name, Value::String(id));
    }

    Ok(Json(ActionResult::success(Value::Object(map))))
}

pub async fn securityclearance_subject(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query(
            "SELECT id, name, type, enabled, object, creator, create_time FROM x_general_assemble_security_clearance WHERE type = 'subject' ORDER BY create_time DESC",
            &[],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let mut map = serde_json::Map::new();
    for row in &rows {
        let name: String = row.get("name");
        let id: String = row.get("id");
        map.insert(name, Value::String(id));
    }

    Ok(Json(ActionResult::success(Value::Object(map))))
}

pub async fn securityclearance_system(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query(
            "SELECT id, name, type, enabled, creator, create_time FROM x_general_assemble_security_clearance WHERE type = 'system' ORDER BY create_time DESC",
            &[],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let mut map = serde_json::Map::new();
    for row in &rows {
        let name: String = row.get("name");
        let id: String = row.get("id");
        map.insert(name, Value::String(id));
    }

    Ok(Json(ActionResult::success(Value::Object(map))))
}

// ---- securityclearance CRUD ----

pub async fn securityclearance_create(
    pool: Extension<Pool>,
    axum::extract::Json(payload): axum::extract::Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let id = uuid::Uuid::new_v4().to_string();
    let name = payload.get("name").and_then(|v| v.as_str()).unwrap_or_default().to_string();
    let clearance_type = payload.get("type").and_then(|v| v.as_str()).unwrap_or("system").to_string();
    let enabled = payload.get("enabled").and_then(|v| v.as_bool()).unwrap_or(false);
    let subject = payload.get("subject").and_then(|v| v.as_str()).unwrap_or_default().to_string();
    let object = payload.get("object").and_then(|v| v.as_str()).unwrap_or_default().to_string();
    let creator = payload.get("creator").and_then(|v| v.as_str()).unwrap_or("system").to_string();

    client
        .execute(
            "INSERT INTO x_general_assemble_security_clearance (id, name, type, enabled, subject, object, creator, create_time) VALUES ($1, $2, $3, $4, $5, $6, $7, NOW())",
            &[&id, &name, &clearance_type, &enabled, &subject, &object, &creator],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("id".to_string(), Value::String(id)),
        ("name".to_string(), Value::String(name)),
        ("type".to_string(), Value::String(clearance_type)),
        ("enabled".to_string(), Value::Bool(enabled)),
    ])))))
}

pub async fn securityclearance_get(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let row = client
        .query_opt(
            "SELECT id, name, type, enabled, subject, object, creator, create_time FROM x_general_assemble_security_clearance WHERE id = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let result = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("name".to_string(), Value::String(row.get("name"))),
                ("type".to_string(), Value::String(row.get("type"))),
                ("enabled".to_string(), Value::Bool(row.get("enabled"))),
                ("subject".to_string(), Value::String(row.get("subject"))),
                ("object".to_string(), Value::String(row.get("object"))),
                ("creator".to_string(), Value::String(row.get("creator"))),
                ("createTime".to_string(), Value::String(row.get("create_time"))),
            ]));
            Ok(Json(ActionResult::success(result)))
        }
        None => Ok(Json(ActionResult::error("security clearance not found"))),
    }
}

pub async fn securityclearance_update(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
    axum::extract::Json(payload): axum::extract::Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let name = payload.get("name").and_then(|v| v.as_str()).unwrap_or_default().to_string();
    let clearance_type = payload.get("type").and_then(|v| v.as_str()).unwrap_or_default().to_string();
    let enabled = payload.get("enabled").and_then(|v| v.as_bool()).unwrap_or(false);
    let subject = payload.get("subject").and_then(|v| v.as_str()).unwrap_or_default().to_string();
    let object = payload.get("object").and_then(|v| v.as_str()).unwrap_or_default().to_string();

    let result = client
        .execute(
            "UPDATE x_general_assemble_security_clearance SET name = $1, type = $2, enabled = $3, subject = $4, object = $5 WHERE id = $6",
            &[&name, &clearance_type, &enabled, &subject, &object, &id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if result == 0 {
        return Ok(Json(ActionResult::error("security clearance not found")));
    }

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("id".to_string(), Value::String(id)),
        ("saved".to_string(), Value::Bool(result > 0)),
        ("name".to_string(), Value::String(name)),
    ])))))
}

pub async fn securityclearance_delete(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let result = client
        .execute("DELETE FROM x_general_assemble_security_clearance WHERE id = $1", &[&id])
        .await
        .map_err(|_| AppError::Internal)?;

    if result == 0 {
        return Ok(Json(ActionResult::error("security clearance not found")));
    }

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("id".to_string(), Value::String(id)),
        ("deleted".to_string(), Value::Bool(result > 0)),
    ])))))
}

pub async fn upgrade_2021090901(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let row = client
        .query_opt(
            "SELECT id FROM x_general_assemble_upgrade WHERE version = '2021090901' LIMIT 1",
            &[],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let has_record = row.is_some();
    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("value".to_string(), Value::Bool(has_record)),
    ])))))
}

pub async fn upgrade_2021090902(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let row = client
        .query_opt(
            "SELECT id FROM x_general_assemble_upgrade WHERE version = '2021090902' LIMIT 1",
            &[],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let has_record = row.is_some();
    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("value".to_string(), Value::Bool(has_record)),
    ])))))
}

pub async fn worktime_betweenholidaycount_start_startDate_end_endDate(
    pool: Extension<Pool>,
    axum::extract::Path(start_date): axum::extract::Path<String>,
    axum::extract::Path(end_date): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let row = client
        .query_one(
            "SELECT COUNT(*) as cnt FROM x_general_assemble_worktime WHERE is_holiday = true AND date >= $1 AND date <= $2",
            &[&start_date, &end_date],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    let count: i64 = row.get("cnt");

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(count))),
        ("startDate".to_string(), Value::String(start_date)),
        ("endDate".to_string(), Value::String(end_date)),
    ])))))
}

pub async fn worktime_betweenminutes_start_start_end_end(
    pool: Extension<Pool>,
    axum::extract::Path(start): axum::extract::Path<String>,
    axum::extract::Path(end): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let row = client
        .query_one(
            "SELECT SUM(minutes) as total FROM x_general_assemble_worktime WHERE date >= $1 AND date <= $2 AND is_worktime = true",
            &[&start, &end],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    let total: Option<i64> = row.get("total");

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("minutes".to_string(), Value::Number(serde_json::Number::from(total.unwrap_or(0)))),
        ("start".to_string(), Value::String(start)),
        ("end".to_string(), Value::String(end)),
    ])))))
}

pub async fn worktime_forwarddays_start_start_days_days(
    pool: Extension<Pool>,
    axum::extract::Path(start): axum::extract::Path<String>,
    axum::extract::Path(days): axum::extract::Path<u32>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let end_date = format!("{} + {} days", start, days);

    let rows = client
        .query(
            "SELECT id, date, is_holiday, is_workday, is_worktime, minutes, creator, create_time FROM x_general_assemble_worktime WHERE date >= $1 AND date <= $2 ORDER BY date",
            &[&start, &end_date],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("date".to_string(), Value::String(row.get("date"))),
                ("isHoliday".to_string(), Value::Bool(row.get("is_holiday"))),
                ("isWorkday".to_string(), Value::Bool(row.get("is_workday"))),
                ("isWorktime".to_string(), Value::Bool(row.get("is_worktime"))),
                ("minutes".to_string(), Value::String(row.get::<_, i64>("minutes").to_string())),
                ("creator".to_string(), Value::String(row.get("creator"))),
                ("createTime".to_string(), Value::String(row.get("create_time"))),
            ]))
        })
        .collect();

    let count = data.len() as i64;
    Ok(Json(ActionResult::java_success(Value::Array(data), count, 0)))
}

pub async fn worktime_forwardminutes_start_start_minutes_minutes(
    pool: Extension<Pool>,
    axum::extract::Path(start): axum::extract::Path<String>,
    axum::extract::Path(minutes): axum::extract::Path<i64>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query(
            "SELECT id, date, is_holiday, is_workday, is_worktime, minutes, creator, create_time FROM x_general_assemble_worktime WHERE date >= $1 AND is_worktime = true ORDER BY date",
            &[&start],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let mut worktime_records = Vec::new();
    let mut accumulated: i64 = 0;

    for row in &rows {
        let record_minutes: i64 = row.get("minutes");
        accumulated += record_minutes;
        worktime_records.push(Value::Object(serde_json::Map::from_iter([
            ("id".to_string(), Value::String(row.get("id"))),
            ("date".to_string(), Value::String(row.get("date"))),
            ("isHoliday".to_string(), Value::Bool(row.get("is_holiday"))),
            ("isWorkday".to_string(), Value::Bool(row.get("is_workday"))),
            ("isWorktime".to_string(), Value::Bool(row.get("is_worktime"))),
            ("minutes".to_string(), Value::String(record_minutes.to_string())),
        ])));
        if accumulated >= minutes {
            break;
        }
    }

    let count = worktime_records.len() as i64;
    Ok(Json(ActionResult::java_success(Value::Array(worktime_records), count, 0)))
}

pub async fn worktime_indefinedholiday_date(
    pool: Extension<Pool>,
    axum::extract::Path(date): axum::extract::Path<String>,
) -> Result<Json<Value>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let row = client
        .query_opt(
            "SELECT id, date, is_holiday, creator, create_time FROM x_general_assemble_worktime WHERE date = $1 LIMIT 1",
            &[&date],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let is_holiday: bool = row.get("is_holiday");
            Ok(Json(Value::Object(serde_json::Map::from_iter([
                ("date".to_string(), Value::String(row.get("date"))),
                ("isHoliday".to_string(), Value::Bool(is_holiday)),
                ("indefined".to_string(), Value::Bool(false)),
            ]))))
        }
        None => {
            let weekday = if date.len() == 10 && date.chars().nth(4) == Some('-') && date.chars().nth(7) == Some('-') {
                let year = date[0..4].parse::<i32>().unwrap_or(2000);
                let month = date[5..7].parse::<u32>().unwrap_or(1);
                let day = date[8..10].parse::<u32>().unwrap_or(1);
                let total_days = (year - 2000) * 365 + (month as i32 - 1) * 30 + day as i32;
                Some((total_days % 7) as u8)
            } else {
                None
            };
            let is_weekend = weekday.map(|w| w >= 5).unwrap_or(false);
            let is_holiday = is_weekend;
            let indefined = weekday.is_none();
            Ok(Json(Value::Object(serde_json::Map::from_iter([
                ("date".to_string(), Value::String(date)),
                ("isHoliday".to_string(), Value::Bool(is_holiday)),
                ("indefined".to_string(), Value::Bool(indefined)),
            ]))))
        }
    }
}

pub async fn worktime_indefinedworkday_date(
    pool: Extension<Pool>,
    axum::extract::Path(date): axum::extract::Path<String>,
) -> Result<Json<Value>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let row = client
        .query_opt(
            "SELECT id, date, is_workday, creator, create_time FROM x_general_assemble_worktime WHERE date = $1 LIMIT 1",
            &[&date],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let is_workday: bool = row.get("is_workday");
            Ok(Json(Value::Object(serde_json::Map::from_iter([
                ("date".to_string(), Value::String(row.get("date"))),
                ("isWorkday".to_string(), Value::Bool(is_workday)),
                ("indefined".to_string(), Value::Bool(false)),
            ]))))
        }
        None => {
            let weekday = if date.len() == 10 && date.chars().nth(4) == Some('-') && date.chars().nth(7) == Some('-') {
                let year = date[0..4].parse::<i32>().unwrap_or(2000);
                let month = date[5..7].parse::<u32>().unwrap_or(1);
                let day = date[8..10].parse::<u32>().unwrap_or(1);
                let total_days = (year - 2000) * 365 + (month as i32 - 1) * 30 + day as i32;
                Some((total_days % 7) as u8)
            } else {
                None
            };
            let is_weekend = weekday.map(|w| w >= 5).unwrap_or(false);
            let is_workday = !is_weekend;
            let indefined = weekday.is_none();
            Ok(Json(Value::Object(serde_json::Map::from_iter([
                ("date".to_string(), Value::String(date)),
                ("isWorkday".to_string(), Value::Bool(is_workday)),
                ("indefined".to_string(), Value::Bool(indefined)),
            ]))))
        }
    }
}

pub async fn worktime_isholiday_date(
    pool: Extension<Pool>,
    axum::extract::Path(date): axum::extract::Path<String>,
) -> Result<Json<Value>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let row = client
        .query_opt(
            "SELECT id, date, is_holiday, creator, create_time FROM x_general_assemble_worktime WHERE date = $1 LIMIT 1",
            &[&date],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let is_holiday: bool = row.get("is_holiday");
            Ok(Json(Value::Object(serde_json::Map::from_iter([
                ("date".to_string(), Value::String(row.get("date"))),
                ("isHoliday".to_string(), Value::Bool(is_holiday)),
                ("indefined".to_string(), Value::Bool(false)),
            ]))))
        }
        None => {
            let weekday = if date.len() == 10 && date.chars().nth(4) == Some('-') && date.chars().nth(7) == Some('-') {
                let year = date[0..4].parse::<i32>().unwrap_or(2000);
                let month = date[5..7].parse::<u32>().unwrap_or(1);
                let day = date[8..10].parse::<u32>().unwrap_or(1);
                let total_days = (year - 2000) * 365 + (month as i32 - 1) * 30 + day as i32;
                Some((total_days % 7) as u8)
            } else {
                None
            };
            let is_weekend = weekday.map(|w| w >= 5).unwrap_or(false);
            let indefined = weekday.is_none();
            Ok(Json(Value::Object(serde_json::Map::from_iter([
                ("date".to_string(), Value::String(date)),
                ("isHoliday".to_string(), Value::Bool(is_weekend)),
                ("indefined".to_string(), Value::Bool(indefined)),
            ]))))
        }
    }
}

pub async fn worktime_isworkday_date(
    pool: Extension<Pool>,
    axum::extract::Path(date): axum::extract::Path<String>,
) -> Result<Json<Value>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let row = client
        .query_opt(
            "SELECT id, date, is_workday, creator, create_time FROM x_general_assemble_worktime WHERE date = $1 LIMIT 1",
            &[&date],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let is_workday: bool = row.get("is_workday");
            Ok(Json(Value::Object(serde_json::Map::from_iter([
                ("date".to_string(), Value::String(row.get("date"))),
                ("isWorkday".to_string(), Value::Bool(is_workday)),
                ("indefined".to_string(), Value::Bool(false)),
            ]))))
        }
        None => {
            let weekday = if date.len() == 10 && date.chars().nth(4) == Some('-') && date.chars().nth(7) == Some('-') {
                let year = date[0..4].parse::<i32>().unwrap_or(2000);
                let month = date[5..7].parse::<u32>().unwrap_or(1);
                let day = date[8..10].parse::<u32>().unwrap_or(1);
                let total_days = (year - 2000) * 365 + (month as i32 - 1) * 30 + day as i32;
                Some((total_days % 7) as u8)
            } else {
                None
            };
            let is_weekend = weekday.map(|w| w >= 5).unwrap_or(false);
            let indefined = weekday.is_none();
            Ok(Json(Value::Object(serde_json::Map::from_iter([
                ("date".to_string(), Value::String(date)),
                ("isWorkday".to_string(), Value::Bool(!is_weekend)),
                ("indefined".to_string(), Value::Bool(indefined)),
            ]))))
        }
    }
}

pub async fn worktime_isworktime_date(
    pool: Extension<Pool>,
    axum::extract::Path(date): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let row = client
        .query_opt(
            "SELECT id, date, is_worktime, minutes, creator, create_time FROM x_general_assemble_worktime WHERE date = $1 LIMIT 1",
            &[&date],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let is_worktime: bool = row.get("is_worktime");
            Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
                ("date".to_string(), Value::String(row.get("date"))),
                ("isWorktime".to_string(), Value::Bool(is_worktime)),
                ("indefined".to_string(), Value::Bool(false)),
            ])))))
        }
        None => {
            Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
                ("date".to_string(), Value::String(date)),
                ("isWorktime".to_string(), Value::Bool(false)),
                ("indefined".to_string(), Value::Bool(true)),
            ])))))
        }
    }
}

pub async fn worktime_minutesofworkday(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let row = client
        .query_one(
            "SELECT SUM(minutes) as total FROM x_general_assemble_worktime WHERE is_worktime = true",
            &[],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    let total: Option<i64> = row.get("total");

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("minutes".to_string(), Value::Number(serde_json::Number::from(total.unwrap_or(0)))),
    ])))))
}

pub async fn generalfile_create(
    pool: Extension<Pool>,
    axum::extract::Json(req): axum::extract::Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let id = uuid::Uuid::new_v4().to_string();
    let name = req.get("name").and_then(|v| v.as_str()).unwrap_or("");
    let flag = req.get("flag").and_then(|v| v.as_str()).unwrap_or("");
    let content = req.get("content").and_then(|v| v.as_str()).unwrap_or("");
    let size = req.get("size").and_then(|v| v.as_i64()).unwrap_or(0);
    let creator = req.get("creator").and_then(|v| v.as_str()).unwrap_or("system");
    client
        .execute(
            "INSERT INTO x_general_assemble_general_file (id, name, flag, content, size, creator, create_time) VALUES ($1, $2, $3, $4, $5, $6, NOW())",
            &[&id, &name, &flag, &content, &size, &creator],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    Ok(Json(ActionResult::success(serde_json::json!({
        "id": id, "name": name, "flag": flag
    }))))
}

pub async fn qrcode_create(
    pool: Extension<Pool>,
    axum::extract::Json(req): axum::extract::Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let id = uuid::Uuid::new_v4().to_string();
    let width = req.get("width").and_then(|v| v.as_i64()).unwrap_or(200);
    let height = req.get("height").and_then(|v| v.as_i64()).unwrap_or(200);
    let text = req.get("text").and_then(|v| v.as_str()).unwrap_or("");
    let content = req.get("content").and_then(|v| v.as_str()).unwrap_or("");
    let creator = req.get("creator").and_then(|v| v.as_str()).unwrap_or("system");
    client
        .execute(
            "INSERT INTO x_general_assemble_qrcode (id, width, height, text, content, creator, create_time) VALUES ($1, $2, $3, $4, $5, $6, NOW())",
            &[&id, &width, &height, &text, &content, &creator],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    Ok(Json(ActionResult::success(serde_json::json!({
        "id": id, "text": text
    }))))
}
