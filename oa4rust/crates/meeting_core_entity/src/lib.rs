use axum::{
    extract::{Extension, Json, Path},
    routing::{get, post},
    Json as AxumJson, Router,
};
use chrono::Utc;
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, QueryOrder, QuerySelect, Set};
use serde_json::Value;
use shared::{error::AppError, response::ActionResult};

pub mod entities;
pub mod routes;

use entities::{meeting, meeting_room};

#[cfg(test)]
mod tests;

/// 获取会议室列表
/// 从数据库查询 x_meeting_room 表
pub async fn room_list(
    db: Extension<DatabaseConnection>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let models = meeting_room::Entity::find()
        .order_by_asc(meeting_room::Column::Name)
        .limit(20)
        .all(&db.0)
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = models
        .iter()
        .map(|m| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(m.id.clone())),
                ("name".to_string(), Value::String(m.name.clone())),
                (
                    "buildingId".to_string(),
                    m.building_id
                        .clone()
                        .map(Value::String)
                        .unwrap_or(Value::Null),
                ),
                (
                    "floor".to_string(),
                    m.floor
                        .clone()
                        .map(Value::String)
                        .unwrap_or(Value::Null),
                ),
                (
                    "capacity".to_string(),
                    m.capacity
                        .map(|v| Value::Number(serde_json::Number::from(v)))
                        .unwrap_or(Value::Null),
                ),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            (
                "count".to_string(),
                Value::Number(serde_json::Number::from(data.len() as i64)),
            ),
            ("data".to_string(), Value::Array(data)),
        ]),
    ))))
}

pub async fn create_room(
    db: Extension<DatabaseConnection>,
    AxumJson(payload): AxumJson<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let name = payload
        .get("name")
        .and_then(|v| v.as_str())
        .ok_or(AppError::BadRequest("name is required".to_string()))?;
    let name = name.to_string();
    let building_id = payload
        .get("buildingId")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string());
    let floor = payload
        .get("floor")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string());
    let capacity = payload
        .get("capacity")
        .and_then(|v| v.as_i64())
        .map(|v| v as i32);
    let equipment = payload
        .get("equipment")
        .and_then(|v| serde_json::to_string(v).ok());
    let description = payload
        .get("description")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string());
    let photo = payload
        .get("photo")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string());
    let order_number = payload
        .get("orderNumber")
        .and_then(|v| v.as_i64())
        .map(|v| v as i32);

    let id = uuid::Uuid::new_v4().to_string();
    let now = Utc::now().naive_utc();

    let active_model = meeting_room::ActiveModel {
        id: Set(id.clone()),
        name: Set(name.clone()),
        building_id: Set(building_id.clone()),
        floor: Set(floor.clone()),
        capacity: Set(capacity),
        equipment: Set(equipment.clone()),
        description: Set(description.clone()),
        photo: Set(photo.clone()),
        open_meeting: Set(None),
        order_number: Set(order_number),
        create_time: Set(Some(now)),
    };

    let m = active_model
        .insert(&db.0)
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("id".to_string(), Value::String(m.id.clone())),
        ("name".to_string(), Value::String(m.name.clone())),
        (
            "buildingId".to_string(),
            m.building_id
                .clone()
                .map(|s| Value::String(s))
                .unwrap_or(Value::Null),
        ),
        (
            "floor".to_string(),
            m.floor
                .clone()
                .map(|s| Value::String(s))
                .unwrap_or(Value::Null),
        ),
        (
            "capacity".to_string(),
            m.capacity
                .map(|v| Value::Number(serde_json::Number::from(v)))
                .unwrap_or(Value::Null),
        ),
        (
            "orderNumber".to_string(),
            m.order_number
                .map(|v| Value::Number(serde_json::Number::from(v)))
                .unwrap_or(Value::Null),
        ),
    ])))))
}

