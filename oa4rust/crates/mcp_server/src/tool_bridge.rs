use axum::{
    body::Body,
    http::{Request},
    Router,
};
use deadpool_postgres::Pool;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use shared::{
    middleware::{cors_middleware, security_headers_middleware, trace_middleware, SecurityState},
    session::SessionManager,
};
use std::collections::HashMap;
use std::sync::Arc;
use tower::ServiceExt;

// ──────────────────────────────────────────────────────────────────────────────
// Tool metadata surfaced to MCP clients via tools/list.
// ──────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolInputSchema {
    #[serde(rename = "type")]
    #[serde(skip_deserializing)]
    pub typ: &'static str,
    #[serde(skip_deserializing)]
    pub properties: HashMap<&'static str, ToolProperty>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(skip_deserializing)]
    pub required: Option<Vec<&'static str>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolProperty {
    #[serde(rename = "type")]
    #[serde(skip_deserializing)]
    pub typ: &'static str,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(skip_deserializing)]
    pub description: Option<&'static str>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpTool {
    pub name: String,
    pub description: String,
    #[serde(rename = "inputSchema")]
    pub input_schema: ToolInputSchema,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListToolsResponse {
    pub tools: Vec<McpTool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CallToolResponse {
    pub content: Vec<ToolContent>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_error: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ToolContent {
    #[serde(rename = "text")]
    Text { text: String },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolCallParams {
    pub name: String,
    pub arguments: HashMap<String, Value>,
}

// ──────────────────────────────────────────────────────────────────────────────
// Internal route descriptor.
// ──────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq)]
enum HttpMethod {
    Get,
    Post,
    Put,
    Delete,
}

impl HttpMethod {
    fn as_str(self) -> &'static str {
        match self {
            HttpMethod::Get => "GET",
            HttpMethod::Post => "POST",
            HttpMethod::Put => "PUT",
            HttpMethod::Delete => "DELETE",
        }
    }

    fn as_axum(self) -> axum::http::Method {
        match self {
            HttpMethod::Get => axum::http::Method::GET,
            HttpMethod::Post => axum::http::Method::POST,
            HttpMethod::Put => axum::http::Method::PUT,
            HttpMethod::Delete => axum::http::Method::DELETE,
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct RouteDef {
    tool_name: &'static str,
    method: HttpMethod,
    path: &'static str,
    description: &'static str,
    path_params: &'static [&'static str],
    body_params: &'static [&'static str],
}

macro_rules! register_tool {
    (
        name: $tool_name:literal,
        method: $method:ident,
        path: $path:literal,
        desc: $desc:literal,
        path_params: [$($pp:literal),* $(,)?],
        body_params: [$($bp:literal),* $(,)?],
    ) => {
        RouteDef {
            tool_name: $tool_name,
            method: HttpMethod::$method,
            path: $path,
            description: $desc,
            path_params: &[$($pp),*],
            body_params: &[$($bp),*],
        }
    };
}

/// Static route registry. Each entry maps an MCP tool name to an axum route.
/// The tool name follows the convention `jaxrs_{crate}_{action}`.
static ROUTE_DEFS: &[RouteDef] = &[
    // ── shared (router.rs) ──────────────────────────────────────────────────
    register_tool! {
        name: "jaxrs_base_health",
        method: Get,
        path: "/health",
        desc: "Health check endpoint",
        path_params: [],
        body_params: [],
    },
    // ── auth ────────────────────────────────────────────────────────────────
    register_tool! {
        name: "jaxrs_authentication_login",
        method: Post,
        path: "/jaxrs/authentication/login",
        desc: "Authenticate with credential",
        path_params: [],
        body_params: ["credential", "password"],
    },
    register_tool! {
        name: "jaxrs_authentication_logout",
        method: Post,
        path: "/jaxrs/authentication/logout",
        desc: "Logout current session",
        path_params: [],
        body_params: [],
    },
    register_tool! {
        name: "jaxrs_authentication_who",
        method: Get,
        path: "/jaxrs/authentication/who",
        desc: "Return current session user info",
        path_params: [],
        body_params: [],
    },
    register_tool! {
        name: "jaxrs_authentication_captcha",
        method: Get,
        path: "/jaxrs/authentication/captcha",
        desc: "Generate captcha image",
        path_params: [],
        body_params: ["width", "height"],
    },
    register_tool! {
        name: "jaxrs_authentication_captcha_dimensions",
        method: Get,
        path: "/jaxrs/authentication/captcha/width/{width}/height/{height}",
        desc: "Generate captcha with specific dimensions",
        path_params: ["width", "height"],
        body_params: [],
    },
    register_tool! {
        name: "jaxrs_authentication_code",
        method: Post,
        path: "/jaxrs/authentication/code",
        desc: "Send verification code",
        path_params: [],
        body_params: ["credential"],
    },
    register_tool! {
        name: "jaxrs_authentication_code_credential",
        method: Get,
        path: "/jaxrs/authentication/code/credential/{credential}",
        desc: "Check verification code status for credential",
        path_params: ["credential"],
        body_params: [],
    },
    register_tool! {
        name: "jaxrs_authentication_bind",
        method: Post,
        path: "/jaxrs/authentication/bind",
        desc: "Bind account to third-party",
        path_params: [],
        body_params: ["meta", "code"],
    },
    register_tool! {
        name: "jaxrs_authentication_bind_meta",
        method: Get,
        path: "/jaxrs/authentication/bind/meta/{meta}",
        desc: "Get bind metadata",
        path_params: ["meta"],
        body_params: [],
    },
    register_tool! {
        name: "jaxrs_authentication_oauth",
        method: Post,
        path: "/jaxrs/authentication/oauth",
        desc: "Initiate OAuth flow",
        path_params: [],
        body_params: ["name"],
    },
    register_tool! {
        name: "jaxrs_authentication_oauth_list",
        method: Get,
        path: "/jaxrs/authentication/oauth/list",
        desc: "List configured OAuth providers",
        path_params: [],
        body_params: [],
    },
    register_tool! {
        name: "jaxrs_authentication_oauth_qywx_config",
        method: Get,
        path: "/jaxrs/authentication/oauth/qywx/config",
        desc: "Get WeCom OAuth config",
        path_params: [],
        body_params: [],
    },
    register_tool! {
        name: "jaxrs_authentication_oauth_dingding_config",
        method: Get,
        path: "/jaxrs/authentication/oauth/dingding/config",
        desc: "Get DingTalk OAuth config",
        path_params: [],
        body_params: [],
    },
    register_tool! {
        name: "jaxrs_authentication_oauth_name",
        method: Get,
        path: "/jaxrs/authentication/oauth/name/{name}",
        desc: "Get OAuth provider config by name",
        path_params: ["name"],
        body_params: [],
    },
    register_tool! {
        name: "jaxrs_authentication_oauth_login_qywx",
        method: Get,
        path: "/jaxrs/authentication/oauth/login/qywx/code/{code}",
        desc: "OAuth login via WeCom authorization code",
        path_params: ["code"],
        body_params: [],
    },
    register_tool! {
        name: "jaxrs_authentication_oauth_login_dingding",
        method: Get,
        path: "/jaxrs/authentication/oauth/login/dingding/code/{code}",
        desc: "OAuth login via DingTalk authorization code",
        path_params: ["code"],
        body_params: [],
    },
    register_tool! {
        name: "jaxrs_authentication_oauth_login_name",
        method: Get,
        path: "/jaxrs/authentication/oauth/login/name/{name}/code/{code}/redirecturi/{redirectUri}",
        desc: "OAuth login with redirect URI",
        path_params: ["name", "code", "redirectUri"],
        body_params: [],
    },
    register_tool! {
        name: "jaxrs_authentication_oauth_bind_name",
        method: Get,
        path: "/jaxrs/authentication/oauth/bind/name/{name}/code/{code}/redirecturi/{redirectUri}",
        desc: "OAuth bind with redirect URI",
        path_params: ["name", "code", "redirectUri"],
        body_params: [],
    },
    register_tool! {
        name: "jaxrs_authentication_refresh",
        method: Post,
        path: "/jaxrs/authentication/refresh",
        desc: "Refresh current session token",
        path_params: [],
        body_params: [],
    },
    register_tool! {
        name: "jaxrs_secret_captcha_verify",
        method: Post,
        path: "/jaxrs/secret/captcha/verify",
        desc: "Verify captcha answer",
        path_params: [],
        body_params: ["captchaId", "answer"],
    },
    // ── personal ─────────────────────────────────────────────────────────────
    register_tool! {
        name: "jaxrs_person_list",
        method: Get,
        path: "/jaxrs/person",
        desc: "List persons",
        path_params: [],
        body_params: [],
    },
    register_tool! {
        name: "jaxrs_person_create",
        method: Post,
        path: "/jaxrs/person",
        desc: "Create a person",
        path_params: [],
        body_params: ["name", "email"],
    },
    register_tool! {
        name: "jaxrs_person_get",
        method: Get,
        path: "/jaxrs/person/{id}",
        desc: "Get person by id",
        path_params: ["id"],
        body_params: [],
    },
    register_tool! {
        name: "jaxrs_person_update",
        method: Put,
        path: "/jaxrs/person/{id}",
        desc: "Update person",
        path_params: ["id"],
        body_params: ["name"],
    },
    register_tool! {
        name: "jaxrs_person_delete",
        method: Delete,
        path: "/jaxrs/person/{id}",
        desc: "Delete person",
        path_params: ["id"],
        body_params: [],
    },
    register_tool! {
        name: "jaxrs_person_password",
        method: Post,
        path: "/jaxrs/person/{id}/password",
        desc: "Change person password",
        path_params: ["id"],
        body_params: ["oldPassword", "newPassword"],
    },
    register_tool! {
        name: "jaxrs_person_icon",
        method: Post,
        path: "/jaxrs/person/{id}/icon",
        desc: "Upload person avatar icon",
        path_params: ["id"],
        body_params: ["icon"],
    },
    register_tool! {
        name: "jaxrs_reset_password_anonymous",
        method: Post,
        path: "/jaxrs/reset/password/anonymous",
        desc: "Reset password without authentication",
        path_params: [],
        body_params: ["credential", "code", "newPassword"],
    },
    // ── base ────────────────────────────────────────────────────────────────
    register_tool! {
        name: "jaxrs_base_echo",
        method: Get,
        path: "/jaxrs/base/echo/get",
        desc: "Echo back request info (health/connectivity check)",
        path_params: [],
        body_params: [],
    },
    register_tool! {
        name: "jaxrs_base_cache_detail",
        method: Get,
        path: "/jaxrs/base/cache/detail",
        desc: "Return cache configuration details",
        path_params: [],
        body_params: [],
    },
    register_tool! {
        name: "jaxrs_base_openapi_info",
        method: Get,
        path: "/jaxrs/base/openapi/info",
        desc: "Return OpenAPI info metadata",
        path_params: [],
        body_params: [],
    },
    // ── control ─────────────────────────────────────────────────────────────
    register_tool! {
        name: "jaxrs_control_unit_list",
        method: Get,
        path: "/jaxrs/control/unit/list",
        desc: "List organizational units",
        path_params: [],
        body_params: [],
    },
    // ── express ─────────────────────────────────────────────────────────────
    register_tool! {
        name: "jaxrs_express_module_list",
        method: Get,
        path: "/jaxrs/express/module/list",
        desc: "List express modules",
        path_params: [],
        body_params: [],
    },
    // ── message ─────────────────────────────────────────────────────────────
    register_tool! {
        name: "jaxrs_message_inbox_list",
        method: Get,
        path: "/jaxrs/message/inbox/list",
        desc: "List inbox messages",
        path_params: [],
        body_params: [],
    },
    register_tool! {
        name: "jaxrs_message_send",
        method: Post,
        path: "/jaxrs/message/send",
        desc: "Send a message",
        path_params: [],
        body_params: ["to", "content", "title"],
    },
    // ── portal ──────────────────────────────────────────────────────────────
    register_tool! {
        name: "jaxrs_portal_home",
        method: Get,
        path: "/jaxrs/portal/home",
        desc: "Return portal home data",
        path_params: [],
        body_params: [],
    },
    // ── bbs ─────────────────────────────────────────────────────────────────
    register_tool! {
        name: "jaxrs_bbs_topic_list",
        method: Get,
        path: "/jaxrs/bbs/topic/list",
        desc: "List BBS topics",
        path_params: [],
        body_params: [],
    },
    register_tool! {
        name: "jaxrs_bbs_topic_create",
        method: Post,
        path: "/jaxrs/bbs/topic",
        desc: "Create a BBS topic",
        path_params: [],
        body_params: ["title", "content"],
    },
    register_tool! {
        name: "jaxrs_bbs_topic_get",
        method: Get,
        path: "/jaxrs/bbs/topic/{id}",
        desc: "Get BBS topic by id",
        path_params: ["id"],
        body_params: [],
    },
    register_tool! {
        name: "jaxrs_bbs_post_list",
        method: Get,
        path: "/jaxrs/bbs/post/list",
        desc: "List BBS posts",
        path_params: [],
        body_params: [],
    },
    register_tool! {
        name: "jaxrs_bbs_post_create",
        method: Post,
        path: "/jaxrs/bbs/post",
        desc: "Create a BBS post",
        path_params: [],
        body_params: ["topicId", "content"],
    },
    // ── calendar ────────────────────────────────────────────────────────────
    register_tool! {
        name: "jaxrs_calendar_event_list",
        method: Get,
        path: "/jaxrs/calendar/event/list",
        desc: "List calendar events",
        path_params: [],
        body_params: [],
    },
    register_tool! {
        name: "jaxrs_calendar_event_create",
        method: Post,
        path: "/jaxrs/calendar/event",
        desc: "Create a calendar event",
        path_params: [],
        body_params: ["title", "\"startTime\"", "\"endTime\""],
    },
    // ── component ───────────────────────────────────────────────────────────
    register_tool! {
        name: "jaxrs_component_list",
        method: Get,
        path: "/jaxrs/component/list",
        desc: "List components",
        path_params: [],
        body_params: [],
    },
    // ── file ────────────────────────────────────────────────────────────────
    register_tool! {
        name: "jaxrs_file_upload",
        method: Post,
        path: "/jaxrs/file/upload",
        desc: "Upload a file",
        path_params: [],
        body_params: ["\"fileName\"", "contentType", "data"],
    },
    register_tool! {
        name: "jaxrs_file_list",
        method: Get,
        path: "/jaxrs/file/list",
        desc: "List uploaded files",
        path_params: [],
        body_params: [],
    },
    register_tool! {
        name: "jaxrs_file_get",
        method: Get,
        path: "/jaxrs/file/{id}",
        desc: "Get file metadata by id",
        path_params: ["id"],
        body_params: [],
    },
    register_tool! {
        name: "jaxrs_file_delete",
        method: Delete,
        path: "/jaxrs/file/{id}",
        desc: "Delete file by id",
        path_params: ["id"],
        body_params: [],
    },
    // ── ai ──────────────────────────────────────────────────────────────────
    register_tool! {
        name: "jaxrs_ai_chat",
        method: Post,
        path: "/jaxrs/ai/chat",
        desc: "Send chat message to AI",
        path_params: [],
        body_params: ["prompt", "model"],
    },
    // ── attendance ──────────────────────────────────────────────────────────
    register_tool! {
        name: "jaxrs_attendance_record_list",
        method: Get,
        path: "/jaxrs/attendance/record/list",
        desc: "List attendance records",
        path_params: [],
        body_params: [],
    },
    register_tool! {
        name: "jaxrs_attendance_record_create",
        method: Post,
        path: "/jaxrs/attendance/record",
        desc: "Clock in / create attendance record",
        path_params: [],
        body_params: ["personId", "type"],
    },
    // ── correlation ─────────────────────────────────────────────────────────
    register_tool! {
        name: "jaxrs_correlation_link_list",
        method: Get,
        path: "/jaxrs/correlation/link/list",
        desc: "List correlation links",
        path_params: [],
        body_params: [],
    },
    register_tool! {
        name: "jaxrs_correlation_link_create",
        method: Post,
        path: "/jaxrs/correlation/link",
        desc: "Create a correlation link",
        path_params: [],
        body_params: ["sourceId", "targetId", "type"],
    },
    // ── general ─────────────────────────────────────────────────────────────
    register_tool! {
        name: "jaxrs_general_config_list",
        method: Get,
        path: "/jaxrs/general/config/list",
        desc: "List general configurations",
        path_params: [],
        body_params: [],
    },
    register_tool! {
        name: "jaxrs_general_config_get",
        method: Get,
        path: "/jaxrs/general/config/{key}",
        desc: "Get general config by key",
        path_params: ["key"],
        body_params: [],
    },
    // ── hotpic ──────────────────────────────────────────────────────────────
    register_tool! {
        name: "jaxrs_hotpic_list",
        method: Get,
        path: "/jaxrs/hotpic/list",
        desc: "List hot pictures",
        path_params: [],
        body_params: [],
    },
    register_tool! {
        name: "jaxrs_hotpic_upload",
        method: Post,
        path: "/jaxrs/hotpic/upload",
        desc: "Upload a hot picture",
        path_params: [],
        body_params: ["title", "imageData"],
    },
    // ── jpush ───────────────────────────────────────────────────────────────
    register_tool! {
        name: "jaxrs_jpush_send",
        method: Post,
        path: "/jaxrs/jpush/send",
        desc: "Send a JPush notification",
        path_params: [],
        body_params: ["alias", "alert", "title"],
    },
    // ── meeting ─────────────────────────────────────────────────────────────
    register_tool! {
        name: "jaxrs_meeting_room_list",
        method: Get,
        path: "/jaxrs/meeting/room/list",
        desc: "List meeting rooms",
        path_params: [],
        body_params: [],
    },
    register_tool! {
        name: "jaxrs_meeting_building_list",
        method: Get,
        path: "/jaxrs/meeting/building/list",
        desc: "List meeting buildings",
        path_params: [],
        body_params: [],
    },
    register_tool! {
        name: "jaxrs_meeting_openmeeting_list_room",
        method: Get,
        path: "/jaxrs/meeting/openmeeting/list/room",
        desc: "List rooms available for open meetings",
        path_params: [],
        body_params: [],
    },
    register_tool! {
        name: "jaxrs_meeting_create",
        method: Post,
        path: "/jaxrs/meeting/create",
        desc: "Create a new meeting",
        path_params: [],
        body_params: ["title", "content", "roomId", "\"startTime\"", "\"endTime\"", "creator"],
    },
    register_tool! {
        name: "jaxrs_meeting_get_by_id",
        method: Get,
        path: "/jaxrs/meeting/{id}",
        desc: "Get meeting by id",
        path_params: ["id"],
        body_params: [],
    },
    register_tool! {
        name: "jaxrs_meeting_list",
        method: Get,
        path: "/jaxrs/meeting/list",
        desc: "List meetings",
        path_params: [],
        body_params: [],
    },
    register_tool! {
        name: "jaxrs_meeting_schedule_days",
        method: Get,
        path: "/jaxrs/meeting/schedule/days/{days}",
        desc: "List meetings scheduled for the next N days",
        path_params: ["days"],
        body_params: [],
    },
    register_tool! {
        name: "jaxrs_meeting_participant_add",
        method: Post,
        path: "/jaxrs/meeting/{\"meetingId\"}/participant/add",
        desc: "Add a participant to a meeting",
        path_params: ["\"meetingId\""],
        body_params: ["invitee"],
    },
    register_tool! {
        name: "jaxrs_meeting_participant_list",
        method: Get,
        path: "/jaxrs/meeting/{\"meetingId\"}/participant/list",
        desc: "List meeting participants",
        path_params: ["\"meetingId\""],
        body_params: [],
    },
    // ── mind ────────────────────────────────────────────────────────────────
    register_tool! {
        name: "jaxrs_mind_map_list",
        method: Get,
        path: "/jaxrs/mind/map/list",
        desc: "List mind maps",
        path_params: [],
        body_params: [],
    },
    register_tool! {
        name: "jaxrs_mind_map_create",
        method: Post,
        path: "/jaxrs/mind/map",
        desc: "Create a mind map",
        path_params: [],
        body_params: ["title", "content"],
    },
    // ── cms_express ─────────────────────────────────────────────────────────
    register_tool! {
        name: "jaxrs_cms_article_list",
        method: Get,
        path: "/jaxrs/cms/article/list",
        desc: "List CMS articles",
        path_params: [],
        body_params: [],
    },
    register_tool! {
        name: "jaxrs_cms_article_create",
        method: Post,
        path: "/jaxrs/cms/article",
        desc: "Create a CMS article",
        path_params: [],
        body_params: ["title", "content", "categoryId"],
    },
    // ── cms_assemble_control ────────────────────────────────────────────────
    register_tool! {
        name: "jaxrs_cms_category_list",
        method: Get,
        path: "/jaxrs/cms/category/list",
        desc: "List CMS categories",
        path_params: [],
        body_params: [],
    },
    // ── process_express ─────────────────────────────────────────────────────
    register_tool! {
        name: "jaxrs_process_instance_list",
        method: Get,
        path: "/jaxrs/process/instance/list",
        desc: "List process instances",
        path_params: [],
        body_params: [],
    },
    register_tool! {
        name: "jaxrs_process_instance_start",
        method: Post,
        path: "/jaxrs/process/instance/start",
        desc: "Start a new process instance",
        path_params: [],
        body_params: ["processId", "variables"],
    },
    // ── query_express ───────────────────────────────────────────────────────
    register_tool! {
        name: "jaxrs_query_search",
        method: Post,
        path: "/jaxrs/query/search",
        desc: "Execute a search query",
        path_params: [],
        body_params: ["query", "page", "size"],
    },
    // ── process_designer ────────────────────────────────────────────────────
    register_tool! {
        name: "jaxrs_process_design_list",
        method: Get,
        path: "/jaxrs/process/design/list",
        desc: "List process definitions",
        path_params: [],
        body_params: [],
    },
    // ── program_center ──────────────────────────────────────────────────────
    register_tool! {
        name: "jaxrs_program_list",
        method: Get,
        path: "/jaxrs/program/list",
        desc: "List programs",
        path_params: [],
        body_params: [],
    },
    register_tool! {
        name: "jaxrs_program_create",
        method: Post,
        path: "/jaxrs/program",
        desc: "Create a program",
        path_params: [],
        body_params: ["name", "description"],
    },
    // ── query_service ───────────────────────────────────────────────────────
    register_tool! {
        name: "jaxrs_query_service_query",
        method: Post,
        path: "/jaxrs/query/service",
        desc: "Execute a service query",
        path_params: [],
        body_params: ["sql", "params"],
    },
    // ── process_bam ─────────────────────────────────────────────────────────
    register_tool! {
        name: "jaxrs_process_bam_monitor",
        method: Get,
        path: "/jaxrs/process/bam/monitor",
        desc: "Get BAM monitoring data",
        path_params: [],
        body_params: [],
    },
    // ── process_surface ─────────────────────────────────────────────────────
    register_tool! {
        name: "jaxrs_process_surface_task_list",
        method: Get,
        path: "/jaxrs/process/surface/task/list",
        desc: "List surface tasks",
        path_params: [],
        body_params: [],
    },
    // ── file_assemble_control ───────────────────────────────────────────────
    register_tool! {
        name: "jaxrs_file_assemble_detail",
        method: Get,
        path: "/jaxrs/file/assemble/detail/{id}",
        desc: "Get assembled file detail",
        path_params: ["id"],
        body_params: [],
    },
    // ── ai_assemble_control ─────────────────────────────────────────────────
    register_tool! {
        name: "jaxrs_ai_assemble_config",
        method: Get,
        path: "/jaxrs/ai/assemble/config",
        desc: "Get AI assemble configuration",
        path_params: [],
        body_params: [],
    },
    // ── hotpic_assemble_control ─────────────────────────────────────────────
    register_tool! {
        name: "jaxrs_hotpic_assemble_list",
        method: Get,
        path: "/jaxrs/hotpic/assemble/list",
        desc: "List hotpic assemble items",
        path_params: [],
        body_params: [],
    },
    // ── organization_assemble_express ───────────────────────────────────────
    register_tool! {
        name: "jaxrs_organization_assemble_list",
        method: Get,
        path: "/jaxrs/organization/assemble/list",
        desc: "List organization assemble data",
        path_params: [],
        body_params: [],
    },
    register_tool! {
        name: "jaxrs_organization_assemble_person_list",
        method: Get,
        path: "/jaxrs/organization/assemble/person/list",
        desc: "List persons in organization",
        path_params: [],
        body_params: [],
    },
    // ── organization_assemble_control ───────────────────────────────────────
    register_tool! {
        name: "jaxrs_organization_assemble_unit_tree",
        method: Get,
        path: "/jaxrs/organization/assemble/unit/tree",
        desc: "Get organization unit tree",
        path_params: [],
        body_params: [],
    },
    // ── mind_assemble_control ───────────────────────────────────────────────
    register_tool! {
        name: "jaxrs_mind_assemble_list",
        method: Get,
        path: "/jaxrs/mind/assemble/list",
        desc: "List assembled mind maps",
        path_params: [],
        body_params: [],
    },
    // ── attendance_assemble_control ─────────────────────────────────────────
    register_tool! {
        name: "jaxrs_attendance_assemble_daily",
        method: Get,
        path: "/jaxrs/attendance/assemble/daily",
        desc: "Get daily attendance assemble data",
        path_params: [],
        body_params: ["date"],
    },
    // ── general_assemble_control ────────────────────────────────────────────
    register_tool! {
        name: "jaxrs_general_assemble_config",
        method: Get,
        path: "/jaxrs/general/assemble/config",
        desc: "Get assembled general config",
        path_params: [],
        body_params: [],
    },
    // ── meeting_assemble_control ────────────────────────────────────────────
    register_tool! {
        name: "jaxrs_meeting_assemble_summary",
        method: Get,
        path: "/jaxrs/meeting/assemble/summary",
        desc: "Get assembled meeting summary",
        path_params: [],
        body_params: ["\"meetingId\""],
    },
    // ── message_assemble_communicate ────────────────────────────────────────
    register_tool! {
        name: "jaxrs_message_assemble_send",
        method: Post,
        path: "/jaxrs/message/assemble/send",
        desc: "Send assembled message",
        path_params: [],
        body_params: ["to", "content", "templateId"],
    },
    // ── portal_assemble_designer ────────────────────────────────────────────
    register_tool! {
        name: "jaxrs_portal_assemble_page_list",
        method: Get,
        path: "/jaxrs/portal/assemble/page/list",
        desc: "List assembled portal pages",
        path_params: [],
        body_params: [],
    },
    // ── correlation_service_processing ──────────────────────────────────────
    register_tool! {
        name: "jaxrs_correlation_service_process",
        method: Post,
        path: "/jaxrs/correlation/service/process",
        desc: "Process correlation service data",
        path_params: [],
        body_params: ["sourceId", "targetId", "relationType"],
    },
    // ── portal_assemble_surface ─────────────────────────────────────────────
    register_tool! {
        name: "jaxrs_portal_assemble_surface_widgets",
        method: Get,
        path: "/jaxrs/portal/assemble/surface/widgets",
        desc: "List portal surface widgets",
        path_params: [],
        body_params: [],
    },
    // ── processplatform_service_processing ──────────────────────────────────
    register_tool! {
        name: "jaxrs_processplatform_service_task_list",
        method: Get,
        path: "/jaxrs/processplatform/service/task/list",
        desc: "List process platform tasks",
        path_params: [],
        body_params: [],
    },
    // ── bbs_assemble_control ────────────────────────────────────────────────
    register_tool! {
        name: "jaxrs_bbs_assemble_topic_list",
        method: Get,
        path: "/jaxrs/bbs/assemble/topic/list",
        desc: "List assembled BBS topics",
        path_params: [],
        body_params: [],
    },
    // ── calendar_assemble_control ───────────────────────────────────────────
    register_tool! {
        name: "jaxrs_calendar_assemble_event_list",
        method: Get,
        path: "/jaxrs/calendar/assemble/event/list",
        desc: "List assembled calendar events",
        path_params: [],
        body_params: [],
    },
    // ── component_assemble_control ──────────────────────────────────────────
    register_tool! {
        name: "jaxrs_component_assemble_list",
        method: Get,
        path: "/jaxrs/component/assemble/list",
        desc: "List assembled components",
        path_params: [],
        body_params: [],
    },
    // ── jpush_assemble_control ──────────────────────────────────────────────
    register_tool! {
        name: "jaxrs_jpush_assemble_send",
        method: Post,
        path: "/jaxrs/jpush/assemble/send",
        desc: "Send push via assemble control",
        path_params: [],
        body_params: ["alias", "alert"],
    },
    // ── processplatform_core_entity ─────────────────────────────────────────
    register_tool! {
        name: "jaxrs_processplatform_core_definition_list",
        method: Get,
        path: "/jaxrs/processplatform/core/definition/list",
        desc: "List process platform core definitions",
        path_params: [],
        body_params: [],
    },
    // ── portal_core_entity ──────────────────────────────────────────────────
    register_tool! {
        name: "jaxrs_portal_core_page_list",
        method: Get,
        path: "/jaxrs/portal/core/page/list",
        desc: "List portal core pages",
        path_params: [],
        body_params: [],
    },
    // ── program_center_core_entity ──────────────────────────────────────────
    register_tool! {
        name: "jaxrs_program_center_core_list",
        method: Get,
        path: "/jaxrs/program_center/core/list",
        desc: "List program center core items",
        path_params: [],
        body_params: [],
    },
    // ── processplatform_core_express ────────────────────────────────────────
    register_tool! {
        name: "jaxrs_processplatform_core_express_task_list",
        method: Get,
        path: "/jaxrs/processplatform/core/express/task/list",
        desc: "List process platform express tasks",
        path_params: [],
        body_params: [],
    },
    // ── query_core_entity ───────────────────────────────────────────────────
    register_tool! {
        name: "jaxrs_query_core_list",
        method: Get,
        path: "/jaxrs/query/core/list",
        desc: "List query core entities",
        path_params: [],
        body_params: [],
    },
    // ── general_core_entity ─────────────────────────────────────────────────
    register_tool! {
        name: "jaxrs_general_core_list",
        method: Get,
        path: "/jaxrs/general/core/list",
        desc: "List general core entities",
        path_params: [],
        body_params: [],
    },
    // ── organization_core_entity ────────────────────────────────────────────
    register_tool! {
        name: "jaxrs_organization_core_unit_list",
        method: Get,
        path: "/jaxrs/organization/core/unit/list",
        desc: "List organization core units",
        path_params: [],
        body_params: [],
    },
    // ── cms_core_entity ─────────────────────────────────────────────────────
    register_tool! {
        name: "jaxrs_cms_core_article_list",
        method: Get,
        path: "/jaxrs/cms/core/article/list",
        desc: "List CMS core articles",
        path_params: [],
        body_params: [],
    },
    // ── query_assemble_designer ─────────────────────────────────────────────
    register_tool! {
        name: "jaxrs_query_assemble_designer_list",
        method: Get,
        path: "/jaxrs/query/assemble/designer/list",
        desc: "List query assemble designer items",
        path_params: [],
        body_params: [],
    },
    // ── query_assemble_surface ──────────────────────────────────────────────
    register_tool! {
        name: "jaxrs_query_assemble_surface_list",
        method: Get,
        path: "/jaxrs/query/assemble/surface/list",
        desc: "List query assemble surface items",
        path_params: [],
        body_params: [],
    },
    // ── console ─────────────────────────────────────────────────────────────
    register_tool! {
        name: "jaxrs_console_status",
        method: Get,
        path: "/jaxrs/console/status",
        desc: "Return console status",
        path_params: [],
        body_params: [],
    },
    // ── processplatform_assemble_surface ────────────────────────────────────
    register_tool! {
        name: "jaxrs_processplatform_assemble_surface_task_list",
        method: Get,
        path: "/jaxrs/processplatform/assemble/surface/task/list",
        desc: "List processplatform assemble surface tasks",
        path_params: [],
        body_params: [],
    },
    // ── bbs_core_entity ─────────────────────────────────────────────────────
    register_tool! {
        name: "jaxrs_bbs_core_topic_list",
        method: Get,
        path: "/jaxrs/bbs/core/topic/list",
        desc: "List BBS core topics",
        path_params: [],
        body_params: [],
    },
    // ── calendar_core_entity ────────────────────────────────────────────────
    register_tool! {
        name: "jaxrs_calendar_core_event_list",
        method: Get,
        path: "/jaxrs/calendar/core/event/list",
        desc: "List calendar core events",
        path_params: [],
        body_params: [],
    },
    // ── component_core_entity ───────────────────────────────────────────────
    register_tool! {
        name: "jaxrs_component_core_list",
        method: Get,
        path: "/jaxrs/component/core/list",
        desc: "List component core entities",
        path_params: [],
        body_params: [],
    },
    // ── file_core_entity ────────────────────────────────────────────────────
    register_tool! {
        name: "jaxrs_file_core_list",
        method: Get,
        path: "/jaxrs/file/core/list",
        desc: "List file core entities",
        path_params: [],
        body_params: [],
    },
    // ── ai_core_entity ──────────────────────────────────────────────────────
    register_tool! {
        name: "jaxrs_ai_core_list",
        method: Get,
        path: "/jaxrs/ai/core/list",
        desc: "List AI core entities",
        path_params: [],
        body_params: [],
    },
    // ── attendance_core_entity ──────────────────────────────────────────────
    register_tool! {
        name: "jaxrs_attendance_core_list",
        method: Get,
        path: "/jaxrs/attendance/core/list",
        desc: "List attendance core records",
        path_params: [],
        body_params: [],
    },
    // ── cms_core_express ────────────────────────────────────────────────────
    register_tool! {
        name: "jaxrs_cms_core_express_article_list",
        method: Get,
        path: "/jaxrs/cms/core/express/article/list",
        desc: "List CMS core express articles",
        path_params: [],
        body_params: [],
    },
    // ── correlation_core_entity ─────────────────────────────────────────────
    register_tool! {
        name: "jaxrs_correlation_core_link_list",
        method: Get,
        path: "/jaxrs/correlation/core/link/list",
        desc: "List correlation core links",
        path_params: [],
        body_params: [],
    },
    // ── correlation_core_express ────────────────────────────────────────────
    register_tool! {
        name: "jaxrs_correlation_core_express_list",
        method: Get,
        path: "/jaxrs/correlation/core/express/list",
        desc: "List correlation core express items",
        path_params: [],
        body_params: [],
    },
    // ── hotpic_core_entity ──────────────────────────────────────────────────
    register_tool! {
        name: "jaxrs_hotpic_core_list",
        method: Get,
        path: "/jaxrs/hotpic/core/list",
        desc: "List hotpic core entities",
        path_params: [],
        body_params: [],
    },
    // ── jpush_core_entity ───────────────────────────────────────────────────
    register_tool! {
        name: "jaxrs_jpush_core_list",
        method: Get,
        path: "/jaxrs/jpush/core/list",
        desc: "List JPush core entities",
        path_params: [],
        body_params: [],
    },
    // ── meeting_core_entity ─────────────────────────────────────────────────
    register_tool! {
        name: "jaxrs_meeting_core_list",
        method: Get,
        path: "/jaxrs/meeting/core/list",
        desc: "List meeting core entities",
        path_params: [],
        body_params: [],
    },
    // ── message_core_entity ─────────────────────────────────────────────────
    register_tool! {
        name: "jaxrs_message_core_list",
        method: Get,
        path: "/jaxrs/message/core/list",
        desc: "List message core entities",
        path_params: [],
        body_params: [],
    },
    // ── mind_core_entity ────────────────────────────────────────────────────
    register_tool! {
        name: "jaxrs_mind_core_list",
        method: Get,
        path: "/jaxrs/mind/core/list",
        desc: "List mind core entities",
        path_params: [],
        body_params: [],
    },
    // ── organization_core_express ───────────────────────────────────────────
    register_tool! {
        name: "jaxrs_organization_core_express_list",
        method: Get,
        path: "/jaxrs/organization/core/express/list",
        desc: "List organization core express items",
        path_params: [],
        body_params: [],
    },
    // ── processplatform_assemble_bam ────────────────────────────────────────
    register_tool! {
        name: "jaxrs_processplatform_assemble_bam_list",
        method: Get,
        path: "/jaxrs/processplatform/assemble/bam/list",
        desc: "List processplatform BAM assemble items",
        path_params: [],
        body_params: [],
    },
    // ── processplatform_assemble_designer ───────────────────────────────────
    register_tool! {
        name: "jaxrs_processplatform_assemble_designer_list",
        method: Get,
        path: "/jaxrs/processplatform/assemble/designer/list",
        desc: "List processplatform assemble designer items",
        path_params: [],
        body_params: [],
    },
    // ── query_core_express ──────────────────────────────────────────────────
    register_tool! {
        name: "jaxrs_query_core_express_list",
        method: Get,
        path: "/jaxrs/query/core/express/list",
        desc: "List query core express items",
        path_params: [],
        body_params: [],
    },
    // ── query_service_processing ────────────────────────────────────────────
    register_tool! {
        name: "jaxrs_query_service_process",
        method: Post,
        path: "/jaxrs/query/service/process",
        desc: "Process a query service request",
        path_params: [],
        body_params: ["query", "params"],
    },
    // ── reset endpoints ─────────────────────────────────────────────────────
    register_tool! {
        name: "jaxrs_reset_check_credential",
        method: Get,
        path: "/jaxrs/reset/check/credential/{credential}",
        desc: "Check if credential is eligible for password reset",
        path_params: ["credential"],
        body_params: [],
    },
    register_tool! {
        name: "jaxrs_reset_check_password",
        method: Get,
        path: "/jaxrs/reset/check/password/{password}",
        desc: "Check password reset eligibility",
        path_params: ["password"],
        body_params: [],
    },
    register_tool! {
        name: "jaxrs_reset_code_credential",
        method: Post,
        path: "/jaxrs/reset/code/credential/{credential}",
        desc: "Request password reset code for credential",
        path_params: ["credential"],
        body_params: [],
    },
    register_tool! {
        name: "jaxrs_reset_code",
        method: Post,
        path: "/jaxrs/reset/code",
        desc: "Request password reset code",
        path_params: [],
        body_params: ["credential"],
    },
    register_tool! {
        name: "jaxrs_reset_check",
        method: Post,
        path: "/jaxrs/reset/check",
        desc: "Check reset code validity",
        path_params: [],
        body_params: ["credential", "code"],
    },
    register_tool! {
        name: "jaxrs_reset_set",
        method: Post,
        path: "/jaxrs/reset/set",
        desc: "Set new password with reset code",
        path_params: [],
        body_params: ["credential", "code", "newPassword"],
    },
    // ── auth extended endpoints ─────────────────────────────────────────────
    register_tool! {
        name: "jaxrs_authentication_two_factor",
        method: Post,
        path: "/jaxrs/authentication/two_factor",
        desc: "Two-factor authentication login",
        path_params: [],
        body_params: ["credential", "code"],
    },
    register_tool! {
        name: "jaxrs_authentication_safe_logout",
        method: Post,
        path: "/jaxrs/authentication/safe/logout",
        desc: "Securely logout and revoke all sessions",
        path_params: [],
        body_params: [],
    },
    register_tool! {
        name: "jaxrs_authentication_check_token",
        method: Post,
        path: "/jaxrs/authentication/check/token",
        desc: "Check token validity",
        path_params: [],
        body_params: ["token"],
    },
    register_tool! {
        name: "jaxrs_authentication_sso_encrypt",
        method: Post,
        path: "/jaxrs/authentication/sso/encrypt",
        desc: "Encrypt SSO token for single sign-on",
        path_params: [],
        body_params: ["client", "key", "credential"],
    },
    register_tool! {
        name: "jaxrs_authentication_sso",
        method: Post,
        path: "/jaxrs/authentication/sso",
        desc: "SSO login via token",
        path_params: [],
        body_params: ["client", "token"],
    },
    register_tool! {
        name: "jaxrs_authentication_switchuser",
        method: Post,
        path: "/jaxrs/authentication/switchuser",
        desc: "Admin switch user to another account",
        path_params: [],
        body_params: ["targetCredential"],
    },
];

// 合并静态路由与自动生成路由
include!("generated_routes.rs");

fn all_route_defs() -> Vec<RouteDef> {
    let mut all = ROUTE_DEFS.to_vec();
    all.extend_from_slice(&GENERATED_ROUTE_DEFS);
    all
}

// ──────────────────────────────────────────────────────────────────────────────
// ToolBridge: owns the internal axum app and dispatches tool calls.
// ──────────────────────────────────────────────────────────────────────────────

pub struct ToolBridge {
    app: Router,
    route_map: HashMap<String, RouteDef>,
}

impl ToolBridge {
    /// Build a new ToolBridge from a database pool and session manager.
    /// The internal router carries the same middleware chain as the main app
    /// minus auth (the MCP layer injects sessions directly).
    pub async fn new(pool: Pool, session_manager: SessionManager) -> Self {
        let security_state = SecurityState {
            session_manager,
            rate_limiter: shared::rate_limit::RateLimiter::new(),
            pool: pool.clone(),
        };

        let app = Router::new()
            .merge(shared::router::router())
            .merge(auth::router(
                pool.clone(),
                shared::rate_limit::RateLimiter::new(),
                security_state.session_manager.clone(),
            ))
            .merge(personal::router(pool.clone(), security_state.session_manager.clone()))
            .merge(cms_control::cms_control_router(pool.clone()))
            .merge(control::control_router(pool.clone()))
            .merge(personal_extend::personal_extend_router(
                pool.clone(),
                security_state.session_manager.clone(),
            ))
            .merge(program_init::program_init_router(pool.clone()))
            .merge(express::router(pool.clone()))
            .merge(message::router(pool.clone()))
            .merge(portal::router(pool.clone()))
            .merge(bbs::router(pool.clone()))
            .merge(calendar::router(pool.clone()))
            .merge(component::router(pool.clone()))
            .merge(file::router(pool.clone()))
            .merge(ai::router(pool.clone()))
            .merge(attendance::router(pool.clone()))
            .merge(correlation::router(pool.clone()))
            .merge(general::router(pool.clone()))
            .merge(hotpic::router(pool.clone()))
            .merge(jpush::router(pool.clone()))
            .merge(meeting::router(pool.clone()))
            .merge(mind::router(pool.clone()))
            .merge(cms_express::router(pool.clone()))
            .merge(cms_assemble_control::router(pool.clone()))
            .merge(process_express::router(pool.clone()))
            .merge(query_express::router(pool.clone()))
            .merge(process_designer::router(pool.clone()))
            .merge(program_center::router(pool.clone()))
            .merge(base::router(pool.clone()))
            .merge(query_service::router(pool.clone()))
            .merge(process_bam::router(pool.clone()))
            .merge(process_surface::router(pool.clone()))
            .merge(file_assemble_control::router(pool.clone()))
            .merge(ai_assemble_control::router(pool.clone()))
            .merge(hotpic_assemble_control::router(pool.clone()))
            .merge(organization_assemble_express::router(pool.clone()))
            .merge(organization_assemble_control::router(pool.clone()))
            .merge(mind_assemble_control::router(pool.clone()))
            .merge(attendance_assemble_control::router(pool.clone()))
            .merge(general_assemble_control::router(pool.clone()))
            .merge(meeting_assemble_control::router(pool.clone()))
            .merge(message_assemble_communicate::router(pool.clone()))
            .merge(portal_assemble_designer::router(pool.clone()))
            .merge(correlation_service_processing::router(pool.clone()))
            .merge(portal_assemble_surface::router(pool.clone()))
            .merge(processplatform_service_processing::router(pool.clone()))
            .merge(bbs_assemble_control::router(pool.clone()))
            .merge(calendar_assemble_control::router(pool.clone()))
            .merge(component_assemble_control::router(pool.clone()))
            .merge(jpush_assemble_control::router(pool.clone()))
            .merge(processplatform_core_entity::router(pool.clone()))
            .merge(portal_core_entity::router(pool.clone()))
            .merge(program_center_core_entity::router(pool.clone()).await)
            .merge(processplatform_core_express::router(pool.clone()))
            .merge(query_core_entity::router(pool.clone()))
            .merge(general_core_entity::router(pool.clone()))
            .merge(organization_core_entity::router(pool.clone()))
            .merge(cms_core_entity::router(pool.clone()))
            .merge(query_assemble_designer::router(pool.clone()))
            .merge(query_assemble_surface::router(pool.clone()))
            .merge(console::router(pool.clone()))
            .merge(processplatform_assemble_surface::router(pool.clone()))
            .merge(bbs_core_entity::router(pool.clone()))
            .merge(calendar_core_entity::router(pool.clone()))
            .merge(component_core_entity::router(pool.clone()))
            .merge(file_core_entity::router(pool.clone()))
            .merge(ai_core_entity::router(pool.clone()))
            .merge(attendance_core_entity::router(pool.clone()))
            .merge(cms_core_express::router(pool.clone()))
            .merge(correlation_core_entity::router(pool.clone()))
            .merge(correlation_core_express::router(pool.clone()))
            .merge(hotpic_core_entity::router(pool.clone()))
            .merge(jpush_core_entity::router(pool.clone()))
            .merge(meeting_core_entity::router(pool.clone()))
            .merge(message_core_entity::router(pool.clone()))
            .merge(mind_core_entity::router(pool.clone()))
            .merge(organization_core_express::router(pool.clone()))
            .merge(processplatform_assemble_bam::router(pool.clone()))
            .merge(processplatform_assemble_designer::router(pool.clone()))
            .merge(query_core_express::router(pool.clone()))
            .merge(query_service_processing::router(pool.clone()))
            // Minimal middleware: CORS + security headers + trace only.
            // Auth is injected per-request by callers via request extensions.
            .layer(cors_middleware())
            .layer(axum::middleware::from_fn(security_headers_middleware))
            .layer(axum::middleware::from_fn(trace_middleware))
            .layer(axum::Extension(pool))
            .layer(axum::Extension(security_state.session_manager));

        let all_defs = all_route_defs();
        let route_map: HashMap<String, RouteDef> = all_defs
            .iter()
            .map(|r| (r.tool_name.to_string(), r.clone()))
            .collect();

        Self { app, route_map }
    }

    /// Return the full tool catalog as MCP ListToolsResult.
    pub fn list_tools(&self) -> ListToolsResponse {
        let all_defs = all_route_defs();
        let tools: Vec<McpTool> = all_defs
            .iter()
            .map(|def| {
                let mut properties = HashMap::new();
                for &p in def.path_params {
                    properties.insert(
                        p,
                        ToolProperty {
                            typ: "string",
                            description: Some("URL path parameter"),
                        },
                    );
                }
                for &p in def.body_params {
                    properties.insert(
                        p,
                        ToolProperty {
                            typ: "string",
                            description: Some("Request body parameter"),
                        },
                    );
                }
                let required: Vec<&str> = def.path_params.iter().copied().collect();

                McpTool {
                    name: def.tool_name.to_string(),
                    description: def.description.to_string(),
                    input_schema: ToolInputSchema {
                        typ: "object",
                        properties,
                        required: if required.is_empty() {
                            None
                        } else {
                            Some(required)
                        },
                    },
                }
            })
            .collect();

        ListToolsResponse { tools }
    }

    /// Execute a tool call. Returns the raw axum response body as a text block.
    pub async fn call_tool(
        &self,
        params: ToolCallParams,
    ) -> Result<CallToolResponse, McpError> {
        let name = &params.name;
        let args = params.arguments;

        let def = self
            .route_map
            .get(name.as_str())
            .ok_or_else(|| McpError::invalid_request(format!("unknown tool: {}", name)))?;

        let mut path = def.path.to_string();

        // Substitute path parameters from args into the URL.
        let mut body_args = serde_json::Map::new();
        for (k, v) in args {
            if def.path_params.contains(&k.as_str()) {
                let encoded: String = urlencoding::encode(&v.to_string()).into();
                path = path.replace(&format!("{{{}}}", k), &encoded);
            } else {
                body_args.insert(k, v);
            }
        }

        // Build the axum Request.
        let method = def.method.as_axum();
        let body = if def.method == HttpMethod::Get {
            if !body_args.is_empty() {
                let qs: Vec<String> = body_args
                    .iter()
                    .map(|(k, v)| format!("{}={}", urlencoding::encode(k), urlencoding::encode(&v.to_string())))
                    .collect();
                path = if path.contains('?') {
                        format!("{}&{}", path, qs.join("&"))
                    } else {
                        format!("{}?{}", path, qs.join("&"))
                    };
            }
            Body::empty()
        } else {
            let json = serde_json::to_vec(&serde_json::Value::Object(body_args))
                .map_err(|e| McpError::invalid_request(e.to_string()))?;
            Body::from(json)
        };

        let mut req_builder = Request::builder()
            .method(method)
            .uri(path)
            .header("content-type", "application/json");

        let mut req = req_builder
            .body(body)
            .map_err(|e| McpError::invalid_request(e.to_string()))?;

        let response = self
            .app
            .clone()
            .oneshot(req)
            .await
                .map_err(|e| McpError::internal(e.to_string()))?;

        let status = response.status();
        let bytes =
            axum::body::to_bytes(response.into_body(), 4096)
                .await
            .map_err(|e| McpError::internal(e.to_string()))?;

        let body_text = String::from_utf8_lossy(&bytes).to_string();

        let is_error = !status.is_success();
        Ok(CallToolResponse {
            content: vec![ToolContent::Text {
                text: format!("HTTP {}:\n{}", status.as_u16(), body_text),
            }],
            is_error: Some(is_error),
        })
    }
}

// ──────────────────────────────────────────────────────────────────────────────
// Error type
// ──────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpError {
    pub code: i32,
    pub message: String,
}

impl McpError {
    pub fn invalid_request(msg: impl Into<String>) -> Self {
        Self {
            code: -32600,
            message: msg.into(),
        }
    }
    pub fn not_found(msg: impl Into<String>) -> Self {
        Self {
            code: -32601,
            message: msg.into(),
        }
    }
    pub fn internal(msg: impl Into<String>) -> Self {
        Self {
            code: -32603,
            message: msg.into(),
        }
    }
}

// ──────────────────────────────────────────────────────────────────────────────
// JSON-RPC 2.0 helpers (minimal MCP over stdio)
// ──────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JsonRpcRequest {
    jsonrpc: String,
    id: Option<Value>,
    method: String,
    #[serde(default)]
    params: Option<Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JsonRpcResponse {
    jsonrpc: &'static str,
    id: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    result: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<JsonRpcError>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct JsonRpcError {
    code: i32,
    message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<Value>,
}

impl JsonRpcResponse {
    pub fn ok(id: Option<Value>, result: impl Serialize) -> Self {
        Self {
            jsonrpc: "2.0",
            id,
            result: Some(serde_json::to_value(result).unwrap_or(serde_json::json!({}))),
            error: None,
        }
    }

    pub fn err(id: Option<Value>, code: i32, message: impl Into<String>) -> Self {
        Self {
            jsonrpc: "2.0",
            id,
            result: None,
            error: Some(JsonRpcError {
                code,
                message: message.into(),
                data: None,
            }),
        }
    }

    pub fn into_json_value(self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or(serde_json::json!({}))
    }
}

// ──────────────────────────────────────────────────────────────────────────────
// stdio transport: read line-delimited JSON-RPC from stdin, write to stdout.
// ──────────────────────────────────────────────────────────────────────────────

pub async fn run_stdio(bridge: Arc<ToolBridge>) -> anyhow::Result<()> {
    use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};

    let stdin = tokio::io::stdin();
    let mut stdout = tokio::io::stdout();
    let mut lines = BufReader::new(stdin).lines();

    // Send initialize response immediately.
    let init_notify = serde_json::json!({
        "jsonrpc": "2.0",
        "id": 1,
        "result": {
            "protocolVersion": "2024-11-05",
            "capabilities": {
                "tools": {"listChanged": false}
            },
            "serverInfo": {"name": "oa4rust-mcp", "version": "0.1.0"}
        }
    });
    let init_line = format!("{}\n", serde_json::to_string(&init_notify)?);
    stdout.write_all(init_line.as_bytes()).await?;
    stdout.flush().await?;

    while let Some(line) = lines.next_line().await? {
        if line.trim().is_empty() {
            continue;
        }
        let req: JsonRpcRequest = match serde_json::from_str(&line) {
            Ok(r) => r,
            Err(_) => {
                let err_resp = JsonRpcResponse::err(None, -32700, "Parse error");
                let err_line = format!("{}\n", serde_json::to_string(&err_resp)?);
                stdout.write_all(err_line.as_bytes()).await?;
                stdout.flush().await?;
                continue;
            }
        };

        let id = req.id.clone();
        let result = handle_json_rpc(&bridge, req).await;
        let resp = match result {
            Ok(r) => JsonRpcResponse::ok(id, r),
            Err(e) => JsonRpcResponse::err(id, e.code, e.message),
        };

        let resp_line = format!("{}\n", serde_json::to_string(&resp)?);
        stdout.write_all(resp_line.as_bytes()).await?;
        stdout.flush().await?;
    }

    Ok(())
}

async fn handle_json_rpc(
    bridge: &Arc<ToolBridge>,
    req: JsonRpcRequest,
) -> Result<Value, McpError> {
    match req.method.as_str() {
        "initialize" => {
            Ok(serde_json::json!({
                "protocolVersion": "2024-11-05",
                "capabilities": {"tools": {"listChanged": false}},
                "serverInfo": {"name": "oa4rust-mcp", "version": "0.1.0"}
            }))
        }
        "tools/list" => {
            Ok(serde_json::to_value(bridge.list_tools()).map_err(|e| McpError::internal(e.to_string()))?)
        }
        "tools/call" => {
            let params = req
                .params
                .ok_or_else(|| McpError::invalid_request("missing params"))?
                .clone();
            let params: ToolCallParams =
                serde_json::from_value(params).map_err(|e| McpError::invalid_request(e.to_string()))?;
            let resp = bridge.call_tool(params).await?;
            Ok(serde_json::to_value(resp).map_err(|e| McpError::internal(e.to_string()))?)
        }
        "shutdown" | "exit" => {
            std::process::exit(0);
        }
        _ => Err(McpError::not_found(format!(
            "method not found: {}",
            req.method
        ))),
    }
}
