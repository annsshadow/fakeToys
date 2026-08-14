use axum::{
    routing::{get, post},
    Router,
};
use deadpool_postgres::Pool;

use crate::{
    add_participant, building_list, create_meeting, get_meeting, list_meetings,
    list_participants, list_schedule, openmeeting_list_room, room_list,
};

pub fn meeting_router(pool: Pool) -> Router {
    Router::new()
        .route("/jaxrs/meeting/room/list", get(room_list))
        .route("/jaxrs/meeting/building/list", get(building_list))
        .route("/jaxrs/meeting/openmeeting/list/room", get(openmeeting_list_room))
        .route("/jaxrs/meeting/create", post(create_meeting))
        .route("/jaxrs/meeting/{id}", get(get_meeting))
        .route("/jaxrs/meeting/list", get(list_meetings))
        .route("/jaxrs/meeting/schedule/days/{days}", get(list_schedule))
        .route("/jaxrs/meeting/{\"meetingId\"}/participant/add", post(add_participant))
        .route("/jaxrs/meeting/{\"meetingId\"}/participant/list", get(list_participants))
        .layer(axum::Extension(pool))
}
