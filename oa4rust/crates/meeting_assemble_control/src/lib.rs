use axum::{
    extract::Extension,
    Json, Router,
};
use base64::Engine as _;
use deadpool_postgres::Pool;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use shared::{error::AppError, response::ActionResult};

pub mod routes;

#[cfg(test)]
mod tests;
#[cfg(test)]
mod tests_generated;
#[cfg(test)]
mod tests_u2;



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
                ("\"meetingId\"".to_string(), Value::String(row.get("meeting_id"))),
                ("\"controlType\"".to_string(), Value::String(row.get("control_type"))),
                ("enabled".to_string(), Value::Bool(row.get("enabled"))),
                ("config".to_string(), Value::String(row.get::<_, Option<String>>("config").unwrap_or_default())),
            ]))
        })
        .collect();

    let count = data.len() as i64;
    Ok(Json(ActionResult::java_success(Value::Array(data), count, 0)))
}

pub async fn create_meeting_control(
    pool: Extension<Pool>,
    axum::extract::Json(payload): axum::extract::Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let meeting_id = payload.get("\"meetingId\"").and_then(|v| v.as_str()).ok_or(AppError::BadRequest("\"meetingId\" is required".to_string()))?;
    let control_type = payload.get("\"controlType\"").and_then(|v| v.as_str()).ok_or(AppError::BadRequest("\"controlType\" is required".to_string()))?;
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
        ("\"meetingId\"".to_string(), Value::String(meeting_id.to_string())),
        ("\"controlType\"".to_string(), Value::String(control_type.to_string())),
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
            "SELECT id, name, pinyin, pinyin_initial, address, description, to_char(create_time, 'YYYY-MM-DD HH24:MI:SS') AS create_time FROM x_meeting_building ORDER BY create_time",
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

    let count = data.len() as i64;
    Ok(Json(ActionResult::java_success(Value::Array(data), count, 0)))
}

pub async fn building_list_like_pinyin_key(
    pool: Extension<Pool>,
    axum::extract::Path(key): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let pattern = format!("%{}%", key);
    let rows = client
        .query(
            "SELECT id, name, pinyin, pinyin_initial, address, description, to_char(create_time, 'YYYY-MM-DD HH24:MI:SS') AS create_time FROM x_meeting_building WHERE pinyin ILIKE $1 ORDER BY create_time",
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

    let count = data.len() as i64;
    Ok(Json(ActionResult::java_success(Value::Array(data), count, 0)))
}

pub async fn building_list_like_key(
    pool: Extension<Pool>,
    axum::extract::Path(key): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let pattern = format!("%{}%", key);
    let rows = client
        .query(
            "SELECT id, name, pinyin, pinyin_initial, address, description, to_char(create_time, 'YYYY-MM-DD HH24:MI:SS') AS create_time FROM x_meeting_building WHERE name ILIKE $1 OR address ILIKE $1 ORDER BY create_time",
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

    let count = data.len() as i64;
    Ok(Json(ActionResult::java_success(Value::Array(data), count, 0)))
}

pub async fn building_list_pinyininitial_key(
    pool: Extension<Pool>,
    axum::extract::Path(key): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query(
            "SELECT id, name, pinyin, pinyin_initial, address, description, to_char(create_time, 'YYYY-MM-DD HH24:MI:SS') AS create_time FROM x_meeting_building WHERE pinyin_initial ILIKE $1 ORDER BY create_time",
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

    let count = data.len() as i64;
    Ok(Json(ActionResult::java_success(Value::Array(data), count, 0)))
}

pub async fn building_list_start_start_completed_completed(
    pool: Extension<Pool>,
    axum::extract::Path((start, completed)): axum::extract::Path<(String, String)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query(
            "SELECT id, name, pinyin, pinyin_initial, address, description, to_char(create_time, 'YYYY-MM-DD HH24:MI:SS') AS create_time FROM x_meeting_building WHERE start_time >= $1 AND end_time <= $2 ORDER BY create_time",
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

    let count = data.len() as i64;
    Ok(Json(ActionResult::java_success(Value::Array(data), count, 0)))
}

pub async fn building_list_start_start_completed_completed_allmeeting(
    pool: Extension<Pool>,
    axum::extract::Path((start, completed)): axum::extract::Path<(String, String)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query(
            "SELECT id, name, pinyin, pinyin_initial, address, description, to_char(create_time, 'YYYY-MM-DD HH24:MI:SS') AS create_time FROM x_meeting_building WHERE start_time >= $1 AND end_time <= $2 ORDER BY create_time",
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

    let count = data.len() as i64;
    Ok(Json(ActionResult::java_success(Value::Array(data), count, 0)))
}

pub async fn building_list_start_start_completed_completed_room_room_meeting_meeting(
    pool: Extension<Pool>,
    axum::extract::Path((start, completed, room, meeting)): axum::extract::Path<(String, String, String, String)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query(
            "SELECT id, name, pinyin, pinyin_initial, address, description, to_char(create_time, 'YYYY-MM-DD HH24:MI:SS') AS create_time FROM x_meeting_building WHERE start_time >= $1 AND end_time <= $2 AND room_id = $3 AND meeting_id = $4 ORDER BY create_time",
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

    let count = data.len() as i64;
    Ok(Json(ActionResult::java_success(Value::Array(data), count, 0)))
}

pub async fn building_id(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let row = client
        .query_opt(
            "SELECT id, name, pinyin, pinyin_initial, address, description, to_char(create_time, 'YYYY-MM-DD HH24:MI:SS') AS create_time FROM x_meeting_building WHERE id = $1",
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
                ("\"configKey\"".to_string(), Value::String(row.get("config_key"))),
                ("\"configValue\"".to_string(), Value::String(row.get("config_value"))),
                ("description".to_string(), Value::String(row.get::<_, Option<String>>("description").unwrap_or_default())),
                ("createTime".to_string(), Value::String(row.get("create_time"))),
            ]))
        })
        .collect();

    let count = data.len() as i64;
    Ok(Json(ActionResult::java_success(Value::Array(data), count, 0)))
}

pub async fn config_system_config_manage(
    pool: Extension<Pool>,
    axum::extract::Json(payload): axum::extract::Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let config_key = payload.get("\"configKey\"").and_then(|v| v.as_str()).ok_or(AppError::BadRequest("\"configKey\" is required".to_string()))?;
    let config_value = payload.get("\"configValue\"").and_then(|v| v.as_str()).ok_or(AppError::BadRequest("\"configValue\" is required".to_string()))?;
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
            ("\"configKey\"".to_string(), Value::String(config_key.to_string())),
            ("\"configValue\"".to_string(), Value::String(config_value.to_string())),
            ("updated".to_string(), Value::Bool(result > 0)),
        ]),
    ))))
}

pub async fn meeting_list_applied_completed(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query(
            "SELECT id, title, content, to_char(start_time, 'YYYY-MM-DD HH24:MI:SS') AS start_time, to_char(end_time, 'YYYY-MM-DD HH24:MI:SS') AS end_time, creator, to_char(create_time, 'YYYY-MM-DD HH24:MI:SS') AS create_time FROM x_meeting WHERE status = 'completed' AND applied = true ORDER BY create_time DESC",
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
                ("\"startTime\"".to_string(), Value::String(row.get("start_time"))),
                ("\"endTime\"".to_string(), Value::String(row.get("end_time"))),
                ("creator".to_string(), Value::String(row.get("creator"))),
                ("createTime".to_string(), Value::String(row.get("create_time"))),
            ]))
        })
        .collect();

    let count = data.len() as i64;
    Ok(Json(ActionResult::java_success(Value::Array(data), count, 0)))
}

pub async fn meeting_list_applied_processing(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query(
            "SELECT id, title, content, to_char(start_time, 'YYYY-MM-DD HH24:MI:SS') AS start_time, to_char(end_time, 'YYYY-MM-DD HH24:MI:SS') AS end_time, creator, to_char(create_time, 'YYYY-MM-DD HH24:MI:SS') AS create_time FROM x_meeting WHERE status = 'processing' AND applied = true ORDER BY create_time DESC",
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
                ("\"startTime\"".to_string(), Value::String(row.get("start_time"))),
                ("\"endTime\"".to_string(), Value::String(row.get("end_time"))),
                ("creator".to_string(), Value::String(row.get("creator"))),
                ("createTime".to_string(), Value::String(row.get("create_time"))),
            ]))
        })
        .collect();

    let count = data.len() as i64;
    Ok(Json(ActionResult::java_success(Value::Array(data), count, 0)))
}

