//! OA4Rust 行为对比测试套件 (U3 / R3)
//!
//! 对比相同请求下 Rust 端点（localhost:3000）与 Java 端点（JAVA_SERVICE_URL）的
//! 响应结构（状态码 + JSON 字段名 + 类型），忽略允许列表中的已知命名差异。
//!
//! 运行方式：
//!   JAVA_SERVICE_URL=http://localhost:8080 cargo test --test behavior_compare
//!
//! 如果 Java 服务不可达，所有 Java 侧对比标记为 SKIP，测试仍通过。
//! 如果 Rust 服务不可达，测试失败。

mod behavior_comparison;

use std::collections::HashMap;

use behavior_comparison::{ComparisonStatus, EndpointComparator, EndpointDef};

/// Rust 服务地址（CI 中通过 cargo test 启动，监听 3000 端口）。
const RUST_BASE_URL: &str = "http://localhost:3000";

/// 默认 Java 服务地址。
const DEFAULT_JAVA_BASE_URL: &str = "http://localhost:8080";

/// 默认登录凭证（行为对比专用测试账户，需两侧数据库均有此账户）。
const DEFAULT_CREDENTIAL: &str = "testadmin";
const DEFAULT_PASSWORD: &str = "testadmin";

/// 允许列表文件路径（相对于 tests/ 目录）。
const ALLOWLIST_PATH: &str = "tests/behavior_comparison/allowlist.yaml";

/// 报告输出路径。
const REPORT_PATH: &str = "target/debug/behavior-report.md";

// ──────────────────────────────────────────────────────────────────────────────
// 端点定义
// ──────────────────────────────────────────────────────────────────────────────

