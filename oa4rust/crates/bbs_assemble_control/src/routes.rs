use axum::{
    extract::Extension,
    routing::get,
    Router,
};
use deadpool_postgres::Pool;

use crate::{get_control_config, list_control_sections, update_control_config};

pub fn router(pool: Pool) -> Router {
    Router::new()
        .route("/jaxrs/bbs/assemble/control/config/get", get(get_control_config))
        .route("/jaxrs/bbs/assemble/control/sections", get(list_control_sections))
        .route("/jaxrs/bbs/assemble/control/config/update", get(update_control_config))
        .layer(Extension(pool))
}
