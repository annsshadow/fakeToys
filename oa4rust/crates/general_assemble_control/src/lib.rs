use axum::{
    extract::Extension,
    Json, Router,
};
use deadpool_postgres::Pool;
use serde_json::Value;
use shared::{error::AppError, response::ActionResult};

pub mod routes;

#[cfg(test)]
mod tests;

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
            ("maintenanceMode".to_string(), Value::Bool(r.get("maintenance_mode"))),
            ("allowRegistration".to_string(), Value::Bool(r.get("allow_registration"))),
            ("version".to_string(), Value::String(r.get("version"))),
        ]),
        Err(_) => serde_json::Map::from_iter([
            ("id".to_string(), Value::String(String::new())),
            ("systemName".to_string(), Value::String(String::new())),
            ("maintenanceMode".to_string(), Value::Bool(false)),
            ("allowRegistration".to_string(), Value::Bool(true)),
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

    let maintenance_mode: bool = payload.get("maintenanceMode").and_then(|v| v.as_bool()).unwrap_or(false);
    let allow_registration: bool = payload.get("allowRegistration").and_then(|v| v.as_bool()).unwrap_or(true);

    client
        .execute(
            "UPDATE x_general_assemble_control_config SET maintenance_mode = $1, allow_registration = $2 WHERE id = 'global'",
            &[&maintenance_mode, &allow_registration],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("maintenanceMode".to_string(), Value::Bool(maintenance_mode)),
        ("allowRegistration".to_string(), Value::Bool(allow_registration)),
        ("updated".to_string(), Value::Bool(true)),
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
                ("userId".to_string(), Value::String(row.get("user_id"))),
                ("canView".to_string(), Value::Bool(row.get("can_view"))),
                ("canEdit".to_string(), Value::Bool(row.get("can_edit"))),
                ("canDelete".to_string(), Value::Bool(row.get("can_delete"))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("module".to_string(), Value::String(module)),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

pub fn general_assemble_control_router(pool: Pool) -> Router {
    routes::general_assemble_control_routes(pool)
}

pub fn router(_pool: deadpool_postgres::Pool) -> axum::Router {
    axum::Router::new()
        .route("/general_assemble_control/health", axum::routing::get(|| async { "TODO: general_assemble_control - real implementation needed" }))
}


/// Stub handler for /jaxrs/general/assemble/control/area/list
/// TODO: Implement real business logic
pub async fn stub_general_assemble_control_area_list() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Null)))
}

/// Stub handler for /jaxrs/general/assemble/control/area/list/province/{province}
/// TODO: Implement real business logic
pub async fn stub_general_assemble_control_area_list_province_province() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Null)))
}

/// Stub handler for /jaxrs/general/assemble/control/area/list/province/{province}/city/{city}
/// TODO: Implement real business logic
pub async fn stub_general_assemble_control_area_list_province_province_city_city() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Null)))
}

/// Stub handler for /jaxrs/general/assemble/control/area/list/province/{province}/city/{city}/district/{district}
/// TODO: Implement real business logic
pub async fn stub_general_assemble_control_area_list_province_province_city_city_district_district() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Null)))
}

/// Stub handler for /jaxrs/general/assemble/control/ecnet/check
/// TODO: Implement real business logic
pub async fn stub_general_assemble_control_ecnet_check() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Null)))
}

/// Stub handler for /jaxrs/general/assemble/control/excel/excelName/{excelName}
/// TODO: Implement real business logic
pub async fn stub_general_assemble_control_excel_excelName_excelName() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Null)))
}

/// Stub handler for /jaxrs/general/assemble/control/excel/excelName/{excelName}/sheetList
/// TODO: Implement real business logic
pub async fn stub_general_assemble_control_excel_excelName_excelName_sheetList() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Null)))
}

/// Stub handler for /jaxrs/general/assemble/control/excel/result/{flag}
/// TODO: Implement real business logic
pub async fn stub_general_assemble_control_excel_result_flag() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Null)))
}

/// Stub handler for /jaxrs/general/assemble/control/excel/upload
/// TODO: Implement real business logic
pub async fn stub_general_assemble_control_excel_upload() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Null)))
}

/// Stub handler for /jaxrs/general/assemble/control/excel/upload/with/url
/// TODO: Implement real business logic
pub async fn stub_general_assemble_control_excel_upload_with_url() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Null)))
}

/// Stub handler for /jaxrs/general/assemble/control/generalfile/download/flag/{flag}
/// TODO: Implement real business logic
pub async fn stub_general_assemble_control_generalfile_download_flag_flag() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Null)))
}

/// Stub handler for /jaxrs/general/assemble/control/generalfile/flag/{flag}
/// TODO: Implement real business logic
pub async fn stub_general_assemble_control_generalfile_flag_flag() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Null)))
}

/// Stub handler for /jaxrs/general/assemble/control/generalfile/flag/{flag}/binary/base64
/// TODO: Implement real business logic
pub async fn stub_general_assemble_control_generalfile_flag_flag_binary_base64() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Null)))
}

/// Stub handler for /jaxrs/general/assemble/control/invoice/create
/// TODO: Implement real business logic
pub async fn stub_general_assemble_control_invoice_create() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Null)))
}

/// Stub handler for /jaxrs/general/assemble/control/invoice/delete/{id}
/// TODO: Implement real business logic
pub async fn stub_general_assemble_control_invoice_delete_id() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Null)))
}

/// Stub handler for /jaxrs/general/assemble/control/invoice/download/flag/{flag}
/// TODO: Implement real business logic
pub async fn stub_general_assemble_control_invoice_download_flag_flag() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Null)))
}