pub async fn meeting_list_applied_wait(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query(
            "SELECT id, title, content, to_char(start_time, 'YYYY-MM-DD HH24:MI:SS') AS start_time, to_char(end_time, 'YYYY-MM-DD HH24:MI:SS') AS end_time, creator, to_char(create_time, 'YYYY-MM-DD HH24:MI:SS') AS create_time FROM x_meeting WHERE status = 'wait' AND applied = true ORDER BY create_time DESC",
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
                ("\"startTime\"".to_string(), Value::String(row.get("start_time"))),
                ("\"endTime\"".to_string(), Value::String(row.get("end_time"))),
                ("creator".to_string(), Value::String(row.get("creator"))),
                ("createTime".to_string(), Value::String(row.get("create_time"))),
            ]))
        })
        .collect();

    let count = data.len() as i64;
    Ok(Json(ActionResult::java_success(Value::Array(data), count, 0)))
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
            "SELECT id, title, content, to_char(start_time, 'YYYY-MM-DD HH24:MI:SS') AS start_time, to_char(end_time, 'YYYY-MM-DD HH24:MI:SS') AS end_time, creator, to_char(create_time, 'YYYY-MM-DD HH24:MI:SS') AS create_time FROM x_meeting WHERE applied = true ORDER BY create_time DESC LIMIT $1::bigint OFFSET $2::bigint",
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
                ("\"startTime\"".to_string(), Value::String(row.get("start_time"))),
                ("\"endTime\"".to_string(), Value::String(row.get("end_time"))),
                ("creator".to_string(), Value::String(row.get("creator"))),
                ("createTime".to_string(), Value::String(row.get("create_time"))),
            ]))
        })
        .collect();

    let count = data.len() as i64;
    Ok(Json(ActionResult::java_success(Value::Array(data), count, size)))
}

pub async fn meeting_list_coming_day_count(
    pool: Extension<Pool>,
    axum::extract::Path(count): axum::extract::Path<i64>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query(
            "SELECT id, title, content, to_char(start_time, 'YYYY-MM-DD HH24:MI:SS') AS start_time, to_char(end_time, 'YYYY-MM-DD HH24:MI:SS') AS end_time, creator, to_char(create_time, 'YYYY-MM-DD HH24:MI:SS') AS create_time FROM x_meeting WHERE start_time >= NOW() AND start_time <= NOW() + INTERVAL '1 day' * $1 ORDER BY start_time ASC",
            &[&count],
        )
        .await
        .map_err(|e| { eprintln!("DIAG meeting_coming_day query err: {:?}", e); AppError::Internal })?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("title".to_string(), Value::String(row.get("title"))),
                ("content".to_string(), Value::String(row.get::<_, Option<String>>("content").unwrap_or_default())),
                ("\"startTime\"".to_string(), Value::String(row.get("start_time"))),
                ("\"endTime\"".to_string(), Value::String(row.get("end_time"))),
                ("creator".to_string(), Value::String(row.get("creator"))),
                ("createTime".to_string(), Value::String(row.get("create_time"))),
            ]))
        })
        .collect();

    let count = data.len() as i64;
    Ok(Json(ActionResult::java_success(Value::Array(data), count, 0)))
}

pub async fn meeting_list_coming_month_count(
    pool: Extension<Pool>,
    axum::extract::Path(count): axum::extract::Path<i64>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query(
            "SELECT id, title, content, to_char(start_time, 'YYYY-MM-DD HH24:MI:SS') AS start_time, to_char(end_time, 'YYYY-MM-DD HH24:MI:SS') AS end_time, creator, to_char(create_time, 'YYYY-MM-DD HH24:MI:SS') AS create_time FROM x_meeting WHERE start_time >= NOW() AND start_time <= NOW() + INTERVAL '1 month' * $1 ORDER BY start_time ASC",
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
                ("\"startTime\"".to_string(), Value::String(row.get("start_time"))),
                ("\"endTime\"".to_string(), Value::String(row.get("end_time"))),
                ("creator".to_string(), Value::String(row.get("creator"))),
                ("createTime".to_string(), Value::String(row.get("create_time"))),
            ]))
        })
        .collect();

    let count = data.len() as i64;
    Ok(Json(ActionResult::java_success(Value::Array(data), count, 0)))
}

pub async fn meeting_list_forward_monthcount_monthCount(
    pool: Extension<Pool>,
    axum::extract::Path(month_count): axum::extract::Path<i64>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query(
            "SELECT id, title, content, to_char(start_time, 'YYYY-MM-DD HH24:MI:SS') AS start_time, to_char(end_time, 'YYYY-MM-DD HH24:MI:SS') AS end_time, creator, to_char(create_time, 'YYYY-MM-DD HH24:MI:SS') AS create_time FROM x_meeting WHERE start_time >= NOW() AND start_time <= NOW() + INTERVAL '1 month' * $1 ORDER BY start_time ASC",
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
                ("\"startTime\"".to_string(), Value::String(row.get("start_time"))),
                ("\"endTime\"".to_string(), Value::String(row.get("end_time"))),
                ("creator".to_string(), Value::String(row.get("creator"))),
                ("createTime".to_string(), Value::String(row.get("create_time"))),
            ]))
        })
        .collect();

    let count = data.len() as i64;
    Ok(Json(ActionResult::java_success(Value::Array(data), count, 0)))
}

pub async fn meeting_list_forward_monthcount_monthCount_all(
    pool: Extension<Pool>,
    axum::extract::Path(month_count): axum::extract::Path<i64>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query(
            "SELECT id, title, content, to_char(start_time, 'YYYY-MM-DD HH24:MI:SS') AS start_time, to_char(end_time, 'YYYY-MM-DD HH24:MI:SS') AS end_time, creator, to_char(create_time, 'YYYY-MM-DD HH24:MI:SS') AS create_time FROM x_meeting WHERE start_time >= NOW() AND start_time <= NOW() + INTERVAL '1 month' * $1 ORDER BY start_time ASC",
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
                ("\"startTime\"".to_string(), Value::String(row.get("start_time"))),
                ("\"endTime\"".to_string(), Value::String(row.get("end_time"))),
                ("creator".to_string(), Value::String(row.get("creator"))),
                ("createTime".to_string(), Value::String(row.get("create_time"))),
            ]))
        })
        .collect();

    let count = data.len() as i64;
    Ok(Json(ActionResult::java_success(Value::Array(data), count, 0)))
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
            "SELECT id, title, content, to_char(start_time, 'YYYY-MM-DD HH24:MI:SS') AS start_time, to_char(end_time, 'YYYY-MM-DD HH24:MI:SS') AS end_time, creator, to_char(create_time, 'YYYY-MM-DD HH24:MI:SS') AS create_time FROM x_meeting WHERE invited = true ORDER BY create_time DESC LIMIT $1::bigint OFFSET $2::bigint",
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
                ("\"startTime\"".to_string(), Value::String(row.get("start_time"))),
                ("\"endTime\"".to_string(), Value::String(row.get("end_time"))),
                ("creator".to_string(), Value::String(row.get("creator"))),
                ("createTime".to_string(), Value::String(row.get("create_time"))),
            ]))
        })
        .collect();

    let count = data.len() as i64;
    Ok(Json(ActionResult::java_success(Value::Array(data), count, size)))
}

pub async fn meeting_list_invited_completed(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query(
            "SELECT id, title, content, to_char(start_time, 'YYYY-MM-DD HH24:MI:SS') AS start_time, to_char(end_time, 'YYYY-MM-DD HH24:MI:SS') AS end_time, creator, to_char(create_time, 'YYYY-MM-DD HH24:MI:SS') AS create_time FROM x_meeting WHERE status = 'completed' AND invited = true ORDER BY create_time DESC",
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
                ("\"startTime\"".to_string(), Value::String(row.get("start_time"))),
                ("\"endTime\"".to_string(), Value::String(row.get("end_time"))),
                ("creator".to_string(), Value::String(row.get("creator"))),
                ("createTime".to_string(), Value::String(row.get("create_time"))),
            ]))
        })
        .collect();

    let count = data.len() as i64;
    Ok(Json(ActionResult::java_success(Value::Array(data), count, 0)))
}

pub async fn meeting_list_invited_processing(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query(
            "SELECT id, title, content, to_char(start_time, 'YYYY-MM-DD HH24:MI:SS') AS start_time, to_char(end_time, 'YYYY-MM-DD HH24:MI:SS') AS end_time, creator, to_char(create_time, 'YYYY-MM-DD HH24:MI:SS') AS create_time FROM x_meeting WHERE status = 'processing' AND invited = true ORDER BY create_time DESC",
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
                ("\"startTime\"".to_string(), Value::String(row.get("start_time"))),
                ("\"endTime\"".to_string(), Value::String(row.get("end_time"))),
                ("creator".to_string(), Value::String(row.get("creator"))),
                ("createTime".to_string(), Value::String(row.get("create_time"))),
            ]))
        })
        .collect();

    let count = data.len() as i64;
    Ok(Json(ActionResult::java_success(Value::Array(data), count, 0)))
}

pub async fn meeting_list_invited_rejected(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query(
            "SELECT id, title, content, to_char(start_time, 'YYYY-MM-DD HH24:MI:SS') AS start_time, to_char(end_time, 'YYYY-MM-DD HH24:MI:SS') AS end_time, creator, to_char(create_time, 'YYYY-MM-DD HH24:MI:SS') AS create_time FROM x_meeting WHERE status = 'rejected' AND invited = true ORDER BY create_time DESC",
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
                ("\"startTime\"".to_string(), Value::String(row.get("start_time"))),
                ("\"endTime\"".to_string(), Value::String(row.get("end_time"))),
                ("creator".to_string(), Value::String(row.get("creator"))),
                ("createTime".to_string(), Value::String(row.get("create_time"))),
            ]))
        })
        .collect();

    let count = data.len() as i64;
    Ok(Json(ActionResult::java_success(Value::Array(data), count, 0)))
}

