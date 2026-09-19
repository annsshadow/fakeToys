use axum::{
    extract::Extension,
    routing::{delete, get, post, put},
    Router,
};
use deadpool_postgres::Pool;

use crate::{
    anonymous_document_filter_list_id_next_count,
    anonymous_document_filter_list_id_next_count_mockputtopost,
    anonymous_document_filter_list_page_size_size,
    anonymous_document_filter_list_page_size_size_mockputtopost, anonymous_document_id_view,
    anonymous_fileinfo_download_document_id, anonymous_fileinfo_download_document_id_stream,
    anonymous_fileinfo_list_document_documentId, anonymous_form_id, anonymous_form_v2_id,
    anonymous_form_v2_id_mobile, anonymous_form_v2_lookup_document_docId,
    anonymous_form_v2_lookup_document_docId_mobile,
    anonymous_surface_appdict_appDictFlag_appInfo_appInfoFlag,
    anonymous_surface_appdict_appDictFlag_appInfo_appInfoFlag_data,
    anonymous_surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_data,
    anonymous_surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_data,
    anonymous_surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_path2_data,
    anonymous_surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_path2_path3_data,
    anonymous_surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_path2_path3_path4_data,
    anonymous_surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_path2_path3_path4_path5_data,
    anonymous_surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_path2_path3_path4_path5_path6_data,
    anonymous_surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_path2_path3_path4_path5_path6_path7_data,
    anonymous_surface_appdict_list_appInfo_appInfoFlag, appconfig_u2_get, appconfig_u2_update,
    appinfo_alias_alias, appinfo_appId_icon_size_size, appinfo_erase_app_id,
    appinfo_erase_app_id_mockdeletetoget, appinfo_filter_list_id_next_count,
    appinfo_filter_list_id_next_count_mockputtopost, appinfo_filter_list_id_prev_count,
    appinfo_filter_list_id_prev_count_mockputtopost, appinfo_flag, appinfo_get_user_publish_appId,
    appinfo_id, appinfo_id_control, appinfo_id_mockdeletetoget, appinfo_list_all,
    appinfo_list_appType, appinfo_list_appType_manager, appinfo_list_has_document,
    appinfo_list_has_document_appType, appinfo_list_has_document_type_appType, appinfo_list_manage,
    appinfo_list_manage_type_appType, appinfo_list_user_publish,
    appinfo_list_user_publish_type_appType, appinfo_list_user_publish_with_process,
    appinfo_list_user_view, appinfo_list_user_view_all, appinfo_list_user_view_all_type_appType,
    appinfo_list_user_view_article_type_appType, appinfo_list_user_view_data,
    appinfo_list_user_view_data_type_appType, appinfo_u2_create, appinfo_u2_delete, application_id,
    categoryinfo_alias_alias, categoryinfo_bind_categoryId_view,
    categoryinfo_bind_categoryId_view_mockputtopost, categoryinfo_erase_category_id,
    categoryinfo_erase_category_id_mockdeletetoget, categoryinfo_ext_content_save_u3,
    categoryinfo_filter_list_id_next_count_app_appId,
    categoryinfo_filter_list_id_next_count_app_appId_mockputtopost,
    categoryinfo_filter_list_id_prev_count_app_appId,
    categoryinfo_filter_list_id_prev_count_app_appId_mockputtopost,
    categoryinfo_filter_list_page_size_size, categoryinfo_filter_list_page_size_size_mockputtopost,
    categoryinfo_flag, categoryinfo_id, categoryinfo_id_control,
    categoryinfo_id_execute_projection, categoryinfo_id_mockdeletetoget, categoryinfo_list_all,
    categoryinfo_list_manage_app_appId, categoryinfo_list_objects_u3,
    categoryinfo_list_publish_app_appId, categoryinfo_list_view_app_appId,
    categoryinfo_list_view_app_appId_all, categoryinfo_list_view_app_appId_data,
    categoryinfo_u2_create, categoryinfo_u2_delete, cms_control_dict_list, cms_control_form_list,
    cms_control_view_list, cms_control_xform_list, commend_id, commend_list_paging,
    commend_list_paging_page_size_size, comment_commend_u3, comment_id, comment_id_mockdeletetoget,
    comment_list_id_next_count, comment_list_id_next_count_mockputtopost,
    comment_list_id_prev_count, comment_list_id_prev_count_mockputtopost, comment_u2_create,
    comment_u2_delete, comment_u2_list_page_size_size, comment_uncommend_u3, correlation_create_u3,
    correlation_doc_docId, correlation_list_doc_docId, correlation_list_doc_docId_site_site,
    correlation_u2_doc_delete, correlation_update_u3, data_document_id,
    data_document_id_array_data, data_document_id_create, data_document_id_delete,
    data_document_id_mockdeletetoget, data_document_id_mockputtopost, data_document_id_path0,
    data_document_id_path0_create, data_document_id_path0_delete,
    data_document_id_path0_mockdeletetoget, data_document_id_path0_mockputtopost,
    data_document_id_path0_path1, data_document_id_path0_path1_create,
    data_document_id_path0_path1_delete, data_document_id_path0_path1_mockdeletetoget,
    data_document_id_path0_path1_mockputtopost, data_document_id_path0_path1_path2,
    data_document_id_path0_path1_path2_create, data_document_id_path0_path1_path2_delete,
    data_document_id_path0_path1_path2_mockdeletetoget,
    data_document_id_path0_path1_path2_mockputtopost, data_document_id_path0_path1_path2_path3,
    data_document_id_path0_path1_path2_path3_create,
    data_document_id_path0_path1_path2_path3_delete,
    data_document_id_path0_path1_path2_path3_mockdeletetoget,
    data_document_id_path0_path1_path2_path3_mockputtopost,
    data_document_id_path0_path1_path2_path3_path4,
    data_document_id_path0_path1_path2_path3_path4_create,
    data_document_id_path0_path1_path2_path3_path4_delete,
    data_document_id_path0_path1_path2_path3_path4_mockdeletetoget,
    data_document_id_path0_path1_path2_path3_path4_mockputtopost,
    data_document_id_path0_path1_path2_path3_path4_path5,
    data_document_id_path0_path1_path2_path3_path4_path5_create,
    data_document_id_path0_path1_path2_path3_path4_path5_delete,
    data_document_id_path0_path1_path2_path3_path4_path5_mockdeletetoget,
    data_document_id_path0_path1_path2_path3_path4_path5_mockputtopost,
    data_document_id_path0_path1_path2_path3_path4_path5_path6,
    data_document_id_path0_path1_path2_path3_path4_path5_path6_create,
    data_document_id_path0_path1_path2_path3_path4_path5_path6_delete,
    data_document_id_path0_path1_path2_path3_path4_path5_path6_mockdeletetoget,
    data_document_id_path0_path1_path2_path3_path4_path5_path6_mockputtopost,
    data_document_id_path0_path1_path2_path3_path4_path5_path6_path7,
    data_document_id_path0_path1_path2_path3_path4_path5_path6_path7_create,
    data_document_id_path0_path1_path2_path3_path4_path5_path6_path7_delete,
    data_document_id_path0_path1_path2_path3_path4_path5_path6_path7_mockdeletetoget,
    data_document_id_path0_path1_path2_path3_path4_path5_path6_path7_mockputtopost,
    data_document_id_path0_path1_path2_path3_path4_path5_path6_path7_update,
    data_document_id_path0_path1_path2_path3_path4_path5_path6_update,
    data_document_id_path0_path1_path2_path3_path4_path5_update,
    data_document_id_path0_path1_path2_path3_path4_update,
    data_document_id_path0_path1_path2_path3_update, data_document_id_path0_path1_path2_update,
    data_document_id_path0_path1_update, data_document_id_path0_update, data_document_id_update,
    design_appdict_create_u3, design_appdict_delete_u3, design_appdict_id,
    design_appdict_id_mockdeletetoget, design_appdict_id_mockputtopost,
    design_appdict_list_appInfo_appId, design_appdict_list_paging_page_size_size,
    design_appdict_update_u3, designer_u2_search, dict_create, dict_delete, dict_save,
    document_achive_u3, document_batch_delete_mock_u3, document_batch_delete_u3,
    document_batch_modify_mock_u3, document_batch_modify_u3, document_batch_name_status_u3,
    document_batch_status_u3, document_cipher_filter_list_page_size_size,
    document_cipher_filter_list_page_size_size_mockputtopost,
    document_cipher_id_persist_view_record, document_cipher_permission_read_u3,
    document_cipher_publish_workflow_mock_u3, document_cipher_publish_workflow_u3,
    document_control_u3, document_draft_next_u3, document_filter_next_u3,
    document_filter_paging_manager_u3, document_filter_paging_u3, document_filter_prev_u3,
    document_id_view_count, document_list_document_data_u3, document_notify_u3,
    document_permission_read_u3, document_persons_u3, document_publish_content_mock_u3,
    document_publish_content_u3, document_publish_html_u3, document_search,
    document_u2_category_change, document_u2_commend, document_u2_create, document_u2_delete,
    document_u2_document_data, document_u2_fields, document_u2_filter_count, document_u2_get,
    document_u2_list_document, document_u2_publish, document_u2_publish_cancel, document_u2_top,
    document_u2_un_top, document_u2_uncommend, document_u2_update, export_app_info_app_info_flag,
    file_copy_u3, file_delete_u3, file_download_with_app_u3, file_flag,
    file_flag_appInfo_appInfoFlag_content, file_flag_mockdeletetoget, file_id, file_id_content,
    file_id_download, file_id_upload, file_list_appInfo_appInfoFlag, file_list_id_next_count,
    file_list_id_prev_count, file_u2_create, file_u2_update, file_update_u3,
    fileinfo_batch_download_doc_docId_site_site, fileinfo_binary_base64_u3,
    fileinfo_download_document_id, fileinfo_download_document_id_stream,
    fileinfo_download_transfer_flag_flag, fileinfo_edit_id_doc_docId,
    fileinfo_edit_id_doc_docId_mockputtopost, fileinfo_id,
    fileinfo_id_doc_docId_change_seqnumber_seqNumber, fileinfo_id_document_documentId,
    fileinfo_id_mockdeletetoget, fileinfo_id_online_info, fileinfo_id_preview_pdf,
    fileinfo_list_all, fileinfo_list_document_documentId, fileinfo_u2_copy_to_doc,
    fileinfo_u2_delete, fileinfo_u2_filter, fileinfo_u2_replace_to_doc,
    fileinfo_update_document_docId_attachment_id,
    fileinfo_update_document_docId_attachment_id_callback_callback, fileinfo_update_id_content,
    fileinfo_upload_doc_docId_save_as_flag, fileinfo_upload_document_docId,
    fileinfo_upload_document_docId_callback_callback, fileinfo_upload_with_url_u3, form_create,
    form_delete, form_filter_list_id_next_count_app_appId,
    form_filter_list_id_next_count_app_appId_mockputtopost,
    form_filter_list_id_prev_count_app_appId,
    form_filter_list_id_prev_count_app_appId_mockputtopost, form_get_with_appinfo_u3, form_id,
    form_id_mockdeletetoget, form_id_mockputtopost, form_list_all, form_list_app_appId,
    form_list_formfield_appInfo_appId, form_list_id_formfield, form_save, form_u2_create,
    form_u2_delete, form_u2_update, form_v2_id, form_v2_id_mobile, form_v2_lookup_document_docId,
    form_v2_lookup_document_docId_mobile, formversion_id, formversion_list_form_formId,
    get_control_config, image_encode_base64, image_encode_base64_size_size,
    image_resize_id_id_width_width_height_height, import_app_info_app_info_flag, input_compare,
    input_compare_mockputtopost, input_cover, input_cover_mockputtopost, input_create,
    input_create_mockputtopost, input_prepare_cover, input_prepare_cover_mockputtopost,
    input_prepare_create, input_prepare_create_mockputtopost, list_control_sections,
    log_filter_list_id_next_count, log_filter_list_id_prev_count, log_id, log_list,
    log_list_app_appId, log_list_category_categoryId, log_list_document_documentId,
    log_list_filter_page_size_size, log_list_level_operationLevel, output_appInfoFlag_select,
    output_appInfoFlag_select_mockputtopost, output_list, permission_appInfo_id_manageable,
    permission_appInfo_id_managers, permission_appInfo_id_publishers,
    permission_appInfo_id_viewers, permission_categoryInfo_id_manageable,
    permission_category_id_managers, permission_category_id_publishers,
    permission_category_id_viewers, permission_management_refresh_all,
    permission_management_refresh_category_categoryId, permission_save_manager_app_u3,
    permission_save_manager_category_u3, permission_save_publisher_app_u3,
    permission_save_publisher_category_u3, permission_save_viewer_app_u3,
    permission_save_viewer_category_u3, permission_u2_app_info, permission_u2_category_info,
    queryview_flag_definition, review_v2_search_u3, script_id, script_id_mockdeletetoget,
    script_id_mockputtopost, script_list_app_appId_name_name, script_list_app_flag,
    script_list_id_next_count, script_list_id_prev_count, script_list_paging_page_size_size,
    script_load_u3, script_post_nested_u3, script_u2_create, script_u2_delete,
    script_u2_list_manager, script_u2_update, script_uniqueName_app_flag_imported,
    scriptversion_id, scriptversion_list_script_scriptId,
    searchfilter_list_archive_filter_category_categoryId,
    searchfilter_list_draft_filter_category_categoryId,
    searchfilter_list_publish_filter_category_categoryId,
    surface_appdict_appDictFlag_appInfo_appInfoFlag,
    surface_appdict_appDictFlag_appInfo_appInfoFlag_data,
    surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_data,
    surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_data_delete,
    surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_data_post,
    surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_data_put,
    surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_data,
    surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_data_delete,
    surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_data_post,
    surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_data_put,
    surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_path2_data,
    surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_path2_data_delete,
    surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_path2_data_post,
    surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_path2_data_put,
    surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_path2_path3_data,
    surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_path2_path3_data_delete,
    surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_path2_path3_data_post,
    surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_path2_path3_data_put,
    surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_path2_path3_path4_data,
    surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_path2_path3_path4_data_delete,
    surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_path2_path3_path4_data_post,
    surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_path2_path3_path4_data_put,
    surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_path2_path3_path4_path5_data,
    surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_path2_path3_path4_path5_data_delete,
    surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_path2_path3_path4_path5_data_post,
    surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_path2_path3_path4_path5_data_put,
    surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_path2_path3_path4_path5_path6_data,
    surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_path2_path3_path4_path5_path6_data_delete,
    surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_path2_path3_path4_path5_path6_data_post,
    surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_path2_path3_path4_path5_path6_data_put,
    surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_path2_path3_path4_path5_path6_path7_data,
    surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_path2_path3_path4_path5_path6_path7_data_delete,
    surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_path2_path3_path4_path5_path6_path7_data_post,
    surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_path2_path3_path4_path5_path6_path7_data_put,
    surface_appdict_appDictFlag_appInfo_appInfoFlag_update,
    surface_appdict_list_appInfo_appInfoFlag, templateform_create, templateform_delete,
    templateform_id, templateform_id_mockdeletetoget, templateform_list,
    templateform_list_category, templateform_list_category_mockputtopost, templateform_save,
    templateform_u2_create, templateform_u2_delete, update_control_config, uuid_random,
    view_create, view_delete, view_id, view_id_mockdeletetoget, view_id_mockputtopost,
    view_list_all, view_list_app_appId, view_list_category_categoryId, view_list_form_formId,
    view_save, view_u2_create, view_u2_delete, view_u2_update, view_viewdata_list_id_next_count,
    viewcategory_id, viewcategory_id_mockdeletetoget, viewcategory_list_all,
    viewcategory_list_category_categoryId, viewcategory_list_view_viewId, viewcategory_u2_create,
    viewcategory_u2_delete, viewfieldconfig_id, viewfieldconfig_id_mockdeletetoget,
    viewfieldconfig_id_mockputtopost, viewfieldconfig_list_all, viewfieldconfig_list_view_viewId,
    viewfieldconfig_u2_create, viewfieldconfig_u2_delete, viewfieldconfig_u2_update,
    viewrecord_by_person_u3, viewrecord_document_docId_filter_list_id_next_count,
    viewrecord_document_docId_has_view, viewrecord_list_install_log_paging_page_size_size,
    viewrecord_unread_u3, xform_create, xform_delete, xform_save,
};

