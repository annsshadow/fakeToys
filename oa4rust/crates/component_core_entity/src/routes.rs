use axum::Router;

use crate::{
    component_core_entity_router,
};

pub fn router() -> Router {
    component_core_entity_router(deadpool_postgres::Pool::builder(
        deadpool_postgres::Manager::new(
            deadpool_postgres::tokio_postgres::Config::new(),
            deadpool_postgres::tokio_postgres::NoTls,
        ),
    )
    .build()
    .unwrap())
}
