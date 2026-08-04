use axum::Router;

use crate::{
    bind_list, custom_list, definition_list, group_list, identity_list,
    organization_core_entity_router, person_list,
};

pub fn router() -> Router {
    organization_core_entity_router(deadpool_postgres::Pool::builder(
        deadpool_postgres::Manager::new(
            deadpool_postgres::tokio_postgres::Config::new(),
            deadpool_postgres::tokio_postgres::NoTls,
        ),
    )
    .build()
    .unwrap())
}
