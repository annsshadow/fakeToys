use axum::{
    extract::{Extension, Json, Path},
    routing::{get, post},
    Json as AxumJson, Router,
};
use chrono::Utc;
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, QueryOrder, QuerySelect, Set};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use shared::{error::AppError, response::{option_to_json, ActionResult}};

pub mod entities;
pub mod routes;

use entities::{cal_calendar, cal_event};

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct CalendarItem {
    pub id: String,
    pub name: String,
    pub calendar_type: String,
    pub target: String,
    pub color: String,
    pub description: Option<String>,
    pub createor: String,
    pub is_public: bool,
    pub status: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateCalendarRequest {
    pub name: Option<String>,
    #[serde(rename = "type")]
    pub calendar_type: Option<String>,
    pub target: Option<String>,
    pub color: Option<String>,
    pub description: Option<String>,
    pub source: Option<String>,
    pub createor: Option<String>,
    pub is_public: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateCalendarRequest {
    pub id: Option<String>,
    pub name: Option<String>,
    #[serde(rename = "type")]
    pub calendar_type: Option<String>,
    pub target: Option<String>,
    pub color: Option<String>,
    pub description: Option<String>,
    pub is_public: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct DeleteCalendarRequest {
    pub id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CalendarEvent {
    pub id: String,
    pub calendar_id: String,
    pub title: String,
    pub content: Option<String>,
    pub location: Option<String>,
    pub start_time: String,
    pub end_time: String,
    pub all_day: bool,
    pub visibility: String,
    pub status: String,
    pub createor: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateEventRequest {
    pub calendar_id: Option<String>,
    pub title: Option<String>,
    pub content: Option<String>,
    pub location: Option<String>,
    pub start_time: Option<String>,
    pub end_time: Option<String>,
    pub all_day: Option<bool>,
    pub visibility: Option<String>,
    pub createor: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateEventRequest {
    pub id: Option<String>,
    pub title: Option<String>,
    pub content: Option<String>,
    pub location: Option<String>,
    pub start_time: Option<String>,
    pub end_time: Option<String>,
    pub all_day: Option<bool>,
    pub visibility: Option<String>,
    pub status: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct DeleteEventRequest {
    pub id: Option<String>,
}

pub async fn calendar_list_public(
    db: Extension<DatabaseConnection>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let models = cal_calendar::Entity::find()
        .filter(
            cal_calendar::Column::IsPublic
                .eq(true)
                .and(cal_calendar::Column::Status.eq("OPEN")),
        )
        .order_by_desc(cal_calendar::Column::CreateTime)
        .limit(50)
        .all(&db.0)
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = models
        .iter()
        .map(|m| {
            let mut map = serde_json::Map::new();
            map.insert("id".to_string(), Value::String(m.id.clone()));
            map.insert("name".to_string(), Value::String(m.name.clone()));
            map.insert("type".to_string(), Value::String(m.type_.clone()));
            map.insert("target".to_string(), Value::String(m.target.clone()));
            map.insert("color".to_string(), Value::String(m.color.clone()));
            if let Some(val) = option_to_json(m.description.clone().map(|s| Value::String(s))) {
                map.insert("description".to_string(), val);
            }
            map.insert("createor".to_string(), Value::String(m.createor.clone()));
            map.insert("isPublic".to_string(), Value::Bool(m.is_public));
            map.insert("status".to_string(), Value::String(m.status.clone()));
            Value::Object(map)
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

pub async fn calendar_list_my(
    db: Extension<DatabaseConnection>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let models = cal_calendar::Entity::find()
        .filter(cal_calendar::Column::Status.eq("OPEN"))
        .order_by_desc(cal_calendar::Column::CreateTime)
        .limit(50)
        .all(&db.0)
        .await
        .map_err(|_| AppError::Internal)?;

    let mut my_calendars = Vec::new();
    let mut unit_calendars = Vec::new();

    for m in models.iter() {
        let mut map = serde_json::Map::new();
        map.insert("id".to_string(), Value::String(m.id.clone()));
        map.insert("name".to_string(), Value::String(m.name.clone()));
        map.insert("type".to_string(), Value::String(m.type_.clone()));
        map.insert("target".to_string(), Value::String(m.target.clone()));
        map.insert("color".to_string(), Value::String(m.color.clone()));
        if let Some(val) = option_to_json(m.description.clone().map(|s| Value::String(s))) {
            map.insert("description".to_string(), val);
        }
        map.insert("createor".to_string(), Value::String(m.createor.clone()));
        map.insert("isPublic".to_string(), Value::Bool(m.is_public));
        map.insert("status".to_string(), Value::String(m.status.clone()));
        let item = Value::Object(map);

        if m.type_.eq_ignore_ascii_case("UNIT") {
            unit_calendars.push(item);
        } else {
            my_calendars.push(item);
        }
    }

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("myCalendars".to_string(), Value::Array(my_calendars)),
            ("unitCalendars".to_string(), Value::Array(unit_calendars)),
            ("followCalendars".to_string(), Value::Array(Vec::<Value>::new())),
        ]),
    ))))
}

pub async fn calendar_get(
    db: Extension<DatabaseConnection>,
    Path(id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let model = cal_calendar::Entity::find_by_id(&id)
        .filter(cal_calendar::Column::Status.eq("OPEN"))
        .one(&db.0)
        .await
        .map_err(|_| AppError::Internal)?;

    match model {
        Some(m) => {
            let mut map = serde_json::Map::new();
            map.insert("id".to_string(), Value::String(m.id.clone()));
            map.insert("name".to_string(), Value::String(m.name.clone()));
            map.insert("type".to_string(), Value::String(m.type_.clone()));
            map.insert("target".to_string(), Value::String(m.target.clone()));
            map.insert("color".to_string(), Value::String(m.color.clone()));
            if let Some(val) = option_to_json(m.description.clone().map(|s| Value::String(s))) {
                map.insert("description".to_string(), val);
            }
            map.insert("createor".to_string(), Value::String(m.createor.clone()));
            map.insert("isPublic".to_string(), Value::Bool(m.is_public));
            map.insert("status".to_string(), Value::String(m.status.clone()));
            let data = Value::Object(map);
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("calendar not found"))),
    }
}

pub async fn calendar_create(
    db: Extension<DatabaseConnection>,
    AxumJson(req): AxumJson<CreateCalendarRequest>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let name = req.name.ok_or_else(|| AppError::BadRequest("name is required".to_string()))?;
    let calendar_type = req
        .calendar_type
        .ok_or_else(|| AppError::BadRequest("type is required".to_string()))?;
    let target = req.target.unwrap_or_else(|| "person".to_string());
    let color = req.color.unwrap_or_else(|| "#1462be".to_string());
    let description = req.description;
    let source = req.source;
    let createor = req.createor.unwrap_or_else(|| "anonymous".to_string());
    let is_public = req.is_public.unwrap_or(false);

    let id = uuid::Uuid::new_v4().to_string();
    let now = Utc::now().naive_utc();

    let active_model = cal_calendar::ActiveModel {
        id: Set(id.clone()),
        name: Set(name.clone()),
        type_: Set(calendar_type.clone()),
        target: Set(target.clone()),
        color: Set(color.clone()),
        description: Set(description.clone()),
        source: Set(source.clone()),
        createor: Set(createor.clone()),
        is_public: Set(is_public),
        status: Set("OPEN".to_string()),
        create_time: Set(Some(now)),
    };

    let m = active_model.insert(&db.0).await.map_err(|_| AppError::Internal)?;

    let mut map = serde_json::Map::new();
    map.insert("id".to_string(), Value::String(m.id.clone()));
    map.insert("name".to_string(), Value::String(m.name.clone()));
    map.insert("type".to_string(), Value::String(m.type_.clone()));
    map.insert("target".to_string(), Value::String(m.target.clone()));
    map.insert("color".to_string(), Value::String(m.color.clone()));
    if let Some(val) = option_to_json(m.description.clone().map(|s| Value::String(s))) {
        map.insert("description".to_string(), val);
    }
    if let Some(val) = option_to_json(m.source.clone().map(|s| Value::String(s))) {
        map.insert("source".to_string(), val);
    }
    map.insert("createor".to_string(), Value::String(m.createor.clone()));
    map.insert("isPublic".to_string(), Value::Bool(m.is_public));
    map.insert("status".to_string(), Value::String(m.status.clone()));
    let data = Value::Object(map);

    Ok(Json(ActionResult::success(data)))
}

pub async fn calendar_update(
    db: Extension<DatabaseConnection>,
    AxumJson(req): AxumJson<UpdateCalendarRequest>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let id = req
        .id
        .ok_or_else(|| AppError::BadRequest("id is required".to_string()))?;

    let existing = cal_calendar::Entity::find_by_id(&id)
        .filter(cal_calendar::Column::Status.eq("OPEN"))
        .one(&db.0)
        .await
        .map_err(|_| AppError::Internal)?;

    let m = existing.ok_or_else(|| AppError::NotFound)?;

    let name = req.name.unwrap_or(m.name);
    let calendar_type = req.calendar_type.unwrap_or(m.type_);
    let target = req.target.unwrap_or(m.target);
    let color = req.color.unwrap_or(m.color);
    let description = req.description.or(m.description);
    let is_public = req.is_public.unwrap_or(m.is_public);

    let active_model = cal_calendar::ActiveModel {
        id: Set(id.clone()),
        name: Set(name),
        type_: Set(calendar_type),
        target: Set(target),
        color: Set(color),
        description: Set(description.clone()),
        source: Set(m.source.clone()),
        createor: Set(m.createor.clone()),
        is_public: Set(is_public),
        status: Set(m.status.clone()),
        create_time: Set(m.create_time.clone()),
    };

    let updated = active_model
        .update(&db.0)
        .await
        .map_err(|_| AppError::Internal)?;

    let mut map = serde_json::Map::new();
    map.insert("id".to_string(), Value::String(id));
    map.insert("name".to_string(), Value::String(updated.name.clone()));
    map.insert("type".to_string(), Value::String(updated.type_.clone()));
    map.insert("target".to_string(), Value::String(updated.target.clone()));
    map.insert("color".to_string(), Value::String(updated.color.clone()));
    if let Some(val) = option_to_json(updated.description.clone().map(|s| Value::String(s))) {
        map.insert("description".to_string(), val);
    }
    if let Some(val) = option_to_json(updated.source.clone().map(|s| Value::String(s))) {
        map.insert("source".to_string(), val);
    }
    map.insert("createor".to_string(), Value::String(updated.createor.clone()));
    map.insert("isPublic".to_string(), Value::Bool(updated.is_public));
    map.insert("status".to_string(), Value::String(updated.status.clone()));
    let data = Value::Object(map);

    Ok(Json(ActionResult::success(data)))
}

pub async fn calendar_remove(
    db: Extension<DatabaseConnection>,
    AxumJson(req): AxumJson<DeleteCalendarRequest>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let id = req
        .id
        .ok_or_else(|| AppError::BadRequest("id is required".to_string()))?;

    let existing = cal_calendar::Entity::find_by_id(&id)
        .filter(cal_calendar::Column::Status.eq("OPEN"))
        .one(&db.0)
        .await
        .map_err(|_| AppError::Internal)?;

    let m = existing.ok_or_else(|| AppError::NotFound)?;

    let active_model = cal_calendar::ActiveModel {
        id: Set(m.id.clone()),
        name: Set(m.name.clone()),
        type_: Set(m.type_.clone()),
        target: Set(m.target.clone()),
        color: Set(m.color.clone()),
        description: Set(m.description.clone()),
        source: Set(m.source.clone()),
        createor: Set(m.createor.clone()),
        is_public: Set(m.is_public),
        status: Set("CLOSED".to_string()),
        create_time: Set(m.create_time.clone()),
    };

    let _updated = active_model
        .update(&db.0)
        .await
        .map_err(|_| AppError::Internal)?;

    let data = Value::Object(serde_json::Map::from_iter([
        ("id".to_string(), Value::String(id)),
        ("name".to_string(), Value::String(m.name.clone())),
        ("type".to_string(), Value::String(m.type_.clone())),
        ("target".to_string(), Value::String(m.target.clone())),
        ("color".to_string(), Value::String(m.color.clone())),
        ("status".to_string(), Value::String("CLOSED".to_string())),
    ]));

    Ok(Json(ActionResult::success(data)))
}

pub async fn event_create(
    db: Extension<DatabaseConnection>,
    AxumJson(req): AxumJson<CreateEventRequest>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let calendar_id = req
        .calendar_id
        .ok_or_else(|| AppError::BadRequest("calendarId is required".to_string()))?;
    let title = req
        .title
        .ok_or_else(|| AppError::BadRequest("title is required".to_string()))?;
    let content = req.content;
    let location = req.location;
    let start_time_str = req
        .start_time
        .ok_or_else(|| AppError::BadRequest("\"startTime\" is required".to_string()))?;
    let end_time_str = req
        .end_time
        .ok_or_else(|| AppError::BadRequest("\"endTime\" is required".to_string()))?;
    let all_day = req.all_day.unwrap_or(false);
    let visibility = req.visibility.unwrap_or_else(|| "PUBLIC".to_string());
    let createor = req.createor.unwrap_or_else(|| "anonymous".to_string());

    let id = uuid::Uuid::new_v4().to_string();

    let start_time: chrono::NaiveDateTime = start_time_str.parse().map_err(|_| AppError::BadRequest("invalid \"startTime\"".to_string()))?;
    let end_time: chrono::NaiveDateTime = end_time_str.parse().map_err(|_| AppError::BadRequest("invalid \"endTime\"".to_string()))?;

    let active_model = cal_event::ActiveModel {
        id: Set(id.clone()),
        calendar_id: Set(calendar_id.clone()),
        title: Set(title.clone()),
        content: Set(content.clone()),
        location: Set(location.clone()),
        start_time: Set(start_time),
        end_time: Set(end_time),
        all_day: Set(all_day),
        visibility: Set(visibility.clone()),
        status: Set("OPEN".to_string()),
        createor: Set(createor.clone()),
        create_time: Set(Some(Utc::now().naive_utc())),
    };

    let m = active_model.insert(&db.0).await.map_err(|_| AppError::Internal)?;

    let mut map = serde_json::Map::new();
    map.insert("id".to_string(), Value::String(m.id.clone()));
    map.insert(
        "calendarId".to_string(),
        Value::String(m.calendar_id.clone()),
    );
    map.insert("title".to_string(), Value::String(m.title.clone()));
    if let Some(val) = option_to_json(m.content.clone().map(|s| Value::String(s))) {
        map.insert("content".to_string(), val);
    }
    if let Some(val) = option_to_json(m.location.clone().map(|s| Value::String(s))) {
        map.insert("location".to_string(), val);
    }
    map.insert(
        "\"startTime\"".to_string(),
        Value::String(m.start_time.to_string()),
    );
    map.insert("\"endTime\"".to_string(), Value::String(m.end_time.to_string()));
    map.insert("allDay".to_string(), Value::Bool(m.all_day));
    map.insert("visibility".to_string(), Value::String(m.visibility.clone()));
    map.insert("status".to_string(), Value::String(m.status.clone()));
    map.insert("createor".to_string(), Value::String(m.createor.clone()));
    let data = Value::Object(map);

    Ok(Json(ActionResult::success(data)))
}

pub async fn event_update(
    db: Extension<DatabaseConnection>,
    AxumJson(req): AxumJson<UpdateEventRequest>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let id = req
        .id
        .ok_or_else(|| AppError::BadRequest("id is required".to_string()))?;

    let existing = cal_event::Entity::find_by_id(&id)
        .filter(cal_event::Column::Status.eq("OPEN"))
        .one(&db.0)
        .await
        .map_err(|_| AppError::Internal)?;

    let m = existing.ok_or_else(|| AppError::NotFound)?;

    let title = req.title.unwrap_or(m.title);
    let content = req.content.or(m.content);
    let location = req.location.or(m.location);
    let start_time_str = req.start_time.unwrap_or_else(|| m.start_time.to_string());
    let end_time_str = req.end_time.unwrap_or_else(|| m.end_time.to_string());
    let all_day = req.all_day.unwrap_or(m.all_day);
    let visibility = req.visibility.unwrap_or(m.visibility);
    let status = req.status.unwrap_or(m.status);

    let start_time: chrono::NaiveDateTime = start_time_str.parse().map_err(|_| AppError::BadRequest("invalid \"startTime\"".to_string()))?;
    let end_time: chrono::NaiveDateTime = end_time_str.parse().map_err(|_| AppError::BadRequest("invalid \"endTime\"".to_string()))?;

    let active_model = cal_event::ActiveModel {
        id: Set(id.clone()),
        calendar_id: Set(m.calendar_id.clone()),
        title: Set(title),
        content: Set(content.clone()),
        location: Set(location.clone()),
        start_time: Set(start_time),
        end_time: Set(end_time),
        all_day: Set(all_day),
        visibility: Set(visibility.clone()),
        status: Set(status.clone()),
        createor: Set(m.createor.clone()),
        create_time: Set(m.create_time.clone()),
    };

    let updated = active_model
        .update(&db.0)
        .await
        .map_err(|_| AppError::Internal)?;

    let mut map = serde_json::Map::new();
    map.insert("id".to_string(), Value::String(id));
    map.insert(
        "calendarId".to_string(),
        Value::String(updated.calendar_id.clone()),
    );
    map.insert("title".to_string(), Value::String(updated.title.clone()));
    if let Some(val) = option_to_json(updated.content.clone().map(|s| Value::String(s))) {
        map.insert("content".to_string(), val);
    }
    if let Some(val) = option_to_json(updated.location.clone().map(|s| Value::String(s))) {
        map.insert("location".to_string(), val);
    }
    map.insert(
        "\"startTime\"".to_string(),
        Value::String(updated.start_time.to_string()),
    );
    map.insert(
        "\"endTime\"".to_string(),
        Value::String(updated.end_time.to_string()),
    );
    map.insert("allDay".to_string(), Value::Bool(updated.all_day));
    map.insert("visibility".to_string(), Value::String(updated.visibility.clone()));
    map.insert("status".to_string(), Value::String(updated.status.clone()));
    map.insert("createor".to_string(), Value::String(updated.createor.clone()));
    let data = Value::Object(map);

    Ok(Json(ActionResult::success(data)))
}

pub async fn event_remove(
    db: Extension<DatabaseConnection>,
    AxumJson(req): AxumJson<DeleteEventRequest>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let id = req
        .id
        .ok_or_else(|| AppError::BadRequest("id is required".to_string()))?;

    let existing = cal_event::Entity::find_by_id(&id)
        .filter(cal_event::Column::Status.eq("OPEN"))
        .one(&db.0)
        .await
        .map_err(|_| AppError::Internal)?;

    let m = existing.ok_or_else(|| AppError::NotFound)?;

    let active_model = cal_event::ActiveModel {
        id: Set(m.id.clone()),
        calendar_id: Set(m.calendar_id.clone()),
        title: Set(m.title.clone()),
        content: Set(m.content.clone()),
        location: Set(m.location.clone()),
        start_time: Set(m.start_time),
        end_time: Set(m.end_time),
        all_day: Set(m.all_day),
        visibility: Set(m.visibility.clone()),
        status: Set("CLOSED".to_string()),
        createor: Set(m.createor.clone()),
        create_time: Set(m.create_time.clone()),
    };

    let _updated = active_model
        .update(&db.0)
        .await
        .map_err(|_| AppError::Internal)?;

    let data = Value::Object(serde_json::Map::from_iter([
        ("id".to_string(), Value::String(id)),
        ("title".to_string(), Value::String(m.title.clone())),
        ("status".to_string(), Value::String("CLOSED".to_string())),
    ]));

    Ok(Json(ActionResult::success(data)))
}

pub async fn event_list_by_calendar(
    db: Extension<DatabaseConnection>,
    Path(calendar_id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let models = cal_event::Entity::find()
        .filter(
            cal_event::Column::CalendarId.eq(&calendar_id)
                .and(cal_event::Column::Status.eq("OPEN")),
        )
        .order_by_asc(cal_event::Column::StartTime)
        .limit(100)
        .all(&db.0)
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = models
        .iter()
        .map(|m| {
            let mut map = serde_json::Map::new();
            map.insert("id".to_string(), Value::String(m.id.clone()));
            map.insert(
                "calendarId".to_string(),
                Value::String(m.calendar_id.clone()),
            );
            map.insert("title".to_string(), Value::String(m.title.clone()));
            if let Some(val) = option_to_json(m.content.clone().map(|s| Value::String(s))) {
                map.insert("content".to_string(), val);
            }
            if let Some(val) = option_to_json(m.location.clone().map(|s| Value::String(s))) {
                map.insert("location".to_string(), val);
            }
            map.insert(
                "\"startTime\"".to_string(),
                Value::String(m.start_time.to_string()),
            );
            map.insert("\"endTime\"".to_string(), Value::String(m.end_time.to_string()));
            map.insert("allDay".to_string(), Value::Bool(m.all_day));
            map.insert("visibility".to_string(), Value::String(m.visibility.clone()));
            map.insert("status".to_string(), Value::String(m.status.clone()));
            map.insert("createor".to_string(), Value::String(m.createor.clone()));
            Value::Object(map)
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            (
                "count".to_string(),
                Value::Number(serde_json::Number::from(data.len() as i64)),
            ),
            (
                "calendarId".to_string(),
                Value::String(calendar_id),
            ),
            ("data".to_string(), Value::Array(data)),
        ]),
    ))))
}

pub fn calendar_core_entity_router(_pool: deadpool_postgres::Pool) -> Router {
    Router::new()
        .route(
            "/jaxrs/calendar/core/entity/calendar/list/public",
            get(calendar_list_public),
        )
        .route(
            "/jaxrs/calendar/core/entity/calendar/list/my",
            get(calendar_list_my),
        )
        .route(
            "/jaxrs/calendar/core/entity/calendar/{id}",
            get(calendar_get),
        )
        .route(
            "/jaxrs/calendar/core/entity/calendar/create",
            post(calendar_create),
        )
        .route(
            "/jaxrs/calendar/core/entity/calendar/update",
            post(calendar_update),
        )
        .route(
            "/jaxrs/calendar/core/entity/calendar/remove",
            post(calendar_remove),
        )
        .route(
            "/jaxrs/calendar/core/entity/event/create",
            post(event_create),
        )
        .route(
            "/jaxrs/calendar/core/entity/event/update",
            post(event_update),
        )
        .route(
            "/jaxrs/calendar/core/entity/event/remove",
            post(event_remove),
        )
        .route(
            "/jaxrs/calendar/core/entity/event/list/{calendarId}",
            get(event_list_by_calendar),
        )
}

#[cfg(test)]
mod tests;
#[cfg(test)]
mod tests_generated;


pub fn router(pool: deadpool_postgres::Pool) -> axum::Router {
    crate::calendar_core_entity_router(pool)
}