pub fn router(pool: deadpool_postgres::Pool) -> axum::Router {
    // ── Rust 自有扩展端点（非 o2server 口径，保留作向后兼容）：
    //   /cms_assemble_control/get|update/control/config、/list/control/sections（控制面配置）
    //   /commend/list/paging/{docId}（按文档查 commend）、/queryview/*、/application/{id}、
    //   /cms_assemble_control/document/search、GET /fileinfo/{id}、/appinfo/flag、
    //   /categoryinfo/flag、/file/flag、GET /correlation/doc/{docId}
    Router::new()
        .route("/api/cms_assemble_control/get/control/config", get(get_control_config))
        .route("/api/cms_assemble_control/list/control/sections", get(list_control_sections))
        .route("/api/cms_assemble_control/update/control/config", get(update_control_config))
        .route("/api/document/{id}/view/count", get(document_id_view_count))
        .route("/api/commend/list/paging/{docId}", get(commend_list_paging))
        .route("/api/queryview/flag/{view_flag}/definition/{query_flag}", get(queryview_flag_definition))
        .route("/api/application/{id}", get(application_id))
        .route("/api/cms_assemble_control/document/search", get(document_search))
        .route("/api/anonymous/document/{id}/view", get(anonymous_document_id_view))
        // ── cms/assemble/control/* 斜杠路径家族（前端 o2server 口径补齐，查真实表）──
        .route("/api/cms/assemble/control/dict/list", get(cms_control_dict_list))
        .route("/api/cms/assemble/control/form/list", get(cms_control_form_list))
        .route("/api/cms/assemble/control/view/list", get(cms_control_view_list))
        .route("/api/cms/assemble/control/xform/list", get(cms_control_xform_list))
        // ── dict/form/view/xform 家族 CRUD（通用参数化写，shared::crud 白名单）──
        .route(
            "/api/cms/assemble/control/dict/create",
            post(dict_create),
        )
        .route("/api/cms/assemble/control/dict/save/{id}", put(dict_save))
        .route("/api/cms/assemble/control/dict/save/{id}", post(dict_save))
        .route("/api/cms/assemble/control/dict/delete/{id}", delete(dict_delete))
        .route("/api/cms/assemble/control/dict/delete/{id}", post(dict_delete))
        .route(
            "/api/cms/assemble/control/form/create",
            post(form_create),
        )
        .route("/api/cms/assemble/control/form/save/{id}", put(form_save))
        .route("/api/cms/assemble/control/form/save/{id}", post(form_save))
        .route("/api/cms/assemble/control/form/delete/{id}", delete(form_delete))
        .route("/api/cms/assemble/control/form/delete/{id}", post(form_delete))
        .route(
            "/api/cms/assemble/control/view/create",
            post(view_create),
        )
        .route("/api/cms/assemble/control/view/save/{id}", put(view_save))
        .route("/api/cms/assemble/control/view/save/{id}", post(view_save))
        .route("/api/cms/assemble/control/view/delete/{id}", delete(view_delete))
        .route("/api/cms/assemble/control/view/delete/{id}", post(view_delete))
        .route(
            "/api/cms/assemble/control/xform/create",
            post(xform_create),
        )
        .route(
            "/api/cms/assemble/control/xform/save/{id}",
            put(xform_save),
        )
        .route(
            "/api/cms/assemble/control/xform/save/{id}",
            post(xform_save),
        )
        .route(
            "/api/cms/assemble/control/xform/delete/{id}",
            delete(xform_delete),
        )
        .route(
            "/api/cms/assemble/control/xform/delete/{id}",
            post(xform_delete),
        )
        // ── data/document 家族（o2server DataAction 对齐：{path0}..{path7} 通配 + 全动词）──
        .route("/api/data/document/{id}", get(data_document_id))
        .route("/api/data/document/{id}", put(data_document_id_update))
        .route("/api/data/document/{id}", post(data_document_id_create))
        .route("/api/data/document/{id}", delete(data_document_id_delete))
        .route("/api/data/document/{id}/array/data", post(data_document_id_array_data))
        .route("/api/data/document/{id}/mockdeletetoget", get(data_document_id_mockdeletetoget))
        .route("/api/data/document/{id}/mockputtopost", post(data_document_id_mockputtopost))
        .route("/api/data/document/{id}/{path0}", get(data_document_id_path0))
        .route("/api/data/document/{id}/{path0}", put(data_document_id_path0_update))
        .route("/api/data/document/{id}/{path0}", post(data_document_id_path0_create))
        .route("/api/data/document/{id}/{path0}", delete(data_document_id_path0_delete))
        .route("/api/data/document/{id}/{path0}/mockdeletetoget", get(data_document_id_path0_mockdeletetoget))
        .route("/api/data/document/{id}/{path0}/mockputtopost", post(data_document_id_path0_mockputtopost))
        .route("/api/data/document/{id}/{path0}/{path1}", get(data_document_id_path0_path1))
        .route("/api/data/document/{id}/{path0}/{path1}", put(data_document_id_path0_path1_update))
        .route("/api/data/document/{id}/{path0}/{path1}", post(data_document_id_path0_path1_create))
        .route("/api/data/document/{id}/{path0}/{path1}", delete(data_document_id_path0_path1_delete))
        .route("/api/data/document/{id}/{path0}/{path1}/mockdeletetoget", get(data_document_id_path0_path1_mockdeletetoget))
        .route("/api/data/document/{id}/{path0}/{path1}/mockputtopost", post(data_document_id_path0_path1_mockputtopost))
        .route("/api/data/document/{id}/{path0}/{path1}/{path2}", get(data_document_id_path0_path1_path2))
        .route("/api/data/document/{id}/{path0}/{path1}/{path2}", put(data_document_id_path0_path1_path2_update))
        .route("/api/data/document/{id}/{path0}/{path1}/{path2}", post(data_document_id_path0_path1_path2_create))
        .route("/api/data/document/{id}/{path0}/{path1}/{path2}", delete(data_document_id_path0_path1_path2_delete))
        .route("/api/data/document/{id}/{path0}/{path1}/{path2}/mockdeletetoget", get(data_document_id_path0_path1_path2_mockdeletetoget))
        .route("/api/data/document/{id}/{path0}/{path1}/{path2}/mockputtopost", post(data_document_id_path0_path1_path2_mockputtopost))
        .route("/api/data/document/{id}/{path0}/{path1}/{path2}/{path3}", get(data_document_id_path0_path1_path2_path3))
        .route("/api/data/document/{id}/{path0}/{path1}/{path2}/{path3}", put(data_document_id_path0_path1_path2_path3_update))
        .route("/api/data/document/{id}/{path0}/{path1}/{path2}/{path3}", post(data_document_id_path0_path1_path2_path3_create))
        .route("/api/data/document/{id}/{path0}/{path1}/{path2}/{path3}", delete(data_document_id_path0_path1_path2_path3_delete))
        .route("/api/data/document/{id}/{path0}/{path1}/{path2}/{path3}/mockdeletetoget", get(data_document_id_path0_path1_path2_path3_mockdeletetoget))
        .route("/api/data/document/{id}/{path0}/{path1}/{path2}/{path3}/mockputtopost", post(data_document_id_path0_path1_path2_path3_mockputtopost))
        .route("/api/data/document/{id}/{path0}/{path1}/{path2}/{path3}/{path4}", get(data_document_id_path0_path1_path2_path3_path4))
        .route("/api/data/document/{id}/{path0}/{path1}/{path2}/{path3}/{path4}", put(data_document_id_path0_path1_path2_path3_path4_update))
        .route("/api/data/document/{id}/{path0}/{path1}/{path2}/{path3}/{path4}", post(data_document_id_path0_path1_path2_path3_path4_create))
        .route("/api/data/document/{id}/{path0}/{path1}/{path2}/{path3}/{path4}", delete(data_document_id_path0_path1_path2_path3_path4_delete))
        .route("/api/data/document/{id}/{path0}/{path1}/{path2}/{path3}/{path4}/mockdeletetoget", get(data_document_id_path0_path1_path2_path3_path4_mockdeletetoget))
        .route("/api/data/document/{id}/{path0}/{path1}/{path2}/{path3}/{path4}/mockputtopost", post(data_document_id_path0_path1_path2_path3_path4_mockputtopost))
        .route("/api/data/document/{id}/{path0}/{path1}/{path2}/{path3}/{path4}/{path5}", get(data_document_id_path0_path1_path2_path3_path4_path5))
        .route("/api/data/document/{id}/{path0}/{path1}/{path2}/{path3}/{path4}/{path5}", put(data_document_id_path0_path1_path2_path3_path4_path5_update))
        .route("/api/data/document/{id}/{path0}/{path1}/{path2}/{path3}/{path4}/{path5}", post(data_document_id_path0_path1_path2_path3_path4_path5_create))
        .route("/api/data/document/{id}/{path0}/{path1}/{path2}/{path3}/{path4}/{path5}", delete(data_document_id_path0_path1_path2_path3_path4_path5_delete))
        .route("/api/data/document/{id}/{path0}/{path1}/{path2}/{path3}/{path4}/{path5}/mockdeletetoget", get(data_document_id_path0_path1_path2_path3_path4_path5_mockdeletetoget))
        .route("/api/data/document/{id}/{path0}/{path1}/{path2}/{path3}/{path4}/{path5}/mockputtopost", post(data_document_id_path0_path1_path2_path3_path4_path5_mockputtopost))
        .route("/api/data/document/{id}/{path0}/{path1}/{path2}/{path3}/{path4}/{path5}/{path6}", get(data_document_id_path0_path1_path2_path3_path4_path5_path6))
        .route("/api/data/document/{id}/{path0}/{path1}/{path2}/{path3}/{path4}/{path5}/{path6}", put(data_document_id_path0_path1_path2_path3_path4_path5_path6_update))
        .route("/api/data/document/{id}/{path0}/{path1}/{path2}/{path3}/{path4}/{path5}/{path6}", post(data_document_id_path0_path1_path2_path3_path4_path5_path6_create))
        .route("/api/data/document/{id}/{path0}/{path1}/{path2}/{path3}/{path4}/{path5}/{path6}", delete(data_document_id_path0_path1_path2_path3_path4_path5_path6_delete))
        .route("/api/data/document/{id}/{path0}/{path1}/{path2}/{path3}/{path4}/{path5}/{path6}/mockdeletetoget", get(data_document_id_path0_path1_path2_path3_path4_path5_path6_mockdeletetoget))
        .route("/api/data/document/{id}/{path0}/{path1}/{path2}/{path3}/{path4}/{path5}/{path6}/mockputtopost", post(data_document_id_path0_path1_path2_path3_path4_path5_path6_mockputtopost))
        .route("/api/data/document/{id}/{path0}/{path1}/{path2}/{path3}/{path4}/{path5}/{path6}/{path7}", get(data_document_id_path0_path1_path2_path3_path4_path5_path6_path7))
        .route("/api/data/document/{id}/{path0}/{path1}/{path2}/{path3}/{path4}/{path5}/{path6}/{path7}", put(data_document_id_path0_path1_path2_path3_path4_path5_path6_path7_update))
        .route("/api/data/document/{id}/{path0}/{path1}/{path2}/{path3}/{path4}/{path5}/{path6}/{path7}", post(data_document_id_path0_path1_path2_path3_path4_path5_path6_path7_create))
        .route("/api/data/document/{id}/{path0}/{path1}/{path2}/{path3}/{path4}/{path5}/{path6}/{path7}", delete(data_document_id_path0_path1_path2_path3_path4_path5_path6_path7_delete))
        .route("/api/data/document/{id}/{path0}/{path1}/{path2}/{path3}/{path4}/{path5}/{path6}/{path7}/mockdeletetoget", get(data_document_id_path0_path1_path2_path3_path4_path5_path6_path7_mockdeletetoget))
        .route("/api/data/document/{id}/{path0}/{path1}/{path2}/{path3}/{path4}/{path5}/{path6}/{path7}/mockputtopost", post(data_document_id_path0_path1_path2_path3_path4_path5_path6_path7_mockputtopost))
        .route("/api/anonymous/fileinfo/download/document/{id}", get(anonymous_fileinfo_download_document_id))
        .route("/api/fileinfo/download/document/{id}", get(fileinfo_download_document_id))
        .route("/api/fileinfo/upload/document/{id}", post(fileinfo_upload_document_docId))
        .route("/api/fileinfo/{id}", get(fileinfo_id))
        .route("/api/fileinfo/{id}/document/{docId}", get(fileinfo_id_document_documentId))
        .route("/api/fileinfo/{id}/mockdeletetoget", get(fileinfo_id_mockdeletetoget))
                // ── anonymous/surface/appdict（o2server AppDictAnonymousAction 对齐）──
        .route("/api/anonymous/surface/appdict/{appDictFlag}/appInfo/{appInfoFlag}", get(anonymous_surface_appdict_appDictFlag_appInfo_appInfoFlag))
        .route("/api/anonymous/surface/appdict/{appDictFlag}/appInfo/{appInfoFlag}/data", get(anonymous_surface_appdict_appDictFlag_appInfo_appInfoFlag_data))
        .route("/api/anonymous/surface/appdict/{appDictFlag}/appInfo/{appInfoFlag}/{path0}/data", get(anonymous_surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_data))
        .route("/api/anonymous/surface/appdict/{appDictFlag}/appInfo/{appInfoFlag}/{path0}/{path1}/data", get(anonymous_surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_data))
        .route("/api/anonymous/surface/appdict/{appDictFlag}/appInfo/{appInfoFlag}/{path0}/{path1}/{path2}/data", get(anonymous_surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_path2_data))
        .route("/api/anonymous/surface/appdict/{appDictFlag}/appInfo/{appInfoFlag}/{path0}/{path1}/{path2}/{path3}/data", get(anonymous_surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_path2_path3_data))
        .route("/api/anonymous/surface/appdict/{appDictFlag}/appInfo/{appInfoFlag}/{path0}/{path1}/{path2}/{path3}/{path4}/data", get(anonymous_surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_path2_path3_path4_data))
        .route("/api/anonymous/surface/appdict/{appDictFlag}/appInfo/{appInfoFlag}/{path0}/{path1}/{path2}/{path3}/{path4}/{path5}/data", get(anonymous_surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_path2_path3_path4_path5_data))
        .route("/api/anonymous/surface/appdict/{appDictFlag}/appInfo/{appInfoFlag}/{path0}/{path1}/{path2}/{path3}/{path4}/{path5}/{path6}/data", get(anonymous_surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_path2_path3_path4_path5_path6_data))
        .route("/api/anonymous/surface/appdict/{appDictFlag}/appInfo/{appInfoFlag}/{path0}/{path1}/{path2}/{path3}/{path4}/{path5}/{path6}/{path7}/data", get(anonymous_surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_path2_path3_path4_path5_path6_path7_data))
                // ── surface/appdict（o2server AppDictAction 对齐：段序修正 + 全动词）──
        .route("/api/surface/appdict/{appDictFlag}/appInfo/{appInfoFlag}", get(surface_appdict_appDictFlag_appInfo_appInfoFlag))
        .route("/api/surface/appdict/{appDictFlag}/appInfo/{appInfoFlag}", put(surface_appdict_appDictFlag_appInfo_appInfoFlag_update))
        .route("/api/surface/appdict/{appDictFlag}/appInfo/{appInfoFlag}/mockputtopost", post(surface_appdict_appDictFlag_appInfo_appInfoFlag_update))
        .route("/api/surface/appdict/{appDictFlag}/appInfo/{appInfoFlag}/data", get(surface_appdict_appDictFlag_appInfo_appInfoFlag_data))
        .route("/api/surface/appdict/{appDictFlag}/appInfo/{appInfoFlag}/{path0}/data", get(surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_data))
        .route("/api/surface/appdict/{appDictFlag}/appInfo/{appInfoFlag}/{path0}/data", put(surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_data_put))
        .route("/api/surface/appdict/{appDictFlag}/appInfo/{appInfoFlag}/{path0}/data", post(surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_data_post))
        .route("/api/surface/appdict/{appDictFlag}/appInfo/{appInfoFlag}/{path0}/data", delete(surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_data_delete))
        .route("/api/surface/appdict/{appDictFlag}/appInfo/{appInfoFlag}/{path0}/data/mockdeletetoget", get(surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_data_delete))
        .route("/api/surface/appdict/{appDictFlag}/appInfo/{appInfoFlag}/{path0}/{path1}/data", get(surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_data))
        .route("/api/surface/appdict/{appDictFlag}/appInfo/{appInfoFlag}/{path0}/{path1}/data", put(surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_data_put))
        .route("/api/surface/appdict/{appDictFlag}/appInfo/{appInfoFlag}/{path0}/{path1}/data", post(surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_data_post))
        .route("/api/surface/appdict/{appDictFlag}/appInfo/{appInfoFlag}/{path0}/{path1}/data", delete(surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_data_delete))
        .route("/api/surface/appdict/{appDictFlag}/appInfo/{appInfoFlag}/{path0}/{path1}/data/mockdeletetoget", get(surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_data_delete))
        .route("/api/surface/appdict/{appDictFlag}/appInfo/{appInfoFlag}/{path0}/{path1}/{path2}/data", get(surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_path2_data))
        .route("/api/surface/appdict/{appDictFlag}/appInfo/{appInfoFlag}/{path0}/{path1}/{path2}/data", put(surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_path2_data_put))
        .route("/api/surface/appdict/{appDictFlag}/appInfo/{appInfoFlag}/{path0}/{path1}/{path2}/data", post(surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_path2_data_post))
        .route("/api/surface/appdict/{appDictFlag}/appInfo/{appInfoFlag}/{path0}/{path1}/{path2}/data", delete(surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_path2_data_delete))
        .route("/api/surface/appdict/{appDictFlag}/appInfo/{appInfoFlag}/{path0}/{path1}/{path2}/data/mockdeletetoget", get(surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_path2_data_delete))
        .route("/api/surface/appdict/{appDictFlag}/appInfo/{appInfoFlag}/{path0}/{path1}/{path2}/data/mockputtopost", post(surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_path2_data_put))
        .route("/api/surface/appdict/{appDictFlag}/appInfo/{appInfoFlag}/{path0}/{path1}/{path2}/{path3}/data", get(surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_path2_path3_data))
        .route("/api/surface/appdict/{appDictFlag}/appInfo/{appInfoFlag}/{path0}/{path1}/{path2}/{path3}/data", put(surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_path2_path3_data_put))
        .route("/api/surface/appdict/{appDictFlag}/appInfo/{appInfoFlag}/{path0}/{path1}/{path2}/{path3}/data", post(surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_path2_path3_data_post))
        .route("/api/surface/appdict/{appDictFlag}/appInfo/{appInfoFlag}/{path0}/{path1}/{path2}/{path3}/data", delete(surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_path2_path3_data_delete))
        .route("/api/surface/appdict/{appDictFlag}/appInfo/{appInfoFlag}/{path0}/{path1}/{path2}/{path3}/data/mockdeletetoget", get(surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_path2_path3_data_delete))
        .route("/api/surface/appdict/{appDictFlag}/appInfo/{appInfoFlag}/{path0}/{path1}/{path2}/{path3}/data/mockputtopost", post(surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_path2_path3_data_put))
        .route("/api/surface/appdict/{appDictFlag}/appInfo/{appInfoFlag}/{path0}/{path1}/{path2}/{path3}/{path4}/data", get(surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_path2_path3_path4_data))
        .route("/api/surface/appdict/{appDictFlag}/appInfo/{appInfoFlag}/{path0}/{path1}/{path2}/{path3}/{path4}/data", put(surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_path2_path3_path4_data_put))
        .route("/api/surface/appdict/{appDictFlag}/appInfo/{appInfoFlag}/{path0}/{path1}/{path2}/{path3}/{path4}/data", post(surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_path2_path3_path4_data_post))
        .route("/api/surface/appdict/{appDictFlag}/appInfo/{appInfoFlag}/{path0}/{path1}/{path2}/{path3}/{path4}/data", delete(surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_path2_path3_path4_data_delete))
        .route("/api/surface/appdict/{appDictFlag}/appInfo/{appInfoFlag}/{path0}/{path1}/{path2}/{path3}/{path4}/data/mockdeletetoget", get(surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_path2_path3_path4_data_delete))
        .route("/api/surface/appdict/{appDictFlag}/appInfo/{appInfoFlag}/{path0}/{path1}/{path2}/{path3}/{path4}/data/mockputtopost", post(surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_path2_path3_path4_data_put))
        .route("/api/surface/appdict/{appDictFlag}/appInfo/{appInfoFlag}/{path0}/{path1}/{path2}/{path3}/{path4}/{path5}/data", get(surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_path2_path3_path4_path5_data))
        .route("/api/surface/appdict/{appDictFlag}/appInfo/{appInfoFlag}/{path0}/{path1}/{path2}/{path3}/{path4}/{path5}/data", put(surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_path2_path3_path4_path5_data_put))
        .route("/api/surface/appdict/{appDictFlag}/appInfo/{appInfoFlag}/{path0}/{path1}/{path2}/{path3}/{path4}/{path5}/data", post(surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_path2_path3_path4_path5_data_post))
        .route("/api/surface/appdict/{appDictFlag}/appInfo/{appInfoFlag}/{path0}/{path1}/{path2}/{path3}/{path4}/{path5}/data", delete(surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_path2_path3_path4_path5_data_delete))
        .route("/api/surface/appdict/{appDictFlag}/appInfo/{appInfoFlag}/{path0}/{path1}/{path2}/{path3}/{path4}/{path5}/data/mockdeletetoget", get(surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_path2_path3_path4_path5_data_delete))
        .route("/api/surface/appdict/{appDictFlag}/appInfo/{appInfoFlag}/{path0}/{path1}/{path2}/{path3}/{path4}/{path5}/data/mockputtopost", post(surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_path2_path3_path4_path5_data_put))
        .route("/api/surface/appdict/{appDictFlag}/appInfo/{appInfoFlag}/{path0}/{path1}/{path2}/{path3}/{path4}/{path5}/{path6}/data", get(surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_path2_path3_path4_path5_path6_data))
        .route("/api/surface/appdict/{appDictFlag}/appInfo/{appInfoFlag}/{path0}/{path1}/{path2}/{path3}/{path4}/{path5}/{path6}/data", put(surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_path2_path3_path4_path5_path6_data_put))
        .route("/api/surface/appdict/{appDictFlag}/appInfo/{appInfoFlag}/{path0}/{path1}/{path2}/{path3}/{path4}/{path5}/{path6}/data", post(surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_path2_path3_path4_path5_path6_data_post))
        .route("/api/surface/appdict/{appDictFlag}/appInfo/{appInfoFlag}/{path0}/{path1}/{path2}/{path3}/{path4}/{path5}/{path6}/data", delete(surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_path2_path3_path4_path5_path6_data_delete))
        .route("/api/surface/appdict/{appDictFlag}/appInfo/{appInfoFlag}/{path0}/{path1}/{path2}/{path3}/{path4}/{path5}/{path6}/data/mockdeletetoget", get(surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_path2_path3_path4_path5_path6_data_delete))
        .route("/api/surface/appdict/{appDictFlag}/appInfo/{appInfoFlag}/{path0}/{path1}/{path2}/{path3}/{path4}/{path5}/{path6}/data/mockputtopost", post(surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_path2_path3_path4_path5_path6_data_put))
        .route("/api/surface/appdict/{appDictFlag}/appInfo/{appInfoFlag}/{path0}/{path1}/{path2}/{path3}/{path4}/{path5}/{path6}/{path7}/data", get(surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_path2_path3_path4_path5_path6_path7_data))
        .route("/api/surface/appdict/{appDictFlag}/appInfo/{appInfoFlag}/{path0}/{path1}/{path2}/{path3}/{path4}/{path5}/{path6}/{path7}/data", put(surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_path2_path3_path4_path5_path6_path7_data_put))
        .route("/api/surface/appdict/{appDictFlag}/appInfo/{appInfoFlag}/{path0}/{path1}/{path2}/{path3}/{path4}/{path5}/{path6}/{path7}/data", post(surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_path2_path3_path4_path5_path6_path7_data_post))
        .route("/api/surface/appdict/{appDictFlag}/appInfo/{appInfoFlag}/{path0}/{path1}/{path2}/{path3}/{path4}/{path5}/{path6}/{path7}/data", delete(surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_path2_path3_path4_path5_path6_path7_data_delete))
        .route("/api/surface/appdict/{appDictFlag}/appInfo/{appInfoFlag}/{path0}/{path1}/{path2}/{path3}/{path4}/{path5}/{path6}/{path7}/data/mockdeletetoget", get(surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_path2_path3_path4_path5_path6_path7_data_delete))
        .route("/api/surface/appdict/{appDictFlag}/appInfo/{appInfoFlag}/{path0}/{path1}/{path2}/{path3}/{path4}/{path5}/{path6}/{path7}/data/mockputtopost", post(surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_path2_path3_path4_path5_path6_path7_data_put))
        .route("/api/appinfo/list/all", get(appinfo_list_all))
        .route("/api/appinfo/list/has/document/type/{appType}", get(appinfo_list_has_document_type_appType))
        .route("/api/appinfo/list/manage", get(appinfo_list_manage))
        .route("/api/appinfo/list/manage/type/{appType}", get(appinfo_list_manage_type_appType))
        .route("/api/appinfo/list/user/publish", get(appinfo_list_user_publish))
        .route("/api/appinfo/list/user/publish/type/{appType}", get(appinfo_list_user_publish_type_appType))
        .route("/api/appinfo/list/user/publish/with/process", get(appinfo_list_user_publish_with_process))
        .route("/api/appinfo/list/user/view", get(appinfo_list_user_view))
        .route("/api/appinfo/list/user/view/all", get(appinfo_list_user_view_all))
        .route("/api/appinfo/list/user/view/all/type/{appType}", get(appinfo_list_user_view_all_type_appType))
        .route("/api/appinfo/list/user/view/article/type/{appType}", get(appinfo_list_user_view_article_type_appType))
        .route("/api/appinfo/list/user/view/data", get(appinfo_list_user_view_data))
        .route("/api/appinfo/list/user/view/data/type/{appType}", get(appinfo_list_user_view_data_type_appType))
        .route("/api/appinfo/flag", get(appinfo_flag))
        .route("/api/appinfo/{id}", get(appinfo_id))
        .route("/api/categoryinfo/filter/list/{id}/next/{count}/app/{appId}", put(categoryinfo_filter_list_id_next_count_app_appId))
        .route("/api/categoryinfo/filter/list/{id}/next/{count}/app/{appId}/mockputtopost", post(categoryinfo_filter_list_id_next_count_app_appId_mockputtopost))
        .route("/api/categoryinfo/filter/list/{id}/prev/{count}/app/{appId}", put(categoryinfo_filter_list_id_prev_count_app_appId))
        .route("/api/categoryinfo/filter/list/{id}/prev/{count}/app/{appId}/mockputtopost", post(categoryinfo_filter_list_id_prev_count_app_appId_mockputtopost))
        .route("/api/categoryinfo/list/all", get(categoryinfo_list_all))
        .route("/api/categoryinfo/list/manage/app/{appId}", get(categoryinfo_list_manage_app_appId))
        .route("/api/categoryinfo/list/publish/app/{appId}", get(categoryinfo_list_publish_app_appId))
        .route("/api/categoryinfo/list/view/app/{appId}", get(categoryinfo_list_view_app_appId))
        .route("/api/categoryinfo/list/view/app/{appId}/all", get(categoryinfo_list_view_app_appId_all))
        .route("/api/categoryinfo/list/view/app/{appId}/data", get(categoryinfo_list_view_app_appId_data))
        .route("/api/categoryinfo/flag", get(categoryinfo_flag))
        .route("/api/categoryinfo/{id}", get(categoryinfo_id))
        .route("/api/commend/list/paging/{page}/size/{size}", post(commend_list_paging_page_size_size))
        .route("/api/commend/{id}", get(commend_id))
        .route("/api/comment/{id}", get(comment_id))
        .route("/api/correlation/doc/{docId}", get(correlation_doc_docId).post(correlation_create_u3))
        .route("/api/correlation/list/doc/{docId}", get(correlation_list_doc_docId))
        .route("/api/correlation/list/doc/{docId}/site/{site}", get(correlation_list_doc_docId_site_site))
        .route("/api/design/appdict/list/appInfo/{appId}", get(design_appdict_list_appInfo_appId))
        .route("/api/design/appdict/list/paging/{page}/size/{size}", post(design_appdict_list_paging_page_size_size))
        .route("/api/design/appdict/{id}", get(design_appdict_id))
        .route("/api/file/list/appInfo/{appInfoFlag}", get(file_list_appInfo_appInfoFlag))
        .route("/api/file/flag", get(file_flag))
        .route("/api/file/{id}", get(file_id))
        .route("/api/fileinfo/list/all", get(fileinfo_list_all))
        .route("/api/fileinfo/list/document/{documentId}", get(fileinfo_list_document_documentId))
        .route("/api/form/list/all", get(form_list_all))
        .route("/api/form/list/app/{appId}", get(form_list_app_appId))
        .route("/api/form/list/formfield/appInfo/{appId}", get(form_list_formfield_appInfo_appId))
        .route("/api/form/list/{id}/formfield", get(form_list_id_formfield))
        .route("/api/form/{id}", get(form_id))
        .route("/api/form/v2/{id}", get(form_v2_id))
        .route("/api/log/list", get(log_list))
        .route("/api/log/list/app/{appId}", get(log_list_app_appId))
        .route("/api/log/list/category/{categoryId}", get(log_list_category_categoryId))
        .route("/api/log/list/document/{documentId}", get(log_list_document_documentId))
        .route("/api/log/list/level/{operationLevel}", get(log_list_level_operationLevel))
        .route("/api/log/{id}", get(log_id))
        .route("/api/output/list", get(output_list))
        .route("/api/permission/appInfo/{id}/manageable", get(permission_appInfo_id_manageable))
        .route("/api/permission/appInfo/{id}/managers", get(permission_appInfo_id_managers))
        .route("/api/permission/appInfo/{id}/publishers", get(permission_appInfo_id_publishers))
        .route("/api/permission/appInfo/{id}/viewers", get(permission_appInfo_id_viewers))
        .route("/api/permission/category/{id}/managers", get(permission_category_id_managers))
        .route("/api/permission/category/{id}/publishers", get(permission_category_id_publishers))
        .route("/api/permission/category/{id}/viewers", get(permission_category_id_viewers))
        .route("/api/permission/categoryInfo/{id}/manageable", get(permission_categoryInfo_id_manageable))
        .route("/api/permission/management/refresh/all", get(permission_management_refresh_all))
        .route("/api/script/list/app/{appId}/name/{name}", get(script_list_app_appId_name_name))
        .route("/api/script/list/app/{flag}", get(script_list_app_flag))
        .route("/api/script/list/paging/{page}/size/{size}", post(script_list_paging_page_size_size))
        .route("/api/script/{id}", get(script_id))
        .route("/api/searchfilter/list/archive/filter/category/{categoryId}", get(searchfilter_list_archive_filter_category_categoryId))
        .route("/api/searchfilter/list/draft/filter/category/{categoryId}", get(searchfilter_list_draft_filter_category_categoryId))
        .route("/api/searchfilter/list/publish/filter/category/{categoryId}", get(searchfilter_list_publish_filter_category_categoryId))
        .route("/api/anonymous/surface/appdict/list/appInfo/{appInfoFlag}", get(anonymous_surface_appdict_list_appInfo_appInfoFlag))
        .route("/api/surface/appdict/list/appInfo/{appInfoFlag}", get(surface_appdict_list_appInfo_appInfoFlag))
        .route("/api/templateform/list", get(templateform_list))
        .route("/api/templateform/list/category", get(templateform_list_category))
        .route("/api/templateform/list/category/mockputtopost", post(templateform_list_category_mockputtopost))
        .route("/api/templateform/create", post(templateform_create))
        .route("/api/templateform/save/{id}", put(templateform_save))
        .route("/api/templateform/save/{id}", post(templateform_save))
        .route("/api/templateform/delete/{id}", delete(templateform_delete))
        .route("/api/templateform/delete/{id}", post(templateform_delete))
        .route("/api/uuid/random", get(uuid_random))
        .route("/api/view/list/all", get(view_list_all))
        .route("/api/view/list/app/{appId}", get(view_list_app_appId))
        .route("/api/view/list/category/{categoryId}", get(view_list_category_categoryId))
        .route("/api/view/list/form/{formId}", get(view_list_form_formId))
        .route("/api/view/{id}", get(view_id))
        .route("/api/viewcategory/list/all", get(viewcategory_list_all))
        .route("/api/viewcategory/list/category/{categoryId}", get(viewcategory_list_category_categoryId))
        .route("/api/viewcategory/list/view/{viewId}", get(viewcategory_list_view_viewId))
        .route("/api/viewcategory/{id}", get(viewcategory_id))
        .route("/api/viewfieldconfig/list/all", get(viewfieldconfig_list_all))
        .route("/api/viewfieldconfig/list/view/{viewId}", get(viewfieldconfig_list_view_viewId))
        .route("/api/viewfieldconfig/{id}", get(viewfieldconfig_id))
        .route("/api/viewrecord/list/install/log/paging/{page}/size/{size}", post(viewrecord_list_install_log_paging_page_size_size))
        .route("/api/image/encode/base64", post(image_encode_base64))
        .route("/api/image/encode/base64/size/{size}", post(image_encode_base64_size_size))
        .route("/api/image/resize/id/{id}/width/{width}/height/{height}", post(image_resize_id_id_width_width_height_height))
        .route("/api/input/compare", put(input_compare))
        .route("/api/input/compare/mockputtopost", post(input_compare_mockputtopost))
        .route("/api/input/cover", put(input_cover))
        .route("/api/input/cover/mockputtopost", post(input_cover_mockputtopost))
        .route("/api/input/create", put(input_create))
        .route("/api/input/create/mockputtopost", post(input_create_mockputtopost))
        .route("/api/input/prepare/cover", put(input_prepare_cover))
        .route("/api/input/prepare/cover/mockputtopost", post(input_prepare_cover_mockputtopost))
        .route("/api/input/prepare/create", put(input_prepare_create))
        .route("/api/input/prepare/create/mockputtopost", post(input_prepare_create_mockputtopost))
        .route("/api/cms_assemble_control/update/control/config", put(update_control_config))
        // ── plan002 U2: o2server 对齐缺口端点 ──
        .route("/api/document/{id}", get(document_u2_get))
        .route("/api/document/{id}", delete(document_u2_delete))
        .route("/api/document", post(document_u2_create))
        .route("/api/document/{id}/update", post(document_u2_update))
        .route("/api/document/publish/{id}", put(document_u2_publish))
        .route("/api/document/publish/{id}/mockputtopost", post(document_u2_publish))
        .route("/api/document/publish/{id}/cancel", put(document_u2_publish_cancel))
        .route("/api/document/publish/{id}/cancel/mockputtopost", post(document_u2_publish_cancel))
        .route("/api/document/{id}/commend", get(document_u2_commend))
        .route("/api/document/{id}/uncommend", get(document_u2_uncommend))
        .route("/api/document/{id}/top", get(document_u2_top))
        .route("/api/document/{id}/unTop", get(document_u2_un_top))
        .route("/api/document/category/change", put(document_u2_category_change))
        .route("/api/document/category/change/mockputtopost", post(document_u2_category_change))
        .route("/api/document/{id}/document/data", get(document_u2_document_data))
        .route("/api/document/list/document", post(document_u2_list_document))
        .route("/api/document/document/fields", get(document_u2_fields))
        .route("/api/document/filter/count", put(document_u2_filter_count))
        .route("/api/document/filter/count/mockputtopost", post(document_u2_filter_count))
        .route("/api/comment", post(comment_u2_create))
        .route("/api/comment/{id}", delete(comment_u2_delete))
        .route("/api/comment/list/{page}/size/{size}", put(comment_u2_list_page_size_size))
        .route("/api/comment/list/{page}/size/{size}/mockputtopost", post(comment_u2_list_page_size_size))
        .route("/api/correlation/doc/{docId}/delete", post(correlation_u2_doc_delete))
        .route("/api/file", post(file_u2_create))
        .route("/api/file/{id}/mockputtopost", post(file_u2_update))
        .route("/api/fileinfo/{id}", delete(fileinfo_u2_delete))
        .route("/api/fileinfo/list/filter", post(fileinfo_u2_filter))
        .route("/api/fileinfo/copy/to/doc/{docId}", post(fileinfo_u2_copy_to_doc))
        .route("/api/fileinfo/replace/to/doc/{docId}", post(fileinfo_u2_replace_to_doc))
        .route("/api/form", post(form_u2_create))
        .route("/api/form/submit", post(crate::form_submit))
        .route("/api/form/{id}", put(form_u2_update))
        .route("/api/form/{id}", delete(form_u2_delete))
        .route("/api/script", post(script_u2_create))
        .route("/api/script/{id}", put(script_u2_update))
        .route("/api/script/{id}", delete(script_u2_delete))
        .route("/api/script/list/manager", post(script_u2_list_manager))
        .route("/api/templateform", post(templateform_u2_create))
        .route("/api/templateform/{id}", delete(templateform_u2_delete))
        .route("/api/view", post(view_u2_create))
        .route("/api/view/{id}", put(view_u2_update))
        .route("/api/view/{id}", delete(view_u2_delete))
        .route("/api/viewcategory", post(viewcategory_u2_create))
        .route("/api/viewcategory/{id}", delete(viewcategory_u2_delete))
        .route("/api/viewfieldconfig", post(viewfieldconfig_u2_create))
        .route("/api/viewfieldconfig/{id}", put(viewfieldconfig_u2_update))
        .route("/api/viewfieldconfig/{id}", delete(viewfieldconfig_u2_delete))
        .route("/api/appinfo", post(appinfo_u2_create))
        .route("/api/appinfo/{id}", delete(appinfo_u2_delete))
        .route("/api/categoryinfo", post(categoryinfo_u2_create))
        .route("/api/categoryinfo/{id}", delete(categoryinfo_u2_delete))
        .route("/api/appinfo/{id}/permission", post(permission_u2_app_info))
        .route("/api/categoryinfo/{id}/permission", post(permission_u2_category_info))
        .route("/api/appconfig/{appId}", post(appconfig_u2_update))
        .route("/api/appconfig/{appId}", get(appconfig_u2_get))
        .route("/api/designer/search", post(designer_u2_search))
        // ── plan002 U2 收尾重试批次（U3）：o2server canonical 对齐缺口 ──
        .route("/api/appinfo/erase/app/{id}", delete(appinfo_erase_app_id))
        .route("/api/appinfo/erase/app/{id}/mockdeletetoget", get(appinfo_erase_app_id_mockdeletetoget))
        .route("/api/appinfo/alias/{alias}", get(appinfo_alias_alias))
        .route("/api/appinfo/filter/list/{id}/next/{count}", put(appinfo_filter_list_id_next_count))
        .route("/api/appinfo/filter/list/{id}/next/{count}/mockputtopost", post(appinfo_filter_list_id_next_count_mockputtopost))
        .route("/api/appinfo/filter/list/{id}/prev/{count}", put(appinfo_filter_list_id_prev_count))
        .route("/api/appinfo/filter/list/{id}/prev/{count}/mockputtopost", post(appinfo_filter_list_id_prev_count_mockputtopost))
        .route("/api/appinfo/get/user/publish/{appId}", get(appinfo_get_user_publish_appId))
        .route("/api/appinfo/list/appType", get(appinfo_list_appType))
        .route("/api/appinfo/list/appType/manager", get(appinfo_list_appType_manager))
        .route("/api/appinfo/list/has/document", get(appinfo_list_has_document))
        .route("/api/appinfo/list/has/document/appType", get(appinfo_list_has_document_appType))
        .route("/api/appinfo/{id}/control", get(appinfo_id_control))
        .route("/api/appinfo/{id}/mockdeletetoget", get(appinfo_id_mockdeletetoget))
        .route("/api/appinfo/{id}/icon/size/{size}", post(appinfo_appId_icon_size_size))
        .route("/api/categoryinfo/erase/category/{id}", delete(categoryinfo_erase_category_id))
        .route("/api/categoryinfo/erase/category/{id}/mockdeletetoget", get(categoryinfo_erase_category_id_mockdeletetoget))
        .route("/api/categoryinfo/alias/{alias}", get(categoryinfo_alias_alias))
        .route("/api/categoryinfo/bind/{categoryId}/view", put(categoryinfo_bind_categoryId_view))
        .route("/api/categoryinfo/bind/{categoryId}/view/mockputtopost", post(categoryinfo_bind_categoryId_view_mockputtopost))
        .route("/api/categoryinfo/extContent", post(categoryinfo_ext_content_save_u3))
        .route("/api/categoryinfo/filter/list/{page}/size/{size}", put(categoryinfo_filter_list_page_size_size))
        .route("/api/categoryinfo/filter/list/{page}/size/{size}/mockputtopost", post(categoryinfo_filter_list_page_size_size_mockputtopost))
        .route("/api/categoryinfo/list/objects", post(categoryinfo_list_objects_u3))
        .route("/api/categoryinfo/{id}/control", get(categoryinfo_id_control))
        .route("/api/categoryinfo/{id}/execute/projection", post(categoryinfo_id_execute_projection))
        .route("/api/categoryinfo/{id}/mockdeletetoget", get(categoryinfo_id_mockdeletetoget))
        .route("/api/comment/list/{id}/next/{count}", put(comment_list_id_next_count))
        .route("/api/comment/list/{id}/next/{count}/mockputtopost", post(comment_list_id_next_count_mockputtopost))
        .route("/api/comment/list/{id}/prev/{count}", put(comment_list_id_prev_count))
        .route("/api/comment/list/{id}/prev/{count}/mockputtopost", post(comment_list_id_prev_count_mockputtopost))
        .route("/api/comment/{id}/commend", get(comment_commend_u3))
        .route("/api/comment/{id}/uncommend", get(comment_uncommend_u3))
        .route("/api/comment/{id}/mockdeletetoget", get(comment_id_mockdeletetoget))
        .route("/api/correlation/update/doc/{docId}", post(correlation_update_u3))
        .route("/api/design/appdict", post(design_appdict_create_u3))
        .route("/api/design/appdict/{id}", put(design_appdict_update_u3))
        .route("/api/design/appdict/{id}", delete(design_appdict_delete_u3))
        .route("/api/design/appdict/{id}/mockdeletetoget", get(design_appdict_id_mockdeletetoget))
        .route("/api/design/appdict/{id}/mockputtopost", post(design_appdict_id_mockputtopost))
        .route("/api/docpermission", post(permission_management_refresh_all))
        .route("/api/review/v2/search", post(review_v2_search_u3))
        .route("/api/document/achive/{id}", get(document_achive_u3))
        .route("/api/document/batch/data/modify", put(document_batch_modify_u3))
        .route("/api/document/batch/data/modify/mockputtopost", post(document_batch_modify_mock_u3))
        .route("/api/document/batch/status", get(document_batch_status_u3))
        .route("/api/document/batch/{id}", delete(document_batch_delete_u3))
        .route("/api/document/batch/{id}/mockdeletetoget", get(document_batch_delete_mock_u3))
        .route("/api/document/batch/{id}/status", get(document_batch_name_status_u3))
        .route("/api/document/cipher/publish/content", put(document_cipher_publish_workflow_u3))
        .route("/api/document/cipher/publish/content/mockputtopost", post(document_cipher_publish_workflow_mock_u3))
        .route("/api/document/cipher/{id}/permission/read/person/{person}", get(document_cipher_permission_read_u3))
        .route("/api/document/cipher/{id}/persist/view/record", post(document_cipher_id_persist_view_record))
        .route("/api/document/cipher/filter/list/{page}/size/{size}", put(document_cipher_filter_list_page_size_size))
        .route("/api/document/cipher/filter/list/{page}/size/{size}/mockputtopost", post(document_cipher_filter_list_page_size_size_mockputtopost))
        .route("/api/document/draft/list/{id}/next/{count}", put(document_draft_next_u3))
        .route("/api/document/draft/list/{id}/next/{count}/mockputtopost", post(document_draft_next_u3))
        .route("/api/document/filter/list/{id}/next/{count}", put(document_filter_next_u3))
        .route("/api/document/filter/list/{id}/next/{count}/mockputtopost", post(document_filter_next_u3))
        .route("/api/document/filter/list/{id}/prev/{count}", put(document_filter_prev_u3))
        .route("/api/document/filter/list/{id}/prev/{count}/mockputtopost", post(document_filter_prev_u3))
        .route("/api/document/filter/list/{page}/size/{size}", put(document_filter_paging_u3))
        .route("/api/document/filter/list/{page}/size/{size}/mockputtopost", post(document_filter_paging_u3))
        .route("/api/document/filter/list/{page}/size/{size}/manager", post(document_filter_paging_manager_u3))
        .route("/api/document/list/document/data", post(document_list_document_data_u3))
        .route("/api/document/publish/content", put(document_publish_content_u3))
        .route("/api/document/publish/content/mockputtopost", post(document_publish_content_mock_u3))
        .route("/api/document/{id}/control", get(document_control_u3))
        .route("/api/document/{id}/mockdeletetoget", get(document_u2_get))
        .route("/api/document/{id}/notify", post(document_notify_u3))
        .route("/api/document/{id}/permission/read", get(document_permission_read_u3))
        .route("/api/document/{id}/persons", get(document_persons_u3))
        .route("/api/document/{id}/publish/html", post(document_publish_html_u3))
        .route("/api/document/{id}/view", get(anonymous_document_id_view))
        .route("/api/file/list/{id}/next/{count}", get(file_list_id_next_count))
        .route("/api/file/list/{id}/prev/{count}", get(file_list_id_prev_count))
        .route("/api/file/{id}", delete(file_delete_u3))
        .route("/api/file/{id}", put(file_update_u3))
        .route("/api/file/{id}/mockdeletetoget", get(file_flag_mockdeletetoget))
        .route("/api/file/{id}/upload", post(file_id_upload))
        .route("/api/file/{id}/content", get(file_id_content))
        .route("/api/file/{id}/download", get(file_id_download))
        .route("/api/file/{flag}/appInfo/{appInfoFlag}", get(file_copy_u3))
        .route("/api/file/{flag}/appInfo/{appInfoFlag}/content", get(file_flag_appInfo_appInfoFlag_content))
        .route("/api/file/{flag}/appInfo/{appInfoFlag}/download", get(file_download_with_app_u3))
        .route("/api/fileinfo/batch/download/doc/{docId}/site/{site}", get(fileinfo_batch_download_doc_docId_site_site))
        .route("/api/fileinfo/download/document/{id}/stream", get(fileinfo_download_document_id_stream))
        .route("/api/fileinfo/download/transfer/flag/{flag}", get(fileinfo_download_transfer_flag_flag))
        .route("/api/fileinfo/{id}/binary/base64/{size}", get(fileinfo_binary_base64_u3))
        .route("/api/fileinfo/{id}/doc/{docId}/change/seqnumber/{seqNumber}", get(fileinfo_id_doc_docId_change_seqnumber_seqNumber))
        .route("/api/fileinfo/{id}/online/info", get(fileinfo_id_online_info))
        .route("/api/fileinfo/{id}/preview/pdf", get(fileinfo_id_preview_pdf))
        .route("/api/fileinfo/edit/{id}/doc/{docId}", put(fileinfo_edit_id_doc_docId))
        .route("/api/fileinfo/edit/{id}/doc/{docId}/mockputtopost", post(fileinfo_edit_id_doc_docId_mockputtopost))
        .route("/api/fileinfo/update/document/{docId}/attachment/{id}", post(fileinfo_update_document_docId_attachment_id))
        .route("/api/fileinfo/update/document/{docId}/attachment/{id}/callback/{callback}", post(fileinfo_update_document_docId_attachment_id_callback_callback))
        .route("/api/fileinfo/update/{id}/content", post(fileinfo_update_id_content))
        .route("/api/fileinfo/upload/doc/{docId}/save/as/{flag}", post(fileinfo_upload_doc_docId_save_as_flag))
        .route("/api/fileinfo/upload/document/{docId}/callback/{callback}", post(fileinfo_upload_document_docId_callback_callback))
        .route("/api/fileinfo/upload/with/url", post(fileinfo_upload_with_url_u3))
        .route("/api/form/v2/lookup/document/{docId}", get(form_v2_lookup_document_docId))
        .route("/api/form/v2/lookup/document/{docId}/mobile", get(form_v2_lookup_document_docId_mobile))
        .route("/api/form/v2/{id}/mobile", get(form_v2_id_mobile))
        .route("/api/form/{id}/appinfo/{appFlag}", get(form_get_with_appinfo_u3))
        .route("/api/form/{id}/mockdeletetoget", get(form_id_mockdeletetoget))
        .route("/api/form/{id}/mockputtopost", post(form_id_mockputtopost))
        .route("/api/form/filter/list/{id}/next/{count}/app/{appId}", put(form_filter_list_id_next_count_app_appId))
        .route("/api/form/filter/list/{id}/next/{count}/app/{appId}/mockputtopost", post(form_filter_list_id_next_count_app_appId_mockputtopost))
        .route("/api/form/filter/list/{id}/prev/{count}/app/{appId}", put(form_filter_list_id_prev_count_app_appId))
        .route("/api/form/filter/list/{id}/prev/{count}/app/{appId}/mockputtopost", post(form_filter_list_id_prev_count_app_appId_mockputtopost))
        .route("/api/formversion/{id}", get(formversion_id))
        .route("/api/formversion/list/form/{formId}", get(formversion_list_form_formId))
        .route("/api/scriptversion/{id}", get(scriptversion_id))
        .route("/api/scriptversion/list/script/{scriptId}", get(scriptversion_list_script_scriptId))
        .route("/api/log/filter/list/{id}/next/{count}", post(log_filter_list_id_next_count))
        .route("/api/log/filter/list/{id}/prev/{count}", post(log_filter_list_id_prev_count))
        .route("/api/log/list/filter/{page}/size/{size}", post(log_list_filter_page_size_size))
        .route("/api/output/{appInfoFlag}/select", put(output_appInfoFlag_select))
        .route("/api/output/{appInfoFlag}/select/mockputtopost", post(output_appInfoFlag_select_mockputtopost))
        .route("/api/permission/management/refresh/category/{categoryId}", get(permission_management_refresh_category_categoryId))
        .route("/api/permission/manager/appInfo/{id}", post(permission_save_manager_app_u3))
        .route("/api/permission/manager/categoryInfo/{id}", post(permission_save_manager_category_u3))
        .route("/api/permission/publisher/appInfo/{id}", post(permission_save_publisher_app_u3))
        .route("/api/permission/publisher/categoryInfo/{id}", post(permission_save_publisher_category_u3))
        .route("/api/permission/viewer/appInfo/{id}", post(permission_save_viewer_app_u3))
        .route("/api/permission/viewer/categoryInfo/{id}", post(permission_save_viewer_category_u3))
        .route("/api/script/list/{id}/next/{count}", get(script_list_id_next_count))
        .route("/api/script/list/{id}/prev/{count}", get(script_list_id_prev_count))
        .route("/api/script/{uniqueName}/app/{flag}", post(script_post_nested_u3))
        .route("/api/script/{uniqueName}/app/{flag}/imported", get(script_uniqueName_app_flag_imported))
        .route("/api/script/{uniqueName}/appInfo/{appFlag}", post(script_load_u3))
        .route("/api/script/{id}/mockputtopost", post(script_id_mockputtopost))
        .route("/api/script/{id}/mockdeletetoget", get(script_id_mockdeletetoget))
        .route("/api/templateform/{id}", get(templateform_id))
        .route("/api/templateform/{id}/mockdeletetoget", get(templateform_id_mockdeletetoget))
        .route("/api/templateform/list/category", put(templateform_list_category))
        .route("/api/view/viewdata/list/{id}/next/{count}", post(view_viewdata_list_id_next_count))
        .route("/api/view/{id}/mockdeletetoget", get(view_id_mockdeletetoget))
        .route("/api/view/{id}/mockputtopost", post(view_id_mockputtopost))
        .route("/api/viewcategory/{id}/mockdeletetoget", get(viewcategory_id_mockdeletetoget))
        .route("/api/viewfieldconfig/{id}/mockdeletetoget", get(viewfieldconfig_id_mockdeletetoget))
        .route("/api/viewfieldconfig/{id}/mockputtopost", post(viewfieldconfig_id_mockputtopost))
        .route("/api/viewrecord/document/{docId}/filter/list/{id}/next/{count}", get(viewrecord_document_docId_filter_list_id_next_count))
        .route("/api/viewrecord/document/{docId}/has/view", get(viewrecord_document_docId_has_view))
        .route("/api/viewrecord/person/{person}", get(viewrecord_by_person_u3))
        .route("/api/viewrecord/unread", put(viewrecord_unread_u3))
        .route("/api/viewrecord/unread/mockputtopost", post(viewrecord_unread_u3))
        .route("/api/anonymous/document/filter/list/{id}/next/{count}", put(anonymous_document_filter_list_id_next_count))
        .route("/api/anonymous/document/filter/list/{id}/next/{count}/mockputtopost", post(anonymous_document_filter_list_id_next_count_mockputtopost))
        .route("/api/anonymous/document/filter/list/{page}/size/{size}", put(anonymous_document_filter_list_page_size_size))
        .route("/api/anonymous/document/filter/list/{page}/size/{size}/mockputtopost", post(anonymous_document_filter_list_page_size_size_mockputtopost))
        .route("/api/anonymous/form/{id}", get(anonymous_form_id))
        .route("/api/anonymous/form/v2/{id}", get(anonymous_form_v2_id))
        .route("/api/anonymous/form/v2/{id}/mobile", get(anonymous_form_v2_id_mobile))
        .route("/api/anonymous/form/v2/lookup/document/{docId}", get(anonymous_form_v2_lookup_document_docId))
        .route("/api/anonymous/form/v2/lookup/document/{docId}/mobile", get(anonymous_form_v2_lookup_document_docId_mobile))
        .route("/api/anonymous/fileinfo/list/document/{documentId}", get(anonymous_fileinfo_list_document_documentId))
        .route("/api/anonymous/fileinfo/{id}/document/{documentId}", get(fileinfo_id_document_documentId))
        .route("/api/anonymous/fileinfo/download/document/{id}/stream", get(anonymous_fileinfo_download_document_id_stream))
        // ── plan002 U2 收尾：o2server 缺口补齐（export/import appInfo）──
        .route("/api/export/appInfo/{appInfoFlag}", get(export_app_info_app_info_flag))
        .route("/api/import/appInfo/{appInfoFlag}", get(import_app_info_app_info_flag))
.layer(Extension(pool))
}

pub fn cms_assemble_control_router(pool: Pool) -> Router {
    router(pool)
}
