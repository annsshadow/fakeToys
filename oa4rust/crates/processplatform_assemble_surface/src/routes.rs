use axum::{
    extract::Extension,
    routing::{get, post},
    Router,
};
use deadpool_postgres::Pool;

use crate::{
    create_surface, get_surface, list_surfaces, preview_surface,
    publish_surface, save_surface, delete_surface,
};

pub fn router(pool: Pool) -> Router {
    Router::new()
        .route("/jaxrs/processplatform/assemble/surface/get/{id}", get(get_surface))
        .route("/jaxrs/processplatform/assemble/surface/create", post(create_surface))
        .route("/jaxrs/processplatform/assemble/surface/list/{category}", get(list_surfaces))
        .route("/jaxrs/processplatform/assemble/surface/save/{id}", post(save_surface))
        .route("/jaxrs/processplatform/assemble/surface/preview/{id}", get(preview_surface))
        .route("/jaxrs/processplatform/assemble/surface/publish/{id}", post(publish_surface))
        .route("/jaxrs/processplatform/assemble/surface/delete/{id}", post(delete_surface))
        .layer(Extension(pool))
}
