use axum::{
    extract::Extension,
    routing::{get, post},
    Router,
};
use deadpool_postgres::Pool;

use crate::{
    file_download, file_upload, folder_create, folder_list_with_folder, folder_remove,
    folder_update, permission_set,
};

pub fn router(pool: Pool) -> Router {
    Router::new()
        .route("/api/file/folder/list/{id}", get(folder_list_with_folder))
        .route("/api/file/upload", post(file_upload))
        .route("/api/file/download/{id}", get(file_download))
        .route("/api/file/folder/create", post(folder_create))
        .route("/api/file/folder/update", post(folder_update))
        .route("/api/file/folder/remove", post(folder_remove))
        .route("/api/file/permission/set", post(permission_set))
        .layer(Extension(pool))
}
