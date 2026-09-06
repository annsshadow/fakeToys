

pub fn router(pool: deadpool_postgres::Pool) -> axum::Router {
    crate::component_core_entity_router(pool)
}

