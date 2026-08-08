use axum::{
    extract::Extension,
    routing::{get, post},
    Router,
};
use deadpool_postgres::Pool;

use crate::{get_express_info, list_express_companies, subscribe_express};

pub fn router(pool: Pool) -> Router {
    Router::new()
        .route("/jaxrs/express/query", get(get_express_info))
        .route("/jaxrs/express/companies", get(list_express_companies))
        .route("/jaxrs/express/subscribe", post(subscribe_express))
        .layer(Extension(pool))
}
