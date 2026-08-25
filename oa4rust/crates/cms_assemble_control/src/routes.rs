use axum::{
    extract::Extension,
    routing::{get, post, put, delete},
    Router,
};
use deadpool_postgres::Pool;

use crate::{
    anonymous_document_filter_list_id_next_count, anonymous_document_filter_list_id_next_count_mockputtopost, anonymous_document_filter_list_page_size_size,
    anonymous_document_filter_list_page_size_size_mockputtopost, anonymous_document_id_view, anonymous_fileinfo_download_document_id, anonymous_fileinfo_download_document_id_stream,
    anonymous_fileinfo_list_document_documentId, anonymous_form_id, anonymous_form_v2_id, anonymous_form_v2_id_mobile, anonymous_form_v2_lookup_document_docId,
    anonymous_form_v2_lookup_document_docId_mobile, anonymous_surface_appdict_appDictFlag_appInfo_appInfoFlag, anonymous_surface_appdict_appDictFlag_appInfo_appInfoFlag_data,
    anonymous_surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_data, anonymous_surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_data,
    anonymous_surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_path2_data, anonymous_surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_path2_path3_data,
    anonymous_surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_path2_path3_path4_data, anonymous_surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_path2_path3_path4_path5_data,
    anonymous_surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_path2_path3_path4_path5_path6_data,
    anonymous_surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_path2_path3_path4_path5_path6_path7_data, anonymous_surface_appdict_list_appInfo_appInfoFlag, appconfig_u2_get,
    appconfig_u2_update, appinfo_alias_alias, appinfo_appId_icon_size_size, appinfo_erase_app_id, appinfo_erase_app_id_mockdeletetoget, appinfo_filter_list_id_next_count,
    appinfo_filter_list_id_next_count_mockputtopost, appinfo_filter_list_id_prev_count, appinfo_filter_list_id_prev_count_mockputtopost, appinfo_flag, appinfo_get_user_publish_appId, appinfo_id,
    appinfo_id_control, appinfo_id_mockdeletetoget, appinfo_list_all, appinfo_list_appType, appinfo_list_appType_manager, appinfo_list_has_document, appinfo_list_has_document_appType,
    appinfo_list_has_document_type_appType, appinfo_list_manage, appinfo_list_manage_type_appType, appinfo_list_user_publish, appinfo_list_user_publish_type_appType,
    appinfo_list_user_publish_with_process, appinfo_list_user_view, appinfo_list_user_view_all, appinfo_list_user_view_all_type_appType, appinfo_list_user_view_article_type_appType,
    appinfo_list_user_view_data, appinfo_list_user_view_data_type_appType, appinfo_u2_create, appinfo_u2_delete, application_id, categoryinfo_alias_alias, categoryinfo_bind_categoryId_view,
    categoryinfo_bind_categoryId_view_mockputtopost, categoryinfo_erase_category_id, categoryinfo_erase_category_id_mockdeletetoget, categoryinfo_ext_content_save_u3,
    categoryinfo_filter_list_id_next_count_app_appId, categoryinfo_filter_list_id_next_count_app_appId_mockputtopost, categoryinfo_filter_list_id_prev_count_app_appId,
    categoryinfo_filter_list_id_prev_count_app_appId_mockputtopost, categoryinfo_filter_list_page_size_size, categoryinfo_filter_list_page_size_size_mockputtopost, categoryinfo_flag, categoryinfo_id,
    categoryinfo_id_control, categoryinfo_id_execute_projection, categoryinfo_id_mockdeletetoget, categoryinfo_list_all, categoryinfo_list_manage_app_appId, categoryinfo_list_objects_u3,
    categoryinfo_list_publish_app_appId, categoryinfo_list_view_app_appId, categoryinfo_list_view_app_appId_all, categoryinfo_list_view_app_appId_data, categoryinfo_u2_create, categoryinfo_u2_delete,
    commend_id, commend_list_paging, commend_list_paging_page_size_size, comment_commend_u3, comment_id, comment_id_mockdeletetoget, comment_list_id_next_count,
    comment_list_id_next_count_mockputtopost, comment_list_id_prev_count, comment_list_id_prev_count_mockputtopost, comment_u2_create, comment_u2_delete, comment_u2_list_page_size_size,
    comment_uncommend_u3, correlation_create_u3, correlation_doc_docId, correlation_list_doc_docId, correlation_list_doc_docId_site_site, correlation_u2_doc_delete, correlation_update_u3,
    data_document_id, data_document_id_array_data, data_document_id_create, data_document_id_delete, data_document_id_mockdeletetoget, data_document_id_mockputtopost, data_document_id_path0,
    data_document_id_path0_create, data_document_id_path0_delete, data_document_id_path0_mockdeletetoget, data_document_id_path0_mockputtopost, data_document_id_path0_path1,
    data_document_id_path0_path1_create, data_document_id_path0_path1_delete, data_document_id_path0_path1_mockdeletetoget, data_document_id_path0_path1_mockputtopost,
    data_document_id_path0_path1_path2, data_document_id_path0_path1_path2_create, data_document_id_path0_path1_path2_delete, data_document_id_path0_path1_path2_mockdeletetoget,
    data_document_id_path0_path1_path2_mockputtopost, data_document_id_path0_path1_path2_path3, data_document_id_path0_path1_path2_path3_create, data_document_id_path0_path1_path2_path3_delete,
    data_document_id_path0_path1_path2_path3_mockdeletetoget, data_document_id_path0_path1_path2_path3_mockputtopost, data_document_id_path0_path1_path2_path3_path4,
    data_document_id_path0_path1_path2_path3_path4_create, data_document_id_path0_path1_path2_path3_path4_delete, data_document_id_path0_path1_path2_path3_path4_mockdeletetoget,
    data_document_id_path0_path1_path2_path3_path4_mockputtopost, data_document_id_path0_path1_path2_path3_path4_path5, data_document_id_path0_path1_path2_path3_path4_path5_create,
    data_document_id_path0_path1_path2_path3_path4_path5_delete, data_document_id_path0_path1_path2_path3_path4_path5_mockdeletetoget,
    data_document_id_path0_path1_path2_path3_path4_path5_mockputtopost, data_document_id_path0_path1_path2_path3_path4_path5_path6, data_document_id_path0_path1_path2_path3_path4_path5_path6_create,
    data_document_id_path0_path1_path2_path3_path4_path5_path6_delete, data_document_id_path0_path1_path2_path3_path4_path5_path6_mockdeletetoget,
    data_document_id_path0_path1_path2_path3_path4_path5_path6_mockputtopost, data_document_id_path0_path1_path2_path3_path4_path5_path6_path7,
    data_document_id_path0_path1_path2_path3_path4_path5_path6_path7_create, data_document_id_path0_path1_path2_path3_path4_path5_path6_path7_delete,
    data_document_id_path0_path1_path2_path3_path4_path5_path6_path7_mockdeletetoget, data_document_id_path0_path1_path2_path3_path4_path5_path6_path7_mockputtopost,
    data_document_id_path0_path1_path2_path3_path4_path5_path6_path7_update, data_document_id_path0_path1_path2_path3_path4_path5_path6_update,
    data_document_id_path0_path1_path2_path3_path4_path5_update, data_document_id_path0_path1_path2_path3_path4_update, data_document_id_path0_path1_path2_path3_update,
    data_document_id_path0_path1_path2_update, data_document_id_path0_path1_update, data_document_id_path0_update, data_document_id_update, design_appdict_create_u3, design_appdict_delete_u3,
    design_appdict_id, design_appdict_id_mockdeletetoget, design_appdict_id_mockputtopost, design_appdict_list_appInfo_appId, design_appdict_list_paging_page_size_size, design_appdict_update_u3,
    designer_u2_search, document_achive_u3, document_batch_delete_mock_u3, document_batch_delete_u3, document_batch_modify_mock_u3, document_batch_modify_u3, document_batch_name_status_u3,
    document_batch_status_u3, document_cipher_filter_list_page_size_size, document_cipher_filter_list_page_size_size_mockputtopost, document_cipher_id_persist_view_record,
    document_cipher_permission_read_u3, document_cipher_publish_workflow_mock_u3, document_cipher_publish_workflow_u3, document_control_u3, document_draft_next_u3, document_filter_next_u3,
    document_filter_paging_manager_u3, document_filter_paging_u3, document_filter_prev_u3, document_id_view_count, document_list_document_data_u3, document_notify_u3, document_permission_read_u3,
    document_persons_u3, document_publish_content_mock_u3, document_publish_content_u3, document_publish_html_u3, document_search, document_u2_category_change, document_u2_commend,
    document_u2_create, document_u2_delete, document_u2_document_data, document_u2_fields, document_u2_filter_count, document_u2_get, document_u2_list_document, document_u2_publish,
    document_u2_publish_cancel, document_u2_top, document_u2_un_top, document_u2_uncommend, document_u2_update, export_app_info_app_info_flag, file_copy_u3, file_delete_u3, file_download_with_app_u3,
    file_flag, file_flag_appInfo_appInfoFlag_content, file_flag_mockdeletetoget, file_id, file_id_content, file_id_download, file_id_upload, file_list_appInfo_appInfoFlag, file_list_id_next_count,
    file_list_id_prev_count, file_u2_create, file_u2_update, file_update_u3, fileinfo_batch_download_doc_docId_site_site, fileinfo_binary_base64_u3, fileinfo_download_document_id,
    fileinfo_download_document_id_stream, fileinfo_download_transfer_flag_flag, fileinfo_edit_id_doc_docId, fileinfo_edit_id_doc_docId_mockputtopost, fileinfo_id,
    fileinfo_id_doc_docId_change_seqnumber_seqNumber, fileinfo_id_document_documentId, fileinfo_id_mockdeletetoget, fileinfo_id_online_info, fileinfo_id_preview_pdf, fileinfo_list_all,
    fileinfo_list_document_documentId, fileinfo_u2_copy_to_doc, fileinfo_u2_delete, fileinfo_u2_filter, fileinfo_u2_replace_to_doc, fileinfo_update_document_docId_attachment_id,
    fileinfo_update_document_docId_attachment_id_callback_callback, fileinfo_update_id_content, fileinfo_upload_doc_docId_save_as_flag, fileinfo_upload_document_docId,
    fileinfo_upload_document_docId_callback_callback, fileinfo_upload_with_url_u3, form_filter_list_id_next_count_app_appId, form_filter_list_id_next_count_app_appId_mockputtopost,
    form_filter_list_id_prev_count_app_appId, form_filter_list_id_prev_count_app_appId_mockputtopost, form_get_with_appinfo_u3, form_id, form_id_mockdeletetoget, form_id_mockputtopost, form_list_all,
    form_list_app_appId, form_list_formfield_appInfo_appId, form_list_id_formfield, form_u2_create, form_u2_delete, form_u2_update, form_v2_id, form_v2_id_mobile, form_v2_lookup_document_docId,
    form_v2_lookup_document_docId_mobile, formversion_id, formversion_list_form_formId, get_control_config, image_encode_base64, image_encode_base64_size_size,
    image_resize_id_id_width_width_height_height, import_app_info_app_info_flag, input_compare, input_compare_mockputtopost, input_cover, input_cover_mockputtopost, input_create,
    input_create_mockputtopost, input_prepare_cover, input_prepare_cover_mockputtopost, input_prepare_create, input_prepare_create_mockputtopost, list_control_sections, log_filter_list_id_next_count,
    log_filter_list_id_prev_count, log_id, log_list_app_appId, log_list_category_categoryId, log_list_document_documentId, log_list_filter_page_size_size, log_list_level_operationLevel,
    output_appInfoFlag_select, output_appInfoFlag_select_mockputtopost, output_list, permission_appInfo_id_manageable, permission_appInfo_id_managers, permission_appInfo_id_publishers,
    permission_appInfo_id_viewers, permission_categoryInfo_id_manageable, permission_category_id_managers, permission_category_id_publishers, permission_category_id_viewers,
    permission_management_refresh_all, permission_management_refresh_category_categoryId, permission_save_manager_app_u3, permission_save_manager_category_u3, permission_save_publisher_app_u3,
    permission_save_publisher_category_u3, permission_save_viewer_app_u3, permission_save_viewer_category_u3, permission_u2_app_info, permission_u2_category_info, queryview_flag_definition,
    review_v2_search_u3, script_id, script_id_mockdeletetoget, script_id_mockputtopost, script_list_app_appId_name_name, script_list_app_flag, script_list_id_next_count, script_list_id_prev_count,
    script_list_paging_page_size_size, script_load_u3, script_post_nested_u3, script_u2_create, script_u2_delete, script_u2_list_manager, script_u2_update, script_uniqueName_app_flag_imported,
    scriptversion_id, scriptversion_list_script_scriptId, searchfilter_list_archive_filter_category_categoryId, searchfilter_list_draft_filter_category_categoryId,
    searchfilter_list_publish_filter_category_categoryId, surface_appdict_appDictFlag_appInfo_appInfoFlag, surface_appdict_appDictFlag_appInfo_appInfoFlag_data,
    surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_data, surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_data_delete, surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_data_post,
    surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_data_put, surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_data,
    surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_data_delete, surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_data_post,
    surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_data_put, surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_path2_data,
    surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_path2_data_delete, surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_path2_data_post,
    surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_path2_data_put, surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_path2_path3_data,
    surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_path2_path3_data_delete, surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_path2_path3_data_post,
    surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_path2_path3_data_put, surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_path2_path3_path4_data,
    surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_path2_path3_path4_data_delete, surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_path2_path3_path4_data_post,
    surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_path2_path3_path4_data_put, surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_path2_path3_path4_path5_data,
    surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_path2_path3_path4_path5_data_delete, surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_path2_path3_path4_path5_data_post,
    surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_path2_path3_path4_path5_data_put, surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_path2_path3_path4_path5_path6_data,
    surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_path2_path3_path4_path5_path6_data_delete,
    surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_path2_path3_path4_path5_path6_data_post,
    surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_path2_path3_path4_path5_path6_data_put,
    surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_path2_path3_path4_path5_path6_path7_data,
    surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_path2_path3_path4_path5_path6_path7_data_delete,
    surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_path2_path3_path4_path5_path6_path7_data_post,
    surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_path2_path3_path4_path5_path6_path7_data_put, surface_appdict_appDictFlag_appInfo_appInfoFlag_update,
    surface_appdict_list_appInfo_appInfoFlag, templateform_id, templateform_id_mockdeletetoget, templateform_list, templateform_list_category, templateform_list_category_mockputtopost,
    templateform_u2_create, templateform_u2_delete, update_control_config, uuid_random, view_id, view_id_mockdeletetoget, view_id_mockputtopost, view_list_all, view_list_app_appId,
    view_list_category_categoryId, view_list_form_formId, view_u2_create, view_u2_delete, view_u2_update, view_viewdata_list_id_next_count, viewcategory_id, viewcategory_id_mockdeletetoget,
    viewcategory_list_all, viewcategory_list_category_categoryId, viewcategory_list_view_viewId, viewcategory_u2_create, viewcategory_u2_delete, viewfieldconfig_id,
    viewfieldconfig_id_mockdeletetoget, viewfieldconfig_id_mockputtopost, viewfieldconfig_list_all, viewfieldconfig_list_view_viewId, viewfieldconfig_u2_create, viewfieldconfig_u2_delete,
    viewfieldconfig_u2_update, viewrecord_by_person_u3, viewrecord_document_docId_filter_list_id_next_count, viewrecord_document_docId_has_view, viewrecord_list_install_log_paging_page_size_size,
    viewrecord_unread_u3
};