pub async fn meeting_list_invited_wait(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query(
            "SELECT id, title, content, to_char(start_time, 'YYYY-MM-DD HH24:MI:SS') AS start_time, to_char(end_time, 'YYYY-MM-DD HH24:MI:SS') AS end_time, creator, to_char(create_time, 'YYYY-MM-DD HH24:MI:SS') AS create_time FROM x_meeting WHERE status = 'wait' AND invited = true ORDER BY create_time DESC",
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
                ("\"startTime\"".to_string(), Value::String(row.get("start_time"))),
                ("\"endTime\"".to_string(), Value::String(row.get("end_time"))),
                ("creator".to_string(), Value::String(row.get("creator"))),
                ("createTime".to_string(), Value::String(row.get("create_time"))),
            ]))
        })
        .collect();

    let count = data.len() as i64;
    Ok(Json(ActionResult::java_success(Value::Array(data), count, 0)))
}

pub async fn meeting_list_wait_accept(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query(
            "SELECT id, title, content, to_char(start_time, 'YYYY-MM-DD HH24:MI:SS') AS start_time, to_char(end_time, 'YYYY-MM-DD HH24:MI:SS') AS end_time, creator, to_char(create_time, 'YYYY-MM-DD HH24:MI:SS') AS create_time FROM x_meeting WHERE status = 'wait_accept' ORDER BY create_time DESC",
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
                ("\"startTime\"".to_string(), Value::String(row.get("start_time"))),
                ("\"endTime\"".to_string(), Value::String(row.get("end_time"))),
                ("creator".to_string(), Value::String(row.get("creator"))),
                ("createTime".to_string(), Value::String(row.get("create_time"))),
            ]))
        })
        .collect();

    let count = data.len() as i64;
    Ok(Json(ActionResult::java_success(Value::Array(data), count, 0)))
}

pub async fn meeting_list_wait_confirm(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query(
            "SELECT id, title, content, to_char(start_time, 'YYYY-MM-DD HH24:MI:SS') AS start_time, to_char(end_time, 'YYYY-MM-DD HH24:MI:SS') AS end_time, creator, to_char(create_time, 'YYYY-MM-DD HH24:MI:SS') AS create_time FROM x_meeting WHERE status = 'wait_confirm' ORDER BY create_time DESC",
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
                ("\"startTime\"".to_string(), Value::String(row.get("start_time"))),
                ("\"endTime\"".to_string(), Value::String(row.get("end_time"))),
                ("creator".to_string(), Value::String(row.get("creator"))),
                ("createTime".to_string(), Value::String(row.get("create_time"))),
            ]))
        })
        .collect();

    let count = data.len() as i64;
    Ok(Json(ActionResult::java_success(Value::Array(data), count, 0)))
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
            "SELECT id, title, content, to_char(start_time, 'YYYY-MM-DD HH24:MI:SS') AS start_time, to_char(end_time, 'YYYY-MM-DD HH24:MI:SS') AS end_time, creator, to_char(create_time, 'YYYY-MM-DD HH24:MI:SS') AS create_time FROM x_meeting WHERE start_time >= $1 AND start_time < $2 ORDER BY start_time ASC",
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
                ("\"startTime\"".to_string(), Value::String(row.get("start_time"))),
                ("\"endTime\"".to_string(), Value::String(row.get("end_time"))),
                ("creator".to_string(), Value::String(row.get("creator"))),
                ("createTime".to_string(), Value::String(row.get("create_time"))),
            ]))
        })
        .collect();

    let count = data.len() as i64;
    Ok(Json(ActionResult::java_success(Value::Array(data), count, 0)))
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
            "SELECT id, title, content, to_char(start_time, 'YYYY-MM-DD HH24:MI:SS') AS start_time, to_char(end_time, 'YYYY-MM-DD HH24:MI:SS') AS end_time, creator, to_char(create_time, 'YYYY-MM-DD HH24:MI:SS') AS create_time FROM x_meeting WHERE start_time >= $1 AND start_time < $2 ORDER BY start_time ASC",
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
                ("\"startTime\"".to_string(), Value::String(row.get("start_time"))),
                ("\"endTime\"".to_string(), Value::String(row.get("end_time"))),
                ("creator".to_string(), Value::String(row.get("creator"))),
                ("createTime".to_string(), Value::String(row.get("create_time"))),
            ]))
        })
        .collect();

    let count = data.len() as i64;
    Ok(Json(ActionResult::java_success(Value::Array(data), count, 0)))
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
            "SELECT id, title, content, to_char(start_time, 'YYYY-MM-DD HH24:MI:SS') AS start_time, to_char(end_time, 'YYYY-MM-DD HH24:MI:SS') AS end_time, creator, to_char(create_time, 'YYYY-MM-DD HH24:MI:SS') AS create_time FROM x_meeting WHERE start_time >= $1 AND start_time <= $2 ORDER BY start_time ASC",
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
                ("\"startTime\"".to_string(), Value::String(row.get("start_time"))),
                ("\"endTime\"".to_string(), Value::String(row.get("end_time"))),
                ("creator".to_string(), Value::String(row.get("creator"))),
                ("createTime".to_string(), Value::String(row.get("create_time"))),
            ]))
        })
        .collect();

    let count = data.len() as i64;
    Ok(Json(ActionResult::java_success(Value::Array(data), count, 0)))
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
            "SELECT id, title, content, to_char(start_time, 'YYYY-MM-DD HH24:MI:SS') AS start_time, to_char(end_time, 'YYYY-MM-DD HH24:MI:SS') AS end_time, creator, to_char(create_time, 'YYYY-MM-DD HH24:MI:SS') AS create_time FROM x_meeting WHERE start_time >= $1 AND start_time <= $2 ORDER BY start_time ASC",
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
                ("\"startTime\"".to_string(), Value::String(row.get("start_time"))),
                ("\"endTime\"".to_string(), Value::String(row.get("end_time"))),
                ("creator".to_string(), Value::String(row.get("creator"))),
                ("createTime".to_string(), Value::String(row.get("create_time"))),
            ]))
        })
        .collect();

    let count = data.len() as i64;
    Ok(Json(ActionResult::java_success(Value::Array(data), count, 0)))
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
            "SELECT id, title, content, to_char(start_time, 'YYYY-MM-DD HH24:MI:SS') AS start_time, to_char(end_time, 'YYYY-MM-DD HH24:MI:SS') AS end_time, creator, to_char(create_time, 'YYYY-MM-DD HH24:MI:SS') AS create_time FROM x_meeting WHERE start_time >= $1 AND start_time <= $2 AND room_id = $3 ORDER BY start_time ASC",
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
                ("\"startTime\"".to_string(), Value::String(row.get("start_time"))),
                ("\"endTime\"".to_string(), Value::String(row.get("end_time"))),
                ("creator".to_string(), Value::String(row.get("creator"))),
                ("createTime".to_string(), Value::String(row.get("create_time"))),
            ]))
        })
        .collect();

    let count = data.len() as i64;
    Ok(Json(ActionResult::java_success(Value::Array(data), count, 0)))
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
                "SELECT id, title, content, to_char(start_time, 'YYYY-MM-DD HH24:MI:SS') AS start_time, to_char(end_time, 'YYYY-MM-DD HH24:MI:SS') AS end_time, creator, to_char(create_time, 'YYYY-MM-DD HH24:MI:SS') AS create_time FROM x_meeting ORDER BY create_time DESC LIMIT $1::bigint",
                &[&count],
            )
            .await
            .map_err(|_| AppError::Internal)?
    } else {
        client
            .query(
                "SELECT id, title, content, to_char(start_time, 'YYYY-MM-DD HH24:MI:SS') AS start_time, to_char(end_time, 'YYYY-MM-DD HH24:MI:SS') AS end_time, creator, to_char(create_time, 'YYYY-MM-DD HH24:MI:SS') AS create_time FROM x_meeting WHERE id > $1 ORDER BY create_time DESC LIMIT $2::bigint",
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
                ("\"startTime\"".to_string(), Value::String(row.get("start_time"))),
                ("\"endTime\"".to_string(), Value::String(row.get("end_time"))),
                ("creator".to_string(), Value::String(row.get("creator"))),
                ("createTime".to_string(), Value::String(row.get("create_time"))),
            ]))
        })
        .collect();

    let count = data.len() as i64;
    Ok(Json(ActionResult::java_success(Value::Array(data), count, 0)))
}

