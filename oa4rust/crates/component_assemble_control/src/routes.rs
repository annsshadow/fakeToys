use axum::{
    extract::Extension,
    routing::get, routing::post,
    Router,
};
use deadpool_postgres::Pool;

use crate::{
    get_component, create_component, list_components, save_component, delete_component,
    get_control_config, list_control_categories, update_control_config,
};

pub fn router(pool: Pool) -> Router {
    Router::new()
        .route("/jaxrs/component/assemble/control/config/get", get(get_control_config))
        .route("/jaxrs/component/assemble/control/categories", get(list_control_categories))
        .route("/jaxrs/component/assemble/control/config/update", get(update_control_config))
        .route("/jaxrs/component/assemble/control/component/list", get(list_components))
        .route("/jaxrs/component/assemble/control/component/get/{id}", get(get_component))
        .route("/jaxrs/component/assemble/control/component/create", post(create_component))
        .route("/jaxrs/component/assemble/control/component/save/{id}", post(save_component))
        .route("/jaxrs/component/assemble/control/component/delete/{id}", post(delete_component))
        .layer(Extension(pool))
}
