use axum::{
    routing::{get, post, put, delete},
    Router,
};
use deadpool_postgres::Pool;

use crate::{
    create_reply, create_topic, get_control_config, get_forum, list_control_sections, list_forums, list_topics_by_forum, update_control_config, forum_view_all, permission_section_sectionId,
    permission_subject_subjectId, reply_list_sub_id, section_viewforum_forumId, subject_top_sectionId, subject_view_id, shutup_create, uuid_generate, forum_id, mobile_view_all, permission_replyPublishable_subjectId,
    permission_subjectPublishable_sectionId, reply_filter_list_page_page_count_count, delete_forum, delete_reply, delete_subject, list_reply_filter, list_topics_creamed, list_topics_recommended, list_subjects_filtered, list_subjects_index,
    list_subjects_recommended_index, login, logout, picture_list, shutup_delete, shutup_list, subject_creamed_list, subject_filter_list, subject_filter_listsubjectinfo, subject_index_list,
    subject_search, subject_statgrade, topic_creamed_list, topic_filter_list, topic_filter_listsubjectinfo, topic_index_list, topic_recommended_index, topic_recommended_list, topic_search, user_forum_list,
    user_info, user_reply_list, user_role_list, user_section_list, user_setting, user_subject_list, subjectattach_list,};


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
        .route("/jaxrs/bbs/assemble/control/delete/forum", post(crate::delete_forum))
        .route("/jaxrs/bbs/assemble/control/delete/reply", post(crate::delete_reply))
        .route("/jaxrs/bbs/assemble/control/delete/subject", post(crate::delete_subject))
        .route("/jaxrs/bbs/assemble/control/list/reply/filter", get(crate::list_reply_filter))
        .route("/jaxrs/bbs/assemble/control/list/subjects/filtered", get(crate::list_subjects_filtered))
        .route("/jaxrs/bbs/assemble/control/list/subjects/index", get(crate::list_subjects_index))
        .route("/jaxrs/bbs/assemble/control/list/subjects/recommended/index", get(crate::list_subjects_recommended_index))
        .route("/jaxrs/bbs/assemble/control/list/topics/creamed", get(crate::list_topics_creamed))
        .route("/jaxrs/bbs/assemble/control/list/topics/recommended", get(crate::list_topics_recommended))
        .route("/jaxrs/bbs/assemble/control/permission/{replyPublishable}/{subjectId}", get(crate::permission_replyPublishable_subjectId))
        .route("/jaxrs/bbs/assemble/control/picture/list", get(crate::picture_list))
        .route("/jaxrs/bbs/assemble/control/reply/filter/list/{page}/{page}/{count}/{count}", get(crate::reply_filter_list_page_page_count_count))
        .route("/jaxrs/bbs/assemble/control/shutup/delete", post(crate::shutup_delete))
        .route("/jaxrs/bbs/assemble/control/shutup/list", get(crate::shutup_list))
        .route("/jaxrs/bbs/assemble/control/subject/creamed/list", get(crate::subject_creamed_list))
        .route("/jaxrs/bbs/assemble/control/subject/filter/list", get(crate::subject_filter_list))
        .route("/jaxrs/bbs/assemble/control/subject/index/list", get(crate::subject_index_list))
        .route("/jaxrs/bbs/assemble/control/subject/search", get(crate::subject_search))
        .route("/jaxrs/bbs/assemble/control/subject/statgrade", get(crate::subject_statgrade))
        .route("/jaxrs/bbs/assemble/control/subjectattach/list", get(crate::subjectattach_list))
        .route("/jaxrs/bbs/assemble/control/topic/creamed/list", get(crate::topic_creamed_list))
        .route("/jaxrs/bbs/assemble/control/topic/filter/list", get(crate::topic_filter_list))
        .route("/jaxrs/bbs/assemble/control/topic/index/list", get(crate::topic_index_list))
        .route("/jaxrs/bbs/assemble/control/topic/recommended/index", get(crate::topic_recommended_index))
        .route("/jaxrs/bbs/assemble/control/topic/recommended/list", get(crate::topic_recommended_list))
        .route("/jaxrs/bbs/assemble/control/topic/search", get(crate::topic_search))
        .route("/jaxrs/bbs/assemble/control/user/info", get(crate::user_info))
}
