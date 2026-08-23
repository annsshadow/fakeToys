use axum::{
    extract::Extension,
    routing::{get, post, put, delete},
    Router,
};
use deadpool_postgres::Pool;

use crate::{
    get_control_config, list_control_categories, list_storage_pools, update_control_config, list_files, get_file, upload_file, create_file, delete_file, create_file_entity,
    update_file_entity, delete_file_entity, file_id_download_stream, attachment_id_download_stream, anonymous_file_id_download_stream, attachment2_id_office_preview_type_type, anonymous_file_id_download, attachment_list_folder_folderId, attachment_list_top, attachment_id,
    attachment_id_binary_base64, attachment_id_download, attachment_id_image_scale_scale_binary_base64, attachment_id_image_width_width_height_height_binary_base64, attachment2_exist_file_fileMd5, attachment2_list_filter_name, attachment2_list_folder_folderId, attachment2_list_top, attachment2_list_type_page_size_size, attachment2_id,
    attachment2_id_binary_base64, attachment2_id_download, attachment2_id_download_image_width_width_height_height, attachment2_id_download_stream, attachment2_id_image_scale_scale_binary_base64, attachment2_id_image_width_width_height_height_binary_base64, complex_folder_id, complex_top, editor_list, file_clean_unused_referencetype_cmsdocument_manage,
    file_copy_attachment_attachmentId_referencetype_referenceType_reference_reference_scale_scale, file_list_referencetype, file_list_referencetype_referenceType_reference_reference, file_list_unused_referencetype_cmsdocument_manage, file_list_id_next_count, file_list_id_next_count_all, file_list_id_next_count_referencetype_referenceType, file_list_id_prev_count, file_list_id_prev_count_all, file_list_id_prev_count_referencetype_referenceType,
    file_referencetype_referenceType_reference_reference, file_id, file_id_binary_base64, file_id_download, folder_list_top, folder_list_id, folder_id, folder2_batch_download, folder2_list_top, folder2_list_id,
    folder2_id, folder2_id_download, recycle_id, share_download_share_shareId_file_fileId, share_list_att_share_shareId_folder_folderId, share_list_folder_share_shareId_folder_folderId, share_share_shareId_file_fileId_folder_folderId, share_shield_id, share_id, share_id_password_password, attachment_id_update, attachment_id_update_callback_callback, recycle_id_delete,
    // plan002 U2：端点全量闭合新增/补注册的 handler
    config_is_file_manager, config_system_config, attachment_list_editor_owner, attachment_list_share_owner,
    attachment2_list_editor_owner, attachment2_list_share_owner, attachment2_user_capacity,
    recycle_empty, recycle_list, recycle_id_resume, share_list, share_list_my, share_list_to_me,
    file_upload_with_url,
    u2_attachment_delete, u2_attachment_update_content, u2_attachment_update_content_callback,
    u2_attachment2_update, u2_attachment2_delete, u2_file_delete_by_id, u2_file_delete_by_reference,
    u2_file_upload_octet_stream, u2_file_upload_multipart, u2_file_upload_callback,
    u2_file_list_reference_types, u2_folder_create, u2_folder_rename, u2_folder_delete,
    u2_share_create, u2_share_get, u2_share_delete, u2_share_shield, u2_share_get_with_password,
    u2_share_save_to_folder, u2_config_save_system_config, u2_attachment2_list_type_page_size_size,
};


