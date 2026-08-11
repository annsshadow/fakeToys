use axum::{
    extract::Extension,
    routing::{get, post},
    Router,
};
use deadpool_postgres::Pool;

use crate::{
    get_control_config, list_control_categories, list_storage_pools, update_control_config,
    list_files, get_file, upload_file, create_file, delete_file,
    create_file_entity, update_file_entity, delete_file_entity,
    file_id_download_stream, attachment_id_download_stream, anonymous_file_id_download_stream,
};

pub fn router(pool: Pool) -> Router {
    Router::new()
        .route("/jaxrs/file/assemble/control/file/list/{folderId}", get(list_files))
        .route("/jaxrs/file/assemble/control/file/{id}", get(get_file))
        .route("/jaxrs/file/assemble/control/file/upload", post(upload_file))
        .route("/jaxrs/file/assemble/control/file/create", post(create_file))
        .route("/jaxrs/file/assemble/control/file/delete/{id}", post(delete_file))
        .route("/jaxrs/file/core/entity/file/create", post(create_file_entity))
        .route("/jaxrs/file/core/entity/file/update/{id}", post(update_file_entity))
        .route("/jaxrs/file/core/entity/file/delete/{id}", post(delete_file_entity))
        .route("/jaxrs/file/{id}/download/stream", get(file_id_download_stream))
        .route("/jaxrs/attachment/download/{attid}/stream", get(attachment_id_download_stream))
        .route("/jaxrs/anonymous/file/{id}/download/stream", get(anonymous_file_id_download_stream))
        .layer(Extension(pool))
}
