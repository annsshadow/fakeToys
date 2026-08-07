use crate::express_router;
use deadpool_postgres::Pool;

pub fn router(pool: Pool) -> axum::Router {
    express_router().layer(axum::extract::Extension(pool))
}
