use axum::{
    extract::Extension,
    Json,
};
use deadpool_postgres::Pool;
use serde::Serialize;
use shared::{error::AppError, response::ActionResult};

pub mod routes;

#[cfg(test)]
mod tests;

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
            id: row.get("id"),
            name: row.get("name"),
            calendar_type: row.get("type"),
            target: row.get("target"),
            color: row.get("color"),
            description: row.get("description"),
            source: row.get("source"),
            createor: row.get("createor"),
            is_public: row.get("is_public"),
            status: row.get("status"),
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
        let calendar_type: String = row.get("type");
        let item = CalendarItem {
            id: row.get("id"),
            name: row.get("name"),
            calendar_type,
            target: row.get("target"),
            color: row.get("color"),
            description: row.get("description"),
            source: row.get("source"),
            createor: row.get("createor"),
            is_public: row.get("is_public"),
            status: row.get("status"),
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
        id: row.get("id"),
        name: row.get("name"),
        calendar_type: row.get("type"),
        target: row.get("target"),
        color: row.get("color"),
        description: row.get("description"),
        source: row.get("source"),
        createor: row.get("createor"),
        is_public: row.get("is_public"),
        status: row.get("status"),
    };

    Ok(Json(ActionResult::success(item)))
}

pub use routes::calendar_router;
