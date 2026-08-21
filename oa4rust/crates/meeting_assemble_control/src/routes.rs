use axum::{
    routing::{delete, get, post, put},
    Router,
};
use deadpool_postgres::Pool;

use crate::{
    create_meeting, create_meeting_control, delete_meeting, delete_meeting_control,
    list_meeting_controls, meeting_id, save_meeting,
    building_id,
    building_list,
    building_list_like_key,
    building_list_like_pinyin_key,
    building_list_pinyininitial_key,
    config_system_config,
    config_system_config_manage,
    meeting_id_accept,
    meeting_id_add_invite,
    meeting_id_checkin,
    meeting_id_confirm_allow,
    meeting_id_confirm_deny,
    meeting_id_delete_invite,
    meeting_id_manual_completed,
    meeting_id_modify_completedtime,
    meeting_id_modify_starttime,
    meeting_id_reject,
    meeting_list_applied_completed,
    meeting_list_applied_processing,
    meeting_list_applied_wait,
    meeting_list_apply_page_size_size,
    meeting_list_coming_day_count,
    meeting_list_invited_completed,
    meeting_list_invited_processing,
    meeting_list_invited_rejected,
    meeting_list_invited_wait,
    meeting_list_wait_accept,
    meeting_list_wait_confirm,
    meeting_list_year_year_month_month,
    meeting_list_year_year_month_month_all,
    meeting_list_year_year_month_month_day_day,
    meeting_list_year_year_month_month_day_day_all,
    openmeeting_list_room,
    room_id,
    room_list,
};

