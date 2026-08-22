use axum::{
    extract::Extension,
    routing::{get, post},
    Router,
};
use deadpool_postgres::Pool;

use crate::{
    complex_top, file_download, file_upload, folder_create, folder_list_top,
    folder_list_with_folder, folder_remove, folder_update, permission_set,
};

pub fn router(pool: Pool) -> Router {
    Router::new()
        .route("/jaxrs/file/folder/list/{id}", get(folder_list_with_folder))
        .route("/jaxrs/file/upload", post(file_upload))
        .route("/jaxrs/file/download/{id}", get(file_download))
        .route("/jaxrs/file/folder/create", post(folder_create))
        .route("/jaxrs/file/folder/update", post(folder_update))
        .route("/jaxrs/file/folder/remove", post(folder_remove))
        .route("/jaxrs/file/permission/set", post(permission_set))
        .layer(Extension(pool))
}
