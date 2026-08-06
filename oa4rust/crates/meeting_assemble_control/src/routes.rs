use axum::{
    routing::{delete, get, post},
    Router,
};
use deadpool_postgres::Pool;

use crate::{
    create_meeting, create_meeting_control, delete_meeting, delete_meeting_control,
    list_meeting_controls, meeting_id, save_meeting,
    stub_meeting_assemble_control_building_id,
    stub_meeting_assemble_control_building_list,
    stub_meeting_assemble_control_building_list_like_key,
    stub_meeting_assemble_control_building_list_like_pinyin_key,
    stub_meeting_assemble_control_building_list_pinyininitial_key,
    stub_meeting_assemble_control_config_system_config,
    stub_meeting_assemble_control_config_system_config_manage,
    stub_meeting_assemble_control_meeting_id_accept,
    stub_meeting_assemble_control_meeting_id_add_invite,
    stub_meeting_assemble_control_meeting_id_checkin,
    stub_meeting_assemble_control_meeting_id_confirm_allow,
    stub_meeting_assemble_control_meeting_id_confirm_deny,
    stub_meeting_assemble_control_meeting_id_delete_invite,
    stub_meeting_assemble_control_meeting_id_manual_completed,
    stub_meeting_assemble_control_meeting_id_modify_completedtime,
    stub_meeting_assemble_control_meeting_id_modify_starttime,
    stub_meeting_assemble_control_meeting_id_reject,
    stub_meeting_assemble_control_meeting_list_applied_completed,
    stub_meeting_assemble_control_meeting_list_applied_processing,
    stub_meeting_assemble_control_meeting_list_applied_wait,
    stub_meeting_assemble_control_meeting_list_apply_page_size_size,
    stub_meeting_assemble_control_meeting_list_coming_day_count,
    stub_meeting_assemble_control_meeting_list_invited_completed,
    stub_meeting_assemble_control_meeting_list_invited_processing,
    stub_meeting_assemble_control_meeting_list_invited_rejected,
    stub_meeting_assemble_control_meeting_list_invited_wait,
    stub_meeting_assemble_control_meeting_list_wait_accept,
    stub_meeting_assemble_control_meeting_list_wait_confirm,
    stub_meeting_assemble_control_meeting_list_year_year_month_month,
    stub_meeting_assemble_control_meeting_list_year_year_month_month_all,
    stub_meeting_assemble_control_meeting_list_year_year_month_month_day_day,
    stub_meeting_assemble_control_meeting_list_year_year_month_month_day_day_all,
    stub_meeting_assemble_control_openmeeting_list_room,
    stub_meeting_assemble_control_room_id,
    stub_meeting_assemble_control_room_list,
};

