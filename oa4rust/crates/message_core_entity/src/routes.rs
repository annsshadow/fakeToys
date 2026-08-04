use axum::Router;

use crate::{
    message_core_entity_router,
};

pub fn router() -> Router {
    message_core_entity_router(deadpool_postgres::Pool::builder(
        deadpool_postgres::Manager::new(
            deadpool_postgres::tokio_postgres::Config::new(),
            deadpool_postgres::tokio_postgres::NoTls,
        ),
    )
    .build()
    .unwrap())
}
