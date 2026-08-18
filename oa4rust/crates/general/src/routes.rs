use crate::{
    area_list,
    is_workday,
    security_clearance_enable,
};
use axum::{
    extract::Extension,
    Router,
};
use deadpool_postgres::Pool;

pub fn general_router(pool: Pool) -> Router {
    Router::new()
        .route("/jaxrs/general/area/list", axum::routing::get(area_list))
        .route(
            "/jaxrs/general/securityclearance/enable",
            axum::routing::get(security_clearance_enable),
        )
        .route(
            "/jaxrs/general/worktime/isworkday/{date}",
            axum::routing::get(is_workday),
        )
        .layer(Extension(pool))
}