const ENDPOINTS: &[EndpointDef] = &[
    // ── base（_base_core_project）─────────────────────────────────────────────
    EndpointDef {
        crate_name: "base",
        method: "GET",
        rust_path: "/jaxrs/base/echo/get",
        java_war: "x_base_core_project",
        java_action: "jaxrs/echo/get",
        body: None,
        requires_auth: false,
    },
    EndpointDef {
        crate_name: "base",
        method: "GET",
        rust_path: "/jaxrs/base/cache/detail",
        java_war: "x_base_core_project",
        java_action: "jaxrs/cache/detail",
        body: None,
        requires_auth: false,
    },
    EndpointDef {
        crate_name: "base",
        method: "GET",
        rust_path: "/jaxrs/base/openapi/info",
        java_war: "x_base_core_project",
        java_action: "jaxrs/openapi/info",
        body: None,
        requires_auth: false,
    },
    // ── auth（x_organization_assemble_authentication）──────────────────────────
    // Note: login endpoint body moved to a const-like pattern using lazy_static.
    EndpointDef {
        crate_name: "auth",
        method: "POST",
        rust_path: "/jaxrs/authentication/login",
        java_war: "x_organization_assemble_authentication",
        java_action: "jaxrs/authentication/login",
        body: None,
        requires_auth: false,
    },
    EndpointDef {
        crate_name: "auth",
        method: "GET",
        rust_path: "/jaxrs/authentication/mode",
        java_war: "x_organization_assemble_authentication",
        java_action: "jaxrs/authentication/mode",
        body: None,
        requires_auth: false,
    },
    // ── control（x_organization_assemble_control）─────────────────────────────
    EndpointDef {
        crate_name: "control",
        method: "GET",
        rust_path: "/jaxrs/control/person/list",
        java_war: "x_organization_assemble_control",
        java_action: "jaxrs/person/list",
        body: None,
        requires_auth: true,
    },
    EndpointDef {
        crate_name: "control",
        method: "GET",
        rust_path: "/jaxrs/control/unit/list",
        java_war: "x_organization_assemble_control",
        java_action: "jaxrs/unit/list",
        body: None,
        requires_auth: true,
    },
    EndpointDef {
        crate_name: "control",
        method: "GET",
        rust_path: "/jaxrs/control/role/list",
        java_war: "x_organization_assemble_control",
        java_action: "jaxrs/role/list",
        body: None,
        requires_auth: true,
    },
    EndpointDef {
        crate_name: "control",
        method: "GET",
        rust_path: "/jaxrs/control/group/list",
        java_war: "x_organization_assemble_control",
        java_action: "jaxrs/group/list",
        body: None,
        requires_auth: true,
    },
    // ── personal（x_personal_extend）─────────────────────────────────────────
    EndpointDef {
        crate_name: "personal",
        method: "GET",
        rust_path: "/jaxrs/person",
        java_war: "x_organization_assemble_personal",
        java_action: "jaxrs/person",
        body: None,
        requires_auth: true,
    },
    EndpointDef {
        crate_name: "personal",
        method: "GET",
        rust_path: "/jaxrs/personal/person",
        java_war: "x_personal_extend",
        java_action: "jaxrs/person",
        body: None,
        requires_auth: true,
    },
    // ── program_init ────────────────────────────────────────────────────────
    EndpointDef {
        crate_name: "program_init",
        method: "GET",
        rust_path: "/jaxrs/secret/check",
        java_war: "x_program_init",
        java_action: "jaxrs/secret/check",
        body: None,
        requires_auth: false,
    },
    // ── express（x_organization_assemble_express）────────────────────────────
    EndpointDef {
        crate_name: "express",
        method: "GET",
        rust_path: "/jaxrs/express/person/list",
        java_war: "x_organization_assemble_express",
        java_action: "jaxrs/person/list",
        body: None,
        requires_auth: true,
    },
    EndpointDef {
        crate_name: "express",
        method: "GET",
        rust_path: "/jaxrs/express/unit/list",
        java_war: "x_organization_assemble_express",
        java_action: "jaxrs/unit/list",
        body: None,
        requires_auth: true,
    },
    // ── message ────────────────────────────────────────────────────────────
    EndpointDef {
        crate_name: "message",
        method: "GET",
        rust_path: "/jaxrs/message/session/list",
        java_war: "x_message_core_entity",
        java_action: "jaxrs/session/list",
        body: None,
        requires_auth: true,
    },
    EndpointDef {
        crate_name: "message",
        method: "GET",
        rust_path: "/jaxrs/message/chat/list",
        java_war: "x_message_assemble_communicate",
        java_action: "jaxrs/chat/list",
        body: None,
        requires_auth: true,
    },
    // ── portal ─────────────────────────────────────────────────────────────
    EndpointDef {
        crate_name: "portal",
        method: "GET",
        rust_path: "/jaxrs/portal/page/list",
        java_war: "x_portal_core_entity",
        java_action: "jaxrs/page/list",
        body: None,
        requires_auth: true,
    },
    EndpointDef {
        crate_name: "portal",
        method: "GET",
        rust_path: "/jaxrs/portal/designer/list",
        java_war: "x_portal_assemble_designer",
        java_action: "jaxrs/designer/list",
        body: None,
        requires_auth: true,
    },
    // ── bbs ────────────────────────────────────────────────────────────────
    EndpointDef {
        crate_name: "bbs",
        method: "GET",
        rust_path: "/jaxrs/bbs/forum/list",
        java_war: "x_bbs_core_entity",
        java_action: "jaxrs/forum/list",
        body: None,
        requires_auth: true,
    },
    EndpointDef {
        crate_name: "bbs",
        method: "GET",
        rust_path: "/jaxrs/bbs/post/list",
        java_war: "x_bbs_assemble_control",
        java_action: "jaxrs/post/list",
        body: None,
        requires_auth: true,
    },
    // ── calendar ───────────────────────────────────────────────────────────
    EndpointDef {
        crate_name: "calendar",
        method: "GET",
        rust_path: "/jaxrs/calendar/event/list",
        java_war: "x_calendar_core_entity",
        java_action: "jaxrs/event/list",
        body: None,
        requires_auth: true,
    },
    // ── component ──────────────────────────────────────────────────────────
    EndpointDef {
        crate_name: "component",
        method: "GET",
        rust_path: "/jaxrs/component/category/list",
        java_war: "x_component_core_entity",
        java_action: "jaxrs/category/list",
        body: None,
        requires_auth: true,
    },
    // ── file ───────────────────────────────────────────────────────────────
    EndpointDef {
        crate_name: "file",
        method: "GET",
        rust_path: "/jaxrs/file/list",
        java_war: "x_file_core_entity",
        java_action: "jaxrs/list",
        body: None,
        requires_auth: true,
    },
    // ── ai ─────────────────────────────────────────────────────────────────
    EndpointDef {
        crate_name: "ai",
        method: "GET",
        rust_path: "/jaxrs/ai/config/list",
        java_war: "x_ai_core_entity",
        java_action: "jaxrs/config/list",
        body: None,
        requires_auth: true,
    },
    EndpointDef {
        crate_name: "ai",
        method: "GET",
        rust_path: "/jaxrs/ai/chat/list",
        java_war: "x_ai_assemble_control",
        java_action: "jaxrs/chat/list",
        body: None,
        requires_auth: true,
    },
    // ── attendance ─────────────────────────────────────────────────────────
    EndpointDef {
        crate_name: "attendance",
        method: "GET",
        rust_path: "/jaxrs/attendance/record/list",
        java_war: "x_attendance_core_entity",
        java_action: "jaxrs/record/list",
        body: None,
        requires_auth: true,
    },
    // ── correlation ───────────────────────────────────────────────────────
    EndpointDef {
        crate_name: "correlation",
        method: "GET",
        rust_path: "/jaxrs/correlation/list",
        java_war: "x_correlation_core_entity",
        java_action: "jaxrs/list",
        body: None,
        requires_auth: true,
    },
    // ── general ────────────────────────────────────────────────────────────
    EndpointDef {
        crate_name: "general",
        method: "GET",
        rust_path: "/jaxrs/general/dictionary/list",
        java_war: "x_general_core_entity",
        java_action: "jaxrs/dictionary/list",
        body: None,
        requires_auth: true,
    },
    // ── hotpic ─────────────────────────────────────────────────────────────
    EndpointDef {
        crate_name: "hotpic",
        method: "GET",
        rust_path: "/jaxrs/hotpic/list",
        java_war: "x_hotpic_core_entity",
        java_action: "jaxrs/list",
        body: None,
        requires_auth: true,
    },
    // ── jpush ──────────────────────────────────────────────────────────────
    EndpointDef {
        crate_name: "jpush",
        method: "GET",
        rust_path: "/jaxrs/jpush/config/list",
        java_war: "x_jpush_core_entity",
        java_action: "jaxrs/config/list",
        body: None,
        requires_auth: true,
    },
    // ── meeting ────────────────────────────────────────────────────────────
    EndpointDef {
        crate_name: "meeting",
        method: "GET",
        rust_path: "/jaxrs/meeting/room/list",
        java_war: "x_meeting_core_entity",
        java_action: "jaxrs/room/list",
        body: None,
        requires_auth: true,
    },
    EndpointDef {
        crate_name: "meeting",
        method: "GET",
        rust_path: "/jaxrs/meeting/building/list",
        java_war: "x_meeting_core_entity",
        java_action: "jaxrs/building/list",
        body: None,
        requires_auth: true,
    },
    // ── mind ───────────────────────────────────────────────────────────────
    EndpointDef {
        crate_name: "mind",
        method: "GET",
        rust_path: "/jaxrs/mind/list",
        java_war: "x_mind_core_entity",
        java_action: "jaxrs/list",
        body: None,
        requires_auth: true,
    },
    // ── cms ────────────────────────────────────────────────────────────────
    EndpointDef {
        crate_name: "cms",
        method: "GET",
        rust_path: "/jaxrs/cms/catalog/list",
        java_war: "x_cms_core_entity",
        java_action: "jaxrs/catalog/list",
        body: None,
        requires_auth: true,
    },
    // ── process_designer ───────────────────────────────────────────────────
    EndpointDef {
        crate_name: "process_designer",
        method: "GET",
        rust_path: "/jaxrs/process/designer/list",
        java_war: "x_processplatform_assemble_designer",
        java_action: "jaxrs/designer/list",
        body: None,
        requires_auth: true,
    },
    // ── process_express ────────────────────────────────────────────────────
    EndpointDef {
        crate_name: "process_express",
        method: "GET",
        rust_path: "/jaxrs/process/work/list",
        java_war: "x_processplatform_core_express",
        java_action: "jaxrs/work/list",
        body: None,
        requires_auth: true,
    },
    // ── process_surface ────────────────────────────────────────────────────
    EndpointDef {
        crate_name: "process_surface",
        method: "GET",
        rust_path: "/jaxrs/process/read/list",
        java_war: "x_processplatform_assemble_surface",
        java_action: "jaxrs/read/list",
        body: None,
        requires_auth: true,
    },
    // ── query ──────────────────────────────────────────────────────────────
    EndpointDef {
        crate_name: "query",
        method: "GET",
        rust_path: "/jaxrs/query/view/list",
        java_war: "x_query_core_entity",
        java_action: "jaxrs/view/list",
        body: None,
        requires_auth: true,
    },
    EndpointDef {
        crate_name: "query",
        method: "GET",
        rust_path: "/jaxrs/query/designer/list",
        java_war: "x_query_assemble_designer",
        java_action: "jaxrs/designer/list",
        body: None,
        requires_auth: true,
    },
    EndpointDef {
        crate_name: "query",
        method: "GET",
        rust_path: "/jaxrs/query/surface/list",
        java_war: "x_query_assemble_surface",
        java_action: "jaxrs/surface/list",
        body: None,
        requires_auth: true,
    },
    // ── portal_assemble_designer ───────────────────────────────────────────
    EndpointDef {
        crate_name: "portal_assemble_designer",
        method: "GET",
        rust_path: "/jaxrs/portal/designer/category/list",
        java_war: "x_portal_assemble_designer",
        java_action: "jaxrs/designer/category/list",
        body: None,
        requires_auth: true,
    },
    // ── query_assemble_designer ────────────────────────────────────────────
    EndpointDef {
        crate_name: "query_assemble_designer",
        method: "GET",
        rust_path: "/jaxrs/query/designer/category/list",
        java_war: "x_query_assemble_designer",
        java_action: "jaxrs/designer/category/list",
        body: None,
        requires_auth: true,
    },
    // ── processplatform_assemble_designer ──────────────────────────────────
    EndpointDef {
        crate_name: "processplatform_assemble_designer",
        method: "GET",
        rust_path: "/jaxrs/process/designer/category/list",
        java_war: "x_processplatform_assemble_designer",
        java_action: "jaxrs/designer/category/list",
        body: None,
        requires_auth: true,
    },
    // ── organization_assemble_control ──────────────────────────────────────
    EndpointDef {
        crate_name: "organization_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/organization/person/list",
        java_war: "x_organization_assemble_control",
        java_action: "jaxrs/person/list",
        body: None,
        requires_auth: true,
    },
    EndpointDef {
        crate_name: "organization_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/organization/unit/list",
        java_war: "x_organization_assemble_control",
        java_action: "jaxrs/unit/list",
        body: None,
        requires_auth: true,
    },
    // ── meeting_assemble_control ───────────────────────────────────────────
    EndpointDef {
        crate_name: "meeting_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/meeting/list",
        java_war: "x_meeting_assemble_control",
        java_action: "jaxrs/meeting/list",
        body: None,
        requires_auth: true,
    },
    // ── message_assemble_communicate ───────────────────────────────────────
    EndpointDef {
        crate_name: "message_assemble_communicate",
        method: "GET",
        rust_path: "/jaxrs/message/chat/list",
        java_war: "x_message_assemble_communicate",
        java_action: "jaxrs/chat/list",
        body: None,
        requires_auth: true,
    },
    // ── bbs_assemble_control ───────────────────────────────────────────────
    EndpointDef {
        crate_name: "bbs_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/bbs/forum/list",
        java_war: "x_bbs_assemble_control",
        java_action: "jaxrs/forum/list",
        body: None,
        requires_auth: true,
    },
    // ── attendance_assemble_control ────────────────────────────────────────
    EndpointDef {
        crate_name: "attendance_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/attendance/statistics/list",
        java_war: "x_attendance_assemble_control",
        java_action: "jaxrs/statistics/list",
        body: None,
        requires_auth: true,
    },
    // ── general_assemble_control ───────────────────────────────────────────
    EndpointDef {
        crate_name: "general_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/general/dictionary/list",
        java_war: "x_general_assemble_control",
        java_action: "jaxrs/dictionary/list",
        body: None,
        requires_auth: true,
    },
    // ── file_assemble_control ──────────────────────────────────────────────
    EndpointDef {
        crate_name: "file_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/file/list",
        java_war: "x_file_assemble_control",
        java_action: "jaxrs/list",
        body: None,
        requires_auth: true,
    },
    // ── ai_assemble_control ────────────────────────────────────────────────
    EndpointDef {
        crate_name: "ai_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/ai/chat/list",
        java_war: "x_ai_assemble_control",
        java_action: "jaxrs/chat/list",
        body: None,
        requires_auth: true,
    },
    // ── hotpic_assemble_control ────────────────────────────────────────────
    EndpointDef {
        crate_name: "hotpic_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/hotpic/list",
        java_war: "x_hotpic_assemble_control",
        java_action: "jaxrs/list",
        body: None,
        requires_auth: true,
    },
    // ── jpush_assemble_control ─────────────────────────────────────────────
    EndpointDef {
        crate_name: "jpush_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/jpush/config/list",
        java_war: "x_jpush_assemble_control",
        java_action: "jaxrs/config/list",
        body: None,
        requires_auth: true,
    },
    // ── mind_assemble_control ──────────────────────────────────────────────
    EndpointDef {
        crate_name: "mind_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/mind/list",
        java_war: "x_mind_assemble_control",
        java_action: "jaxrs/list",
        body: None,
        requires_auth: true,
    },
    // ── correlation_service_processing ────────────────────────────────────
    EndpointDef {
        crate_name: "correlation_service_processing",
        method: "GET",
        rust_path: "/jaxrs/correlation/list",
        java_war: "x_correlation_service_processing",
        java_action: "jaxrs/list",
        body: None,
        requires_auth: true,
    },
    // ── processplatform_service_processing ─────────────────────────────────
    EndpointDef {
        crate_name: "processplatform_service_processing",
        method: "GET",
        rust_path: "/jaxrs/process/work/list",
        java_war: "x_processplatform_service_processing",
        java_action: "jaxrs/work/list",
        body: None,
        requires_auth: true,
    },
    // ── portal_assemble_surface ────────────────────────────────────────────
    EndpointDef {
        crate_name: "portal_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/portal/page/list",
        java_war: "x_portal_assemble_surface",
        java_action: "jaxrs/page/list",
        body: None,
        requires_auth: true,
    },
    // ── organization_assemble_express ──────────────────────────────────────
    EndpointDef {
        crate_name: "organization_assemble_express",
        method: "GET",
        rust_path: "/jaxrs/express/person/list",
        java_war: "x_organization_assemble_express",
        java_action: "jaxrs/person/list",
        body: None,
        requires_auth: true,
    },
    // ── calendar_assemble_control ──────────────────────────────────────────
    EndpointDef {
        crate_name: "calendar_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/calendar/event/list",
        java_war: "x_calendar_assemble_control",
        java_action: "jaxrs/event/list",
        body: None,
        requires_auth: true,
    },
    // ── component_assemble_control ─────────────────────────────────────────
    EndpointDef {
        crate_name: "component_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/component/category/list",
        java_war: "x_component_assemble_control",
        java_action: "jaxrs/category/list",
        body: None,
        requires_auth: true,
    },
    // ── query_service_processing ───────────────────────────────────────────
    EndpointDef {
        crate_name: "query_service_processing",
        method: "GET",
        rust_path: "/jaxrs/query/view/list",
        java_war: "x_query_service_processing",
        java_action: "jaxrs/view/list",
        body: None,
        requires_auth: true,
    },
    // ── processplatform_assemble_surface ───────────────────────────────────
    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/process/read/list",
        java_war: "x_processplatform_assemble_surface",
        java_action: "jaxrs/read/list",
        body: None,
        requires_auth: true,
    },
    // ── processplatform_core_entity ────────────────────────────────────────
    EndpointDef {
        crate_name: "processplatform_core_entity",
        method: "GET",
        rust_path: "/jaxrs/process/work/list",
        java_war: "x_processplatform_core_entity",
        java_action: "jaxrs/work/list",
        body: None,
        requires_auth: true,
    },
    // ── processplatform_core_express ───────────────────────────────────────
    EndpointDef {
        crate_name: "processplatform_core_express",
        method: "GET",
        rust_path: "/jaxrs/process/work/list",
        java_war: "x_processplatform_core_express",
        java_action: "jaxrs/work/list",
        body: None,
        requires_auth: true,
    },
    // ── query_core_entity ──────────────────────────────────────────────────
    EndpointDef {
        crate_name: "query_core_entity",
        method: "GET",
        rust_path: "/jaxrs/query/view/list",
        java_war: "x_query_core_entity",
        java_action: "jaxrs/view/list",
        body: None,
        requires_auth: true,
    },
    // ── query_core_express ─────────────────────────────────────────────────
    EndpointDef {
        crate_name: "query_core_express",
        method: "GET",
        rust_path: "/jaxrs/query/view/list",
        java_war: "x_query_core_express",
        java_action: "jaxrs/view/list",
        body: None,
        requires_auth: true,
    },
    // ── correlation_core_entity ────────────────────────────────────────────
    EndpointDef {
        crate_name: "correlation_core_entity",
        method: "GET",
        rust_path: "/jaxrs/correlation/list",
        java_war: "x_correlation_core_entity",
        java_action: "jaxrs/list",
        body: None,
        requires_auth: true,
    },
    // ── correlation_core_express ───────────────────────────────────────────
    EndpointDef {
        crate_name: "correlation_core_express",
        method: "GET",
        rust_path: "/jaxrs/correlation/list",
        java_war: "x_correlation_core_express",
        java_action: "jaxrs/list",
        body: None,
        requires_auth: true,
    },
    // ── cms_core_entity ────────────────────────────────────────────────────
    EndpointDef {
        crate_name: "cms_core_entity",
        method: "GET",
        rust_path: "/jaxrs/cms/catalog/list",
        java_war: "x_cms_core_entity",
        java_action: "jaxrs/catalog/list",
        body: None,
        requires_auth: true,
    },
    // ── cms_core_express ───────────────────────────────────────────────────
    EndpointDef {
        crate_name: "cms_core_express",
        method: "GET",
        rust_path: "/jaxrs/cms/article/list",
        java_war: "x_cms_core_express",
        java_action: "jaxrs/article/list",
        body: None,
        requires_auth: true,
    },
    // ── cms_express ────────────────────────────────────────────────────────
    EndpointDef {
        crate_name: "cms_express",
        method: "GET",
        rust_path: "/jaxrs/cms/article/list",
        java_war: "x_cms_express",
        java_action: "jaxrs/article/list",
        body: None,
        requires_auth: true,
    },
    // ── process_bam ────────────────────────────────────────────────────────
    EndpointDef {
        crate_name: "process_bam",
        method: "GET",
        rust_path: "/jaxrs/process/bam/list",
        java_war: "x_processplatform_assemble_bam",
        java_action: "jaxrs/bam/list",
        body: None,
        requires_auth: true,
    },
    // ── console ────────────────────────────────────────────────────────────
    EndpointDef {
        crate_name: "console",
        method: "GET",
        rust_path: "/jaxrs/console/server/list",
        java_war: "x_console",
        java_action: "jaxrs/server/list",
        body: None,
        requires_auth: true,
    },
    // ── POST endpoints for behavior comparison ───────────────────────────────
    EndpointDef {
        crate_name: "bbs",
        method: "POST",
        rust_path: "/jaxrs/bbs/subject/create",
        java_war: "x_bbs_core_project",
        java_action: "jaxrs/subject/create",
        body: Some(r#"{"sectionId": "test-section", "title": "Test Topic", "content": "Test content"}"#),
        requires_auth: true,
    },
    EndpointDef {
        crate_name: "file",
        method: "POST",
        rust_path: "/jaxrs/file/folder/create",
        java_war: "x_file_core_project",
        java_action: "jaxrs/folder/create",
        body: Some(r#"{"name": "test-folder", "parentId": ""}"#),
        requires_auth: true,
    },
    EndpointDef {
        crate_name: "meeting",
        method: "POST",
        rust_path: "/jaxrs/meeting/create",
        java_war: "x_meeting_core_project",
        java_action: "jaxrs/meeting/create",
        body: Some(r#"{"title": "Test Meeting", "startTime": "2026-01-01T10:00:00Z", "endTime": "2026-01-01T11:00:00Z"}"#),
        requires_auth: true,
    },
];

// ──────────────────────────────────────────────────────────────────────────────
// 测试
// ──────────────────────────────────────────────────────────────────────────────

#[tokio::test]
async fn behavior_compare_rust_vs_java() {
    let java_url = std::env::var("JAVA_SERVICE_URL").unwrap_or_else(|_| DEFAULT_JAVA_BASE_URL.to_string());

    eprintln!("[behavior_compare] Rust base: {}", RUST_BASE_URL);
    eprintln!("[behavior_compare] Java base: {}", java_url);

    // ── 检查 Rust 服务可达性 ──────────────────────────────────────────────
    if !behavior_comparison::comparator::is_service_reachable(RUST_BASE_URL).await {
        eprintln!("[behavior_compare] Rust service unreachable at {} — aborting", RUST_BASE_URL);
        panic!("Rust service unreachable at {} — cannot run behavior comparison", RUST_BASE_URL);
    }
    eprintln!("[behavior_compare] Rust service reachable");

    // ── 检查 Java 服务可达性 ──────────────────────────────────────────────
    let java_reachable =
        behavior_comparison::comparator::is_service_reachable(&java_url).await;
    if !java_reachable {
        eprintln!("[behavior_compare] Java service unreachable at {} — all Java results will be SKIP", java_url);
    } else {
        eprintln!("[behavior_compare] Java service reachable");
    }

    // ── 加载允许列表 ──────────────────────────────────────────────────────
    let allowlist_path = std::env::var("BEHAVIOR_ALLOWLIST_PATH").unwrap_or_else(|_| ALLOWLIST_PATH.to_string());
    let comparator = match EndpointComparator::new(RUST_BASE_URL, &java_url)
        .with_allowlist(&allowlist_path)
    {
        Ok(c) => c,
        Err(e) => {
            eprintln!("[behavior_compare] Failed to load allowlist from {}: {}", allowlist_path, e);
            eprintln!("[behavior_compare] Continuing with empty allowlist");
            EndpointComparator::new(RUST_BASE_URL, &java_url)
        }
    };
    let allowlist_entries = comparator.allowlist.entries.len();

    // ── 尝试登录获取认证令牌 ──────────────────────────────────────────────
    let credential = std::env::var("BEHAVIOR_TEST_CREDENTIAL").unwrap_or_else(|_| DEFAULT_CREDENTIAL.to_string());
    let password = std::env::var("BEHAVIOR_TEST_PASSWORD").unwrap_or_else(|_| DEFAULT_PASSWORD.to_string());

    let comparator = if java_reachable {
        match comparator.login(RUST_BASE_URL, &credential, &password).await {
            Some(rust_token) => {
                eprintln!("[behavior_compare] Rust login successful, token acquired");
                if let Some(java_token) = comparator.login(&java_url, &credential, &password).await {
                    eprintln!("[behavior_compare] Java login successful, token acquired");
                    comparator.with_auth_token(java_token)
                } else {
                    eprintln!("[behavior_compare] Java login failed — protected endpoints will be SKIP");
                    comparator.with_auth_token(rust_token)
                }
            }
            None => {
                eprintln!("[behavior_compare] Rust login failed — protected endpoints may be SKIP");
                comparator
            }
        }
    } else {
        eprintln!("[behavior_compare] Java unreachable — skipping all Java comparisons");
        comparator
    };

    // ── 执行对比 ──────────────────────────────────────────────────────────
    eprintln!("[behavior_compare] Comparing {} endpoints...", ENDPOINTS.len());
    let results = comparator.compare_all(ENDPOINTS).await;

    let passed = results.iter().filter(|r| r.status == ComparisonStatus::Pass).count();
    let failed = results.iter().filter(|r| r.status == ComparisonStatus::Fail).count();
    let skipped = results.iter().filter(|r| r.status == ComparisonStatus::Skip).count();

    eprintln!("[behavior_compare] Results: {} passed, {} failed, {} skipped", passed, failed, skipped);

    // ── 生成报告 ──────────────────────────────────────────────────────────
    let mut report = behavior_comparison::reporter::ComparisonReport::new(&java_url)
        .with_allowlist_count(allowlist_entries);

    for result in results {
        report.add_result(result);
    }

    let markdown = report.to_markdown();

    // 确保输出目录存在
    if let Some(parent) = std::path::Path::new(REPORT_PATH).parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    if let Err(e) = std::fs::write(REPORT_PATH, &markdown) {
        eprintln!("[behavior_compare] Failed to write report: {}", e);
    } else {
        eprintln!("[behavior_compare] Report written to {}", REPORT_PATH);
    }

    // ── 断言：无 FAIL（Java 不可达时不在此处失败，但记录 SKIP）───────────
    if failed > 0 {
        panic!(
            "behavior comparison: {} endpoint(s) FAILED ({} passed, {} skipped). See {}",
            failed, passed, skipped, REPORT_PATH
        );
    }

    eprintln!("[behavior_compare] All comparisons passed or skipped ({} SKIP due to Java unreachable)", skipped);
}
