use axum::{
    extract::Extension,
    routing::{delete, get, post, put},
    Router,
};
use deadpool_postgres::Pool;

use crate::{
    create_flow, get_flow, list_flows, save_flow, delete_flow, preview_flow,
};

pub fn router(pool: Pool) -> Router {
    Router::new()
        .route("/jaxrs/processplatform/assemble/designer/create", post(create_flow))
        .route("/jaxrs/processplatform/assemble/designer/get/{id}", get(get_flow))
        .route("/jaxrs/processplatform/assemble/designer/list/{category}", get(list_flows))
        .route("/jaxrs/processplatform/assemble/designer/save/{id}", post(save_flow))
        .route("/jaxrs/processplatform/assemble/designer/delete/{id}", post(delete_flow))
        .route("/jaxrs/processplatform/assemble/designer/preview/{id}", get(preview_flow))
        // ── application 族 ────────────────────────────────────────────────────
        .route("/jaxrs/processplatform/assemble/designer/application/{id}", get(crate::application_id).put(crate::application_id_update))
        .route("/jaxrs/processplatform/assemble/designer/application/icon/{id}", get(crate::application_id_icon))
        .route("/jaxrs/processplatform/assemble/designer/application/{id}/icon", put(crate::application_id_icon_update))
        .route("/jaxrs/processplatform/assemble/designer/application/{id}/permission", post(crate::application_id_permission_save))
        .route("/jaxrs/processplatform/assemble/designer/application/{id}/{onlyRemoveNotCompleted}", post(crate::application_id_onlyRemoveNotCompleted))
        .route("/jaxrs/processplatform/assemble/designer/application/permission/{id}", get(crate::application_id_permission))
        .route("/jaxrs/processplatform/assemble/designer/application/list/applicationcategory/{applicationCategory}", get(crate::application_list_applicationcategory_applicationCategory))
        .route("/jaxrs/processplatform/assemble/designer/application/list/summary/applicationcategory/{applicationCategory}", get(crate::application_list_summary_applicationcategory_applicationCategory))
        // ── applicationcategory 族 ────────────────────────────────────────────
        .route("/jaxrs/processplatform/assemble/designer/applicationcategory/list", get(crate::applicationcategory_list))
        // ── applicationdict 族 ────────────────────────────────────────────────
        .route("/jaxrs/processplatform/assemble/designer/applicationdict", post(crate::applicationdict_create))
        .route("/jaxrs/processplatform/assemble/designer/applicationdict/{id}", get(crate::applicationdict_id).delete(crate::applicationdict_id_delete))
        .route("/jaxrs/processplatform/assemble/designer/applicationdict/list/application/{applicationId}", get(crate::applicationdict_list_application_applicationId))
        .route("/jaxrs/processplatform/assemble/designer/applicationdict/list/paging/{page}/size/{size}", post(crate::applicationdict_paging_post))
        .route("/jaxrs/processplatform/assemble/designer/applicationdict/list/paging/{page}/{size}/{size}", get(crate::applicationdict_list_paging_page_size_size))
        // ── elementtool 孤儿检测 ──────────────────────────────────────────────
        .route("/jaxrs/processplatform/assemble/designer/elementtool/applicationdict/orphan", get(crate::elementtool_applicationdict_orphan))
        .route("/jaxrs/processplatform/assemble/designer/elementtool/form/orphan", get(crate::elementtool_form_orphan))
        .route("/jaxrs/processplatform/assemble/designer/elementtool/process/orphan", get(crate::elementtool_process_orphan))
        .route("/jaxrs/processplatform/assemble/designer/elementtool/script/orphan", get(crate::elementtool_script_orphan))
        // ── file 族 ───────────────────────────────────────────────────────────
        .route("/jaxrs/processplatform/assemble/designer/file/{flag}", get(crate::file_flag))
        .route("/jaxrs/processplatform/assemble/designer/file/{flag}/application/{applicationFlag}", get(crate::file_flag_in_application))
        .route("/jaxrs/processplatform/assemble/designer/file/application/{flag}/{applicationFlag}", get(crate::file_flag_application_applicationFlag))
        .route("/jaxrs/processplatform/assemble/designer/file/content/{id}", get(crate::file_id_content))
        .route("/jaxrs/processplatform/assemble/designer/file/download/{id}", get(crate::file_id_download))
        .route("/jaxrs/processplatform/assemble/designer/file/upload/{id}", post(crate::file_id_upload))
        .route("/jaxrs/processplatform/assemble/designer/file/list/application/{applicationFlag}", get(crate::file_list_application_applicationFlag))
        .route("/jaxrs/processplatform/assemble/designer/file/list/{id}/{next}/{count}", get(crate::file_list_id_next_count))
        // ── form 族 ───────────────────────────────────────────────────────────
        .route("/jaxrs/processplatform/assemble/designer/form/{id}", get(crate::form_id))
        .route("/jaxrs/processplatform/assemble/designer/form/list/application/{applicationId}", get(crate::form_list_application_applicationId))
        .route("/jaxrs/processplatform/assemble/designer/form/list/formfield/application/{applicationId}", get(crate::form_list_formfield_application_applicationId))
        .route("/jaxrs/processplatform/assemble/designer/form/list/formfield/{id}", get(crate::form_list_id_formfield))
        .route("/jaxrs/processplatform/assemble/designer/form/list/{id}/next/{count}", get(crate::form_list_next_exact))
        .route("/jaxrs/processplatform/assemble/designer/form/list/{id}/prev/{count}", get(crate::form_list_prev_exact))
        .route("/jaxrs/processplatform/assemble/designer/form/list/{id}/{next}/{count}", get(crate::form_list_id_next_count))
        // ── formversion / id 族 ───────────────────────────────────────────────
        .route("/jaxrs/processplatform/assemble/designer/formversion/{id}", get(crate::formversion_id))
        .route("/jaxrs/processplatform/assemble/designer/formversion/list/form/{formId}", get(crate::formversion_list_form_formId))
        .route("/jaxrs/processplatform/assemble/designer/{id}/{count}", get(crate::id_count))
        // ── item-access 族（Java 精确形态）────────────────────────────────────
        .route("/jaxrs/processplatform/assemble/designer/item-access", post(crate::item_access_create))
        .route("/jaxrs/processplatform/assemble/designer/item-access/bach/save", post(crate::item_access_bach_save))
        .route("/jaxrs/processplatform/assemble/designer/item-access/delete/process/{processId}/path/{path}", delete(crate::item_access_delete_exact))
        .route("/jaxrs/processplatform/assemble/designer/item-access/path/{path}", get(crate::item_access_path_list))
        .route("/jaxrs/processplatform/assemble/designer/item-access/process/{processId}", get(crate::item_access_process_processId))
        .route("/jaxrs/processplatform/assemble/designer/item-access/process/{processId}/path/{path}", get(crate::item_access_process_path_list))
        .route("/jaxrs/processplatform/assemble/designer/item-access/{id}", get(crate::item_access_id))
        // ── item access 兼容旧注册（下划线形态）───────────────────────────────
        .route("/jaxrs/processplatform/assemble/designer/item/access/delete/process/path/path/{processId}", post(crate::item_access_delete_process_processId_path_path))
        .route("/jaxrs/processplatform/assemble/designer/item/access/path/path", get(crate::item_access_path_path))
        .route("/jaxrs/processplatform/assemble/designer/item/access/process/path/path/{processId}", get(crate::item_access_process_processId_path_path))
        // ── mapping 族 ────────────────────────────────────────────────────────
        .route("/jaxrs/processplatform/assemble/designer/mapping", post(crate::mapping_create))
        .route("/jaxrs/processplatform/assemble/designer/mapping/{flag}", get(crate::mapping_flag).put(crate::mapping_id_update).delete(crate::mapping_id_delete))
        .route("/jaxrs/processplatform/assemble/designer/mapping/{flag}/execute", get(crate::mapping_flag_execute))
        .route("/jaxrs/processplatform/assemble/designer/mapping/execute/{flag}", get(crate::mapping_flag_execute))
        .route("/jaxrs/processplatform/assemble/designer/mapping/list/application/{applicationFlag}", get(crate::mapping_list_application_applicationFlag))
        .route("/jaxrs/processplatform/assemble/designer/mapping/list/{id}/next/{count}", get(crate::mapping_list_next_exact))
        .route("/jaxrs/processplatform/assemble/designer/mapping/list/{id}/prev/{count}", get(crate::mapping_list_prev_exact))
        .route("/jaxrs/processplatform/assemble/designer/mapping/list/{id}/{next}/{count}", get(crate::mapping_list_id_next_count))
        // ── mergeitemplan 族 ──────────────────────────────────────────────────
        .route("/jaxrs/processplatform/assemble/designer/mergeitemplan", post(crate::mergeitemplan_create))
        .route("/jaxrs/processplatform/assemble/designer/mergeitemplan/estimate", post(crate::mergeitemplan_estimate))
        .route("/jaxrs/processplatform/assemble/designer/mergeitemplan/{id}", get(crate::mergeitemplan_id).post(crate::mergeitemplan_id).put(crate::mergeitemplan_id_update).delete(crate::mergeitemplan_id_delete))
        .route("/jaxrs/processplatform/assemble/designer/mergeitemplan/list/application/{applicationId}/paging/{page}/size/{size}", get(crate::mergeitemplan_paging_by_application))
        .route("/jaxrs/processplatform/assemble/designer/mergeitemplan/list/application/paging/{applicationId}/{page}/{size}/{size}", post(crate::mergeitemplan_list_application_applicationId_paging_page_size_size))
        .route("/jaxrs/processplatform/assemble/designer/mergeitemplan/list/paging/{page}/size/{size}", get(crate::mergeitemplan_paging_exact))
        .route("/jaxrs/processplatform/assemble/designer/mergeitemplan/list/paging/{page}/{size}/{size}", post(crate::mergeitemplan_list_paging_page_size_size))
        // ── output 族 ─────────────────────────────────────────────────────────
        .route("/jaxrs/processplatform/assemble/designer/output/select/{applicationFlag}", get(crate::output_applicationFlag_select))
        // ── process 族 ────────────────────────────────────────────────────────
        .route("/jaxrs/processplatform/assemble/designer/process/activity/{flag}/{activityType}/{activityType}", get(crate::process_activity_flag_activityType_activityType))
        .route("/jaxrs/processplatform/assemble/designer/process/application/{applicationId}", get(crate::process_application_applicationId))
        .route("/jaxrs/processplatform/assemble/designer/process/application/{applicationId}/disable/edition", get(crate::process_disable_edition_list))
        .route("/jaxrs/processplatform/assemble/designer/process/application/{applicationId}/edition/{edition}", get(crate::process_edition_list))
        .route("/jaxrs/processplatform/assemble/designer/process/application/disable/edition/{applicationId}", post(crate::process_application_applicationId_disable_edition))
        .route("/jaxrs/processplatform/assemble/designer/process/application/edition/edition/{applicationId}", post(crate::process_application_applicationId_edition_edition))
        .route("/jaxrs/processplatform/assemble/designer/process/form/{formId}", get(crate::process_form_formId))
        .route("/jaxrs/processplatform/assemble/designer/process/upgrade/all", get(crate::process_upgrade_all))
        .route("/jaxrs/processplatform/assemble/designer/process/{id}", get(crate::process_id).put(crate::process_id_update))
        .route("/jaxrs/processplatform/assemble/designer/process/{id}/disable", get(crate::process_id_disable))
        .route("/jaxrs/processplatform/assemble/designer/process/{id}/enable", get(crate::process_id_enable))
        .route("/jaxrs/processplatform/assemble/designer/process/{id}/enabled", get(crate::process_id_enabled))
        .route("/jaxrs/processplatform/assemble/designer/process/{id}/execute/projection", post(crate::process_id_execute_projection))
        .route("/jaxrs/processplatform/assemble/designer/process/{id}/lead/out", get(crate::process_id_lead_out))
        .route("/jaxrs/processplatform/assemble/designer/process/{id}/list/element", post(crate::process_id_list_element))
        .route("/jaxrs/processplatform/assemble/designer/process/{id}/permission", post(crate::process_id_permission_save))
        .route("/jaxrs/processplatform/assemble/designer/process/{id}/process", get(crate::process_id_process))
        .route("/jaxrs/processplatform/assemble/designer/process/{id}/upgrade", post(crate::process_id_upgrade))
        .route("/jaxrs/processplatform/assemble/designer/process/{id}/{onlyRemoveNotCompleted}", post(crate::process_id_onlyRemoveNotCompleted))
        .route("/jaxrs/processplatform/assemble/designer/process/{id}/{onlyRemoveNotCompleted}/edition", delete(crate::process_edition_delete))
        .route("/jaxrs/processplatform/assemble/designer/process/disable/{id}", get(crate::process_id_disable))
        .route("/jaxrs/processplatform/assemble/designer/process/enable/{id}", get(crate::process_id_enable))
        .route("/jaxrs/processplatform/assemble/designer/process/enabled/{id}", get(crate::process_id_enabled))
        .route("/jaxrs/processplatform/assemble/designer/process/execute/projection/{id}", get(crate::process_id_execute_projection))
        .route("/jaxrs/processplatform/assemble/designer/process/lead/out/{id}", get(crate::process_id_lead_out))
        .route("/jaxrs/processplatform/assemble/designer/process/list/element/{id}", get(crate::process_id_list_element))
        .route("/jaxrs/processplatform/assemble/designer/process/permission/{id}", get(crate::process_id_permission))
        .route("/jaxrs/processplatform/assemble/designer/process/process/{id}", get(crate::process_id_process))
        .route("/jaxrs/processplatform/assemble/designer/process/edition/{id}/{onlyRemoveNotCompleted}", post(crate::process_id_onlyRemoveNotCompleted_edition))
        .route("/jaxrs/processplatform/assemble/designer/process/upgrade/{id}", get(crate::process_id_upgrade))
        // ── processversion 族 ─────────────────────────────────────────────────
        .route("/jaxrs/processplatform/assemble/designer/processversion/{id}", get(crate::processversion_id))
        .route("/jaxrs/processplatform/assemble/designer/processversion/list/process/{processId}", get(crate::processversion_list_process_processId))
        // ── script 族 ─────────────────────────────────────────────────────────
        .route("/jaxrs/processplatform/assemble/designer/script/application/{applicationId}", get(crate::script_application_applicationId))
        .route("/jaxrs/processplatform/assemble/designer/script/application/{applicationId}/name/{name}", get(crate::script_by_name_exact))
        .route("/jaxrs/processplatform/assemble/designer/script/application/{applicationId}/{name}/{name}", get(crate::script_application_applicationId_name_name))
        .route("/jaxrs/processplatform/assemble/designer/script/{id}", get(crate::script_id))
        .route("/jaxrs/processplatform/assemble/designer/script/list/{id}/{next}/{count}", get(crate::script_list_id_next_count))
        .route("/jaxrs/processplatform/assemble/designer/script/list/paging/{page}/{size}/{size}", get(crate::script_list_paging_page_size_size))
        .route("/jaxrs/processplatform/assemble/designer/scriptversion/{id}", get(crate::scriptversion_id))
        .route("/jaxrs/processplatform/assemble/designer/scriptversion/list/script/{scriptId}", get(crate::scriptversion_list_script_scriptId))
        // ── templateform 族 ───────────────────────────────────────────────────
        .route("/jaxrs/processplatform/assemble/designer/templateform/{id}", get(crate::templateform_id))
        .route("/jaxrs/processplatform/assemble/designer/templateform/list/{category}", get(crate::templateform_list_category))
        // ── workcompleted 合并数据（GET 精确形态）─────────────────────────────
        .route("/jaxrs/processplatform/assemble/designer/workcompleted/application/{applicationFlag}/merge/data", get(crate::workcompleted_application_applicationFlag_merge_data))
        .route("/jaxrs/processplatform/assemble/designer/workcompleted/process/{processFlag}/merge/data", get(crate::workcompleted_process_processFlag_merge_data))
        .route("/jaxrs/processplatform/assemble/designer/workcompleted/application/merge/data/{applicationFlag}", post(crate::workcompleted_application_applicationFlag_merge_data))
        .route("/jaxrs/processplatform/assemble/designer/workcompleted/process/merge/data/{processFlag}", post(crate::workcompleted_process_processFlag_merge_data))
        .layer(Extension(pool))
}