pub async fn get_room(
    db: Extension<DatabaseConnection>,
    Path(id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let model = meeting_room::Entity::find_by_id(&id)
        .one(&db.0)
        .await
        .map_err(|_| AppError::Internal)?;

    match model {
        Some(m) => {
            let result = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(m.id.clone())),
                ("name".to_string(), Value::String(m.name.clone())),
                (
                    "buildingId".to_string(),
                    m.building_id
                        .clone()
                        .map(Value::String)
                        .unwrap_or(Value::Null),
                ),
                (
                    "floor".to_string(),
                    m.floor
                        .clone()
                        .map(Value::String)
                        .unwrap_or(Value::Null),
                ),
                (
                    "capacity".to_string(),
                    m.capacity
                        .map(|v| Value::Number(serde_json::Number::from(v)))
                        .unwrap_or(Value::Null),
                ),
                (
                    "equipment".to_string(),
                    m.equipment
                        .clone()
                        .and_then(|s| serde_json::from_str(&s).ok())
                        .unwrap_or(Value::Null),
                ),
                (
                    "description".to_string(),
                    m.description
                        .clone()
                        .map(Value::String)
                        .unwrap_or(Value::Null),
                ),
                (
                    "photo".to_string(),
                    m.photo
                        .clone()
                        .map(Value::String)
                        .unwrap_or(Value::Null),
                ),
                (
                    "orderNumber".to_string(),
                    m.order_number
                        .map(|v| Value::Number(serde_json::Number::from(v)))
                        .unwrap_or(Value::Null),
                ),
                (
                    "createTime".to_string(),
                    Value::String(
                        m.create_time
                            .clone()
                            .map(|dt| dt.to_string())
                            .unwrap_or_default(),
                    ),
                ),
            ]));
            Ok(Json(ActionResult::success(result)))
        }
        None => Ok(Json(ActionResult::error("room not found"))),
    }
}

pub async fn update_room(
    db: Extension<DatabaseConnection>,
    Path(id): Path<String>,
    AxumJson(payload): AxumJson<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let existing = meeting_room::Entity::find_by_id(&id)
        .one(&db.0)
        .await
        .map_err(|_| AppError::Internal)?;

    let m = existing.ok_or_else(|| AppError::NotFound)?;

    let name = payload
        .get("name")
        .and_then(|v| v.as_str())
        .unwrap_or_default();
    let building_id = payload
        .get("buildingId")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string());
    let floor = payload
        .get("floor")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string());
    let capacity = payload
        .get("capacity")
        .and_then(|v| v.as_i64())
        .map(|v| v as i32);
    let equipment = payload
        .get("equipment")
        .and_then(|v| serde_json::to_string(v).ok());
    let description = payload
        .get("description")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string());
    let photo = payload
        .get("photo")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string());
    let order_number = payload
        .get("orderNumber")
        .and_then(|v| v.as_i64())
        .map(|v| v as i32);

    let active_model = meeting_room::ActiveModel {
        id: Set(id.clone()),
        name: Set(name.to_string()),
        building_id: Set(building_id),
        floor: Set(floor),
        capacity: Set(capacity),
        equipment: Set(equipment),
        description: Set(description),
        photo: Set(photo),
        open_meeting: Set(m.open_meeting),
        order_number: Set(order_number),
        create_time: Set(m.create_time),
    };

    let updated = active_model
        .update(&db.0)
        .await
        .map_err(|_| AppError::Internal)?;

    if updated.name != name {
        return Ok(Json(ActionResult::error("room not found")));
    }

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("id".to_string(), Value::String(id)),
        ("saved".to_string(), Value::Bool(true)),
        ("name".to_string(), Value::String(name.to_string())),
    ])))))
}

pub async fn delete_room(
    db: Extension<DatabaseConnection>,
    Path(id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let existing = meeting_room::Entity::find_by_id(&id)
        .one(&db.0)
        .await
        .map_err(|_| AppError::Internal)?;

    if existing.is_none() {
        return Ok(Json(ActionResult::error("room not found")));
    }

    let active_model: meeting_room::ActiveModel = existing.unwrap().into();
    active_model
        .delete(&db.0)
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("id".to_string(), Value::String(id)),
        ("deleted".to_string(), Value::Bool(true)),
    ])))))
}

/// 获取会议列表
/// 从数据库查询 x_meeting 表
pub async fn meeting_list(
    db: Extension<DatabaseConnection>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let models = meeting::Entity::find()
        .order_by_desc(meeting::Column::StartTime)
        .limit(20)
        .all(&db.0)
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = models
        .iter()
        .map(|m| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(m.id.clone())),
                ("title".to_string(), Value::String(m.title.clone())),
                (
                    "roomId".to_string(),
                    Value::String(m.room_id.clone().unwrap_or_default()),
                ),
                ("startTime".to_string(), Value::String(m.start_time.to_string())),
                ("endTime".to_string(), Value::String(m.end_time.to_string())),
                (
                    "organizerId".to_string(),
                    Value::String(m.creator.clone().unwrap_or_default()),
                ),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            (
                "count".to_string(),
                Value::Number(serde_json::Number::from(data.len() as i64)),
            ),
            ("data".to_string(), Value::Array(data)),
        ]),
    ))))
}

