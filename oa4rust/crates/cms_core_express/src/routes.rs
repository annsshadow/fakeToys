use axum::Router;

use crate::{
    cms_core_express_router,
};

pub fn router() -> Router {
    cms_core_express_router(deadpool_postgres::Pool::builder(
        deadpool_postgres::Manager::new(
            deadpool_postgres::tokio_postgres::Config::new(),
            deadpool_postgres::tokio_postgres::NoTls,
        ),
    )
    .build()
    .unwrap())
}
