use axum::{
    routing::get,
    Router,
};
use deadpool_postgres::Pool;

use crate::{building_list, openmeeting_list_room, room_list};

pub fn meeting_router(pool: Pool) -> Router {
    Router::new()
        .route("/jaxrs/meeting/room/list", get(room_list))
        .route("/jaxrs/meeting/building/list", get(building_list))
        .route("/jaxrs/meeting/openmeeting/list/room", get(openmeeting_list_room))
        .layer(axum::Extension(pool))
}
