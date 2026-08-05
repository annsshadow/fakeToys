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

pub use routes::meeting_router;

pub fn router(_pool: deadpool_postgres::Pool) -> axum::Router {
    axum::Router::new()
        .route("/meeting/health", axum::routing::get(|| async { "TODO: meeting - real implementation needed" }))
}