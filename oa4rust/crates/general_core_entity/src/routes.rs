

pub fn router(pool: deadpool_postgres::Pool) -> axum::Router {
    crate::general_core_entity_router(pool)
}