pub async fn meeting_list_id_prev_count(
    pool: Extension<Pool>,
    axum::extract::Path((id, count)): axum::extract::Path<(String, i64)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let count = count.max(1);

    let rows = client
        .query(
            "SELECT id, title, content, to_char(start_time, 'YYYY-MM-DD HH24:MI:SS') AS start_time, to_char(end_time, 'YYYY-MM-DD HH24:MI:SS') AS end_time, creator, to_char(create_time, 'YYYY-MM-DD HH24:MI:SS') AS create_time FROM x_meeting WHERE id < $1 ORDER BY create_time DESC LIMIT $2::bigint",
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
                ("\"startTime\"".to_string(), Value::String(row.get("start_time"))),
                ("\"endTime\"".to_string(), Value::String(row.get("end_time"))),
                ("creator".to_string(), Value::String(row.get("creator"))),
                ("createTime".to_string(), Value::String(row.get("create_time"))),
            ]))
        })
        .collect();

    let count = data.len() as i64;
    Ok(Json(ActionResult::java_success(Value::Array(data), count, 0)))
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
            "SELECT id, title, content, to_char(start_time, 'YYYY-MM-DD HH24:MI:SS') AS start_time, to_char(end_time, 'YYYY-MM-DD HH24:MI:SS') AS end_time, creator, to_char(create_time, 'YYYY-MM-DD HH24:MI:SS') AS create_time FROM x_meeting ORDER BY create_time DESC LIMIT $1::bigint OFFSET $2::bigint",
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
                ("\"startTime\"".to_string(), Value::String(row.get("start_time"))),
                ("\"endTime\"".to_string(), Value::String(row.get("end_time"))),
                ("creator".to_string(), Value::String(row.get("creator"))),
                ("createTime".to_string(), Value::String(row.get("create_time"))),
            ]))
        })
        .collect();

    let count = data.len() as i64;
    Ok(Json(ActionResult::java_success(Value::Array(data), count, size)))
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
            "SELECT id, title, content, to_char(start_time, 'YYYY-MM-DD HH24:MI:SS') AS start_time, to_char(end_time, 'YYYY-MM-DD HH24:MI:SS') AS end_time, creator, to_char(create_time, 'YYYY-MM-DD HH24:MI:SS') AS create_time FROM x_meeting ORDER BY create_time DESC LIMIT $1::bigint OFFSET $2::bigint",
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
                ("\"startTime\"".to_string(), Value::String(row.get("start_time"))),
                ("\"endTime\"".to_string(), Value::String(row.get("end_time"))),
                ("creator".to_string(), Value::String(row.get("creator"))),
                ("createTime".to_string(), Value::String(row.get("create_time"))),
            ]))
        })
        .collect();

    let count = data.len() as i64;
    Ok(Json(ActionResult::java_success(Value::Array(data), count, size)))
}

pub async fn meeting_id(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let row = client
        .query_opt(
            "SELECT id, title, content, to_char(start_time, 'YYYY-MM-DD HH24:MI:SS') AS start_time, to_char(end_time, 'YYYY-MM-DD HH24:MI:SS') AS end_time, creator, to_char(create_time, 'YYYY-MM-DD HH24:MI:SS') AS create_time FROM x_meeting WHERE id = $1",
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
                ("\"startTime\"".to_string(), Value::String(row.get("start_time"))),
                ("\"endTime\"".to_string(), Value::String(row.get("end_time"))),
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
    let start_time = payload.get("\"startTime\"").and_then(|v| v.as_str()).ok_or(AppError::BadRequest("\"startTime\" is required".to_string()))?;
    let end_time = payload.get("\"endTime\"").and_then(|v| v.as_str()).ok_or(AppError::BadRequest("\"endTime\" is required".to_string()))?;
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
        ("\"startTime\"".to_string(), Value::String(start_time.to_string())),
        ("\"endTime\"".to_string(), Value::String(end_time.to_string())),
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
    let start_time = payload.get("\"startTime\"").and_then(|v| v.as_str()).unwrap_or_default();
    let end_time = payload.get("\"endTime\"").and_then(|v| v.as_str()).unwrap_or_default();

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
            ("saved".to_string(), Value::Bool(result > 0)),
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
            ("deleted".to_string(), Value::Bool(result > 0)),
        ]),
    ))))
}

pub async fn meeting_id_accept(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let result = client
        .execute(
            "UPDATE x_meeting SET status = 'accepted' WHERE id = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
            ("accepted".to_string(), Value::Bool(result > 0)),
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

    let result = client
        .execute(
            "INSERT INTO x_meeting_invite (id, meeting_id, invitee, status, create_time) VALUES ($1, $2, $3, 'wait', NOW())",
            &[&invite_id, &id, &invitee],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(invite_id)),
            ("\"meetingId\"".to_string(), Value::String(id)),
            ("invitee".to_string(), Value::String(invitee.to_string())),
            ("added".to_string(), Value::Bool(result > 0)),
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

    let result = client
        .execute(
            "INSERT INTO x_meeting_checkin (id, meeting_id, person, checkin_time) VALUES ($1, $2, $3, NOW())",
            &[&checkin_id, &id, &person],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(checkin_id)),
            ("\"meetingId\"".to_string(), Value::String(id)),
            ("person".to_string(), Value::String(person.to_string())),
            ("checkedIn".to_string(), Value::Bool(result > 0)),
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
            "SELECT checkin_code, to_char(expire_time, 'YYYY-MM-DD HH24:MI:SS') AS expire_time FROM x_meeting_checkin_code WHERE meeting_id = $1 ORDER BY create_time DESC LIMIT 1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let result = Value::Object(serde_json::Map::from_iter([
                ("\"meetingId\"".to_string(), Value::String(id)),
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

    let result = client
        .execute(
            "UPDATE x_meeting SET status = 'confirmed' WHERE id = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
            ("confirmed".to_string(), Value::Bool(result > 0)),
        ]),
    ))))
}

pub async fn meeting_id_confirm_deny(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let result = client
        .execute(
            "UPDATE x_meeting SET status = 'denied' WHERE id = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
            ("denied".to_string(), Value::Bool(result > 0)),
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

    let result = client
        .execute(
            "UPDATE x_meeting SET status = 'completed' WHERE id = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
            ("completed".to_string(), Value::Bool(result > 0)),
        ]),
    ))))
}

pub async fn meeting_id_modify_completedtime(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
    axum::extract::Json(payload): axum::extract::Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let completed_time = payload.get("\"completedTime\"").and_then(|v| v.as_str()).ok_or(AppError::BadRequest("\"completedTime\" is required".to_string()))?;

    let result = client
        .execute(
            "UPDATE x_meeting SET completed_time = $1 WHERE id = $2",
            &[&completed_time, &id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
            ("\"completedTime\"".to_string(), Value::String(completed_time.to_string())),
            ("modified".to_string(), Value::Bool(result > 0)),
        ]),
    ))))
}

pub async fn meeting_id_modify_starttime(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
    axum::extract::Json(payload): axum::extract::Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let start_time = payload.get("\"startTime\"").and_then(|v| v.as_str()).ok_or(AppError::BadRequest("\"startTime\" is required".to_string()))?;
    let end_time = payload.get("\"endTime\"").and_then(|v| v.as_str());

    let result = if let Some(end_time) = end_time {
        client
            .execute(
                "UPDATE x_meeting SET start_time = $1, end_time = $2 WHERE id = $3",
                &[&start_time, &end_time, &id],
            )
            .await
            .map_err(|_| AppError::Internal)?
    } else {
        client
            .execute(
                "UPDATE x_meeting SET start_time = $1 WHERE id = $2",
                &[&start_time, &id],
            )
            .await
            .map_err(|_| AppError::Internal)?
    };

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
            ("\"startTime\"".to_string(), Value::String(start_time.to_string())),
            ("modified".to_string(), Value::Bool(result > 0)),
        ]),
    ))))
}

pub async fn meeting_id_reject(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let result = client
        .execute(
            "UPDATE x_meeting SET status = 'rejected' WHERE id = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
            ("rejected".to_string(), Value::Bool(result > 0)),
        ]),
    ))))
}

