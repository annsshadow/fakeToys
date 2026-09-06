

pub fn router(pool: deadpool_postgres::Pool) -> axum::Router {
    crate::correlation_core_express_router(pool)
}

