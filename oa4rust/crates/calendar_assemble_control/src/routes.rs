use axum::{
    extract::Extension,
    routing::get,
    Router,
};
use deadpool_postgres::Pool;

use crate::{calendar_assemble_control_router, get_control_config, list_control_calendars, update_control_config};

pub fn router(pool: deadpool_postgres::Pool) -> axum::Router {
    axum::Router::new()
        .route("/jaxrs/calendar_assemble_control/get/control/config", get(get_control_config))
        .route("/jaxrs/calendar_assemble_control/list/control/calendars", get(list_control_calendars))
        .route("/jaxrs/calendar_assemble_control/update/control/config", get(update_control_config))
        // ════════ plan002 U2：Java 全集对齐（x_calendar_assemble_control jaxrs，补齐 7 条）════════
        // 注意：必须注册在 Extension(pool) 层之前，否则处理器取不到连接池。
        .route("/jaxrs/calendar_assemble_control/calendar/list/my", get(crate::u2::calendar_list_my))
        .route("/jaxrs/calendar_assemble_control/calendar/list/public", get(crate::u2::calendar_list_public))
        .route("/jaxrs/calendar_assemble_control/calendar/{id}", get(crate::u2::calendar_get))
        .route("/jaxrs/calendar_assemble_control/calendar/ismanager", get(crate::u2::calendar_ismanager))
        .route("/jaxrs/calendar_assemble_control/event/{id}", get(crate::u2::event_get))
        .route("/jaxrs/calendar_assemble_control/setting/list/all", get(crate::u2::setting_list_all))
        .route("/jaxrs/calendar_assemble_control/setting/ismanager", get(crate::u2::setting_ismanager))
        .layer(Extension(pool))
}

