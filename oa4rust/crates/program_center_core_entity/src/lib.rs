use axum::Router;
use deadpool_postgres::Pool;

pub mod entities;
pub mod handlers;
pub mod routes;

pub use routes::router;

const MAX_NAME_LEN: usize = 200;
const MAX_TEXT_LEN: usize = 500;
const MAX_LONG_TEXT_LEN: usize = 2000;

pub async fn program_center_core_entity_router(pool: Pool) -> Router {
    // NOTE: do NOT block_on here — this runs inside the tokio runtime during
    // create_app. Obtain the SeaORM pool asynchronously instead.
    let db = shared::db::create_sea_orm_pool().await.ok();

    let app = Router::new()
        .merge(handlers::application::_router(pool.clone(), db.clone()))
        .merge(handlers::script::_router(pool.clone(), db.clone()))
        .merge(handlers::invoke::_router(pool.clone(), db.clone()))
        .merge(handlers::agent::_router(pool.clone(), db.clone()))
        .merge(handlers::structure::_router(pool, db));

    app
}

#[cfg(test)]
pub fn program_center_mock_router(_db: sea_orm::DatabaseConnection) -> Router {
    use axum::routing::get;
    Router::new()
        .route("/jaxrs/program_center/application/list", get(|| async { "ok" }))
        .route("/jaxrs/program_center/script/list", get(|| async { "ok" }))
        .route("/jaxrs/program_center/invoke/list", get(|| async { "ok" }))
        .route("/jaxrs/program_center/agent/list", get(|| async { "ok" }))
        .route("/jaxrs/program_center/structure/list", get(|| async { "ok" }))
}

#[cfg(test)]
mod tests;
#[cfg(test)]
mod tests_generated;

#[cfg(test)]
pub fn test_router(_pool: deadpool_postgres::Pool) -> axum::Router {
    crate::program_center_mock_router(sea_orm::DatabaseConnection::default())
}

