use axum::{
    extract::Extension,
    Json, Router,
};
use deadpool_postgres::Pool;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use shared::{error::AppError, response::ActionResult};

pub mod routes;

#[cfg(test)]
mod tests;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MeetingControl {
    pub id: String,
    pub meeting_id: String,
    pub control_type: String,
    pub enabled: bool,
    pub config: Option<String>,
}

pub async fn list_meeting_controls(
    pool: Extension<Pool>,
    axum::extract::Path(meeting_id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query(
            "SELECT id, meeting_id, control_type, enabled, config FROM x_meeting_assemble_control WHERE meeting_id = $1 ORDER BY create_time",
            &[&meeting_id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("meetingId".to_string(), Value::String(row.get("meeting_id"))),
                ("controlType".to_string(), Value::String(row.get("control_type"))),
                ("enabled".to_string(), Value::Bool(row.get("enabled"))),
                ("config".to_string(), Value::String(row.get::<_, Option<String>>("config").unwrap_or_default())),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("meetingId".to_string(), Value::String(meeting_id)),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

pub async fn create_meeting_control(
    pool: Extension<Pool>,
    axum::extract::Json(payload): axum::extract::Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let meeting_id = payload.get("meetingId").and_then(|v| v.as_str()).ok_or(AppError::BadRequest("meetingId is required".to_string()))?;
    let control_type = payload.get("controlType").and_then(|v| v.as_str()).ok_or(AppError::BadRequest("controlType is required".to_string()))?;
    let enabled: bool = payload.get("enabled").and_then(|v| v.as_bool()).unwrap_or(true);
    let config = payload.get("config").and_then(|v| v.as_str()).map(|s| s.to_string());

    let id: String = uuid::Uuid::new_v4().to_string();

    client
        .execute(
            "INSERT INTO x_meeting_assemble_control (id, meeting_id, control_type, enabled, config) VALUES ($1, $2, $3, $4, $5)",
            &[&id, &meeting_id, &control_type, &enabled, &config],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("id".to_string(), Value::String(id)),
        ("meetingId".to_string(), Value::String(meeting_id.to_string())),
        ("controlType".to_string(), Value::String(control_type.to_string())),
        ("enabled".to_string(), Value::Bool(enabled)),
    ])))))
}

pub async fn delete_meeting_control(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let count = client
        .execute(
            "DELETE FROM x_meeting_assemble_control WHERE id = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("id".to_string(), Value::String(id)),
        ("deleted".to_string(), Value::Bool(count > 0)),
    ])))))
}

pub fn meeting_assemble_control_router(pool: Pool) -> Router {
    routes::meeting_assemble_control_routes(pool)
}

pub fn router(_pool: deadpool_postgres::Pool) -> axum::Router {
    axum::Router::new()
        .route("/meeting_assemble_control/health", axum::routing::get(|| async { "TODO: meeting_assemble_control - real implementation needed" }))
}


/// Stub handler for /jaxrs/meeting/assemble/control/building/list
/// TODO: Implement real business logic
pub async fn stub_meeting_assemble_control_building_list() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Null)))
}

/// Stub handler for /jaxrs/meeting/assemble/control/building/list/like/pinyin/{key}
/// TODO: Implement real business logic
pub async fn stub_meeting_assemble_control_building_list_like_pinyin_key() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Null)))
}

/// Stub handler for /jaxrs/meeting/assemble/control/building/list/like/{key}
/// TODO: Implement real business logic
pub async fn stub_meeting_assemble_control_building_list_like_key() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Null)))
}

/// Stub handler for /jaxrs/meeting/assemble/control/building/list/pinyininitial/{key}
/// TODO: Implement real business logic
pub async fn stub_meeting_assemble_control_building_list_pinyininitial_key() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Null)))
}

/// Stub handler for /jaxrs/meeting/assemble/control/building/list/start/{start}/completed/{completed}
/// TODO: Implement real business logic
pub async fn stub_meeting_assemble_control_building_list_start_start_completed_completed() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Null)))
}

/// Stub handler for /jaxrs/meeting/assemble/control/building/list/start/{start}/completed/{completed}/allmeeting
/// TODO: Implement real business logic
pub async fn stub_meeting_assemble_control_building_list_start_start_completed_completed_allmeeting() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Null)))
}

/// Stub handler for /jaxrs/meeting/assemble/control/building/list/start/{start}/completed/{completed}/room/{room}/meeting/{meeting}
/// TODO: Implement real business logic
pub async fn stub_meeting_assemble_control_building_list_start_start_completed_completed_room_room_meeting_meeting() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Null)))
}

/// Stub handler for /jaxrs/meeting/assemble/control/building/{id}
/// TODO: Implement real business logic
pub async fn stub_meeting_assemble_control_building_id() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Null)))
}

/// Stub handler for /jaxrs/meeting/assemble/control/config/system/config
/// TODO: Implement real business logic
pub async fn stub_meeting_assemble_control_config_system_config() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Null)))
}

