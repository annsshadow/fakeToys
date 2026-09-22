// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

use axum::{
    extract::Extension,
    routing::{delete, get, post},
    Router,
};

use crate::{
    create_jpush, delete_jpush, device_admin_unbind_all_person, device_bind,
    device_check_deviceName_deviceType_pushType, device_config_push_type, device_list_pushType,
    device_unbind_deviceName_deviceType, device_unbind_new_deviceName_deviceType_pushType,
    get_control_config, get_jpush, list_control_apps, list_jpushs, message_send, message_test_send,
    save_jpush, update_control_config,
};

pub fn router(pool: deadpool_postgres::Pool) -> axum::Router {
    Router::new()
        .route("/api/jpush_assemble_control/list/jpushs", get(list_jpushs))
        .route("/api/jpush_assemble_control/get/jpush/{id}", get(get_jpush))
        .route(
            "/api/jpush_assemble_control/create/jpush",
            get(create_jpush),
        )
        .route("/api/jpush_assemble_control/save/jpush", get(save_jpush))
        .route(
            "/api/jpush_assemble_control/delete/jpush",
            get(delete_jpush),
        )
        .route(
            "/api/jpush_assemble_control/get/control/config",
            get(get_control_config),
        )
        .route(
            "/api/jpush_assemble_control/list/control/apps",
            get(list_control_apps),
        )
        .route(
            "/api/jpush_assemble_control/update/control/config",
            get(update_control_config),
        )
        .route(
            "/api/jpush_assemble_control/device/admin/unbind/all/person",
            post(device_admin_unbind_all_person),
        )
        .route("/api/jpush_assemble_control/device/bind", post(device_bind))
        .route(
            "/api/jpush_assemble_control/device/check/deviceName/deviceType/pushType",
            get(device_check_deviceName_deviceType_pushType),
        )
        .route(
            "/api/jpush_assemble_control/device/config/push/type",
            get(device_config_push_type),
        )
        .route(
            "/api/jpush_assemble_control/device/list/{pushType}",
            get(device_list_pushType),
        )
        .route(
            "/api/jpush_assemble_control/device/unbind/new/deviceName/deviceType/pushType",
            get(device_unbind_new_deviceName_deviceType_pushType),
        )
        .route(
            "/api/jpush_assemble_control/device/unbind/deviceName/deviceType",
            get(device_unbind_deviceName_deviceType),
        )
        .route(
            "/api/jpush_assemble_control/message/test/send",
            get(message_test_send),
        )
        .route("/api/jpush/list", get(list_jpushs))
        .route("/api/jpush/get/{id}", get(get_jpush))
        .route("/api/jpush/create", post(create_jpush))
        .route("/api/jpush/save/{id}", post(save_jpush))
        .route("/api/jpush/delete/{id}", post(delete_jpush))
        .route(
            "/api/jpush/assemble/control/config",
            get(get_control_config),
        )
        .route(
            "/api/jpush/assemble/control/list/control/apps",
            get(list_control_apps),
        )
        .route(
            "/api/jpush/assemble/control/update/control/config",
            post(update_control_config),
        )
        .route(
            "/api/jpush/assemble/control/device/admin/unbind/all/person",
            get(device_admin_unbind_all_person),
        )
        .route("/api/jpush/assemble/control/device/bind", post(device_bind))
        .route(
            "/api/jpush/assemble/control/device/check/deviceName/deviceType/pushType",
            get(device_check_deviceName_deviceType_pushType),
        )
        .route(
            "/api/jpush/assemble/control/device/config/push/type",
            get(device_config_push_type),
        )
        .route(
            "/api/jpush/assemble/control/device/list/{pushType}",
            get(device_list_pushType),
        )
        .route(
            "/api/jpush/assemble/control/device/unbind/new/deviceName/deviceType/pushType",
            post(device_unbind_new_deviceName_deviceType_pushType),
        )
        .route(
            "/api/jpush/assemble/control/device/unbind/deviceName/deviceType",
            post(device_unbind_deviceName_deviceType),
        )
        .route(
            "/api/jpush/assemble/control/message/test/send",
            get(message_test_send),
        )
        // ---- plan002 U2 gaps: verb variants + missing ----
        .route(
            "/api/jpush_assemble_control/device/check/{deviceName}/{deviceType}/{pushType}",
            get(device_check_deviceName_deviceType_pushType),
        )
        .route(
            "/api/jpush_assemble_control/device/unbind/new/{deviceName}/{deviceType}/{pushType}",
            get(device_unbind_new_deviceName_deviceType_pushType),
        )
        .route(
            "/api/jpush_assemble_control/device/unbind/{deviceName}/{deviceType}",
            delete(device_unbind_deviceName_deviceType),
        )
        .route(
            "/api/jpush_assemble_control/message/send",
            post(message_send),
        )
        .route(
            "/api/jpush_assemble_control/message/test/send",
            post(message_test_send),
        )
        .layer(Extension(pool))
}
