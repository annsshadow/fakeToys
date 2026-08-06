use axum::{
    routing::get, routing::post,
    Router,
};
use deadpool_postgres::Pool;

use crate::{list_folders, get_folder, save_folder};

pub fn mind_assemble_control_routes(pool: Pool) -> Router {
    Router::new()
        .route("/jaxrs/mind/assemble/control/config", get(crate::get_control_config))
        .route("/jaxrs/mind/assemble/control/config/update", axum::routing::post(crate::update_control_config))
        .route("/jaxrs/mind/assemble/control/folder/tree/my", get(list_folders))
        .route("/jaxrs/mind/assemble/control/folder/{id}", get(get_folder))
        .route("/jaxrs/mind/assemble/control/folder/save", post(save_folder))
        .layer(axum::Extension(pool))
}
