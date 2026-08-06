use axum::{
    extract::Extension,
    Json,
};
use deadpool_postgres::Pool;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use shared::{error::AppError, response::ActionResult};

pub mod routes;

#[cfg(test)]
mod tests;

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

pub async fn room_list(
    _pool: Extension<Pool>,
) -> Result<Json<ActionResult<Vec<Room>>>, AppError> {
    let rooms = vec![
        Room {
            id: "room-001".to_string(),
            name: "第一会议室".to_string(),
            building_id: Some("building-001".to_string()),
            floor: Some("3F".to_string()),
            capacity: Some(20),
            equipment: Some(Value::Array(vec![
                Value::String("投影仪".to_string()),
                Value::String("视频会议".to_string()),
            ])),
            description: Some("大型会议室".to_string()),
            photo: None,
            order_number: Some(1),
        },
        Room {
            id: "room-002".to_string(),
            name: "第二会议室".to_string(),
            building_id: Some("building-001".to_string()),
            floor: Some("5F".to_string()),
            capacity: Some(10),
            equipment: Some(Value::Array(vec![
                Value::String("投影仪".to_string()),
            ])),
            description: Some("中型会议室".to_string()),
            photo: None,
            order_number: Some(2),
        },
    ];

    Ok(Json(ActionResult::success(rooms)))
}

pub async fn building_list(
    _pool: Extension<Pool>,
) -> Result<Json<ActionResult<Vec<Building>>>, AppError> {
    let buildings = vec![
        Building {
            id: "building-001".to_string(),
            name: "总部大楼".to_string(),
            address: Some("北京市朝阳区xxx路1号".to_string()),
            description: Some("公司总部".to_string()),
            order_number: Some(1),
        },
        Building {
            id: "building-002".to_string(),
            name: "研发中心".to_string(),
            address: Some("北京市海淀区xxx路2号".to_string()),
            description: Some("研发中心".to_string()),
            order_number: Some(2),
        },
    ];

    Ok(Json(ActionResult::success(buildings)))
}

pub async fn openmeeting_list_room(
    _pool: Extension<Pool>,
) -> Result<Json<ActionResult<Vec<OpenMeetingRoom>>>, AppError> {
    let rooms = vec![
        OpenMeetingRoom {
            id: "open-001".to_string(),
            name: "开放式讨论区A".to_string(),
            url: Some("https://meeting.example.com/room/open-001".to_string()),
        },
        OpenMeetingRoom {
            id: "open-002".to_string(),
            name: "开放式讨论区B".to_string(),
            url: Some("https://meeting.example.com/room/open-002".to_string()),
        },
    ];

    Ok(Json(ActionResult::success(rooms)))
}

pub async fn create_meeting(
    pool: Extension<Pool>,
    axum::extract::Json(payload): axum::extract::Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let title = payload.get("title").and_then(|v| v.as_str()).ok_or(AppError::BadRequest("title is required".to_string()))?;
    let content = payload.get("content").and_then(|v| v.as_str()).map(|s| s.to_string());
    let room_id = payload.get("roomId").and_then(|v| v.as_str()).ok_or(AppError::BadRequest("roomId is required".to_string()))?;
    let start_time = payload.get("startTime").and_then(|v| v.as_str()).ok_or(AppError::BadRequest("startTime is required".to_string()))?;
    let end_time = payload.get("endTime").and_then(|v| v.as_str()).ok_or(AppError::BadRequest("endTime is required".to_string()))?;
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
        ("startTime".to_string(), Value::String(start_time.to_string())),
        ("endTime".to_string(), Value::String(end_time.to_string())),
    ])))))
}

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
        ("meetingId".to_string(), Value::String(meeting_id)),
        ("invitee".to_string(), Value::String(invitee.to_string())),
        ("added".to_string(), Value::Bool(true)),
    ])))))
}

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
                ("meetingId".to_string(), Value::String(row.get("meeting_id"))),
                ("invitee".to_string(), Value::String(row.get("invitee"))),
                ("status".to_string(), Value::String(row.get("status"))),
                ("createTime".to_string(), Value::String(row.get("create_time"))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

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

pub use routes::meeting_router;

pub fn router(pool: deadpool_postgres::Pool) -> axum::Router {
    meeting_router(pool)
        .route("/meeting/health", axum::routing::get(|| async { "ok" }))
}