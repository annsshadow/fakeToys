// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

use axum::{
    extract::Extension,
    routing::{get, post},
    Router,
};
use deadpool_postgres::Pool;

use crate::{
    calendar_create, calendar_get, calendar_list_my, calendar_list_public, calendar_remove,
    calendar_update, event_create, event_list, event_remove, event_update,
};

pub fn calendar_router(pool: Pool) -> Router {
    Router::new()
        .route(
            "/api/calendar/calendar/list/public",
            get(calendar_list_public),
        )
        .route("/api/calendar/calendar/list/my", get(calendar_list_my))
        .route("/api/calendar/calendar/{id}", get(calendar_get))
        .route("/api/calendar/calendar/create", post(calendar_create))
        .route("/api/calendar/calendar/update", post(calendar_update))
        .route("/api/calendar/calendar/remove", post(calendar_remove))
        .route("/api/calendar/event/create", post(event_create))
        .route("/api/calendar/event/update", post(event_update))
        .route("/api/calendar/event/remove", post(event_remove))
        .route("/api/calendar/event/list/{calendarId}", get(event_list))
        .layer(Extension(pool))
}