/// Stub handler for /jaxrs/meeting/assemble/control/config/system/config/manage
/// TODO: Implement real business logic
pub async fn stub_meeting_assemble_control_config_system_config_manage() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Null)))
}

/// Stub handler for /jaxrs/meeting/assemble/control/meeting/list/applied/completed
/// TODO: Implement real business logic
pub async fn stub_meeting_assemble_control_meeting_list_applied_completed() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Null)))
}

/// Stub handler for /jaxrs/meeting/assemble/control/meeting/list/applied/processing
/// TODO: Implement real business logic
pub async fn stub_meeting_assemble_control_meeting_list_applied_processing() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Null)))
}

/// Stub handler for /jaxrs/meeting/assemble/control/meeting/list/applied/wait
/// TODO: Implement real business logic
pub async fn stub_meeting_assemble_control_meeting_list_applied_wait() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Null)))
}

/// Stub handler for /jaxrs/meeting/assemble/control/meeting/list/apply/{page}/size/{size}
/// TODO: Implement real business logic
pub async fn stub_meeting_assemble_control_meeting_list_apply_page_size_size() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Null)))
}

/// Stub handler for /jaxrs/meeting/assemble/control/meeting/list/coming/day/{count}
/// TODO: Implement real business logic
pub async fn stub_meeting_assemble_control_meeting_list_coming_day_count() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Null)))
}

/// Stub handler for /jaxrs/meeting/assemble/control/meeting/list/coming/month/{count}
/// TODO: Implement real business logic
pub async fn stub_meeting_assemble_control_meeting_list_coming_month_count() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Null)))
}

/// Stub handler for /jaxrs/meeting/assemble/control/meeting/list/forward/monthcount/{monthCount}
/// TODO: Implement real business logic
pub async fn stub_meeting_assemble_control_meeting_list_forward_monthcount_monthCount() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Null)))
}

/// Stub handler for /jaxrs/meeting/assemble/control/meeting/list/forward/monthcount/{monthCount}/all
/// TODO: Implement real business logic
pub async fn stub_meeting_assemble_control_meeting_list_forward_monthcount_monthCount_all() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Null)))
}

/// Stub handler for /jaxrs/meeting/assemble/control/meeting/list/invite/{page}/size/{size}
/// TODO: Implement real business logic
pub async fn stub_meeting_assemble_control_meeting_list_invite_page_size_size() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Null)))
}

/// Stub handler for /jaxrs/meeting/assemble/control/meeting/list/invited/completed
/// TODO: Implement real business logic
pub async fn stub_meeting_assemble_control_meeting_list_invited_completed() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Null)))
}

/// Stub handler for /jaxrs/meeting/assemble/control/meeting/list/invited/processing
/// TODO: Implement real business logic
pub async fn stub_meeting_assemble_control_meeting_list_invited_processing() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Null)))
}

/// Stub handler for /jaxrs/meeting/assemble/control/meeting/list/invited/rejected
/// TODO: Implement real business logic
pub async fn stub_meeting_assemble_control_meeting_list_invited_rejected() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Null)))
}

/// Stub handler for /jaxrs/meeting/assemble/control/meeting/list/invited/wait
/// TODO: Implement real business logic
pub async fn stub_meeting_assemble_control_meeting_list_invited_wait() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Null)))
}

/// Stub handler for /jaxrs/meeting/assemble/control/meeting/list/wait/accept
/// TODO: Implement real business logic
pub async fn stub_meeting_assemble_control_meeting_list_wait_accept() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Null)))
}

/// Stub handler for /jaxrs/meeting/assemble/control/meeting/list/wait/confirm
/// TODO: Implement real business logic
pub async fn stub_meeting_assemble_control_meeting_list_wait_confirm() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Null)))
}

/// Stub handler for /jaxrs/meeting/assemble/control/meeting/list/year/{year}/month/{month}
/// TODO: Implement real business logic
pub async fn stub_meeting_assemble_control_meeting_list_year_year_month_month() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Null)))
}

/// Stub handler for /jaxrs/meeting/assemble/control/meeting/list/year/{year}/month/{month}/all
/// TODO: Implement real business logic
pub async fn stub_meeting_assemble_control_meeting_list_year_year_month_month_all() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Null)))
}

/// Stub handler for /jaxrs/meeting/assemble/control/meeting/list/year/{year}/month/{month}/day/{day}
/// TODO: Implement real business logic
pub async fn stub_meeting_assemble_control_meeting_list_year_year_month_month_day_day() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Null)))
}

/// Stub handler for /jaxrs/meeting/assemble/control/meeting/list/year/{year}/month/{month}/day/{day}/all
/// TODO: Implement real business logic
pub async fn stub_meeting_assemble_control_meeting_list_year_year_month_month_day_day_all() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Null)))
}

/// Stub handler for /jaxrs/meeting/assemble/control/meeting/list/year/{year}/month/{month}/day/{day}/{roomId}
/// TODO: Implement real business logic
pub async fn stub_meeting_assemble_control_meeting_list_year_year_month_month_day_day_roomId() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Null)))
}

