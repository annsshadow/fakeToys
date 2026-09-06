use axum::{
    extract::Extension,
    Json,
};
use deadpool_postgres::Pool;
use serde::{Deserialize, Serialize};
use shared::{error::AppError, response::ActionResult};

pub mod routes;

#[cfg(test)]
mod tests;
#[cfg(test)]
mod tests_generated;


#[derive(Debug, Serialize)]
pub struct CalendarItem {
    pub id: String,
    pub name: String,
    #[serde(rename = "type")]
    pub calendar_type: String,
    pub target: String,
    pub color: String,
    pub description: Option<String>,
    pub source: Option<String>,
    pub createor: String,
    #[serde(rename = "isPublic")]
    pub is_public: bool,
    pub status: String,
}

#[derive(Debug, Serialize)]
pub struct MyCalendarResponse {
    pub my_calendars: Vec<CalendarItem>,
    pub unit_calendars: Vec<CalendarItem>,
    pub follow_calendars: Vec<CalendarItem>,
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

#[derive(Debug, Serialize, Deserialize)]
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

#[derive(Debug, Deserialize)]
pub struct ListEventsRequest {
    pub calendar_id: Option<String>,
}

pub async fn calendar_list_public(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Vec<CalendarItem>>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query(
            "SELECT id, name, type, target, color, description, source, createor, is_public, status \
             FROM CAL_CALENDAR WHERE is_public = true AND status = 'OPEN' ORDER BY create_time DESC",
            &[],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<CalendarItem> = rows
        .iter()
        .map(|row| CalendarItem {
            id: row.get::<_, Option<String>>("id").unwrap_or_default(),
            name: row.get::<_, Option<String>>("name").unwrap_or_default(),
            calendar_type: row.get::<_, Option<String>>("type").unwrap_or_default(),
            target: row.get::<_, Option<String>>("target").unwrap_or_default(),
            color: row.get::<_, Option<String>>("color").unwrap_or_default(),
            description: row.get("description"),
            source: row.get("source"),
            createor: row.get::<_, Option<String>>("createor").unwrap_or_default(),
            is_public: row.get("is_public"),
            status: row.get::<_, Option<String>>("status").unwrap_or_default(),
        })
        .collect();

    Ok(Json(ActionResult::success(data)))
}

pub async fn calendar_list_my(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<MyCalendarResponse>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query(
            "SELECT id, name, type, target, color, description, source, createor, is_public, status \
             FROM CAL_CALENDAR WHERE status = 'OPEN' ORDER BY create_time DESC",
            &[],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let mut my_calendars = Vec::new();
    let mut unit_calendars = Vec::new();

    for row in rows.iter() {
        let calendar_type: String = row.get::<_, Option<String>>("type").unwrap_or_default();
        let item = CalendarItem {
            id: row.get::<_, Option<String>>("id").unwrap_or_default(),
            name: row.get::<_, Option<String>>("name").unwrap_or_default(),
            calendar_type,
            target: row.get::<_, Option<String>>("target").unwrap_or_default(),
            color: row.get::<_, Option<String>>("color").unwrap_or_default(),
            description: row.get("description"),
            source: row.get("source"),
            createor: row.get::<_, Option<String>>("createor").unwrap_or_default(),
            is_public: row.get("is_public"),
            status: row.get::<_, Option<String>>("status").unwrap_or_default(),
        };

        if item.calendar_type.eq_ignore_ascii_case("UNIT") {
            unit_calendars.push(item);
        } else {
            my_calendars.push(item);
        }
    }

    let response = MyCalendarResponse {
        my_calendars,
        unit_calendars,
        follow_calendars: Vec::new(),
    };

    Ok(Json(ActionResult::success(response)))
}

pub async fn calendar_get(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<CalendarItem>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let row = client
        .query_one(
            "SELECT id, name, type, target, color, description, source, createor, is_public, status \
             FROM CAL_CALENDAR WHERE id = $1 AND status = 'OPEN'",
            &[&id],
        )
        .await
        .map_err(|_| AppError::NotFound)?;

    let item = CalendarItem {
        id: row.get::<_, Option<String>>("id").unwrap_or_default(),
        name: row.get::<_, Option<String>>("name").unwrap_or_default(),
        calendar_type: row.get::<_, Option<String>>("type").unwrap_or_default(),
        target: row.get::<_, Option<String>>("target").unwrap_or_default(),
        color: row.get::<_, Option<String>>("color").unwrap_or_default(),
        description: row.get("description"),
        source: row.get("source"),
        createor: row.get::<_, Option<String>>("createor").unwrap_or_default(),
        is_public: row.get("is_public"),
        status: row.get::<_, Option<String>>("status").unwrap_or_default(),
    };

    Ok(Json(ActionResult::success(item)))
}

pub async fn calendar_create(
    pool: Extension<Pool>,
    axum::extract::Json(req): axum::extract::Json<CreateCalendarRequest>,
) -> Result<Json<ActionResult<CalendarItem>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let name = req.name.ok_or_else(|| AppError::BadRequest("name is required".to_string()))?;
    let calendar_type = req
        .calendar_type
        .ok_or_else(|| AppError::BadRequest("type is required".to_string()))?;
    let target = req
        .target
        .unwrap_or_else(|| "person".to_string());
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

    let item = CalendarItem {
        id,
        name,
        calendar_type,
        target,
        color,
        description,
        source,
        createor,
        is_public,
        status: "OPEN".to_string(),
    };

    Ok(Json(ActionResult::success(item)))
}

pub async fn calendar_update(
    pool: Extension<Pool>,
    axum::extract::Json(req): axum::extract::Json<UpdateCalendarRequest>,
) -> Result<Json<ActionResult<CalendarItem>>, AppError> {
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
    let description = req.description.or_else(|| existing.get("description"));
    let is_public = req
        .is_public
        .unwrap_or_else(|| existing.get("is_public"));
    let status: String = existing.get("status");
    let source: Option<String> = existing.get("source");
    let createor: String = existing.get("createor");

    client
        .execute(
            "UPDATE CAL_CALENDAR SET name = $1, type = $2, target = $3, color = $4, description = $5, is_public = $6 \
             WHERE id = $7",
            &[&name, &calendar_type, &target, &color, &description, &is_public, &id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let item = CalendarItem {
        id,
        name,
        calendar_type,
        target,
        color,
        description,
        source,
        createor,
        is_public,
        status,
    };

    Ok(Json(ActionResult::success(item)))
}

pub async fn calendar_remove(
    pool: Extension<Pool>,
    axum::extract::Json(req): axum::extract::Json<DeleteCalendarRequest>,
) -> Result<Json<ActionResult<CalendarItem>>, AppError> {
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

    let item = CalendarItem {
        id: existing.get("id"),
        name: existing.get("name"),
        calendar_type: existing.get("type"),
        target: existing.get("target"),
        color: existing.get("color"),
        description: existing.get("description"),
        source: existing.get("source"),
        createor: existing.get("createor"),
        is_public: existing.get("is_public"),
        status: "CLOSED".to_string(),
    };

    Ok(Json(ActionResult::success(item)))
}

pub async fn event_create(
    pool: Extension<Pool>,
    axum::extract::Json(req): axum::extract::Json<CreateEventRequest>,
) -> Result<Json<ActionResult<CalendarEvent>>, AppError> {
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

    let event = CalendarEvent {
        id,
        calendar_id,
        title,
        content,
        location,
        start_time,
        end_time,
        all_day,
        visibility,
        status: "OPEN".to_string(),
        createor,
    };

    Ok(Json(ActionResult::success(event)))
}

pub async fn event_update(
    pool: Extension<Pool>,
    axum::extract::Json(req): axum::extract::Json<UpdateEventRequest>,
) -> Result<Json<ActionResult<CalendarEvent>>, AppError> {
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
        .unwrap_or_else(|| existing.get::<_, String>("start_time").clone());
    let end_time = req
        .end_time
        .unwrap_or_else(|| existing.get::<_, String>("end_time").clone());
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

    let event = CalendarEvent {
        id,
        calendar_id,
        title,
        content,
        location,
        start_time,
        end_time,
        all_day,
        visibility,
        status,
        createor,
    };

    Ok(Json(ActionResult::success(event)))
}

pub async fn event_remove(
    pool: Extension<Pool>,
    axum::extract::Json(req): axum::extract::Json<DeleteEventRequest>,
) -> Result<Json<ActionResult<CalendarEvent>>, AppError> {
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

    let event = CalendarEvent {
        id: existing.get("id"),
        calendar_id: existing.get("calendar_id"),
        title: existing.get("title"),
        content: existing.get("content"),
        location: existing.get("location"),
        start_time: existing.get("start_time"),
        end_time: existing.get("end_time"),
        all_day: existing.get("all_day"),
        visibility: existing.get("visibility"),
        status: "CLOSED".to_string(),
        createor: existing.get("createor"),
    };

    Ok(Json(ActionResult::success(event)))
}

pub async fn event_list(
    pool: Extension<Pool>,
    axum::extract::Path(calendar_id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Vec<CalendarEvent>>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query(
            "SELECT id, calendar_id, title, content, location, start_time, end_time, all_day, visibility, status, createor \
             FROM CAL_EVENT WHERE calendar_id = $1 AND status = 'OPEN' ORDER BY start_time ASC",
            &[&calendar_id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<CalendarEvent> = rows
        .iter()
        .map(|row| CalendarEvent {
            id: row.get::<_, Option<String>>("id").unwrap_or_default(),
            calendar_id: row.get("calendar_id"),
            title: row.get("title"),
            content: row.get("content"),
            location: row.get("location"),
            start_time: row.get("start_time"),
            end_time: row.get("end_time"),
            all_day: row.get("all_day"),
            visibility: row.get("visibility"),
            status: row.get::<_, Option<String>>("status").unwrap_or_default(),
            createor: row.get::<_, Option<String>>("createor").unwrap_or_default(),
        })
        .collect();

    Ok(Json(ActionResult::success(data)))
}

pub use routes::calendar_router;

pub fn router(pool: deadpool_postgres::Pool) -> axum::Router {
    crate::calendar_router(pool)
}

