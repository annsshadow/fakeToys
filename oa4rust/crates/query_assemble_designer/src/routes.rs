use axum::Router;

use crate::query_assemble_designer_router;

pub fn router(pool: deadpool_postgres::Pool) -> axum::Router {
    query_assemble_designer_router(Some(pool))
}

