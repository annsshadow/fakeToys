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

pub fn router(pool: deadpool_postgres::Pool) -> axum::Router {
    Router::new()
        .route("/jaxrs/component_assemble_control/get/component", get(get_component))
        .route("/jaxrs/component_assemble_control/create/component", get(create_component))
        .route("/jaxrs/component_assemble_control/list/components", get(list_components))
        .route("/jaxrs/component_assemble_control/save/component", get(save_component))
        .route("/jaxrs/component_assemble_control/delete/component", get(delete_component))
        .route("/jaxrs/component_assemble_control/get/control/config", get(get_control_config))
        .route("/jaxrs/component_assemble_control/list/control/categories", get(list_control_categories))
        .route("/jaxrs/component_assemble_control/update/control/config", get(update_control_config))
        .layer(Extension(pool))
}

