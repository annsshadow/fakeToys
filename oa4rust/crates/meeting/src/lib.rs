use axum::{
    extract::Extension,
    Json,
};
use deadpool_postgres::Pool;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use shared::{error::AppError, response::{option_to_json, row_opt_json, ActionResult}};

pub mod routes;

#[cfg(test)]
mod tests;
#[cfg(test)]
mod tests_generated;


#[derive(Debug, Serialize, Deserialize)]
pub struct Room {
    pub id: String,
    pub name: String,
    pub building_id: Option<String>,
    pub floor: Option<String>,
    pub capacity: Option<i32>,
    pub equipment: Option<Value>,
    pub description: Option<String>,
    pub photo: Option<String>,
    pub order_number: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Building {
    pub id: String,
    pub name: String,
    pub address: Option<String>,
    pub description: Option<String>,
    pub order_number: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OpenMeetingRoom {
    pub id: String,
    pub name: String,
    pub url: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Meeting {
    pub id: String,
    pub title: String,
    pub content: Option<String>,
    pub room_id: String,
    pub start_time: String,
    pub end_time: String,
    pub creator: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Participant {
    pub id: String,
    pub meeting_id: String,
    pub invitee: String,
    pub status: String,
}

#[utoipa::path(
    get,
    path = "/jaxrs/meeting/room/list",
    responses(
        (status = 200, description = "Success", body = serde_json::Value),
        (status = 400, description = "Bad Request"),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal Server Error")
    ),
    tag = "meeting"
)]
pub async fn room_list(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT id, name, building_id, floor, capacity, equipment::text AS equipment, description, photo, order_number FROM x_meeting_room ORDER BY name LIMIT 50",
            &[],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            let mut map = serde_json::Map::new();
            map.insert("id".to_string(), Value::String(row.get("id")));
            map.insert("name".to_string(), Value::String(row.get("name")));
            if let Some(val) = row_opt_json::<String>(row, "building_id") {
                map.insert("buildingId".to_string(), val);
            }
            if let Some(val) = row_opt_json::<String>(row, "floor") {
                map.insert("floor".to_string(), val);
            }
            if let Some(val) = row_opt_json::<i32>(row, "capacity") {
                map.insert("capacity".to_string(), val);
            }
            if let Some(val) = option_to_json::<Value>(row.get::<_, Option<String>>("equipment").and_then(|s| serde_json::from_str(&s).ok())) {
                map.insert("equipment".to_string(), val);
            }
            if let Some(val) = row_opt_json::<String>(row, "description") {
                map.insert("description".to_string(), val);
            }
            if let Some(val) = row_opt_json::<String>(row, "photo") {
                map.insert("photo".to_string(), val);
            }
            if let Some(val) = row_opt_json::<i32>(row, "order_number") {
                map.insert("orderNumber".to_string(), val);
            }
            Value::Object(map)
        })
        .collect();

    let count = data.len() as i64;
    Ok(Json(ActionResult::java_success(Value::Array(data), count, 0)))
}

#[utoipa::path(
    get,
    path = "/jaxrs/meeting/building/list",
    responses(
        (status = 200, description = "Success", body = serde_json::Value),
        (status = 400, description = "Bad Request"),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal Server Error")
    ),
    tag = "meeting"
)]
pub async fn building_list(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT id, name, address, description, order_number, create_time FROM x_meeting_building ORDER BY name LIMIT 50",
            &[],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            let mut map = serde_json::Map::new();
            map.insert("id".to_string(), Value::String(row.get("id")));
            map.insert("name".to_string(), Value::String(row.get("name")));
            if let Some(val) = row_opt_json::<String>(row, "address") {
                map.insert("address".to_string(), val);
            }
            if let Some(val) = row_opt_json::<String>(row, "description") {
                map.insert("description".to_string(), val);
            }
            if let Some(val) = row_opt_json::<i32>(row, "order_number") {
                map.insert("orderNumber".to_string(), val);
            }
            map.insert("createTime".to_string(), Value::String(row.get("create_time")));
            Value::Object(map)
        })
        .collect();

    let count = data.len() as i64;
    Ok(Json(ActionResult::java_success(Value::Array(data), count, 0)))
}

#[utoipa::path(
    get,
    path = "/jaxrs/meeting/openmeeting/list/room",
    responses(
        (status = 200, description = "Success", body = serde_json::Value),
        (status = 400, description = "Bad Request"),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal Server Error")
    ),
    tag = "meeting"
)]
pub async fn openmeeting_list_room(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT id, name, url FROM x_meeting_room WHERE open_meeting = true ORDER BY name LIMIT 50",
            &[],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            let mut map = serde_json::Map::new();
            map.insert("id".to_string(), Value::String(row.get("id")));
            map.insert("name".to_string(), Value::String(row.get("name")));
            if let Some(val) = row_opt_json::<String>(row, "url") {
                map.insert("url".to_string(), val);
            }
            Value::Object(map)
        })
        .collect();

    let count = data.len() as i64;
    Ok(Json(ActionResult::java_success(Value::Array(data), count, 0)))
}

