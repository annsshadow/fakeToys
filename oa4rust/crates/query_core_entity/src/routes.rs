use axum::Router;

use crate::{
    import_model_list, import_record_list, item_access_list, item_list, query_core_entity_router,
    view_list,
};

pub fn router(pool: deadpool_postgres::Pool) -> axum::Router {
    crate::query_core_entity_router(pool)
}

