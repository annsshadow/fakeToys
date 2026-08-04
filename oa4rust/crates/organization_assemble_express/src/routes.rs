use axum::{
    extract::Extension,
    routing::get,
    Router,
};
use deadpool_postgres::Pool;

pub fn router(pool: Pool) -> Router {
    Router::new()
        .route("/jaxrs/organization/assemble/express/config/get", get(super::get_express_config))
        .route("/jaxrs/organization/assemble/express/units", get(super::list_organization_units))
        .route("/jaxrs/organization/assemble/express/sync", get(super::sync_organization_data))
        .route("/jaxrs/organization/assemble/express/status", get(super::get_express_status))
        .layer(Extension(pool))
}
