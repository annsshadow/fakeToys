use axum::{
    extract::Extension,
    routing::{get, post},
    Router,
};
use deadpool_postgres::Pool;

use crate::{
    create_reply, create_topic, get_control_config, get_forum, list_control_sections,
    list_forums, list_topics_by_forum, update_control_config,
    stub_bbs_assemble_control_permission_section_sectionId,
    stub_bbs_assemble_control_permission_subject_subjectId,
    stub_bbs_assemble_control_reply_list_sub_id,
    stub_bbs_assemble_control_section_viewforum_forumId,
    stub_bbs_assemble_control_subject_top_sectionId,
    stub_bbs_assemble_control_subject_view_id,
};

pub fn router(pool: Pool) -> Router {
    Router::new()
        .route("/jaxrs/bbs/assemble/control/config/get", get(get_control_config))
        .route("/jaxrs/bbs/assemble/control/sections", get(list_control_sections))
        .route("/jaxrs/bbs/assemble/control/config/update", get(update_control_config))
        .route("/jaxrs/bbs/assemble/control/forum/list", get(list_forums))
        .route("/jaxrs/bbs/assemble/control/forum/{id}", get(get_forum))
        .route("/jaxrs/bbs/assemble/control/topic/create", post(create_topic))
        .route("/jaxrs/bbs/assemble/control/topic/list/{forumId}", get(list_topics_by_forum))
        .route("/jaxrs/bbs/assemble/control/reply/create", post(create_reply))
        .route("/jaxrs/bbs/assemble/control/reply/list/sub/{id}", get(stub_bbs_assemble_control_reply_list_sub_id))
        .route("/jaxrs/bbs/assemble/control/subject/view/{id}", get(stub_bbs_assemble_control_subject_view_id))
        .route("/jaxrs/bbs/assemble/control/subject/top/{sectionId}", get(stub_bbs_assemble_control_subject_top_sectionId))
        .route("/jaxrs/bbs/assemble/control/permission/section/{sectionId}", get(stub_bbs_assemble_control_permission_section_sectionId))
        .route("/jaxrs/bbs/assemble/control/permission/subject/{subjectId}", get(stub_bbs_assemble_control_permission_subject_subjectId))
        .route("/jaxrs/bbs/assemble/control/section/viewforum/{forumId}", get(stub_bbs_assemble_control_section_viewforum_forumId))
        .layer(Extension(pool))
}
