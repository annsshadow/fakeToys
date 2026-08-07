use axum::{
    extract::Extension,
    routing::{get, post},
    Router,
};
use deadpool_postgres::Pool;

use crate::{
    create_reply, create_topic, get_control_config, get_forum, list_control_sections,
    list_forums, list_topics_by_forum, update_control_config,
    permission_section_sectionId,
    permission_subject_subjectId,
    reply_list_sub_id,
    section_viewforum_forumId,
    subject_top_sectionId,
    subject_view_id,
};

pub fn router(pool: deadpool_postgres::Pool) -> axum::Router {
    Router::new()
        .route("/jaxrs/bbs_assemble_control/create/reply", get(create_reply))
        .route("/jaxrs/bbs_assemble_control/create/topic", get(create_topic))
        .route("/jaxrs/bbs_assemble_control/get/control/config", get(get_control_config))
        .route("/jaxrs/bbs_assemble_control/get/forum", get(get_forum))
        .route("/jaxrs/bbs_assemble_control/list/control/sections", get(list_control_sections))
        .route("/jaxrs/bbs_assemble_control/list/forums", get(list_forums))
        .route("/jaxrs/bbs_assemble_control/list/topics/by/forum", get(list_topics_by_forum))
        .route("/jaxrs/bbs_assemble_control/update/control/config", get(update_control_config))
        .route("/jaxrs/bbs_assemble_control/permission/section/sectionId", get(permission_section_sectionId))
        .route("/jaxrs/bbs_assemble_control/permission/subject/subjectId", get(permission_subject_subjectId))
        .route("/jaxrs/bbs_assemble_control/reply/list/sub/id", get(reply_list_sub_id))
        .route("/jaxrs/bbs_assemble_control/section/viewforum/forumId", get(section_viewforum_forumId))
        .route("/jaxrs/bbs_assemble_control/subject/top/sectionId", get(subject_top_sectionId))
        .route("/jaxrs/bbs_assemble_control/subject/view/id", get(subject_view_id))
        .layer(Extension(pool))
}

