use axum::{
    extract::Extension,
    routing::{get, post},
    Router,
};
use deadpool_postgres::Pool;

use crate::{
    create_page, dict_list, delete_page, get_page, list_portal_category,
    portal_get, portal_list, save_page,
};

pub fn router(pool: Pool) -> Router {
    Router::new()
        .route("/jaxrs/portal/{id}", get(portal_get))
        .route("/jaxrs/portal/list", get(portal_list))
        .route("/jaxrs/portalcategory/list", get(list_portal_category))
        .route("/jaxrs/portal/page/{id}", get(get_page))
        .route("/jaxrs/portal/page/create", post(create_page))
        .route("/jaxrs/portal/page/save/{id}", post(save_page))
        .route("/jaxrs/portal/page/delete/{id}", post(delete_page))
        .route("/jaxrs/portal/dict/list", get(dict_list))
        .layer(Extension(pool))
}
