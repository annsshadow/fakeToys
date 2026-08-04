use axum::Router;

use crate::{
    organization_core_express_router,
};

pub fn router() -> Router {
    organization_core_express_router(deadpool_postgres::Pool::builder(
        deadpool_postgres::Manager::new(
            deadpool_postgres::tokio_postgres::Config::new(),
            deadpool_postgres::tokio_postgres::NoTls,
        ),
    )
    .build()
    .unwrap())
}