pub fn meeting_assemble_control_routes(pool: Pool) -> Router {
    Router::new()
        .route("/jaxrs/meeting/assemble/control/list/{meetingId}", get(list_meeting_controls))
        .route("/jaxrs/meeting/assemble/control/create", post(create_meeting_control))
        .route("/jaxrs/meeting/assemble/control/delete/{id}", delete(delete_meeting_control))
        .route("/jaxrs/meeting/assemble/control/building/list", get(stub_meeting_assemble_control_building_list))
        .route("/jaxrs/meeting/assemble/control/building/{id}", get(stub_meeting_assemble_control_building_id))
        .route("/jaxrs/meeting/assemble/control/building/list/like/{key}", get(stub_meeting_assemble_control_building_list_like_key))
        .route("/jaxrs/meeting/assemble/control/building/list/like/pinyin/{key}", get(stub_meeting_assemble_control_building_list_like_pinyin_key))
        .route("/jaxrs/meeting/assemble/control/building/list/pinyininitial/{key}", get(stub_meeting_assemble_control_building_list_pinyininitial_key))
        .route("/jaxrs/meeting/assemble/control/config/system/config", get(stub_meeting_assemble_control_config_system_config))
        .route("/jaxrs/meeting/assemble/control/config/system/config/manage", post(stub_meeting_assemble_control_config_system_config_manage))
        .route("/jaxrs/meeting/assemble/control/meeting/list/applied/completed", get(stub_meeting_assemble_control_meeting_list_applied_completed))
        .route("/jaxrs/meeting/assemble/control/meeting/list/applied/processing", get(stub_meeting_assemble_control_meeting_list_applied_processing))
        .route("/jaxrs/meeting/assemble/control/meeting/list/applied/wait", get(stub_meeting_assemble_control_meeting_list_applied_wait))
        .route("/jaxrs/meeting/assemble/control/meeting/list/apply/{page}/size/{size}", get(stub_meeting_assemble_control_meeting_list_apply_page_size_size))
        .route("/jaxrs/meeting/assemble/control/meeting/list/coming/day/{count}", get(stub_meeting_assemble_control_meeting_list_coming_day_count))
        .route("/jaxrs/meeting/assemble/control/meeting/list/invited/completed", get(stub_meeting_assemble_control_meeting_list_invited_completed))
        .route("/jaxrs/meeting/assemble/control/meeting/list/invited/processing", get(stub_meeting_assemble_control_meeting_list_invited_processing))
        .route("/jaxrs/meeting/assemble/control/meeting/list/invited/rejected", get(stub_meeting_assemble_control_meeting_list_invited_rejected))
        .route("/jaxrs/meeting/assemble/control/meeting/list/invited/wait", get(stub_meeting_assemble_control_meeting_list_invited_wait))
        .route("/jaxrs/meeting/assemble/control/meeting/list/wait/accept", get(stub_meeting_assemble_control_meeting_list_wait_accept))
        .route("/jaxrs/meeting/assemble/control/meeting/list/wait/confirm", get(stub_meeting_assemble_control_meeting_list_wait_confirm))
        .route("/jaxrs/meeting/assemble/control/meeting/list/year/{year}/month/{month}", get(stub_meeting_assemble_control_meeting_list_year_year_month_month))
        .route("/jaxrs/meeting/assemble/control/meeting/list/year/{year}/month/{month}/all", get(stub_meeting_assemble_control_meeting_list_year_year_month_month_all))
        .route("/jaxrs/meeting/assemble/control/meeting/list/year/{year}/month/{month}/day/{day}", get(stub_meeting_assemble_control_meeting_list_year_year_month_month_day_day))
        .route("/jaxrs/meeting/assemble/control/meeting/list/year/{year}/month/{month}/day/{day}/all", get(stub_meeting_assemble_control_meeting_list_year_year_month_month_day_day_all))
        .route("/jaxrs/meeting/assemble/control/meeting/create", post(create_meeting))
        .route("/jaxrs/meeting/assemble/control/meeting/{id}", get(meeting_id))
        .route("/jaxrs/meeting/assemble/control/meeting/save/{id}", post(save_meeting))
        .route("/jaxrs/meeting/assemble/control/meeting/delete/{id}", post(delete_meeting))
        .route("/jaxrs/meeting/assemble/control/meeting/{id}/accept", post(stub_meeting_assemble_control_meeting_id_accept))
        .route("/jaxrs/meeting/assemble/control/meeting/{id}/add/invite", post(stub_meeting_assemble_control_meeting_id_add_invite))
        .route("/jaxrs/meeting/assemble/control/meeting/{id}/checkin", post(stub_meeting_assemble_control_meeting_id_checkin))
        .route("/jaxrs/meeting/assemble/control/meeting/{id}/confirm/allow", post(stub_meeting_assemble_control_meeting_id_confirm_allow))
        .route("/jaxrs/meeting/assemble/control/meeting/{id}/confirm/deny", post(stub_meeting_assemble_control_meeting_id_confirm_deny))
        .route("/jaxrs/meeting/assemble/control/meeting/{id}/delete/invite", post(stub_meeting_assemble_control_meeting_id_delete_invite))
        .route("/jaxrs/meeting/assemble/control/meeting/{id}/manual/completed", post(stub_meeting_assemble_control_meeting_id_manual_completed))
        .route("/jaxrs/meeting/assemble/control/meeting/{id}/modify/completedtime", post(stub_meeting_assemble_control_meeting_id_modify_completedtime))
        .route("/jaxrs/meeting/assemble/control/meeting/{id}/modify/starttime", post(stub_meeting_assemble_control_meeting_id_modify_starttime))
        .route("/jaxrs/meeting/assemble/control/meeting/{id}/reject", post(stub_meeting_assemble_control_meeting_id_reject))
        .route("/jaxrs/meeting/assemble/control/openmeeting/list/room", get(stub_meeting_assemble_control_openmeeting_list_room))
        .route("/jaxrs/meeting/assemble/control/room/list", get(stub_meeting_assemble_control_room_list))
        .route("/jaxrs/meeting/assemble/control/room/{id}", get(stub_meeting_assemble_control_room_id))
        .layer(axum::Extension(pool))
}