/// 按会议室查询会议
/// 查询指定会议室的会议列表
pub async fn meeting_list_by_room(
    db: Extension<DatabaseConnection>,
    Path(room_id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let models = meeting::Entity::find()
        .filter(meeting::Column::RoomId.eq(&room_id))
        .order_by_desc(meeting::Column::StartTime)
        .limit(20)
        .all(&db.0)
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = models
        .iter()
        .map(|m| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(m.id.clone())),
                ("title".to_string(), Value::String(m.title.clone())),
                (
                    "roomId".to_string(),
                    Value::String(m.room_id.clone().unwrap_or_default()),
                ),
                ("startTime".to_string(), Value::String(m.start_time.to_string())),
                ("endTime".to_string(), Value::String(m.end_time.to_string())),
                (
                    "organizerId".to_string(),
                    Value::String(m.creator.clone().unwrap_or_default()),
                ),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            (
                "count".to_string(),
                Value::Number(serde_json::Number::from(data.len() as i64)),
            ),
            ("data".to_string(), Value::Array(data)),
        ]),
    ))))
}

/// 创建会议
pub async fn create_meeting(
    db: Extension<DatabaseConnection>,
    AxumJson(payload): AxumJson<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let title = payload
        .get("title")
        .and_then(|v| v.as_str())
        .ok_or(AppError::BadRequest("title is required".to_string()))?;
    let title = title.to_string();
    let content = payload
        .get("content")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string());
    let room_id = payload
        .get("roomId")
        .and_then(|v| v.as_str())
        .ok_or(AppError::BadRequest("roomId is required".to_string()))?;
    let room_id = room_id.to_string();
    let start_time_str = payload
        .get("startTime")
        .and_then(|v| v.as_str())
        .ok_or(AppError::BadRequest("startTime is required".to_string()))?;
    let start_time: chrono::NaiveDateTime = start_time_str
        .parse()
        .map_err(|_| AppError::BadRequest("invalid startTime".to_string()))?;
    let end_time_str = payload
        .get("endTime")
        .and_then(|v| v.as_str())
        .ok_or(AppError::BadRequest("endTime is required".to_string()))?;
    let end_time: chrono::NaiveDateTime = end_time_str
        .parse()
        .map_err(|_| AppError::BadRequest("invalid endTime".to_string()))?;
    let organizer_id = payload
        .get("organizerId")
        .and_then(|v| v.as_str())
        .unwrap_or("system");
    let organizer_id = organizer_id.to_string();

    let id = uuid::Uuid::new_v4().to_string();

    let active_model = meeting::ActiveModel {
        id: Set(id.clone()),
        title: Set(title.clone()),
        content: Set(content),
        room_id: Set(Some(room_id.clone())),
        start_time: Set(start_time),
        end_time: Set(end_time),
        creator: Set(Some(organizer_id.clone())),
        create_time: Set(Some(Utc::now().naive_utc())),
    };

    let m = active_model
        .insert(&db.0)
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("id".to_string(), Value::String(m.id.clone())),
        ("title".to_string(), Value::String(m.title.clone())),
        (
            "roomId".to_string(),
            Value::String(m.room_id.clone().unwrap_or(room_id)),
        ),
        ("startTime".to_string(), Value::String(m.start_time.to_string())),
        ("endTime".to_string(), Value::String(m.end_time.to_string())),
    ])))))
}

