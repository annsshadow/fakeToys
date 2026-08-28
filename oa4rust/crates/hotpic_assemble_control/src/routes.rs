use axum::{
    extract::Extension,
    routing::{delete, get, post, put},
    Router,
};
use deadpool_postgres::Pool;

use crate::{
    list_hotpics, get_hotpic, create_hotpic, save_hotpic, delete_hotpic,
    get_control_config, list_control_panels, update_control_config, list_control_applications,
    cipher_hotpic_bbs_id,
    cipher_hotpic_cms_id,
    cipher_hotpic_filter_list_page_page_count_count,
    cipher_hotpic_id,
    user_hotpic_changeTitle,
    user_hotpic_exists_check,
    user_hotpic_filter_list_page_page_count_count,
    user_hotpic_application_infoId,
    user_hotpic_id,
    user_hotpic_delete_by_ids,
};

pub fn router(pool: deadpool_postgres::Pool) -> axum::Router {
    Router::new()
        .route("/jaxrs/hotpic_assemble_control/list/hotpics", get(list_hotpics))
        .route("/jaxrs/hotpic_assemble_control/get/hotpic/{id}", get(get_hotpic))
        .route("/jaxrs/hotpic_assemble_control/create/hotpic", get(create_hotpic))
        .route("/jaxrs/hotpic_assemble_control/save/hotpic", get(save_hotpic))
        .route("/jaxrs/hotpic_assemble_control/delete/hotpic", get(delete_hotpic))
        .route("/jaxrs/hotpic_assemble_control/get/control/config", get(get_control_config))
        .route("/jaxrs/hotpic_assemble_control/list/control/panels", get(list_control_panels))
        .route("/jaxrs/hotpic_assemble_control/update/control/config", get(update_control_config))
        .route("/jaxrs/hotpic_assemble_control/list/control/applications", get(list_control_applications))
        .route("/jaxrs/hotpic_assemble_control/cipher/hotpic/bbs/id", get(cipher_hotpic_bbs_id))
        .route("/jaxrs/hotpic_assemble_control/cipher/hotpic/cms/id", get(cipher_hotpic_cms_id))
        .route("/jaxrs/hotpic_assemble_control/cipher/hotpic/filter/list/page/page/count/count", get(cipher_hotpic_filter_list_page_page_count_count))
        .route("/jaxrs/hotpic_assemble_control/cipher/hotpic/id", get(cipher_hotpic_id))
        .route("/jaxrs/hotpic_assemble_control/user/hotpic/changeTitle", get(user_hotpic_changeTitle))
        .route("/jaxrs/hotpic_assemble_control/user/hotpic/exists/check", get(user_hotpic_exists_check))
        .route("/jaxrs/hotpic_assemble_control/user/hotpic/filter/list/page/page/count/count", get(user_hotpic_filter_list_page_page_count_count))
        .route("/jaxrs/hotpic_assemble_control/user/hotpic/application/infoId", get(user_hotpic_application_infoId))
        .route("/jaxrs/hotpic_assemble_control/user/hotpic/id", get(user_hotpic_id))
        .route("/jaxrs/hotpic/list/hotpics", get(list_hotpics))
        .route("/jaxrs/hotpic/get/hotpic/{id}", get(get_hotpic))
        .route("/jaxrs/hotpic/create/hotpic", post(create_hotpic))
        .route("/jaxrs/hotpic/save/hotpic", post(save_hotpic))
        .route("/jaxrs/hotpic/delete/hotpic", post(delete_hotpic))
        .route("/jaxrs/hotpic/assemble/control/config", get(get_control_config))
        .route("/jaxrs/hotpic/assemble/control/list/control/panels", get(list_control_panels))
        .route("/jaxrs/hotpic/assemble/control/list/control/applications", get(list_control_applications))
        .route("/jaxrs/hotpic/assemble/control/update/control/config", post(update_control_config))
        .route("/jaxrs/hotpic/assemble/control/cipher/hotpic/bbs/{id}", get(cipher_hotpic_bbs_id))
        .route("/jaxrs/hotpic/assemble/control/cipher/hotpic/cms/{id}", get(cipher_hotpic_cms_id))
        .route("/jaxrs/hotpic/assemble/control/cipher/hotpic/{id}", get(cipher_hotpic_id))
        .route("/jaxrs/hotpic/assemble/control/cipher/hotpic/filter/list/page/{page}/count/{count}", get(cipher_hotpic_filter_list_page_page_count_count))
        .route("/jaxrs/hotpic/assemble/control/user/hotpic/changeTitle", post(user_hotpic_changeTitle))
        .route("/jaxrs/hotpic/assemble/control/user/hotpic/exists/check", get(user_hotpic_exists_check))
        .route("/jaxrs/hotpic/assemble/control/user/hotpic/filter/list/page/{page}/count/{count}", get(user_hotpic_filter_list_page_page_count_count))
        .route("/jaxrs/hotpic/assemble/control/user/hotpic/application/{infoId}", get(user_hotpic_application_infoId))
        .route("/jaxrs/hotpic/assemble/control/user/hotpic/{id}", get(user_hotpic_id))
        // ---- plan002 U2 gaps: verb variants + missing ----
        .route("/jaxrs/hotpic/assemble/control/cipher/hotpic/bbs/{id}", delete(cipher_hotpic_bbs_id))
        .route("/jaxrs/hotpic/assemble/control/cipher/hotpic/cms/{id}", delete(cipher_hotpic_cms_id))
        .route("/jaxrs/hotpic/assemble/control/cipher/hotpic/filter/list/page/{page}/count/{count}", put(cipher_hotpic_filter_list_page_page_count_count))
        .route("/jaxrs/hotpic/assemble/control/user/hotpic", post(create_hotpic))
        .route("/jaxrs/hotpic/assemble/control/user/hotpic/filter/list/page/{page}/count/{count}", put(user_hotpic_filter_list_page_page_count_count))
        .route("/jaxrs/hotpic/assemble/control/user/hotpic/{id}", delete(user_hotpic_id))
        .route("/jaxrs/hotpic/assemble/control/user/hotpic/{id}/{id2}", delete(user_hotpic_delete_by_ids))
        .layer(Extension(pool))
}
