use axum::{
    routing::get,
    Router,
};
use deadpool_postgres::Pool;

pub fn general_assemble_control_routes(pool: Pool) -> Router {
    Router::new()
        .route("/jaxrs/general/assemble/control/status", get(crate::get_general_control_status))
        .route("/jaxrs/general/assemble/control/status/update", axum::routing::post(crate::update_general_control_status))
        .route("/jaxrs/general/assemble/control/permissions/{module}", get(crate::get_module_permissions))
        .layer(axum::Extension(pool))
}
