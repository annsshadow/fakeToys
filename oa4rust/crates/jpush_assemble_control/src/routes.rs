use axum::{
    extract::Extension,
    routing::{delete, get, post},
    Router,
};
use deadpool_postgres::Pool;

use crate::{
    list_jpushs, get_jpush, create_jpush, save_jpush, delete_jpush,
    get_control_config, list_control_apps, update_control_config,
    device_admin_unbind_all_person,
    device_bind,
    device_check_deviceName_deviceType_pushType,
    device_config_push_type,
    device_list_pushType,
    device_unbind_new_deviceName_deviceType_pushType,
    device_unbind_deviceName_deviceType,
    message_test_send,
    message_send,
};

pub fn router(pool: deadpool_postgres::Pool) -> axum::Router {
    Router::new()
        .route("/jaxrs/jpush_assemble_control/list/jpushs", get(list_jpushs))
        .route("/jaxrs/jpush_assemble_control/get/jpush", get(get_jpush))
        .route("/jaxrs/jpush_assemble_control/create/jpush", get(create_jpush))
        .route("/jaxrs/jpush_assemble_control/save/jpush", get(save_jpush))
        .route("/jaxrs/jpush_assemble_control/delete/jpush", get(delete_jpush))
        .route("/jaxrs/jpush_assemble_control/get/control/config", get(get_control_config))
        .route("/jaxrs/jpush_assemble_control/list/control/apps", get(list_control_apps))
        .route("/jaxrs/jpush_assemble_control/update/control/config", get(update_control_config))
        .route("/jaxrs/jpush_assemble_control/device/admin/unbind/all/person", get(device_admin_unbind_all_person))
        .route("/jaxrs/jpush_assemble_control/device/bind", get(device_bind))
        .route("/jaxrs/jpush_assemble_control/device/check/deviceName/deviceType/pushType", get(device_check_deviceName_deviceType_pushType))
        .route("/jaxrs/jpush_assemble_control/device/config/push/type", get(device_config_push_type))
        .route("/jaxrs/jpush_assemble_control/device/list/pushType", get(device_list_pushType))
        .route("/jaxrs/jpush_assemble_control/device/unbind/new/deviceName/deviceType/pushType", get(device_unbind_new_deviceName_deviceType_pushType))
        .route("/jaxrs/jpush_assemble_control/device/unbind/deviceName/deviceType", get(device_unbind_deviceName_deviceType))
        .route("/jaxrs/jpush_assemble_control/message/test/send", get(message_test_send))
        .route("/jaxrs/jpush/list", get(list_jpushs))
        .route("/jaxrs/jpush/get/{id}", get(get_jpush))
        .route("/jaxrs/jpush/create", post(create_jpush))
        .route("/jaxrs/jpush/save/{id}", post(save_jpush))
        .route("/jaxrs/jpush/delete/{id}", post(delete_jpush))
        .route("/jaxrs/jpush/assemble/control/config", get(get_control_config))
        .route("/jaxrs/jpush/assemble/control/list/control/apps", get(list_control_apps))
        .route("/jaxrs/jpush/assemble/control/update/control/config", post(update_control_config))
        .route("/jaxrs/jpush/assemble/control/device/admin/unbind/all/person", get(device_admin_unbind_all_person))
        .route("/jaxrs/jpush/assemble/control/device/bind", post(device_bind))
        .route("/jaxrs/jpush/assemble/control/device/check/deviceName/deviceType/pushType", get(device_check_deviceName_deviceType_pushType))
        .route("/jaxrs/jpush/assemble/control/device/config/push/type", get(device_config_push_type))
        .route("/jaxrs/jpush/assemble/control/device/list/pushType", get(device_list_pushType))
        .route("/jaxrs/jpush/assemble/control/device/unbind/new/deviceName/deviceType/pushType", post(device_unbind_new_deviceName_deviceType_pushType))
        .route("/jaxrs/jpush/assemble/control/device/unbind/deviceName/deviceType", post(device_unbind_deviceName_deviceType))
        .route("/jaxrs/jpush/assemble/control/message/test/send", get(message_test_send))
        // ---- plan002 U2 gaps: verb variants + missing ----
        .route("/jaxrs/jpush_assemble_control/device/admin/unbind/all/person", post(device_admin_unbind_all_person))
        .route("/jaxrs/jpush_assemble_control/device/check/{deviceName}/{deviceType}/{pushType}", get(device_check_deviceName_deviceType_pushType))
        .route("/jaxrs/jpush_assemble_control/device/list/{pushType}", get(device_list_pushType))
        .route("/jaxrs/jpush_assemble_control/device/unbind/new/{deviceName}/{deviceType}/{pushType}", get(device_unbind_new_deviceName_deviceType_pushType))
        .route("/jaxrs/jpush_assemble_control/device/unbind/{deviceName}/{deviceType}", delete(device_unbind_deviceName_deviceType))
        .route("/jaxrs/jpush_assemble_control/message/send", post(message_send))
        .route("/jaxrs/jpush_assemble_control/message/test/send", post(message_test_send))
        .layer(Extension(pool))
}
