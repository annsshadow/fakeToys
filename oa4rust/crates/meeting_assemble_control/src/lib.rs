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
#[cfg(test)]
mod tests_generated;


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

pub fn router(pool: deadpool_postgres::Pool) -> axum::Router {
    crate::routes::meeting_assemble_control_routes(pool)
}



pub async fn building_list(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query(
            "SELECT id, name, pinyin, pinyin_initial, address, description, create_time FROM x_meeting_building ORDER BY create_time",
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
                ("pinyin".to_string(), Value::String(row.get::<_, Option<String>>("pinyin").unwrap_or_default())),
                ("pinyinInitial".to_string(), Value::String(row.get::<_, Option<String>>("pinyin_initial").unwrap_or_default())),
                ("address".to_string(), Value::String(row.get::<_, Option<String>>("address").unwrap_or_default())),
                ("description".to_string(), Value::String(row.get::<_, Option<String>>("description").unwrap_or_default())),
                ("createTime".to_string(), Value::String(row.get("create_time"))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

pub async fn building_list_like_pinyin_key(
    pool: Extension<Pool>,
    axum::extract::Path(key): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let pattern = format!("%{}%", key);
    let rows = client
        .query(
            "SELECT id, name, pinyin, pinyin_initial, address, description, create_time FROM x_meeting_building WHERE pinyin ILIKE $1 ORDER BY create_time",
            &[&pattern],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("name".to_string(), Value::String(row.get("name"))),
                ("pinyin".to_string(), Value::String(row.get::<_, Option<String>>("pinyin").unwrap_or_default())),
                ("pinyinInitial".to_string(), Value::String(row.get::<_, Option<String>>("pinyin_initial").unwrap_or_default())),
                ("address".to_string(), Value::String(row.get::<_, Option<String>>("address").unwrap_or_default())),
                ("description".to_string(), Value::String(row.get::<_, Option<String>>("description").unwrap_or_default())),
                ("createTime".to_string(), Value::String(row.get("create_time"))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

pub async fn building_list_like_key(
    pool: Extension<Pool>,
    axum::extract::Path(key): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let pattern = format!("%{}%", key);
    let rows = client
        .query(
            "SELECT id, name, pinyin, pinyin_initial, address, description, create_time FROM x_meeting_building WHERE name ILIKE $1 OR address ILIKE $1 ORDER BY create_time",
            &[&pattern],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("name".to_string(), Value::String(row.get("name"))),
                ("pinyin".to_string(), Value::String(row.get::<_, Option<String>>("pinyin").unwrap_or_default())),
                ("pinyinInitial".to_string(), Value::String(row.get::<_, Option<String>>("pinyin_initial").unwrap_or_default())),
                ("address".to_string(), Value::String(row.get::<_, Option<String>>("address").unwrap_or_default())),
                ("description".to_string(), Value::String(row.get::<_, Option<String>>("description").unwrap_or_default())),
                ("createTime".to_string(), Value::String(row.get("create_time"))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

pub async fn building_list_pinyininitial_key(
    pool: Extension<Pool>,
    axum::extract::Path(key): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query(
            "SELECT id, name, pinyin, pinyin_initial, address, description, create_time FROM x_meeting_building WHERE pinyin_initial ILIKE $1 ORDER BY create_time",
            &[&key],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("name".to_string(), Value::String(row.get("name"))),
                ("pinyin".to_string(), Value::String(row.get::<_, Option<String>>("pinyin").unwrap_or_default())),
                ("pinyinInitial".to_string(), Value::String(row.get::<_, Option<String>>("pinyin_initial").unwrap_or_default())),
                ("address".to_string(), Value::String(row.get::<_, Option<String>>("address").unwrap_or_default())),
                ("description".to_string(), Value::String(row.get::<_, Option<String>>("description").unwrap_or_default())),
                ("createTime".to_string(), Value::String(row.get("create_time"))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

pub async fn building_list_start_start_completed_completed(
    pool: Extension<Pool>,
    axum::extract::Path((start, completed)): axum::extract::Path<(String, String)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query(
            "SELECT id, name, pinyin, pinyin_initial, address, description, create_time FROM x_meeting_building WHERE start_time >= $1 AND end_time <= $2 ORDER BY create_time",
            &[&start, &completed],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("name".to_string(), Value::String(row.get("name"))),
                ("pinyin".to_string(), Value::String(row.get::<_, Option<String>>("pinyin").unwrap_or_default())),
                ("pinyinInitial".to_string(), Value::String(row.get::<_, Option<String>>("pinyin_initial").unwrap_or_default())),
                ("address".to_string(), Value::String(row.get::<_, Option<String>>("address").unwrap_or_default())),
                ("description".to_string(), Value::String(row.get::<_, Option<String>>("description").unwrap_or_default())),
                ("createTime".to_string(), Value::String(row.get("create_time"))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

pub async fn building_list_start_start_completed_completed_allmeeting(
    pool: Extension<Pool>,
    axum::extract::Path((start, completed)): axum::extract::Path<(String, String)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query(
            "SELECT id, name, pinyin, pinyin_initial, address, description, create_time FROM x_meeting_building WHERE start_time >= $1 AND end_time <= $2 ORDER BY create_time",
            &[&start, &completed],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("name".to_string(), Value::String(row.get("name"))),
                ("pinyin".to_string(), Value::String(row.get::<_, Option<String>>("pinyin").unwrap_or_default())),
                ("pinyinInitial".to_string(), Value::String(row.get::<_, Option<String>>("pinyin_initial").unwrap_or_default())),
                ("address".to_string(), Value::String(row.get::<_, Option<String>>("address").unwrap_or_default())),
                ("description".to_string(), Value::String(row.get::<_, Option<String>>("description").unwrap_or_default())),
                ("createTime".to_string(), Value::String(row.get("create_time"))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

pub async fn building_list_start_start_completed_completed_room_room_meeting_meeting(
    pool: Extension<Pool>,
    axum::extract::Path((start, completed, room, meeting)): axum::extract::Path<(String, String, String, String)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query(
            "SELECT id, name, pinyin, pinyin_initial, address, description, create_time FROM x_meeting_building WHERE start_time >= $1 AND end_time <= $2 AND room_id = $3 AND meeting_id = $4 ORDER BY create_time",
            &[&start, &completed, &room, &meeting],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("name".to_string(), Value::String(row.get("name"))),
                ("pinyin".to_string(), Value::String(row.get::<_, Option<String>>("pinyin").unwrap_or_default())),
                ("pinyinInitial".to_string(), Value::String(row.get::<_, Option<String>>("pinyin_initial").unwrap_or_default())),
                ("address".to_string(), Value::String(row.get::<_, Option<String>>("address").unwrap_or_default())),
                ("description".to_string(), Value::String(row.get::<_, Option<String>>("description").unwrap_or_default())),
                ("createTime".to_string(), Value::String(row.get("create_time"))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

pub async fn building_id(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let row = client
        .query_opt(
            "SELECT id, name, pinyin, pinyin_initial, address, description, create_time FROM x_meeting_building WHERE id = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let result = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("name".to_string(), Value::String(row.get("name"))),
                ("pinyin".to_string(), Value::String(row.get::<_, Option<String>>("pinyin").unwrap_or_default())),
                ("pinyinInitial".to_string(), Value::String(row.get::<_, Option<String>>("pinyin_initial").unwrap_or_default())),
                ("address".to_string(), Value::String(row.get::<_, Option<String>>("address").unwrap_or_default())),
                ("description".to_string(), Value::String(row.get::<_, Option<String>>("description").unwrap_or_default())),
                ("createTime".to_string(), Value::String(row.get("create_time"))),
            ]));
            Ok(Json(ActionResult::success(result)))
        }
        None => Ok(Json(ActionResult::error("building not found"))),
    }
}

pub async fn config_system_config(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query(
            "SELECT id, config_key, config_value, description, create_time FROM x_meeting_config ORDER BY create_time",
            &[],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("configKey".to_string(), Value::String(row.get("config_key"))),
                ("configValue".to_string(), Value::String(row.get("config_value"))),
                ("description".to_string(), Value::String(row.get::<_, Option<String>>("description").unwrap_or_default())),
                ("createTime".to_string(), Value::String(row.get("create_time"))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

pub async fn config_system_config_manage(
    pool: Extension<Pool>,
    axum::extract::Json(payload): axum::extract::Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let config_key = payload.get("configKey").and_then(|v| v.as_str()).ok_or(AppError::BadRequest("configKey is required".to_string()))?;
    let config_value = payload.get("configValue").and_then(|v| v.as_str()).ok_or(AppError::BadRequest("configValue is required".to_string()))?;
    let description = payload.get("description").and_then(|v| v.as_str()).map(|s| s.to_string());

    let result = client
        .execute(
            "UPDATE x_meeting_config SET config_value = $1, description = $2 WHERE config_key = $3",
            &[&config_value, &description, &config_key],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if result == 0 {
        return Ok(Json(ActionResult::error("config not found")));
    }

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("configKey".to_string(), Value::String(config_key.to_string())),
            ("configValue".to_string(), Value::String(config_value.to_string())),
            ("updated".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn meeting_list_applied_completed(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query(
            "SELECT id, title, content, start_time, end_time, creator, create_time FROM x_meeting WHERE status = 'completed' AND applied = true ORDER BY create_time DESC",
            &[],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("title".to_string(), Value::String(row.get("title"))),
                ("content".to_string(), Value::String(row.get::<_, Option<String>>("content").unwrap_or_default())),
                ("startTime".to_string(), Value::String(row.get("start_time"))),
                ("endTime".to_string(), Value::String(row.get("end_time"))),
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

pub async fn meeting_list_applied_processing(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query(
            "SELECT id, title, content, start_time, end_time, creator, create_time FROM x_meeting WHERE status = 'processing' AND applied = true ORDER BY create_time DESC",
            &[],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("title".to_string(), Value::String(row.get("title"))),
                ("content".to_string(), Value::String(row.get::<_, Option<String>>("content").unwrap_or_default())),
                ("startTime".to_string(), Value::String(row.get("start_time"))),
                ("endTime".to_string(), Value::String(row.get("end_time"))),
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

pub async fn meeting_list_applied_wait(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query(
            "SELECT id, title, content, start_time, end_time, creator, create_time FROM x_meeting WHERE status = 'wait' AND applied = true ORDER BY create_time DESC",
            &[],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("title".to_string(), Value::String(row.get("title"))),
                ("content".to_string(), Value::String(row.get::<_, Option<String>>("content").unwrap_or_default())),
                ("startTime".to_string(), Value::String(row.get("start_time"))),
                ("endTime".to_string(), Value::String(row.get("end_time"))),
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

pub async fn meeting_list_apply_page_size_size(
    pool: Extension<Pool>,
    axum::extract::Path((page, size)): axum::extract::Path<(i64, i64)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let offset = ((page.max(1) - 1) * size).max(0);
    let limit = size.max(1);

    let rows = client
        .query(
            "SELECT id, title, content, start_time, end_time, creator, create_time FROM x_meeting WHERE applied = true ORDER BY create_time DESC LIMIT $1 OFFSET $2",
            &[&limit, &offset],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("title".to_string(), Value::String(row.get("title"))),
                ("content".to_string(), Value::String(row.get::<_, Option<String>>("content").unwrap_or_default())),
                ("startTime".to_string(), Value::String(row.get("start_time"))),
                ("endTime".to_string(), Value::String(row.get("end_time"))),
                ("creator".to_string(), Value::String(row.get("creator"))),
                ("createTime".to_string(), Value::String(row.get("create_time"))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("page".to_string(), Value::Number(serde_json::Number::from(page))),
        ("size".to_string(), Value::Number(serde_json::Number::from(size))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

pub async fn meeting_list_coming_day_count(
    pool: Extension<Pool>,
    axum::extract::Path(count): axum::extract::Path<i64>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query(
            "SELECT id, title, content, start_time, end_time, creator, create_time FROM x_meeting WHERE start_time >= NOW() AND start_time <= NOW() + INTERVAL '1 day' * $1 ORDER BY start_time ASC",
            &[&count],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("title".to_string(), Value::String(row.get("title"))),
                ("content".to_string(), Value::String(row.get::<_, Option<String>>("content").unwrap_or_default())),
                ("startTime".to_string(), Value::String(row.get("start_time"))),
                ("endTime".to_string(), Value::String(row.get("end_time"))),
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

pub async fn meeting_list_coming_month_count(
    pool: Extension<Pool>,
    axum::extract::Path(count): axum::extract::Path<i64>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query(
            "SELECT id, title, content, start_time, end_time, creator, create_time FROM x_meeting WHERE start_time >= NOW() AND start_time <= NOW() + INTERVAL '1 month' * $1 ORDER BY start_time ASC",
            &[&count],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("title".to_string(), Value::String(row.get("title"))),
                ("content".to_string(), Value::String(row.get::<_, Option<String>>("content").unwrap_or_default())),
                ("startTime".to_string(), Value::String(row.get("start_time"))),
                ("endTime".to_string(), Value::String(row.get("end_time"))),
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

pub async fn meeting_list_forward_monthcount_monthCount(
    pool: Extension<Pool>,
    axum::extract::Path(month_count): axum::extract::Path<i64>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query(
            "SELECT id, title, content, start_time, end_time, creator, create_time FROM x_meeting WHERE start_time >= NOW() AND start_time <= NOW() + INTERVAL '1 month' * $1 ORDER BY start_time ASC",
            &[&month_count],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("title".to_string(), Value::String(row.get("title"))),
                ("content".to_string(), Value::String(row.get::<_, Option<String>>("content").unwrap_or_default())),
                ("startTime".to_string(), Value::String(row.get("start_time"))),
                ("endTime".to_string(), Value::String(row.get("end_time"))),
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

pub async fn meeting_list_forward_monthcount_monthCount_all(
    pool: Extension<Pool>,
    axum::extract::Path(month_count): axum::extract::Path<i64>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query(
            "SELECT id, title, content, start_time, end_time, creator, create_time FROM x_meeting WHERE start_time >= NOW() AND start_time <= NOW() + INTERVAL '1 month' * $1 ORDER BY start_time ASC",
            &[&month_count],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("title".to_string(), Value::String(row.get("title"))),
                ("content".to_string(), Value::String(row.get::<_, Option<String>>("content").unwrap_or_default())),
                ("startTime".to_string(), Value::String(row.get("start_time"))),
                ("endTime".to_string(), Value::String(row.get("end_time"))),
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

pub async fn meeting_list_invite_page_size_size(
    pool: Extension<Pool>,
    axum::extract::Path((page, size)): axum::extract::Path<(i64, i64)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let offset = ((page.max(1) - 1) * size).max(0);
    let limit = size.max(1);

    let rows = client
        .query(
            "SELECT id, title, content, start_time, end_time, creator, create_time FROM x_meeting WHERE invited = true ORDER BY create_time DESC LIMIT $1 OFFSET $2",
            &[&limit, &offset],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("title".to_string(), Value::String(row.get("title"))),
                ("content".to_string(), Value::String(row.get::<_, Option<String>>("content").unwrap_or_default())),
                ("startTime".to_string(), Value::String(row.get("start_time"))),
                ("endTime".to_string(), Value::String(row.get("end_time"))),
                ("creator".to_string(), Value::String(row.get("creator"))),
                ("createTime".to_string(), Value::String(row.get("create_time"))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("page".to_string(), Value::Number(serde_json::Number::from(page))),
        ("size".to_string(), Value::Number(serde_json::Number::from(size))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

pub async fn meeting_list_invited_completed(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query(
            "SELECT id, title, content, start_time, end_time, creator, create_time FROM x_meeting WHERE status = 'completed' AND invited = true ORDER BY create_time DESC",
            &[],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("title".to_string(), Value::String(row.get("title"))),
                ("content".to_string(), Value::String(row.get::<_, Option<String>>("content").unwrap_or_default())),
                ("startTime".to_string(), Value::String(row.get("start_time"))),
                ("endTime".to_string(), Value::String(row.get("end_time"))),
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

pub async fn meeting_list_invited_processing(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query(
            "SELECT id, title, content, start_time, end_time, creator, create_time FROM x_meeting WHERE status = 'processing' AND invited = true ORDER BY create_time DESC",
            &[],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("title".to_string(), Value::String(row.get("title"))),
                ("content".to_string(), Value::String(row.get::<_, Option<String>>("content").unwrap_or_default())),
                ("startTime".to_string(), Value::String(row.get("start_time"))),
                ("endTime".to_string(), Value::String(row.get("end_time"))),
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

pub async fn meeting_list_invited_rejected(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query(
            "SELECT id, title, content, start_time, end_time, creator, create_time FROM x_meeting WHERE status = 'rejected' AND invited = true ORDER BY create_time DESC",
            &[],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("title".to_string(), Value::String(row.get("title"))),
                ("content".to_string(), Value::String(row.get::<_, Option<String>>("content").unwrap_or_default())),
                ("startTime".to_string(), Value::String(row.get("start_time"))),
                ("endTime".to_string(), Value::String(row.get("end_time"))),
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

pub async fn meeting_list_invited_wait(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query(
            "SELECT id, title, content, start_time, end_time, creator, create_time FROM x_meeting WHERE status = 'wait' AND invited = true ORDER BY create_time DESC",
            &[],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("title".to_string(), Value::String(row.get("title"))),
                ("content".to_string(), Value::String(row.get::<_, Option<String>>("content").unwrap_or_default())),
                ("startTime".to_string(), Value::String(row.get("start_time"))),
                ("endTime".to_string(), Value::String(row.get("end_time"))),
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

pub async fn meeting_list_wait_accept(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query(
            "SELECT id, title, content, start_time, end_time, creator, create_time FROM x_meeting WHERE status = 'wait_accept' ORDER BY create_time DESC",
            &[],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("title".to_string(), Value::String(row.get("title"))),
                ("content".to_string(), Value::String(row.get::<_, Option<String>>("content").unwrap_or_default())),
                ("startTime".to_string(), Value::String(row.get("start_time"))),
                ("endTime".to_string(), Value::String(row.get("end_time"))),
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

pub async fn meeting_list_wait_confirm(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query(
            "SELECT id, title, content, start_time, end_time, creator, create_time FROM x_meeting WHERE status = 'wait_confirm' ORDER BY create_time DESC",
            &[],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("title".to_string(), Value::String(row.get("title"))),
                ("content".to_string(), Value::String(row.get::<_, Option<String>>("content").unwrap_or_default())),
                ("startTime".to_string(), Value::String(row.get("start_time"))),
                ("endTime".to_string(), Value::String(row.get("end_time"))),
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

pub async fn meeting_list_year_year_month_month(
    pool: Extension<Pool>,
    axum::extract::Path((year, month)): axum::extract::Path<(i32, i32)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let start_date = format!("{}-{:02}-01", year, month);
    let end_date = if month == 12 {
        format!("{}-01-01", year + 1)
    } else {
        format!("{}-{:02}-01", year, month + 1)
    };

    let rows = client
        .query(
            "SELECT id, title, content, start_time, end_time, creator, create_time FROM x_meeting WHERE start_time >= $1 AND start_time < $2 ORDER BY start_time ASC",
            &[&start_date, &end_date],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("title".to_string(), Value::String(row.get("title"))),
                ("content".to_string(), Value::String(row.get::<_, Option<String>>("content").unwrap_or_default())),
                ("startTime".to_string(), Value::String(row.get("start_time"))),
                ("endTime".to_string(), Value::String(row.get("end_time"))),
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

pub async fn meeting_list_year_year_month_month_all(
    pool: Extension<Pool>,
    axum::extract::Path((year, month)): axum::extract::Path<(i32, i32)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let start_date = format!("{}-{:02}-01", year, month);
    let end_date = if month == 12 {
        format!("{}-01-01", year + 1)
    } else {
        format!("{}-{:02}-01", year, month + 1)
    };

    let rows = client
        .query(
            "SELECT id, title, content, start_time, end_time, creator, create_time FROM x_meeting WHERE start_time >= $1 AND start_time < $2 ORDER BY start_time ASC",
            &[&start_date, &end_date],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("title".to_string(), Value::String(row.get("title"))),
                ("content".to_string(), Value::String(row.get::<_, Option<String>>("content").unwrap_or_default())),
                ("startTime".to_string(), Value::String(row.get("start_time"))),
                ("endTime".to_string(), Value::String(row.get("end_time"))),
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

pub async fn meeting_list_year_year_month_month_day_day(
    pool: Extension<Pool>,
    axum::extract::Path((year, month, day)): axum::extract::Path<(i32, i32, i32)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let start_date = format!("{}-{:02}-{:02} 00:00:00", year, month, day);
    let end_date = format!("{}-{:02}-{:02} 23:59:59", year, month, day);

    let rows = client
        .query(
            "SELECT id, title, content, start_time, end_time, creator, create_time FROM x_meeting WHERE start_time >= $1 AND start_time <= $2 ORDER BY start_time ASC",
            &[&start_date, &end_date],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("title".to_string(), Value::String(row.get("title"))),
                ("content".to_string(), Value::String(row.get::<_, Option<String>>("content").unwrap_or_default())),
                ("startTime".to_string(), Value::String(row.get("start_time"))),
                ("endTime".to_string(), Value::String(row.get("end_time"))),
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

pub async fn meeting_list_year_year_month_month_day_day_all(
    pool: Extension<Pool>,
    axum::extract::Path((year, month, day)): axum::extract::Path<(i32, i32, i32)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let start_date = format!("{}-{:02}-{:02} 00:00:00", year, month, day);
    let end_date = format!("{}-{:02}-{:02} 23:59:59", year, month, day);

    let rows = client
        .query(
            "SELECT id, title, content, start_time, end_time, creator, create_time FROM x_meeting WHERE start_time >= $1 AND start_time <= $2 ORDER BY start_time ASC",
            &[&start_date, &end_date],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("title".to_string(), Value::String(row.get("title"))),
                ("content".to_string(), Value::String(row.get::<_, Option<String>>("content").unwrap_or_default())),
                ("startTime".to_string(), Value::String(row.get("start_time"))),
                ("endTime".to_string(), Value::String(row.get("end_time"))),
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

pub async fn meeting_list_year_year_month_month_day_day_roomId(
    pool: Extension<Pool>,
    axum::extract::Path((year, month, day, room_id)): axum::extract::Path<(i32, i32, i32, String)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let start_date = format!("{}-{:02}-{:02} 00:00:00", year, month, day);
    let end_date = format!("{}-{:02}-{:02} 23:59:59", year, month, day);

    let rows = client
        .query(
            "SELECT id, title, content, start_time, end_time, creator, create_time FROM x_meeting WHERE start_time >= $1 AND start_time <= $2 AND room_id = $3 ORDER BY start_time ASC",
            &[&start_date, &end_date, &room_id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("title".to_string(), Value::String(row.get("title"))),
                ("content".to_string(), Value::String(row.get::<_, Option<String>>("content").unwrap_or_default())),
                ("startTime".to_string(), Value::String(row.get("start_time"))),
                ("endTime".to_string(), Value::String(row.get("end_time"))),
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

pub async fn meeting_list_id_next_count(
    pool: Extension<Pool>,
    axum::extract::Path((flag, count)): axum::extract::Path<(String, i64)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let count = count.max(1);

    let rows = if flag.is_empty() {
        client
            .query(
                "SELECT id, title, content, start_time, end_time, creator, create_time FROM x_meeting ORDER BY create_time DESC LIMIT $1",
                &[&count],
            )
            .await
            .map_err(|_| AppError::Internal)?
    } else {
        client
            .query(
                "SELECT id, title, content, start_time, end_time, creator, create_time FROM x_meeting WHERE id > $1 ORDER BY create_time DESC LIMIT $2",
                &[&flag, &count],
            )
            .await
            .map_err(|_| AppError::Internal)?
    };

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("title".to_string(), Value::String(row.get("title"))),
                ("content".to_string(), Value::String(row.get::<_, Option<String>>("content").unwrap_or_default())),
                ("startTime".to_string(), Value::String(row.get("start_time"))),
                ("endTime".to_string(), Value::String(row.get("end_time"))),
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

pub async fn meeting_list_id_prev_count(
    pool: Extension<Pool>,
    axum::extract::Path((id, count)): axum::extract::Path<(String, i64)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let count = count.max(1);

    let rows = client
        .query(
            "SELECT id, title, content, start_time, end_time, creator, create_time FROM x_meeting WHERE id < $1 ORDER BY create_time DESC LIMIT $2",
            &[&id, &count],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("title".to_string(), Value::String(row.get("title"))),
                ("content".to_string(), Value::String(row.get::<_, Option<String>>("content").unwrap_or_default())),
                ("startTime".to_string(), Value::String(row.get("start_time"))),
                ("endTime".to_string(), Value::String(row.get("end_time"))),
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

pub async fn meeting_list_page_size_size(
    pool: Extension<Pool>,
    axum::extract::Path((page, size)): axum::extract::Path<(i64, i64)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let offset = ((page.max(1) - 1) * size).max(0);
    let limit = size.max(1);

    let rows = client
        .query(
            "SELECT id, title, content, start_time, end_time, creator, create_time FROM x_meeting ORDER BY create_time DESC LIMIT $1 OFFSET $2",
            &[&limit, &offset],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("title".to_string(), Value::String(row.get("title"))),
                ("content".to_string(), Value::String(row.get::<_, Option<String>>("content").unwrap_or_default())),
                ("startTime".to_string(), Value::String(row.get("start_time"))),
                ("endTime".to_string(), Value::String(row.get("end_time"))),
                ("creator".to_string(), Value::String(row.get("creator"))),
                ("createTime".to_string(), Value::String(row.get("create_time"))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("page".to_string(), Value::Number(serde_json::Number::from(page))),
        ("size".to_string(), Value::Number(serde_json::Number::from(size))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

pub async fn meeting_list_page_size_size_manage(
    pool: Extension<Pool>,
    axum::extract::Path((page, size)): axum::extract::Path<(i64, i64)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let offset = ((page.max(1) - 1) * size).max(0);
    let limit = size.max(1);

    let rows = client
        .query(
            "SELECT id, title, content, start_time, end_time, creator, create_time FROM x_meeting ORDER BY create_time DESC LIMIT $1 OFFSET $2",
            &[&limit, &offset],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("title".to_string(), Value::String(row.get("title"))),
                ("content".to_string(), Value::String(row.get::<_, Option<String>>("content").unwrap_or_default())),
                ("startTime".to_string(), Value::String(row.get("start_time"))),
                ("endTime".to_string(), Value::String(row.get("end_time"))),
                ("creator".to_string(), Value::String(row.get("creator"))),
                ("createTime".to_string(), Value::String(row.get("create_time"))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("page".to_string(), Value::Number(serde_json::Number::from(page))),
        ("size".to_string(), Value::Number(serde_json::Number::from(size))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

pub async fn meeting_id(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let row = client
        .query_opt(
            "SELECT id, title, content, start_time, end_time, creator, create_time FROM x_meeting WHERE id = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let result = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("title".to_string(), Value::String(row.get("title"))),
                ("content".to_string(), Value::String(row.get::<_, Option<String>>("content").unwrap_or_default())),
                ("startTime".to_string(), Value::String(row.get("start_time"))),
                ("endTime".to_string(), Value::String(row.get("end_time"))),
                ("creator".to_string(), Value::String(row.get("creator"))),
                ("createTime".to_string(), Value::String(row.get("create_time"))),
            ]));
            Ok(Json(ActionResult::success(result)))
        }
        None => Ok(Json(ActionResult::error("meeting not found"))),
    }
}

pub async fn create_meeting(
    pool: Extension<Pool>,
    axum::extract::Json(payload): axum::extract::Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let title = payload.get("title").and_then(|v| v.as_str()).ok_or(AppError::BadRequest("title is required".to_string()))?;
    let content = payload.get("content").and_then(|v| v.as_str()).map(|s| s.to_string());
    let start_time = payload.get("startTime").and_then(|v| v.as_str()).ok_or(AppError::BadRequest("startTime is required".to_string()))?;
    let end_time = payload.get("endTime").and_then(|v| v.as_str()).ok_or(AppError::BadRequest("endTime is required".to_string()))?;
    let creator = payload.get("creator").and_then(|v| v.as_str()).unwrap_or("system");

    let id = uuid::Uuid::new_v4().to_string();

    client
        .execute(
            "INSERT INTO x_meeting (id, title, content, start_time, end_time, creator, create_time) VALUES ($1, $2, $3, $4, $5, $6, NOW())",
            &[&id, &title, &content, &start_time, &end_time, &creator],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("id".to_string(), Value::String(id)),
        ("title".to_string(), Value::String(title.to_string())),
        ("creator".to_string(), Value::String(creator.to_string())),
        ("startTime".to_string(), Value::String(start_time.to_string())),
        ("endTime".to_string(), Value::String(end_time.to_string())),
    ])))))
}

pub async fn save_meeting(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
    axum::extract::Json(payload): axum::extract::Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let title = payload.get("title").and_then(|v| v.as_str()).unwrap_or_default();
    let content = payload.get("content").and_then(|v| v.as_str()).map(|s| s.to_string());
    let start_time = payload.get("startTime").and_then(|v| v.as_str()).unwrap_or_default();
    let end_time = payload.get("endTime").and_then(|v| v.as_str()).unwrap_or_default();

    let result = client
        .execute(
            "UPDATE x_meeting SET title = $1, content = $2, start_time = $3, end_time = $4 WHERE id = $5",
            &[&title, &content, &start_time, &end_time, &id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if result == 0 {
        return Ok(Json(ActionResult::error("meeting not found")));
    }

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
            ("saved".to_string(), Value::Bool(true)),
            ("title".to_string(), Value::String(title.to_string())),
        ]),
    ))))
}

pub async fn delete_meeting(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let result = client
        .execute(
            "DELETE FROM x_meeting WHERE id = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if result == 0 {
        return Ok(Json(ActionResult::error("meeting not found")));
    }

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
            ("deleted".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn meeting_id_accept(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    client
        .execute(
            "UPDATE x_meeting SET status = 'accepted' WHERE id = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
            ("accepted".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn meeting_id_add_invite(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
    axum::extract::Json(payload): axum::extract::Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let invitee = payload.get("invitee").and_then(|v| v.as_str()).ok_or(AppError::BadRequest("invitee is required".to_string()))?;
    let invite_id = uuid::Uuid::new_v4().to_string();

    client
        .execute(
            "INSERT INTO x_meeting_invite (id, meeting_id, invitee, status, create_time) VALUES ($1, $2, $3, 'wait', NOW())",
            &[&invite_id, &id, &invitee],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(invite_id)),
            ("meetingId".to_string(), Value::String(id)),
            ("invitee".to_string(), Value::String(invitee.to_string())),
            ("added".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn meeting_id_checkin(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
    axum::extract::Json(payload): axum::extract::Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let person = payload.get("person").and_then(|v| v.as_str()).ok_or(AppError::BadRequest("person is required".to_string()))?;
    let checkin_id = uuid::Uuid::new_v4().to_string();

    client
        .execute(
            "INSERT INTO x_meeting_checkin (id, meeting_id, person, checkin_time) VALUES ($1, $2, $3, NOW())",
            &[&checkin_id, &id, &person],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(checkin_id)),
            ("meetingId".to_string(), Value::String(id)),
            ("person".to_string(), Value::String(person.to_string())),
            ("checkedIn".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn meeting_id_checkin_code(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let row = client
        .query_opt(
            "SELECT checkin_code, expire_time FROM x_meeting_checkin_code WHERE meeting_id = $1 ORDER BY create_time DESC LIMIT 1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let result = Value::Object(serde_json::Map::from_iter([
                ("meetingId".to_string(), Value::String(id)),
                ("checkinCode".to_string(), Value::String(row.get("checkin_code"))),
                ("expireTime".to_string(), Value::String(row.get("expire_time"))),
            ]));
            Ok(Json(ActionResult::success(result)))
        }
        None => Ok(Json(ActionResult::error("checkin code not found"))),
    }
}

pub async fn meeting_id_confirm_allow(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    client
        .execute(
            "UPDATE x_meeting SET status = 'confirmed' WHERE id = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
            ("confirmed".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn meeting_id_confirm_deny(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    client
        .execute(
            "UPDATE x_meeting SET status = 'denied' WHERE id = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
            ("denied".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn meeting_id_delete_invite(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
    axum::extract::Json(payload): axum::extract::Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let invitee = payload.get("invitee").and_then(|v| v.as_str()).ok_or(AppError::BadRequest("invitee is required".to_string()))?;

    let result = client
        .execute(
            "DELETE FROM x_meeting_invite WHERE meeting_id = $1 AND invitee = $2",
            &[&id, &invitee],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
            ("invitee".to_string(), Value::String(invitee.to_string())),
            ("deleted".to_string(), Value::Bool(result > 0)),
        ]),
    ))))
}

pub async fn meeting_id_manual_completed(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    client
        .execute(
            "UPDATE x_meeting SET status = 'completed' WHERE id = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
            ("completed".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn meeting_id_modify_completedtime(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
    axum::extract::Json(payload): axum::extract::Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let completed_time = payload.get("completedTime").and_then(|v| v.as_str()).ok_or(AppError::BadRequest("completedTime is required".to_string()))?;

    client
        .execute(
            "UPDATE x_meeting SET completed_time = $1 WHERE id = $2",
            &[&completed_time, &id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
            ("completedTime".to_string(), Value::String(completed_time.to_string())),
            ("modified".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn meeting_id_modify_starttime(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
    axum::extract::Json(payload): axum::extract::Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let start_time = payload.get("startTime").and_then(|v| v.as_str()).ok_or(AppError::BadRequest("startTime is required".to_string()))?;
    let end_time = payload.get("endTime").and_then(|v| v.as_str());

    if let Some(end_time) = end_time {
        client
            .execute(
                "UPDATE x_meeting SET start_time = $1, end_time = $2 WHERE id = $3",
                &[&start_time, &end_time, &id],
            )
            .await
            .map_err(|_| AppError::Internal)?;
    } else {
        client
            .execute(
                "UPDATE x_meeting SET start_time = $1 WHERE id = $2",
                &[&start_time, &id],
            )
            .await
            .map_err(|_| AppError::Internal)?;
    }

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
            ("startTime".to_string(), Value::String(start_time.to_string())),
            ("modified".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn meeting_id_reject(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    client
        .execute(
            "UPDATE x_meeting SET status = 'rejected' WHERE id = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
            ("rejected".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn openmeeting_list_room(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query(
            "SELECT id, name, pinyin, pinyin_initial, address, description, create_time FROM x_meeting_room WHERE open_meeting = true ORDER BY create_time",
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
                ("pinyin".to_string(), Value::String(row.get::<_, Option<String>>("pinyin").unwrap_or_default())),
                ("pinyinInitial".to_string(), Value::String(row.get::<_, Option<String>>("pinyin_initial").unwrap_or_default())),
                ("address".to_string(), Value::String(row.get::<_, Option<String>>("address").unwrap_or_default())),
                ("description".to_string(), Value::String(row.get::<_, Option<String>>("description").unwrap_or_default())),
                ("createTime".to_string(), Value::String(row.get("create_time"))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

pub async fn room_list(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query(
            "SELECT id, name, pinyin, pinyin_initial, address, description, create_time FROM x_meeting_room ORDER BY create_time",
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
                ("pinyin".to_string(), Value::String(row.get::<_, Option<String>>("pinyin").unwrap_or_default())),
                ("pinyinInitial".to_string(), Value::String(row.get::<_, Option<String>>("pinyin_initial").unwrap_or_default())),
                ("address".to_string(), Value::String(row.get::<_, Option<String>>("address").unwrap_or_default())),
                ("description".to_string(), Value::String(row.get::<_, Option<String>>("description").unwrap_or_default())),
                ("createTime".to_string(), Value::String(row.get("create_time"))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

pub async fn room_list_like_pinyin_key(
    pool: Extension<Pool>,
    axum::extract::Path(key): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let pattern = format!("%{}%", key);
    let rows = client
        .query(
            "SELECT id, name, pinyin, pinyin_initial, address, description, create_time FROM x_meeting_room WHERE pinyin ILIKE $1 ORDER BY create_time",
            &[&pattern],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("name".to_string(), Value::String(row.get("name"))),
                ("pinyin".to_string(), Value::String(row.get::<_, Option<String>>("pinyin").unwrap_or_default())),
                ("pinyinInitial".to_string(), Value::String(row.get::<_, Option<String>>("pinyin_initial").unwrap_or_default())),
                ("address".to_string(), Value::String(row.get::<_, Option<String>>("address").unwrap_or_default())),
                ("description".to_string(), Value::String(row.get::<_, Option<String>>("description").unwrap_or_default())),
                ("createTime".to_string(), Value::String(row.get("create_time"))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

pub async fn room_list_like_key(
    pool: Extension<Pool>,
    axum::extract::Path(key): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let pattern = format!("%{}%", key);
    let rows = client
        .query(
            "SELECT id, name, pinyin, pinyin_initial, address, description, create_time FROM x_meeting_room WHERE name ILIKE $1 OR address ILIKE $1 ORDER BY create_time",
            &[&pattern],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("name".to_string(), Value::String(row.get("name"))),
                ("pinyin".to_string(), Value::String(row.get::<_, Option<String>>("pinyin").unwrap_or_default())),
                ("pinyinInitial".to_string(), Value::String(row.get::<_, Option<String>>("pinyin_initial").unwrap_or_default())),
                ("address".to_string(), Value::String(row.get::<_, Option<String>>("address").unwrap_or_default())),
                ("description".to_string(), Value::String(row.get::<_, Option<String>>("description").unwrap_or_default())),
                ("createTime".to_string(), Value::String(row.get("create_time"))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

pub async fn room_list_pinyininitial_key(
    pool: Extension<Pool>,
    axum::extract::Path(key): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query(
            "SELECT id, name, pinyin, pinyin_initial, address, description, create_time FROM x_meeting_room WHERE pinyin_initial ILIKE $1 ORDER BY create_time",
            &[&key],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("name".to_string(), Value::String(row.get("name"))),
                ("pinyin".to_string(), Value::String(row.get::<_, Option<String>>("pinyin").unwrap_or_default())),
                ("pinyinInitial".to_string(), Value::String(row.get::<_, Option<String>>("pinyin_initial").unwrap_or_default())),
                ("address".to_string(), Value::String(row.get::<_, Option<String>>("address").unwrap_or_default())),
                ("description".to_string(), Value::String(row.get::<_, Option<String>>("description").unwrap_or_default())),
                ("createTime".to_string(), Value::String(row.get("create_time"))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

pub async fn room_id(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let row = client
        .query_opt(
            "SELECT id, name, pinyin, pinyin_initial, address, description, create_time FROM x_meeting_room WHERE id = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let result = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("name".to_string(), Value::String(row.get("name"))),
                ("pinyin".to_string(), Value::String(row.get::<_, Option<String>>("pinyin").unwrap_or_default())),
                ("pinyinInitial".to_string(), Value::String(row.get::<_, Option<String>>("pinyin_initial").unwrap_or_default())),
                ("address".to_string(), Value::String(row.get::<_, Option<String>>("address").unwrap_or_default())),
                ("description".to_string(), Value::String(row.get::<_, Option<String>>("description").unwrap_or_default())),
                ("createTime".to_string(), Value::String(row.get("create_time"))),
            ]));
            Ok(Json(ActionResult::success(result)))
        }
        None => Ok(Json(ActionResult::error("room not found"))),
    }
}

pub async fn room_id_photo(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let row = client
        .query_opt(
            "SELECT photo_url, photo_name, create_time FROM x_meeting_room_photo WHERE room_id = $1 ORDER BY create_time DESC LIMIT 1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let result = Value::Object(serde_json::Map::from_iter([
                ("roomId".to_string(), Value::String(id)),
                ("photoUrl".to_string(), Value::String(row.get("photo_url"))),
                ("photoName".to_string(), Value::String(row.get("photo_name"))),
                ("createTime".to_string(), Value::String(row.get("create_time"))),
            ]));
            Ok(Json(ActionResult::success(result)))
        }
        None => Ok(Json(ActionResult::error("photo not found"))),
    }
}
