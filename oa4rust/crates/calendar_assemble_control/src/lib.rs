use axum::{
    extract::Extension,
    routing::get,
    Json, Router,
};
use deadpool_postgres::Pool;
use serde_json::Value;
use shared::{error::AppError, response::ActionResult};

pub mod routes;

#[cfg(test)]
mod tests;
#[cfg(test)]
mod tests_generated;


#[axum::debug_handler]
pub async fn get_control_config(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let row = client
        .query_one(
            "SELECT config_key, config_value FROM cal_control_config WHERE config_key = 'global' ORDER BY create_time DESC LIMIT 1",
            &[],
        )
        .await
        .ok();

    let (enabled, default_time_zone, allow_sharing) = match row {
        Some(r) => {
            let cfg: Option<String> = r.get("config_value");
            match cfg {
                Some(ref s) if s.contains("\"enabled\":true") => (true, "UTC+8".to_string(), true),
                Some(ref s) if s.contains("\"enabled\":false") => (false, "UTC+8".to_string(), false),
                _ => (true, "UTC+8".to_string(), true),
            }
        }
        None => (true, "UTC+8".to_string(), true),
    };

    let data = Value::Object(serde_json::Map::from_iter([
        ("enabled".to_string(), Value::Bool(enabled)),
        ("defaultTimeZone".to_string(), Value::String(default_time_zone)),
        ("allowSharing".to_string(), Value::Bool(allow_sharing)),
    ]));

    Ok(Json(ActionResult::success(data)))
}

#[axum::debug_handler]
pub async fn list_control_calendars(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query(
            "SELECT id, name, type, is_public, status FROM CAL_CALENDAR WHERE status = 'OPEN' ORDER BY create_time DESC LIMIT 50",
            &[],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let mut calendars = Vec::new();
    for row in rows.iter() {
        let cal_type: String = row.get("type");
        let is_public: bool = row.get("is_public");
        calendars.push(Value::Object(serde_json::Map::from_iter([
            ("id".to_string(), Value::String(row.get("id"))),
            ("name".to_string(), Value::String(row.get("name"))),
            ("type".to_string(), Value::String(cal_type)),
            ("enabled".to_string(), Value::Bool(is_public)),
        ])));
    }

    Ok(Json(ActionResult::success(Value::Array(calendars))))
}

#[axum::debug_handler]
pub async fn update_control_config(
    pool: Extension<Pool>,
    body: axum::extract::Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let config = body.0;
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let enabled = config
        .get("enabled")
        .and_then(|v| v.as_bool())
        .unwrap_or(true);
    let default_time_zone = config
        .get("defaultTimeZone")
        .and_then(|v| v.as_str())
        .unwrap_or("UTC+8")
        .to_string();
    let allow_sharing = config
        .get("allowSharing")
        .and_then(|v| v.as_bool())
        .unwrap_or(true);

    let config_value = serde_json::json!({
        "enabled": enabled,
        "defaultTimeZone": default_time_zone,
        "allowSharing": allow_sharing,
    });

    let config_key = "global";
    let result = client
        .execute(
            "INSERT INTO cal_control_config (config_key, config_value) VALUES ($1, $2) \
              ON CONFLICT (config_key) DO UPDATE SET config_value = EXCLUDED.config_value, update_time = NOW()",
            &[&config_key, &config_value.to_string()],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    tracing::info!("Updated calendar assemble control config: {:?}", config_value);

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("updated".to_string(), Value::Bool(result > 0)),
            ("config".to_string(), config_value),
        ]),
    ))))
}

pub async fn get_calendar_detail(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let calendar_row = client
        .query_one(
            "SELECT id, name, type, target, color, description, source, createor, is_public, status \
             FROM CAL_CALENDAR WHERE id = $1 AND status = 'OPEN'",
            &[&id],
        )
        .await
        .map_err(|_| AppError::NotFound)?;

    let event_rows = client
        .query(
            "SELECT id, calendar_id, title, content, location, start_time, end_time, all_day, visibility, status, createor \
             FROM CAL_EVENT WHERE calendar_id = $1 AND status = 'OPEN' ORDER BY start_time ASC LIMIT 100",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let events: Vec<Value> = event_rows
        .iter()
        .map(|row| {
            let mut map = serde_json::Map::new();
            map.insert("id".to_string(), Value::String(row.get("id")));
            map.insert("calendarId".to_string(), Value::String(row.get("calendar_id")));
            map.insert("title".to_string(), Value::String(row.get("title")));
            if let Some(content) = row.get::<_, Option<String>>("content") {
                map.insert("content".to_string(), Value::String(content));
            }
            if let Some(location) = row.get::<_, Option<String>>("location") {
                map.insert("location".to_string(), Value::String(location));
            }
            map.insert("startTime".to_string(), Value::String(row.get("start_time")));
            map.insert("endTime".to_string(), Value::String(row.get("end_time")));
            map.insert("allDay".to_string(), Value::Bool(row.get("all_day")));
            map.insert("visibility".to_string(), Value::String(row.get("visibility")));
            map.insert("status".to_string(), Value::String(row.get("status")));
            Value::Object(map)
        })
        .collect();

    let mut data_map = serde_json::Map::new();
    data_map.insert("id".to_string(), Value::String(calendar_row.get("id")));
    data_map.insert("name".to_string(), Value::String(calendar_row.get("name")));
    data_map.insert("type".to_string(), Value::String(calendar_row.get("type")));
    data_map.insert("target".to_string(), Value::String(calendar_row.get("target")));
    data_map.insert("color".to_string(), Value::String(calendar_row.get("color")));
    if let Some(description) = calendar_row.get::<_, Option<String>>("description") {
        data_map.insert("description".to_string(), Value::String(description));
    }
    data_map.insert("createor".to_string(), Value::String(calendar_row.get("createor")));
    data_map.insert("isPublic".to_string(), Value::Bool(calendar_row.get("is_public")));
    data_map.insert("status".to_string(), Value::String(calendar_row.get("status")));
    data_map.insert("eventCount".to_string(), Value::Number(serde_json::Number::from(events.len() as i64)));
    data_map.insert("events".to_string(), Value::Array(events));
    let data = Value::Object(data_map);

    Ok(Json(ActionResult::success(data)))
}

pub fn calendar_assemble_control_router(pool: Pool) -> Router {
    routes::router(pool)
        .route("/jaxrs/calendar/assemble/control/calendar/detail/{id}", get(get_calendar_detail))
}

pub fn router(pool: deadpool_postgres::Pool) -> axum::Router {
    crate::calendar_assemble_control_router(pool)
}

