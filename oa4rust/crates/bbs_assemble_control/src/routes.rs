use axum::{
    routing::{get, post},
    Router,
};
use deadpool_postgres::Pool;

use crate::{
    create_reply, create_topic, get_control_config, get_forum, list_control_sections,
    list_forums, list_topics_by_forum, update_control_config,
    forum_view_all, permission_section_sectionId,
    permission_subject_subjectId,
    reply_list_sub_id,
    section_viewforum_forumId,
    subject_top_sectionId,
    subject_view_id,
    shutup_create, uuid_generate,
};

pub fn router(pool: Pool) -> Router {
    Router::new()
        .route("/jaxrs/bbs/assemble/control/config", get(get_control_config))
        .route("/jaxrs/bbs/assemble/control/section/list", get(list_control_sections))
        .route("/jaxrs/bbs/assemble/control/forum/list", get(list_forums))
        .route("/jaxrs/bbs/assemble/control/forum/view/all", get(forum_view_all))
        .route("/jaxrs/bbs/assemble/control/forum/{id}", get(get_forum))
        .route("/jaxrs/bbs/assemble/control/topic/create", post(create_topic))
        .route("/jaxrs/bbs/assemble/control/topic/list/forum/{forumId}", get(list_topics_by_forum))
        .route("/jaxrs/bbs/assemble/control/reply/create", post(create_reply))
        .route("/jaxrs/bbs/assemble/control/reply/list/sub/{id}", get(reply_list_sub_id))
        .route("/jaxrs/bbs/assemble/control/subject/view/{id}", get(subject_view_id))
        .route("/jaxrs/bbs/assemble/control/subject/top/{sectionId}", get(subject_top_sectionId))
        .route("/jaxrs/bbs/assemble/control/permission/section/{sectionId}", get(permission_section_sectionId))
        .route("/jaxrs/bbs/assemble/control/permission/subject/{subjectId}", get(permission_subject_subjectId))
        .route("/jaxrs/bbs/assemble/control/section/viewforum/{forumId}", get(section_viewforum_forumId))
        .route("/jaxrs/bbs/assemble/control/update/control/config", post(update_control_config))
        .route("/jaxrs/bbs/assemble/control/shutup/create", post(shutup_create))
        .route("/jaxrs/bbs/assemble/control/uuid", get(uuid_generate))
}
