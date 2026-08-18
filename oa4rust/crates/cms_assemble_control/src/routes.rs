use axum::{
    extract::Extension,
    routing::{get, post},
    Router,
};
use deadpool_postgres::Pool;

use crate::{
    get_control_config, list_control_sections, update_control_config,
    document_id_view_count, commend_list_paging, queryview_flag_definition, application_id,
};

pub fn router(pool: deadpool_postgres::Pool) -> axum::Router {
    Router::new()
        .route("/jaxrs/cms_assemble_control/get/control/config", get(get_control_config))
        .route("/jaxrs/cms_assemble_control/list/control/sections", get(list_control_sections))
        .route("/jaxrs/cms_assemble_control/update/control/config", get(update_control_config))
        .route("/jaxrs/document/{id}/view/count", post(document_id_view_count))
        .route("/jaxrs/commend/list/paging/{docId}", get(commend_list_paging))
        .route("/jaxrs/queryview/flag/{view_flag}/definition/{query_flag}", get(queryview_flag_definition))
        .route("/jaxrs/application/{id}", get(application_id))
        .layer(Extension(pool))
}

