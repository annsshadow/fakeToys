use axum::{
    extract::Extension,
    routing::{delete, get, post, put},
    Router,
};
use deadpool_postgres::Pool;

use crate::{
    anonymous_file_id_download,
    anonymous_file_id_download_stream,
    attachment2_exist_file_fileMd5,
    attachment2_id,
    attachment2_id_binary_base64,
    attachment2_id_download,
    attachment2_id_download_image_width_width_height_height,
    attachment2_id_download_stream,
    attachment2_id_image_scale_scale_binary_base64,
    attachment2_id_image_width_width_height_height_binary_base64,
    attachment2_id_office_preview_type_type,
    attachment2_list_editor_owner,
    attachment2_list_filter_name,
    attachment2_list_folder_folderId,
    attachment2_list_share_owner,
    attachment2_list_top,
    attachment2_list_type_page_size_size,
    attachment2_user_capacity,
    attachment_id,
    attachment_id_binary_base64,
    attachment_id_download,
    attachment_id_download_stream,
    attachment_id_image_scale_scale_binary_base64,
    attachment_id_image_width_width_height_height_binary_base64,
    attachment_id_update,
    attachment_id_update_callback_callback,
    attachment_list_editor_owner,
    attachment_list_folder_folderId,
    attachment_list_share_owner,
    attachment_list_top,
    complex_folder_id,
    complex_top,
    // plan002 U2：端点全量闭合新增/补注册的 handler
    config_is_file_manager,
    config_system_config,
    create_file,
    create_file_entity,
    delete_file,
    delete_file_entity,
    editor_list,
    file_clean_unused_referencetype_cmsdocument_manage,
    file_copy_attachment_attachmentId_referencetype_referenceType_reference_reference_scale_scale,
    file_id,
    file_id_binary_base64,
    file_id_download,
    file_id_download_stream,
    file_list_id_next_count,
    file_list_id_next_count_all,
    file_list_id_next_count_referencetype_referenceType,
    file_list_id_prev_count,
    file_list_id_prev_count_all,
    file_list_id_prev_count_referencetype_referenceType,
    file_list_referencetype,
    file_list_referencetype_referenceType_reference_reference,
    file_list_unused_referencetype_cmsdocument_manage,
    file_referencetype_referenceType_reference_reference,
    file_upload_with_url,
    folder2_batch_download,
    folder2_id,
    folder2_id_download,
    folder2_list_id,
    folder2_list_top,
    folder_id,
    folder_list_id,
    folder_list_top,
    get_file,
    list_files,
    recycle_empty,
    recycle_id,
    recycle_id_delete,
    recycle_id_resume,
    recycle_list,
    share_download_share_shareId_file_fileId,
    share_id,
    share_id_password_password,
    share_list,
    share_list_att_share_shareId_folder_folderId,
    share_list_folder_share_shareId_folder_folderId,
    share_list_my,
    share_list_to_me,
    share_share_shareId_file_fileId_folder_folderId,
    share_shield_id,
    u2_attachment2_delete,
    u2_attachment2_list_type_page_size_size,
    u2_attachment2_update,
    u2_attachment_delete,
    u2_attachment_update_content,
    u2_attachment_update_content_callback,
    u2_config_save_system_config,
    u2_file_delete_by_id,
    u2_file_delete_by_reference,
    u2_file_list_reference_types,
    u2_file_upload_callback,
    u2_file_upload_multipart,
    u2_file_upload_octet_stream,
    u2_folder_create,
    u2_folder_delete,
    u2_folder_rename,
    u2_share_create,
    u2_share_delete,
    u2_share_get,
    u2_share_get_with_password,
    u2_share_save_to_folder,
    u2_share_shield,
    update_control_config,
    update_file_entity,
    upload_file,
};

