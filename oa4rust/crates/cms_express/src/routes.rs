use crate::{template_form_list, uuid_random, view_list_all};
use axum::{routing::get, Router};
use deadpool_postgres::Pool;

pub fn cms_express_router() -> Router {
    Router::new()
        .route("/jaxrs/cms/uuid/random", get(uuid_random))
        .route("/jaxrs/cms/templateform/list", get(template_form_list))
        .route("/jaxrs/cms/view/list/all", get(view_list_all))
}

pub fn router(pool: Pool) -> axum::Router {
    cms_express_router().layer(axum::extract::Extension(pool))
}
