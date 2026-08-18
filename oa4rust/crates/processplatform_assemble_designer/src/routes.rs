use axum::{
    extract::Extension,
    routing::{get, post},
    Router,
};
use deadpool_postgres::Pool;

use crate::{
    create_flow, get_flow, list_flows, save_flow, delete_flow, preview_flow,
};

pub fn router(pool: Pool) -> Router {
    Router::new()
        .route("/jaxrs/processplatform/assemble/designer/create", post(create_flow))
        .route("/jaxrs/processplatform/assemble/designer/get/{id}", get(get_flow))
        .route("/jaxrs/processplatform/assemble/designer/list/{category}", get(list_flows))
        .route("/jaxrs/processplatform/assemble/designer/save/{id}", post(save_flow))
        .route("/jaxrs/processplatform/assemble/designer/delete/{id}", post(delete_flow))
        .route("/jaxrs/processplatform/assemble/designer/preview/{id}", get(preview_flow))
        .layer(Extension(pool))
}
