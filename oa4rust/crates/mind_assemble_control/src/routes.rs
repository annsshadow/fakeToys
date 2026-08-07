use axum::{
    extract::Extension,
    routing::{get, post},
    Router,
};
use deadpool_postgres::Pool;

use crate::{
    get_control_config, get_folder, list_folders, save_folder, update_control_config, update_folder,
    folder_id_force,
    folder_move_folderId,
};

pub fn mind_assemble_control_routes(pool: Pool) -> Router {
    Router::new()
        .route("/jaxrs/mind/assemble/control/config", get(get_control_config))
        .route("/jaxrs/mind/assemble/control/config/update", post(update_control_config))
        .route("/jaxrs/mind/assemble/control/folder/tree/my", get(list_folders))
        .route("/jaxrs/mind/assemble/control/folder/{id}", get(get_folder))
        .route("/jaxrs/mind/assemble/control/folder/save", post(save_folder))
        .route("/jaxrs/mind/assemble/control/folder/{id}/update", post(update_folder))
        .route("/jaxrs/mind/assemble/control/folder/move/{folderId}", post(folder_move_folderId))
        .route("/jaxrs/mind/assemble/control/folder/{id}/force", post(folder_id_force))
        .layer(axum::Extension(pool))
}