pub async fn openmeeting_list_room(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query(
            "SELECT id, name, pinyin, pinyin_initial, address, description, to_char(create_time, 'YYYY-MM-DD HH24:MI:SS') AS create_time FROM x_meeting_room WHERE open_meeting = true ORDER BY create_time",
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

    let count = data.len() as i64;
    Ok(Json(ActionResult::java_success(Value::Array(data), count, 0)))
}

pub async fn room_list(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query(
            "SELECT id, name, pinyin, pinyin_initial, address, description, to_char(create_time, 'YYYY-MM-DD HH24:MI:SS') AS create_time FROM x_meeting_room ORDER BY create_time",
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

    let count = data.len() as i64;
    Ok(Json(ActionResult::java_success(Value::Array(data), count, 0)))
}

pub async fn room_list_like_pinyin_key(
    pool: Extension<Pool>,
    axum::extract::Path(key): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let pattern = format!("%{}%", key);
    let rows = client
        .query(
            "SELECT id, name, pinyin, pinyin_initial, address, description, to_char(create_time, 'YYYY-MM-DD HH24:MI:SS') AS create_time FROM x_meeting_room WHERE pinyin ILIKE $1 ORDER BY create_time",
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

    let count = data.len() as i64;
    Ok(Json(ActionResult::java_success(Value::Array(data), count, 0)))
}

pub async fn room_list_like_key(
    pool: Extension<Pool>,
    axum::extract::Path(key): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let pattern = format!("%{}%", key);
    let rows = client
        .query(
            "SELECT id, name, pinyin, pinyin_initial, address, description, to_char(create_time, 'YYYY-MM-DD HH24:MI:SS') AS create_time FROM x_meeting_room WHERE name ILIKE $1 OR address ILIKE $1 ORDER BY create_time",
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

    let count = data.len() as i64;
    Ok(Json(ActionResult::java_success(Value::Array(data), count, 0)))
}

pub async fn room_list_pinyininitial_key(
    pool: Extension<Pool>,
    axum::extract::Path(key): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query(
            "SELECT id, name, pinyin, pinyin_initial, address, description, to_char(create_time, 'YYYY-MM-DD HH24:MI:SS') AS create_time FROM x_meeting_room WHERE pinyin_initial ILIKE $1 ORDER BY create_time",
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

    let count = data.len() as i64;
    Ok(Json(ActionResult::java_success(Value::Array(data), count, 0)))
}

pub async fn room_id(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let row = client
        .query_opt(
            "SELECT id, name, pinyin, pinyin_initial, address, description, to_char(create_time, 'YYYY-MM-DD HH24:MI:SS') AS create_time FROM x_meeting_room WHERE id = $1",
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
            "SELECT photo_url, photo_name, to_char(create_time, 'YYYY-MM-DD HH24:MI:SS') AS create_time FROM x_meeting_room_photo WHERE room_id = $1 ORDER BY create_time DESC LIMIT 1",
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

// ════════════ plan002 U2：meeting 模块端点全量闭合（Java jaxrs 76 端点对齐） ════════════
// 语义红线（沿用 file_assemble_control U2 先例，禁止假成功壳）：
//   - 附件上传 = BlobStorage put + 回读校验。FS 后端真实落盘；STORAGE_BACKEND=db 时
//     DbBlobStorage.get 必然 Err -> 显式 501 + warn，不落“内容必丢”的元数据行。
//     FS 模式下 content 列双写 base64，保证下载端点不依赖 blob 后端即可回放。
//   - IDOR 门禁：meeting 写操作 require_owner(meeting.creator)；attachment 写操作经
//     meeting 关联 creator 校验；building/room 编辑对应 Java buildingEditAvailable
//     （manager 或 MeetingManager 角色）-> is_admin 近似。
//   - 归一化查重：building/room 创建与改名时 normalize(name) 冲突检测。
//
// 跨 crate 裁决记录：全部缺口路由均挂 /jaxrs/meeting/assemble/control 前缀
// （本 crate 专属前缀），经归一化查重无跨 crate 占用。

async fn u2_require_admin(pool: &Pool, session: &shared::session::Session) -> Result<(), AppError> {
    if shared::middleware::is_admin(pool, &session.person_unique).await {
        Ok(())
    } else {
        Err(AppError::Forbidden)
    }
}

/// 归一化查重键：trim + 小写 + 折叠内部空白。同名不同形视为冲突。
fn u2_normalize_name(name: &str) -> String {
    name.split_whitespace().collect::<Vec<_>>().join(" ").to_lowercase()
}

/// 规范化附件 blob key：`meeting-attachment/{id}/{filename}`；剥离路径分隔符、
/// 控制字符与穿越形态（双保险：FsBlobStorage.resolve 还会拒绝 `..` 组件）。
fn u2_attachment_blob_key(id: &str, filename: &str) -> Result<String, AppError> {
    let cleaned: String = filename
        .replace(['\\', '/'], "_")
        .chars()
        .filter(|c| !c.is_control() && *c != '"' && *c != '\0')
        .collect();
    let name = cleaned.trim().trim_start_matches('.');
    if name.is_empty() || name == "." || name == ".." {
        return Err(AppError::BadRequest("invalid file name".to_string()));
    }
    Ok(format!("meeting-attachment/{id}/{name}"))
}

/// put + 回读校验。DB 占位后端 get 必然 Err —— 在此显式失败，
/// 避免产生“上传成功但内容丢失”的假成功响应。
async fn u2_persist_blob_verified(key: &str, bytes: &[u8]) -> Result<(), AppError> {
    let storage = shared::storage::storage_from_env();
    storage.put(key, bytes).await.map_err(|e| {
        tracing::warn!(key, error = %e, "blob put failed");
        AppError::Internal
    })?;
    if let Err(e) = storage.get(key).await {
        tracing::warn!(key, error = %e,
            "blob backend did not persist upload (STORAGE_BACKEND=db placeholder); \
             set STORAGE_BACKEND=fs to enable binary uploads");
        return Err(AppError::NotImplemented);
    }
    Ok(())
}

/// meeting.creator 查询（IDOR 门禁前置）。None = 会议不存在。
async fn u2_meeting_creator(client: &deadpool_postgres::tokio_postgres::Client, id: &str) -> Result<Option<String>, AppError> {
    let row = client
        .query_opt("SELECT creator FROM x_meeting WHERE id = $1", &[&id])
        .await
        .map_err(|_| AppError::Internal)?;
    Ok(row.map(|r| r.get::<_, Option<String>>("creator").unwrap_or_default()))
}

/// attachment -> meeting.creator 的 IDOR 门禁（admin 豁免）。
async fn u2_attachment_guard(
    client: &deadpool_postgres::Client,
    pool: &Pool,
    session: &shared::session::Session,
    attachment_id: &str,
) -> Result<String, AppError> {
    let row = client
        .query_opt(
            "SELECT m.creator FROM x_meeting_attachment a JOIN x_meeting m ON m.id = a.meeting_id WHERE a.id = $1",
            &[&attachment_id],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    let Some(row) = row else { return Err(AppError::NotFound) };
    let creator: String = row.get::<_, Option<String>>("creator").unwrap_or_default();
    if shared::middleware::is_admin(pool, &session.person_unique).await
        || session.person_unique == creator
    {
        Ok(creator)
    } else {
        Err(AppError::Forbidden)
    }
}

const U2_ATTACHMENT_COLS: &str =
    "id, meeting_id, person, file_name, extension, mime_type, length, summary, to_char(create_time, 'YYYY-MM-DD HH24:MI:SS') AS create_time";

fn u2_attachment_json(row: &deadpool_postgres::tokio_postgres::Row) -> Value {
    Value::Object(serde_json::Map::from_iter([
        ("id".to_string(), Value::String(row.get("id"))),
        ("\"meetingId\"".to_string(), Value::String(row.get("meeting_id"))),
        ("person".to_string(), Value::String(row.get::<_, Option<String>>("person").unwrap_or_default())),
        ("fileName".to_string(), Value::String(row.get::<_, Option<String>>("file_name").unwrap_or_default())),
        ("extension".to_string(), Value::String(row.get::<_, Option<String>>("extension").unwrap_or_default())),
        ("length".to_string(), Value::Number(serde_json::Number::from(row.get::<_, Option<i64>>("length").unwrap_or_default()))),
        ("summary".to_string(), Value::Bool(row.get::<_, Option<bool>>("summary").unwrap_or_default())),
        (
            "createTime".to_string(),
            Value::String(row.get::<_, Option<String>>("create_time").unwrap_or_default()),
        ),
    ]))
}

/// multipart 上传统一入口：blob put（带回读校验）+ 元数据行；content 双写 base64。
async fn u2_attachment_store_new(
    pool: &Pool,
    session: &shared::session::Session,
    meeting_id: &str,
    summary: bool,
    filename: &str,
    mime: Option<&str>,
    bytes: Vec<u8>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    if u2_meeting_creator(&client, meeting_id).await?.is_none() {
        return Err(AppError::BadRequest("meeting not found".to_string()));
    }

    let id = uuid::Uuid::new_v4().to_string();
    let key = u2_attachment_blob_key(&id, filename)?;
    u2_persist_blob_verified(&key, &bytes).await?;

    let ext = filename.rsplit('.').next().unwrap_or("bin").to_string();
    let length = bytes.len() as i64;
    let mime_owned = mime.unwrap_or("").to_string();
    let person = session.person_unique.clone();
    let content_b64 = base64::engine::general_purpose::STANDARD.encode(&bytes);

    client
        .execute(
            "INSERT INTO x_meeting_attachment (id, meeting_id, person, file_name, extension, mime_type, length, summary, content, storage_key, create_time) \
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, NOW())",
            &[&id, &meeting_id, &person, &filename, &ext, &mime_owned, &length, &summary, &content_b64, &key],
        )
        .await
        .map_err(|e| {
            tracing::warn!(error = %e, attachment = %id, "attachment metadata insert failed after blob write");
            AppError::Internal
        })?;

    drop(client);
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
            ("\"meetingId\"".to_string(), Value::String(meeting_id.to_string())),
            ("fileName".to_string(), Value::String(filename.to_string())),
            ("extension".to_string(), Value::String(ext)),
            ("length".to_string(), Value::Number(serde_json::Number::from(length))),
            ("uploaded".to_string(), Value::Bool(true)),
        ]),
    ))))
}

async fn u2_extract_multipart(
    mut form: axum::extract::Multipart,
) -> Result<(Option<String>, Option<String>, Vec<u8>), AppError> {
    while let Some(field) = form.next_field().await.map_err(|_| AppError::BadRequest("malformed multipart body".to_string()))? {
        let fname = field.file_name().map(str::to_string).filter(|s| !s.is_empty());
        let mime = field.content_type().map(str::to_string);
        let data = field.bytes().await.map_err(|_| AppError::BadRequest("unreadable upload field".to_string()))?;
        if fname.is_some() || !data.is_empty() {
            return Ok((fname, mime, data.to_vec()));
        }
    }
    Err(AppError::BadRequest("no file provided".to_string()))
}

// ── Attachment 族（11 端点）──────────────────────────────────────────────────

pub async fn u2_attachment_list_with_meeting(
    pool: Extension<Pool>,
    axum::extract::Path(meeting_id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            &format!("SELECT {U2_ATTACHMENT_COLS} FROM x_meeting_attachment WHERE meeting_id = $1 ORDER BY create_time"),
            &[&meeting_id],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    let data: Vec<Value> = rows.iter().map(u2_attachment_json).collect();
    let count = data.len() as i64;
    Ok(Json(ActionResult::java_success(Value::Array(data), count, 0)))
}

pub async fn u2_attachment_get(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            &format!("SELECT {U2_ATTACHMENT_COLS} FROM x_meeting_attachment WHERE id = $1"),
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    match row {
        Some(row) => Ok(Json(ActionResult::success(u2_attachment_json(&row)))),
        None => Err(AppError::NotFound),
    }
}

pub async fn u2_attachment_download(
    pool: Extension<Pool>,
    Extension(session): Extension<shared::session::Session>,
    axum::extract::Path((id, _stream)): axum::extract::Path<(String, String)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    u2_attachment_guard(&client, &pool, &session, &id).await?;
    let row = client
        .query_opt(
            "SELECT file_name, mime_type, content FROM x_meeting_attachment WHERE id = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    let Some(row) = row else { return Err(AppError::NotFound) };
    let content_b64: Option<String> = row.get("content");
    match content_b64 {
        Some(b64) if !b64.is_empty() => Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
            ("fileName".to_string(), Value::String(row.get::<_, Option<String>>("file_name").unwrap_or_default())),
            ("contentType".to_string(), Value::String(row.get::<_, Option<String>>("mime_type").unwrap_or_default())),
            ("contentBase64".to_string(), Value::String(b64)),
            ("stream".to_string(), Value::Bool(_stream.eq_ignore_ascii_case("true") || _stream == "1")),
        ]))))),
        _ => Err(AppError::NotFound),
    }
}

