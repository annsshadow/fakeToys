use axum::{
    extract::Extension,
    routing::{get, post},
    Router,
};
use deadpool_postgres::Pool;

use crate::{get_control_config, list_control_sections, update_control_config};

pub fn router(pool: deadpool_postgres::Pool) -> axum::Router {
    Router::new()
        .route("/jaxrs/cms_assemble_control/get/control/config", get(get_control_config))
        .route("/jaxrs/cms_assemble_control/list/control/sections", get(list_control_sections))
        .route("/jaxrs/cms_assemble_control/update/control/config", get(update_control_config))
        .layer(Extension(pool))
}

