use axum::{
    routing::{get, post},
    Router,
};
use deadpool_postgres::Pool;

use crate::{
    add_participant, building_list, create_meeting, get_meeting, list_meetings, list_participants,
    list_schedule, openmeeting_list_room, room_list,
};

pub fn meeting_router(pool: Pool) -> Router {
    Router::new()
        .route("/api/meeting/room/list", get(room_list))
        .route("/api/meeting/building/list", get(building_list))
        .route(
            "/api/meeting/openmeeting/list/room",
            get(openmeeting_list_room),
        )
        .route("/api/meeting/create", post(create_meeting))
        .route("/api/meeting/{id}", get(get_meeting))
        .route("/api/meeting/list", get(list_meetings))
        .route("/api/meeting/schedule/days/{days}", get(list_schedule))
        .route(
            "/api/meeting/{\"meetingId\"}/participant/add",
            post(add_participant),
        )
        .route(
            "/api/meeting/{\"meetingId\"}/participant/list",
            get(list_participants),
        )
        .layer(axum::Extension(pool))
}
