use axum::Router;

use crate::{
    app_config_list, app_list, category_ext_list, category_list, cms_core_entity_router,
};

pub fn router() -> Router {
    cms_core_entity_router(deadpool_postgres::Pool::builder(
        deadpool_postgres::Manager::new(
            deadpool_postgres::tokio_postgres::Config::new(),
            deadpool_postgres::tokio_postgres::NoTls,
        ),
    )
    .build()
    .unwrap())
}
