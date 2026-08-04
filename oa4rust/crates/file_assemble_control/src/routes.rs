use axum::{
    extract::Extension,
    routing::get,
    Router,
};
use deadpool_postgres::Pool;

pub fn router(pool: Pool) -> Router {
    Router::new()
        .route("/jaxrs/file/assemble/control/config/get", get(super::get_control_config))
        .route("/jaxrs/file/assemble/control/storage/pools", get(super::list_storage_pools))
        .route("/jaxrs/file/assemble/control/config/update", get(super::update_control_config))
        .route("/jaxrs/file/assemble/control/categories", get(super::list_control_categories))
        .layer(Extension(pool))
}
