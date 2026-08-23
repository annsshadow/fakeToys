use axum::{
    routing::{delete, get, post, put},
    Router,
};
use deadpool_postgres::Pool;

// 动词对齐说明：o2server 前端大量使用 PUT 承载 filter/list 查询、GET 承载部分触发操作。
// 本路由表在既有 handler 上链式注册同义动词（如 get(h).put(h)），使 PUT/GET 均可达，
// handler 本体不变（仍是真实参数化 SQL 操作）。

pub fn attendance_assemble_control_routes(pool: Pool) -> Router {
    Router::new()
        .route("/jaxrs/attendance/assemble/control/rule/list", get(crate::list_control_rules))
        // 同一路径只能注册一次：POST 与 PUT 链式合并（原两处独立注册会在 axum 构建时 panic）
        .route(
            "/jaxrs/attendance/assemble/control/rule/{id}/toggle",
            post(crate::toggle_control_rule).put(crate::toggle_control_rule),
        )
        .route("/jaxrs/attendance/assemble/control/attendanceadmin/list/all", get(crate::attendanceadmin_list_all))
        .route(
            "/jaxrs/attendance/assemble/control/attendanceadmin/{id}",
            get(crate::attendanceadmin_id).delete(crate::attendanceadmin_delete),
        )
        .route("/jaxrs/attendance/assemble/control/attendanceadmin", post(crate::attendanceadmin_create))
        .route("/jaxrs/attendance/assemble/control/attendanceappealInfo/appeal/{id}", post(crate::attendanceappealInfo_appeal_id).put(crate::attendanceappealInfo_appeal_id))
        .route("/jaxrs/attendance/assemble/control/attendanceappealInfo/archive/{id}", post(crate::attendanceappealInfo_archive_id).get(crate::attendanceappealInfo_archive_id))
        .route("/jaxrs/attendance/assemble/control/attendanceappealInfo/audit", post(crate::attendanceappealInfo_audit).put(crate::attendanceappealInfo_audit))
        .route("/jaxrs/attendance/assemble/control/attendanceappealInfo/check", post(crate::attendanceappealInfo_check).put(crate::attendanceappealInfo_check))
        .route("/jaxrs/attendance/assemble/control/attendanceappealInfo/filter/list/{id}/next/{count}", get(crate::attendanceappealInfo_filter_list_id_next_count).put(crate::attendanceappealInfo_filter_list_id_next_count))
        .route("/jaxrs/attendance/assemble/control/attendanceappealInfo/filter/list/{id}/prev/{count}", get(crate::attendanceappealInfo_filter_list_id_prev_count).put(crate::attendanceappealInfo_filter_list_id_prev_count))
        .route("/jaxrs/attendance/assemble/control/attendanceappealInfo/manager/list/{id}/next/{count}", get(crate::attendanceappealInfo_manager_list_id_next_count).put(crate::attendanceappealInfo_manager_list_id_next_count))
        .route("/jaxrs/attendance/assemble/control/attendanceappealInfo/workflow/appeal/{id}", post(crate::attendanceappealInfo_workflow_appeal_id).put(crate::attendanceappealInfo_workflow_appeal_id))
        .route("/jaxrs/attendance/assemble/control/attendanceappealInfo/workflow/sync", post(crate::attendanceappealInfo_workflow_sync).put(crate::attendanceappealInfo_workflow_sync))
        .route(
            "/jaxrs/attendance/assemble/control/attendanceappealInfo/{id}",
            get(crate::attendanceappealInfo_id).delete(crate::attendanceappealInfo_delete),
        )
        .route("/jaxrs/attendance/assemble/control/attendancedetail/analyse", post(crate::attendancedetail_analyse))
        .route("/jaxrs/attendance/assemble/control/attendancedetail/analyse/id/{id}", post(crate::attendancedetail_analyse_id_id).get(crate::attendancedetail_analyse_id_id))
        .route("/jaxrs/attendance/assemble/control/attendancedetail/analyse/redo", post(crate::attendancedetail_analyse_redo).put(crate::attendancedetail_analyse_redo))
        .route("/jaxrs/attendance/assemble/control/attendancedetail/analyse/{startDate}/{endDate}", post(crate::attendancedetail_analyse_startDate_endDate).get(crate::attendancedetail_analyse_startDate_endDate))
        .route("/jaxrs/attendance/assemble/control/attendancedetail/archive/{id}", post(crate::attendancedetail_archive_id).get(crate::attendancedetail_archive_id))
        .route("/jaxrs/attendance/assemble/control/attendancedetail/checkDetailWithPersonByCycle/{cycleYear}/{cycleMonth}", post(crate::attendancedetail_checkDetailWithPersonByCycle_cycleYear_cycleMonth).get(crate::attendancedetail_checkDetailWithPersonByCycle_cycleYear_cycleMonth))
        .route("/jaxrs/attendance/assemble/control/attendancedetail/filter/list", get(crate::attendancedetail_filter_list).put(crate::attendancedetail_filter_list))
        .route("/jaxrs/attendance/assemble/control/attendancedetail/filter/list/topUnit", get(crate::attendancedetail_filter_list_topUnit).put(crate::attendancedetail_filter_list_topUnit))
        .route("/jaxrs/attendance/assemble/control/attendancedetail/filter/list/unit", get(crate::attendancedetail_filter_list_unit).put(crate::attendancedetail_filter_list_unit))
        .route("/jaxrs/attendance/assemble/control/attendancedetail/filter/list/user", get(crate::attendancedetail_filter_list_user).put(crate::attendancedetail_filter_list_user))
        .route("/jaxrs/attendance/assemble/control/attendancedetail/filter/list/{id}/next/{count}", get(crate::attendancedetail_filter_list_id_next_count).put(crate::attendancedetail_filter_list_id_next_count))
        .route("/jaxrs/attendance/assemble/control/attendancedetail/filter/list/{id}/prev/{count}", get(crate::attendancedetail_filter_list_id_prev_count).put(crate::attendancedetail_filter_list_id_prev_count))
        .route("/jaxrs/attendance/assemble/control/attendancedetail/list/persons/nonesign", get(crate::attendancedetail_list_persons_nonesign).put(crate::attendancedetail_list_persons_nonesign))
        .route(
            "/jaxrs/attendance/assemble/control/attendancedetail/list/{file_id}",
            get(crate::attendancedetail_list_file_id),
        )
        .route("/jaxrs/attendance/assemble/control/attendancedetail/mobile/filter/list/page/{page}/count/{count}", get(crate::attendancedetail_mobile_filter_list_page_page_count_count).put(crate::attendancedetail_mobile_filter_list_page_page_count_count))
        .route("/jaxrs/attendance/assemble/control/attendancedetail/mobile/mobilepreview", post(crate::attendancedetail_mobile_mobilepreview).get(crate::attendancedetail_mobile_mobilepreview))
        .route("/jaxrs/attendance/assemble/control/attendancedetail/mobile/my", post(crate::attendancedetail_mobile_my).get(crate::attendancedetail_mobile_my))
        .route("/jaxrs/attendance/assemble/control/attendancedetail/mobile/recive", post(crate::attendancedetail_mobile_recive))
        .route(
            "/jaxrs/attendance/assemble/control/attendancedetail/mobile/{id}",
            get(crate::attendancedetail_mobile_id).delete(crate::attendancedetail_mobile_delete),
        )
        .route("/jaxrs/attendance/assemble/control/attendancedetail/recive", post(crate::attendancedetail_recive))
        .route("/jaxrs/attendance/assemble/control/attendancedetail/reciveSingle", post(crate::attendancedetail_reciveSingle))
        .route(
            "/jaxrs/attendance/assemble/control/attendancedetail/{id}",
            get(crate::attendancedetail_id).delete(crate::attendancedetail_delete),
        )
        .route("/jaxrs/attendance/assemble/control/attendancedetail", post(crate::attendancedetail_create))
        .route("/jaxrs/attendance/assemble/control/attendanceemployeeconfig/list/all", get(crate::attendanceemployeeconfig_list_all))
        .route(
            "/jaxrs/attendance/assemble/control/attendanceemployeeconfig/{id}",
            get(crate::attendanceemployeeconfig_id).delete(crate::attendanceemployeeconfig_delete),
        )
        .route("/jaxrs/attendance/assemble/control/attendanceemployeeconfig", post(crate::attendanceemployeeconfig_create))
        .route("/jaxrs/attendance/assemble/control/attendanceimportfileinfo/list/all", get(crate::attendanceimportfileinfo_list_all))
        .route(
            "/jaxrs/attendance/assemble/control/attendanceimportfileinfo/{id}",
            get(crate::attendanceimportfileinfo_id).delete(crate::attendanceimportfileinfo_delete),
        )
        .route("/jaxrs/attendance/assemble/control/attendanceschedulesetting/list/all", get(crate::attendanceschedulesetting_list_all))
        .route("/jaxrs/attendance/assemble/control/attendanceschedulesetting/list/topUnit/{name}", get(crate::attendanceschedulesetting_list_topUnit_name))
        .route("/jaxrs/attendance/assemble/control/attendanceschedulesetting/list/unit/{name}", get(crate::attendanceschedulesetting_list_unit_name))
        .route(
            "/jaxrs/attendance/assemble/control/attendanceschedulesetting/{id}",
            get(crate::attendanceschedulesetting_id).delete(crate::attendanceschedulesetting_delete),
        )
        .route("/jaxrs/attendance/assemble/control/attendanceschedulesetting", post(crate::attendanceschedulesetting_create))
        .route("/jaxrs/attendance/assemble/control/attendanceselfholiday/filter/list/{id}/next/{count}", get(crate::attendanceselfholiday_filter_list_id_next_count).put(crate::attendanceselfholiday_filter_list_id_next_count))
        .route("/jaxrs/attendance/assemble/control/attendanceselfholiday/filter/list/{id}/prev/{count}", get(crate::attendanceselfholiday_filter_list_id_prev_count).put(crate::attendanceselfholiday_filter_list_id_prev_count))
        .route("/jaxrs/attendance/assemble/control/attendanceselfholiday/list/all", get(crate::attendanceselfholiday_list_all))
        .route(
            "/jaxrs/attendance/assemble/control/attendanceselfholiday/{id}",
            get(crate::attendanceselfholiday_id).delete(crate::attendanceselfholiday_delete),
        )
        .route("/jaxrs/attendance/assemble/control/attendanceselfholiday", post(crate::attendanceselfholiday_create))
        .route("/jaxrs/attendance/assemble/control/attendancesetting/code/{code}", get(crate::attendancesetting_code_code))
        .route("/jaxrs/attendance/assemble/control/attendancesetting/enable/type", post(crate::attendancesetting_enable_type).get(crate::attendancesetting_enable_type))
        .route("/jaxrs/attendance/assemble/control/attendancesetting/list/all", get(crate::attendancesetting_list_all))
        .route(
            "/jaxrs/attendance/assemble/control/attendancesetting/{id}",
            get(crate::attendancesetting_id).delete(crate::attendancesetting_delete),
        )
        .route("/jaxrs/attendance/assemble/control/attendancesetting", post(crate::attendancesetting_create))
        .route("/jaxrs/attendance/assemble/control/attendancestatisticalcycle/cycleDetail/{year}/{month}", get(crate::attendancestatisticalcycle_cycleDetail_year_month))
        .route("/jaxrs/attendance/assemble/control/attendancestatisticalcycle/list/all", get(crate::attendancestatisticalcycle_list_all))
        .route(
            "/jaxrs/attendance/assemble/control/attendancestatisticalcycle/{id}",
            get(crate::attendancestatisticalcycle_id).delete(crate::attendancestatisticalcycle_delete),
        )
        .route("/jaxrs/attendance/assemble/control/attendancestatisticalcycle", post(crate::attendancestatisticalcycle_create))
        .route("/jaxrs/attendance/assemble/control/attendancestatisticrequirelog/list/all", get(crate::attendancestatisticrequirelog_list_all))
        .route(
            "/jaxrs/attendance/assemble/control/attendancestatisticrequirelog/{id}",
            get(crate::attendancestatisticrequirelog_id).delete(crate::attendancestatisticrequirelog_delete),
        )
        .route("/jaxrs/attendance/assemble/control/attendancestatisticrequirelog", post(crate::attendancestatisticrequirelog_create))
        .route("/jaxrs/attendance/assemble/control/attendanceworkdayconfig/filter", post(crate::attendanceworkdayconfig_filter).put(crate::attendanceworkdayconfig_filter))
        .route("/jaxrs/attendance/assemble/control/attendanceworkdayconfig/list/all", get(crate::attendanceworkdayconfig_list_all))
        .route(
            "/jaxrs/attendance/assemble/control/attendanceworkdayconfig/{id}",
            get(crate::attendanceworkdayconfig_id).delete(crate::attendanceworkdayconfig_delete),
        )
        .route("/jaxrs/attendance/assemble/control/attendanceworkdayconfig", post(crate::attendanceworkdayconfig_create))
        .route(
            "/jaxrs/attendance/assemble/control/selfholidaysimple/docId/{docId}",
            get(crate::selfholidaysimple_docId_docId).delete(crate::selfholidaysimple_docId_docId_delete),
        )
        .route("/jaxrs/attendance/assemble/control/selfholidaysimple", post(crate::selfholidaysimple_create))
        .route("/jaxrs/attendance/assemble/control/statistic/do", post(crate::statistic_do).get(crate::statistic_do))
        .route("/jaxrs/attendance/assemble/control/statisticshow/filter/personMonth/list/{id}/next/{count}", get(crate::statisticshow_filter_personMonth_list_id_next_count).put(crate::statisticshow_filter_personMonth_list_id_next_count))
        .route("/jaxrs/attendance/assemble/control/statisticshow/filter/personMonth/list/{id}/prev/{count}", get(crate::statisticshow_filter_personMonth_list_id_prev_count).put(crate::statisticshow_filter_personMonth_list_id_prev_count))
        .route("/jaxrs/attendance/assemble/control/statisticshow/filter/topUnitDay/list/{id}/next/{count}", get(crate::statisticshow_filter_topUnitDay_list_id_next_count).put(crate::statisticshow_filter_topUnitDay_list_id_next_count))
        .route("/jaxrs/attendance/assemble/control/statisticshow/filter/topUnitDay/list/{id}/prev/{count}", get(crate::statisticshow_filter_topUnitDay_list_id_prev_count).put(crate::statisticshow_filter_topUnitDay_list_id_prev_count))
        .route("/jaxrs/attendance/assemble/control/statisticshow/filter/topUnitMonth/list/{id}/next/{count}", get(crate::statisticshow_filter_topUnitMonth_list_id_next_count).put(crate::statisticshow_filter_topUnitMonth_list_id_next_count))
        .route("/jaxrs/attendance/assemble/control/statisticshow/filter/topUnitMonth/list/{id}/prev/{count}", get(crate::statisticshow_filter_topUnitMonth_list_id_prev_count).put(crate::statisticshow_filter_topUnitMonth_list_id_prev_count))
        .route("/jaxrs/attendance/assemble/control/statisticshow/filter/unitDay/list/{id}/next/{count}", get(crate::statisticshow_filter_unitDay_list_id_next_count).put(crate::statisticshow_filter_unitDay_list_id_next_count))
        .route("/jaxrs/attendance/assemble/control/statisticshow/filter/unitDay/list/{id}/prev/{count}", get(crate::statisticshow_filter_unitDay_list_id_prev_count).put(crate::statisticshow_filter_unitDay_list_id_prev_count))
        .route("/jaxrs/attendance/assemble/control/statisticshow/filter/unitMonth/list/{id}/next/{count}", get(crate::statisticshow_filter_unitMonth_list_id_next_count).put(crate::statisticshow_filter_unitMonth_list_id_next_count))
        .route("/jaxrs/attendance/assemble/control/statisticshow/filter/unitMonth/list/{id}/prev/{count}", get(crate::statisticshow_filter_unitMonth_list_id_prev_count).put(crate::statisticshow_filter_unitMonth_list_id_prev_count))
        .route("/jaxrs/attendance/assemble/control/statisticshow/person/{name}/{year}/{month}", get(crate::statisticshow_person_name_year_month))
        .route("/jaxrs/attendance/assemble/control/statisticshow/persons/unit/subnested/{name}/{year}/{month}", get(crate::statisticshow_persons_unit_subnested_name_year_month))
        .route("/jaxrs/attendance/assemble/control/statisticshow/persons/unit/{name}/{year}/{month}", get(crate::statisticshow_persons_unit_name_year_month))
        .route("/jaxrs/attendance/assemble/control/statisticshow/topUnit/day/{name}/{year}/{month}", get(crate::statisticshow_topUnit_day_name_year_month))
        .route("/jaxrs/attendance/assemble/control/statisticshow/topUnit/{name}/{year}/{month}", get(crate::statisticshow_topUnit_name_year_month))
        .route("/jaxrs/attendance/assemble/control/statisticshow/unit/day/topUnit/{name}/{date}", get(crate::statisticshow_unit_day_topUnit_name_date))
        .route("/jaxrs/attendance/assemble/control/statisticshow/unit/day/{name}/{date}", get(crate::statisticshow_unit_day_name_date))
        .route("/jaxrs/attendance/assemble/control/statisticshow/unit/day/{name}/{year}/{month}", get(crate::statisticshow_unit_day_name_year_month))
        .route("/jaxrs/attendance/assemble/control/statisticshow/unit/subnested/{name}/{year}/{month}", get(crate::statisticshow_unit_subnested_name_year_month))
        .route("/jaxrs/attendance/assemble/control/statisticshow/unit/sum/{name}/{year}/{month}", get(crate::statisticshow_unit_sum_name_year_month))
        .route("/jaxrs/attendance/assemble/control/statisticshow/unit/topUnit/{name}/{year}/{month}", get(crate::statisticshow_unit_topUnit_name_year_month))
        .route("/jaxrs/attendance/assemble/control/statisticshow/unit/{name}/{year}/{month}", get(crate::statisticshow_unit_name_year_month))
        .route("/jaxrs/attendance/assemble/control/uuid/random", get(crate::uuid_random))
        .route("/jaxrs/attendance/assemble/control/workplace/list/all", get(crate::workplace_list_all))
        .route(
            "/jaxrs/attendance/assemble/control/workplace/{id}",
            get(crate::workplace_id).delete(crate::workplace_delete),
        )
        .route("/jaxrs/attendance/assemble/control/workplace", post(crate::workplace_create))
        // v2 group
        .route("/jaxrs/attendance/assemble/control/v2/group", post(crate::v2_group_create))
        .route("/jaxrs/attendance/assemble/control/v2/group/list/{page}/size/{size}", post(crate::v2_group_list_page_size))
        .route("/jaxrs/attendance/assemble/control/v2/group/person/{person}/date/{date}", get(crate::v2_group_person_date))
        .route("/jaxrs/attendance/assemble/control/v2/group/{id}", get(crate::v2_group_get))
        .route("/jaxrs/attendance/assemble/control/v2/group/{id}/delete", get(crate::v2_group_delete))
        .route("/jaxrs/attendance/assemble/control/v2/group/{id}/refresh/participate", get(crate::v2_group_refresh_participate))
        // v2 shift
        .route("/jaxrs/attendance/assemble/control/v2/shift/create", post(crate::v2_shift_create))
        .route("/jaxrs/attendance/assemble/control/v2/shift/update", post(crate::v2_shift_update))
        .route("/jaxrs/attendance/assemble/control/v2/shift/list/{page}/size/{size}", post(crate::v2_shift_list_page_size))
        .route("/jaxrs/attendance/assemble/control/v2/shift/delete/{id}", get(crate::v2_shift_delete))
        .route("/jaxrs/attendance/assemble/control/v2/shift/{id}", get(crate::v2_shift_get))
        // v2 leave
        .route("/jaxrs/attendance/assemble/control/v2/leave", post(crate::v2_leave_create))
        .route("/jaxrs/attendance/assemble/control/v2/leave/delete/{id}", get(crate::v2_leave_delete))
        .route("/jaxrs/attendance/assemble/control/v2/leave/import", post(crate::v2_leave_import))
        .route("/jaxrs/attendance/assemble/control/v2/leave/import/result/flag/{flag}", get(crate::v2_leave_import_result_flag))
        .route("/jaxrs/attendance/assemble/control/v2/leave/list/{page}/size/{size}", post(crate::v2_leave_list_page_size))
        // v2 config：同路径双方法链式注册
        .route(
            "/jaxrs/attendance/assemble/control/v2/config",
            get(crate::v2_config_get).post(crate::v2_config_post),
        )
        .route(
            "/jaxrs/attendance/assemble/control/v2/config/person",
            get(crate::v2_config_person_get).post(crate::v2_config_person_post),
        )
        // v2 record / detail / my
        .route("/jaxrs/attendance/assemble/control/v2/record/{id}", get(crate::v2_record_get))
        .route("/jaxrs/attendance/assemble/control/v2/record/list/{page}/size/{size}", post(crate::v2_record_list_page_size))
        .route("/jaxrs/attendance/assemble/control/v2/detail/list/{page}/size/{size}", post(crate::v2_detail_list_page_size))
        .route("/jaxrs/attendance/assemble/control/v2/my/statistic", post(crate::v2_my_statistic))
        // ── plan002 U2 legacy 族闭合新增注册 ──────────────────────────
        // dingding / qywx
        .route(
            "/jaxrs/attendance/assemble/control/dingding/all",
            delete(crate::dingding_delete_all),
        )
        .route(
            "/jaxrs/attendance/assemble/control/dingding/sync/from/{from}/to/{to}/start",
            get(crate::dingding_sync_start),
        )
        .route("/jaxrs/attendance/assemble/control/dingding/sync/list", get(crate::dingding_sync_list))
        .route(
            "/jaxrs/attendance/assemble/control/dingding/attendance/list/{id}/next/{count}",
            put(crate::dingding_attendance_list_next),
        )
        .route(
            "/jaxrs/attendance/assemble/control/dingding/statistic/person/year/{year}/month/{month}",
            get(crate::dingding_statistic_person_trigger),
        )
        .route(
            "/jaxrs/attendance/assemble/control/dingding/statistic/unit/year/{year}/month/{month}/day/{day}",
            get(crate::dingding_statistic_unit_day_trigger),
        )
        .route(
            "/jaxrs/attendance/assemble/control/dingdingstatistic/person/{person}/{year}/{month}",
            get(crate::dingdingstatistic_person),
        )
        .route(
            "/jaxrs/attendance/assemble/control/dingdingstatistic/person/unit/{unit}/{year}/{month}",
            get(crate::dingdingstatistic_person_unit),
        )
        .route(
            "/jaxrs/attendance/assemble/control/dingdingstatistic/unit/{unit}/{year}/{month}",
            get(crate::dingdingstatistic_unit),
        )
        .route(
            "/jaxrs/attendance/assemble/control/qywx/all",
            delete(crate::qywx_delete_all),
        )
        .route(
            "/jaxrs/attendance/assemble/control/qywx/sync/from/{from}/to/{to}/start",
            get(crate::qywx_sync_start),
        )
        .route("/jaxrs/attendance/assemble/control/qywx/sync/list", get(crate::qywx_sync_list))
        .route(
            "/jaxrs/attendance/assemble/control/qywx/attendance/list/{id}/next/{count}",
            put(crate::qywx_attendance_list_next),
        )
        .route(
            "/jaxrs/attendance/assemble/control/qywx/statistic/person/year/{year}/month/{month}",
            get(crate::qywx_statistic_person_trigger),
        )
        .route(
            "/jaxrs/attendance/assemble/control/qywx/statistic/unit/year/{year}/month/{month}/day/{day}",
            get(crate::qywx_statistic_unit_day_trigger),
        )
        .route(
            "/jaxrs/attendance/assemble/control/qywxstatistic/person/{person}/{year}/{month}",
            get(crate::qywxstatistic_person),
        )
        .route(
            "/jaxrs/attendance/assemble/control/qywxstatistic/person/unit/{unit}/{year}/{month}",
            get(crate::qywxstatistic_person_unit),
        )
        .route(
            "/jaxrs/attendance/assemble/control/qywxstatistic/unit/{unit}/{year}/{month}",
            get(crate::qywxstatistic_unit),
        )
        // v2 appeal
        .route(
            "/jaxrs/attendance/assemble/control/v2/appeal/list/{page}/size/{size}",
            post(crate::v2_appeal_list_page_size),
        )
        .route(
            "/jaxrs/attendance/assemble/control/v2/appeal/list/manager/{page}/size/{size}",
            post(crate::v2_appeal_manager_list_page_size),
        )
        .route("/jaxrs/attendance/assemble/control/v2/appeal/{id}", get(crate::v2_appeal_get))
        .route(
            "/jaxrs/attendance/assemble/control/v2/appeal/{id}/manager/status",
            get(crate::v2_appeal_manager_status),
        )
        .route(
            "/jaxrs/attendance/assemble/control/v2/appeal/{id}/start/check",
            get(crate::v2_appeal_start_check),
        )
        .route(
            "/jaxrs/attendance/assemble/control/v2/appeal/{id}/start/process",
            post(crate::v2_appeal_start_process),
        )
        .route(
            "/jaxrs/attendance/assemble/control/v2/appeal/{id}/reset/status",
            get(crate::v2_appeal_reset_status),
        )
        .route(
            "/jaxrs/attendance/assemble/control/v2/appeal/{id}/end/process",
            post(crate::v2_appeal_end_process),
        )
        // v2 detail
        .route(
            "/jaxrs/attendance/assemble/control/v2/detail/rebuild/person/{person}/date/{date}",
            get(crate::v2_detail_rebuild_person_date),
        )
        .route(
            "/jaxrs/attendance/assemble/control/v2/detail/statistic/{detailId}/list/record",
            get(crate::v2_detail_statistic_record_list),
        )
        .route(
            "/jaxrs/attendance/assemble/control/v2/detail/statistic/filter",
            post(crate::v2_detail_statistic_filter),
        )
        .route(
            "/jaxrs/attendance/assemble/control/v2/detail/statistic/export/filter",
            post(crate::v2_detail_statistic_export_filter),
        )
        // v2 group rebuild + groupschedule
        .route(
            "/jaxrs/attendance/assemble/control/v2/group/rebuild/detail/group/{groupId}/date/{date}",
            get(crate::v2_group_rebuild_detail_group_date),
        )
        .route(
            "/jaxrs/attendance/assemble/control/v2/groupschedule",
            post(crate::v2_groupschedule_post),
        )
        .route(
            "/jaxrs/attendance/assemble/control/v2/groupschedule/config/group/{groupId}",
            get(crate::v2_groupschedule_config_get),
        )
        .route(
            "/jaxrs/attendance/assemble/control/v2/groupschedule/list/group/{groupId}/month/{month}",
            get(crate::v2_groupschedule_list_group_month),
        )
        .route(
            "/jaxrs/attendance/assemble/control/v2/groupschedule/list/filter",
            post(crate::v2_groupschedule_list_filter),
        )
        // v2 leave template
        .route("/jaxrs/attendance/assemble/control/v2/leave/template", get(crate::v2_leave_template))
        // v2 mobile（Java 端 "check/ from/out" 路径含空格：同时注册空格与 %20 形态）
        .route("/jaxrs/attendance/assemble/control/v2/mobile/check/pre", get(crate::v2_mobile_pre_check))
        .route("/jaxrs/attendance/assemble/control/v2/mobile/check", post(crate::v2_mobile_check))
        .route(
            "/jaxrs/attendance/assemble/control/v2/mobile/check/ from/out",
            post(crate::v2_mobile_check_from_out),
        )
        .route(
            "/jaxrs/attendance/assemble/control/v2/mobile/check/%20from/out",
            post(crate::v2_mobile_check_from_out),
        )
        // v2 my
        .route("/jaxrs/attendance/assemble/control/v2/my/version", get(crate::v2_my_version))
        .route("/jaxrs/attendance/assemble/control/v2/my/controls", get(crate::v2_my_controls))
        .route("/jaxrs/attendance/assemble/control/v2/my/detail/list", post(crate::v2_my_detail_list))
        .route("/jaxrs/attendance/assemble/control/v2/my/rest/date/check", post(crate::v2_my_rest_date_check))
        // v2 record
        .route(
            "/jaxrs/attendance/assemble/control/v2/record/delete/people/{people}/date/{date}",
            get(crate::v2_record_delete_people_date),
        )
        .route("/jaxrs/attendance/assemble/control/v2/record/template", get(crate::v2_record_template))
        .route("/jaxrs/attendance/assemble/control/v2/record/import", post(crate::v2_record_import))
        .route("/jaxrs/attendance/assemble/control/v2/record/import/daily", post(crate::v2_record_import_daily))
        // v2 workplace
        .route("/jaxrs/attendance/assemble/control/v2/workplace", post(crate::v2_workplace_post))
        .route(
            "/jaxrs/attendance/assemble/control/v2/workplace/{id}",
            delete(crate::v2_workplace_delete),
        )
        .route("/jaxrs/attendance/assemble/control/v2/workplace/list/all", get(crate::v2_workplace_list_all))
        .route(
            "/jaxrs/attendance/assemble/control/v2/workplace/list/ids",
            post(crate::v2_workplace_list_ids),
        )
        .layer(axum::Extension(pool))
}
