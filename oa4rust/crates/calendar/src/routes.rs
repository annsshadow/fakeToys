use axum::{
    routing::get,
    Router,
};
use deadpool_postgres::Pool;

use crate::{calendar_get, calendar_list_my, calendar_list_public};

pub fn calendar_router(pool: Pool) -> Router {
    Router::new()
        .route("/jaxrs/calendar/calendar/list/public", get(calendar_list_public))
        .route("/jaxrs/calendar/calendar/list/my", get(calendar_list_my))
        .route("/jaxrs/calendar/calendar/{id}", get(calendar_get))
        .layer(axum::Extension(pool))
}
