use axum::{
    extract::Extension,
    routing::get,
    Router,
};
use deadpool_postgres::Pool;

use crate::{calendar_assemble_control_router, get_control_config, list_control_calendars, update_control_config};

pub fn router(pool: deadpool_postgres::Pool) -> axum::Router {
    Router::new()
        .route("/jaxrs/calendar_assemble_control/get/control/config", get(get_control_config))
        .route("/jaxrs/calendar_assemble_control/list/control/calendars", get(list_control_calendars))
        .route("/jaxrs/calendar_assemble_control/update/control/config", get(update_control_config))
        .layer(Extension(pool))
}

