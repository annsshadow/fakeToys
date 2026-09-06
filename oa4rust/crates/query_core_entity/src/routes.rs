
pub fn router(pool: deadpool_postgres::Pool) -> axum::Router {
    crate::query_core_entity_router(pool)
}
