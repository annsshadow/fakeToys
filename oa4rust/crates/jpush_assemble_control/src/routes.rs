use axum::{
    extract::Extension,
    routing::get, routing::post,
    Router,
};
use deadpool_postgres::Pool;

use crate::{
    list_jpushs, get_jpush, create_jpush, save_jpush, delete_jpush,
    get_control_config, list_control_apps, update_control_config,
    stub_jpush_assemble_control_device_admin_unbind_all_person,
    stub_jpush_assemble_control_device_bind,
    stub_jpush_assemble_control_device_check_deviceName_deviceType_pushType,
    stub_jpush_assemble_control_device_config_push_type,
    stub_jpush_assemble_control_device_list_pushType,
    stub_jpush_assemble_control_device_unbind_new_deviceName_deviceType_pushType,
    stub_jpush_assemble_control_device_unbind_deviceName_deviceType,
    stub_jpush_assemble_control_message_test_send,
};

pub fn router(pool: Pool) -> Router {
    Router::new()
        .route("/jaxrs/jpush/assemble/control/config/get", get(get_control_config))
        .route("/jaxrs/jpush/assemble/control/apps", get(list_control_apps))
        .route("/jaxrs/jpush/assemble/control/config/update", get(update_control_config))
        .route("/jaxrs/jpush/assemble/control/message/list", get(list_jpushs))
        .route("/jaxrs/jpush/assemble/control/message/get/{id}", get(get_jpush))
        .route("/jaxrs/jpush/assemble/control/message/send", post(create_jpush))
        .route("/jaxrs/jpush/assemble/control/message/save/{id}", post(save_jpush))
        .route("/jaxrs/jpush/assemble/control/message/delete/{id}", post(delete_jpush))
        .route("/jaxrs/jpush/assemble/control/device/admin/unbind/all/person", post(stub_jpush_assemble_control_device_admin_unbind_all_person))
        .route("/jaxrs/jpush/assemble/control/device/bind", post(stub_jpush_assemble_control_device_bind))
        .route("/jaxrs/jpush/assemble/control/device/check/{deviceName}/{deviceType}/{pushType}", get(stub_jpush_assemble_control_device_check_deviceName_deviceType_pushType))
        .route("/jaxrs/jpush/assemble/control/device/config/push/type", get(stub_jpush_assemble_control_device_config_push_type))
        .route("/jaxrs/jpush/assemble/control/device/list/{pushType}", get(stub_jpush_assemble_control_device_list_pushType))
        .route("/jaxrs/jpush/assemble/control/device/unbind/new/{deviceName}/{deviceType}/{pushType}", post(stub_jpush_assemble_control_device_unbind_new_deviceName_deviceType_pushType))
        .route("/jaxrs/jpush/assemble/control/device/unbind/{deviceName}/{deviceType}", post(stub_jpush_assemble_control_device_unbind_deviceName_deviceType))
        .route("/jaxrs/jpush/assemble/control/message/test/send", post(stub_jpush_assemble_control_message_test_send))
        .layer(Extension(pool))
}
