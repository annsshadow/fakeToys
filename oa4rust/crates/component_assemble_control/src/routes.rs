use axum::{
    extract::Extension,
    routing::{delete, get, post, put},
    Router,
};

use crate::{
    component_delete_all, create_component, delete_component, get_component, get_control_config,
    list_components, list_control_categories, save_component, status_list, update_control_config,
};

pub fn router(pool: deadpool_postgres::Pool) -> axum::Router {
    Router::new()
        .route(
            "/jaxrs/component_assemble_control/get/component/{id}",
            get(get_component),
        )
        .route(
            "/jaxrs/component_assemble_control/create/component",
            get(create_component),
        )
        .route(
            "/jaxrs/component_assemble_control/list/components",
            get(list_components),
        )
        .route(
            "/jaxrs/component_assemble_control/save/component",
            get(save_component),
        )
        .route(
            "/jaxrs/component_assemble_control/delete/component",
            get(delete_component),
        )
        .route(
            "/jaxrs/component_assemble_control/get/control/config",
            get(get_control_config),
        )
        .route(
            "/jaxrs/component_assemble_control/list/control/categories",
            get(list_control_categories),
        )
        .route(
            "/jaxrs/component_assemble_control/update/control/config",
            get(update_control_config),
        )
        .route(
            "/jaxrs/component/assemble/control/component/delete/all",
            post(component_delete_all),
        )
        .route(
            "/jaxrs/component/assemble/control/status/list",
            get(status_list),
        )
        // ---- plan002 U2 gaps: verb variants reusing existing handlers ----
        .route(
            "/jaxrs/component_assemble_control/component",
            post(create_component),
        )
        .route(
            "/jaxrs/component/assemble/control/component/delete/all",
            delete(component_delete_all),
        )
        .route(
            "/jaxrs/component_assemble_control/component/{id}",
            delete(delete_component),
        )
        .route(
            "/jaxrs/component_assemble_control/component/{id}",
            put(save_component),
        )
        .layer(Extension(pool))
}
