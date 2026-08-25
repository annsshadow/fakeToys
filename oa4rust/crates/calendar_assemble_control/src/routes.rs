use axum::{
    extract::Extension,
    routing::{delete, get, post, put},
};

use crate::{get_control_config, list_control_calendars, update_control_config};

pub fn router(pool: deadpool_postgres::Pool) -> axum::Router {
    axum::Router::new()
        // ── 控制面配置（既有 3 条，非 jaxrs 清单路径）──
        .route("/jaxrs/calendar_assemble_control/get/control/config", get(get_control_config))
        .route("/jaxrs/calendar_assemble_control/list/control/calendars", get(list_control_calendars))
        .route("/jaxrs/calendar_assemble_control/update/control/config", get(update_control_config))
        // ════════ plan002 U2：x_calendar_assemble_control jaxrs 全集 31 条对齐 ═════════
        // 注册顺序须在 Extension(pool) 层之前，确保处理器能取到连接池。
        .route("/jaxrs/calendar_assemble_control/calendar", post(crate::u2::calendar_create))
        .route("/jaxrs/calendar_assemble_control/calendar/follow/{id}", get(crate::u2::calendar_follow_get))
        .route("/jaxrs/calendar_assemble_control/calendar/follow/{id}/cancel", get(crate::u2::calendar_follow_cancel))
        .route("/jaxrs/calendar_assemble_control/calendar/ismanager", get(crate::u2::calendar_ismanager))
        .route("/jaxrs/calendar_assemble_control/calendar/ismanager/calendar/{id}", get(crate::u2::calendar_ismanager_calendar))
        .route("/jaxrs/calendar_assemble_control/calendar/list/filter", put(crate::u2::calendar_list_filter))
        .route("/jaxrs/calendar_assemble_control/calendar/list/my", get(crate::u2::calendar_list_my))
        .route("/jaxrs/calendar_assemble_control/calendar/list/public", get(crate::u2::calendar_list_public))
        .route("/jaxrs/calendar_assemble_control/calendar/manager/list/with/person/{id}", get(crate::u2::calendar_manager_list_with_person))
        .route("/jaxrs/calendar_assemble_control/calendar/{id}", delete(crate::u2::calendar_delete))
        .route("/jaxrs/calendar_assemble_control/calendar/{id}", get(crate::u2::calendar_get))
        .route("/jaxrs/calendar_assemble_control/event", post(crate::u2::event_create))
        .route("/jaxrs/calendar_assemble_control/event/after/{id}", delete(crate::u2::event_delete_after))
        .route("/jaxrs/calendar_assemble_control/event/all/{id}", delete(crate::u2::event_delete_all))
        .route("/jaxrs/calendar_assemble_control/event/list/filter", put(crate::u2::event_list_filter))
        .route("/jaxrs/calendar_assemble_control/event/list/filter/sample", put(crate::u2::event_list_filter_sample))
        .route("/jaxrs/calendar_assemble_control/event/list/filter/sample/manager", post(crate::u2::event_list_filter_sample_manager))
        .route("/jaxrs/calendar_assemble_control/event/manage", post(crate::u2::event_manage))
        .route("/jaxrs/calendar_assemble_control/event/rfc/{id}", get(crate::u2::event_rfc))
        .route("/jaxrs/calendar_assemble_control/event/single/{id}", delete(crate::u2::event_delete_single))
        .route("/jaxrs/calendar_assemble_control/event/update/after/{id}", put(crate::u2::event_update_after))
        .route("/jaxrs/calendar_assemble_control/event/update/all/{id}", put(crate::u2::event_update_all))
        .route("/jaxrs/calendar_assemble_control/event/update/single/{id}", put(crate::u2::event_update_single))
        .route("/jaxrs/calendar_assemble_control/event/{id}", get(crate::u2::event_get))
        .route("/jaxrs/calendar_assemble_control/message", post(crate::u2::message_create))
        .route("/jaxrs/calendar_assemble_control/setting", post(crate::u2::setting_create))
        .route("/jaxrs/calendar_assemble_control/setting/code/{code}", get(crate::u2::setting_get_by_code))
        .route("/jaxrs/calendar_assemble_control/setting/ismanager", get(crate::u2::setting_ismanager))
        .route("/jaxrs/calendar_assemble_control/setting/list/all", get(crate::u2::setting_list_all))
        .route("/jaxrs/calendar_assemble_control/setting/{id}", get(crate::u2::setting_get))
        .route("/jaxrs/calendar_assemble_control/test/1", get(crate::u2::test_1))
        .layer(Extension(pool))
}
