use axum::{
    extract::Extension,
    routing::{delete, get, post, put},
    Router,
};
use deadpool_postgres::Pool;

use crate::{
    consume_id_type_type, consume_list_consume_count_count,
    consume_list_consume_currentperson_count_count,
    consume_list_consume_person_person_count_count, consume_type_type,
    consume_type_type_mockputtopost, connector_create, im_conversation,
    im_conversation_business_businessId, im_conversation_id,
    im_conversation_id_group, im_conversation_id_group_mockdeletetoget,
    im_conversation_id_group_quit_self, im_conversation_id_icon,
    im_conversation_id_read, im_conversation_id_read_mockputtopost,
    im_conversation_id_single, im_conversation_id_single_delete_virtual,
    im_conversation_id_top_cancel, im_conversation_id_top_cancel_mockputtopost,
    im_conversation_id_top_set, im_conversation_id_top_set_mockputtopost,
    im_conversation_list_my,
    im_conversation_list_with_person, im_conversation_mockputtopost,
    im_conversation_update, im_manager_config, im_manager_config_post, im_msg,
    im_msg_clear, im_msg_collection, im_msg_collection_list_page_size_size,
    im_msg_collection_remove, im_msg_download_id,
    im_msg_download_id_image_width_width_height_height, im_msg_list_object,
    im_msg_list_page_size_size, im_msg_revoke_id,
    im_msg_upload_conversationId_type_type,
    instant_currentperson_consumed, instant_currentperson_consumed_all,
    instant_currentperson_consumed_mockputtopost,
    instant_currentperson_consumed_put,
    instant_list_currentperson_consumed_count_count_asc,
    instant_list_currentperson_consumed_count_count_desc,
    instant_list_currentperson_count_count_asc,
    instant_list_currentperson_count_count_desc,
    instant_list_currentperson_noim_count_count_desc,
    instant_list_currentperson_not_consumed_count_count_asc,
    instant_list_currentperson_not_consumed_count_count_desc,
    instant_list_id_next_count, instant_list_id_prev_count, mark_read,
    mass_create, mass_enable_type, mass_enable_type_get, mass_id,
    mass_id_mockdeletetoget, mass_list_id_next_count, mass_list_id_prev_count,
    message_custom_create, message_list_paging_page_size_size, receive_list,
    send_message, ws_count_person, ws_create, ws_list_person,
    ws_list_person_current_node,
};

