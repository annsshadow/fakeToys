use axum::{
    routing::get,
    Router,
};
use deadpool_postgres::Pool;

use crate::{
    state_organization, state_running, state_summary,
};

pub fn process_bam_router(pool: Pool) -> Router {
    Router::new()
        .route("/jaxrs/process/state/summary", get(state_summary))
        .route("/jaxrs/process/state/running", get(state_running))
        .route("/jaxrs/process/state/organization", get(state_organization))
        .layer(axum::Extension(pool))
}
