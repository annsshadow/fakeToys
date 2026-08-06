use axum::{
    extract::Extension,
    routing::get, routing::post,
    Router,
};
use deadpool_postgres::Pool;

use crate::{
    list_hotpics, get_hotpic, create_hotpic, save_hotpic, delete_hotpic,
    get_control_config, list_control_panels, update_control_config, list_control_applications,
    stub_hotpic_assemble_control_cipher_hotpic_bbs_id,
    stub_hotpic_assemble_control_cipher_hotpic_cms_id,
    stub_hotpic_assemble_control_cipher_hotpic_filter_list_page_page_count_count,
    stub_hotpic_assemble_control_cipher_hotpic_id,
    stub_hotpic_assemble_control_user_hotpic_changeTitle,
    stub_hotpic_assemble_control_user_hotpic_exists_check,
    stub_hotpic_assemble_control_user_hotpic_filter_list_page_page_count_count,
    stub_hotpic_assemble_control_user_hotpic_application_infoId,
    stub_hotpic_assemble_control_user_hotpic_id,
};

pub fn router(pool: Pool) -> Router {
    Router::new()
        .route("/jaxrs/hotpic/assemble/control/config/get", get(get_control_config))
        .route("/jaxrs/hotpic/assemble/control/panels", get(list_control_panels))
        .route("/jaxrs/hotpic/assemble/control/config/update", get(update_control_config))
        .route("/jaxrs/hotpic/assemble/control/applications", get(list_control_applications))
        .route("/jaxrs/hotpic/assemble/control/hotpic/list", get(list_hotpics))
        .route("/jaxrs/hotpic/assemble/control/hotpic/get/{id}", get(get_hotpic))
        .route("/jaxrs/hotpic/assemble/control/hotpic/create", post(create_hotpic))
        .route("/jaxrs/hotpic/assemble/control/hotpic/save/{id}", post(save_hotpic))
        .route("/jaxrs/hotpic/assemble/control/hotpic/delete/{id}", post(delete_hotpic))
        .route("/jaxrs/hotpic/assemble/control/cipher/hotpic/bbs/{id}", get(stub_hotpic_assemble_control_cipher_hotpic_bbs_id))
        .route("/jaxrs/hotpic/assemble/control/cipher/hotpic/cms/{id}", get(stub_hotpic_assemble_control_cipher_hotpic_cms_id))
        .route("/jaxrs/hotpic/assemble/control/cipher/hotpic/filter/list/page/{page}/count/{count}", get(stub_hotpic_assemble_control_cipher_hotpic_filter_list_page_page_count_count))
        .route("/jaxrs/hotpic/assemble/control/cipher/hotpic/{id}", get(stub_hotpic_assemble_control_cipher_hotpic_id))
        .route("/jaxrs/hotpic/assemble/control/user/hotpic/changeTitle", post(stub_hotpic_assemble_control_user_hotpic_changeTitle))
        .route("/jaxrs/hotpic/assemble/control/user/hotpic/exists/check", post(stub_hotpic_assemble_control_user_hotpic_exists_check))
        .route("/jaxrs/hotpic/assemble/control/user/hotpic/filter/list/page/{page}/count/{count}", get(stub_hotpic_assemble_control_user_hotpic_filter_list_page_page_count_count))
        .route("/jaxrs/hotpic/assemble/control/user/hotpic/{application}/{infoId}", get(stub_hotpic_assemble_control_user_hotpic_application_infoId))
        .route("/jaxrs/hotpic/assemble/control/user/hotpic/{id}", get(stub_hotpic_assemble_control_user_hotpic_id))
        .layer(Extension(pool))
}
