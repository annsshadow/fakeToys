use axum::{
    routing::get,
    Router,
};

pub fn mind_routes() -> Router {
    Router::new()
        .route("/jaxrs/mind/mind/{id}", get(crate::get_mind_with_id))
        .route("/jaxrs/mind/folder/tree/my", get(crate::list_my_folders))
        .route("/jaxrs/mind/mind/list/{id}/version", get(crate::list_versions_with_mind_id))
        .fallback(axum::routing::any(|| async { "not found" }))
}