pub fn router(pool: Pool) -> Router {
    Router::new()
        .route("/api/file/assemble/control/file/list/{id}", get(list_files))
        .route("/api/file/assemble/control/file/{id}", get(get_file))
        .route("/api/file/assemble/control/file/upload", post(upload_file))
        .route("/api/file/assemble/control/file/create", post(create_file))
        .route("/api/file/assemble/control/file/delete/{id}", post(delete_file))
        .route("/api/file/core/entity/file/create", post(create_file_entity))
        .route("/api/file/core/entity/file/update/{id}", post(update_file_entity))
        .route("/api/file/core/entity/file/delete/{id}", post(delete_file_entity))
        .route("/api/file/{id}/download/stream", get(file_id_download_stream))
        .route("/api/attachment/download/{attid}/stream", get(attachment_id_download_stream))
        .route("/api/anonymous/file/{id}/download/stream", get(anonymous_file_id_download_stream))
        .route("/api/file/assemble/control/attachment2/{id}/office/preview/type/{type}", get(attachment2_id_office_preview_type_type))
                .route("/api/file/anonymous/file/id/download", get(anonymous_file_id_download))
        .route("/api/file/attachment/list/folder/folderId", get(attachment_list_folder_folderId))
        .route("/api/file/attachment/list/top", get(attachment_list_top))
        .route("/api/file/attachment/id", get(attachment_id))
        .route("/api/file/attachment/id/binary/base64", get(attachment_id_binary_base64))
        .route("/api/file/attachment/id/download", get(attachment_id_download))
        .route("/api/file/attachment/id/image/scale/scale/binary/base64", get(attachment_id_image_scale_scale_binary_base64))
        .route("/api/file/attachment/id/image/width/width/height/height/binary/base64", get(attachment_id_image_width_width_height_height_binary_base64))
        .route("/api/file/attachment2/exist/file/fileMd5", get(attachment2_exist_file_fileMd5))
        .route("/api/file/attachment2/list/filter/name", get(attachment2_list_filter_name))
        .route("/api/file/attachment2/list/folder/folderId", get(attachment2_list_folder_folderId))
        .route("/api/file/attachment2/list/top", get(attachment2_list_top))
        .route("/api/file/attachment2/list/type/page/size/size", get(attachment2_list_type_page_size_size))
        .route("/api/file/attachment2/id", get(attachment2_id))
        .route("/api/file/attachment2/id/binary/base64", get(attachment2_id_binary_base64))
        .route("/api/file/attachment2/id/download", get(attachment2_id_download))
        .route("/api/file/attachment2/id/download/image/width/width/height/height", get(attachment2_id_download_image_width_width_height_height))
        .route("/api/file/attachment2/id/download/stream", get(attachment2_id_download_stream))
        .route("/api/file/attachment2/id/image/scale/scale/binary/base64", get(attachment2_id_image_scale_scale_binary_base64))
        .route("/api/file/attachment2/id/image/width/width/height/height/binary/base64", get(attachment2_id_image_width_width_height_height_binary_base64))
        .route("/api/file/complex/folder/id", get(complex_folder_id))
        .route("/api/file/complex/top", get(complex_top))
        .route("/api/file/editor/list", get(editor_list))
        .route("/api/file/clean/unused/referencetype/cmsdocument/manage", get(file_clean_unused_referencetype_cmsdocument_manage))
        .route("/api/file/copy/attachment/attachmentId/referencetype/referenceType/reference/reference/scale/scale", get(file_copy_attachment_attachmentId_referencetype_referenceType_reference_reference_scale_scale))
        .route("/api/file/list/referencetype", get(file_list_referencetype))
        .route("/api/file/list/referencetype/referenceType/reference/reference", get(file_list_referencetype_referenceType_reference_reference))
        .route("/api/file/list/unused/referencetype/cmsdocument/manage", get(file_list_unused_referencetype_cmsdocument_manage))
        .route("/api/file/list/id/next/count", get(file_list_id_next_count))
        .route("/api/file/list/id/next/count/all", get(file_list_id_next_count_all))
        .route("/api/file/list/id/next/count/referencetype/referenceType", get(file_list_id_next_count_referencetype_referenceType))
        .route("/api/file/list/id/prev/count", get(file_list_id_prev_count))
        .route("/api/file/list/id/prev/count/all", get(file_list_id_prev_count_all))
        .route("/api/file/list/id/prev/count/referencetype/referenceType", get(file_list_id_prev_count_referencetype_referenceType))
        .route("/api/file/referencetype/referenceType/reference/reference", get(file_referencetype_referenceType_reference_reference))
        .route("/api/file/id", get(file_id))
        .route("/api/file/id/binary/base64", get(file_id_binary_base64))
        .route("/api/file/id/download", get(file_id_download))
        .route("/api/file/folder/list/top", get(folder_list_top))
        .route("/api/file/folder/list/id", get(folder_list_id))
        .route("/api/file/folder/id", get(folder_id))
        .route("/api/file/folder2/batch/download", get(folder2_batch_download))
        .route("/api/file/folder2/list/top", get(folder2_list_top))
        .route("/api/file/folder2/list/id", get(folder2_list_id))
        .route("/api/file/folder2/id", get(folder2_id))
        .route("/api/file/folder2/id/download", get(folder2_id_download))
        .route("/api/file/recycle/id", get(recycle_id))
        .route("/api/file/share/download/share/shareId/file/fileId", get(share_download_share_shareId_file_fileId))
        .route("/api/file/share/list/att/share/shareId/folder/folderId", get(share_list_att_share_shareId_folder_folderId))
        .route("/api/file/share/list/folder/share/shareId/folder/folderId", get(share_list_folder_share_shareId_folder_folderId))
        .route("/api/file/share/share/shareId/file/fileId/folder/folderId", get(share_share_shareId_file_fileId_folder_folderId))
        .route("/api/file/share/shield/id", get(share_shield_id))
        .route("/api/file/share/id", get(share_id))
        .route("/api/file/share/id/password/password", get(share_id_password_password))
        .route("/api/attachment2/upload/folder/{folderId}", post(crate::attachment2_upload_folder_folderId))
        .route("/api/attachment/update/{id}", post(crate::attachment_id_update))
        .route("/api/attachment/update/callback/callback/{id}", post(crate::attachment_id_update_callback_callback))
        .route("/api/attachment/upload/folder/{folderId}", post(crate::attachment_upload_folder_folderId))
        .route("/api/attachment/upload/folder/callback/callback/{folderId}", post(crate::attachment_upload_folder_folderId_callback_callback))
        .route("/api/attachment/upload/folder/{folderId}/callback/{callback}", post(crate::attachment_upload_folder_folderId_callback_callback))
        .route("/api/file/upload/referencetype/reference/reference/scale/scale/{referenceType}", post(crate::file_upload_referencetype_referenceType_reference_reference_scale_scale))
        .route("/api/file/upload/referencetype/reference/reference/scale/scale/callback/callback/{referenceType}", post(crate::file_upload_referencetype_referenceType_reference_reference_scale_scale_callback_callback))
        .route("/api/recycle/delete/{id}", post(crate::recycle_id_delete))
        .route("/api/recycle/resume/{id}", post(crate::recycle_id_resume))
        .route("/api/share/list/my2/{shareType}/{fileType}", get(crate::share_list_my2_shareType_fileType))
        .route("/api/share/list/to/me2/{fileType}", get(crate::share_list_to_me2_fileType))
        .route("/api/attachment/update/{id}", put(attachment_id_update))
        .route("/api/attachment/update/callback/callback/{id}", put(attachment_id_update_callback_callback))
        .route("/api/file/assemble/control/file/delete/{id}", delete(delete_file))
        .route("/api/file/core/entity/file/delete/{id}", delete(delete_file_entity))
        .route("/api/recycle/delete/{id}", delete(recycle_id_delete))
        .route("/api/file/assemble/control/update/control/config", put(update_control_config))
        .route("/api/file/core/entity/file/update/{id}", put(update_file_entity))
        // ══════ plan002 U2：o2server 端点全量闭合（真实 o2server 路径） ══════
        // anonymous/file 族
        .route("/api/anonymous/file/{id}/download", get(anonymous_file_id_download).post(anonymous_file_id_download))
        .route("/api/anonymous/file/{id}/download/stream", post(anonymous_file_id_download_stream))
        // attachment 族
        .route("/api/attachment/list/top", get(attachment_list_top))
        .route("/api/attachment/list/editor/{owner}", get(attachment_list_editor_owner))
        .route("/api/attachment/list/folder/{folderId}", get(attachment_list_folder_folderId))
        .route("/api/attachment/list/share/{owner}", get(attachment_list_share_owner))
        .route("/api/attachment/{id}", get(attachment_id).put(attachment_id_update).delete(u2_attachment_delete))
        .route("/api/attachment/{id}/binary/base64", get(attachment_id_binary_base64))
        .route("/api/attachment/{id}/download", get(attachment_id_download).post(attachment_id_download))
        .route("/api/attachment/{id}/download/stream", get(attachment_id_download_stream).post(attachment_id_download_stream))
        .route("/api/attachment/{id}/image/scale/{scale}/binary/base64", get(attachment_id_image_scale_scale_binary_base64))
        .route("/api/attachment/{id}/image/width/{width}/height/{height}/binary/base64", get(attachment_id_image_width_width_height_height_binary_base64))
        .route("/api/attachment/{id}/update", put(u2_attachment_update_content))
        .route("/api/attachment/{id}/update/callback/{callback}", post(u2_attachment_update_content_callback))
        // attachment2 族
        .route("/api/attachment2/exist/file/{fileMd5}", get(attachment2_exist_file_fileMd5))
        .route("/api/attachment2/list/top", get(attachment2_list_top))
        .route("/api/attachment2/list/editor/{owner}", get(attachment2_list_editor_owner))
        .route("/api/attachment2/list/filter/{name}", get(attachment2_list_filter_name))
        .route("/api/attachment2/list/folder/{folderId}", get(attachment2_list_folder_folderId))
        .route("/api/attachment2/list/share/{owner}", get(attachment2_list_share_owner))
        .route("/api/attachment2/list/type/{page}/size/{size}", post(u2_attachment2_list_type_page_size_size))
        .route("/api/attachment2/user/capacity", get(attachment2_user_capacity))
        .route("/api/attachment2/{id}", get(attachment2_id).put(u2_attachment2_update).delete(u2_attachment2_delete))
        .route("/api/attachment2/{id}/binary/base64", get(attachment2_id_binary_base64))
        .route("/api/attachment2/{id}/download", get(attachment2_id_download).post(attachment2_id_download))
        .route("/api/attachment2/{id}/download/image/width/{width}/height/{height}", get(attachment2_id_download_image_width_width_height_height))
        .route("/api/attachment2/{id}/download/stream", get(attachment2_id_download_stream).post(attachment2_id_download_stream))
        .route("/api/attachment2/{id}/image/scale/{scale}/binary/base64", get(attachment2_id_image_scale_scale_binary_base64))
        .route("/api/attachment2/{id}/image/width/{width}/height/{height}/binary/base64", get(attachment2_id_image_width_width_height_height_binary_base64))
        .route("/api/attachment2/{id}/office/preview/type/{type}", get(attachment2_id_office_preview_type_type))
        // complex / config / editor 族
        .route("/api/complex/folder/{id}", get(complex_folder_id))
        .route("/api/complex/top", get(complex_top))
        .route("/api/config", post(u2_config_save_system_config))
        .route("/api/config/is/file/manager", get(config_is_file_manager))
        .route("/api/config/system/config", get(config_system_config))
        .route("/api/editor/list", get(editor_list))
        // folder / folder2 族（folder2 CRUD 复用 folder 实现，同一 FILE_FOLDER 表）
        .route("/api/folder", post(u2_folder_create))
        .route("/api/folder/list/top", get(folder_list_top))
        .route("/api/folder/list/{id}", get(folder_list_id))
        .route("/api/folder/{id}", get(folder_id).put(u2_folder_rename).delete(u2_folder_delete))
        .route("/api/folder2", post(u2_folder_create))
        .route("/api/folder2/batch/download", get(folder2_batch_download))
        .route("/api/folder2/list/top", get(folder2_list_top))
        .route("/api/folder2/list/{id}", get(folder2_list_id))
        .route("/api/folder2/{id}", get(folder2_id).put(u2_folder_rename).delete(u2_folder_delete))
        .route("/api/folder2/{id}/download", get(folder2_id_download))
        // recycle 族
        .route("/api/recycle/empty", delete(recycle_empty))
        .route("/api/recycle/list", get(recycle_list))
        .route("/api/recycle/{id}", get(recycle_id))
        .route("/api/recycle/{id}/delete", delete(recycle_id_delete))
        .route("/api/recycle/{id}/resume", post(recycle_id_resume))
        // share 族
        .route("/api/share", post(u2_share_create))
        .route("/api/share/download/share/{shareId}/file/{fileId}", get(share_download_share_shareId_file_fileId))
        .route("/api/share/list", get(share_list))
        .route("/api/share/list/my", get(share_list_my))
        .route("/api/share/list/to/me", get(share_list_to_me))
        .route("/api/share/list/att/share/{shareId}/folder/{folderId}", get(share_list_att_share_shareId_folder_folderId))
        .route("/api/share/list/folder/share/{shareId}/folder/{folderId}", get(share_list_folder_share_shareId_folder_folderId))
        .route("/api/share/share/{shareId}/file/{fileId}/folder/{folderId}", post(u2_share_save_to_folder))
        .route("/api/share/shield/{id}", get(u2_share_shield))
        .route("/api/share/{id}", get(u2_share_get).delete(u2_share_delete))
        .route("/api/share/{id}/password/{password}", get(u2_share_get_with_password))
        // file 族（裸 GET /api/file/{id} 已被 cms_assemble_control 占用 —— 裁决见 lib.rs；
        // 本族统一挂在模块前缀 /api/file/assemble/control/file 下）
        .route("/api/file/assemble/control/file/list/referencetype", get(u2_file_list_reference_types))
        .route("/api/file/assemble/control/file/list/referencetype/{referenceType}/reference/{reference}", get(file_list_referencetype_referenceType_reference_reference))
        .route("/api/file/assemble/control/file/list/unused/referencetype/cmsdocument/manage", get(file_list_unused_referencetype_cmsdocument_manage))
        .route("/api/file/assemble/control/file/list/{id}/next/{count}", get(file_list_id_next_count))
        .route("/api/file/assemble/control/file/list/{id}/next/{count}/all", get(file_list_id_next_count_all))
        .route("/api/file/assemble/control/file/list/{id}/next/{count}/referencetype/{referenceType}", get(file_list_id_next_count_referencetype_referenceType))
        .route("/api/file/assemble/control/file/list/{id}/prev/{count}", get(file_list_id_prev_count))
        .route("/api/file/assemble/control/file/list/{id}/prev/{count}/all", get(file_list_id_prev_count_all))
        .route("/api/file/assemble/control/file/list/{id}/prev/{count}/referencetype/{referenceType}", get(file_list_id_prev_count_referencetype_referenceType))
        .route("/api/file/assemble/control/file/clean/unused/referencetype/cmsdocument/manage", delete(file_clean_unused_referencetype_cmsdocument_manage))
        .route("/api/file/assemble/control/file/copy/attachment/{attachmentId}/referencetype/{referenceType}/reference/{reference}/scale/{scale}", get(file_copy_attachment_attachmentId_referencetype_referenceType_reference_reference_scale_scale))
        .route("/api/file/assemble/control/file/referencetype/{referenceType}/reference/{reference}", delete(u2_file_delete_by_reference))
        .route("/api/file/assemble/control/file/upload/referencetype/{referenceType}/reference/{reference}/scale/{scale}", post(u2_file_upload_octet_stream).put(u2_file_upload_multipart))
        .route("/api/file/assemble/control/file/upload/referencetype/{referenceType}/reference/{reference}/scale/{scale}/callback/{callback}", post(u2_file_upload_callback))
        .route("/api/file/assemble/control/file/upload/with/url", post(file_upload_with_url))
        .route("/api/file/assemble/control/file/{id}/binary/base64", get(file_id_binary_base64))
        .route("/api/file/assemble/control/file/{id}/download", get(file_id_download).post(file_id_download))
        .route("/api/file/assemble/control/file/{id}/download/stream", post(file_id_download_stream))
        .route("/api/file/assemble/control/file/{id}", delete(u2_file_delete_by_id))
.layer(Extension(pool))
}