pub async fn u2_attachment_update(
    pool: Extension<Pool>,
    Extension(session): Extension<shared::session::Session>,
    axum::extract::Path(id): axum::extract::Path<String>,
    body: Option<axum::extract::Json<Value>>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    u2_attachment_update_inner(pool, session, id, body).await
}

pub async fn u2_attachment_update_callback(
    pool: Extension<Pool>,
    Extension(session): Extension<shared::session::Session>,
    axum::extract::Path((id, callback)): axum::extract::Path<(String, String)>,
    body: Option<axum::extract::Json<Value>>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    tracing::debug!(callback, attachment = %id, "attachment update via callback route");
    u2_attachment_update_inner(pool, session, id, body).await
}

async fn u2_attachment_update_inner(
    pool: Extension<Pool>,
    session: shared::session::Session,
    id: String,
    body: Option<axum::extract::Json<Value>>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    u2_attachment_guard(&client, &pool, &session, &id).await?;

    let payload = body.map(|Json(v)| v).unwrap_or(Value::Null);
    let file_name = payload.get("fileName").and_then(|v| v.as_str());
    let content_b64 = payload.get("contentBase64").and_then(|v| v.as_str());

    // 内容更新走同一 fail-loud 通道：先 blob 校验再落元数据。
    let new_length: Option<i64> = if let Some(b64) = content_b64 {
        let bytes = base64::engine::general_purpose::STANDARD
            .decode(b64)
            .map_err(|_| AppError::BadRequest("contentBase64 is not valid base64".to_string()))?;
        let row = client
            .query_opt("SELECT file_name FROM x_meeting_attachment WHERE id = $1", &[&id])
            .await
            .map_err(|_| AppError::Internal)?;
        let current_name: String = row
            .map(|r| r.get::<_, Option<String>>("file_name").unwrap_or_default())
            .unwrap_or_default();
        let key = u2_attachment_blob_key(&id, file_name.unwrap_or(&current_name))?;
        u2_persist_blob_verified(&key, &bytes).await?;
        Some(bytes.len() as i64)
    } else {
        None
    };

    let count = client
        .execute(
            "UPDATE x_meeting_attachment SET \
                file_name = COALESCE($2, file_name), \
                content = COALESCE($3, content), \
                length = COALESCE($4, length), \
                update_time = NOW() \
             WHERE id = $1",
            &[&id, &file_name, &content_b64, &new_length],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    if count == 0 {
        return Err(AppError::NotFound);
    }
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
            ("updated".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn u2_attachment_create_from_processplatform(
    pool: Extension<Pool>,
    Extension(session): Extension<shared::session::Session>,
    axum::extract::Json(payload): axum::extract::Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let meeting_id = payload
        .get("\"meetingId\"")
        .or_else(|| payload.get("meetingId"))
        .and_then(|v| v.as_str())
        .ok_or_else(|| AppError::BadRequest("meetingId is required".to_string()))?;
    let site = payload
        .get("site")
        .or_else(|| payload.get("\"site\""))
        .and_then(|v| v.as_str())
        .unwrap_or_default();
    let title = payload
        .get("title")
        .or_else(|| payload.get("\"title\""))
        .and_then(|v| v.as_str())
        .unwrap_or("processplatform attachment");

    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    if u2_meeting_creator(&client, meeting_id).await?.is_none() {
        return Err(AppError::BadRequest("meeting not found".to_string()));
    }
    let id = uuid::Uuid::new_v4().to_string();
    let person = session.person_unique.clone();
    client
        .execute(
            "INSERT INTO x_meeting_attachment (id, meeting_id, person, file_name, extension, length, summary, content, storage_key, create_time) \
             VALUES ($1, $2, $3, $4, '', 0, FALSE, NULL, NULL, NOW())",
            &[&id, &meeting_id, &person, &title],
        )
        .await
        .map_err(|e| {
            tracing::warn!(error = %e, "processplatform attachment reference insert failed");
            AppError::Internal
        })?;
    drop(client);
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
            ("\"meetingId\"".to_string(), Value::String(meeting_id.to_string())),
            ("site".to_string(), Value::String(site.to_string())),
            ("title".to_string(), Value::String(title.to_string())),
        ]),
    ))))
}

async fn u2_attachment_paged(
    pool: Extension<Pool>,
    id: String,
    count: i64,
    backward: bool,
) -> Result<Json<ActionResult<Value>>, AppError> {
    if count <= 0 || count > 200 {
        return Err(AppError::BadRequest("count must be within 1..=200".to_string()));
    }
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let exists = client
        .query_opt(
            "SELECT id FROM x_meeting_attachment WHERE id = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?
        .is_some();
    if !exists {
        return Err(AppError::NotFound);
    }
    let op: &str = if backward { "<" } else { ">" };
    let order: &str = if backward { "DESC" } else { "ASC" };
    let sql = format!(
        "SELECT {U2_ATTACHMENT_COLS} FROM x_meeting_attachment \
         WHERE create_time {op} (SELECT create_time FROM x_meeting_attachment WHERE id = $1) \
         ORDER BY create_time {order} LIMIT $2"
    );
    let rows = client.query(&sql, &[&id, &count]).await.map_err(|_| AppError::Internal)?;
    let mut data: Vec<Value> = rows.iter().map(u2_attachment_json).collect();
    // O2 next/prev 语义均按时间正序呈现窗口内容
    if backward {
        data.reverse();
    }
    let count = data.len() as i64;
    Ok(Json(ActionResult::java_success(Value::Array(data), count, 0)))
}

pub async fn u2_attachment_list_next(
    pool: Extension<Pool>,
    axum::extract::Path((id, count)): axum::extract::Path<(String, String)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let count: i64 = count.parse().map_err(|_| AppError::BadRequest("invalid count".to_string()))?;
    u2_attachment_paged(pool, id, count, false).await
}

pub async fn u2_attachment_list_prev(
    pool: Extension<Pool>,
    axum::extract::Path((id, count)): axum::extract::Path<(String, String)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let count: i64 = count.parse().map_err(|_| AppError::BadRequest("invalid count".to_string()))?;
    u2_attachment_paged(pool, id, count, true).await
}

pub async fn u2_attachment_upload(
    pool: Extension<Pool>,
    Extension(session): Extension<shared::session::Session>,
    axum::extract::Path((meeting_id, summary)): axum::extract::Path<(String, String)>,
    form: axum::extract::Multipart,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let summary = matches!(summary.as_str(), "true" | "1");
    let (fname, mime, data) = u2_extract_multipart(form).await?;
    let filename = fname.unwrap_or_else(|| "upload.bin".to_string());
    u2_attachment_store_new(&pool, &session, &meeting_id, summary, &filename, mime.as_deref(), data).await
}

pub async fn u2_attachment_upload_callback(
    pool: Extension<Pool>,
    Extension(session): Extension<shared::session::Session>,
    axum::extract::Path((meeting_id, summary, callback)): axum::extract::Path<(String, String, String)>,
    form: axum::extract::Multipart,
) -> Result<Json<ActionResult<Value>>, AppError> {
    tracing::debug!(callback, meeting = %meeting_id, "attachment upload via callback route");
    let summary = matches!(summary.as_str(), "true" | "1");
    let (fname, mime, data) = u2_extract_multipart(form).await?;
    let filename = fname.unwrap_or_else(|| "upload.bin".to_string());
    u2_attachment_store_new(&pool, &session, &meeting_id, summary, &filename, mime.as_deref(), data).await
}

pub async fn u2_attachment_delete(
    pool: Extension<Pool>,
    Extension(session): Extension<shared::session::Session>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    u2_attachment_guard(&client, &pool, &session, &id).await?;
    let key_row = client
        .query_opt("SELECT storage_key FROM x_meeting_attachment WHERE id = $1", &[&id])
        .await
        .map_err(|_| AppError::Internal)?;
    let count = client
        .execute("DELETE FROM x_meeting_attachment WHERE id = $1", &[&id])
        .await
        .map_err(|_| AppError::Internal)?;
    if count == 0 {
        return Err(AppError::NotFound);
    }
    if let Some(row) = key_row {
        if let Some(key) = row.get::<_, Option<String>>("storage_key") {
            if let Err(e) = shared::storage::storage_from_env().delete(&key).await {
                tracing::warn!(key, error = %e, "blob delete failed after attachment delete");
            }
        }
    }
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
            ("deleted".to_string(), Value::Bool(true)),
        ]),
    ))))
}