pub fn router(pool: Pool) -> Router {
    Router::new()
        .route("/jaxrs/message/assemble/communicate/send", post(send_message))
        .route("/jaxrs/message/assemble/communicate/receive/{consume}", get(receive_list))
        .route("/jaxrs/message/assemble/communicate/mark_read/{id}", post(mark_read))
        // connector（Java ConnectorAction POST /connector）
        .route("/jaxrs/message/assemble/communicate/connector", post(connector_create))
        // ws（Java WsAction）
        .route("/jaxrs/message/assemble/communicate/ws", post(ws_create))
        .route("/jaxrs/message/assemble/communicate/ws/count/person", get(ws_count_person))
        .route("/jaxrs/message/assemble/communicate/ws/list/person/current/node", get(ws_list_person_current_node))
        .route("/jaxrs/message/assemble/communicate/ws/list/person", get(ws_list_person))
        // consume（GET /{id}/type 为 Java 主动词；PUT type 承载 ActionUpdate，与 mockputtopost 同实现）
        .route("/jaxrs/message/assemble/communicate/consume/list/{consume}/count/{count}", get(consume_list_consume_count_count))
        .route("/jaxrs/message/assemble/communicate/consume/list/{consume}/currentperson/count/{count}", get(consume_list_consume_currentperson_count_count))
        .route("/jaxrs/message/assemble/communicate/consume/list/{consume}/person/{person}/count/{count}", get(consume_list_consume_person_person_count_count))
        .route("/jaxrs/message/assemble/communicate/consume/type/{type}", get(consume_type_type))
        .route("/jaxrs/message/assemble/communicate/consume/type/{type}", put(consume_type_type_mockputtopost))
        .route("/jaxrs/message/assemble/communicate/consume/type/{type}/mockputtopost", post(consume_type_type_mockputtopost))
        .route("/jaxrs/message/assemble/communicate/consume/{id}/type/{type}", post(consume_id_type_type))
        .route("/jaxrs/message/assemble/communicate/consume/{id}/type/{type}", get(consume_id_type_type))
        // im conversation
        .route("/jaxrs/message/assemble/communicate/im/conversation", post(im_conversation))
.route("/jaxrs/message/assemble/communicate/im/conversation", put(im_conversation))
        .route("/jaxrs/message/assemble/communicate/im/conversation/business/{businessId}", get(im_conversation_business_businessId))
        .route("/jaxrs/message/assemble/communicate/im/conversation/list/my", get(im_conversation_list_my))
        .route("/jaxrs/message/assemble/communicate/im/conversation/list/with/person", get(im_conversation_list_with_person))
        .route("/jaxrs/message/assemble/communicate/im/conversation/list/with/person", post(im_conversation_list_with_person))
        .route("/jaxrs/message/assemble/communicate/im/conversation/mockputtopost", post(im_conversation_mockputtopost))
        .route("/jaxrs/message/assemble/communicate/im/conversation/{id}", get(im_conversation_id))
        .route("/jaxrs/message/assemble/communicate/im/conversation/{id}/group", get(im_conversation_id_group))
.route("/jaxrs/message/assemble/communicate/im/conversation/{id}/group", delete(im_conversation_id_group))
        .route("/jaxrs/message/assemble/communicate/im/conversation/{id}/group/mockdeletetoget", delete(im_conversation_id_group_mockdeletetoget))
        .route("/jaxrs/message/assemble/communicate/im/conversation/{id}/group/mockdeletetoget", get(im_conversation_id_group_mockdeletetoget))
        .route("/jaxrs/message/assemble/communicate/im/conversation/{id}/group/quit/self", post(im_conversation_id_group_quit_self))
        .route("/jaxrs/message/assemble/communicate/im/conversation/{id}/group/quit/self", get(im_conversation_id_group_quit_self))
        .route("/jaxrs/message/assemble/communicate/im/conversation/{id}/icon", get(im_conversation_id_icon))
        .route("/jaxrs/message/assemble/communicate/im/conversation/{id}/read", post(im_conversation_id_read))
        .route("/jaxrs/message/assemble/communicate/im/conversation/{id}/read", put(im_conversation_id_read))
        .route("/jaxrs/message/assemble/communicate/im/conversation/{id}/read/mockputtopost", post(im_conversation_id_read_mockputtopost))
        .route("/jaxrs/message/assemble/communicate/im/conversation/{id}/single", get(im_conversation_id_single))
        .route("/jaxrs/message/assemble/communicate/im/conversation/{id}/single", delete(im_conversation_id_single_delete_virtual))
        .route("/jaxrs/message/assemble/communicate/im/conversation/{id}/single/mockdeletetoget", delete(im_conversation_id_single_delete_virtual))
        .route("/jaxrs/message/assemble/communicate/im/conversation/{id}/single/mockdeletetoget", get(im_conversation_id_single_delete_virtual))
        .route("/jaxrs/message/assemble/communicate/im/conversation/{id}/top/cancel", post(im_conversation_id_top_cancel))
        .route("/jaxrs/message/assemble/communicate/im/conversation/{id}/top/cancel", put(im_conversation_id_top_cancel))
        .route("/jaxrs/message/assemble/communicate/im/conversation/{id}/top/cancel/mockputtopost", post(im_conversation_id_top_cancel_mockputtopost))
        .route("/jaxrs/message/assemble/communicate/im/conversation/{id}/top/cancel/mockputtopost", put(im_conversation_id_top_cancel_mockputtopost))
        .route("/jaxrs/message/assemble/communicate/im/conversation/{id}/top/set", post(im_conversation_id_top_set))
        .route("/jaxrs/message/assemble/communicate/im/conversation/{id}/top/set", put(im_conversation_id_top_set))
        .route("/jaxrs/message/assemble/communicate/im/conversation/{id}/top/set/mockputtopost", post(im_conversation_id_top_set_mockputtopost))
        .route("/jaxrs/message/assemble/communicate/im/conversation/{id}", put(im_conversation_update))
        .route("/jaxrs/message/assemble/communicate/im/manager/config", get(im_manager_config))
        .route("/jaxrs/message/assemble/communicate/im/manager/config", post(im_manager_config_post))
        // im msg
        .route("/jaxrs/message/assemble/communicate/im/msg", post(im_msg))
        .route("/jaxrs/message/assemble/communicate/im/msg/clear", post(im_msg_clear))
        .route("/jaxrs/message/assemble/communicate/im/msg/collection", post(im_msg_collection))
        .route("/jaxrs/message/assemble/communicate/im/msg/collection/list/{page}/size/{size}", get(im_msg_collection_list_page_size_size))
        .route("/jaxrs/message/assemble/communicate/im/msg/collection/list/{page}/size/{size}", post(im_msg_collection_list_page_size_size))
        .route("/jaxrs/message/assemble/communicate/im/msg/collection/remove", post(im_msg_collection_remove))
        .route("/jaxrs/message/assemble/communicate/im/msg/collection/remove", delete(im_msg_collection_remove))
        .route("/jaxrs/message/assemble/communicate/im/msg/download/{id}", get(im_msg_download_id))
        .route("/jaxrs/message/assemble/communicate/im/msg/download/{id}/image/width/{width}/height/{height}", get(im_msg_download_id_image_width_width_height_height))
        .route("/jaxrs/message/assemble/communicate/im/msg/list/object", get(im_msg_list_object))
        .route("/jaxrs/message/assemble/communicate/im/msg/list/object", post(im_msg_list_object))
        .route("/jaxrs/message/assemble/communicate/im/msg/list/{page}/size/{size}", get(im_msg_list_page_size_size))
        .route("/jaxrs/message/assemble/communicate/im/msg/list/{page}/size/{size}", post(im_msg_list_page_size_size))
        .route("/jaxrs/message/assemble/communicate/im/msg/revoke/{id}", post(im_msg_revoke_id))
        .route("/jaxrs/message/assemble/communicate/im/msg/revoke/{id}", get(im_msg_revoke_id))
        .route("/jaxrs/message/assemble/communicate/im/msg/upload/{conversationId}/type/{type}", post(im_msg_upload_conversationId_type_type))
        // instant（PUT currentperson/consumed 为 Java 主动词）
        .route("/jaxrs/message/assemble/communicate/instant/currentperson/consumed", get(instant_currentperson_consumed))
        .route("/jaxrs/message/assemble/communicate/instant/currentperson/consumed", put(instant_currentperson_consumed_put))
        .route("/jaxrs/message/assemble/communicate/instant/currentperson/consumed/all", get(instant_currentperson_consumed_all))
        .route("/jaxrs/message/assemble/communicate/instant/currentperson/consumed/mockputtopost", post(instant_currentperson_consumed_mockputtopost))
        .route("/jaxrs/message/assemble/communicate/instant/list/currentperson/consumed/count/{count}/asc", get(instant_list_currentperson_consumed_count_count_asc))
        .route("/jaxrs/message/assemble/communicate/instant/list/currentperson/consumed/count/{count}/desc", get(instant_list_currentperson_consumed_count_count_desc))
        .route("/jaxrs/message/assemble/communicate/instant/list/currentperson/count/{count}/asc", get(instant_list_currentperson_count_count_asc))
        .route("/jaxrs/message/assemble/communicate/instant/list/currentperson/count/{count}/desc", get(instant_list_currentperson_count_count_desc))
        .route("/jaxrs/message/assemble/communicate/instant/list/currentperson/noim/count/{count}/desc", get(instant_list_currentperson_noim_count_count_desc))
        .route("/jaxrs/message/assemble/communicate/instant/list/currentperson/not/consumed/count/{count}/asc", get(instant_list_currentperson_not_consumed_count_count_asc))
        .route("/jaxrs/message/assemble/communicate/instant/list/currentperson/not/consumed/count/{count}/desc", get(instant_list_currentperson_not_consumed_count_count_desc))
        .route("/jaxrs/message/assemble/communicate/instant/list/{id}/next/{count}", get(instant_list_id_next_count))
        .route("/jaxrs/message/assemble/communicate/instant/list/{id}/prev/{count}", get(instant_list_id_prev_count))
        // mass（POST /mass、DELETE /mass/{id}、GET enable/type、GET mockdeletetoget 均为 Java 动词）
        .route("/jaxrs/message/assemble/communicate/mass", post(mass_create))
        .route("/jaxrs/message/assemble/communicate/mass/enable/type", post(mass_enable_type))
        .route("/jaxrs/message/assemble/communicate/mass/enable/type", get(mass_enable_type_get))
        .route("/jaxrs/message/assemble/communicate/mass/list/{id}/next/{count}", get(mass_list_id_next_count))
        .route("/jaxrs/message/assemble/communicate/mass/list/{id}/prev/{count}", get(mass_list_id_prev_count))
        .route("/jaxrs/message/assemble/communicate/mass/{id}", get(mass_id))
        .route("/jaxrs/message/assemble/communicate/mass/{id}", delete(mass_id_mockdeletetoget))
        .route("/jaxrs/message/assemble/communicate/mass/{id}/mockdeletetoget", delete(mass_id_mockdeletetoget))
        .route("/jaxrs/message/assemble/communicate/mass/{id}/mockdeletetoget", get(mass_id_mockdeletetoget))
        // message
        .route("/jaxrs/message/assemble/communicate/message/custom/create", post(message_custom_create))
        .route("/jaxrs/message/assemble/communicate/message/list/paging/{page}/size/{size}", get(message_list_paging_page_size_size))
        .route("/jaxrs/message/assemble/communicate/message/list/paging/{page}/size/{size}", post(message_list_paging_page_size_size))
        .layer(Extension(pool))
}
