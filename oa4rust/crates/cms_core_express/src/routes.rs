pub fn router(pool: deadpool_postgres::Pool) -> axum::Router {
    crate::cms_core_express_router(pool)
}
