use axum::Router;

use crate::{
    dict_item_list, dict_list, file_list, general_core_entity_router, invoice_list,
};

pub fn router() -> Router {
    general_core_entity_router(deadpool_postgres::Pool::builder(
        deadpool_postgres::Manager::new(
            deadpool_postgres::tokio_postgres::Config::new(),
            deadpool_postgres::tokio_postgres::NoTls,
        ),
    )
    .build()
    .unwrap())
}