/// Stub handler for /jaxrs/general/assemble/control/invoice/get/{id}
/// TODO: Implement real business logic
pub async fn stub_general_assemble_control_invoice_get_id() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Null)))
}

/// Stub handler for /jaxrs/general/assemble/control/invoice/list/paging/{page}/size/{size}
/// TODO: Implement real business logic
pub async fn stub_general_assemble_control_invoice_list_paging_page_size_size() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Null)))
}

/// Stub handler for /jaxrs/general/assemble/control/invoice/update/apply/status/{id}
/// TODO: Implement real business logic
pub async fn stub_general_assemble_control_invoice_update_apply_status_id() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Null)))
}

/// Stub handler for /jaxrs/general/assemble/control/invoice/update/{id}
/// TODO: Implement real business logic
pub async fn stub_general_assemble_control_invoice_update_id() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Null)))
}

/// Stub handler for /jaxrs/general/assemble/control/invoice/upload
/// TODO: Implement real business logic
pub async fn stub_general_assemble_control_invoice_upload() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Null)))
}

/// Stub handler for /jaxrs/general/assemble/control/invoice/upload/for/create
/// TODO: Implement real business logic
pub async fn stub_general_assemble_control_invoice_upload_for_create() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Null)))
}

/// Stub handler for /jaxrs/general/assemble/control/invoice/upload/with/url
/// TODO: Implement real business logic
pub async fn stub_general_assemble_control_invoice_upload_with_url() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Null)))
}

/// Stub handler for /jaxrs/general/assemble/control/office/html/to/word
/// TODO: Implement real business logic
pub async fn stub_general_assemble_control_office_html_to_word() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Null)))
}

/// Stub handler for /jaxrs/general/assemble/control/office/html/to/word/result/{flag}
/// TODO: Implement real business logic
pub async fn stub_general_assemble_control_office_html_to_word_result_flag() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Null)))
}

/// Stub handler for /jaxrs/general/assemble/control/qrcode/width/{width}/height/{height}/text/{text}
/// TODO: Implement real business logic
pub async fn stub_general_assemble_control_qrcode_width_width_height_height_text_text() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Null)))
}

/// Stub handler for /jaxrs/general/assemble/control/securityclearance/enable
/// TODO: Implement real business logic
pub async fn stub_general_assemble_control_securityclearance_enable() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Null)))
}

/// Stub handler for /jaxrs/general/assemble/control/securityclearance/object
/// TODO: Implement real business logic
pub async fn stub_general_assemble_control_securityclearance_object() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Null)))
}

/// Stub handler for /jaxrs/general/assemble/control/securityclearance/subject
/// TODO: Implement real business logic
pub async fn stub_general_assemble_control_securityclearance_subject() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Null)))
}

/// Stub handler for /jaxrs/general/assemble/control/securityclearance/system
/// TODO: Implement real business logic
pub async fn stub_general_assemble_control_securityclearance_system() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Null)))
}

/// Stub handler for /jaxrs/general/assemble/control/upgrade/2021090901
/// TODO: Implement real business logic
pub async fn stub_general_assemble_control_upgrade_2021090901() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Null)))
}

/// Stub handler for /jaxrs/general/assemble/control/upgrade/2021090902
/// TODO: Implement real business logic
pub async fn stub_general_assemble_control_upgrade_2021090902() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Null)))
}

/// Stub handler for /jaxrs/general/assemble/control/worktime/betweenholidaycount/start/{startDate}/end/{endDate}
/// TODO: Implement real business logic
pub async fn stub_general_assemble_control_worktime_betweenholidaycount_start_startDate_end_endDate() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Null)))
}

/// Stub handler for /jaxrs/general/assemble/control/worktime/betweenminutes/start/{start}/end/{end}
/// TODO: Implement real business logic
pub async fn stub_general_assemble_control_worktime_betweenminutes_start_start_end_end() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Null)))
}

/// Stub handler for /jaxrs/general/assemble/control/worktime/forwarddays/start/{start}/days/{days}
/// TODO: Implement real business logic
pub async fn stub_general_assemble_control_worktime_forwarddays_start_start_days_days() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Null)))
}

/// Stub handler for /jaxrs/general/assemble/control/worktime/forwardminutes/start/{start}/minutes/{minutes}
/// TODO: Implement real business logic
pub async fn stub_general_assemble_control_worktime_forwardminutes_start_start_minutes_minutes() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Null)))
}

/// Stub handler for /jaxrs/general/assemble/control/worktime/indefinedholiday/{date}
/// TODO: Implement real business logic
pub async fn stub_general_assemble_control_worktime_indefinedholiday_date() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Null)))
}

/// Stub handler for /jaxrs/general/assemble/control/worktime/indefinedworkday/{date}
/// TODO: Implement real business logic
pub async fn stub_general_assemble_control_worktime_indefinedworkday_date() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Null)))
}

/// Stub handler for /jaxrs/general/assemble/control/worktime/isholiday/{date}
/// TODO: Implement real business logic
pub async fn stub_general_assemble_control_worktime_isholiday_date() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Null)))
}

/// Stub handler for /jaxrs/general/assemble/control/worktime/isworkday/{date}
/// TODO: Implement real business logic
pub async fn stub_general_assemble_control_worktime_isworkday_date() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Null)))
}

/// Stub handler for /jaxrs/general/assemble/control/worktime/isworktime/{date}
/// TODO: Implement real business logic
pub async fn stub_general_assemble_control_worktime_isworktime_date() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Null)))
}

/// Stub handler for /jaxrs/general/assemble/control/worktime/minutesofworkday
/// TODO: Implement real business logic
pub async fn stub_general_assemble_control_worktime_minutesofworkday() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Null)))
}
