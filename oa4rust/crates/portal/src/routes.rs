use axum::{
    extract::Extension,
    routing::{get, post},
    Router,
};
use deadpool_postgres::Pool;

use crate::{
    delete_page, dict_list, get_page, list_portal_category, portal_id, portal_list, save_page,
};

pub fn router(pool: Pool) -> Router {
    Router::new()
        .route("/api/portal/{id}", get(portal_id))
        .route("/api/portal/list", get(portal_list))
        .route("/api/portalcategory/list", get(list_portal_category))
        .route("/api/portal/page/{id}", get(get_page))
        .route("/api/portal/page/save/{id}", post(save_page))
        .route("/api/portal/page/delete/{id}", post(delete_page))
        .route("/api/portal/dict/list", get(dict_list))
        .layer(Extension(pool))
}