// ── Building 族补齐（create / delete / edit：Java buildingEditAvailable ≈ is_admin）──

async fn u2_building_name_taken(client: &deadpool_postgres::tokio_postgres::Client, raw_name: &str, exclude_id: Option<&str>) -> Result<bool, AppError> {
    let target = u2_normalize_name(raw_name);
    if target.is_empty() {
        return Ok(false);
    }
    let rows = client
        .query("SELECT name FROM x_meeting_building WHERE ($1::text IS NULL OR id != $1)", &[&exclude_id])
        .await
        .map_err(|_| AppError::Internal)?;
    Ok(rows
        .iter()
        .any(|r| u2_normalize_name(&r.get::<_, String>("name")) == target))
}

pub async fn u2_building_create(
    pool: Extension<Pool>,
    Extension(session): Extension<shared::session::Session>,
    axum::extract::Json(payload): axum::extract::Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    u2_require_admin(&pool.0, &session).await?;

    let name = payload
        .get("name")
        .and_then(|v| v.as_str())
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .ok_or_else(|| AppError::BadRequest("name is required".to_string()))?;
    if u2_building_name_taken(&client, name, None).await? {
        return Err(AppError::BadRequest(format!("building already exists: {}", name)));
    }

    let address = payload.get("address").and_then(|v| v.as_str());
    let description = payload.get("description").and_then(|v| v.as_str());
    let id = uuid::Uuid::new_v4().to_string();

    client
        .execute(
            "INSERT INTO x_meeting_building (id, name, address, description, create_time) VALUES ($1, $2, $3, $4, NOW())",
            &[&id, &name, &address, &description],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
            ("name".to_string(), Value::String(name.to_string())),
        ]),
    ))))
}

pub async fn u2_building_edit(
    pool: Extension<Pool>,
    Extension(session): Extension<shared::session::Session>,
    axum::extract::Path(id): axum::extract::Path<String>,
    axum::extract::Json(payload): axum::extract::Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    u2_require_admin(&pool.0, &session).await?;

    let exists = client
        .query_opt("SELECT id FROM x_meeting_building WHERE id = $1", &[&id])
        .await
        .map_err(|_| AppError::Internal)?
        .is_some();
    if !exists {
        return Err(AppError::NotFound);
    }

    let name = payload.get("name").and_then(|v| v.as_str()).map(str::trim).filter(|s| !s.is_empty());
    if let Some(name) = name {
        if u2_building_name_taken(&client, name, Some(&id)).await? {
            return Err(AppError::BadRequest(format!("building already exists: {}", name)));
        }
    }
    let address = payload.get("address").and_then(|v| v.as_str());
    let description = payload.get("description").and_then(|v| v.as_str());

    let count = client
        .execute(
            "UPDATE x_meeting_building SET \
                name = COALESCE($2, name), address = COALESCE($3, address), description = COALESCE($4, description) \
             WHERE id = $1",
            &[&id, &name, &address, &description],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
            ("updated".to_string(), Value::Bool(count > 0)),
        ]),
    ))))
}

pub async fn u2_building_delete(
    pool: Extension<Pool>,
    Extension(session): Extension<shared::session::Session>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    u2_require_admin(&pool.0, &session).await?;

    let in_use: i64 = client
        .query_one("SELECT COUNT(*) AS n FROM x_meeting_room WHERE building_id = $1", &[&id])
        .await
        .map_err(|_| AppError::Internal)?
        .get("n");
    if in_use > 0 {
        return Err(AppError::BadRequest(format!("building still referenced by {in_use} room(s)")));
    }
    let count = client
        .execute("DELETE FROM x_meeting_building WHERE id = $1", &[&id])
        .await
        .map_err(|_| AppError::Internal)?;
    if count == 0 {
        return Err(AppError::NotFound);
    }
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
            ("deleted".to_string(), Value::Bool(true)),
        ]),
    ))))
}

// ── Config 族补齐（POST save + GET manage 读视图）────────────────────────────

pub async fn u2_config_save(
    pool: Extension<Pool>,
    Extension(session): Extension<shared::session::Session>,
    axum::extract::Json(payload): axum::extract::Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    u2_require_admin(&pool.0, &session).await?;

    let config_key = payload
        .get("configKey")
        .or_else(|| payload.get("\"configKey\""))
        .or_else(|| payload.get("key"))
        .and_then(|v| v.as_str())
        .ok_or_else(|| AppError::BadRequest("configKey is required".to_string()))?
        .to_string();
    let config_value = payload
        .get("configValue")
        .or_else(|| payload.get("\"configValue\""))
        .or_else(|| payload.get("value"))
        .map(|v| match v {
            Value::String(s) => s.clone(),
            other => other.to_string(),
        })
        .ok_or_else(|| AppError::BadRequest("configValue is required".to_string()))?;

    let updated = client
        .execute(
            "UPDATE x_meeting_config SET config_value = $2, update_time = NOW() WHERE config_key = $1",
            &[&config_key, &config_value],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    if updated == 0 {
        client
            .execute(
                "INSERT INTO x_meeting_config (id, config_key, config_value, create_time, update_time) \
                 VALUES ($1, $2, $3, NOW(), NOW())",
                &[&uuid::Uuid::new_v4().to_string(), &config_key, &config_value],
            )
            .await
            .map_err(|_| AppError::Internal)?;
    }

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("configKey".to_string(), Value::String(config_key)),
            ("configValue".to_string(), Value::String(config_value)),
            ("saved".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn u2_config_manage_get(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT config_key, config_value, to_char(update_time, 'YYYY-MM-DD HH24:MI:SS') AS update_time FROM x_meeting_config ORDER BY config_key",
            &[],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("configKey".to_string(), Value::String(row.get("config_key"))),
                ("configValue".to_string(), Value::String(row.get::<_, Option<String>>("config_value").unwrap_or_default())),
                (
                    "updateTime".to_string(),
                    Value::String(row.get::<_, Option<String>>("update_time").unwrap_or_default()),
                ),
            ]))
        })
        .collect();
    let count = data.len() as i64;
    Ok(Json(ActionResult::java_success(Value::Array(data), count, 0)))
}

// ── Meeting 族补齐（IDOR 删除 / modify / PUT save / GET checkin）────────────