pub fn meeting_assemble_control_routes(pool: Pool) -> Router {
    Router::new()
        .route("/jaxrs/meeting/assemble/control/list/{\"meetingId\"}", get(list_meeting_controls))
        .route("/jaxrs/meeting/assemble/control/create", post(create_meeting_control))
        .route("/jaxrs/meeting/assemble/control/delete/{id}", delete(delete_meeting_control))
        .route("/jaxrs/meeting/assemble/control/building/list", get(building_list))
        .route("/jaxrs/meeting/assemble/control/building/{id}", get(building_id))
        .route("/jaxrs/meeting/assemble/control/building/list/like/{key}", get(building_list_like_key))
        .route("/jaxrs/meeting/assemble/control/building/list/like/pinyin/{key}", get(building_list_like_pinyin_key))
        .route("/jaxrs/meeting/assemble/control/building/list/pinyininitial/{key}", get(building_list_pinyininitial_key))
        .route("/jaxrs/meeting/assemble/control/config/system/config", get(config_system_config))
        .route("/jaxrs/meeting/assemble/control/config/system/config/manage", post(config_system_config_manage))
        .route("/jaxrs/meeting/assemble/control/meeting/list/applied/completed", get(meeting_list_applied_completed))
        .route("/jaxrs/meeting/assemble/control/meeting/list/applied/processing", get(meeting_list_applied_processing))
        .route("/jaxrs/meeting/assemble/control/meeting/list/applied/wait", get(meeting_list_applied_wait))
        .route("/jaxrs/meeting/assemble/control/meeting/list/apply/{page}/size/{size}", get(meeting_list_apply_page_size_size))
        .route("/jaxrs/meeting/assemble/control/meeting/list/coming/day/{count}", get(meeting_list_coming_day_count))
        .route("/jaxrs/meeting/assemble/control/meeting/list/invited/completed", get(meeting_list_invited_completed))
        .route("/jaxrs/meeting/assemble/control/meeting/list/invited/processing", get(meeting_list_invited_processing))
        .route("/jaxrs/meeting/assemble/control/meeting/list/invited/rejected", get(meeting_list_invited_rejected))
        .route("/jaxrs/meeting/assemble/control/meeting/list/invited/wait", get(meeting_list_invited_wait))
        .route("/jaxrs/meeting/assemble/control/meeting/list/wait/accept", get(meeting_list_wait_accept))
        .route("/jaxrs/meeting/assemble/control/meeting/list/wait/confirm", get(meeting_list_wait_confirm))
        .route("/jaxrs/meeting/assemble/control/meeting/list/year/{year}/month/{month}", get(meeting_list_year_year_month_month))
        .route("/jaxrs/meeting/assemble/control/meeting/list/year/{year}/month/{month}/all", get(meeting_list_year_year_month_month_all))
        .route("/jaxrs/meeting/assemble/control/meeting/list/year/{year}/month/{month}/day/{day}", get(meeting_list_year_year_month_month_day_day))
        .route("/jaxrs/meeting/assemble/control/meeting/list/year/{year}/month/{month}/day/{day}/all", get(meeting_list_year_year_month_month_day_day_all))
        .route("/jaxrs/meeting/assemble/control/meeting/create", post(create_meeting))
        .route("/jaxrs/meeting/assemble/control/meeting/{id}", get(meeting_id))
        .route("/jaxrs/meeting/assemble/control/meeting/save/{id}", post(save_meeting))
        .route("/jaxrs/meeting/assemble/control/meeting/delete/{id}", post(delete_meeting))
        .route("/jaxrs/meeting/assemble/control/meeting/{id}/accept", post(meeting_id_accept))
        .route("/jaxrs/meeting/assemble/control/meeting/{id}/add/invite", post(meeting_id_add_invite))
        .route("/jaxrs/meeting/assemble/control/meeting/{id}/checkin", post(meeting_id_checkin))
        .route("/jaxrs/meeting/assemble/control/meeting/{id}/confirm/allow", post(meeting_id_confirm_allow))
        .route("/jaxrs/meeting/assemble/control/meeting/{id}/confirm/deny", post(meeting_id_confirm_deny))
        .route("/jaxrs/meeting/assemble/control/meeting/{id}/delete/invite", post(meeting_id_delete_invite))
        .route("/jaxrs/meeting/assemble/control/meeting/{id}/manual/completed", post(meeting_id_manual_completed))
        .route("/jaxrs/meeting/assemble/control/meeting/{id}/modify/completedtime", post(meeting_id_modify_completedtime))
        .route("/jaxrs/meeting/assemble/control/meeting/{id}/modify/starttime", post(meeting_id_modify_starttime))
        .route("/jaxrs/meeting/assemble/control/meeting/{id}/reject", post(meeting_id_reject))
        .route("/jaxrs/meeting/assemble/control/openmeeting/list/room", get(openmeeting_list_room))
        .route("/jaxrs/meeting/assemble/control/room/list", get(room_list))
        .route("/jaxrs/meeting/assemble/control/room/{id}", get(room_id))
        .route("/jaxrs/meeting/assemble/control/building/list/completed/completed/{start}/{start}", post(crate::building_list_start_start_completed_completed))
        .route("/jaxrs/meeting/assemble/control/building/list/completed/completed/allmeeting/{start}/{start}", post(crate::building_list_start_start_completed_completed_allmeeting))
        .route("/jaxrs/meeting/assemble/control/building/list/completed/completed/room/room/meeting/meeting/{start}/{start}", post(crate::building_list_start_start_completed_completed_room_room_meeting_meeting))
        .route("/jaxrs/meeting/assemble/control/list/meeting/controls", get(crate::list_meeting_controls))
        .route("/jaxrs/meeting/assemble/control/meeting/checkin/code/{id}", post(crate::meeting_id_checkin_code))
        .route("/jaxrs/meeting/assemble/control/meeting/list/coming/{month}/{count}", get(crate::meeting_list_coming_month_count))
        .route("/jaxrs/meeting/assemble/control/meeting/list/forward/monthcount/{monthCount}", get(crate::meeting_list_forward_monthcount_monthCount))
        .route("/jaxrs/meeting/assemble/control/meeting/list/forward/monthcount/all/{monthCount}", get(crate::meeting_list_forward_monthcount_monthCount_all))
        .route("/jaxrs/meeting/assemble/control/meeting/list/{id}/{next}/{count}", get(crate::meeting_list_id_next_count))
        .route("/jaxrs/meeting/assemble/control/meeting/list/invite/{page}/{size}/{size}", get(crate::meeting_list_invite_page_size_size))
        .route("/jaxrs/meeting/assemble/control/meeting/list/manage/{page}/{size}/{size}", post(crate::meeting_list_page_size_size_manage))
        .route("/jaxrs/meeting/assemble/control/meeting/list/{year}/{year}/{month}/{month}/{day}/{day}/{roomId}", get(crate::meeting_list_year_year_month_month_day_day_roomId))
        .route("/jaxrs/meeting/assemble/control/room/photo/{id}", get(crate::room_id_photo))
        .route("/jaxrs/meeting/assemble/control/room/list/like/{key}", get(crate::room_list_like_key))
        .route("/jaxrs/meeting/assemble/control/room/list/like/pinyin/{key}", get(crate::room_list_like_pinyin_key))
        .route("/jaxrs/meeting/assemble/control/room/list/pinyininitial/{key}", get(crate::room_list_pinyininitial_key))
        .route("/jaxrs/meeting/assemble/control/meeting/delete/{id}", delete(delete_meeting))
        .route("/jaxrs/meeting/assemble/control/meeting/{id}/delete/invite", delete(meeting_id_delete_invite))
        .route("/jaxrs/meeting/assemble/control/meeting/save/{id}", put(save_meeting))
        .layer(axum::Extension(pool))
}