#[utoipa::path(
    post,
    path = "/jaxrs/meeting/create",
    request_body = serde_json::Value,
    responses(
        (status = 200, description = "Success", body = serde_json::Value),
        (status = 400, description = "Bad Request"),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal Server Error")
    ),
    tag = "meeting"
)]
pub async fn create_meeting(
    pool: Extension<Pool>,
    axum::extract::Json(payload): axum::extract::Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let title = payload.get("title").and_then(|v| v.as_str()).ok_or(AppError::BadRequest("title is required".to_string()))?;
    let content = payload.get("content").and_then(|v| v.as_str()).map(|s| s.to_string());
    let room_id = payload.get("roomId").and_then(|v| v.as_str()).ok_or(AppError::BadRequest("roomId is required".to_string()))?;
    let start_time = payload.get("\"startTime\"").and_then(|v| v.as_str()).ok_or(AppError::BadRequest("\"startTime\" is required".to_string()))?;
    let end_time = payload.get("\"endTime\"").and_then(|v| v.as_str()).ok_or(AppError::BadRequest("\"endTime\" is required".to_string()))?;
    let creator = payload.get("creator").and_then(|v| v.as_str()).unwrap_or("system");

    let id = uuid::Uuid::new_v4().to_string();
    client
        .execute(
            "INSERT INTO x_meeting (id, title, content, room_id, start_time, end_time, creator, create_time) VALUES ($1, $2, $3, $4, $5, $6, $7, NOW())",
            &[&id, &title, &content, &room_id, &start_time, &end_time, &creator],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("id".to_string(), Value::String(id)),
        ("title".to_string(), Value::String(title.to_string())),
        ("roomId".to_string(), Value::String(room_id.to_string())),
        ("\"startTime\"".to_string(), Value::String(start_time.to_string())),
        ("\"endTime\"".to_string(), Value::String(end_time.to_string())),
    ])))))
}

#[utoipa::path(
    get,
    path = "/jaxrs/meeting/{id}",
    params(
        ("id" = String, Path, description = "Meeting ID")
    ),
    responses(
        (status = 200, description = "Success", body = serde_json::Value),
        (status = 400, description = "Bad Request"),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal Server Error")
    ),
    tag = "meeting"
)]
pub async fn get_meeting(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT id, title, content, room_id, start_time, end_time, creator, create_time FROM x_meeting WHERE id = $1",
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
                ("roomId".to_string(), Value::String(row.get("room_id"))),
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

#[utoipa::path(
    get,
    path = "/jaxrs/meeting/list",
    responses(
        (status = 200, description = "Success", body = serde_json::Value),
        (status = 400, description = "Bad Request"),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal Server Error")
    ),
    tag = "meeting"
)]
pub async fn list_meetings(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT id, title, content, room_id, start_time, end_time, creator, create_time FROM x_meeting ORDER BY start_time DESC LIMIT 50",
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
                ("roomId".to_string(), Value::String(row.get("room_id"))),
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

#[utoipa::path(
    post,
    path = "/jaxrs/meeting/{\"meetingId\"}/participant/add",
    params(
        ("\"meetingId\"" = String, Path, description = "Meeting ID")
    ),
    request_body = serde_json::Value,
    responses(
        (status = 200, description = "Success", body = serde_json::Value),
        (status = 400, description = "Bad Request"),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal Server Error")
    ),
    tag = "meeting"
)]
pub async fn add_participant(
    pool: Extension<Pool>,
    axum::extract::Path(meeting_id): axum::extract::Path<String>,
    axum::extract::Json(payload): axum::extract::Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let invitee = payload.get("invitee").and_then(|v| v.as_str()).ok_or(AppError::BadRequest("invitee is required".to_string()))?;
    let id = uuid::Uuid::new_v4().to_string();

    client
        .execute(
            "INSERT INTO x_meeting_invite (id, meeting_id, invitee, status, create_time) VALUES ($1, $2, $3, 'wait', NOW())",
            &[&id, &meeting_id, &invitee],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("id".to_string(), Value::String(id)),
        ("\"meetingId\"".to_string(), Value::String(meeting_id)),
        ("invitee".to_string(), Value::String(invitee.to_string())),
        ("added".to_string(), Value::Bool(true)),
    ])))))
}

#[utoipa::path(
    get,
    path = "/jaxrs/meeting/{\"meetingId\"}/participant/list",
    params(
        ("\"meetingId\"" = String, Path, description = "Meeting ID")
    ),
    responses(
        (status = 200, description = "Success", body = serde_json::Value),
        (status = 400, description = "Bad Request"),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal Server Error")
    ),
    tag = "meeting"
)]
pub async fn list_participants(
    pool: Extension<Pool>,
    axum::extract::Path(meeting_id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT id, meeting_id, invitee, status, create_time FROM x_meeting_invite WHERE meeting_id = $1 ORDER BY create_time",
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
                ("invitee".to_string(), Value::String(row.get("invitee"))),
                ("status".to_string(), Value::String(row.get("status"))),
                ("createTime".to_string(), Value::String(row.get("create_time"))),
            ]))
        })
        .collect();

    let count = data.len() as i64;
    Ok(Json(ActionResult::java_success(Value::Array(data), count, 0)))
}

#[utoipa::path(
    get,
    path = "/jaxrs/meeting/schedule/days/{days}",
    params(
        ("days" = i64, Path, description = "Number of days to look ahead")
    ),
    responses(
        (status = 200, description = "Success", body = serde_json::Value),
        (status = 400, description = "Bad Request"),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal Server Error")
    ),
    tag = "meeting"
)]
pub async fn list_schedule(
    pool: Extension<Pool>,
    axum::extract::Path(days): axum::extract::Path<i64>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT id, title, content, room_id, start_time, end_time, creator, create_time FROM x_meeting WHERE start_time >= NOW() AND start_time <= NOW() + INTERVAL '1 day' * $1 ORDER BY start_time ASC",
            &[&days],
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
                ("roomId".to_string(), Value::String(row.get("room_id"))),
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

pub use routes::meeting_router;

pub fn router(pool: deadpool_postgres::Pool) -> axum::Router {
    crate::meeting_router(pool)
}