/// Stub handler for /jaxrs/meeting/assemble/control/meeting/list/{id}/next/{count}
/// TODO: Implement real business logic
pub async fn stub_meeting_assemble_control_meeting_list_id_next_count() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Null)))
}

/// Stub handler for /jaxrs/meeting/assemble/control/meeting/list/{id}/prev/{count}
/// TODO: Implement real business logic
pub async fn stub_meeting_assemble_control_meeting_list_id_prev_count() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Null)))
}

/// Stub handler for /jaxrs/meeting/assemble/control/meeting/list/{page}/size/{size}
/// TODO: Implement real business logic
pub async fn stub_meeting_assemble_control_meeting_list_page_size_size() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Null)))
}

/// Stub handler for /jaxrs/meeting/assemble/control/meeting/list/{page}/size/{size}/manage
/// TODO: Implement real business logic
pub async fn stub_meeting_assemble_control_meeting_list_page_size_size_manage() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Null)))
}

/// Stub handler for /jaxrs/meeting/assemble/control/meeting/{id}
/// TODO: Implement real business logic
pub async fn stub_meeting_assemble_control_meeting_id() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Null)))
}

/// Stub handler for /jaxrs/meeting/assemble/control/meeting/{id}/accept
/// TODO: Implement real business logic
pub async fn stub_meeting_assemble_control_meeting_id_accept() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Null)))
}

/// Stub handler for /jaxrs/meeting/assemble/control/meeting/{id}/add/invite
/// TODO: Implement real business logic
pub async fn stub_meeting_assemble_control_meeting_id_add_invite() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Null)))
}

/// Stub handler for /jaxrs/meeting/assemble/control/meeting/{id}/checkin
/// TODO: Implement real business logic
pub async fn stub_meeting_assemble_control_meeting_id_checkin() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Null)))
}

/// Stub handler for /jaxrs/meeting/assemble/control/meeting/{id}/checkin/code
/// TODO: Implement real business logic
pub async fn stub_meeting_assemble_control_meeting_id_checkin_code() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Null)))
}

/// Stub handler for /jaxrs/meeting/assemble/control/meeting/{id}/confirm/allow
/// TODO: Implement real business logic
pub async fn stub_meeting_assemble_control_meeting_id_confirm_allow() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Null)))
}

/// Stub handler for /jaxrs/meeting/assemble/control/meeting/{id}/confirm/deny
/// TODO: Implement real business logic
pub async fn stub_meeting_assemble_control_meeting_id_confirm_deny() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Null)))
}

/// Stub handler for /jaxrs/meeting/assemble/control/meeting/{id}/delete/invite
/// TODO: Implement real business logic
pub async fn stub_meeting_assemble_control_meeting_id_delete_invite() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Null)))
}

/// Stub handler for /jaxrs/meeting/assemble/control/meeting/{id}/manual/completed
/// TODO: Implement real business logic
pub async fn stub_meeting_assemble_control_meeting_id_manual_completed() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Null)))
}

/// Stub handler for /jaxrs/meeting/assemble/control/meeting/{id}/modify/completedtime
/// TODO: Implement real business logic
pub async fn stub_meeting_assemble_control_meeting_id_modify_completedtime() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Null)))
}

/// Stub handler for /jaxrs/meeting/assemble/control/meeting/{id}/modify/starttime
/// TODO: Implement real business logic
pub async fn stub_meeting_assemble_control_meeting_id_modify_starttime() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Null)))
}

/// Stub handler for /jaxrs/meeting/assemble/control/meeting/{id}/reject
/// TODO: Implement real business logic
pub async fn stub_meeting_assemble_control_meeting_id_reject() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Null)))
}

/// Stub handler for /jaxrs/meeting/assemble/control/openmeeting/list/room
/// TODO: Implement real business logic
pub async fn stub_meeting_assemble_control_openmeeting_list_room() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Null)))
}

/// Stub handler for /jaxrs/meeting/assemble/control/room/list
/// TODO: Implement real business logic
pub async fn stub_meeting_assemble_control_room_list() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Null)))
}

/// Stub handler for /jaxrs/meeting/assemble/control/room/list/like/pinyin/{key}
/// TODO: Implement real business logic
pub async fn stub_meeting_assemble_control_room_list_like_pinyin_key() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Null)))
}

/// Stub handler for /jaxrs/meeting/assemble/control/room/list/like/{key}
/// TODO: Implement real business logic
pub async fn stub_meeting_assemble_control_room_list_like_key() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Null)))
}

/// Stub handler for /jaxrs/meeting/assemble/control/room/list/pinyininitial/{key}
/// TODO: Implement real business logic
pub async fn stub_meeting_assemble_control_room_list_pinyininitial_key() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Null)))
}

/// Stub handler for /jaxrs/meeting/assemble/control/room/{id}
/// TODO: Implement real business logic
pub async fn stub_meeting_assemble_control_room_id() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Null)))
}

/// Stub handler for /jaxrs/meeting/assemble/control/room/{id}/photo
/// TODO: Implement real business logic
pub async fn stub_meeting_assemble_control_room_id_photo() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Null)))
}
