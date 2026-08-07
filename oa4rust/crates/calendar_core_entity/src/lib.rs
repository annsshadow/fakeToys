use axum::{
    extract::Extension,
    routing::{get, post},
    Json, Router,
};
use deadpool_postgres::Pool;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use shared::{error::AppError, response::ActionResult};

pub mod routes;

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
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT id, name, type, target, color, description, createor, is_public, status \
             FROM CAL_CALENDAR WHERE is_public = true AND status = 'OPEN' ORDER BY create_time DESC LIMIT 50",
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
                ("type".to_string(), Value::String(row.get("type"))),
                ("target".to_string(), Value::String(row.get("target"))),
                ("color".to_string(), Value::String(row.get("color"))),
                (
                    "description".to_string(),
                    row.get::<_, Option<String>>("description")
                        .map(Value::String)
                        .unwrap_or(Value::Null),
                ),
                ("createor".to_string(), Value::String(row.get("createor"))),
                ("isPublic".to_string(), Value::Bool(row.get("is_public"))),
                ("status".to_string(), Value::String(row.get("status"))),
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

pub async fn calendar_list_my(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT id, name, type, target, color, description, createor, is_public, status \
             FROM CAL_CALENDAR WHERE status = 'OPEN' ORDER BY create_time DESC LIMIT 50",
            &[],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let mut my_calendars = Vec::new();
    let mut unit_calendars = Vec::new();

    for row in rows.iter() {
        let calendar_type: String = row.get("type");
        let item: Value = Value::Object(serde_json::Map::from_iter([
            ("id".to_string(), Value::String(row.get("id"))),
            ("name".to_string(), Value::String(row.get("name"))),
            ("type".to_string(), Value::String(calendar_type.clone())),
            ("target".to_string(), Value::String(row.get("target"))),
            ("color".to_string(), Value::String(row.get("color"))),
            (
                "description".to_string(),
                row.get::<_, Option<String>>("description")
                    .map(Value::String)
                    .unwrap_or(Value::Null),
            ),
            ("createor".to_string(), Value::String(row.get("createor"))),
            ("isPublic".to_string(), Value::Bool(row.get("is_public"))),
            ("status".to_string(), Value::String(row.get("status"))),
        ]));

        if calendar_type.eq_ignore_ascii_case("UNIT") {
            unit_calendars.push(item);
        } else {
            my_calendars.push(item);
        }
    }

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            (
                "myCalendars".to_string(),
                Value::Array(my_calendars),
            ),
            (
                "unitCalendars".to_string(),
                Value::Array(unit_calendars),
            ),
            (
                "followCalendars".to_string(),
                Value::Array(Vec::<Value>::new()),
            ),
        ]),
    ))))
}

pub async fn calendar_get(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_one(
            "SELECT id, name, type, target, color, description, createor, is_public, status \
             FROM CAL_CALENDAR WHERE id = $1 AND status = 'OPEN'",
            &[&id],
        )
        .await
        .map_err(|_| AppError::NotFound)?;

    let data = Value::Object(serde_json::Map::from_iter([
        ("id".to_string(), Value::String(row.get("id"))),
        ("name".to_string(), Value::String(row.get("name"))),
        ("type".to_string(), Value::String(row.get("type"))),
        ("target".to_string(), Value::String(row.get("target"))),
        ("color".to_string(), Value::String(row.get("color"))),
        (
            "description".to_string(),
            row.get::<_, Option<String>>("description")
                .map(Value::String)
                .unwrap_or(Value::Null),
        ),
        ("createor".to_string(), Value::String(row.get("createor"))),
        ("isPublic".to_string(), Value::Bool(row.get("is_public"))),
        ("status".to_string(), Value::String(row.get("status"))),
    ]));

    Ok(Json(ActionResult::success(data)))
}

