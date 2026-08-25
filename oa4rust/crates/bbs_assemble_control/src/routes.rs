use axum::{
    routing::{get, post, put, delete},
    Router,
};
use deadpool_postgres::Pool;

use crate::{
    create_reply, create_topic, get_control_config, list_control_sections, list_forums, update_control_config, forum_view_all, get_forum,
    permission_section_sectionId, permission_subject_subjectId, reply_list_sub_id, section_viewforum_forumId,
    subject_top_sectionId, subject_view_id, shutup_create, uuid_generate, mobile_view_all, shutup_delete, shutup_list,
    topic_recommended_index, subject_statgrade, subject_creamed_list, subject_filter_list, subject_index_list,
    user_forum_list, user_info, user_section_list, login, logout,
};
use crate::u2;

/// plan002 U2 — Java 端点全集（106 条）统一前缀。
pub const JAVA_BASE: &str = "/jaxrs/bbs/assemble/control";

pub fn router(pool: Pool) -> Router {
    Router::new()
        // ════════ 既有业务路由（保留，非 Java 全集成员的扩展端点）════════
        .route("/jaxrs/bbs/assemble/control/config", get(get_control_config))
        .route("/jaxrs/bbs/assemble/control/section/list", get(list_control_sections))
        .route("/jaxrs/bbs/assemble/control/forum/list", get(list_forums))
        .route("/jaxrs/bbs/assemble/control/topic/create", post(create_topic))
        .route("/jaxrs/bbs/assemble/control/topic/list/forum/{forumId}", get(crate::list_topics_by_forum))
        .route("/jaxrs/bbs/assemble/control/reply/create", post(create_reply))
        .route("/jaxrs/bbs/assemble/control/update/control/config", post(update_control_config).put(update_control_config))
        .route("/jaxrs/bbs/assemble/control/shutup/create", post(shutup_create))
        .route("/jaxrs/bbs/assemble/control/shutup/delete", post(shutup_delete).delete(shutup_delete))
        .route("/jaxrs/bbs/assemble/control/delete/forum", post(crate::delete_forum).delete(crate::delete_forum))
        .route("/jaxrs/bbs/assemble/control/delete/reply", post(crate::delete_reply).delete(crate::delete_reply))
        .route("/jaxrs/bbs/assemble/control/delete/subject", post(crate::delete_subject).delete(crate::delete_subject))
        .route("/jaxrs/bbs/assemble/control/list/reply/filter", get(crate::list_reply_filter))
        .route("/jaxrs/bbs/assemble/control/list/subjects/filtered", get(crate::list_subjects_filtered))
        .route("/jaxrs/bbs/assemble/control/list/subjects/index", get(crate::list_subjects_index))
        .route("/jaxrs/bbs/assemble/control/list/subjects/recommended/index", get(crate::list_subjects_recommended_index))
        .route("/jaxrs/bbs/assemble/control/list/topics/creamed", get(crate::list_topics_creamed))
        .route("/jaxrs/bbs/assemble/control/list/topics/recommended", get(crate::list_topics_recommended))
        .route("/jaxrs/bbs/assemble/control/picture/list/{subjectId}", get(crate::picture_list))
        .route("/jaxrs/bbs/assemble/control/subject/creamed/list", get(subject_creamed_list))
        .route("/jaxrs/bbs/assemble/control/subject/filter/list", get(subject_filter_list))
        .route("/jaxrs/bbs/assemble/control/subject/index/list", get(subject_index_list))
        .route("/jaxrs/bbs/assemble/control/subject/search", get(crate::subject_search))
        .route("/jaxrs/bbs/assemble/control/subject/statgrade", get(subject_statgrade))
        .route("/jaxrs/bbs/assemble/control/topic/creamed/list", get(crate::topic_creamed_list))
        .route("/jaxrs/bbs/assemble/control/topic/filter/list", get(crate::topic_filter_list))
        .route("/jaxrs/bbs/assemble/control/topic/filter/listsubjectinfo", post(crate::topic_filter_listsubjectinfo))
        .route("/jaxrs/bbs/assemble/control/topic/index/list", get(crate::topic_index_list))
        .route("/jaxrs/bbs/assemble/control/topic/recommended/index", get(crate::topic_recommended_index))
        .route("/jaxrs/bbs/assemble/control/topic/recommended/list", get(crate::topic_recommended_list))
        .route("/jaxrs/bbs/assemble/control/topic/search", get(crate::topic_search))
        .route("/jaxrs/bbs/assemble/control/user/info", get(user_info))

        // ════════ Java 全集对齐（106 条；U2 冲刺 100%）════════
        // ── attachment（AttachmentAction，7 条；二进制流显式 501）──
        .route(&fmt("attachment/{id}"), get(u2::u2_attachment_get))
        .route(&fmt("attachment/{id}"), delete(u2::u2_attachment_delete))
        .route(&fmt("attachment/download/{id}"), get(u2::attachment_download_501))
        .route(&fmt("attachment/download/{id}/stream/{stream}"), get(u2::attachment_download_stream_501))
        .route(&fmt("attachment/list/subject/{subjectId}"), get(u2::u2_attachment_list_by_subject))
        .route(&fmt("attachment/upload/subject/{subjectId}"), post(u2::attachment_upload_501))
        .route(&fmt("attachment/upload/subject/{subjectId}/callback/{callback}"), post(u2::attachment_upload_callback_501))
        // ── forum（ForumInfoAction，2 条）──
        .route(&fmt("forum/{id}"), get(get_forum))
        .route(&fmt("forum/view/all"), get(forum_view_all))
        // ── login / logout ──
        .route(&fmt("login"), post(login))
        .route(&fmt("logout"), post(logout))
        // ── mobile（MobileIndexAction）──
        .route(&fmt("mobile/view/all"), get(mobile_view_all))
        // ── permission（PermissionInfoAction，5 条；通配路由已按归一化规则移除）──
        .route(&fmt("permission"), get(u2::u2_permission_root))
        .route(&fmt("permission/replyPublishable/{subjectId}"), get(crate::permission_replyPublishable_subjectId))
        .route(&fmt("permission/section/{sectionId}"), get(permission_section_sectionId))
        .route(&fmt("permission/subject/{subjectId}"), get(permission_subject_subjectId))
        .route(&fmt("permission/subjectPublishable/{sectionId}"), get(crate::permission_subjectPublishable_sectionId))
        // ── picture（PictureAction，2 条；图像引擎依赖显式 501）──
        .route(&fmt("picture/encode/base64/size/{size}"), post(u2::picture_encode_501))
        .route(&fmt("picture/section/{id}/icon"), post(u2::picture_section_icon_501))
        // ── reply（ReplyInfoAction，3 条；畸形 {page}/{page} 路由已替换）──
        .route(&fmt("reply/{id}"), get(u2::u2_reply_get))
        .route(&fmt("reply/filter/list/page/{page}/count/{count}"), put(u2::u2_reply_filter_list))
        .route(&fmt("reply/list/sub/{id}"), get(reply_list_sub_id))
        // ── section（SectionInfoAction，4 条）──
        .route(&fmt("section/{id}"), get(u2::u2_section_get))
        .route(&fmt("section/syn"), get(u2::section_syn_501))
        .route(&fmt("section/viewforum/{forumId}"), get(section_viewforum_forumId))
        .route(&fmt("section/viewsub/{sectionId}"), get(u2::u2_section_viewsub))
        // ── setting（BBSConfigSettingAnonymousAction）──
        .route(&fmt("setting/bbsName"), get(u2::u2_setting_bbs_name))
        // ── shutup（ShutupAction，4 条）──
        .route(&fmt("shutup/get/shutup"), get(u2::u2_shutup_get_mine))
        .route(&fmt("shutup/list/paging/{page}/size/{size}"), post(shutup_list))
        .route(&fmt("shutup/save"), post(shutup_create))
        .route(&fmt("shutup/{id}"), delete(u2::u2_shutup_delete_admin))
        // ── subject（SubjectInfoAction，10 条）──
        .route(&fmt("subject/recommended/index/{count}"), get(topic_recommended_index))
        .route(&fmt("subject/statgrade/sectionName/{sectionName}/subjectType/{subjectType}"), get(u2::u2_statgrade))
        .route(&fmt("subject/top/{sectionId}"), get(subject_top_sectionId))
        .route(&fmt("subject/view/{id}"), get(subject_view_id))
        .route(&fmt("subject/filter/listsubjectinfo/page/{page}/count/{count}"), post(u2::u2_subject_listsubjectinfo_page))
        .route(&fmt("subject/creamed/list/page/{page}/count/{count}"), put(subject_creamed_list))
        .route(&fmt("subject/filter/list/page/{page}/count/{count}"), put(subject_filter_list))
        .route(&fmt("subject/index/list/page/{page}/count/{count}"), put(subject_index_list))
        .route(&fmt("subject/recommended/list/page/{page}/count/{count}"), put(crate::topic_recommended_list))
        .route(&fmt("subject/search/list/page/{page}/count/{count}"), put(u2::u2_subject_search_page))
        // ── subjectattach（SubjectAttachmentAction，4 条）──
        .route(&fmt("subjectattach/{id}"), get(u2::u2_subjectattach_get).delete(u2::u2_subjectattach_delete))
        .route(&fmt("subjectattach/{id}/binary/base64/{size}"), get(u2::u2_subjectattach_base64))
        .route(&fmt("subjectattach/list/subject/{id}"), get(u2::u2_subjectattach_list))
        // ── user/forum（ForumInfoManagerUserAction，3 条）──
        .route(&fmt("user/forum"), post(u2::u2_user_forum_save))
        .route(&fmt("user/forum/{id}"), delete(u2::u2_user_forum_delete))
        .route(&fmt("user/forum/all"), get(user_forum_list))
        // ── user/permission（PermissionInfoAdminAction，3 条）──
        .route(&fmt("user/permission/forum/{forumId}"), get(u2::u2_permission_admin_forum))
        .route(&fmt("user/permission/role/{roleCode}"), get(u2::u2_permission_admin_role))
        .route(&fmt("user/permission/section/{sectionId}"), get(u2::u2_permission_admin_section))
        // ── user/reply（ReplyInfoManagerUserAction，4 条）──
        .route(&fmt("user/reply"), post(u2::u2_user_reply_save))
        .route(&fmt("user/reply/accept"), put(u2::u2_user_reply_accept))
        .route(&fmt("user/reply/my/list/page/{page}/count/{count}"), put(u2::u2_my_reply_list))
        .route(&fmt("user/reply/{id}"), delete(u2::u2_user_reply_delete))
        // ── user/role（RoleInfoAction，11 条）──
        .route(&fmt("user/role"), post(u2::u2_role_save))
        .route(&fmt("user/role/bind/object"), put(u2::u2_role_bind_object))
        .route(&fmt("user/role/bind/role"), put(u2::u2_role_bind_user))
        .route(&fmt("user/role/forum/{forumId}"), put(u2::u2_role_list_by_forum))
        .route(&fmt("user/role/rolecode/selected"), put(u2::u2_role_selected_by_code))
        .route(&fmt("user/role/section/{sectionId}"), put(u2::u2_role_list_by_section))
        .route(&fmt("user/role/unit/selected"), put(u2::u2_role_by_unit))
        .route(&fmt("user/role/user/selected"), put(u2::u2_role_by_user))
        .route(&fmt("user/role/{id}"), get(u2::u2_role_get).delete(u2::u2_role_delete))
        .route(&fmt("user/role/all"), get(u2::u2_role_all))
        // ── user/section（SectionInfoManagerUserAction，6 条）──
        .route(&fmt("user/section"), post(u2::u2_user_section_save))
        .route(&fmt("user/section/force/{id}"), delete(u2::u2_user_section_delete_force))
        .route(&fmt("user/section/forum/{forumId}"), get(u2::u2_user_section_forum))
        .route(&fmt("user/section/sub/{sectionId}"), get(u2::u2_user_section_sub))
        .route(&fmt("user/section/{id}"), delete(u2::u2_user_section_delete))
        .route(&fmt("user/section/all"), get(user_section_list))
        // ── user/setting（BBSConfigSettingAction，4 条）──
        .route(&fmt("user/setting/code"), put(u2::u2_setting_get_by_code))
        .route(&fmt("user/setting/{id}"), get(u2::u2_setting_get))
        .route(&fmt("user/setting/all"), get(u2::u2_setting_all))
        .route(&fmt("user/setting"), put(u2::u2_setting_update))
        // ── user/subject（SubjectInfoManagerUserAction，27 条）──
        .route(&fmt("user/subject/change/section"), put(u2::u2_subject_change_section))
        .route(&fmt("user/subject/my/list/page/{page}/count/{count}"), put(u2::u2_my_subject_list))
        .route(&fmt("user/subject/vote/submit"), put(u2::u2_vote_submit))
        .route(&fmt("user/subject/voterecord/list/page/{page}/count/{count}"), put(u2::u2_voterecord_list))
        .route(&fmt("user/subject/acceptreply/{id}/{replyId}"), get(u2::u2_subject_accept_reply))
        .route(&fmt("user/subject/unacceptreply/{id}"), get(u2::u2_subject_unaccept_reply))
        .route(
            &fmt("user/subject/setCream/{id}"),
            get(u2::u2_subject_set_cream),
        )
        .route(&fmt("user/subject/nonCream/{id}"), get(u2::u2_subject_non_cream))
        .route(&fmt("user/subject/setOriginal/{id}"), get(u2::u2_subject_set_original))
        .route(&fmt("user/subject/nonOriginal/{id}"), get(u2::u2_subject_non_original))
        .route(&fmt("user/subject/setRecommendToBBSIndex/{id}"), get(u2::u2_subject_set_recommend_index))
        .route(&fmt("user/subject/nonRecommendToBBSIndex/{id}"), get(u2::u2_subject_non_recommend_index))
        .route(&fmt("user/subject/topToBBS/{id}"), get(u2::u2_subject_top_to_bbs))
        .route(&fmt("user/subject/nonTopToBBS/{id}"), get(u2::u2_subject_non_top_to_bbs))
        .route(&fmt("user/subject/topToForum/{id}"), get(u2::u2_subject_top_to_forum))
        .route(&fmt("user/subject/nonTopToForum/{id}"), get(u2::u2_subject_non_top_to_forum))
        .route(&fmt("user/subject/topToMainSection/{id}"), get(u2::u2_subject_top_to_main_section))
        .route(&fmt("user/subject/nonTopToMainSection/{id}"), get(u2::u2_subject_non_top_to_main_section))
        .route(&fmt("user/subject/topToSection/{id}"), get(u2::u2_subject_top_to_section))
        .route(&fmt("user/subject/nonTopToSection/{id}"), get(u2::u2_subject_non_top_to_section))
        .route(&fmt("user/subject/lock/{id}"), get(u2::u2_subject_lock))
        .route(&fmt("user/subject/unlock/{id}"), get(u2::u2_subject_unlock))
        .route(&fmt("user/subject/complete/{id}"), get(u2::u2_subject_complete))
        .route(&fmt("user/subject/uncomplete/{id}"), get(u2::u2_subject_uncomplete))
        .route(&fmt("user/subject"), post(u2::u2_subject_save))
        .route(&fmt("user/subject/{id}"), get(u2::u2_subject_get))
        .route(&fmt("user/subject/{id}"), delete(u2::u2_subject_soft_delete))
        // ── userinfo（UserInfoAction，2 条）──
        .route(&fmt("userinfo/update/nick/name/{person}"), get(u2::u2_userinfo_update_nick))
        .route(&fmt("userinfo"), put(u2::u2_userinfo_filter))
        // ── uuid（UUIDAction；legacy /uuid 保留兼容）──
        .route(&fmt("uuid/random"), get(uuid_generate))
        .route("/jaxrs/bbs/assemble/control/uuid", get(uuid_generate))
        .layer(axum::extract::Extension(pool))
}

fn fmt(java_relative: &str) -> String {
    let normalized = u2::normalize_java_path("", java_relative);
    format!("{}/{}", JAVA_BASE, normalized)
}
