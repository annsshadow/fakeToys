use axum::{
    extract::Extension,
    routing::get,
    Router,
};
use deadpool_postgres::Pool;

use crate::{get_control_config, list_control_apps, update_control_config};

pub fn router(pool: Pool) -> Router {
    Router::new()
        .route("/jaxrs/jpush/assemble/control/config/get", get(get_control_config))
        .route("/jaxrs/jpush/assemble/control/apps", get(list_control_apps))
        .route("/jaxrs/jpush/assemble/control/config/update", get(update_control_config))
        .layer(Extension(pool))
}