pub async fn calendar_create(
    pool: Extension<Pool>,
    axum::extract::Json(req): axum::extract::Json<CreateCalendarRequest>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let name = req.name.ok_or_else(|| AppError::BadRequest("name is required".to_string()))?;
    let calendar_type = req
        .calendar_type
        .ok_or_else(|| AppError::BadRequest("type is required".to_string()))?;
    let target = req.target.unwrap_or_else(|| "person".to_string());
    let color = req.color.unwrap_or_else(|| "#1462be".to_string());
    let description = req.description;
    let source = req.source;
    let createor = req
        .createor
        .unwrap_or_else(|| "anonymous".to_string());
    let is_public = req.is_public.unwrap_or(false);

    let id = uuid::Uuid::new_v4().to_string();

    client
        .execute(
            "INSERT INTO CAL_CALENDAR (id, name, type, target, color, description, source, createor, is_public, status) \
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, 'OPEN')",
            &[&id, &name, &calendar_type, &target, &color, &description, &source, &createor, &is_public],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data = Value::Object(serde_json::Map::from_iter([
        ("id".to_string(), Value::String(id)),
        ("name".to_string(), Value::String(name)),
        ("type".to_string(), Value::String(calendar_type)),
        ("target".to_string(), Value::String(target)),
        ("color".to_string(), Value::String(color)),
        (
            "description".to_string(),
            description.map(Value::String).unwrap_or(Value::Null),
        ),
        (
            "source".to_string(),
            source.map(Value::String).unwrap_or(Value::Null),
        ),
        ("createor".to_string(), Value::String(createor)),
        ("isPublic".to_string(), Value::Bool(is_public)),
        ("status".to_string(), Value::String("OPEN".to_string())),
    ]));

    Ok(Json(ActionResult::success(data)))
}

pub async fn calendar_update(
    pool: Extension<Pool>,
    axum::extract::Json(req): axum::extract::Json<UpdateCalendarRequest>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let id = req
        .id
        .ok_or_else(|| AppError::BadRequest("id is required".to_string()))?;

    let existing = client
        .query_one(
            "SELECT id, name, type, target, color, description, source, createor, is_public, status \
             FROM CAL_CALENDAR WHERE id = $1 AND status = 'OPEN'",
            &[&id],
        )
        .await
        .map_err(|_| AppError::NotFound)?;

    let name = req.name.unwrap_or_else(|| existing.get("name"));
    let calendar_type = req
        .calendar_type
        .unwrap_or_else(|| existing.get("type"));
    let target = req
        .target
        .unwrap_or_else(|| existing.get("target"));
    let color = req
        .color
        .unwrap_or_else(|| existing.get("color"));
    let description = req
        .description
        .or_else(|| existing.get("description"));
    let is_public = req
        .is_public
        .unwrap_or_else(|| existing.get("is_public"));

    client
        .execute(
            "UPDATE CAL_CALENDAR SET name = $1, type = $2, target = $3, color = $4, description = $5, is_public = $6 \
             WHERE id = $7",
            &[&name, &calendar_type, &target, &color, &description, &is_public, &id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let source: Option<String> = existing.get("source");
    let createor: String = existing.get("createor");
    let status: String = existing.get("status");

    let data = Value::Object(serde_json::Map::from_iter([
        ("id".to_string(), Value::String(id)),
        ("name".to_string(), Value::String(name)),
        ("type".to_string(), Value::String(calendar_type)),
        ("target".to_string(), Value::String(target)),
        ("color".to_string(), Value::String(color)),
        (
            "description".to_string(),
            description.map(Value::String).unwrap_or(Value::Null),
        ),
        (
            "source".to_string(),
            source.map(Value::String).unwrap_or(Value::Null),
        ),
        ("createor".to_string(), Value::String(createor)),
        ("isPublic".to_string(), Value::Bool(is_public)),
        ("status".to_string(), Value::String(status)),
    ]));

    Ok(Json(ActionResult::success(data)))
}

pub async fn calendar_remove(
    pool: Extension<Pool>,
    axum::extract::Json(req): axum::extract::Json<DeleteCalendarRequest>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let id = req
        .id
        .ok_or_else(|| AppError::BadRequest("id is required".to_string()))?;

    let existing = client
        .query_one(
            "SELECT id, name, type, target, color, description, source, createor, is_public, status \
             FROM CAL_CALENDAR WHERE id = $1 AND status = 'OPEN'",
            &[&id],
        )
        .await
        .map_err(|_| AppError::NotFound)?;

    client
        .execute(
            "UPDATE CAL_CALENDAR SET status = 'CLOSED' WHERE id = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data = Value::Object(serde_json::Map::from_iter([
        ("id".to_string(), Value::String(id)),
        ("name".to_string(), Value::String(existing.get("name"))),
        ("type".to_string(), Value::String(existing.get("type"))),
        ("target".to_string(), Value::String(existing.get("target"))),
        ("color".to_string(), Value::String(existing.get("color"))),
        ("status".to_string(), Value::String("CLOSED".to_string())),
    ]));

    Ok(Json(ActionResult::success(data)))
}

pub async fn event_create(
    pool: Extension<Pool>,
    axum::extract::Json(req): axum::extract::Json<CreateEventRequest>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let calendar_id = req
        .calendar_id
        .ok_or_else(|| AppError::BadRequest("calendarId is required".to_string()))?;
    let title = req
        .title
        .ok_or_else(|| AppError::BadRequest("title is required".to_string()))?;
    let content = req.content;
    let location = req.location;
    let start_time = req
        .start_time
        .ok_or_else(|| AppError::BadRequest("startTime is required".to_string()))?;
    let end_time = req
        .end_time
        .ok_or_else(|| AppError::BadRequest("endTime is required".to_string()))?;
    let all_day = req.all_day.unwrap_or(false);
    let visibility = req.visibility.unwrap_or_else(|| "PUBLIC".to_string());
    let createor = req
        .createor
        .unwrap_or_else(|| "anonymous".to_string());

    let id = uuid::Uuid::new_v4().to_string();

    client
        .execute(
            "INSERT INTO CAL_EVENT (id, calendar_id, title, content, location, start_time, end_time, all_day, visibility, status, createor) \
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, 'OPEN', $10)",
            &[&id, &calendar_id, &title, &content, &location, &start_time, &end_time, &all_day, &visibility, &createor],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data = Value::Object(serde_json::Map::from_iter([
        ("id".to_string(), Value::String(id)),
        ("calendarId".to_string(), Value::String(calendar_id)),
        ("title".to_string(), Value::String(title)),
        (
            "content".to_string(),
            content.map(Value::String).unwrap_or(Value::Null),
        ),
        (
            "location".to_string(),
            location.map(Value::String).unwrap_or(Value::Null),
        ),
        ("startTime".to_string(), Value::String(start_time)),
        ("endTime".to_string(), Value::String(end_time)),
        ("allDay".to_string(), Value::Bool(all_day)),
        ("visibility".to_string(), Value::String(visibility)),
        ("status".to_string(), Value::String("OPEN".to_string())),
        ("createor".to_string(), Value::String(createor)),
    ]));

    Ok(Json(ActionResult::success(data)))
}

pub async fn event_update(
    pool: Extension<Pool>,
    axum::extract::Json(req): axum::extract::Json<UpdateEventRequest>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let id = req
        .id
        .ok_or_else(|| AppError::BadRequest("id is required".to_string()))?;

    let existing = client
        .query_one(
            "SELECT id, calendar_id, title, content, location, start_time, end_time, all_day, visibility, status, createor \
             FROM CAL_EVENT WHERE id = $1 AND status = 'OPEN'",
            &[&id],
        )
        .await
        .map_err(|_| AppError::NotFound)?;

    let title = req.title.unwrap_or_else(|| existing.get("title"));
    let content = req.content.or_else(|| existing.get("content"));
    let location = req.location.or_else(|| existing.get("location"));
    let start_time = req
        .start_time
        .unwrap_or_else(|| existing.get::<_, String>("start_time"));
    let end_time = req
        .end_time
        .unwrap_or_else(|| existing.get::<_, String>("end_time"));
    let all_day = req.all_day.unwrap_or_else(|| existing.get("all_day"));
    let visibility = req
        .visibility
        .unwrap_or_else(|| existing.get("visibility"));
    let status = req
        .status
        .unwrap_or_else(|| existing.get("status"));
    let calendar_id: String = existing.get("calendar_id");
    let createor: String = existing.get("createor");

    client
        .execute(
            "UPDATE CAL_EVENT SET title = $1, content = $2, location = $3, start_time = $4, end_time = $5, all_day = $6, visibility = $7, status = $8 \
             WHERE id = $9",
            &[&title, &content, &location, &start_time, &end_time, &all_day, &visibility, &status, &id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data = Value::Object(serde_json::Map::from_iter([
        ("id".to_string(), Value::String(id)),
        ("calendarId".to_string(), Value::String(calendar_id)),
        ("title".to_string(), Value::String(title)),
        (
            "content".to_string(),
            content.map(Value::String).unwrap_or(Value::Null),
        ),
        (
            "location".to_string(),
            location.map(Value::String).unwrap_or(Value::Null),
        ),
        ("startTime".to_string(), Value::String(start_time)),
        ("endTime".to_string(), Value::String(end_time)),
        ("allDay".to_string(), Value::Bool(all_day)),
        ("visibility".to_string(), Value::String(visibility)),
        ("status".to_string(), Value::String(status)),
        ("createor".to_string(), Value::String(createor)),
    ]));

    Ok(Json(ActionResult::success(data)))
}

pub async fn event_remove(
    pool: Extension<Pool>,
    axum::extract::Json(req): axum::extract::Json<DeleteEventRequest>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let id = req
        .id
        .ok_or_else(|| AppError::BadRequest("id is required".to_string()))?;

    let existing = client
        .query_one(
            "SELECT id, calendar_id, title, content, location, start_time, end_time, all_day, visibility, status, createor \
             FROM CAL_EVENT WHERE id = $1 AND status = 'OPEN'",
            &[&id],
        )
        .await
        .map_err(|_| AppError::NotFound)?;

    client
        .execute(
            "UPDATE CAL_EVENT SET status = 'CLOSED' WHERE id = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data = Value::Object(serde_json::Map::from_iter([
        ("id".to_string(), Value::String(id)),
        ("title".to_string(), Value::String(existing.get("title"))),
        ("status".to_string(), Value::String("CLOSED".to_string())),
    ]));

    Ok(Json(ActionResult::success(data)))
}

pub async fn event_list_by_calendar(
    pool: Extension<Pool>,
    axum::extract::Path(calendar_id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query(
            "SELECT id, calendar_id, title, content, location, start_time, end_time, all_day, visibility, status, createor \
             FROM CAL_EVENT WHERE calendar_id = $1 AND status = 'OPEN' ORDER BY start_time ASC LIMIT 100",
            &[&calendar_id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("calendarId".to_string(), Value::String(row.get("calendar_id"))),
                ("title".to_string(), Value::String(row.get("title"))),
                (
                    "content".to_string(),
                    row.get::<_, Option<String>>("content")
                        .map(Value::String)
                        .unwrap_or(Value::Null),
                ),
                (
                    "location".to_string(),
                    row.get::<_, Option<String>>("location")
                        .map(Value::String)
                        .unwrap_or(Value::Null),
                ),
                ("startTime".to_string(), Value::String(row.get("start_time"))),
                ("endTime".to_string(), Value::String(row.get("end_time"))),
                ("allDay".to_string(), Value::Bool(row.get("all_day"))),
                ("visibility".to_string(), Value::String(row.get("visibility"))),
                ("status".to_string(), Value::String(row.get("status"))),
                ("createor".to_string(), Value::String(row.get("createor"))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
            ("calendarId".to_string(), Value::String(calendar_id)),
            ("data".to_string(), Value::Array(data)),
        ]),
    ))))
}

pub fn calendar_core_entity_router(pool: Pool) -> Router {
    Router::new()
        .route("/jaxrs/calendar/core/entity/calendar/list/public", get(calendar_list_public))
        .route("/jaxrs/calendar/core/entity/calendar/list/my", get(calendar_list_my))
        .route("/jaxrs/calendar/core/entity/calendar/{id}", get(calendar_get))
        .route("/jaxrs/calendar/core/entity/calendar/create", post(calendar_create))
        .route("/jaxrs/calendar/core/entity/calendar/update", post(calendar_update))
        .route("/jaxrs/calendar/core/entity/calendar/remove", post(calendar_remove))
        .route("/jaxrs/calendar/core/entity/event/create", post(event_create))
        .route("/jaxrs/calendar/core/entity/event/update", post(event_update))
        .route("/jaxrs/calendar/core/entity/event/remove", post(event_remove))
        .route("/jaxrs/calendar/core/entity/event/list/{calendarId}", get(event_list_by_calendar))
        .layer(Extension(pool))
}

#[cfg(test)]
mod tests;

pub fn router(pool: deadpool_postgres::Pool) -> axum::Router {
    crate::calendar_core_entity_router(pool)
}

