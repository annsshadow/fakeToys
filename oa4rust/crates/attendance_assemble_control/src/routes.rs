use axum::{
    routing::{get, post},
    Router,
};
use deadpool_postgres::Pool;

pub fn attendance_assemble_control_routes(pool: Pool) -> Router {
    Router::new()
        .route("/jaxrs/attendance/assemble/control/rule/list", get(crate::list_control_rules))
        .route("/jaxrs/attendance/assemble/control/rule/{id}/toggle", post(crate::toggle_control_rule))
        .layer(axum::Extension(pool))
}
