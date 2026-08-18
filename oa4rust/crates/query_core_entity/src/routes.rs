use crate::{
    import_list, item_list, query_core_entity_router, view_create, view_get, view_list,
};

pub fn router(pool: deadpool_postgres::Pool) -> axum::Router {
    crate::query_core_entity_router(pool)
}
