use axum::Router;

use crate::file_router;

pub fn router(pool: deadpool_postgres::Pool) -> Router {
    file_router(pool)
}
