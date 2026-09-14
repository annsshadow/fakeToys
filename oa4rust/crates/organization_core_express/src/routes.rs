pub fn router(pool: deadpool_postgres::Pool) -> axum::Router {
    crate::organization_core_express_router(pool)
}
