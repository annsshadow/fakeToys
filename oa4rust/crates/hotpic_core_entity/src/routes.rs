

pub fn router(pool: deadpool_postgres::Pool) -> axum::Router {
    crate::hotpic_core_entity_router(pool)
}

