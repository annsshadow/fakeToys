use axum::{
    routing::get,
    Router,
};
use deadpool_postgres::Pool;

pub fn mind_assemble_control_routes(pool: Pool) -> Router {
    Router::new()
        .route("/jaxrs/mind/assemble/control/config", get(crate::get_control_config))
        .route("/jaxrs/mind/assemble/control/config/update", axum::routing::post(crate::update_control_config))
        .layer(axum::Extension(pool))
}
