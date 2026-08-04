use axum::{
    routing::{delete, get, post},
    Router,
};
use deadpool_postgres::Pool;

pub fn meeting_assemble_control_routes(pool: Pool) -> Router {
    Router::new()
        .route("/jaxrs/meeting/assemble/control/list/{meetingId}", get(crate::list_meeting_controls))
        .route("/jaxrs/meeting/assemble/control/create", post(crate::create_meeting_control))
        .route("/jaxrs/meeting/assemble/control/delete/{id}", delete(crate::delete_meeting_control))
        .layer(axum::Extension(pool))
}
