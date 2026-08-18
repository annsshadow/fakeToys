use axum::{
    routing::get, routing::post, routing::delete,
    Router,
};

pub fn mind_routes() -> Router {
    Router::new()
        .route("/jaxrs/mind/mind/{id}", get(crate::get_mind_with_id))
        .route("/jaxrs/mind/mind", post(crate::create_mind))
        .route("/jaxrs/mind/mind/{id}", post(crate::update_mind))
        .route("/jaxrs/mind/mind/{id}", delete(crate::delete_mind))
        .route("/jaxrs/mind/folder/tree/my", get(crate::list_my_folders))
        .route("/jaxrs/mind/folder", post(crate::create_folder))
        .route("/jaxrs/mind/folder/{id}", post(crate::update_folder))
        .route("/jaxrs/mind/folder/{id}", delete(crate::delete_folder))
        .route("/jaxrs/mind/mind/list/{id}/version", get(crate::list_versions_with_mind_id))
        .route("/jaxrs/mind/version", post(crate::create_version))
        .fallback(axum::routing::any(|| async { "not found" }))
}
