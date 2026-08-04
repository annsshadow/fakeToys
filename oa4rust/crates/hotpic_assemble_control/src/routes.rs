use axum::{
    extract::Extension,
    routing::get,
    Router,
};
use deadpool_postgres::Pool;

pub fn router(pool: Pool) -> Router {
    Router::new()
        .route("/jaxrs/hotpic/assemble/control/config/get", get(super::get_control_config))
        .route("/jaxrs/hotpic/assemble/control/panels", get(super::list_control_panels))
        .route("/jaxrs/hotpic/assemble/control/config/update", get(super::update_control_config))
        .route("/jaxrs/hotpic/assemble/control/applications", get(super::list_control_applications))
        .layer(Extension(pool))
}
