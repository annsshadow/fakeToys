use axum::{
    extract::Extension,
    routing::get,
    Json, Router,
};
use deadpool_postgres::Pool;
use serde_json::Value;
use shared::{error::AppError, response::ActionResult};

pub mod routes;

// 会议室实体
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct MeetingRoom {
    pub id: String,
    pub name: String,
    pub building_id: Option<String>,
    pub floor: Option<String>,
    pub capacity: Option<i32>,
}

// 会议实体
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct Meeting {
    pub id: String,
    pub title: String,
    pub room_id: String,
    pub start_time: String,
    pub end_time: String,
    pub organizer_id: String,
}

/// 获取会议室列表
/// 从数据库查询 x_meeting_room 表
pub async fn room_list(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT id, name, building_id, floor, capacity FROM x_meeting_room ORDER BY name LIMIT 20",
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
                (
                    "buildingId".to_string(),
                    row.get::<_, Option<String>>("building_id")
                        .map(Value::String)
                        .unwrap_or(Value::Null),
                ),
                (
                    "floor".to_string(),
                    row.get::<_, Option<String>>("floor")
                        .map(Value::String)
                        .unwrap_or(Value::Null),
                ),
                (
                    "capacity".to_string(),
                    row.get::<_, Option<i32>>("capacity")
                        .map(|v| Value::Number(serde_json::Number::from(v)))
                        .unwrap_or(Value::Null),
                ),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
            ("data".to_string(), Value::Array(data)),
        ]),
    ))))
}

/// 获取会议列表
/// 从数据库查询 x_meeting 表
pub async fn meeting_list(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT id, title, room_id, start_time, end_time, organizer_id FROM x_meeting ORDER BY start_time DESC LIMIT 20",
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
                ("roomId".to_string(), Value::String(row.get("room_id"))),
                ("startTime".to_string(), Value::String(row.get("start_time"))),
                ("endTime".to_string(), Value::String(row.get("end_time"))),
                ("organizerId".to_string(), Value::String(row.get("organizer_id"))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
            ("data".to_string(), Value::Array(data)),
        ]),
    ))))
}

/// 按会议室查询会议
/// 查询指定会议室的会议列表
pub async fn meeting_list_by_room(
    pool: Extension<Pool>,
    axum::extract::Path(room_id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT id, title, room_id, start_time, end_time, organizer_id FROM x_meeting WHERE room_id = $1 ORDER BY start_time DESC LIMIT 20",
            &[&room_id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("title".to_string(), Value::String(row.get("title"))),
                ("roomId".to_string(), Value::String(row.get("room_id"))),
                ("startTime".to_string(), Value::String(row.get("start_time"))),
                ("endTime".to_string(), Value::String(row.get("end_time"))),
                ("organizerId".to_string(), Value::String(row.get("organizer_id"))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
            ("data".to_string(), Value::Array(data)),
        ]),
    ))))
}

/// 创建会议核心实体路由
/// 注册以下路由：
/// - /jaxrs/meeting/core/entity/room/list - 会议室列表
/// - /jaxrs/meeting/core/entity/meeting/list - 会议列表
/// - /jaxrs/meeting/core/entity/meeting/list/by/{roomId} - 按会议室查询
pub fn meeting_core_entity_router(pool: Pool) -> Router {
    Router::new()
        .route("/jaxrs/meeting/core/entity/room/list", get(room_list))
        .route("/jaxrs/meeting/core/entity/meeting/list", get(meeting_list))
        .route("/jaxrs/meeting/core/entity/meeting/list/by/{roomId}", get(meeting_list_by_room))
        .layer(Extension(pool))
}

#[cfg(test)]
mod tests;