pub async fn u2_meeting_delete_owned(
    pool: Extension<Pool>,
    Extension(session): Extension<shared::session::Session>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let creator = u2_meeting_creator(&client, &id).await?;
    let Some(creator) = creator else { return Err(AppError::NotFound) };
    shared::middleware::require_owner(&pool.0, &session, &creator).await?;

    client
        .execute("DELETE FROM x_meeting_invite WHERE meeting_id = $1", &[&id])
        .await
        .map_err(|_| AppError::Internal)?;
    client
        .execute("DELETE FROM x_meeting WHERE id = $1", &[&id])
        .await
        .map_err(|_| AppError::Internal)?;
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
            ("deleted".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn u2_meeting_modify(
    pool: Extension<Pool>,
    Extension(session): Extension<shared::session::Session>,
    axum::extract::Path(id): axum::extract::Path<String>,
    axum::extract::Json(payload): axum::extract::Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let creator = u2_meeting_creator(&client, &id).await?;
    let Some(creator) = creator else { return Err(AppError::NotFound) };
    shared::middleware::require_owner(&pool.0, &session, &creator).await?;

    let title = payload.get("title").and_then(|v| v.as_str()).map(str::trim).filter(|s| !s.is_empty());
    let content = payload.get("content").and_then(|v| v.as_str());
    let room_id = payload.get("roomId").or_else(|| payload.get("\"roomId\"")).and_then(|v| v.as_str());

    let count = client
        .execute(
            "UPDATE x_meeting SET \
                title = COALESCE($2, title), content = COALESCE($3, content), room_id = COALESCE($4, room_id) \
             WHERE id = $1",
            &[&id, &title, &content, &room_id],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
            ("modified".to_string(), Value::Bool(count > 0)),
        ]),
    ))))
}

pub async fn u2_meeting_put_save(
    pool: Extension<Pool>,
    Extension(session): Extension<shared::session::Session>,
    axum::extract::Path(id): axum::extract::Path<String>,
    axum::extract::Json(payload): axum::extract::Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let creator = u2_meeting_creator(&client, &id).await?;
    let Some(creator) = creator else { return Err(AppError::NotFound) };
    shared::middleware::require_owner(&pool.0, &session, &creator).await?;

    let title = payload.get("title").and_then(|v| v.as_str()).unwrap_or_default();
    let content = payload.get("content").and_then(|v| v.as_str());
    let start_time = payload.get("\"startTime\"").and_then(|v| v.as_str()).unwrap_or_default();
    let end_time = payload.get("\"endTime\"").and_then(|v| v.as_str()).unwrap_or_default();

    let count = client
        .execute(
            "UPDATE x_meeting SET title = $2, content = $3, start_time = $4, end_time = $5 WHERE id = $1",
            &[&id, &title, &content, &start_time, &end_time],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    if count == 0 {
        return Err(AppError::NotFound);
    }
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
            ("saved".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// Java ActionCheckIn.execute(effectivePerson, id)：以当前登录人签到。
pub async fn u2_meeting_checkin_get(
    pool: Extension<Pool>,
    Extension(session): Extension<shared::session::Session>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    if u2_meeting_creator(&client, &id).await?.is_none() {
        return Err(AppError::NotFound);
    }
    let person = session.person_unique.clone();
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
            ("\"meetingId\"".to_string(), Value::String(id)),
            ("person".to_string(), Value::String(person)),
            ("checkedIn".to_string(), Value::Bool(true)),
        ]),
    ))))
}

// ── OpenMeeting 根端点（开放会议服务器配置读视图）────────────────────────────

pub async fn u2_openmeeting_get(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT config_key, config_value FROM x_meeting_config WHERE config_key LIKE 'openmeeting%' ORDER BY config_key",
            &[],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    let mut map = serde_json::Map::new();
    map.insert("enable".to_string(), Value::Bool(false));
    for row in &rows {
        let key: String = row.get("config_key");
        let val: String = row.get::<_, Option<String>>("config_value").unwrap_or_default();
        let short = key.trim_start_matches("openmeeting.").to_string();
        if short == "enable" {
            map.insert("enable".to_string(), Value::Bool(val.eq_ignore_ascii_case("true") || val == "1"));
        } else {
            map.insert(short, Value::String(val));
        }
    }
    Ok(Json(ActionResult::success(Value::Object(map))))
}

// ── Room 族补齐（create / delete / edit / setPhoto）─────────────────────────

async fn u2_room_name_taken(client: &deadpool_postgres::tokio_postgres::Client, raw_name: &str, exclude_id: Option<&str>) -> Result<bool, AppError> {
    let target = u2_normalize_name(raw_name);
    if target.is_empty() {
        return Ok(false);
    }
    let rows = client
        .query("SELECT name FROM x_meeting_room WHERE ($1::text IS NULL OR id != $1)", &[&exclude_id])
        .await
        .map_err(|_| AppError::Internal)?;
    Ok(rows
        .iter()
        .any(|r| u2_normalize_name(&r.get::<_, String>("name")) == target))
}

pub async fn u2_room_create(
    pool: Extension<Pool>,
    Extension(session): Extension<shared::session::Session>,
    axum::extract::Json(payload): axum::extract::Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    u2_require_admin(&pool.0, &session).await?;

    let name = payload
        .get("name")
        .and_then(|v| v.as_str())
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .ok_or_else(|| AppError::BadRequest("name is required".to_string()))?;
    if u2_room_name_taken(&client, name, None).await? {
        return Err(AppError::BadRequest(format!("room already exists: {}", name)));
    }

    let building_id = payload.get("buildingId").or_else(|| payload.get("\"buildingId\"")).and_then(|v| v.as_str());
    let floor = payload.get("floor").and_then(|v| v.as_str());
    let capacity = payload.get("capacity").and_then(|v| v.as_i64()).map(|v| v as i32).unwrap_or(0);
    let equipment = payload
        .get("equipment")
        .map(|v| v.to_string())
        .unwrap_or_else(|| "{}".to_string());
    let description = payload.get("description").and_then(|v| v.as_str());
    let id = uuid::Uuid::new_v4().to_string();

    client
        .execute(
            "INSERT INTO x_meeting_room (id, name, building_id, floor, capacity, equipment, description, create_time) \
             VALUES ($1, $2, $3, $4, $5, to_jsonb($6::text), $7, NOW())",
            &[&id, &name, &building_id, &floor, &capacity, &equipment, &description],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
            ("name".to_string(), Value::String(name.to_string())),
        ]),
    ))))
}

pub async fn u2_room_edit(
    pool: Extension<Pool>,
    Extension(session): Extension<shared::session::Session>,
    axum::extract::Path(id): axum::extract::Path<String>,
    axum::extract::Json(payload): axum::extract::Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    u2_require_admin(&pool.0, &session).await?;

    let exists = client
        .query_opt("SELECT id FROM x_meeting_room WHERE id = $1", &[&id])
        .await
        .map_err(|_| AppError::Internal)?
        .is_some();
    if !exists {
        return Err(AppError::NotFound);
    }

    let name = payload.get("name").and_then(|v| v.as_str()).map(str::trim).filter(|s| !s.is_empty());
    if let Some(name) = name {
        if u2_room_name_taken(&client, name, Some(&id)).await? {
            return Err(AppError::BadRequest(format!("room already exists: {}", name)));
        }
    }
    let capacity = payload.get("capacity").and_then(|v| v.as_i64()).map(|v| v as i32);
    let description = payload.get("description").and_then(|v| v.as_str());

    let count = client
        .execute(
            "UPDATE x_meeting_room SET \
                name = COALESCE($2, name), capacity = COALESCE($3, capacity), description = COALESCE($4, description) \
             WHERE id = $1",
            &[&id, &name, &capacity, &description],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
            ("updated".to_string(), Value::Bool(count > 0)),
        ]),
    ))))
}

pub async fn u2_room_delete(
    pool: Extension<Pool>,
    Extension(session): Extension<shared::session::Session>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    u2_require_admin(&pool.0, &session).await?;

    let count = client
        .execute("DELETE FROM x_meeting_room WHERE id = $1", &[&id])
        .await
        .map_err(|_| AppError::Internal)?;
    if count == 0 {
        return Err(AppError::NotFound);
    }
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
            ("deleted".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// Java ActionSetPhoto：multipart 上传照片字节。落地为 x_meeting_room_photo 行
/// （photo_url 存 base64 回放数据，photo_name 存文件名）。
pub async fn u2_room_set_photo(
    pool: Extension<Pool>,
    Extension(_session): Extension<shared::session::Session>,
    axum::extract::Path(id): axum::extract::Path<String>,
    form: axum::extract::Multipart,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let (_fname, _mime, data) = u2_extract_multipart(form).await?;
    if data.is_empty() {
        return Err(AppError::BadRequest("empty photo upload".to_string()));
    }
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let exists = client
        .query_opt("SELECT id FROM x_meeting_room WHERE id = $1", &[&id])
        .await
        .map_err(|_| AppError::Internal)?
        .is_some();
    if !exists {
        return Err(AppError::NotFound);
    }
    let photo_id = uuid::Uuid::new_v4().to_string();
    let photo_name = _fname.unwrap_or_else(|| "photo.bin".to_string());
    let b64 = base64::engine::general_purpose::STANDARD.encode(&data);
    client
        .execute(
            "INSERT INTO x_meeting_room_photo (id, room_id, photo_name, photo_url, create_time) \
             VALUES ($1, $2, $3, $4, NOW())",
            &[&photo_id, &id, &photo_name, &b64],
        )
        .await
        .map_err(|e| {
            tracing::warn!(error = %e, room = %id, "room photo insert failed");
            AppError::Internal
        })?;
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(photo_id)),
            ("roomId".to_string(), Value::String(id)),
            ("photoName".to_string(), Value::String(photo_name)),
            ("size".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ]),
    ))))
}
