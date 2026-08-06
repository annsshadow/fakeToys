use axum::{
    extract::Extension,
    routing::get, routing::post,
    Router,
};
use deadpool_postgres::Pool;

use crate::{
    list_hotpics, get_hotpic, create_hotpic, save_hotpic, delete_hotpic,
    get_control_config, list_control_panels, update_control_config, list_control_applications,
};

pub fn router(pool: Pool) -> Router {
    Router::new()
        .route("/jaxrs/hotpic/assemble/control/config/get", get(get_control_config))
        .route("/jaxrs/hotpic/assemble/control/panels", get(list_control_panels))
        .route("/jaxrs/hotpic/assemble/control/config/update", get(update_control_config))
        .route("/jaxrs/hotpic/assemble/control/applications", get(list_control_applications))
        .route("/jaxrs/hotpic/assemble/control/hotpic/list", get(list_hotpics))
        .route("/jaxrs/hotpic/assemble/control/hotpic/get/{id}", get(get_hotpic))
        .route("/jaxrs/hotpic/assemble/control/hotpic/create", post(create_hotpic))
        .route("/jaxrs/hotpic/assemble/control/hotpic/save/{id}", post(save_hotpic))
        .route("/jaxrs/hotpic/assemble/control/hotpic/delete/{id}", post(delete_hotpic))
        .layer(Extension(pool))
}