pub fn router(pool: deadpool_postgres::Pool) -> axum::Router {
    // ── Rust 自有扩展端点（非 Java 口径，保留作向后兼容）：
    //   /cms_assemble_control/get|update/control/config、/list/control/sections（控制面配置）
    //   /commend/list/paging/{docId}（按文档查 commend）、/queryview/*、/application/{id}、
    //   /cms_assemble_control/document/search、GET /fileinfo/{id}、/appinfo/flag、
    //   /categoryinfo/flag、/file/flag、GET /correlation/doc/{docId}
    Router::new()
        .route("/jaxrs/cms_assemble_control/get/control/config", get(get_control_config))
        .route("/jaxrs/cms_assemble_control/list/control/sections", get(list_control_sections))
        .route("/jaxrs/cms_assemble_control/update/control/config", get(update_control_config))
        .route("/jaxrs/document/{id}/view/count", get(document_id_view_count))
        .route("/jaxrs/commend/list/paging/{docId}", get(commend_list_paging))
        .route("/jaxrs/queryview/flag/{view_flag}/definition/{query_flag}", get(queryview_flag_definition))
        .route("/jaxrs/application/{id}", get(application_id))
        .route("/jaxrs/cms_assemble_control/document/search", get(document_search))
        .route("/jaxrs/anonymous/document/{id}/view", get(anonymous_document_id_view))
        // ── data/document 家族（Java DataAction 对齐：{path0}..{path7} 通配 + 全动词）──
        .route("/jaxrs/data/document/{id}", get(data_document_id))
        .route("/jaxrs/data/document/{id}", put(data_document_id_update))
        .route("/jaxrs/data/document/{id}", post(data_document_id_create))
        .route("/jaxrs/data/document/{id}", delete(data_document_id_delete))
        .route("/jaxrs/data/document/{id}/array/data", post(data_document_id_array_data))
        .route("/jaxrs/data/document/{id}/mockdeletetoget", get(data_document_id_mockdeletetoget))
        .route("/jaxrs/data/document/{id}/mockputtopost", post(data_document_id_mockputtopost))
        .route("/jaxrs/data/document/{id}/{path0}", get(data_document_id_path0))
        .route("/jaxrs/data/document/{id}/{path0}", put(data_document_id_path0_update))
        .route("/jaxrs/data/document/{id}/{path0}", post(data_document_id_path0_create))
        .route("/jaxrs/data/document/{id}/{path0}", delete(data_document_id_path0_delete))
        .route("/jaxrs/data/document/{id}/{path0}/mockdeletetoget", get(data_document_id_path0_mockdeletetoget))
        .route("/jaxrs/data/document/{id}/{path0}/mockputtopost", post(data_document_id_path0_mockputtopost))
        .route("/jaxrs/data/document/{id}/{path0}/{path1}", get(data_document_id_path0_path1))
        .route("/jaxrs/data/document/{id}/{path0}/{path1}", put(data_document_id_path0_path1_update))
        .route("/jaxrs/data/document/{id}/{path0}/{path1}", post(data_document_id_path0_path1_create))
        .route("/jaxrs/data/document/{id}/{path0}/{path1}", delete(data_document_id_path0_path1_delete))
        .route("/jaxrs/data/document/{id}/{path0}/{path1}/mockdeletetoget", get(data_document_id_path0_path1_mockdeletetoget))
        .route("/jaxrs/data/document/{id}/{path0}/{path1}/mockputtopost", post(data_document_id_path0_path1_mockputtopost))
        .route("/jaxrs/data/document/{id}/{path0}/{path1}/{path2}", get(data_document_id_path0_path1_path2))
        .route("/jaxrs/data/document/{id}/{path0}/{path1}/{path2}", put(data_document_id_path0_path1_path2_update))
        .route("/jaxrs/data/document/{id}/{path0}/{path1}/{path2}", post(data_document_id_path0_path1_path2_create))
        .route("/jaxrs/data/document/{id}/{path0}/{path1}/{path2}", delete(data_document_id_path0_path1_path2_delete))
        .route("/jaxrs/data/document/{id}/{path0}/{path1}/{path2}/mockdeletetoget", get(data_document_id_path0_path1_path2_mockdeletetoget))
        .route("/jaxrs/data/document/{id}/{path0}/{path1}/{path2}/mockputtopost", post(data_document_id_path0_path1_path2_mockputtopost))
        .route("/jaxrs/data/document/{id}/{path0}/{path1}/{path2}/{path3}", get(data_document_id_path0_path1_path2_path3))
        .route("/jaxrs/data/document/{id}/{path0}/{path1}/{path2}/{path3}", put(data_document_id_path0_path1_path2_path3_update))
        .route("/jaxrs/data/document/{id}/{path0}/{path1}/{path2}/{path3}", post(data_document_id_path0_path1_path2_path3_create))
        .route("/jaxrs/data/document/{id}/{path0}/{path1}/{path2}/{path3}", delete(data_document_id_path0_path1_path2_path3_delete))
        .route("/jaxrs/data/document/{id}/{path0}/{path1}/{path2}/{path3}/mockdeletetoget", get(data_document_id_path0_path1_path2_path3_mockdeletetoget))
        .route("/jaxrs/data/document/{id}/{path0}/{path1}/{path2}/{path3}/mockputtopost", post(data_document_id_path0_path1_path2_path3_mockputtopost))
        .route("/jaxrs/data/document/{id}/{path0}/{path1}/{path2}/{path3}/{path4}", get(data_document_id_path0_path1_path2_path3_path4))
        .route("/jaxrs/data/document/{id}/{path0}/{path1}/{path2}/{path3}/{path4}", put(data_document_id_path0_path1_path2_path3_path4_update))
        .route("/jaxrs/data/document/{id}/{path0}/{path1}/{path2}/{path3}/{path4}", post(data_document_id_path0_path1_path2_path3_path4_create))
        .route("/jaxrs/data/document/{id}/{path0}/{path1}/{path2}/{path3}/{path4}", delete(data_document_id_path0_path1_path2_path3_path4_delete))
        .route("/jaxrs/data/document/{id}/{path0}/{path1}/{path2}/{path3}/{path4}/mockdeletetoget", get(data_document_id_path0_path1_path2_path3_path4_mockdeletetoget))
        .route("/jaxrs/data/document/{id}/{path0}/{path1}/{path2}/{path3}/{path4}/mockputtopost", post(data_document_id_path0_path1_path2_path3_path4_mockputtopost))
        .route("/jaxrs/data/document/{id}/{path0}/{path1}/{path2}/{path3}/{path4}/{path5}", get(data_document_id_path0_path1_path2_path3_path4_path5))
        .route("/jaxrs/data/document/{id}/{path0}/{path1}/{path2}/{path3}/{path4}/{path5}", put(data_document_id_path0_path1_path2_path3_path4_path5_update))
        .route("/jaxrs/data/document/{id}/{path0}/{path1}/{path2}/{path3}/{path4}/{path5}", post(data_document_id_path0_path1_path2_path3_path4_path5_create))
        .route("/jaxrs/data/document/{id}/{path0}/{path1}/{path2}/{path3}/{path4}/{path5}", delete(data_document_id_path0_path1_path2_path3_path4_path5_delete))
        .route("/jaxrs/data/document/{id}/{path0}/{path1}/{path2}/{path3}/{path4}/{path5}/mockdeletetoget", get(data_document_id_path0_path1_path2_path3_path4_path5_mockdeletetoget))
        .route("/jaxrs/data/document/{id}/{path0}/{path1}/{path2}/{path3}/{path4}/{path5}/mockputtopost", post(data_document_id_path0_path1_path2_path3_path4_path5_mockputtopost))
        .route("/jaxrs/data/document/{id}/{path0}/{path1}/{path2}/{path3}/{path4}/{path5}/{path6}", get(data_document_id_path0_path1_path2_path3_path4_path5_path6))
        .route("/jaxrs/data/document/{id}/{path0}/{path1}/{path2}/{path3}/{path4}/{path5}/{path6}", put(data_document_id_path0_path1_path2_path3_path4_path5_path6_update))
        .route("/jaxrs/data/document/{id}/{path0}/{path1}/{path2}/{path3}/{path4}/{path5}/{path6}", post(data_document_id_path0_path1_path2_path3_path4_path5_path6_create))
        .route("/jaxrs/data/document/{id}/{path0}/{path1}/{path2}/{path3}/{path4}/{path5}/{path6}", delete(data_document_id_path0_path1_path2_path3_path4_path5_path6_delete))
        .route("/jaxrs/data/document/{id}/{path0}/{path1}/{path2}/{path3}/{path4}/{path5}/{path6}/mockdeletetoget", get(data_document_id_path0_path1_path2_path3_path4_path5_path6_mockdeletetoget))
        .route("/jaxrs/data/document/{id}/{path0}/{path1}/{path2}/{path3}/{path4}/{path5}/{path6}/mockputtopost", post(data_document_id_path0_path1_path2_path3_path4_path5_path6_mockputtopost))
        .route("/jaxrs/data/document/{id}/{path0}/{path1}/{path2}/{path3}/{path4}/{path5}/{path6}/{path7}", get(data_document_id_path0_path1_path2_path3_path4_path5_path6_path7))
        .route("/jaxrs/data/document/{id}/{path0}/{path1}/{path2}/{path3}/{path4}/{path5}/{path6}/{path7}", put(data_document_id_path0_path1_path2_path3_path4_path5_path6_path7_update))
        .route("/jaxrs/data/document/{id}/{path0}/{path1}/{path2}/{path3}/{path4}/{path5}/{path6}/{path7}", post(data_document_id_path0_path1_path2_path3_path4_path5_path6_path7_create))
        .route("/jaxrs/data/document/{id}/{path0}/{path1}/{path2}/{path3}/{path4}/{path5}/{path6}/{path7}", delete(data_document_id_path0_path1_path2_path3_path4_path5_path6_path7_delete))
        .route("/jaxrs/data/document/{id}/{path0}/{path1}/{path2}/{path3}/{path4}/{path5}/{path6}/{path7}/mockdeletetoget", get(data_document_id_path0_path1_path2_path3_path4_path5_path6_path7_mockdeletetoget))
        .route("/jaxrs/data/document/{id}/{path0}/{path1}/{path2}/{path3}/{path4}/{path5}/{path6}/{path7}/mockputtopost", post(data_document_id_path0_path1_path2_path3_path4_path5_path6_path7_mockputtopost))
        .route("/jaxrs/anonymous/fileinfo/download/document/{id}", get(anonymous_fileinfo_download_document_id))
        .route("/jaxrs/fileinfo/download/document/{id}", get(fileinfo_download_document_id))
        .route("/jaxrs/fileinfo/upload/document/{id}", post(fileinfo_upload_document_docId))
        .route("/jaxrs/fileinfo/{id}", get(fileinfo_id))
        .route("/jaxrs/fileinfo/{id}/document/{docId}", get(fileinfo_id_document_documentId))
        .route("/jaxrs/fileinfo/{id}/mockdeletetoget", get(fileinfo_id_mockdeletetoget))
                // ── anonymous/surface/appdict（Java AppDictAnonymousAction 对齐）──
        .route("/jaxrs/anonymous/surface/appdict/{appDictFlag}/appInfo/{appInfoFlag}", get(anonymous_surface_appdict_appDictFlag_appInfo_appInfoFlag))
        .route("/jaxrs/anonymous/surface/appdict/{appDictFlag}/appInfo/{appInfoFlag}/data", get(anonymous_surface_appdict_appDictFlag_appInfo_appInfoFlag_data))
        .route("/jaxrs/anonymous/surface/appdict/{appDictFlag}/appInfo/{appInfoFlag}/{path0}/data", get(anonymous_surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_data))
        .route("/jaxrs/anonymous/surface/appdict/{appDictFlag}/appInfo/{appInfoFlag}/{path0}/{path1}/data", get(anonymous_surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_data))
        .route("/jaxrs/anonymous/surface/appdict/{appDictFlag}/appInfo/{appInfoFlag}/{path0}/{path1}/{path2}/data", get(anonymous_surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_path2_data))
        .route("/jaxrs/anonymous/surface/appdict/{appDictFlag}/appInfo/{appInfoFlag}/{path0}/{path1}/{path2}/{path3}/data", get(anonymous_surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_path2_path3_data))
        .route("/jaxrs/anonymous/surface/appdict/{appDictFlag}/appInfo/{appInfoFlag}/{path0}/{path1}/{path2}/{path3}/{path4}/data", get(anonymous_surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_path2_path3_path4_data))
        .route("/jaxrs/anonymous/surface/appdict/{appDictFlag}/appInfo/{appInfoFlag}/{path0}/{path1}/{path2}/{path3}/{path4}/{path5}/data", get(anonymous_surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_path2_path3_path4_path5_data))
        .route("/jaxrs/anonymous/surface/appdict/{appDictFlag}/appInfo/{appInfoFlag}/{path0}/{path1}/{path2}/{path3}/{path4}/{path5}/{path6}/data", get(anonymous_surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_path2_path3_path4_path5_path6_data))
        .route("/jaxrs/anonymous/surface/appdict/{appDictFlag}/appInfo/{appInfoFlag}/{path0}/{path1}/{path2}/{path3}/{path4}/{path5}/{path6}/{path7}/data", get(anonymous_surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_path2_path3_path4_path5_path6_path7_data))
                // ── surface/appdict（Java AppDictAction 对齐：段序修正 + 全动词）──
        .route("/jaxrs/surface/appdict/{appDictFlag}/appInfo/{appInfoFlag}", get(surface_appdict_appDictFlag_appInfo_appInfoFlag))
        .route("/jaxrs/surface/appdict/{appDictFlag}/appInfo/{appInfoFlag}", put(surface_appdict_appDictFlag_appInfo_appInfoFlag_update))
        .route("/jaxrs/surface/appdict/{appDictFlag}/appInfo/{appInfoFlag}/mockputtopost", post(surface_appdict_appDictFlag_appInfo_appInfoFlag_update))
        .route("/jaxrs/surface/appdict/{appDictFlag}/appInfo/{appInfoFlag}/data", get(surface_appdict_appDictFlag_appInfo_appInfoFlag_data))
        .route("/jaxrs/surface/appdict/{appDictFlag}/appInfo/{appInfoFlag}/{path0}/data", get(surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_data))
        .route("/jaxrs/surface/appdict/{appDictFlag}/appInfo/{appInfoFlag}/{path0}/data", put(surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_data_put))
        .route("/jaxrs/surface/appdict/{appDictFlag}/appInfo/{appInfoFlag}/{path0}/data", post(surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_data_post))
        .route("/jaxrs/surface/appdict/{appDictFlag}/appInfo/{appInfoFlag}/{path0}/data", delete(surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_data_delete))
        .route("/jaxrs/surface/appdict/{appDictFlag}/appInfo/{appInfoFlag}/{path0}/data/mockdeletetoget", get(surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_data_delete))
        .route("/jaxrs/surface/appdict/{appDictFlag}/appInfo/{appInfoFlag}/{path0}/{path1}/data", get(surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_data))
        .route("/jaxrs/surface/appdict/{appDictFlag}/appInfo/{appInfoFlag}/{path0}/{path1}/data", put(surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_data_put))
        .route("/jaxrs/surface/appdict/{appDictFlag}/appInfo/{appInfoFlag}/{path0}/{path1}/data", post(surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_data_post))
        .route("/jaxrs/surface/appdict/{appDictFlag}/appInfo/{appInfoFlag}/{path0}/{path1}/data", delete(surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_data_delete))
        .route("/jaxrs/surface/appdict/{appDictFlag}/appInfo/{appInfoFlag}/{path0}/{path1}/data/mockdeletetoget", get(surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_data_delete))
        .route("/jaxrs/surface/appdict/{appDictFlag}/appInfo/{appInfoFlag}/{path0}/{path1}/{path2}/data", get(surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_path2_data))
        .route("/jaxrs/surface/appdict/{appDictFlag}/appInfo/{appInfoFlag}/{path0}/{path1}/{path2}/data", put(surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_path2_data_put))
        .route("/jaxrs/surface/appdict/{appDictFlag}/appInfo/{appInfoFlag}/{path0}/{path1}/{path2}/data", post(surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_path2_data_post))
        .route("/jaxrs/surface/appdict/{appDictFlag}/appInfo/{appInfoFlag}/{path0}/{path1}/{path2}/data", delete(surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_path2_data_delete))
        .route("/jaxrs/surface/appdict/{appDictFlag}/appInfo/{appInfoFlag}/{path0}/{path1}/{path2}/data/mockdeletetoget", get(surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_path2_data_delete))
        .route("/jaxrs/surface/appdict/{appDictFlag}/appInfo/{appInfoFlag}/{path0}/{path1}/{path2}/data/mockputtopost", post(surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_path2_data_put))
        .route("/jaxrs/surface/appdict/{appDictFlag}/appInfo/{appInfoFlag}/{path0}/{path1}/{path2}/{path3}/data", get(surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_path2_path3_data))
        .route("/jaxrs/surface/appdict/{appDictFlag}/appInfo/{appInfoFlag}/{path0}/{path1}/{path2}/{path3}/data", put(surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_path2_path3_data_put))
        .route("/jaxrs/surface/appdict/{appDictFlag}/appInfo/{appInfoFlag}/{path0}/{path1}/{path2}/{path3}/data", post(surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_path2_path3_data_post))
        .route("/jaxrs/surface/appdict/{appDictFlag}/appInfo/{appInfoFlag}/{path0}/{path1}/{path2}/{path3}/data", delete(surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_path2_path3_data_delete))
        .route("/jaxrs/surface/appdict/{appDictFlag}/appInfo/{appInfoFlag}/{path0}/{path1}/{path2}/{path3}/data/mockdeletetoget", get(surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_path2_path3_data_delete))
        .route("/jaxrs/surface/appdict/{appDictFlag}/appInfo/{appInfoFlag}/{path0}/{path1}/{path2}/{path3}/data/mockputtopost", post(surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_path2_path3_data_put))
        .route("/jaxrs/surface/appdict/{appDictFlag}/appInfo/{appInfoFlag}/{path0}/{path1}/{path2}/{path3}/{path4}/data", get(surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_path2_path3_path4_data))
        .route("/jaxrs/surface/appdict/{appDictFlag}/appInfo/{appInfoFlag}/{path0}/{path1}/{path2}/{path3}/{path4}/data", put(surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_path2_path3_path4_data_put))
        .route("/jaxrs/surface/appdict/{appDictFlag}/appInfo/{appInfoFlag}/{path0}/{path1}/{path2}/{path3}/{path4}/data", post(surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_path2_path3_path4_data_post))
        .route("/jaxrs/surface/appdict/{appDictFlag}/appInfo/{appInfoFlag}/{path0}/{path1}/{path2}/{path3}/{path4}/data", delete(surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_path2_path3_path4_data_delete))
        .route("/jaxrs/surface/appdict/{appDictFlag}/appInfo/{appInfoFlag}/{path0}/{path1}/{path2}/{path3}/{path4}/data/mockdeletetoget", get(surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_path2_path3_path4_data_delete))
        .route("/jaxrs/surface/appdict/{appDictFlag}/appInfo/{appInfoFlag}/{path0}/{path1}/{path2}/{path3}/{path4}/data/mockputtopost", post(surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_path2_path3_path4_data_put))
        .route("/jaxrs/surface/appdict/{appDictFlag}/appInfo/{appInfoFlag}/{path0}/{path1}/{path2}/{path3}/{path4}/{path5}/data", get(surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_path2_path3_path4_path5_data))
        .route("/jaxrs/surface/appdict/{appDictFlag}/appInfo/{appInfoFlag}/{path0}/{path1}/{path2}/{path3}/{path4}/{path5}/data", put(surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_path2_path3_path4_path5_data_put))
        .route("/jaxrs/surface/appdict/{appDictFlag}/appInfo/{appInfoFlag}/{path0}/{path1}/{path2}/{path3}/{path4}/{path5}/data", post(surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_path2_path3_path4_path5_data_post))
        .route("/jaxrs/surface/appdict/{appDictFlag}/appInfo/{appInfoFlag}/{path0}/{path1}/{path2}/{path3}/{path4}/{path5}/data", delete(surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_path2_path3_path4_path5_data_delete))
        .route("/jaxrs/surface/appdict/{appDictFlag}/appInfo/{appInfoFlag}/{path0}/{path1}/{path2}/{path3}/{path4}/{path5}/data/mockdeletetoget", get(surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_path2_path3_path4_path5_data_delete))
        .route("/jaxrs/surface/appdict/{appDictFlag}/appInfo/{appInfoFlag}/{path0}/{path1}/{path2}/{path3}/{path4}/{path5}/data/mockputtopost", post(surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_path2_path3_path4_path5_data_put))
        .route("/jaxrs/surface/appdict/{appDictFlag}/appInfo/{appInfoFlag}/{path0}/{path1}/{path2}/{path3}/{path4}/{path5}/{path6}/data", get(surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_path2_path3_path4_path5_path6_data))
        .route("/jaxrs/surface/appdict/{appDictFlag}/appInfo/{appInfoFlag}/{path0}/{path1}/{path2}/{path3}/{path4}/{path5}/{path6}/data", put(surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_path2_path3_path4_path5_path6_data_put))
        .route("/jaxrs/surface/appdict/{appDictFlag}/appInfo/{appInfoFlag}/{path0}/{path1}/{path2}/{path3}/{path4}/{path5}/{path6}/data", post(surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_path2_path3_path4_path5_path6_data_post))
        .route("/jaxrs/surface/appdict/{appDictFlag}/appInfo/{appInfoFlag}/{path0}/{path1}/{path2}/{path3}/{path4}/{path5}/{path6}/data", delete(surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_path2_path3_path4_path5_path6_data_delete))
        .route("/jaxrs/surface/appdict/{appDictFlag}/appInfo/{appInfoFlag}/{path0}/{path1}/{path2}/{path3}/{path4}/{path5}/{path6}/data/mockdeletetoget", get(surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_path2_path3_path4_path5_path6_data_delete))
        .route("/jaxrs/surface/appdict/{appDictFlag}/appInfo/{appInfoFlag}/{path0}/{path1}/{path2}/{path3}/{path4}/{path5}/{path6}/data/mockputtopost", post(surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_path2_path3_path4_path5_path6_data_put))
        .route("/jaxrs/surface/appdict/{appDictFlag}/appInfo/{appInfoFlag}/{path0}/{path1}/{path2}/{path3}/{path4}/{path5}/{path6}/{path7}/data", get(surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_path2_path3_path4_path5_path6_path7_data))
        .route("/jaxrs/surface/appdict/{appDictFlag}/appInfo/{appInfoFlag}/{path0}/{path1}/{path2}/{path3}/{path4}/{path5}/{path6}/{path7}/data", put(surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_path2_path3_path4_path5_path6_path7_data_put))
        .route("/jaxrs/surface/appdict/{appDictFlag}/appInfo/{appInfoFlag}/{path0}/{path1}/{path2}/{path3}/{path4}/{path5}/{path6}/{path7}/data", post(surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_path2_path3_path4_path5_path6_path7_data_post))
        .route("/jaxrs/surface/appdict/{appDictFlag}/appInfo/{appInfoFlag}/{path0}/{path1}/{path2}/{path3}/{path4}/{path5}/{path6}/{path7}/data", delete(surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_path2_path3_path4_path5_path6_path7_data_delete))
        .route("/jaxrs/surface/appdict/{appDictFlag}/appInfo/{appInfoFlag}/{path0}/{path1}/{path2}/{path3}/{path4}/{path5}/{path6}/{path7}/data/mockdeletetoget", get(surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_path2_path3_path4_path5_path6_path7_data_delete))
        .route("/jaxrs/surface/appdict/{appDictFlag}/appInfo/{appInfoFlag}/{path0}/{path1}/{path2}/{path3}/{path4}/{path5}/{path6}/{path7}/data/mockputtopost", post(surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_path2_path3_path4_path5_path6_path7_data_put))
        .route("/jaxrs/appinfo/list/all", get(appinfo_list_all))
        .route("/jaxrs/appinfo/list/has/document/type/{appType}", get(appinfo_list_has_document_type_appType))
        .route("/jaxrs/appinfo/list/manage", get(appinfo_list_manage))
        .route("/jaxrs/appinfo/list/manage/type/{appType}", get(appinfo_list_manage_type_appType))
        .route("/jaxrs/appinfo/list/user/publish", get(appinfo_list_user_publish))
        .route("/jaxrs/appinfo/list/user/publish/type/{appType}", get(appinfo_list_user_publish_type_appType))
        .route("/jaxrs/appinfo/list/user/publish/with/process", get(appinfo_list_user_publish_with_process))
        .route("/jaxrs/appinfo/list/user/view", get(appinfo_list_user_view))
        .route("/jaxrs/appinfo/list/user/view/all", get(appinfo_list_user_view_all))
        .route("/jaxrs/appinfo/list/user/view/all/type/{appType}", get(appinfo_list_user_view_all_type_appType))
        .route("/jaxrs/appinfo/list/user/view/article/type/{appType}", get(appinfo_list_user_view_article_type_appType))
        .route("/jaxrs/appinfo/list/user/view/data", get(appinfo_list_user_view_data))
        .route("/jaxrs/appinfo/list/user/view/data/type/{appType}", get(appinfo_list_user_view_data_type_appType))
        .route("/jaxrs/appinfo/flag", get(appinfo_flag))
        .route("/jaxrs/appinfo/{id}", get(appinfo_id))
        .route("/jaxrs/categoryinfo/filter/list/{id}/next/{count}/app/{appId}", put(categoryinfo_filter_list_id_next_count_app_appId))
        .route("/jaxrs/categoryinfo/filter/list/{id}/next/{count}/app/{appId}/mockputtopost", post(categoryinfo_filter_list_id_next_count_app_appId_mockputtopost))
        .route("/jaxrs/categoryinfo/filter/list/{id}/prev/{count}/app/{appId}", put(categoryinfo_filter_list_id_prev_count_app_appId))
        .route("/jaxrs/categoryinfo/filter/list/{id}/prev/{count}/app/{appId}/mockputtopost", post(categoryinfo_filter_list_id_prev_count_app_appId_mockputtopost))
        .route("/jaxrs/categoryinfo/list/all", get(categoryinfo_list_all))
        .route("/jaxrs/categoryinfo/list/manage/app/{appId}", get(categoryinfo_list_manage_app_appId))
        .route("/jaxrs/categoryinfo/list/publish/app/{appId}", get(categoryinfo_list_publish_app_appId))
        .route("/jaxrs/categoryinfo/list/view/app/{appId}", get(categoryinfo_list_view_app_appId))
        .route("/jaxrs/categoryinfo/list/view/app/{appId}/all", get(categoryinfo_list_view_app_appId_all))
        .route("/jaxrs/categoryinfo/list/view/app/{appId}/data", get(categoryinfo_list_view_app_appId_data))
        .route("/jaxrs/categoryinfo/flag", get(categoryinfo_flag))
        .route("/jaxrs/categoryinfo/{id}", get(categoryinfo_id))
        .route("/jaxrs/commend/list/paging/{page}/size/{size}", post(commend_list_paging_page_size_size))
        .route("/jaxrs/commend/{id}", get(commend_id))
        .route("/jaxrs/comment/{id}", get(comment_id))
        .route("/jaxrs/correlation/doc/{docId}", get(correlation_doc_docId).post(correlation_create_u3))
        .route("/jaxrs/correlation/list/doc/{docId}", get(correlation_list_doc_docId))
        .route("/jaxrs/correlation/list/doc/{docId}/site/{site}", get(correlation_list_doc_docId_site_site))
        .route("/jaxrs/design/appdict/list/appInfo/{appId}", get(design_appdict_list_appInfo_appId))
        .route("/jaxrs/design/appdict/list/paging/{page}/size/{size}", post(design_appdict_list_paging_page_size_size))
        .route("/jaxrs/design/appdict/{id}", get(design_appdict_id))
        .route("/jaxrs/file/list/appInfo/{appInfoFlag}", get(file_list_appInfo_appInfoFlag))
        .route("/jaxrs/file/flag", get(file_flag))
        .route("/jaxrs/file/{id}", get(file_id))
        .route("/jaxrs/fileinfo/list/all", get(fileinfo_list_all))
        .route("/jaxrs/fileinfo/list/document/{documentId}", get(fileinfo_list_document_documentId))
        .route("/jaxrs/form/list/all", get(form_list_all))
        .route("/jaxrs/form/list/app/{appId}", get(form_list_app_appId))
        .route("/jaxrs/form/list/formfield/appInfo/{appId}", get(form_list_formfield_appInfo_appId))
        .route("/jaxrs/form/list/{id}/formfield", get(form_list_id_formfield))
        .route("/jaxrs/form/{id}", get(form_id))
        .route("/jaxrs/form/v2/{id}", get(form_v2_id))
        .route("/jaxrs/log/list/app/{appId}", get(log_list_app_appId))
        .route("/jaxrs/log/list/category/{categoryId}", get(log_list_category_categoryId))
        .route("/jaxrs/log/list/document/{documentId}", get(log_list_document_documentId))
        .route("/jaxrs/log/list/level/{operationLevel}", get(log_list_level_operationLevel))
        .route("/jaxrs/log/{id}", get(log_id))
        .route("/jaxrs/output/list", get(output_list))
        .route("/jaxrs/permission/appInfo/{id}/manageable", get(permission_appInfo_id_manageable))
        .route("/jaxrs/permission/appInfo/{id}/managers", get(permission_appInfo_id_managers))
        .route("/jaxrs/permission/appInfo/{id}/publishers", get(permission_appInfo_id_publishers))
        .route("/jaxrs/permission/appInfo/{id}/viewers", get(permission_appInfo_id_viewers))
        .route("/jaxrs/permission/category/{id}/managers", get(permission_category_id_managers))
        .route("/jaxrs/permission/category/{id}/publishers", get(permission_category_id_publishers))
        .route("/jaxrs/permission/category/{id}/viewers", get(permission_category_id_viewers))
        .route("/jaxrs/permission/categoryInfo/{id}/manageable", get(permission_categoryInfo_id_manageable))
        .route("/jaxrs/permission/management/refresh/all", get(permission_management_refresh_all))
        .route("/jaxrs/script/list/app/{appId}/name/{name}", get(script_list_app_appId_name_name))
        .route("/jaxrs/script/list/app/{flag}", get(script_list_app_flag))
        .route("/jaxrs/script/list/paging/{page}/size/{size}", post(script_list_paging_page_size_size))
        .route("/jaxrs/script/{id}", get(script_id))
        .route("/jaxrs/searchfilter/list/archive/filter/category/{categoryId}", get(searchfilter_list_archive_filter_category_categoryId))
        .route("/jaxrs/searchfilter/list/draft/filter/category/{categoryId}", get(searchfilter_list_draft_filter_category_categoryId))
        .route("/jaxrs/searchfilter/list/publish/filter/category/{categoryId}", get(searchfilter_list_publish_filter_category_categoryId))
        .route("/jaxrs/anonymous/surface/appdict/list/appInfo/{appInfoFlag}", get(anonymous_surface_appdict_list_appInfo_appInfoFlag))
        .route("/jaxrs/surface/appdict/list/appInfo/{appInfoFlag}", get(surface_appdict_list_appInfo_appInfoFlag))
        .route("/jaxrs/templateform/list", get(templateform_list))
        .route("/jaxrs/templateform/list/category", get(templateform_list_category))
        .route("/jaxrs/templateform/list/category/mockputtopost", post(templateform_list_category_mockputtopost))
        .route("/jaxrs/uuid/random", get(uuid_random))
        .route("/jaxrs/view/list/all", get(view_list_all))
        .route("/jaxrs/view/list/app/{appId}", get(view_list_app_appId))
        .route("/jaxrs/view/list/category/{categoryId}", get(view_list_category_categoryId))
        .route("/jaxrs/view/list/form/{formId}", get(view_list_form_formId))
        .route("/jaxrs/view/{id}", get(view_id))
        .route("/jaxrs/viewcategory/list/all", get(viewcategory_list_all))
        .route("/jaxrs/viewcategory/list/category/{categoryId}", get(viewcategory_list_category_categoryId))
        .route("/jaxrs/viewcategory/list/view/{viewId}", get(viewcategory_list_view_viewId))
        .route("/jaxrs/viewcategory/{id}", get(viewcategory_id))
        .route("/jaxrs/viewfieldconfig/list/all", get(viewfieldconfig_list_all))
        .route("/jaxrs/viewfieldconfig/list/view/{viewId}", get(viewfieldconfig_list_view_viewId))
        .route("/jaxrs/viewfieldconfig/{id}", get(viewfieldconfig_id))
        .route("/jaxrs/viewrecord/list/install/log/paging/{page}/size/{size}", post(viewrecord_list_install_log_paging_page_size_size))
        .route("/jaxrs/image/encode/base64", post(image_encode_base64))
        .route("/jaxrs/image/encode/base64/size/{size}", post(image_encode_base64_size_size))
        .route("/jaxrs/image/resize/id/{id}/width/{width}/height/{height}", post(image_resize_id_id_width_width_height_height))
        .route("/jaxrs/input/compare", put(input_compare))
        .route("/jaxrs/input/compare/mockputtopost", post(input_compare_mockputtopost))
        .route("/jaxrs/input/cover", put(input_cover))
        .route("/jaxrs/input/cover/mockputtopost", post(input_cover_mockputtopost))
        .route("/jaxrs/input/create", put(input_create))
        .route("/jaxrs/input/create/mockputtopost", post(input_create_mockputtopost))
        .route("/jaxrs/input/prepare/cover", put(input_prepare_cover))
        .route("/jaxrs/input/prepare/cover/mockputtopost", post(input_prepare_cover_mockputtopost))
        .route("/jaxrs/input/prepare/create", put(input_prepare_create))
        .route("/jaxrs/input/prepare/create/mockputtopost", post(input_prepare_create_mockputtopost))
        .route("/jaxrs/cms_assemble_control/update/control/config", put(update_control_config))
        // ── plan002 U2: Java 对齐缺口端点 ──
        .route("/jaxrs/document/{id}", get(document_u2_get))
        .route("/jaxrs/document/{id}", delete(document_u2_delete))
        .route("/jaxrs/document", post(document_u2_create))
        .route("/jaxrs/document/{id}/update", post(document_u2_update))
        .route("/jaxrs/document/publish/{id}", put(document_u2_publish))
        .route("/jaxrs/document/publish/{id}/mockputtopost", post(document_u2_publish))
        .route("/jaxrs/document/publish/{id}/cancel", put(document_u2_publish_cancel))
        .route("/jaxrs/document/publish/{id}/cancel/mockputtopost", post(document_u2_publish_cancel))
        .route("/jaxrs/document/{id}/commend", get(document_u2_commend))
        .route("/jaxrs/document/{id}/uncommend", get(document_u2_uncommend))
        .route("/jaxrs/document/{id}/top", get(document_u2_top))
        .route("/jaxrs/document/{id}/unTop", get(document_u2_un_top))
        .route("/jaxrs/document/category/change", put(document_u2_category_change))
        .route("/jaxrs/document/category/change/mockputtopost", post(document_u2_category_change))
        .route("/jaxrs/document/{id}/document/data", get(document_u2_document_data))
        .route("/jaxrs/document/list/document", post(document_u2_list_document))
        .route("/jaxrs/document/document/fields", get(document_u2_fields))
        .route("/jaxrs/document/filter/count", put(document_u2_filter_count))
        .route("/jaxrs/document/filter/count/mockputtopost", post(document_u2_filter_count))
        .route("/jaxrs/comment", post(comment_u2_create))
        .route("/jaxrs/comment/{id}", delete(comment_u2_delete))
        .route("/jaxrs/comment/list/{page}/size/{size}", put(comment_u2_list_page_size_size))
        .route("/jaxrs/comment/list/{page}/size/{size}/mockputtopost", post(comment_u2_list_page_size_size))
        .route("/jaxrs/correlation/doc/{docId}/delete", post(correlation_u2_doc_delete))
        .route("/jaxrs/file", post(file_u2_create))
        .route("/jaxrs/file/{id}/mockputtopost", post(file_u2_update))
        .route("/jaxrs/fileinfo/{id}", delete(fileinfo_u2_delete))
        .route("/jaxrs/fileinfo/list/filter", post(fileinfo_u2_filter))
        .route("/jaxrs/fileinfo/copy/to/doc/{docId}", post(fileinfo_u2_copy_to_doc))
        .route("/jaxrs/fileinfo/replace/to/doc/{docId}", post(fileinfo_u2_replace_to_doc))
        .route("/jaxrs/form", post(form_u2_create))
        .route("/jaxrs/form/{id}", put(form_u2_update))
        .route("/jaxrs/form/{id}", delete(form_u2_delete))
        .route("/jaxrs/script", post(script_u2_create))
        .route("/jaxrs/script/{id}", put(script_u2_update))
        .route("/jaxrs/script/{id}", delete(script_u2_delete))
        .route("/jaxrs/script/list/manager", post(script_u2_list_manager))
        .route("/jaxrs/templateform", post(templateform_u2_create))
        .route("/jaxrs/templateform/{id}", delete(templateform_u2_delete))
        .route("/jaxrs/view", post(view_u2_create))
        .route("/jaxrs/view/{id}", put(view_u2_update))
        .route("/jaxrs/view/{id}", delete(view_u2_delete))
        .route("/jaxrs/viewcategory", post(viewcategory_u2_create))
        .route("/jaxrs/viewcategory/{id}", delete(viewcategory_u2_delete))
        .route("/jaxrs/viewfieldconfig", post(viewfieldconfig_u2_create))
        .route("/jaxrs/viewfieldconfig/{id}", put(viewfieldconfig_u2_update))
        .route("/jaxrs/viewfieldconfig/{id}", delete(viewfieldconfig_u2_delete))
        .route("/jaxrs/appinfo", post(appinfo_u2_create))
        .route("/jaxrs/appinfo/{id}", delete(appinfo_u2_delete))
        .route("/jaxrs/categoryinfo", post(categoryinfo_u2_create))
        .route("/jaxrs/categoryinfo/{id}", delete(categoryinfo_u2_delete))
        .route("/jaxrs/appinfo/{id}/permission", post(permission_u2_app_info))
        .route("/jaxrs/categoryinfo/{id}/permission", post(permission_u2_category_info))
        .route("/jaxrs/appconfig/{appId}", post(appconfig_u2_update))
        .route("/jaxrs/appconfig/{appId}", get(appconfig_u2_get))
        .route("/jaxrs/designer/search", post(designer_u2_search))
        // ── plan002 U2 收尾重试批次（U3）：Java canonical 对齐缺口 ──
        .route("/jaxrs/appinfo/erase/app/{id}", delete(appinfo_erase_app_id))
        .route("/jaxrs/appinfo/erase/app/{id}/mockdeletetoget", get(appinfo_erase_app_id_mockdeletetoget))
        .route("/jaxrs/appinfo/alias/{alias}", get(appinfo_alias_alias))
        .route("/jaxrs/appinfo/filter/list/{id}/next/{count}", put(appinfo_filter_list_id_next_count))
        .route("/jaxrs/appinfo/filter/list/{id}/next/{count}/mockputtopost", post(appinfo_filter_list_id_next_count_mockputtopost))
        .route("/jaxrs/appinfo/filter/list/{id}/prev/{count}", put(appinfo_filter_list_id_prev_count))
        .route("/jaxrs/appinfo/filter/list/{id}/prev/{count}/mockputtopost", post(appinfo_filter_list_id_prev_count_mockputtopost))
        .route("/jaxrs/appinfo/get/user/publish/{appId}", get(appinfo_get_user_publish_appId))
        .route("/jaxrs/appinfo/list/appType", get(appinfo_list_appType))
        .route("/jaxrs/appinfo/list/appType/manager", get(appinfo_list_appType_manager))
        .route("/jaxrs/appinfo/list/has/document", get(appinfo_list_has_document))
        .route("/jaxrs/appinfo/list/has/document/appType", get(appinfo_list_has_document_appType))
        .route("/jaxrs/appinfo/{id}/control", get(appinfo_id_control))
        .route("/jaxrs/appinfo/{id}/mockdeletetoget", get(appinfo_id_mockdeletetoget))
        .route("/jaxrs/appinfo/{id}/icon/size/{size}", post(appinfo_appId_icon_size_size))
        .route("/jaxrs/categoryinfo/erase/category/{id}", delete(categoryinfo_erase_category_id))
        .route("/jaxrs/categoryinfo/erase/category/{id}/mockdeletetoget", get(categoryinfo_erase_category_id_mockdeletetoget))
        .route("/jaxrs/categoryinfo/alias/{alias}", get(categoryinfo_alias_alias))
        .route("/jaxrs/categoryinfo/bind/{categoryId}/view", put(categoryinfo_bind_categoryId_view))
        .route("/jaxrs/categoryinfo/bind/{categoryId}/view/mockputtopost", post(categoryinfo_bind_categoryId_view_mockputtopost))
        .route("/jaxrs/categoryinfo/extContent", post(categoryinfo_ext_content_save_u3))
        .route("/jaxrs/categoryinfo/filter/list/{page}/size/{size}", put(categoryinfo_filter_list_page_size_size))
        .route("/jaxrs/categoryinfo/filter/list/{page}/size/{size}/mockputtopost", post(categoryinfo_filter_list_page_size_size_mockputtopost))
        .route("/jaxrs/categoryinfo/list/objects", post(categoryinfo_list_objects_u3))
        .route("/jaxrs/categoryinfo/{id}/control", get(categoryinfo_id_control))
        .route("/jaxrs/categoryinfo/{id}/execute/projection", post(categoryinfo_id_execute_projection))
        .route("/jaxrs/categoryinfo/{id}/mockdeletetoget", get(categoryinfo_id_mockdeletetoget))
        .route("/jaxrs/comment/list/{id}/next/{count}", put(comment_list_id_next_count))
        .route("/jaxrs/comment/list/{id}/next/{count}/mockputtopost", post(comment_list_id_next_count_mockputtopost))
        .route("/jaxrs/comment/list/{id}/prev/{count}", put(comment_list_id_prev_count))
        .route("/jaxrs/comment/list/{id}/prev/{count}/mockputtopost", post(comment_list_id_prev_count_mockputtopost))
        .route("/jaxrs/comment/{id}/commend", get(comment_commend_u3))
        .route("/jaxrs/comment/{id}/uncommend", get(comment_uncommend_u3))
        .route("/jaxrs/comment/{id}/mockdeletetoget", get(comment_id_mockdeletetoget))
        .route("/jaxrs/correlation/update/doc/{docId}", post(correlation_update_u3))
        .route("/jaxrs/design/appdict", post(design_appdict_create_u3))
        .route("/jaxrs/design/appdict/{id}", put(design_appdict_update_u3))
        .route("/jaxrs/design/appdict/{id}", delete(design_appdict_delete_u3))
        .route("/jaxrs/design/appdict/{id}/mockdeletetoget", get(design_appdict_id_mockdeletetoget))
        .route("/jaxrs/design/appdict/{id}/mockputtopost", post(design_appdict_id_mockputtopost))
        .route("/jaxrs/docpermission", post(permission_management_refresh_all))
        .route("/jaxrs/review/v2/search", post(review_v2_search_u3))
        .route("/jaxrs/document/achive/{id}", get(document_achive_u3))
        .route("/jaxrs/document/batch/data/modify", put(document_batch_modify_u3))
        .route("/jaxrs/document/batch/data/modify/mockputtopost", post(document_batch_modify_mock_u3))
        .route("/jaxrs/document/batch/status", get(document_batch_status_u3))
        .route("/jaxrs/document/batch/{id}", delete(document_batch_delete_u3))
        .route("/jaxrs/document/batch/{id}/mockdeletetoget", get(document_batch_delete_mock_u3))
        .route("/jaxrs/document/batch/{id}/status", get(document_batch_name_status_u3))
        .route("/jaxrs/document/cipher/publish/content", put(document_cipher_publish_workflow_u3))
        .route("/jaxrs/document/cipher/publish/content/mockputtopost", post(document_cipher_publish_workflow_mock_u3))
        .route("/jaxrs/document/cipher/{id}/permission/read/person/{person}", get(document_cipher_permission_read_u3))
        .route("/jaxrs/document/cipher/{id}/persist/view/record", post(document_cipher_id_persist_view_record))
        .route("/jaxrs/document/cipher/filter/list/{page}/size/{size}", put(document_cipher_filter_list_page_size_size))
        .route("/jaxrs/document/cipher/filter/list/{page}/size/{size}/mockputtopost", post(document_cipher_filter_list_page_size_size_mockputtopost))
        .route("/jaxrs/document/draft/list/{id}/next/{count}", put(document_draft_next_u3))
        .route("/jaxrs/document/draft/list/{id}/next/{count}/mockputtopost", post(document_draft_next_u3))
        .route("/jaxrs/document/filter/list/{id}/next/{count}", put(document_filter_next_u3))
        .route("/jaxrs/document/filter/list/{id}/next/{count}/mockputtopost", post(document_filter_next_u3))
        .route("/jaxrs/document/filter/list/{id}/prev/{count}", put(document_filter_prev_u3))
        .route("/jaxrs/document/filter/list/{id}/prev/{count}/mockputtopost", post(document_filter_prev_u3))
        .route("/jaxrs/document/filter/list/{page}/size/{size}", put(document_filter_paging_u3))
        .route("/jaxrs/document/filter/list/{page}/size/{size}/mockputtopost", post(document_filter_paging_u3))
        .route("/jaxrs/document/filter/list/{page}/size/{size}/manager", post(document_filter_paging_manager_u3))
        .route("/jaxrs/document/list/document/data", post(document_list_document_data_u3))
        .route("/jaxrs/document/publish/content", put(document_publish_content_u3))
        .route("/jaxrs/document/publish/content/mockputtopost", post(document_publish_content_mock_u3))
        .route("/jaxrs/document/{id}/control", get(document_control_u3))
        .route("/jaxrs/document/{id}/mockdeletetoget", get(document_u2_get))
        .route("/jaxrs/document/{id}/notify", post(document_notify_u3))
        .route("/jaxrs/document/{id}/permission/read", get(document_permission_read_u3))
        .route("/jaxrs/document/{id}/persons", get(document_persons_u3))
        .route("/jaxrs/document/{id}/publish/html", post(document_publish_html_u3))
        .route("/jaxrs/document/{id}/view", get(anonymous_document_id_view))
        .route("/jaxrs/file/list/{id}/next/{count}", get(file_list_id_next_count))
        .route("/jaxrs/file/list/{id}/prev/{count}", get(file_list_id_prev_count))
        .route("/jaxrs/file/{id}", delete(file_delete_u3))
        .route("/jaxrs/file/{id}", put(file_update_u3))
        .route("/jaxrs/file/{id}/mockdeletetoget", get(file_flag_mockdeletetoget))
        .route("/jaxrs/file/{id}/upload", post(file_id_upload))
        .route("/jaxrs/file/{id}/content", get(file_id_content))
        .route("/jaxrs/file/{id}/download", get(file_id_download))
        .route("/jaxrs/file/{flag}/appInfo/{appInfoFlag}", get(file_copy_u3))
        .route("/jaxrs/file/{flag}/appInfo/{appInfoFlag}/content", get(file_flag_appInfo_appInfoFlag_content))
        .route("/jaxrs/file/{flag}/appInfo/{appInfoFlag}/download", get(file_download_with_app_u3))
        .route("/jaxrs/fileinfo/batch/download/doc/{docId}/site/{site}", get(fileinfo_batch_download_doc_docId_site_site))
        .route("/jaxrs/fileinfo/download/document/{id}/stream", get(fileinfo_download_document_id_stream))
        .route("/jaxrs/fileinfo/download/transfer/flag/{flag}", get(fileinfo_download_transfer_flag_flag))
        .route("/jaxrs/fileinfo/{id}/binary/base64/{size}", get(fileinfo_binary_base64_u3))
        .route("/jaxrs/fileinfo/{id}/doc/{docId}/change/seqnumber/{seqNumber}", get(fileinfo_id_doc_docId_change_seqnumber_seqNumber))
        .route("/jaxrs/fileinfo/{id}/online/info", get(fileinfo_id_online_info))
        .route("/jaxrs/fileinfo/{id}/preview/pdf", get(fileinfo_id_preview_pdf))
        .route("/jaxrs/fileinfo/edit/{id}/doc/{docId}", put(fileinfo_edit_id_doc_docId))
        .route("/jaxrs/fileinfo/edit/{id}/doc/{docId}/mockputtopost", post(fileinfo_edit_id_doc_docId_mockputtopost))
        .route("/jaxrs/fileinfo/update/document/{docId}/attachment/{id}", post(fileinfo_update_document_docId_attachment_id))
        .route("/jaxrs/fileinfo/update/document/{docId}/attachment/{id}/callback/{callback}", post(fileinfo_update_document_docId_attachment_id_callback_callback))
        .route("/jaxrs/fileinfo/update/{id}/content", post(fileinfo_update_id_content))
        .route("/jaxrs/fileinfo/upload/doc/{docId}/save/as/{flag}", post(fileinfo_upload_doc_docId_save_as_flag))
        .route("/jaxrs/fileinfo/upload/document/{docId}/callback/{callback}", post(fileinfo_upload_document_docId_callback_callback))
        .route("/jaxrs/fileinfo/upload/with/url", post(fileinfo_upload_with_url_u3))
        .route("/jaxrs/form/v2/lookup/document/{docId}", get(form_v2_lookup_document_docId))
        .route("/jaxrs/form/v2/lookup/document/{docId}/mobile", get(form_v2_lookup_document_docId_mobile))
        .route("/jaxrs/form/v2/{id}/mobile", get(form_v2_id_mobile))
        .route("/jaxrs/form/{id}/appinfo/{appFlag}", get(form_get_with_appinfo_u3))
        .route("/jaxrs/form/{id}/mockdeletetoget", get(form_id_mockdeletetoget))
        .route("/jaxrs/form/{id}/mockputtopost", post(form_id_mockputtopost))
        .route("/jaxrs/form/filter/list/{id}/next/{count}/app/{appId}", put(form_filter_list_id_next_count_app_appId))
        .route("/jaxrs/form/filter/list/{id}/next/{count}/app/{appId}/mockputtopost", post(form_filter_list_id_next_count_app_appId_mockputtopost))
        .route("/jaxrs/form/filter/list/{id}/prev/{count}/app/{appId}", put(form_filter_list_id_prev_count_app_appId))
        .route("/jaxrs/form/filter/list/{id}/prev/{count}/app/{appId}/mockputtopost", post(form_filter_list_id_prev_count_app_appId_mockputtopost))
        .route("/jaxrs/formversion/{id}", get(formversion_id))
        .route("/jaxrs/formversion/list/form/{formId}", get(formversion_list_form_formId))
        .route("/jaxrs/scriptversion/{id}", get(scriptversion_id))
        .route("/jaxrs/scriptversion/list/script/{scriptId}", get(scriptversion_list_script_scriptId))
        .route("/jaxrs/log/filter/list/{id}/next/{count}", post(log_filter_list_id_next_count))
        .route("/jaxrs/log/filter/list/{id}/prev/{count}", post(log_filter_list_id_prev_count))
        .route("/jaxrs/log/list/filter/{page}/size/{size}", post(log_list_filter_page_size_size))
        .route("/jaxrs/output/{appInfoFlag}/select", put(output_appInfoFlag_select))
        .route("/jaxrs/output/{appInfoFlag}/select/mockputtopost", post(output_appInfoFlag_select_mockputtopost))
        .route("/jaxrs/permission/management/refresh/category/{categoryId}", get(permission_management_refresh_category_categoryId))
        .route("/jaxrs/permission/manager/appInfo/{id}", post(permission_save_manager_app_u3))
        .route("/jaxrs/permission/manager/categoryInfo/{id}", post(permission_save_manager_category_u3))
        .route("/jaxrs/permission/publisher/appInfo/{id}", post(permission_save_publisher_app_u3))
        .route("/jaxrs/permission/publisher/categoryInfo/{id}", post(permission_save_publisher_category_u3))
        .route("/jaxrs/permission/viewer/appInfo/{id}", post(permission_save_viewer_app_u3))
        .route("/jaxrs/permission/viewer/categoryInfo/{id}", post(permission_save_viewer_category_u3))
        .route("/jaxrs/script/list/{id}/next/{count}", get(script_list_id_next_count))
        .route("/jaxrs/script/list/{id}/prev/{count}", get(script_list_id_prev_count))
        .route("/jaxrs/script/{uniqueName}/app/{flag}", post(script_post_nested_u3))
        .route("/jaxrs/script/{uniqueName}/app/{flag}/imported", get(script_uniqueName_app_flag_imported))
        .route("/jaxrs/script/{uniqueName}/appInfo/{appFlag}", post(script_load_u3))
        .route("/jaxrs/script/{id}/mockputtopost", post(script_id_mockputtopost))
        .route("/jaxrs/script/{id}/mockdeletetoget", get(script_id_mockdeletetoget))
        .route("/jaxrs/templateform/{id}", get(templateform_id))
        .route("/jaxrs/templateform/{id}/mockdeletetoget", get(templateform_id_mockdeletetoget))
        .route("/jaxrs/templateform/list/category", put(templateform_list_category))
        .route("/jaxrs/view/viewdata/list/{id}/next/{count}", post(view_viewdata_list_id_next_count))
        .route("/jaxrs/view/{id}/mockdeletetoget", get(view_id_mockdeletetoget))
        .route("/jaxrs/view/{id}/mockputtopost", post(view_id_mockputtopost))
        .route("/jaxrs/viewcategory/{id}/mockdeletetoget", get(viewcategory_id_mockdeletetoget))
        .route("/jaxrs/viewfieldconfig/{id}/mockdeletetoget", get(viewfieldconfig_id_mockdeletetoget))
        .route("/jaxrs/viewfieldconfig/{id}/mockputtopost", post(viewfieldconfig_id_mockputtopost))
        .route("/jaxrs/viewrecord/document/{docId}/filter/list/{id}/next/{count}", get(viewrecord_document_docId_filter_list_id_next_count))
        .route("/jaxrs/viewrecord/document/{docId}/has/view", get(viewrecord_document_docId_has_view))
        .route("/jaxrs/viewrecord/person/{person}", get(viewrecord_by_person_u3))
        .route("/jaxrs/viewrecord/unread", put(viewrecord_unread_u3))
        .route("/jaxrs/viewrecord/unread/mockputtopost", post(viewrecord_unread_u3))
        .route("/jaxrs/anonymous/document/filter/list/{id}/next/{count}", put(anonymous_document_filter_list_id_next_count))
        .route("/jaxrs/anonymous/document/filter/list/{id}/next/{count}/mockputtopost", post(anonymous_document_filter_list_id_next_count_mockputtopost))
        .route("/jaxrs/anonymous/document/filter/list/{page}/size/{size}", put(anonymous_document_filter_list_page_size_size))
        .route("/jaxrs/anonymous/document/filter/list/{page}/size/{size}/mockputtopost", post(anonymous_document_filter_list_page_size_size_mockputtopost))
        .route("/jaxrs/anonymous/form/{id}", get(anonymous_form_id))
        .route("/jaxrs/anonymous/form/v2/{id}", get(anonymous_form_v2_id))
        .route("/jaxrs/anonymous/form/v2/{id}/mobile", get(anonymous_form_v2_id_mobile))
        .route("/jaxrs/anonymous/form/v2/lookup/document/{docId}", get(anonymous_form_v2_lookup_document_docId))
        .route("/jaxrs/anonymous/form/v2/lookup/document/{docId}/mobile", get(anonymous_form_v2_lookup_document_docId_mobile))
        .route("/jaxrs/anonymous/fileinfo/list/document/{documentId}", get(anonymous_fileinfo_list_document_documentId))
        .route("/jaxrs/anonymous/fileinfo/{id}/document/{documentId}", get(fileinfo_id_document_documentId))
        .route("/jaxrs/anonymous/fileinfo/download/document/{id}/stream", get(anonymous_fileinfo_download_document_id_stream))
        // ── plan002 U2 收尾：Java 缺口补齐（export/import appInfo）──
        .route("/jaxrs/export/appInfo/{appInfoFlag}", get(export_app_info_app_info_flag))
        .route("/jaxrs/import/appInfo/{appInfoFlag}", get(import_app_info_app_info_flag))
.layer(Extension(pool))
}

pub fn cms_assemble_control_router(pool: Pool) -> Router {
    router(pool)
}