pub fn router(pool: Pool) -> Router {
    Router::new()
        .route("/jaxrs/file/assemble/control/file/list/{id}", get(list_files))
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
        .route("/jaxrs/file/assemble/control/attachment2/{id}/office/preview/type/{type}", get(attachment2_id_office_preview_type_type))
                .route("/jaxrs/file/anonymous/file/id/download", get(anonymous_file_id_download))
        .route("/jaxrs/file/attachment/list/folder/folderId", get(attachment_list_folder_folderId))
        .route("/jaxrs/file/attachment/list/top", get(attachment_list_top))
        .route("/jaxrs/file/attachment/id", get(attachment_id))
        .route("/jaxrs/file/attachment/id/binary/base64", get(attachment_id_binary_base64))
        .route("/jaxrs/file/attachment/id/download", get(attachment_id_download))
        .route("/jaxrs/file/attachment/id/image/scale/scale/binary/base64", get(attachment_id_image_scale_scale_binary_base64))
        .route("/jaxrs/file/attachment/id/image/width/width/height/height/binary/base64", get(attachment_id_image_width_width_height_height_binary_base64))
        .route("/jaxrs/file/attachment2/exist/file/fileMd5", get(attachment2_exist_file_fileMd5))
        .route("/jaxrs/file/attachment2/list/filter/name", get(attachment2_list_filter_name))
        .route("/jaxrs/file/attachment2/list/folder/folderId", get(attachment2_list_folder_folderId))
        .route("/jaxrs/file/attachment2/list/top", get(attachment2_list_top))
        .route("/jaxrs/file/attachment2/list/type/page/size/size", get(attachment2_list_type_page_size_size))
        .route("/jaxrs/file/attachment2/id", get(attachment2_id))
        .route("/jaxrs/file/attachment2/id/binary/base64", get(attachment2_id_binary_base64))
        .route("/jaxrs/file/attachment2/id/download", get(attachment2_id_download))
        .route("/jaxrs/file/attachment2/id/download/image/width/width/height/height", get(attachment2_id_download_image_width_width_height_height))
        .route("/jaxrs/file/attachment2/id/download/stream", get(attachment2_id_download_stream))
        .route("/jaxrs/file/attachment2/id/image/scale/scale/binary/base64", get(attachment2_id_image_scale_scale_binary_base64))
        .route("/jaxrs/file/attachment2/id/image/width/width/height/height/binary/base64", get(attachment2_id_image_width_width_height_height_binary_base64))
        .route("/jaxrs/file/complex/folder/id", get(complex_folder_id))
        .route("/jaxrs/file/complex/top", get(complex_top))
        .route("/jaxrs/file/editor/list", get(editor_list))
        .route("/jaxrs/file/file/clean/unused/referencetype/cmsdocument/manage", get(file_clean_unused_referencetype_cmsdocument_manage))
        .route("/jaxrs/file/file/copy/attachment/attachmentId/referencetype/referenceType/reference/reference/scale/scale", get(file_copy_attachment_attachmentId_referencetype_referenceType_reference_reference_scale_scale))
        .route("/jaxrs/file/file/list/referencetype", get(file_list_referencetype))
        .route("/jaxrs/file/file/list/referencetype/referenceType/reference/reference", get(file_list_referencetype_referenceType_reference_reference))
        .route("/jaxrs/file/file/list/unused/referencetype/cmsdocument/manage", get(file_list_unused_referencetype_cmsdocument_manage))
        .route("/jaxrs/file/file/list/id/next/count", get(file_list_id_next_count))
        .route("/jaxrs/file/file/list/id/next/count/all", get(file_list_id_next_count_all))
        .route("/jaxrs/file/file/list/id/next/count/referencetype/referenceType", get(file_list_id_next_count_referencetype_referenceType))
        .route("/jaxrs/file/file/list/id/prev/count", get(file_list_id_prev_count))
        .route("/jaxrs/file/file/list/id/prev/count/all", get(file_list_id_prev_count_all))
        .route("/jaxrs/file/file/list/id/prev/count/referencetype/referenceType", get(file_list_id_prev_count_referencetype_referenceType))
        .route("/jaxrs/file/file/referencetype/referenceType/reference/reference", get(file_referencetype_referenceType_reference_reference))
        .route("/jaxrs/file/file/id", get(file_id))
        .route("/jaxrs/file/file/id/binary/base64", get(file_id_binary_base64))
        .route("/jaxrs/file/file/id/download", get(file_id_download))
        .route("/jaxrs/file/folder/list/top", get(folder_list_top))
        .route("/jaxrs/file/folder/list/id", get(folder_list_id))
        .route("/jaxrs/file/folder/id", get(folder_id))
        .route("/jaxrs/file/folder2/batch/download", get(folder2_batch_download))
        .route("/jaxrs/file/folder2/list/top", get(folder2_list_top))
        .route("/jaxrs/file/folder2/list/id", get(folder2_list_id))
        .route("/jaxrs/file/folder2/id", get(folder2_id))
        .route("/jaxrs/file/folder2/id/download", get(folder2_id_download))
        .route("/jaxrs/file/recycle/id", get(recycle_id))
        .route("/jaxrs/file/share/download/share/shareId/file/fileId", get(share_download_share_shareId_file_fileId))
        .route("/jaxrs/file/share/list/att/share/shareId/folder/folderId", get(share_list_att_share_shareId_folder_folderId))
        .route("/jaxrs/file/share/list/folder/share/shareId/folder/folderId", get(share_list_folder_share_shareId_folder_folderId))
        .route("/jaxrs/file/share/share/shareId/file/fileId/folder/folderId", get(share_share_shareId_file_fileId_folder_folderId))
        .route("/jaxrs/file/share/shield/id", get(share_shield_id))
        .route("/jaxrs/file/share/id", get(share_id))
        .route("/jaxrs/file/share/id/password/password", get(share_id_password_password))
        .route("/jaxrs/attachment2/upload/folder/{folderId}", post(crate::attachment2_upload_folder_folderId))
        .route("/jaxrs/attachment/update/{id}", post(crate::attachment_id_update))
        .route("/jaxrs/attachment/update/callback/callback/{id}", post(crate::attachment_id_update_callback_callback))
        .route("/jaxrs/attachment/upload/folder/{folderId}", post(crate::attachment_upload_folder_folderId))
        .route("/jaxrs/attachment/upload/folder/callback/callback/{folderId}", post(crate::attachment_upload_folder_folderId_callback_callback))
        .route("/jaxrs/attachment/upload/folder/{folderId}/callback/{callback}", post(crate::attachment_upload_folder_folderId_callback_callback))
        .route("/jaxrs/file/upload/referencetype/reference/reference/scale/scale/{referenceType}", post(crate::file_upload_referencetype_referenceType_reference_reference_scale_scale))
        .route("/jaxrs/file/upload/referencetype/reference/reference/scale/scale/callback/callback/{referenceType}", post(crate::file_upload_referencetype_referenceType_reference_reference_scale_scale_callback_callback))
        .route("/jaxrs/recycle/delete/{id}", post(crate::recycle_id_delete))
        .route("/jaxrs/recycle/resume/{id}", post(crate::recycle_id_resume))
        .route("/jaxrs/share/list/my2/{shareType}/{fileType}", get(crate::share_list_my2_shareType_fileType))
        .route("/jaxrs/share/list/to/me2/{fileType}", get(crate::share_list_to_me2_fileType))
        .route("/jaxrs/attachment/update/{id}", put(attachment_id_update))
        .route("/jaxrs/attachment/update/callback/callback/{id}", put(attachment_id_update_callback_callback))
        .route("/jaxrs/file/assemble/control/file/delete/{id}", delete(delete_file))
        .route("/jaxrs/file/core/entity/file/delete/{id}", delete(delete_file_entity))
        .route("/jaxrs/recycle/delete/{id}", delete(recycle_id_delete))
        .route("/jaxrs/file/assemble/control/update/control/config", put(update_control_config))
        .route("/jaxrs/file/core/entity/file/update/{id}", put(update_file_entity))
        // ══════ plan002 U2：Java 端点全量闭合（真实 Java 路径） ══════
        // anonymous/file 族
        .route("/jaxrs/anonymous/file/{id}/download", get(anonymous_file_id_download).post(anonymous_file_id_download))
        .route("/jaxrs/anonymous/file/{id}/download/stream", post(anonymous_file_id_download_stream))
        // attachment 族
        .route("/jaxrs/attachment/list/top", get(attachment_list_top))
        .route("/jaxrs/attachment/list/editor/{owner}", get(attachment_list_editor_owner))
        .route("/jaxrs/attachment/list/folder/{folderId}", get(attachment_list_folder_folderId))
        .route("/jaxrs/attachment/list/share/{owner}", get(attachment_list_share_owner))
        .route("/jaxrs/attachment/{id}", get(attachment_id).put(attachment_id_update).delete(u2_attachment_delete))
        .route("/jaxrs/attachment/{id}/binary/base64", get(attachment_id_binary_base64))
        .route("/jaxrs/attachment/{id}/download", get(attachment_id_download).post(attachment_id_download))
        .route("/jaxrs/attachment/{id}/download/stream", get(attachment_id_download_stream).post(attachment_id_download_stream))
        .route("/jaxrs/attachment/{id}/image/scale/{scale}/binary/base64", get(attachment_id_image_scale_scale_binary_base64))
        .route("/jaxrs/attachment/{id}/image/width/{width}/height/{height}/binary/base64", get(attachment_id_image_width_width_height_height_binary_base64))
        .route("/jaxrs/attachment/{id}/update", put(u2_attachment_update_content))
        .route("/jaxrs/attachment/{id}/update/callback/{callback}", post(u2_attachment_update_content_callback))
        // attachment2 族
        .route("/jaxrs/attachment2/exist/file/{fileMd5}", get(attachment2_exist_file_fileMd5))
        .route("/jaxrs/attachment2/list/top", get(attachment2_list_top))
        .route("/jaxrs/attachment2/list/editor/{owner}", get(attachment2_list_editor_owner))
        .route("/jaxrs/attachment2/list/filter/{name}", get(attachment2_list_filter_name))
        .route("/jaxrs/attachment2/list/folder/{folderId}", get(attachment2_list_folder_folderId))
        .route("/jaxrs/attachment2/list/share/{owner}", get(attachment2_list_share_owner))
        .route("/jaxrs/attachment2/list/type/{page}/size/{size}", post(u2_attachment2_list_type_page_size_size))
        .route("/jaxrs/attachment2/user/capacity", get(attachment2_user_capacity))
        .route("/jaxrs/attachment2/{id}", get(attachment2_id).put(u2_attachment2_update).delete(u2_attachment2_delete))
        .route("/jaxrs/attachment2/{id}/binary/base64", get(attachment2_id_binary_base64))
        .route("/jaxrs/attachment2/{id}/download", get(attachment2_id_download).post(attachment2_id_download))
        .route("/jaxrs/attachment2/{id}/download/image/width/{width}/height/{height}", get(attachment2_id_download_image_width_width_height_height))
        .route("/jaxrs/attachment2/{id}/download/stream", get(attachment2_id_download_stream).post(attachment2_id_download_stream))
        .route("/jaxrs/attachment2/{id}/image/scale/{scale}/binary/base64", get(attachment2_id_image_scale_scale_binary_base64))
        .route("/jaxrs/attachment2/{id}/image/width/{width}/height/{height}/binary/base64", get(attachment2_id_image_width_width_height_height_binary_base64))
        .route("/jaxrs/attachment2/{id}/office/preview/type/{type}", get(attachment2_id_office_preview_type_type))
        // complex / config / editor 族
        .route("/jaxrs/complex/folder/{id}", get(complex_folder_id))
        .route("/jaxrs/complex/top", get(complex_top))
        .route("/jaxrs/config", post(u2_config_save_system_config))
        .route("/jaxrs/config/is/file/manager", get(config_is_file_manager))
        .route("/jaxrs/config/system/config", get(config_system_config))
        .route("/jaxrs/editor/list", get(editor_list))
        // folder / folder2 族（folder2 CRUD 复用 folder 实现，同一 FILE_FOLDER 表）
        .route("/jaxrs/folder", post(u2_folder_create))
        .route("/jaxrs/folder/list/top", get(folder_list_top))
        .route("/jaxrs/folder/list/{id}", get(folder_list_id))
        .route("/jaxrs/folder/{id}", get(folder_id).put(u2_folder_rename).delete(u2_folder_delete))
        .route("/jaxrs/folder2", post(u2_folder_create))
        .route("/jaxrs/folder2/batch/download", get(folder2_batch_download))
        .route("/jaxrs/folder2/list/top", get(folder2_list_top))
        .route("/jaxrs/folder2/list/{id}", get(folder2_list_id))
        .route("/jaxrs/folder2/{id}", get(folder2_id).put(u2_folder_rename).delete(u2_folder_delete))
        .route("/jaxrs/folder2/{id}/download", get(folder2_id_download))
        // recycle 族
        .route("/jaxrs/recycle/empty", delete(recycle_empty))
        .route("/jaxrs/recycle/list", get(recycle_list))
        .route("/jaxrs/recycle/{id}", get(recycle_id))
        .route("/jaxrs/recycle/{id}/delete", delete(recycle_id_delete))
        .route("/jaxrs/recycle/{id}/resume", post(recycle_id_resume))
        // share 族
        .route("/jaxrs/share", post(u2_share_create))
        .route("/jaxrs/share/download/share/{shareId}/file/{fileId}", get(share_download_share_shareId_file_fileId))
        .route("/jaxrs/share/list", get(share_list))
        .route("/jaxrs/share/list/my", get(share_list_my))
        .route("/jaxrs/share/list/to/me", get(share_list_to_me))
        .route("/jaxrs/share/list/att/share/{shareId}/folder/{folderId}", get(share_list_att_share_shareId_folder_folderId))
        .route("/jaxrs/share/list/folder/share/{shareId}/folder/{folderId}", get(share_list_folder_share_shareId_folder_folderId))
        .route("/jaxrs/share/share/{shareId}/file/{fileId}/folder/{folderId}", post(u2_share_save_to_folder))
        .route("/jaxrs/share/shield/{id}", get(u2_share_shield))
        .route("/jaxrs/share/{id}", get(u2_share_get).delete(u2_share_delete))
        .route("/jaxrs/share/{id}/password/{password}", get(u2_share_get_with_password))
        // file 族（裸 GET /jaxrs/file/{id} 已被 cms_assemble_control 占用 —— 裁决见 lib.rs；
        // 本族统一挂在模块前缀 /jaxrs/file/assemble/control/file 下）
        .route("/jaxrs/file/assemble/control/file/list/referencetype", get(u2_file_list_reference_types))
        .route("/jaxrs/file/assemble/control/file/list/referencetype/{referenceType}/reference/{reference}", get(file_list_referencetype_referenceType_reference_reference))
        .route("/jaxrs/file/assemble/control/file/list/unused/referencetype/cmsdocument/manage", get(file_list_unused_referencetype_cmsdocument_manage))
        .route("/jaxrs/file/assemble/control/file/list/{id}/next/{count}", get(file_list_id_next_count))
        .route("/jaxrs/file/assemble/control/file/list/{id}/next/{count}/all", get(file_list_id_next_count_all))
        .route("/jaxrs/file/assemble/control/file/list/{id}/next/{count}/referencetype/{referenceType}", get(file_list_id_next_count_referencetype_referenceType))
        .route("/jaxrs/file/assemble/control/file/list/{id}/prev/{count}", get(file_list_id_prev_count))
        .route("/jaxrs/file/assemble/control/file/list/{id}/prev/{count}/all", get(file_list_id_prev_count_all))
        .route("/jaxrs/file/assemble/control/file/list/{id}/prev/{count}/referencetype/{referenceType}", get(file_list_id_prev_count_referencetype_referenceType))
        .route("/jaxrs/file/assemble/control/file/clean/unused/referencetype/cmsdocument/manage", delete(file_clean_unused_referencetype_cmsdocument_manage))
        .route("/jaxrs/file/assemble/control/file/copy/attachment/{attachmentId}/referencetype/{referenceType}/reference/{reference}/scale/{scale}", get(file_copy_attachment_attachmentId_referencetype_referenceType_reference_reference_scale_scale))
        .route("/jaxrs/file/assemble/control/file/referencetype/{referenceType}/reference/{reference}", delete(u2_file_delete_by_reference))
        .route("/jaxrs/file/assemble/control/file/upload/referencetype/{referenceType}/reference/{reference}/scale/{scale}", post(u2_file_upload_octet_stream).put(u2_file_upload_multipart))
        .route("/jaxrs/file/assemble/control/file/upload/referencetype/{referenceType}/reference/{reference}/scale/{scale}/callback/{callback}", post(u2_file_upload_callback))
        .route("/jaxrs/file/assemble/control/file/upload/with/url", post(file_upload_with_url))
        .route("/jaxrs/file/assemble/control/file/{id}/binary/base64", get(file_id_binary_base64))
        .route("/jaxrs/file/assemble/control/file/{id}/download", get(file_id_download).post(file_id_download))
        .route("/jaxrs/file/assemble/control/file/{id}/download/stream", post(file_id_download_stream))
        .route("/jaxrs/file/assemble/control/file/{id}", delete(u2_file_delete_by_id))
.layer(Extension(pool))
}
