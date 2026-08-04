use axum::Router;

use crate::bbs_router;

pub fn router(pool: deadpool_postgres::Pool) -> Router {
    bbs_router(pool)
}
