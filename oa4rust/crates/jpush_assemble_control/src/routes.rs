use axum::{
    extract::Extension,
    routing::get, routing::post,
    Router,
};
use deadpool_postgres::Pool;

use crate::{
    list_jpushs, get_jpush, create_jpush, save_jpush, delete_jpush,
    get_control_config, list_control_apps, update_control_config,
};

pub fn router(pool: Pool) -> Router {
    Router::new()
        .route("/jaxrs/jpush/assemble/control/config/get", get(get_control_config))
        .route("/jaxrs/jpush/assemble/control/apps", get(list_control_apps))
        .route("/jaxrs/jpush/assemble/control/config/update", get(update_control_config))
        .route("/jaxrs/jpush/assemble/control/message/list", get(list_jpushs))
        .route("/jaxrs/jpush/assemble/control/message/get/{id}", get(get_jpush))
        .route("/jaxrs/jpush/assemble/control/message/send", post(create_jpush))
        .route("/jaxrs/jpush/assemble/control/message/save/{id}", post(save_jpush))
        .route("/jaxrs/jpush/assemble/control/message/delete/{id}", post(delete_jpush))
        .layer(Extension(pool))
}