/// 获取单个会议
pub async fn get_meeting(
    db: Extension<DatabaseConnection>,
    Path(id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let model = meeting::Entity::find_by_id(&id)
        .one(&db.0)
        .await
        .map_err(|_| AppError::Internal)?;

    match model {
        Some(m) => {
            let result = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(m.id.clone())),
                ("title".to_string(), Value::String(m.title.clone())),
                (
                    "content".to_string(),
                    Value::String(m.content.clone().unwrap_or_default()),
                ),
                (
                    "roomId".to_string(),
                    Value::String(m.room_id.clone().unwrap_or_default()),
                ),
                ("startTime".to_string(), Value::String(m.start_time.to_string())),
                ("endTime".to_string(), Value::String(m.end_time.to_string())),
                (
                    "organizerId".to_string(),
                    Value::String(m.creator.clone().unwrap_or_default()),
                ),
                (
                    "createTime".to_string(),
                    Value::String(
                        m.create_time
                            .clone()
                            .map(|dt| dt.to_string())
                            .unwrap_or_default(),
                    ),
                ),
            ]));
            Ok(Json(ActionResult::success(result)))
        }
        None => Ok(Json(ActionResult::error("meeting not found"))),
    }
}

/// 更新会议
pub async fn update_meeting(
    db: Extension<DatabaseConnection>,
    Path(id): Path<String>,
    AxumJson(payload): AxumJson<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let existing = meeting::Entity::find_by_id(&id)
        .one(&db.0)
        .await
        .map_err(|_| AppError::Internal)?;

    let m = existing.ok_or_else(|| AppError::NotFound)?;

    let title = payload
        .get("title")
        .and_then(|v| v.as_str())
        .unwrap_or_default();
    let content = payload
        .get("content")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string());
    let start_time_str = payload
        .get("startTime")
        .and_then(|v| v.as_str())
        .unwrap_or_default();
    let end_time_str = payload
        .get("endTime")
        .and_then(|v| v.as_str())
        .unwrap_or_default();

    let start_time: chrono::NaiveDateTime = start_time_str
        .parse()
        .map_err(|_| AppError::BadRequest("invalid startTime".to_string()))?;
    let end_time: chrono::NaiveDateTime = end_time_str
        .parse()
        .map_err(|_| AppError::BadRequest("invalid endTime".to_string()))?;

    let active_model = meeting::ActiveModel {
        id: Set(id.clone()),
        title: Set(title.to_string()),
        content: Set(content),
        room_id: Set(m.room_id.clone()),
        start_time: Set(start_time),
        end_time: Set(end_time),
        creator: Set(m.creator.clone()),
        create_time: Set(m.create_time.clone()),
    };

    let updated = active_model
        .update(&db.0)
        .await
        .map_err(|_| AppError::Internal)?;

    if updated.title != title {
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
    db: Extension<DatabaseConnection>,
    Path(id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let existing = meeting::Entity::find_by_id(&id)
        .one(&db.0)
        .await
        .map_err(|_| AppError::Internal)?;

    if existing.is_none() {
        return Ok(Json(ActionResult::error("meeting not found")));
    }

    let active_model: meeting::ActiveModel = existing.unwrap().into();
    active_model
        .delete(&db.0)
        .await
        .map_err(|_| AppError::Internal)?;

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
pub fn meeting_core_entity_router(_pool: deadpool_postgres::Pool) -> Router {
    let db = tokio::runtime::Handle::current()
        .block_on(shared::db::create_sea_orm_pool())
        .expect("failed to create sea-orm connection");

    Router::new()
        .route("/jaxrs/meeting/core/entity/room/list", get(room_list))
        .route("/jaxrs/meeting/core/entity/room/create", post(create_room))
        .route("/jaxrs/meeting/core/entity/room/{id}", get(get_room))
        .route("/jaxrs/meeting/core/entity/room/save/{id}", post(update_room))
        .route(
            "/jaxrs/meeting/core/entity/room/delete/{id}",
            post(delete_room),
        )
        .route(
            "/jaxrs/meeting/core/entity/meeting/list",
            get(meeting_list),
        )
        .route(
            "/jaxrs/meeting/core/entity/meeting/list/by/{roomId}",
            get(meeting_list_by_room),
        )
        .route(
            "/jaxrs/meeting/core/entity/meeting/create",
            post(create_meeting),
        )
        .route(
            "/jaxrs/meeting/core/entity/meeting/{id}",
            get(get_meeting),
        )
        .route(
            "/jaxrs/meeting/core/entity/meeting/save/{id}",
            post(update_meeting),
        )
        .route(
            "/jaxrs/meeting/core/entity/meeting/delete/{id}",
            post(delete_meeting),
        )
        .layer(Extension(db))
}

pub fn router(pool: deadpool_postgres::Pool) -> axum::Router {
    crate::meeting_core_entity_router(pool)
}

