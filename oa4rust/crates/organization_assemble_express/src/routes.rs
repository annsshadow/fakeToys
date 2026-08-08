use axum::{
    extract::Extension,
    routing::get,
    Router,
};
use deadpool_postgres::Pool;

use crate::{
    get_express_config,
    list_organization_units,
    sync_organization_data,
    get_express_status,
};

pub fn router(pool: Pool) -> axum::Router {
    Router::new()
        .route("/jaxrs/organization/assemble/express/config/get", get(get_express_config))
        .route("/jaxrs/organization/assemble/express/units/list", get(list_organization_units))
        .route("/jaxrs/organization/assemble/express/data/sync", get(sync_organization_data))
        .route("/jaxrs/organization/assemble/express/status/get", get(get_express_status))
        .layer(Extension(pool))
}
