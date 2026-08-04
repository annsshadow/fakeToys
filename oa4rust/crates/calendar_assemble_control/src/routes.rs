use axum::{
    extract::Extension,
    routing::get,
    Router,
};
use deadpool_postgres::Pool;

use crate::{get_control_config, list_control_calendars, update_control_config};

pub fn router(pool: Pool) -> Router {
    Router::new()
        .route("/jaxrs/calendar/assemble/control/config/get", get(get_control_config))
        .route("/jaxrs/calendar/assemble/control/calendars", get(list_control_calendars))
        .route("/jaxrs/calendar/assemble/control/config/update", get(update_control_config))
        .layer(Extension(pool))
}
