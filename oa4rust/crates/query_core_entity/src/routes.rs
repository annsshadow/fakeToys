use axum::Router;

use crate::{
    import_model_list, import_record_list, item_access_list, item_list, query_core_entity_router,
    view_list,
};

pub fn router() -> Router {
    query_core_entity_router(deadpool_postgres::Pool::builder(
        deadpool_postgres::Manager::new(
            deadpool_postgres::tokio_postgres::Config::new(),
            deadpool_postgres::tokio_postgres::NoTls,
        ),
    )
    .build()
    .unwrap())
}
