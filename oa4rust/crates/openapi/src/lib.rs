use utoipa::OpenApi;

// Minimal inline path handlers for OpenAPI spec generation.
// These are placeholders that satisfy the utoipa derive macro.

#[utoipa::path(get, path = "/jaxrs/base/echo/get", tag = "base")]
async fn echo_get() {}

#[utoipa::path(get, path = "/jaxrs/base/cache/detail", tag = "base")]
async fn cache_detail() {}

#[utoipa::path(get, path = "/jaxrs/base/openapi/info", tag = "base")]
async fn openapi_info() {}

#[utoipa::path(get, path = "/jaxrs/control/person/list", tag = "control")]
async fn control_person_list() {}

#[utoipa::path(get, path = "/jaxrs/control/unit/list", tag = "control")]
async fn control_unit_list() {}

#[utoipa::path(get, path = "/jaxrs/control/role/list", tag = "control")]
async fn control_role_list() {}

#[utoipa::path(get, path = "/jaxrs/control/group/list", tag = "control")]
async fn control_group_list() {}

#[utoipa::path(get, path = "/jaxrs/meeting/room/list", tag = "meeting")]
async fn meeting_room_list() {}

#[utoipa::path(get, path = "/jaxrs/meeting/building/list", tag = "meeting")]
async fn meeting_building_list() {}

#[utoipa::path(get, path = "/jaxrs/bbs/forum/list", tag = "bbs")]
async fn bbs_forum_list() {}

#[utoipa::path(get, path = "/jaxrs/bbs/post/list", tag = "bbs")]
async fn bbs_post_list() {}

#[utoipa::path(get, path = "/jaxrs/file/list", tag = "file")]
async fn file_list() {}

#[utoipa::path(get, path = "/jaxrs/portal/page/list", tag = "portal")]
async fn portal_page_list() {}

#[utoipa::path(get, path = "/jaxrs/correlation/list", tag = "correlation")]
async fn correlation_list() {}

#[derive(OpenApi)]
#[openapi(
    paths(
        echo_get,
        cache_detail,
        openapi_info,
        control_person_list,
        control_unit_list,
        control_role_list,
        control_group_list,
        meeting_room_list,
        meeting_building_list,
        bbs_forum_list,
        bbs_post_list,
        file_list,
        portal_page_list,
        correlation_list,
    ),
    tags(
        (name = "base", description = "Base operations"),
        (name = "control", description = "Control operations"),
        (name = "meeting", description = "Meeting operations"),
        (name = "bbs", description = "BBS operations"),
        (name = "file", description = "File operations"),
        (name = "portal", description = "Portal operations"),
        (name = "correlation", description = "Correlation operations"),
    ),
    info(
        title = "OA4Rust API",
        description = "OA4Rust OpenAPI specification",
        version = "0.1.0"
    )
)]
pub struct ApiDoc;
