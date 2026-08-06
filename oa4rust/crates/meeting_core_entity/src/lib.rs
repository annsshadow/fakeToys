use axum::{
    extract::Extension,
    routing::{get, post},
    Json, Router,
};
use deadpool_postgres::Pool;
use serde_json::Value;
use shared::{error::AppError, response::ActionResult};

pub mod routes;

#[cfg(test)]
mod tests;

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct MeetingRoom {
    pub id: String,
    pub name: String,
    pub building_id: Option<String>,
    pub floor: Option<String>,
    pub capacity: Option<i32>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct Meeting {
    pub id: String,
    pub title: String,
    pub content: Option<String>,
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
            "SELECT id, title, content, room_id, start_time, end_time, organizer_id FROM x_meeting ORDER BY start_time DESC LIMIT 20",
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

/// 创建会议
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
    let organizer_id = payload.get("organizerId").and_then(|v| v.as_str()).unwrap_or("system");

    let id = uuid::Uuid::new_v4().to_string();
    client
        .execute(
            "INSERT INTO x_meeting (id, title, content, room_id, start_time, end_time, organizer_id, create_time) VALUES ($1, $2, $3, $4, $5, $6, $7, NOW())",
            &[&id, &title, &content, &room_id, &start_time, &end_time, &organizer_id],
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

/// 获取单个会议
pub async fn get_meeting(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT id, title, content, room_id, start_time, end_time, organizer_id, create_time FROM x_meeting WHERE id = $1",
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
                ("organizerId".to_string(), Value::String(row.get("organizer_id"))),
                ("createTime".to_string(), Value::String(row.get("create_time"))),
            ]));
            Ok(Json(ActionResult::success(result)))
        }
        None => Ok(Json(ActionResult::error("meeting not found"))),
    }
}

/// 更新会议
pub async fn update_meeting(
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

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("id".to_string(), Value::String(id)),
        ("saved".to_string(), Value::Bool(true)),
        ("title".to_string(), Value::String(title.to_string())),
    ])))))
}

/// 删除会议
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

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("id".to_string(), Value::String(id)),
        ("deleted".to_string(), Value::Bool(true)),
    ])))))
}

/// 创建会议核心实体路由
/// 注册以下路由：
/// - /jaxrs/meeting/core/entity/room/list - 会议室列表
/// - /jaxrs/meeting/core/entity/meeting/list - 会议列表
/// - /jaxrs/meeting/core/entity/meeting/list/by/{roomId} - 按会议室查询
/// - /jaxrs/meeting/core/entity/meeting/create - 创建会议
/// - /jaxrs/meeting/core/entity/meeting/{id} - 获取单个会议
/// - /jaxrs/meeting/core/entity/meeting/save/{id} - 更新会议
/// - /jaxrs/meeting/core/entity/meeting/delete/{id} - 删除会议
pub fn meeting_core_entity_router(pool: Pool) -> Router {
    Router::new()
        .route("/jaxrs/meeting/core/entity/room/list", get(room_list))
        .route("/jaxrs/meeting/core/entity/meeting/list", get(meeting_list))
        .route("/jaxrs/meeting/core/entity/meeting/list/by/{roomId}", get(meeting_list_by_room))
        .route("/jaxrs/meeting/core/entity/meeting/create", post(create_meeting))
        .route("/jaxrs/meeting/core/entity/meeting/{id}", get(get_meeting))
        .route("/jaxrs/meeting/core/entity/meeting/save/{id}", post(update_meeting))
        .route("/jaxrs/meeting/core/entity/meeting/delete/{id}", post(delete_meeting))
        .layer(Extension(pool))
}

pub fn router(pool: deadpool_postgres::Pool) -> axum::Router {
    meeting_core_entity_router(pool)
        .route("/meeting_core_entity/health", axum::routing::get(|| async { "ok" }))
}