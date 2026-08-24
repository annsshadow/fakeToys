use axum::{
    routing::{delete, get, post, put},
    Router,
};
use deadpool_postgres::Pool;

use crate::{
    get_control_config, get_folder, list_folders, save_folder, update_folder,
    folder_id_force, folder_move_folderId, update_control_config,
};

pub fn mind_assemble_control_routes(pool: Pool) -> Router {
    Router::new()
        // ── 控制面配置（既有 2 条，非 jaxrs 清单路径）──
        .route("/jaxrs/mind/assemble/control/config", get(get_control_config))
        .route("/jaxrs/mind/assemble/control/config/update", post(update_control_config))
        // ── 既有 folder 操作（方法对齐 Java）──
        .route("/jaxrs/mind/assemble/control/folder/tree/my", get(list_folders))
        .route("/jaxrs/mind/assemble/control/folder/{id}", get(get_folder))
        .route("/jaxrs/mind/assemble/control/folder/save", post(save_folder))
        .route("/jaxrs/mind/assemble/control/folder/{id}/update", post(update_folder))
        .route("/jaxrs/mind/assemble/control/folder/move/{folderId}", put(folder_move_folderId))
        .route("/jaxrs/mind/assemble/control/folder/{id}/force", delete(folder_id_force))
        // ════════ plan002 U2：x_mind_assemble_control jaxrs 全集 23 条对齐 ═════════
        // 注册顺序须在 Extension(pool) 层之前，确保处理器能取到连接池。
        .route("/jaxrs/mind/assemble/control/folder/{id}", delete(crate::u2::folder_delete))
        .route("/jaxrs/mind/assemble/control/mind/filter/list/{id}/next/{page}", put(crate::u2::mind_filter_list))
        .route("/jaxrs/mind/assemble/control/mind/filter/recived/{id}/next/{page}", put(crate::u2::mind_filter_received))
        .route("/jaxrs/mind/assemble/control/mind/filter/recycle/{id}/next/{page}", put(crate::u2::mind_filter_recycle))
        .route("/jaxrs/mind/assemble/control/mind/filter/shared/{id}/next/{page}", put(crate::u2::mind_filter_shared))
        .route("/jaxrs/mind/assemble/control/mind/list/{id}/shareRecords", get(crate::u2::mind_share_records))
        .route("/jaxrs/mind/assemble/control/mind/list/{id}/version", get(crate::u2::mind_version_list))
        .route("/jaxrs/mind/assemble/control/mind/recycle/{id}", delete(crate::u2::mind_recycle))
        .route("/jaxrs/mind/assemble/control/mind/restore/{id}", get(crate::u2::mind_restore))
        .route("/jaxrs/mind/assemble/control/mind/save", post(crate::u2::mind_save))
        .route("/jaxrs/mind/assemble/control/mind/share/{id}", put(crate::u2::mind_share))
        .route("/jaxrs/mind/assemble/control/mind/share/{id}/cancel", put(crate::u2::mind_share_cancel))
        .route("/jaxrs/mind/assemble/control/mind/version/{id}", get(crate::u2::mind_version_get))
        .route("/jaxrs/mind/assemble/control/mind/view/{id}", get(crate::u2::mind_view))
        .route("/jaxrs/mind/assemble/control/mind/{id}", get(crate::u2::mind_get))
        .route("/jaxrs/mind/assemble/control/mind/{id}/destorymind", delete(crate::u2::mind_destroy))
        .route("/jaxrs/mind/assemble/control/mind/{id}/destoryrecycle", delete(crate::u2::mind_destroy_recycle))
        .route("/jaxrs/mind/assemble/control/mind/{id}/icon", get(crate::u2::mind_icon_get))
        .route("/jaxrs/mind/assemble/control/mind/{id}/icon/size/{size}", post(crate::u2::mind_icon_set))
        .layer(axum::Extension(pool))
}
