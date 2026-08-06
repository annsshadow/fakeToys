use axum::{
    extract::Extension,
    routing::{get, post},
    Router,
};
use deadpool_postgres::Pool;

use crate::{
    calendar_create, calendar_get, calendar_list_my, calendar_list_public,
    calendar_remove, calendar_update, event_create, event_list, event_remove, event_update,
};

pub fn calendar_router(pool: Pool) -> Router {
    Router::new()
        .route("/jaxrs/calendar/calendar/list/public", get(calendar_list_public))
        .route("/jaxrs/calendar/calendar/list/my", get(calendar_list_my))
        .route("/jaxrs/calendar/calendar/{id}", get(calendar_get))
        .route("/jaxrs/calendar/calendar/create", post(calendar_create))
        .route("/jaxrs/calendar/calendar/update", post(calendar_update))
        .route("/jaxrs/calendar/calendar/remove", post(calendar_remove))
        .route("/jaxrs/calendar/event/create", post(event_create))
        .route("/jaxrs/calendar/event/update", post(event_update))
        .route("/jaxrs/calendar/event/remove", post(event_remove))
        .route("/jaxrs/calendar/event/list/{calendarId}", get(event_list))
        .layer(Extension(pool))
}
