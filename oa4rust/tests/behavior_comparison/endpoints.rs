/// 行为对比测试端点列表（自动生成）

/// 生成时间: 2026-08-21 19:47:15
use super::EndpointDef;

pub const ENDPOINTS: &[EndpointDef] = &[

    EndpointDef {
        crate_name: "ai",
        method: "GET",
        rust_path: "/jaxrs/ai/app/list",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "ai",
        method: "GET",
        rust_path: "/jaxrs/ai/chat/delete/{clue_id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "ai",
        method: "GET",
        rust_path: "/jaxrs/ai/chat/list/completion/{clue_id}/paging/{page}/size/{size}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "ai",
        method: "GET",
        rust_path: "/jaxrs/ai/chat/list/paging/{page}/size/{size}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "ai",
        method: "GET",
        rust_path: "/jaxrs/ai/config/base/config",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "ai",
        method: "GET",
        rust_path: "/jaxrs/ai/config/get",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "ai",
        method: "GET",
        rust_path: "/jaxrs/ai/config/get/mcp/{flag}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "ai",
        method: "GET",
        rust_path: "/jaxrs/ai/config/get/model/{flag}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "ai",
        method: "GET",
        rust_path: "/jaxrs/ai/config/list/enable/model",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "ai",
        method: "GET",
        rust_path: "/jaxrs/ai/config/list/mcp/paging/{page}/size/{size}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "ai",
        method: "GET",
        rust_path: "/jaxrs/ai/config/list/model/paging/{page}/size/{size}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "ai",
        method: "GET",
        rust_path: "/jaxrs/ai/conversation/list",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "ai",
        method: "GET",
        rust_path: "/jaxrs/ai/file/delete/{flag}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "ai",
        method: "GET",
        rust_path: "/jaxrs/ai/file/{flag}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "ai",
        method: "GET",
        rust_path: "/jaxrs/ai/file/{id}/download",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "ai",
        method: "GET",
        rust_path: "/jaxrs/ai/file/{id}/download/scale",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "ai",
        method: "GET",
        rust_path: "/jaxrs/ai/index/cms/doc/with/app/{appId}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "ai",
        method: "GET",
        rust_path: "/jaxrs/ai/index/cms/doc/{docId}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "ai",
        method: "GET",
        rust_path: "/jaxrs/ai/index/delete/{flag}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "ai",
        method: "GET",
        rust_path: "/jaxrs/ai/index/sync/to/knowledge",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "ai",
        method: "GET",
        rust_path: "/jaxrs/ai/model/list",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "ai_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/ai/assemble/control/config/get/mcp/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "ai_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/ai/assemble/control/config/list/mcp/paging/{page}/size/{size}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "ai_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/ai_assemble_control/config/base/config",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "ai_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/ai_assemble_control/config/create/mcp",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "ai_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/ai_assemble_control/config/create/model",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "ai_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/ai_assemble_control/config/delete/mcp/flag",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "ai_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/ai_assemble_control/config/delete/model/flag",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "ai_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/ai_assemble_control/config/get/mcp/ext/flag",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "ai_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/ai_assemble_control/config/get/mcp/flag",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "ai_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/ai_assemble_control/config/get/model/flag",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "ai_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/ai_assemble_control/config/list/enable/model",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "ai_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/ai_assemble_control/config/list/mcp/paging/{page}/size/{size}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "ai_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/ai_assemble_control/config/list/model/paging/{page}/size/{size}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "ai_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/ai_assemble_control/config/save",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "ai_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/ai_assemble_control/config/update/mcp/flag",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "ai_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/ai_assemble_control/config/update/model/flag",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "ai_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/ai_assemble_control/file/copy/file",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "ai_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/ai_assemble_control/file/delete/flag",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "ai_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/ai_assemble_control/file/flag",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "ai_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/ai_assemble_control/file/id/download",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "ai_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/ai_assemble_control/file/id/download/scale",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "ai_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/ai_assemble_control/file/list",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "ai_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/ai_assemble_control/file/list/paging/{page}/size/{size}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "ai_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/ai_assemble_control/file/upload",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "ai_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/ai_assemble_control/get/ai/control/config",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "ai_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/ai_assemble_control/get/usage/stats",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "ai_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/ai_assemble_control/index/cms/doc/docId",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "ai_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/ai_assemble_control/index/cms/doc/with/app/appId",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "ai_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/ai_assemble_control/index/delete/flag",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "ai_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/ai_assemble_control/index/list/paging/{page}/size/{size}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "ai_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/ai_assemble_control/index/sync/to/knowledge",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "ai_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/ai_assemble_control/list/ai/models",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "ai_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/ai_assemble_control/update/ai/control/config",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "ai_assemble_control",
        method: "POST",
        rust_path: "/jaxrs/ai/assemble/control/config/create/mcp",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "ai_assemble_control",
        method: "POST",
        rust_path: "/jaxrs/ai/assemble/control/config/delete/mcp/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "ai_assemble_control",
        method: "POST",
        rust_path: "/jaxrs/ai/assemble/control/config/update/mcp/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "ai_assemble_control",
        method: "POST",
        rust_path: "/jaxrs/ai_assemble_control/chat/completion",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "ai_assemble_control",
        method: "POST",
        rust_path: "/jaxrs/ai_assemble_control/chat/completion/stream",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "ai_core_entity",
        method: "GET",
        rust_path: "/jaxrs/ai/core/entity/app/list",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "ai_core_entity",
        method: "GET",
        rust_path: "/jaxrs/ai/core/entity/conversation/list",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "ai_core_entity",
        method: "GET",
        rust_path: "/jaxrs/ai/core/entity/model/list",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "attendance",
        method: "GET",
        rust_path: "/jaxrs/attendance/admin/list/all",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "attendance",
        method: "GET",
        rust_path: "/jaxrs/attendance/appeal/list",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "attendance",
        method: "GET",
        rust_path: "/jaxrs/attendance/employee/config/list/all",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "attendance",
        method: "GET",
        rust_path: "/jaxrs/attendance/record/list",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "attendance",
        method: "GET",
        rust_path: "/jaxrs/attendance/rule/list",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "attendance",
        method: "GET",
        rust_path: "/jaxrs/attendance/statistical/cycle/list/all",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "attendance",
        method: "POST",
        rust_path: "/jaxrs/attendance/appeal/archive/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "attendance",
        method: "POST",
        rust_path: "/jaxrs/attendance/appeal/audit",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "attendance",
        method: "POST",
        rust_path: "/jaxrs/attendance/appeal/submit",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "attendance_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/attendance/assemble/control/attendanceadmin/list/all",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "attendance_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/attendance/assemble/control/attendanceadmin/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "attendance_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/attendance/assemble/control/attendanceappealInfo/filter/list/{id}/next/{count}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "attendance_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/attendance/assemble/control/attendanceappealInfo/filter/list/{id}/prev/{count}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "attendance_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/attendance/assemble/control/attendanceappealInfo/manager/list/{id}/next/{count}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "attendance_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/attendance/assemble/control/attendanceappealInfo/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "attendance_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/attendance/assemble/control/attendancedetail/filter/list",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "attendance_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/attendance/assemble/control/attendancedetail/filter/list/topUnit",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "attendance_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/attendance/assemble/control/attendancedetail/filter/list/unit",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "attendance_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/attendance/assemble/control/attendancedetail/filter/list/user",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "attendance_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/attendance/assemble/control/attendancedetail/filter/list/{id}/next/{count}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "attendance_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/attendance/assemble/control/attendancedetail/filter/list/{id}/prev/{count}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "attendance_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/attendance/assemble/control/attendancedetail/list/persons/nonesign",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "attendance_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/attendance/assemble/control/attendancedetail/list/{file_id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "attendance_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/attendance/assemble/control/attendancedetail/mobile/filter/list/page/{page}/count/{count}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "attendance_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/attendance/assemble/control/attendancedetail/mobile/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "attendance_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/attendance/assemble/control/attendancedetail/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "attendance_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/attendance/assemble/control/attendanceemployeeconfig/list/all",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "attendance_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/attendance/assemble/control/attendanceemployeeconfig/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "attendance_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/attendance/assemble/control/attendanceimportfileinfo/list/all",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "attendance_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/attendance/assemble/control/attendanceimportfileinfo/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "attendance_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/attendance/assemble/control/attendanceschedulesetting/list/all",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "attendance_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/attendance/assemble/control/attendanceschedulesetting/list/topUnit/{name}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "attendance_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/attendance/assemble/control/attendanceschedulesetting/list/unit/{name}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "attendance_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/attendance/assemble/control/attendanceschedulesetting/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "attendance_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/attendance/assemble/control/attendanceselfholiday/filter/list/{id}/next/{count}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "attendance_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/attendance/assemble/control/attendanceselfholiday/filter/list/{id}/prev/{count}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "attendance_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/attendance/assemble/control/attendanceselfholiday/list/all",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "attendance_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/attendance/assemble/control/attendanceselfholiday/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "attendance_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/attendance/assemble/control/attendancesetting/code/{code}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "attendance_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/attendance/assemble/control/attendancesetting/list/all",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "attendance_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/attendance/assemble/control/attendancesetting/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "attendance_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/attendance/assemble/control/attendancestatisticalcycle/cycleDetail/{year}/{month}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "attendance_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/attendance/assemble/control/attendancestatisticalcycle/list/all",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "attendance_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/attendance/assemble/control/attendancestatisticalcycle/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "attendance_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/attendance/assemble/control/attendancestatisticrequirelog/list/all",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "attendance_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/attendance/assemble/control/attendancestatisticrequirelog/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "attendance_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/attendance/assemble/control/attendanceworkdayconfig/list/all",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "attendance_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/attendance/assemble/control/attendanceworkdayconfig/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "attendance_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/attendance/assemble/control/rule/list",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "attendance_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/attendance/assemble/control/selfholidaysimple/docId/{docId}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "attendance_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/attendance/assemble/control/statisticshow/filter/personMonth/list/{id}/next/{count}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "attendance_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/attendance/assemble/control/statisticshow/filter/personMonth/list/{id}/prev/{count}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "attendance_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/attendance/assemble/control/statisticshow/filter/topUnitDay/list/{id}/next/{count}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "attendance_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/attendance/assemble/control/statisticshow/filter/topUnitDay/list/{id}/prev/{count}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "attendance_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/attendance/assemble/control/statisticshow/filter/topUnitMonth/list/{id}/next/{count}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "attendance_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/attendance/assemble/control/statisticshow/filter/topUnitMonth/list/{id}/prev/{count}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "attendance_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/attendance/assemble/control/statisticshow/filter/unitDay/list/{id}/next/{count}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "attendance_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/attendance/assemble/control/statisticshow/filter/unitDay/list/{id}/prev/{count}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "attendance_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/attendance/assemble/control/statisticshow/filter/unitMonth/list/{id}/next/{count}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "attendance_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/attendance/assemble/control/statisticshow/filter/unitMonth/list/{id}/prev/{count}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "attendance_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/attendance/assemble/control/statisticshow/person/{name}/{year}/{month}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "attendance_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/attendance/assemble/control/statisticshow/persons/unit/subnested/{name}/{year}/{month}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "attendance_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/attendance/assemble/control/statisticshow/persons/unit/{name}/{year}/{month}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "attendance_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/attendance/assemble/control/statisticshow/topUnit/day/{name}/{year}/{month}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "attendance_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/attendance/assemble/control/statisticshow/topUnit/{name}/{year}/{month}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "attendance_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/attendance/assemble/control/statisticshow/unit/day/topUnit/{name}/{date}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "attendance_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/attendance/assemble/control/statisticshow/unit/day/{name}/{date}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "attendance_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/attendance/assemble/control/statisticshow/unit/day/{name}/{year}/{month}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "attendance_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/attendance/assemble/control/statisticshow/unit/subnested/{name}/{year}/{month}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "attendance_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/attendance/assemble/control/statisticshow/unit/sum/{name}/{year}/{month}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "attendance_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/attendance/assemble/control/statisticshow/unit/topUnit/{name}/{year}/{month}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "attendance_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/attendance/assemble/control/statisticshow/unit/{name}/{year}/{month}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "attendance_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/attendance/assemble/control/uuid/random",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "attendance_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/attendance/assemble/control/workplace/list/all",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "attendance_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/attendance/assemble/control/workplace/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "attendance_assemble_control",
        method: "POST",
        rust_path: "/jaxrs/attendance/assemble/control/attendanceappealInfo/appeal/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "attendance_assemble_control",
        method: "POST",
        rust_path: "/jaxrs/attendance/assemble/control/attendanceappealInfo/archive/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "attendance_assemble_control",
        method: "POST",
        rust_path: "/jaxrs/attendance/assemble/control/attendanceappealInfo/audit",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "attendance_assemble_control",
        method: "POST",
        rust_path: "/jaxrs/attendance/assemble/control/attendanceappealInfo/check",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "attendance_assemble_control",
        method: "POST",
        rust_path: "/jaxrs/attendance/assemble/control/attendanceappealInfo/workflow/appeal/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "attendance_assemble_control",
        method: "POST",
        rust_path: "/jaxrs/attendance/assemble/control/attendanceappealInfo/workflow/sync",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "attendance_assemble_control",
        method: "POST",
        rust_path: "/jaxrs/attendance/assemble/control/attendancedetail/analyse",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "attendance_assemble_control",
        method: "POST",
        rust_path: "/jaxrs/attendance/assemble/control/attendancedetail/analyse/id/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "attendance_assemble_control",
        method: "POST",
        rust_path: "/jaxrs/attendance/assemble/control/attendancedetail/analyse/redo",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "attendance_assemble_control",
        method: "POST",
        rust_path: "/jaxrs/attendance/assemble/control/attendancedetail/analyse/{startDate}/{endDate}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "attendance_assemble_control",
        method: "POST",
        rust_path: "/jaxrs/attendance/assemble/control/attendancedetail/archive/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "attendance_assemble_control",
        method: "POST",
        rust_path: "/jaxrs/attendance/assemble/control/attendancedetail/checkDetailWithPersonByCycle/{cycleYear}/{cycleMonth}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "attendance_assemble_control",
        method: "POST",
        rust_path: "/jaxrs/attendance/assemble/control/attendancedetail/mobile/mobilepreview",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "attendance_assemble_control",
        method: "POST",
        rust_path: "/jaxrs/attendance/assemble/control/attendancedetail/mobile/my",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "attendance_assemble_control",
        method: "POST",
        rust_path: "/jaxrs/attendance/assemble/control/attendancedetail/mobile/recive",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "attendance_assemble_control",
        method: "POST",
        rust_path: "/jaxrs/attendance/assemble/control/attendancedetail/recive",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "attendance_assemble_control",
        method: "POST",
        rust_path: "/jaxrs/attendance/assemble/control/attendancedetail/reciveSingle",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "attendance_assemble_control",
        method: "POST",
        rust_path: "/jaxrs/attendance/assemble/control/attendancesetting/enable/type",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "attendance_assemble_control",
        method: "POST",
        rust_path: "/jaxrs/attendance/assemble/control/attendanceworkdayconfig/filter",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "attendance_assemble_control",
        method: "POST",
        rust_path: "/jaxrs/attendance/assemble/control/rule/{id}/toggle",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "attendance_assemble_control",
        method: "POST",
        rust_path: "/jaxrs/attendance/assemble/control/statistic/do",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "attendance_core_entity",
        method: "GET",
        rust_path: "/jaxrs/attendance/core/entity/record/list",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "attendance_core_entity",
        method: "GET",
        rust_path: "/jaxrs/attendance/core/entity/record/{id}/delete",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "attendance_core_entity",
        method: "GET",
        rust_path: "/jaxrs/attendance/core/entity/rule/list",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "attendance_core_entity",
        method: "GET",
        rust_path: "/jaxrs/attendance/core/entity/rule/{id}/delete",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "attendance_core_entity",
        method: "POST",
        rust_path: "/jaxrs/attendance/core/entity/record/create",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "attendance_core_entity",
        method: "POST",
        rust_path: "/jaxrs/attendance/core/entity/record/{id}/update",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "attendance_core_entity",
        method: "POST",
        rust_path: "/jaxrs/attendance/core/entity/rule/create",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "attendance_core_entity",
        method: "POST",
        rust_path: "/jaxrs/attendance/core/entity/rule/{id}/update",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "auth",
        method: "DELETE",
        rust_path: "/jaxrs/authentication",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "auth",
        method: "GET",
        rust_path: "/jaxrs/andfx/moa/sso/token/{token}/enter/{enterId}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "auth",
        method: "GET",
        rust_path: "/jaxrs/authentication",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "auth",
        method: "GET",
        rust_path: "/jaxrs/authentication/code/credential/{credential}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "auth",
        method: "GET",
        rust_path: "/jaxrs/authentication/group/list",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "auth",
        method: "GET",
        rust_path: "/jaxrs/authentication/oauth/bind/name/{name}/code/{code}/redirecturi/{redirectUri}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "auth",
        method: "GET",
        rust_path: "/jaxrs/authentication/oauth/dingding/config",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "auth",
        method: "GET",
        rust_path: "/jaxrs/authentication/oauth/list",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "auth",
        method: "GET",
        rust_path: "/jaxrs/authentication/oauth/login/dingding/code/{code}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "auth",
        method: "GET",
        rust_path: "/jaxrs/authentication/oauth/login/name/{name}/code/{code}/redirecturi/{redirectUri}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "auth",
        method: "GET",
        rust_path: "/jaxrs/authentication/oauth/login/qywx/code/{code}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "auth",
        method: "GET",
        rust_path: "/jaxrs/authentication/oauth/name/{name}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "auth",
        method: "GET",
        rust_path: "/jaxrs/authentication/oauth/qywx/config",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "auth",
        method: "GET",
        rust_path: "/jaxrs/authentication/role/list",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "auth",
        method: "GET",
        rust_path: "/jaxrs/authentication/sso/client/{client}/token/{token}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "auth",
        method: "GET",
        rust_path: "/jaxrs/authentication/unit/list",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "auth",
        method: "GET",
        rust_path: "/jaxrs/authentication/who",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "auth",
        method: "POST",
        rust_path: "/jaxrs/authentication",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "auth",
        method: "POST",
        rust_path: "/jaxrs/authentication/check/token",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "auth",
        method: "POST",
        rust_path: "/jaxrs/authentication/code",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "auth",
        method: "POST",
        rust_path: "/jaxrs/authentication/login",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "auth",
        method: "POST",
        rust_path: "/jaxrs/authentication/logout",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "auth",
        method: "POST",
        rust_path: "/jaxrs/authentication/refresh",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "auth",
        method: "POST",
        rust_path: "/jaxrs/authentication/safe/logout",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "auth",
        method: "POST",
        rust_path: "/jaxrs/authentication/sso",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "auth",
        method: "POST",
        rust_path: "/jaxrs/authentication/sso/encrypt",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "auth",
        method: "POST",
        rust_path: "/jaxrs/authentication/switchuser",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "auth",
        method: "POST",
        rust_path: "/jaxrs/authentication/two_factor",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "base",
        method: "GET",
        rust_path: "/jaxrs/base/cache/detail",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "base",
        method: "GET",
        rust_path: "/jaxrs/base/echo/get",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "base",
        method: "GET",
        rust_path: "/jaxrs/base/openapi/info",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "bbs",
        method: "GET",
        rust_path: "/jaxrs/bbs/forum/view/all",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "bbs",
        method: "GET",
        rust_path: "/jaxrs/bbs/forum/view/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "bbs",
        method: "GET",
        rust_path: "/jaxrs/bbs/section/view/all",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "bbs",
        method: "GET",
        rust_path: "/jaxrs/bbs/section/viewforum/{forumId}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "bbs",
        method: "GET",
        rust_path: "/jaxrs/bbs/subject/list/{sectionId}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "bbs",
        method: "GET",
        rust_path: "/jaxrs/bbs/subject/search",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "bbs",
        method: "GET",
        rust_path: "/jaxrs/bbs/subject/top/{sectionId}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "bbs",
        method: "GET",
        rust_path: "/jaxrs/bbs/subject/view/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "bbs",
        method: "POST",
        rust_path: "/jaxrs/bbs/subject/create",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "bbs_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/bbs/assemble/control/config",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "bbs_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/bbs/assemble/control/forum/list",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "bbs_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/bbs/assemble/control/forum/view/all",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "bbs_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/bbs/assemble/control/forum/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "bbs_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/bbs/assemble/control/list/reply/filter",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "bbs_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/bbs/assemble/control/list/subjects/filtered",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "bbs_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/bbs/assemble/control/list/subjects/index",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "bbs_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/bbs/assemble/control/list/subjects/recommended/index",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "bbs_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/bbs/assemble/control/list/topics/creamed",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "bbs_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/bbs/assemble/control/list/topics/recommended",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "bbs_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/bbs/assemble/control/permission/section/{sectionId}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "bbs_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/bbs/assemble/control/permission/subject/{subjectId}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "bbs_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/bbs/assemble/control/permission/{replyPublishable}/{subjectId}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "bbs_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/bbs/assemble/control/picture/list",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "bbs_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/bbs/assemble/control/reply/filter/list/{page}/{page}/{count}/{count}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "bbs_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/bbs/assemble/control/reply/list/sub/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "bbs_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/bbs/assemble/control/section/list",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "bbs_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/bbs/assemble/control/section/viewforum/{forumId}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "bbs_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/bbs/assemble/control/shutup/list",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "bbs_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/bbs/assemble/control/subject/creamed/list",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "bbs_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/bbs/assemble/control/subject/filter/list",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "bbs_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/bbs/assemble/control/subject/index/list",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "bbs_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/bbs/assemble/control/subject/search",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "bbs_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/bbs/assemble/control/subject/statgrade",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "bbs_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/bbs/assemble/control/subject/top/{sectionId}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "bbs_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/bbs/assemble/control/subject/view/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "bbs_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/bbs/assemble/control/subjectattach/list",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "bbs_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/bbs/assemble/control/topic/creamed/list",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "bbs_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/bbs/assemble/control/topic/filter/list",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "bbs_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/bbs/assemble/control/topic/index/list",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "bbs_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/bbs/assemble/control/topic/list/forum/{forumId}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "bbs_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/bbs/assemble/control/topic/recommended/index",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "bbs_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/bbs/assemble/control/topic/recommended/list",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "bbs_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/bbs/assemble/control/topic/search",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "bbs_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/bbs/assemble/control/user/info",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "bbs_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/bbs/assemble/control/uuid",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "bbs_assemble_control",
        method: "POST",
        rust_path: "/jaxrs/bbs/assemble/control/delete/forum",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "bbs_assemble_control",
        method: "POST",
        rust_path: "/jaxrs/bbs/assemble/control/delete/reply",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "bbs_assemble_control",
        method: "POST",
        rust_path: "/jaxrs/bbs/assemble/control/delete/subject",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "bbs_assemble_control",
        method: "POST",
        rust_path: "/jaxrs/bbs/assemble/control/reply/create",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "bbs_assemble_control",
        method: "POST",
        rust_path: "/jaxrs/bbs/assemble/control/shutup/create",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "bbs_assemble_control",
        method: "POST",
        rust_path: "/jaxrs/bbs/assemble/control/shutup/delete",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "bbs_assemble_control",
        method: "POST",
        rust_path: "/jaxrs/bbs/assemble/control/topic/create",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "bbs_assemble_control",
        method: "POST",
        rust_path: "/jaxrs/bbs/assemble/control/update/control/config",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "bbs_core_entity",
        method: "DELETE",
        rust_path: "/jaxrs/bbs/core/entity/forum/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "bbs_core_entity",
        method: "DELETE",
        rust_path: "/jaxrs/bbs/core/entity/section/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "bbs_core_entity",
        method: "DELETE",
        rust_path: "/jaxrs/bbs/core/entity/subject/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "bbs_core_entity",
        method: "GET",
        rust_path: "/jaxrs/bbs/core/entity/forum/list",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "bbs_core_entity",
        method: "GET",
        rust_path: "/jaxrs/bbs/core/entity/section/list/{forumId}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "bbs_core_entity",
        method: "GET",
        rust_path: "/jaxrs/bbs/core/entity/subject/list/{sectionId}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "bbs_core_entity",
        method: "GET",
        rust_path: "/jaxrs/bbs/core/entity/subject/search",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "bbs_core_entity",
        method: "GET",
        rust_path: "/jaxrs/bbs/core/entity/subject/top/{sectionId}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "bbs_core_entity",
        method: "POST",
        rust_path: "/jaxrs/bbs/core/entity/forum",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "bbs_core_entity",
        method: "POST",
        rust_path: "/jaxrs/bbs/core/entity/forum/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "bbs_core_entity",
        method: "POST",
        rust_path: "/jaxrs/bbs/core/entity/reply",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "bbs_core_entity",
        method: "POST",
        rust_path: "/jaxrs/bbs/core/entity/section",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "bbs_core_entity",
        method: "POST",
        rust_path: "/jaxrs/bbs/core/entity/section/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "bbs_core_entity",
        method: "POST",
        rust_path: "/jaxrs/bbs/core/entity/subject",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "bbs_core_entity",
        method: "POST",
        rust_path: "/jaxrs/bbs/core/entity/subject/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "calendar",
        method: "GET",
        rust_path: "/jaxrs/calendar/calendar/list/my",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "calendar",
        method: "GET",
        rust_path: "/jaxrs/calendar/calendar/list/public",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "calendar",
        method: "GET",
        rust_path: "/jaxrs/calendar/calendar/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "calendar",
        method: "GET",
        rust_path: "/jaxrs/calendar/event/list/{calendarId}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "calendar",
        method: "POST",
        rust_path: "/jaxrs/calendar/calendar/create",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "calendar",
        method: "POST",
        rust_path: "/jaxrs/calendar/calendar/remove",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "calendar",
        method: "POST",
        rust_path: "/jaxrs/calendar/calendar/update",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "calendar",
        method: "POST",
        rust_path: "/jaxrs/calendar/event/create",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "calendar",
        method: "POST",
        rust_path: "/jaxrs/calendar/event/remove",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "calendar",
        method: "POST",
        rust_path: "/jaxrs/calendar/event/update",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "calendar_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/calendar/assemble/control/calendar/detail/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "calendar_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/calendar_assemble_control/get/control/config",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "calendar_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/calendar_assemble_control/list/control/calendars",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "calendar_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/calendar_assemble_control/update/control/config",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "calendar_core_entity",
        method: "GET",
        rust_path: "/jaxrs/calendar/core/entity/calendar/list/my",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "calendar_core_entity",
        method: "GET",
        rust_path: "/jaxrs/calendar/core/entity/calendar/list/public",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "calendar_core_entity",
        method: "GET",
        rust_path: "/jaxrs/calendar/core/entity/calendar/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "calendar_core_entity",
        method: "GET",
        rust_path: "/jaxrs/calendar/core/entity/event/list/{calendarId}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "calendar_core_entity",
        method: "POST",
        rust_path: "/jaxrs/calendar/core/entity/calendar/create",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "calendar_core_entity",
        method: "POST",
        rust_path: "/jaxrs/calendar/core/entity/calendar/remove",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "calendar_core_entity",
        method: "POST",
        rust_path: "/jaxrs/calendar/core/entity/calendar/update",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "calendar_core_entity",
        method: "POST",
        rust_path: "/jaxrs/calendar/core/entity/event/create",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "calendar_core_entity",
        method: "POST",
        rust_path: "/jaxrs/calendar/core/entity/event/remove",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "calendar_core_entity",
        method: "POST",
        rust_path: "/jaxrs/calendar/core/entity/event/update",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "cms_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/anonymous/document/{id}/view",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "cms_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/anonymous/fileinfo/download/document/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "cms_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/anonymous/fileinfo/list/{documentId}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "cms_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/anonymous/surface/appdict/list/appInfo/{appInfoFlag}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "cms_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/appinfo/flag",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "cms_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/appinfo/list/all",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "cms_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/appinfo/list/has/document/type/{appType}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "cms_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/appinfo/list/has/document/{appType}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "cms_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/appinfo/list/manage",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "cms_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/appinfo/list/manage/type/{appType}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "cms_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/appinfo/list/user/publish",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "cms_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/appinfo/list/user/publish/type/{appType}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "cms_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/appinfo/list/user/publish/with/process",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "cms_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/appinfo/list/user/view",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "cms_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/appinfo/list/user/view/all",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "cms_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/appinfo/list/user/view/all/type/{appType}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "cms_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/appinfo/list/user/view/article/type/{appType}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "cms_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/appinfo/list/user/view/data",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "cms_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/appinfo/list/user/view/data/type/{appType}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "cms_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/appinfo/list/{appType}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "cms_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/appinfo/list/{appType}/manager",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "cms_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/appinfo/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "cms_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/application/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "cms_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/categoryinfo/flag",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "cms_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/categoryinfo/list/all",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "cms_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/categoryinfo/list/manage/app/{appId}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "cms_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/categoryinfo/list/objects",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "cms_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/categoryinfo/list/publish/app/{appId}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "cms_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/categoryinfo/list/view/app/{appId}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "cms_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/categoryinfo/list/view/app/{appId}/all",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "cms_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/categoryinfo/list/view/app/{appId}/data",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "cms_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/categoryinfo/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "cms_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/cms/assemble/control/anonymous/fileinfo/download/document/stream/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "cms_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/cms/assemble/control/anonymous/form/v2/lookup/document/mobile/{docId}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "cms_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/cms/assemble/control/anonymous/form/v2/lookup/document/{docId}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "cms_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/cms/assemble/control/anonymous/form/v2/mobile/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "cms_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/cms/assemble/control/anonymous/form/v2/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "cms_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/cms/assemble/control/anonymous/form/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "cms_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/cms/assemble/control/anonymous/surface/appdict/data/{appDictFlag}/{appInfo}/{appInfoFlag}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "cms_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/cms/assemble/control/anonymous/surface/appdict/path0/data/{appDictFlag}/{appInfo}/{appInfoFlag}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "cms_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/cms/assemble/control/anonymous/surface/appdict/path0/path1/data/{appDictFlag}/{appInfo}/{appInfoFlag}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "cms_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/cms/assemble/control/anonymous/surface/appdict/path0/path1/path2/data/{appDictFlag}/{appInfo}/{appInfoFlag}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "cms_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/cms/assemble/control/anonymous/surface/appdict/path0/path1/path2/path3/data/{appDictFlag}/{appInfo}/{appInfoFlag}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "cms_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/cms/assemble/control/anonymous/surface/appdict/path0/path1/path2/path3/path4/data/{appDictFlag}/{appInfo}/{appInfoFlag}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "cms_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/cms/assemble/control/anonymous/surface/appdict/path0/path1/path2/path3/path4/path5/data/{appDictFlag}/{appInfo}/{appInfoFlag}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "cms_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/cms/assemble/control/anonymous/surface/appdict/path0/path1/path2/path3/path4/path5/path6/data/{appDictFlag}/{appInfo}/{appInfoFlag}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "cms_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/cms/assemble/control/anonymous/surface/appdict/path0/path1/path2/path3/path4/path5/path6/path7/data/{appDictFlag}/{appInfo}/{appInfoFlag}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "cms_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/cms/assemble/control/anonymous/surface/appdict/{appDictFlag}/{appInfo}/{appInfoFlag}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "cms_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/cms/assemble/control/appinfo/alias/alias",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "cms_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/cms/assemble/control/appinfo/control/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "cms_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/cms/assemble/control/appinfo/icon/{appId}/{size}/{size}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "cms_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/cms/assemble/control/appinfo/permission/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "cms_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/cms/assemble/control/categoryinfo/alias/alias",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "cms_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/cms/assemble/control/categoryinfo/control/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "cms_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/cms/assemble/control/categoryinfo/execute/projection/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "cms_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/cms/assemble/control/categoryinfo/permission/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "cms_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/cms/assemble/control/categoryinfo/{extContent}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "cms_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/cms/assemble/control/comment/commend/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "cms_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/cms/assemble/control/comment/uncommend/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "cms_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/cms/assemble/control/design/appdict/mockputtopost/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "cms_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/cms/assemble/control/document/cipher/permission/read/person/person/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "cms_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/cms/assemble/control/document/cipher/persist/view/record/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "cms_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/cms/assemble/control/file/content/{flag}/{appInfo}/{appInfoFlag}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "cms_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/cms/assemble/control/file/content/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "cms_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/cms/assemble/control/file/download/{flag}/{appInfo}/{appInfoFlag}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "cms_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/cms/assemble/control/file/download/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "cms_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/cms/assemble/control/file/mockputtopost/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "cms_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/cms/assemble/control/fileinfo/batch/download/doc/site/site/{docId}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "cms_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/cms/assemble/control/fileinfo/binary/base64/{id}/{size}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "cms_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/cms/assemble/control/fileinfo/copy/to/doc/{docId}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "cms_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/cms/assemble/control/fileinfo/doc/change/seqnumber/{id}/{docId}/{seqNumber}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "cms_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/cms/assemble/control/fileinfo/download/document/stream/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "cms_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/cms/assemble/control/fileinfo/download/transfer/{flag}/{flag}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "cms_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/cms/assemble/control/fileinfo/edit/doc/mockputtopost/{id}/{docId}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "cms_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/cms/assemble/control/fileinfo/edit/doc/{id}/{docId}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "cms_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/cms/assemble/control/fileinfo/online/info/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "cms_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/cms/assemble/control/fileinfo/preview/pdf/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "cms_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/cms/assemble/control/fileinfo/replace/to/doc/{docId}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "cms_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/cms/assemble/control/form/appinfo/{formFlag}/{appFlag}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "cms_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/cms/assemble/control/form/mockputtopost/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "cms_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/cms/assemble/control/form/v2/lookup/document/mobile/{docId}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "cms_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/cms/assemble/control/form/v2/lookup/document/{docId}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "cms_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/cms/assemble/control/form/v2/mobile/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "cms_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/cms/assemble/control/formversion/list/form/{formId}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "cms_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/cms/assemble/control/formversion/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "cms_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/cms/assemble/control/output/select/mockputtopost/{appInfoFlag}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "cms_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/cms/assemble/control/output/select/{appInfoFlag}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "cms_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/cms/assemble/control/permission/management/refresh/{category}/{categoryId}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "cms_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/cms/assemble/control/script/app/imported/{uniqueName}/{flag}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "cms_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/cms/assemble/control/script/app/{uniqueName}/{flag}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "cms_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/cms/assemble/control/script/mockputtopost/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "cms_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/cms/assemble/control/scriptversion/list/script/{scriptId}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "cms_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/cms/assemble/control/scriptversion/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "cms_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/cms/assemble/control/surface/appdict/data/{appDictFlag}/{appInfo}/{appInfoFlag}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "cms_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/cms/assemble/control/surface/appdict/mockputtopost/{appDictFlag}/{appInfo}/{appInfoFlag}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "cms_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/cms/assemble/control/surface/appdict/path0/data/{appDictFlag}/{appInfo}/{appInfoFlag}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "cms_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/cms/assemble/control/surface/appdict/path0/path1/data/{appDictFlag}/{appInfo}/{appInfoFlag}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "cms_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/cms/assemble/control/surface/appdict/path0/path1/path2/data/mockputtopost/{appDictFlag}/{appInfo}/{appInfoFlag}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "cms_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/cms/assemble/control/surface/appdict/path0/path1/path2/data/{appDictFlag}/{appInfo}/{appInfoFlag}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "cms_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/cms/assemble/control/surface/appdict/path0/path1/path2/path3/data/mockputtopost/{appDictFlag}/{appInfo}/{appInfoFlag}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "cms_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/cms/assemble/control/surface/appdict/path0/path1/path2/path3/data/{appDictFlag}/{appInfo}/{appInfoFlag}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "cms_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/cms/assemble/control/surface/appdict/path0/path1/path2/path3/path4/data/mockputtopost/{appDictFlag}/{appInfo}/{appInfoFlag}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "cms_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/cms/assemble/control/surface/appdict/path0/path1/path2/path3/path4/data/{appDictFlag}/{appInfo}/{appInfoFlag}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "cms_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/cms/assemble/control/surface/appdict/path0/path1/path2/path3/path4/path5/data/mockputtopost/{appDictFlag}/{appInfo}/{appInfoFlag}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "cms_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/cms/assemble/control/surface/appdict/path0/path1/path2/path3/path4/path5/data/{appDictFlag}/{appInfo}/{appInfoFlag}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "cms_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/cms/assemble/control/surface/appdict/path0/path1/path2/path3/path4/path5/path6/data/mockputtopost/{appDictFlag}/{appInfo}/{appInfoFlag}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "cms_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/cms/assemble/control/surface/appdict/path0/path1/path2/path3/path4/path5/path6/data/{appDictFlag}/{appInfo}/{appInfoFlag}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "cms_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/cms/assemble/control/surface/appdict/path0/path1/path2/path3/path4/path5/path6/path7/data/mockputtopost/{appDictFlag}/{appInfo}/{appInfoFlag}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "cms_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/cms/assemble/control/surface/appdict/path0/path1/path2/path3/path4/path5/path6/path7/data/{appDictFlag}/{appInfo}/{appInfoFlag}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "cms_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/cms/assemble/control/surface/appdict/{appDictFlag}/{appInfo}/{appInfoFlag}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "cms_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/cms/assemble/control/templateform/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "cms_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/cms/assemble/control/view/mockputtopost/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "cms_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/cms/assemble/control/viewfieldconfig/mockputtopost/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "cms_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/cms/assemble/control/viewrecord/document/has/view/{docId}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "cms_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/cms_assemble_control/anonymous/document/filter/list/id/next/count",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "cms_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/cms_assemble_control/anonymous/document/filter/list/page/size/size",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "cms_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/cms_assemble_control/appinfo/filter/list/id/next/count",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "cms_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/cms_assemble_control/appinfo/filter/list/id/prev/count",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "cms_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/cms_assemble_control/document/search",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "cms_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/cms_assemble_control/get/control/config",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "cms_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/cms_assemble_control/list/control/sections",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "cms_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/cms_assemble_control/update/control/config",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "cms_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/commend/list/paging/{docId}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "cms_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/commend/list/paging/{pageSize}/{size}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "cms_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/commend/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "cms_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/comment/list/page/{size}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "cms_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/comment/list/page/{size}/mockputtopost",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "cms_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/comment/list/{id}/next/count",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "cms_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/comment/list/{id}/next/count/mockputtopost",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "cms_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/comment/list/{id}/prev/count",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "cms_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/comment/list/{id}/prev/count/mockputtopost",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "cms_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/comment/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "cms_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/correlation/doc/{docId}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "cms_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/correlation/list/doc/{docId}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "cms_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/correlation/list/doc/{docId}/site/{site}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "cms_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/data/document/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "cms_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/data/document/{id}/array/data",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "cms_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/data/document/{id}/mockdeletetoget",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "cms_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/data/document/{id}/path0",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "cms_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/data/document/{id}/path0/mockdeletetoget",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "cms_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/data/document/{id}/path0/path1",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "cms_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/data/document/{id}/path0/path1/mockdeletetoget",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "cms_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/data/document/{id}/path0/path1/path2",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "cms_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/data/document/{id}/path0/path1/path2/mockdeletetoget",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "cms_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/data/document/{id}/path0/path1/path2/path3",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "cms_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/data/document/{id}/path0/path1/path2/path3/mockdeletetoget",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "cms_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/data/document/{id}/path0/path1/path2/path3/path4",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "cms_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/data/document/{id}/path0/path1/path2/path3/path4/mockdeletetoget",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "cms_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/data/document/{id}/path0/path1/path2/path3/path4/path5",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "cms_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/data/document/{id}/path0/path1/path2/path3/path4/path5/mockdeletetoget",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "cms_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/data/document/{id}/path0/path1/path2/path3/path4/path5/path6",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "cms_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/data/document/{id}/path0/path1/path2/path3/path4/path5/path6/mockdeletetoget",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "cms_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/data/document/{id}/path0/path1/path2/path3/path4/path5/path6/path7",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "cms_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/data/document/{id}/path0/path1/path2/path3/path4/path5/path6/path7/mockdeletetoget",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "cms_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/design/appdict/list/appInfo/{appId}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "cms_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/design/appdict/list/paging/{pageSize}/{size}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "cms_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/design/appdict/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "cms_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/file/flag",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "cms_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/file/flag/appInfo/{appInfoFlag}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "cms_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/file/list/appInfo/{appInfoFlag}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "cms_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/file/list/{id}/next/count",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "cms_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/file/list/{id}/prev/count",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "cms_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/file/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "cms_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/fileinfo/download/document/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "cms_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/fileinfo/list/all",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "cms_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/fileinfo/list/document/{documentId}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "cms_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/fileinfo/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "cms_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/fileinfo/{id}/document/{docId}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "cms_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/fileinfo/{id}/mockdeletetoget",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "cms_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/form/list/all",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "cms_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/form/list/app/{appId}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "cms_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/form/list/formfield/appInfo/{appId}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "cms_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/form/list/{id}/formfield",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "cms_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/form/v2/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "cms_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/form/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "cms_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/log/filter/list/{id}/next/count",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "cms_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/log/filter/list/{id}/prev/count",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "cms_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/log/list/app/{appId}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "cms_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/log/list/category/{categoryId}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "cms_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/log/list/document/{documentId}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "cms_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/log/list/level/{operationLevel}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "cms_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/log/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "cms_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/output/list",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "cms_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/permission/appInfo/{id}/manageable",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "cms_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/permission/appInfo/{id}/managers",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "cms_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/permission/appInfo/{id}/publishers",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "cms_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/permission/appInfo/{id}/viewers",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "cms_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/permission/category/{id}/managers",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "cms_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/permission/category/{id}/publishers",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "cms_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/permission/category/{id}/viewers",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "cms_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/permission/categoryInfo/{id}/manageable",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "cms_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/permission/management/refresh/all",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "cms_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/permission/manager/appInfo/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "cms_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/permission/manager/categoryInfo/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "cms_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/permission/publisher/appInfo/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "cms_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/permission/publisher/categoryInfo/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "cms_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/permission/viewer/appInfo/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "cms_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/permission/viewer/categoryInfo/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "cms_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/queryview/flag/{view_flag}/definition/{query_flag}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "cms_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/script/flag/appInfo/{appInfoFlag}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "cms_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/script/list/app/{appId}/name/{name}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "cms_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/script/list/app/{flag}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "cms_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/script/list/manager",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "cms_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/script/list/paging/{pageSize}/{size}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "cms_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/script/list/{id}/next/count",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "cms_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/script/list/{id}/prev/count",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "cms_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/script/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "cms_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/searchfilter/list/archive/filter/category/{categoryId}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "cms_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/searchfilter/list/draft/filter/category/{categoryId}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "cms_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/searchfilter/list/publish/filter/category/{categoryId}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "cms_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/surface/appdict/list/appInfo/{appInfoFlag}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "cms_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/templateform/list",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "cms_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/templateform/list/category",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "cms_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/uuid/random",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "cms_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/view/list/all",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "cms_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/view/list/app/{appId}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "cms_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/view/list/category/{categoryId}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "cms_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/view/list/form/{formId}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "cms_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/view/viewdata/list/{id}/next/count",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "cms_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/view/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "cms_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/viewcategory/list/all",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "cms_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/viewcategory/list/category/{categoryId}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "cms_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/viewcategory/list/view/{viewId}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "cms_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/viewcategory/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "cms_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/viewfieldconfig/list/all",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "cms_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/viewfieldconfig/list/view/{viewId}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "cms_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/viewfieldconfig/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "cms_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/viewrecord/document/{docId}/filter/list/{id}/next/count",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "cms_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/viewrecord/list/install/log/paging/{pageSize}/{size}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "cms_assemble_control",
        method: "GET",
        rust_path: "/jianfo/list/has/document",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "cms_assemble_control",
        method: "POST",
        rust_path: "/jaxrs/categoryinfo/filter/list/page/{size}/mockputtopost",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "cms_assemble_control",
        method: "POST",
        rust_path: "/jaxrs/categoryinfo/filter/list/{id}/next/{count}/app/{appId}/mockputtopost",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "cms_assemble_control",
        method: "POST",
        rust_path: "/jaxrs/categoryinfo/filter/list/{id}/prev/{count}/app/{appId}/mockputtopost",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "cms_assemble_control",
        method: "POST",
        rust_path: "/jaxrs/cms/assemble/control/appinfo/erase/app/mockdeletetoget/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "cms_assemble_control",
        method: "POST",
        rust_path: "/jaxrs/cms/assemble/control/appinfo/erase/app/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "cms_assemble_control",
        method: "POST",
        rust_path: "/jaxrs/cms/assemble/control/appinfo/get/user/publish/{appId}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "cms_assemble_control",
        method: "POST",
        rust_path: "/jaxrs/cms/assemble/control/appinfo/mockdeletetoget/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "cms_assemble_control",
        method: "POST",
        rust_path: "/jaxrs/cms/assemble/control/categoryinfo/bind/view/mockputtopost/{categoryId}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "cms_assemble_control",
        method: "POST",
        rust_path: "/jaxrs/cms/assemble/control/categoryinfo/bind/view/{categoryId}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "cms_assemble_control",
        method: "POST",
        rust_path: "/jaxrs/cms/assemble/control/categoryinfo/erase/mockdeletetoget/{category}/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "cms_assemble_control",
        method: "POST",
        rust_path: "/jaxrs/cms/assemble/control/categoryinfo/erase/{category}/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "cms_assemble_control",
        method: "POST",
        rust_path: "/jaxrs/cms/assemble/control/categoryinfo/mockdeletetoget/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "cms_assemble_control",
        method: "POST",
        rust_path: "/jaxrs/cms/assemble/control/comment/mockdeletetoget/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "cms_assemble_control",
        method: "POST",
        rust_path: "/jaxrs/cms/assemble/control/correlation/doc/delete/{docId}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "cms_assemble_control",
        method: "POST",
        rust_path: "/jaxrs/cms/assemble/control/correlation/update/doc/{docId}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "cms_assemble_control",
        method: "POST",
        rust_path: "/jaxrs/cms/assemble/control/design/appdict/mockdeletetoget/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "cms_assemble_control",
        method: "POST",
        rust_path: "/jaxrs/cms/assemble/control/document/cipher/publish/content",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "cms_assemble_control",
        method: "POST",
        rust_path: "/jaxrs/cms/assemble/control/document/cipher/publish/content/mockputtopost",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "cms_assemble_control",
        method: "POST",
        rust_path: "/jaxrs/cms/assemble/control/file/mockdeletetoget/{flag}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "cms_assemble_control",
        method: "POST",
        rust_path: "/jaxrs/cms/assemble/control/file/upload/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "cms_assemble_control",
        method: "POST",
        rust_path: "/jaxrs/cms/assemble/control/fileinfo/update/content/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "cms_assemble_control",
        method: "POST",
        rust_path: "/jaxrs/cms/assemble/control/fileinfo/update/document/attachment/callback/callback/{docId}/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "cms_assemble_control",
        method: "POST",
        rust_path: "/jaxrs/cms/assemble/control/fileinfo/update/document/attachment/{docId}/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "cms_assemble_control",
        method: "POST",
        rust_path: "/jaxrs/cms/assemble/control/fileinfo/upload/doc/save/as/{docId}/{flag}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "cms_assemble_control",
        method: "POST",
        rust_path: "/jaxrs/cms/assemble/control/fileinfo/upload/document/callback/callback/{docId}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "cms_assemble_control",
        method: "POST",
        rust_path: "/jaxrs/cms/assemble/control/fileinfo/upload/with/url",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "cms_assemble_control",
        method: "POST",
        rust_path: "/jaxrs/cms/assemble/control/form/mockdeletetoget/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "cms_assemble_control",
        method: "POST",
        rust_path: "/jaxrs/cms/assemble/control/script/mockdeletetoget/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "cms_assemble_control",
        method: "POST",
        rust_path: "/jaxrs/cms/assemble/control/surface/appdict/path0/data/mockdeletetoget/{appDictFlag}/{appInfo}/{appInfoFlag}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "cms_assemble_control",
        method: "POST",
        rust_path: "/jaxrs/cms/assemble/control/surface/appdict/path0/path1/data/mockdeletetoget/{appDictFlag}/{appInfo}/{appInfoFlag}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "cms_assemble_control",
        method: "POST",
        rust_path: "/jaxrs/cms/assemble/control/surface/appdict/path0/path1/path2/data/mockdeletetoget/{appDictFlag}/{appInfo}/{appInfoFlag}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "cms_assemble_control",
        method: "POST",
        rust_path: "/jaxrs/cms/assemble/control/surface/appdict/path0/path1/path2/path3/data/mockdeletetoget/{appDictFlag}/{appInfo}/{appInfoFlag}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "cms_assemble_control",
        method: "POST",
        rust_path: "/jaxrs/cms/assemble/control/surface/appdict/path0/path1/path2/path3/path4/data/mockdeletetoget/{appDictFlag}/{appInfo}/{appInfoFlag}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "cms_assemble_control",
        method: "POST",
        rust_path: "/jaxrs/cms/assemble/control/surface/appdict/path0/path1/path2/path3/path4/path5/data/mockdeletetoget/{appDictFlag}/{appInfo}/{appInfoFlag}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "cms_assemble_control",
        method: "POST",
        rust_path: "/jaxrs/cms/assemble/control/surface/appdict/path0/path1/path2/path3/path4/path5/path6/data/mockdeletetoget/{appDictFlag}/{appInfo}/{appInfoFlag}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "cms_assemble_control",
        method: "POST",
        rust_path: "/jaxrs/cms/assemble/control/surface/appdict/path0/path1/path2/path3/path4/path5/path6/path7/data/mockdeletetoget/{appDictFlag}/{appInfo}/{appInfoFlag}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "cms_assemble_control",
        method: "POST",
        rust_path: "/jaxrs/cms/assemble/control/templateform/mockdeletetoget/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "cms_assemble_control",
        method: "POST",
        rust_path: "/jaxrs/cms/assemble/control/view/mockdeletetoget/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "cms_assemble_control",
        method: "POST",
        rust_path: "/jaxrs/cms/assemble/control/viewcategory/mockdeletetoget/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "cms_assemble_control",
        method: "POST",
        rust_path: "/jaxrs/cms/assemble/control/viewfieldconfig/mockdeletetoget/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "cms_assemble_control",
        method: "POST",
        rust_path: "/jaxrs/cms_assemble_control/anonymous/document/filter/list/id/next/count/mockputtopost",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "cms_assemble_control",
        method: "POST",
        rust_path: "/jaxrs/cms_assemble_control/anonymous/document/filter/list/page/size/size/mockputtopost",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "cms_assemble_control",
        method: "POST",
        rust_path: "/jaxrs/cms_assemble_control/appinfo/filter/list/id/next/count/mockputtopost",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "cms_assemble_control",
        method: "POST",
        rust_path: "/jaxrs/cms_assemble_control/appinfo/filter/list/id/prev/count/mockputtopost",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "cms_assemble_control",
        method: "POST",
        rust_path: "/jaxrs/data/document/{id}/mockputtopost",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "cms_assemble_control",
        method: "POST",
        rust_path: "/jaxrs/data/document/{id}/path0/mockputtopost",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "cms_assemble_control",
        method: "POST",
        rust_path: "/jaxrs/data/document/{id}/path0/path1/mockputtopost",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "cms_assemble_control",
        method: "POST",
        rust_path: "/jaxrs/data/document/{id}/path0/path1/path2/mockputtopost",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "cms_assemble_control",
        method: "POST",
        rust_path: "/jaxrs/data/document/{id}/path0/path1/path2/path3/mockputtopost",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "cms_assemble_control",
        method: "POST",
        rust_path: "/jaxrs/data/document/{id}/path0/path1/path2/path3/path4/mockputtopost",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "cms_assemble_control",
        method: "POST",
        rust_path: "/jaxrs/data/document/{id}/path0/path1/path2/path3/path4/path5/mockputtopost",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "cms_assemble_control",
        method: "POST",
        rust_path: "/jaxrs/data/document/{id}/path0/path1/path2/path3/path4/path5/path6/mockputtopost",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "cms_assemble_control",
        method: "POST",
        rust_path: "/jaxrs/data/document/{id}/path0/path1/path2/path3/path4/path5/path6/path7/mockputtopost",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "cms_assemble_control",
        method: "POST",
        rust_path: "/jaxrs/document/cipher/filter/list/page/{size}/mockputtopost",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "cms_assemble_control",
        method: "POST",
        rust_path: "/jaxrs/document/{id}/view/count",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "cms_assemble_control",
        method: "POST",
        rust_path: "/jaxrs/fileinfo/upload/document/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "cms_assemble_control",
        method: "POST",
        rust_path: "/jaxrs/form/filter/list/{id}/next/count/app/{appId}/mockputtopost",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "cms_assemble_control",
        method: "POST",
        rust_path: "/jaxrs/form/filter/list/{id}/prev/count/app/{appId}/mockputtopost",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "cms_assemble_control",
        method: "POST",
        rust_path: "/jaxrs/image/encode/base64",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "cms_assemble_control",
        method: "POST",
        rust_path: "/jaxrs/image/encode/base64/{size}/{size}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "cms_assemble_control",
        method: "POST",
        rust_path: "/jaxrs/image/resize/{id}/{id}/width/{width}/{height}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "cms_assemble_control",
        method: "POST",
        rust_path: "/jaxrs/input/compare/mockputtopost",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "cms_assemble_control",
        method: "POST",
        rust_path: "/jaxrs/input/cover/mockputtopost",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "cms_assemble_control",
        method: "POST",
        rust_path: "/jaxrs/input/create/mockputtopost",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "cms_assemble_control",
        method: "POST",
        rust_path: "/jaxrs/input/prepare/cover/mockputtopost",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "cms_assemble_control",
        method: "POST",
        rust_path: "/jaxrs/input/prepare/create/mockputtopost",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "cms_assemble_control",
        method: "POST",
        rust_path: "/jaxrs/templateform/list/category/mockputtopost",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "cms_assemble_control",
        method: "PUT",
        rust_path: "/jaxrs/categoryinfo/filter/list/page/{size}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "cms_assemble_control",
        method: "PUT",
        rust_path: "/jaxrs/categoryinfo/filter/list/{id}/next/{count}/app/{appId}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "cms_assemble_control",
        method: "PUT",
        rust_path: "/jaxrs/categoryinfo/filter/list/{id}/prev/{count}/app/{appId}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "cms_assemble_control",
        method: "PUT",
        rust_path: "/jaxrs/document/cipher/filter/list/page/{size}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "cms_assemble_control",
        method: "PUT",
        rust_path: "/jaxrs/fileinfo/list/filter",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "cms_assemble_control",
        method: "PUT",
        rust_path: "/jaxrs/form/filter/list/{id}/next/count/app/{appId}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "cms_assemble_control",
        method: "PUT",
        rust_path: "/jaxrs/form/filter/list/{id}/prev/count/app/{appId}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "cms_assemble_control",
        method: "PUT",
        rust_path: "/jaxrs/input/compare",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "cms_assemble_control",
        method: "PUT",
        rust_path: "/jaxrs/input/cover",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "cms_assemble_control",
        method: "PUT",
        rust_path: "/jaxrs/input/create",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "cms_assemble_control",
        method: "PUT",
        rust_path: "/jaxrs/input/prepare/cover",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "cms_assemble_control",
        method: "PUT",
        rust_path: "/jaxrs/input/prepare/create",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "cms_assemble_control",
        method: "PUT",
        rust_path: "/jaxrs/log/list/filter/page/{size}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "cms_control",
        method: "GET",
        rust_path: "/jaxrs/cms_control/get/control/config",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "cms_control",
        method: "GET",
        rust_path: "/jaxrs/cms_control/list/control/sections",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "cms_core_entity",
        method: "GET",
        rust_path: "/jaxrs/cms/article/list",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "cms_core_entity",
        method: "GET",
        rust_path: "/jaxrs/cms/article/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "cms_core_entity",
        method: "GET",
        rust_path: "/jaxrs/cms/category/list",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "cms_core_entity",
        method: "GET",
        rust_path: "/jaxrs/cms/category/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "cms_core_entity",
        method: "POST",
        rust_path: "/jaxrs/cms/article/create",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "cms_core_entity",
        method: "POST",
        rust_path: "/jaxrs/cms/category/create",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "cms_core_express",
        method: "GET",
        rust_path: "/jaxrs/cms/core/express/content/detail/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "cms_core_express",
        method: "GET",
        rust_path: "/jaxrs/cms/core/express/content/list",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "cms_express",
        method: "GET",
        rust_path: "/jaxrs/cms/templateform/list",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "cms_express",
        method: "GET",
        rust_path: "/jaxrs/cms/uuid/random",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "cms_express",
        method: "GET",
        rust_path: "/jaxrs/cms/view/list/all",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "cms_express",
        method: "POST",
        rust_path: "/jaxrs/cms/view/publish/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "cms_express",
        method: "POST",
        rust_path: "/jaxrs/cms/view/unpublish/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "component",
        method: "GET",
        rust_path: "/jaxrs/component/count",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "component",
        method: "GET",
        rust_path: "/jaxrs/component/list/all",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "component",
        method: "GET",
        rust_path: "/jaxrs/component/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "component_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/component/assemble/control/status/list",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "component_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/component_assemble_control/create/component",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "component_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/component_assemble_control/delete/component",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "component_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/component_assemble_control/get/component",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "component_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/component_assemble_control/get/control/config",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "component_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/component_assemble_control/list/components",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "component_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/component_assemble_control/list/control/categories",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "component_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/component_assemble_control/save/component",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "component_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/component_assemble_control/update/control/config",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "component_assemble_control",
        method: "POST",
        rust_path: "/jaxrs/component/assemble/control/component/delete/all",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "component_core_entity",
        method: "GET",
        rust_path: "/jaxrs/component/core/entity/count",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "component_core_entity",
        method: "GET",
        rust_path: "/jaxrs/component/core/entity/list/all",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "component_core_entity",
        method: "GET",
        rust_path: "/jaxrs/component/core/entity/{flag}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "console",
        method: "GET",
        rust_path: "/jaxrs/console/logs/{type}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "console",
        method: "GET",
        rust_path: "/jaxrs/console/metric/{name}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "console",
        method: "GET",
        rust_path: "/jaxrs/console/status",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "console",
        method: "GET",
        rust_path: "/jaxrs/console/system/info",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "console",
        method: "POST",
        rust_path: "/jaxrs/console/cache/clear/{type}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "console",
        method: "POST",
        rust_path: "/jaxrs/console/command/execute",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "console",
        method: "POST",
        rust_path: "/jaxrs/console/send/message",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "control",
        method: "DELETE",
        rust_path: "/jaxrs/group/{flag}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "control",
        method: "DELETE",
        rust_path: "/jaxrs/person/{flag}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "control",
        method: "DELETE",
        rust_path: "/jaxrs/role/{flag}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "control",
        method: "DELETE",
        rust_path: "/jaxrs/unit/{flag}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "control",
        method: "GET",
        rust_path: "/jaxrs/group/list/{flag}/next/{count}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "control",
        method: "GET",
        rust_path: "/jaxrs/group/list/{flag}/prev/{count}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "control",
        method: "GET",
        rust_path: "/jaxrs/group/{flag}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "control",
        method: "GET",
        rust_path: "/jaxrs/person/list/{flag}/next/{count}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "control",
        method: "GET",
        rust_path: "/jaxrs/person/list/{flag}/prev/{count}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "control",
        method: "GET",
        rust_path: "/jaxrs/person/{flag}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "control",
        method: "GET",
        rust_path: "/jaxrs/role/list/{flag}/next/{count}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "control",
        method: "GET",
        rust_path: "/jaxrs/role/list/{flag}/prev/{count}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "control",
        method: "GET",
        rust_path: "/jaxrs/role/{flag}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "control",
        method: "GET",
        rust_path: "/jaxrs/unit/list",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "control",
        method: "GET",
        rust_path: "/jaxrs/unit/list/{flag}/next/{count}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "control",
        method: "GET",
        rust_path: "/jaxrs/unit/list/{flag}/prev/{count}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "control",
        method: "GET",
        rust_path: "/jaxrs/unit/{flag}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "control",
        method: "POST",
        rust_path: "/jaxrs/group",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "control",
        method: "POST",
        rust_path: "/jaxrs/person",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "control",
        method: "POST",
        rust_path: "/jaxrs/role",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "control",
        method: "POST",
        rust_path: "/jaxrs/unit",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "control",
        method: "PUT",
        rust_path: "/jaxrs/group/{flag}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "control",
        method: "PUT",
        rust_path: "/jaxrs/person/{flag}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "control",
        method: "PUT",
        rust_path: "/jaxrs/role/{flag}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "control",
        method: "PUT",
        rust_path: "/jaxrs/unit/{flag}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "correlation",
        method: "GET",
        rust_path: "/jaxrs/correlation/type/cms/list",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "correlation",
        method: "GET",
        rust_path: "/jaxrs/correlation/type/cms/readable",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "correlation",
        method: "GET",
        rust_path: "/jaxrs/correlation/type/processplatform/list",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "correlation_core_entity",
        method: "DELETE",
        rust_path: "/jaxrs/correlation/core/entity/delete/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "correlation_core_entity",
        method: "GET",
        rust_path: "/jaxrs/correlation/core/entity/list",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "correlation_core_entity",
        method: "GET",
        rust_path: "/jaxrs/correlation/core/entity/list/by/{sourceType}/{sourceId}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "correlation_core_entity",
        method: "POST",
        rust_path: "/jaxrs/correlation/core/entity/create",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "correlation_core_express",
        method: "GET",
        rust_path: "/jaxrs/correlation/core/express/status",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "correlation_core_express",
        method: "GET",
        rust_path: "/jaxrs/correlation/core/express/sync",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "correlation_service_processing",
        method: "GET",
        rust_path: "/jaxrs/correlation/service/processing/correlation/list/type/cms/document/{document}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "correlation_service_processing",
        method: "GET",
        rust_path: "/jaxrs/correlation/service/processing/correlation/list/type/cms/document/{document}/site/{site}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "correlation_service_processing",
        method: "GET",
        rust_path: "/jaxrs/correlation/service/processing/correlation/list/type/processplatform/job/{job}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "correlation_service_processing",
        method: "GET",
        rust_path: "/jaxrs/correlation/service/processing/correlation/list/type/processplatform/job/{job}/site/{site}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "correlation_service_processing",
        method: "GET",
        rust_path: "/jaxrs/correlation/service/processing/correlation/readable/type/cms",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "correlation_service_processing",
        method: "GET",
        rust_path: "/jaxrs/correlation/service/processing/correlation/readable/type/processplatform",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "correlation_service_processing",
        method: "GET",
        rust_path: "/jaxrs/correlation/service/processing/correlation/type/cms/document/{document}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "correlation_service_processing",
        method: "GET",
        rust_path: "/jaxrs/correlation/service/processing/correlation/type/processplatform/job/{job}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "correlation_service_processing",
        method: "GET",
        rust_path: "/jaxrs/correlation/service/processing/link/{sourceType}/{sourceId}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "correlation_service_processing",
        method: "GET",
        rust_path: "/jaxrs/correlation/service/processing/list/{personId}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "correlation_service_processing",
        method: "GET",
        rust_path: "/jaxrs/correlation/service/processing/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "correlation_service_processing",
        method: "POST",
        rust_path: "/jaxrs/correlation/service/processing/correlation/delete/type/cms/document/{document}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "correlation_service_processing",
        method: "POST",
        rust_path: "/jaxrs/correlation/service/processing/correlation/delete/type/processplatform/job/{job}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "correlation_service_processing",
        method: "POST",
        rust_path: "/jaxrs/correlation/service/processing/correlation/update/type/cms/document/{document}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "correlation_service_processing",
        method: "POST",
        rust_path: "/jaxrs/correlation/service/processing/correlation/update/type/processplatform/job/{job}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "correlation_service_processing",
        method: "POST",
        rust_path: "/jaxrs/correlation/service/processing/create",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "correlation_service_processing",
        method: "POST",
        rust_path: "/jaxrs/correlation/service/processing/delete/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "correlation_service_processing",
        method: "POST",
        rust_path: "/jaxrs/correlation/service/processing/link",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "correlation_service_processing",
        method: "POST",
        rust_path: "/jaxrs/correlation/service/processing/save/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "correlation_service_processing",
        method: "POST",
        rust_path: "/jaxrs/correlation/service/processing/unlink/{sourceType}/{sourceId}/{targetType}/{targetId}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "express",
        method: "GET",
        rust_path: "/jaxrs/express/companies",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "express",
        method: "GET",
        rust_path: "/jaxrs/express/query",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "express",
        method: "POST",
        rust_path: "/jaxrs/express/group/list",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "express",
        method: "POST",
        rust_path: "/jaxrs/express/identity/list",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "express",
        method: "POST",
        rust_path: "/jaxrs/express/person/list",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "express",
        method: "POST",
        rust_path: "/jaxrs/express/person/with/identity",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "express",
        method: "POST",
        rust_path: "/jaxrs/express/person/with/unit",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "express",
        method: "POST",
        rust_path: "/jaxrs/express/role/list",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "express",
        method: "POST",
        rust_path: "/jaxrs/express/subscribe",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "express",
        method: "POST",
        rust_path: "/jaxrs/express/unit/list",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "file",
        method: "GET",
        rust_path: "/jaxrs/file/complex/top",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "file",
        method: "GET",
        rust_path: "/jaxrs/file/download/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "file",
        method: "GET",
        rust_path: "/jaxrs/file/folder/list/top",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "file",
        method: "GET",
        rust_path: "/jaxrs/file/folder/list/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "file",
        method: "POST",
        rust_path: "/jaxrs/file/folder/create",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "file",
        method: "POST",
        rust_path: "/jaxrs/file/folder/remove",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "file",
        method: "POST",
        rust_path: "/jaxrs/file/folder/update",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "file",
        method: "POST",
        rust_path: "/jaxrs/file/permission/set",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "file",
        method: "POST",
        rust_path: "/jaxrs/file/upload",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "file_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/anonymous/file/{id}/download/stream",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "file_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/attachment/download/{attid}/stream",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "file_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/file/anonymous/file/id/download",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "file_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/file/assemble/control/attachment2/{id}/office/preview/type/{type}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "file_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/file/assemble/control/file/list/{folderId}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "file_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/file/assemble/control/file/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "file_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/file/attachment/id",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "file_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/file/attachment/id/binary/base64",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "file_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/file/attachment/id/download",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "file_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/file/attachment/id/image/scale/scale/binary/base64",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "file_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/file/attachment/id/image/width/width/height/height/binary/base64",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "file_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/file/attachment/list/folder/folderId",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "file_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/file/attachment/list/top",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "file_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/file/attachment2/exist/file/fileMd5",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "file_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/file/attachment2/id",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "file_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/file/attachment2/id/binary/base64",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "file_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/file/attachment2/id/download",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "file_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/file/attachment2/id/download/image/width/width/height/height",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "file_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/file/attachment2/id/download/stream",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "file_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/file/attachment2/id/image/scale/scale/binary/base64",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "file_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/file/attachment2/id/image/width/width/height/height/binary/base64",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "file_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/file/attachment2/list/filter/name",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "file_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/file/attachment2/list/folder/folderId",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "file_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/file/attachment2/list/top",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "file_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/file/attachment2/list/type/page/size/size",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "file_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/file/complex/folder/id",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "file_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/file/complex/top",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "file_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/file/editor/list",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "file_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/file/file/clean/unused/referencetype/cmsdocument/manage",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "file_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/file/file/copy/attachment/attachmentId/referencetype/referenceType/reference/reference/scale/scale",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "file_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/file/file/id",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "file_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/file/file/id/binary/base64",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "file_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/file/file/id/download",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "file_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/file/file/list/id/next/count",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "file_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/file/file/list/id/next/count/all",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "file_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/file/file/list/id/next/count/referencetype/referenceType",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "file_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/file/file/list/id/prev/count",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "file_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/file/file/list/id/prev/count/all",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "file_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/file/file/list/id/prev/count/referencetype/referenceType",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "file_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/file/file/list/referencetype",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "file_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/file/file/list/referencetype/referenceType/reference/reference",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "file_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/file/file/list/unused/referencetype/cmsdocument/manage",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "file_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/file/file/referencetype/referenceType/reference/reference",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "file_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/file/folder/id",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "file_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/file/folder/list/id",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "file_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/file/folder/list/top",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "file_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/file/folder2/batch/download",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "file_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/file/folder2/id",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "file_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/file/folder2/id/download",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "file_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/file/folder2/list/id",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "file_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/file/folder2/list/top",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "file_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/file/recycle/id",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "file_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/file/share/download/share/shareId/file/fileId",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "file_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/file/share/id",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "file_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/file/share/id/password/password",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "file_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/file/share/list/att/share/shareId/folder/folderId",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "file_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/file/share/list/folder/share/shareId/folder/folderId",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "file_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/file/share/share/shareId/file/fileId/folder/folderId",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "file_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/file/share/shield/id",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "file_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/file/{id}/download/stream",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "file_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/share/list/my2/{shareType}/{fileType}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "file_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/share/list/to/me2/{fileType}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "file_assemble_control",
        method: "POST",
        rust_path: "/jaxrs/attachment/update/callback/callback/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "file_assemble_control",
        method: "POST",
        rust_path: "/jaxrs/attachment/update/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "file_assemble_control",
        method: "POST",
        rust_path: "/jaxrs/attachment/upload/folder/callback/callback/{folderId}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "file_assemble_control",
        method: "POST",
        rust_path: "/jaxrs/attachment/upload/folder/{folderId}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "file_assemble_control",
        method: "POST",
        rust_path: "/jaxrs/attachment2/upload/folder/{folderId}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "file_assemble_control",
        method: "POST",
        rust_path: "/jaxrs/file/assemble/control/file/create",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "file_assemble_control",
        method: "POST",
        rust_path: "/jaxrs/file/assemble/control/file/delete/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "file_assemble_control",
        method: "POST",
        rust_path: "/jaxrs/file/assemble/control/file/upload",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "file_assemble_control",
        method: "POST",
        rust_path: "/jaxrs/file/core/entity/file/create",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "file_assemble_control",
        method: "POST",
        rust_path: "/jaxrs/file/core/entity/file/delete/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "file_assemble_control",
        method: "POST",
        rust_path: "/jaxrs/file/core/entity/file/update/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "file_assemble_control",
        method: "POST",
        rust_path: "/jaxrs/file/upload/referencetype/reference/reference/scale/scale/callback/callback/{referenceType}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "file_assemble_control",
        method: "POST",
        rust_path: "/jaxrs/file/upload/referencetype/reference/reference/scale/scale/{referenceType}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "file_assemble_control",
        method: "POST",
        rust_path: "/jaxrs/recycle/delete/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "file_assemble_control",
        method: "POST",
        rust_path: "/jaxrs/recycle/resume/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "file_core_entity",
        method: "DELETE",
        rust_path: "/jaxrs/file/core/entity/folder/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "file_core_entity",
        method: "GET",
        rust_path: "/jaxrs/file/core/entity/complex/top",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "file_core_entity",
        method: "GET",
        rust_path: "/jaxrs/file/core/entity/file/list",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "file_core_entity",
        method: "GET",
        rust_path: "/jaxrs/file/core/entity/folder/list/top",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "file_core_entity",
        method: "GET",
        rust_path: "/jaxrs/file/core/entity/folder/list/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "file_core_entity",
        method: "POST",
        rust_path: "/jaxrs/file/core/entity/file",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "file_core_entity",
        method: "POST",
        rust_path: "/jaxrs/file/core/entity/folder",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "general_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/general/assemble/control/area/list",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "general_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/general/assemble/control/area/list/province/{province}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "general_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/general/assemble/control/area/list/province/{province}/city/{city}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "general_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/general/assemble/control/area/list/province/{province}/city/{city}/district/{district}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "general_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/general/assemble/control/area/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "general_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/general/assemble/control/attendscope/list",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "general_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/general/assemble/control/attendscope/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "general_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/general/assemble/control/ecnet/check",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "general_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/general/assemble/control/excel/result/flag/{flag}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "general_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/general/assemble/control/excel/{excelName}/{excelName}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "general_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/general/assemble/control/excel/{excelName}/{excelName}/{sheetList}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "general_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/general/assemble/control/generalfile/download/flag/{flag}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "general_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/general/assemble/control/generalfile/flag/{flag}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "general_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/general/assemble/control/generalfile/flag/{flag}/binary/base64",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "general_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/general/assemble/control/invoice/download/flag/{flag}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "general_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/general/assemble/control/invoice/get/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "general_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/general/assemble/control/invoice/list/paging/{page}/size/{size}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "general_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/general/assemble/control/office/html/to/word/result/flag/{flag}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "general_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/general/assemble/control/permissions/{module}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "general_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/general/assemble/control/qrcode/list",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "general_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/general/assemble/control/qrcode/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "general_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/general/assemble/control/securityclearance/object",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "general_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/general/assemble/control/securityclearance/subject",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "general_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/general/assemble/control/securityclearance/system",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "general_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/general/assemble/control/securityclearance/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "general_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/general/assemble/control/status",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "general_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/general/assemble/control/upgrade/2021090901",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "general_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/general/assemble/control/upgrade/2021090902",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "general_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/general/assemble/control/worktime/between/holiday/count/start/{startDate}/end/{endDate}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "general_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/general/assemble/control/worktime/between/minutes/start/{start}/end/{end}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "general_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/general/assemble/control/worktime/forward/days/start/{start}/days/{days}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "general_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/general/assemble/control/worktime/forward/minutes/start/{start}/minutes/{minutes}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "general_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/general/assemble/control/worktime/indefined/holiday/{date}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "general_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/general/assemble/control/worktime/indefined/workday/{date}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "general_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/general/assemble/control/worktime/is/holiday/{date}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "general_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/general/assemble/control/worktime/is/workday/{date}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "general_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/general/assemble/control/worktime/is/worktime/{date}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "general_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/general/assemble/control/worktime/minutes/of/workday",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "general_assemble_control",
        method: "POST",
        rust_path: "/jaxrs/general/assemble/control/area/create",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "general_assemble_control",
        method: "POST",
        rust_path: "/jaxrs/general/assemble/control/area/delete/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "general_assemble_control",
        method: "POST",
        rust_path: "/jaxrs/general/assemble/control/area/update/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "general_assemble_control",
        method: "POST",
        rust_path: "/jaxrs/general/assemble/control/attendscope/create",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "general_assemble_control",
        method: "POST",
        rust_path: "/jaxrs/general/assemble/control/attendscope/delete/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "general_assemble_control",
        method: "POST",
        rust_path: "/jaxrs/general/assemble/control/attendscope/save/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "general_assemble_control",
        method: "POST",
        rust_path: "/jaxrs/general/assemble/control/excel/upload",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "general_assemble_control",
        method: "POST",
        rust_path: "/jaxrs/general/assemble/control/excel/upload/with/url",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "general_assemble_control",
        method: "POST",
        rust_path: "/jaxrs/general/assemble/control/invoice/create",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "general_assemble_control",
        method: "POST",
        rust_path: "/jaxrs/general/assemble/control/invoice/delete/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "general_assemble_control",
        method: "POST",
        rust_path: "/jaxrs/general/assemble/control/invoice/update/apply/status/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "general_assemble_control",
        method: "POST",
        rust_path: "/jaxrs/general/assemble/control/invoice/update/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "general_assemble_control",
        method: "POST",
        rust_path: "/jaxrs/general/assemble/control/invoice/upload",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "general_assemble_control",
        method: "POST",
        rust_path: "/jaxrs/general/assemble/control/invoice/upload/for/create",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "general_assemble_control",
        method: "POST",
        rust_path: "/jaxrs/general/assemble/control/invoice/upload/with/url",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "general_assemble_control",
        method: "POST",
        rust_path: "/jaxrs/general/assemble/control/office/html/to/word",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "general_assemble_control",
        method: "POST",
        rust_path: "/jaxrs/general/assemble/control/qrcode/delete/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "general_assemble_control",
        method: "POST",
        rust_path: "/jaxrs/general/assemble/control/qrcode/width/{width}/height/{height}/text/{text}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "general_assemble_control",
        method: "POST",
        rust_path: "/jaxrs/general/assemble/control/securityclearance/create",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "general_assemble_control",
        method: "POST",
        rust_path: "/jaxrs/general/assemble/control/securityclearance/delete/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "general_assemble_control",
        method: "POST",
        rust_path: "/jaxrs/general/assemble/control/securityclearance/enable",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "general_assemble_control",
        method: "POST",
        rust_path: "/jaxrs/general/assemble/control/securityclearance/update/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "general_assemble_control",
        method: "POST",
        rust_path: "/jaxrs/general/assemble/control/status/update",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "general_core_entity",
        method: "GET",
        rust_path: "/jaxrs/general/dict/item/list/{dictId}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "general_core_entity",
        method: "GET",
        rust_path: "/jaxrs/general/dict/item/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "general_core_entity",
        method: "GET",
        rust_path: "/jaxrs/general/dict/list",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "general_core_entity",
        method: "GET",
        rust_path: "/jaxrs/general/dict/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "general_core_entity",
        method: "GET",
        rust_path: "/jaxrs/general/file/download/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "general_core_entity",
        method: "GET",
        rust_path: "/jaxrs/general/file/list",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "general_core_entity",
        method: "GET",
        rust_path: "/jaxrs/general/file/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "general_core_entity",
        method: "GET",
        rust_path: "/jaxrs/general/invoice/list",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "general_core_entity",
        method: "GET",
        rust_path: "/jaxrs/general/invoice/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "general_core_entity",
        method: "POST",
        rust_path: "/jaxrs/general/dict/create",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "general_core_entity",
        method: "POST",
        rust_path: "/jaxrs/general/dict/delete/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "general_core_entity",
        method: "POST",
        rust_path: "/jaxrs/general/dict/item/create",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "general_core_entity",
        method: "POST",
        rust_path: "/jaxrs/general/dict/item/delete/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "general_core_entity",
        method: "POST",
        rust_path: "/jaxrs/general/dict/item/update/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "general_core_entity",
        method: "POST",
        rust_path: "/jaxrs/general/dict/update/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "general_core_entity",
        method: "POST",
        rust_path: "/jaxrs/general/file/create",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "general_core_entity",
        method: "POST",
        rust_path: "/jaxrs/general/file/delete/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "general_core_entity",
        method: "POST",
        rust_path: "/jaxrs/general/file/update/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "general_core_entity",
        method: "POST",
        rust_path: "/jaxrs/general/invoice/create",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "general_core_entity",
        method: "POST",
        rust_path: "/jaxrs/general/invoice/delete/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "general_core_entity",
        method: "POST",
        rust_path: "/jaxrs/general/invoice/update/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "hotpic",
        method: "GET",
        rust_path: "/jaxrs/hotpic/user/hotpic/exists/check",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "hotpic",
        method: "GET",
        rust_path: "/jaxrs/hotpic/user/hotpic/{application}/{infoId}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "hotpic",
        method: "GET",
        rust_path: "/jaxrs/hotpic/user/hotpic/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "hotpic_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/hotpic/assemble/control/cipher/hotpic/bbs/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "hotpic_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/hotpic/assemble/control/cipher/hotpic/cms/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "hotpic_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/hotpic/assemble/control/cipher/hotpic/filter/list/page/{page}/count/{count}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "hotpic_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/hotpic/assemble/control/cipher/hotpic/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "hotpic_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/hotpic/assemble/control/config",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "hotpic_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/hotpic/assemble/control/list/control/applications",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "hotpic_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/hotpic/assemble/control/list/control/panels",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "hotpic_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/hotpic/assemble/control/user/hotpic/application/{infoId}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "hotpic_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/hotpic/assemble/control/user/hotpic/exists/check",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "hotpic_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/hotpic/assemble/control/user/hotpic/filter/list/page/{page}/count/{count}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "hotpic_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/hotpic/assemble/control/user/hotpic/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "hotpic_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/hotpic/get/hotpic/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "hotpic_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/hotpic/list/hotpics",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "hotpic_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/hotpic_assemble_control/cipher/hotpic/bbs/id",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "hotpic_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/hotpic_assemble_control/cipher/hotpic/cms/id",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "hotpic_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/hotpic_assemble_control/cipher/hotpic/filter/list/page/page/count/count",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "hotpic_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/hotpic_assemble_control/cipher/hotpic/id",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "hotpic_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/hotpic_assemble_control/create/hotpic",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "hotpic_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/hotpic_assemble_control/delete/hotpic",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "hotpic_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/hotpic_assemble_control/get/control/config",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "hotpic_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/hotpic_assemble_control/get/hotpic",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "hotpic_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/hotpic_assemble_control/list/control/applications",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "hotpic_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/hotpic_assemble_control/list/control/panels",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "hotpic_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/hotpic_assemble_control/list/hotpics",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "hotpic_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/hotpic_assemble_control/save/hotpic",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "hotpic_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/hotpic_assemble_control/update/control/config",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "hotpic_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/hotpic_assemble_control/user/hotpic/application/infoId",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "hotpic_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/hotpic_assemble_control/user/hotpic/changeTitle",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "hotpic_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/hotpic_assemble_control/user/hotpic/exists/check",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "hotpic_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/hotpic_assemble_control/user/hotpic/filter/list/page/page/count/count",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "hotpic_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/hotpic_assemble_control/user/hotpic/id",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "hotpic_assemble_control",
        method: "POST",
        rust_path: "/jaxrs/hotpic/assemble/control/update/control/config",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "hotpic_assemble_control",
        method: "POST",
        rust_path: "/jaxrs/hotpic/assemble/control/user/hotpic/changeTitle",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "hotpic_assemble_control",
        method: "POST",
        rust_path: "/jaxrs/hotpic/create/hotpic",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "hotpic_assemble_control",
        method: "POST",
        rust_path: "/jaxrs/hotpic/delete/hotpic",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "hotpic_assemble_control",
        method: "POST",
        rust_path: "/jaxrs/hotpic/save/hotpic",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "hotpic_core_entity",
        method: "DELETE",
        rust_path: "/jaxrs/hotpic/core/entity/delete/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "hotpic_core_entity",
        method: "GET",
        rust_path: "/jaxrs/hotpic/core/entity/exists/check/{application}/{infoId}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "hotpic_core_entity",
        method: "GET",
        rust_path: "/jaxrs/hotpic/core/entity/list",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "hotpic_core_entity",
        method: "GET",
        rust_path: "/jaxrs/hotpic/core/entity/list/by/{application}/{infoId}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "hotpic_core_entity",
        method: "POST",
        rust_path: "/jaxrs/hotpic/core/entity/create",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "jpush",
        method: "GET",
        rust_path: "/hello/world",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "jpush",
        method: "GET",
        rust_path: "/jaxrs/jpush/device/list",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "jpush",
        method: "GET",
        rust_path: "/jaxrs/jpush/device/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "jpush",
        method: "GET",
        rust_path: "/jaxrs/jpush/template/list",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "jpush",
        method: "GET",
        rust_path: "/jaxrs/jpush/template/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "jpush",
        method: "POST",
        rust_path: "/jaxrs/jpush/device/create",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "jpush_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/jpush/assemble/control/config",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "jpush_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/jpush/assemble/control/device/admin/unbind/all/person",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "jpush_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/jpush/assemble/control/device/check/deviceName/deviceType/pushType",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "jpush_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/jpush/assemble/control/device/config/push/type",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "jpush_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/jpush/assemble/control/device/list/pushType",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "jpush_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/jpush/assemble/control/list/control/apps",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "jpush_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/jpush/assemble/control/message/test/send",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "jpush_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/jpush/get/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "jpush_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/jpush/list",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "jpush_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/jpush_assemble_control/create/jpush",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "jpush_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/jpush_assemble_control/delete/jpush",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "jpush_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/jpush_assemble_control/device/admin/unbind/all/person",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "jpush_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/jpush_assemble_control/device/bind",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "jpush_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/jpush_assemble_control/device/check/deviceName/deviceType/pushType",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "jpush_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/jpush_assemble_control/device/config/push/type",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "jpush_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/jpush_assemble_control/device/list/pushType",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "jpush_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/jpush_assemble_control/device/unbind/deviceName/deviceType",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "jpush_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/jpush_assemble_control/device/unbind/new/deviceName/deviceType/pushType",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "jpush_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/jpush_assemble_control/get/control/config",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "jpush_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/jpush_assemble_control/get/jpush",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "jpush_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/jpush_assemble_control/list/control/apps",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "jpush_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/jpush_assemble_control/list/jpushs",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "jpush_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/jpush_assemble_control/message/test/send",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "jpush_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/jpush_assemble_control/save/jpush",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "jpush_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/jpush_assemble_control/update/control/config",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "jpush_assemble_control",
        method: "POST",
        rust_path: "/jaxrs/jpush/assemble/control/device/bind",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "jpush_assemble_control",
        method: "POST",
        rust_path: "/jaxrs/jpush/assemble/control/device/unbind/deviceName/deviceType",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "jpush_assemble_control",
        method: "POST",
        rust_path: "/jaxrs/jpush/assemble/control/device/unbind/new/deviceName/deviceType/pushType",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "jpush_assemble_control",
        method: "POST",
        rust_path: "/jaxrs/jpush/assemble/control/update/control/config",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "jpush_assemble_control",
        method: "POST",
        rust_path: "/jaxrs/jpush/create",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "jpush_assemble_control",
        method: "POST",
        rust_path: "/jaxrs/jpush/delete/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "jpush_assemble_control",
        method: "POST",
        rust_path: "/jaxrs/jpush/save/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "jpush_core_entity",
        method: "GET",
        rust_path: "/jaxrs/jpush/core/entity/device/list",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "jpush_core_entity",
        method: "GET",
        rust_path: "/jaxrs/jpush/core/entity/device/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "jpush_core_entity",
        method: "GET",
        rust_path: "/jaxrs/jpush/core/entity/template/list",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "jpush_core_entity",
        method: "GET",
        rust_path: "/jaxrs/jpush/core/entity/template/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "jpush_core_entity",
        method: "POST",
        rust_path: "/jaxrs/jpush/core/entity/device/create",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "meeting",
        method: "GET",
        rust_path: "/jaxrs/meeting/building/list",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "meeting",
        method: "GET",
        rust_path: "/jaxrs/meeting/list",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "meeting",
        method: "GET",
        rust_path: "/jaxrs/meeting/openmeeting/list/room",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "meeting",
        method: "GET",
        rust_path: "/jaxrs/meeting/room/list",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "meeting",
        method: "GET",
        rust_path: "/jaxrs/meeting/schedule/days/{days}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "meeting",
        method: "GET",
        rust_path: "/jaxrs/meeting/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "meeting",
        method: "POST",
        rust_path: "/jaxrs/meeting/create",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "meeting_assemble_control",
        method: "DELETE",
        rust_path: "/jaxrs/meeting/assemble/control/delete/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "meeting_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/meeting/assemble/control/building/list",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "meeting_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/meeting/assemble/control/building/list/like/pinyin/{key}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "meeting_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/meeting/assemble/control/building/list/like/{key}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "meeting_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/meeting/assemble/control/building/list/pinyininitial/{key}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "meeting_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/meeting/assemble/control/building/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "meeting_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/meeting/assemble/control/config/system/config",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "meeting_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/meeting/assemble/control/list/meeting/controls",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "meeting_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/meeting/assemble/control/meeting/list/applied/completed",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "meeting_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/meeting/assemble/control/meeting/list/applied/processing",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "meeting_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/meeting/assemble/control/meeting/list/applied/wait",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "meeting_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/meeting/assemble/control/meeting/list/apply/{page}/size/{size}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "meeting_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/meeting/assemble/control/meeting/list/coming/day/{count}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "meeting_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/meeting/assemble/control/meeting/list/coming/{month}/{count}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "meeting_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/meeting/assemble/control/meeting/list/forward/monthcount/all/{monthCount}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "meeting_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/meeting/assemble/control/meeting/list/forward/monthcount/{monthCount}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "meeting_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/meeting/assemble/control/meeting/list/invite/{page}/{size}/{size}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "meeting_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/meeting/assemble/control/meeting/list/invited/completed",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "meeting_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/meeting/assemble/control/meeting/list/invited/processing",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "meeting_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/meeting/assemble/control/meeting/list/invited/rejected",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "meeting_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/meeting/assemble/control/meeting/list/invited/wait",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "meeting_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/meeting/assemble/control/meeting/list/wait/accept",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "meeting_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/meeting/assemble/control/meeting/list/wait/confirm",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "meeting_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/meeting/assemble/control/meeting/list/year/{year}/month/{month}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "meeting_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/meeting/assemble/control/meeting/list/year/{year}/month/{month}/all",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "meeting_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/meeting/assemble/control/meeting/list/year/{year}/month/{month}/day/{day}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "meeting_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/meeting/assemble/control/meeting/list/year/{year}/month/{month}/day/{day}/all",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "meeting_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/meeting/assemble/control/meeting/list/{id}/{next}/{count}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "meeting_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/meeting/assemble/control/meeting/list/{year}/{year}/{month}/{month}/{day}/{day}/{roomId}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "meeting_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/meeting/assemble/control/meeting/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "meeting_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/meeting/assemble/control/openmeeting/list/room",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "meeting_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/meeting/assemble/control/room/list",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "meeting_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/meeting/assemble/control/room/list/like/pinyin/{key}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "meeting_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/meeting/assemble/control/room/list/like/{key}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "meeting_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/meeting/assemble/control/room/list/pinyininitial/{key}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "meeting_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/meeting/assemble/control/room/photo/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "meeting_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/meeting/assemble/control/room/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "meeting_assemble_control",
        method: "POST",
        rust_path: "/jaxrs/meeting/assemble/control/building/list/completed/completed/allmeeting/{start}/{start}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "meeting_assemble_control",
        method: "POST",
        rust_path: "/jaxrs/meeting/assemble/control/building/list/completed/completed/room/room/meeting/meeting/{start}/{start}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "meeting_assemble_control",
        method: "POST",
        rust_path: "/jaxrs/meeting/assemble/control/building/list/completed/completed/{start}/{start}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "meeting_assemble_control",
        method: "POST",
        rust_path: "/jaxrs/meeting/assemble/control/config/system/config/manage",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "meeting_assemble_control",
        method: "POST",
        rust_path: "/jaxrs/meeting/assemble/control/create",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "meeting_assemble_control",
        method: "POST",
        rust_path: "/jaxrs/meeting/assemble/control/meeting/checkin/code/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "meeting_assemble_control",
        method: "POST",
        rust_path: "/jaxrs/meeting/assemble/control/meeting/create",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "meeting_assemble_control",
        method: "POST",
        rust_path: "/jaxrs/meeting/assemble/control/meeting/delete/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "meeting_assemble_control",
        method: "POST",
        rust_path: "/jaxrs/meeting/assemble/control/meeting/list/manage/{page}/{size}/{size}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "meeting_assemble_control",
        method: "POST",
        rust_path: "/jaxrs/meeting/assemble/control/meeting/save/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "meeting_assemble_control",
        method: "POST",
        rust_path: "/jaxrs/meeting/assemble/control/meeting/{id}/accept",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "meeting_assemble_control",
        method: "POST",
        rust_path: "/jaxrs/meeting/assemble/control/meeting/{id}/add/invite",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "meeting_assemble_control",
        method: "POST",
        rust_path: "/jaxrs/meeting/assemble/control/meeting/{id}/checkin",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "meeting_assemble_control",
        method: "POST",
        rust_path: "/jaxrs/meeting/assemble/control/meeting/{id}/confirm/allow",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "meeting_assemble_control",
        method: "POST",
        rust_path: "/jaxrs/meeting/assemble/control/meeting/{id}/confirm/deny",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "meeting_assemble_control",
        method: "POST",
        rust_path: "/jaxrs/meeting/assemble/control/meeting/{id}/delete/invite",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "meeting_assemble_control",
        method: "POST",
        rust_path: "/jaxrs/meeting/assemble/control/meeting/{id}/manual/completed",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "meeting_assemble_control",
        method: "POST",
        rust_path: "/jaxrs/meeting/assemble/control/meeting/{id}/modify/completedtime",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "meeting_assemble_control",
        method: "POST",
        rust_path: "/jaxrs/meeting/assemble/control/meeting/{id}/modify/starttime",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "meeting_assemble_control",
        method: "POST",
        rust_path: "/jaxrs/meeting/assemble/control/meeting/{id}/reject",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "meeting_core_entity",
        method: "GET",
        rust_path: "/jaxrs/meeting/core/entity/meeting/list",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "meeting_core_entity",
        method: "GET",
        rust_path: "/jaxrs/meeting/core/entity/meeting/list/by/{roomId}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "meeting_core_entity",
        method: "GET",
        rust_path: "/jaxrs/meeting/core/entity/meeting/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "meeting_core_entity",
        method: "GET",
        rust_path: "/jaxrs/meeting/core/entity/room/list",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "meeting_core_entity",
        method: "GET",
        rust_path: "/jaxrs/meeting/core/entity/room/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "meeting_core_entity",
        method: "POST",
        rust_path: "/jaxrs/meeting/core/entity/meeting/create",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "meeting_core_entity",
        method: "POST",
        rust_path: "/jaxrs/meeting/core/entity/meeting/delete/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "meeting_core_entity",
        method: "POST",
        rust_path: "/jaxrs/meeting/core/entity/meeting/save/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "meeting_core_entity",
        method: "POST",
        rust_path: "/jaxrs/meeting/core/entity/room/create",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "meeting_core_entity",
        method: "POST",
        rust_path: "/jaxrs/meeting/core/entity/room/delete/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "meeting_core_entity",
        method: "POST",
        rust_path: "/jaxrs/meeting/core/entity/room/save/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "message",
        method: "GET",
        rust_path: "/jaxrs/message/consume/list/{consume}/count/{count}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "message",
        method: "GET",
        rust_path: "/jaxrs/message/consume/{id}/type/{type}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "message",
        method: "GET",
        rust_path: "/jaxrs/message/unread/count/{consume}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "message",
        method: "POST",
        rust_path: "/jaxrs/message/custom/create",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "message",
        method: "POST",
        rust_path: "/jaxrs/message/mark_read/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "message_assemble_communicate",
        method: "DELETE",
        rust_path: "/jaxrs/message/assemble/communicate/im/conversation/{id}/group/mockdeletetoget",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "message_assemble_communicate",
        method: "DELETE",
        rust_path: "/jaxrs/message/assemble/communicate/im/conversation/{id}/single/mockdeletetoget",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "message_assemble_communicate",
        method: "DELETE",
        rust_path: "/jaxrs/message/assemble/communicate/mass/{id}/mockdeletetoget",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "message_assemble_communicate",
        method: "GET",
        rust_path: "/jaxrs/message/assemble/communicate/consume/list/{consume}/count/{count}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "message_assemble_communicate",
        method: "GET",
        rust_path: "/jaxrs/message/assemble/communicate/consume/list/{consume}/currentperson/count/{count}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "message_assemble_communicate",
        method: "GET",
        rust_path: "/jaxrs/message/assemble/communicate/consume/list/{consume}/person/{person}/count/{count}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "message_assemble_communicate",
        method: "GET",
        rust_path: "/jaxrs/message/assemble/communicate/consume/type/{type}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "message_assemble_communicate",
        method: "GET",
        rust_path: "/jaxrs/message/assemble/communicate/im/conversation/business/{businessId}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "message_assemble_communicate",
        method: "GET",
        rust_path: "/jaxrs/message/assemble/communicate/im/conversation/list/my",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "message_assemble_communicate",
        method: "GET",
        rust_path: "/jaxrs/message/assemble/communicate/im/conversation/list/with/person",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "message_assemble_communicate",
        method: "GET",
        rust_path: "/jaxrs/message/assemble/communicate/im/conversation/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "message_assemble_communicate",
        method: "GET",
        rust_path: "/jaxrs/message/assemble/communicate/im/conversation/{id}/group",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "message_assemble_communicate",
        method: "GET",
        rust_path: "/jaxrs/message/assemble/communicate/im/conversation/{id}/icon",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "message_assemble_communicate",
        method: "GET",
        rust_path: "/jaxrs/message/assemble/communicate/im/conversation/{id}/single",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "message_assemble_communicate",
        method: "GET",
        rust_path: "/jaxrs/message/assemble/communicate/im/manager/config",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "message_assemble_communicate",
        method: "GET",
        rust_path: "/jaxrs/message/assemble/communicate/im/msg/collection/list/{page}/size/{size}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "message_assemble_communicate",
        method: "GET",
        rust_path: "/jaxrs/message/assemble/communicate/im/msg/download/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "message_assemble_communicate",
        method: "GET",
        rust_path: "/jaxrs/message/assemble/communicate/im/msg/download/{id}/image/width/{width}/height/{height}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "message_assemble_communicate",
        method: "GET",
        rust_path: "/jaxrs/message/assemble/communicate/im/msg/list/object",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "message_assemble_communicate",
        method: "GET",
        rust_path: "/jaxrs/message/assemble/communicate/im/msg/list/{page}/size/{size}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "message_assemble_communicate",
        method: "GET",
        rust_path: "/jaxrs/message/assemble/communicate/instant/currentperson/consumed",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "message_assemble_communicate",
        method: "GET",
        rust_path: "/jaxrs/message/assemble/communicate/instant/currentperson/consumed/all",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "message_assemble_communicate",
        method: "GET",
        rust_path: "/jaxrs/message/assemble/communicate/instant/list/currentperson/consumed/count/{count}/asc",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "message_assemble_communicate",
        method: "GET",
        rust_path: "/jaxrs/message/assemble/communicate/instant/list/currentperson/consumed/count/{count}/desc",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "message_assemble_communicate",
        method: "GET",
        rust_path: "/jaxrs/message/assemble/communicate/instant/list/currentperson/count/{count}/asc",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "message_assemble_communicate",
        method: "GET",
        rust_path: "/jaxrs/message/assemble/communicate/instant/list/currentperson/count/{count}/desc",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "message_assemble_communicate",
        method: "GET",
        rust_path: "/jaxrs/message/assemble/communicate/instant/list/currentperson/noim/count/{count}/desc",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "message_assemble_communicate",
        method: "GET",
        rust_path: "/jaxrs/message/assemble/communicate/instant/list/currentperson/not/consumed/count/{count}/asc",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "message_assemble_communicate",
        method: "GET",
        rust_path: "/jaxrs/message/assemble/communicate/instant/list/currentperson/not/consumed/count/{count}/desc",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "message_assemble_communicate",
        method: "GET",
        rust_path: "/jaxrs/message/assemble/communicate/instant/list/{id}/next/{count}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "message_assemble_communicate",
        method: "GET",
        rust_path: "/jaxrs/message/assemble/communicate/instant/list/{id}/prev/{count}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "message_assemble_communicate",
        method: "GET",
        rust_path: "/jaxrs/message/assemble/communicate/mass/list/{id}/next/{count}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "message_assemble_communicate",
        method: "GET",
        rust_path: "/jaxrs/message/assemble/communicate/mass/list/{id}/prev/{count}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "message_assemble_communicate",
        method: "GET",
        rust_path: "/jaxrs/message/assemble/communicate/mass/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "message_assemble_communicate",
        method: "GET",
        rust_path: "/jaxrs/message/assemble/communicate/message/list/paging/{page}/size/{size}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "message_assemble_communicate",
        method: "GET",
        rust_path: "/jaxrs/message/assemble/communicate/receive/{consume}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "message_assemble_communicate",
        method: "POST",
        rust_path: "/jaxrs/message/assemble/communicate/consume/type/{type}/mockputtopost",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "message_assemble_communicate",
        method: "POST",
        rust_path: "/jaxrs/message/assemble/communicate/consume/{id}/type/{type}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "message_assemble_communicate",
        method: "POST",
        rust_path: "/jaxrs/message/assemble/communicate/im/conversation",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "message_assemble_communicate",
        method: "POST",
        rust_path: "/jaxrs/message/assemble/communicate/im/conversation/mockputtopost",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "message_assemble_communicate",
        method: "POST",
        rust_path: "/jaxrs/message/assemble/communicate/im/conversation/{id}/group/quit/self",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "message_assemble_communicate",
        method: "POST",
        rust_path: "/jaxrs/message/assemble/communicate/im/conversation/{id}/read",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "message_assemble_communicate",
        method: "POST",
        rust_path: "/jaxrs/message/assemble/communicate/im/conversation/{id}/read/mockputtopost",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "message_assemble_communicate",
        method: "POST",
        rust_path: "/jaxrs/message/assemble/communicate/im/conversation/{id}/top/cancel",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "message_assemble_communicate",
        method: "POST",
        rust_path: "/jaxrs/message/assemble/communicate/im/conversation/{id}/top/cancel/mockputtopost",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "message_assemble_communicate",
        method: "POST",
        rust_path: "/jaxrs/message/assemble/communicate/im/conversation/{id}/top/set",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "message_assemble_communicate",
        method: "POST",
        rust_path: "/jaxrs/message/assemble/communicate/im/conversation/{id}/top/set/mockputtopost",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "message_assemble_communicate",
        method: "POST",
        rust_path: "/jaxrs/message/assemble/communicate/im/msg",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "message_assemble_communicate",
        method: "POST",
        rust_path: "/jaxrs/message/assemble/communicate/im/msg/clear",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "message_assemble_communicate",
        method: "POST",
        rust_path: "/jaxrs/message/assemble/communicate/im/msg/collection",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "message_assemble_communicate",
        method: "POST",
        rust_path: "/jaxrs/message/assemble/communicate/im/msg/collection/remove",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "message_assemble_communicate",
        method: "POST",
        rust_path: "/jaxrs/message/assemble/communicate/im/msg/revoke/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "message_assemble_communicate",
        method: "POST",
        rust_path: "/jaxrs/message/assemble/communicate/instant/currentperson/consumed/mockputtopost",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "message_assemble_communicate",
        method: "POST",
        rust_path: "/jaxrs/message/assemble/communicate/mark_read/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "message_assemble_communicate",
        method: "POST",
        rust_path: "/jaxrs/message/assemble/communicate/mass/enable/type",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "message_assemble_communicate",
        method: "POST",
        rust_path: "/jaxrs/message/assemble/communicate/message/custom/create",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "message_assemble_communicate",
        method: "POST",
        rust_path: "/jaxrs/message/assemble/communicate/send",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "message_assemble_communicate",
        method: "PUT",
        rust_path: "/jaxrs/message/assemble/communicate/im/conversation/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "message_core_entity",
        method: "GET",
        rust_path: "/jaxrs/message/core/entity/list",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "message_core_entity",
        method: "GET",
        rust_path: "/jaxrs/message/core/entity/list/by/{consume}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "message_core_entity",
        method: "GET",
        rust_path: "/jaxrs/message/core/entity/unread/count/{consume}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "mind",
        method: "DELETE",
        rust_path: "/jaxrs/mind/folder/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "mind",
        method: "DELETE",
        rust_path: "/jaxrs/mind/mind/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "mind",
        method: "GET",
        rust_path: "/jaxrs/mind/folder/tree/my",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "mind",
        method: "GET",
        rust_path: "/jaxrs/mind/mind/list/{id}/version",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "mind",
        method: "GET",
        rust_path: "/jaxrs/mind/mind/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "mind",
        method: "POST",
        rust_path: "/jaxrs/mind/folder",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "mind",
        method: "POST",
        rust_path: "/jaxrs/mind/folder/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "mind",
        method: "POST",
        rust_path: "/jaxrs/mind/mind",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "mind",
        method: "POST",
        rust_path: "/jaxrs/mind/mind/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "mind",
        method: "POST",
        rust_path: "/jaxrs/mind/version",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "mind_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/mind/assemble/control/config",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "mind_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/mind/assemble/control/folder/tree/my",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "mind_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/mind/assemble/control/folder/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "mind_assemble_control",
        method: "POST",
        rust_path: "/jaxrs/mind/assemble/control/config/update",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "mind_assemble_control",
        method: "POST",
        rust_path: "/jaxrs/mind/assemble/control/folder/move/{folderId}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "mind_assemble_control",
        method: "POST",
        rust_path: "/jaxrs/mind/assemble/control/folder/save",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "mind_assemble_control",
        method: "POST",
        rust_path: "/jaxrs/mind/assemble/control/folder/{id}/force",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "mind_assemble_control",
        method: "POST",
        rust_path: "/jaxrs/mind/assemble/control/folder/{id}/update",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "mind_core_entity",
        method: "DELETE",
        rust_path: "/jaxrs/mind/core/entity/folder/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "mind_core_entity",
        method: "DELETE",
        rust_path: "/jaxrs/mind/core/entity/mind/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "mind_core_entity",
        method: "GET",
        rust_path: "/jaxrs/mind/core/entity/folder/list",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "mind_core_entity",
        method: "GET",
        rust_path: "/jaxrs/mind/core/entity/list",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "mind_core_entity",
        method: "GET",
        rust_path: "/jaxrs/mind/core/entity/version/list/{mindId}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "mind_core_entity",
        method: "POST",
        rust_path: "/jaxrs/mind/core/entity/folder",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "mind_core_entity",
        method: "POST",
        rust_path: "/jaxrs/mind/core/entity/folder/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "mind_core_entity",
        method: "POST",
        rust_path: "/jaxrs/mind/core/entity/mind",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "mind_core_entity",
        method: "POST",
        rust_path: "/jaxrs/mind/core/entity/mind/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "mind_core_entity",
        method: "POST",
        rust_path: "/jaxrs/mind/core/entity/version",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "organization_assemble_authentication",
        method: "GET",
        rust_path: "/jaxrs/organization/assemble/authentication/dingding/bind/{meta}/callback/{code}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "organization_assemble_authentication",
        method: "GET",
        rust_path: "/jaxrs/organization/assemble/authentication/dingding/login/{code}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "organization_assemble_authentication",
        method: "GET",
        rust_path: "/jaxrs/organization/assemble/authentication/identity/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "organization_assemble_authentication",
        method: "GET",
        rust_path: "/jaxrs/organization/assemble/authentication/person/{id}/icon",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "organization_assemble_authentication",
        method: "GET",
        rust_path: "/jaxrs/organization/assemble/authentication/qiyeweixin/bind/{meta}/callback/{code}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "organization_assemble_authentication",
        method: "GET",
        rust_path: "/jaxrs/organization/assemble/authentication/qiyeweixin/login/{code}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "organization_assemble_authentication",
        method: "GET",
        rust_path: "/jaxrs/organization/assemble/authentication/zhengwudingding/bind/{meta}/callback/{code}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "organization_assemble_authentication",
        method: "GET",
        rust_path: "/jaxrs/organization/assemble/authentication/zhengwudingding/login/{code}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "organization_assemble_authentication",
        method: "POST",
        rust_path: "/jaxrs/organization/assemble/authentication/dingding/bind/{meta}/confirm",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "organization_assemble_authentication",
        method: "POST",
        rust_path: "/jaxrs/organization/assemble/authentication/qiyeweixin/bind/{meta}/confirm",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "organization_assemble_authentication",
        method: "POST",
        rust_path: "/jaxrs/organization/assemble/authentication/zhengwudingding/bind/{meta}/confirm",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "organization_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/identity/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "organization_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/organization/assemble/control/export/export/all",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "organization_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/organization/assemble/control/export/result/flag/{flag}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "organization_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/organization/assemble/control/export/zhengwudingding/person",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "organization_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/organization/assemble/control/group/list/like",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "organization_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/organization/assemble/control/group/list/like/mockputtopost",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "organization_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/organization/assemble/control/group/list/like/pinyin",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "organization_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/organization/assemble/control/group/list/like/pinyin/mockputtopost",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "organization_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/organization/assemble/control/group/list/person/{personFlag}/sup/direct",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "organization_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/organization/assemble/control/group/list/person/{personFlag}/sup/nested",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "organization_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/organization/assemble/control/group/list/pinyininitial",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "organization_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/organization/assemble/control/group/list/pinyininitial/mockputtopost",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "organization_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/organization/assemble/control/group/list/role/{roleFlag}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "organization_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/organization/assemble/control/group/list/{flag}/next/{count}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "organization_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/organization/assemble/control/group/list/{flag}/prev/{count}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "organization_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/organization/assemble/control/group/list/{flag}/sub/direct",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "organization_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/organization/assemble/control/group/list/{flag}/sub/nested",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "organization_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/organization/assemble/control/group/list/{flag}/sup/direct",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "organization_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/organization/assemble/control/group/list/{flag}/sup/nested",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "organization_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/organization/assemble/control/group/{flag}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "organization_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/organization/assemble/control/group/{flag}/add/member",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "organization_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/organization/assemble/control/group/{flag}/add/member/mockputtopost",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "organization_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/organization/assemble/control/group/{flag}/delete/member",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "organization_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/organization/assemble/control/group/{flag}/delete/member/mockputtopost",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "organization_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/organization/assemble/control/group/{flag}/mockdeletetoget",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "organization_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/organization/assemble/control/group/{flag}/mockputtopost",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "organization_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/organization/assemble/control/identity/list/like",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "organization_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/organization/assemble/control/identity/list/like/mockputtopost",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "organization_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/organization/assemble/control/identity/list/like/pinyin",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "organization_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/organization/assemble/control/identity/list/like/pinyin/mockputtopost",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "organization_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/organization/assemble/control/identity/list/person/{personFlag}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "organization_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/organization/assemble/control/identity/list/pinyininitial",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "organization_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/organization/assemble/control/identity/list/pinyininitial/mockputtopost",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "organization_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/organization/assemble/control/identity/list/unit/{unitFlag}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "organization_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/organization/assemble/control/identity/list/unitduty/name/{unitDutyName}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "organization_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/organization/assemble/control/identity/list/{flag}/next/{count}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "organization_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/organization/assemble/control/identity/list/{flag}/prev/{count}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "organization_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/organization/assemble/control/identity/list/{flag}/unitduty/name/{unitDutyName}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "organization_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/organization/assemble/control/identity/{flag}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "organization_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/organization/assemble/control/identity/{flag}/mockdeletetoget",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "organization_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/organization/assemble/control/identity/{flag}/mockputtopost",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "organization_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/organization/assemble/control/identity/{flag}/order/before/{followFlag}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "organization_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/organization/assemble/control/inputperson/result/flag/{flag}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "organization_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/organization/assemble/control/inputperson/template",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "organization_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/organization/assemble/control/inputperson/wipe",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "organization_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/organization/assemble/control/loginrecord/{stream}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "organization_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/organization/assemble/control/permissionsetting/list",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "organization_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/organization/assemble/control/permissionsetting/{flag}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "organization_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/organization/assemble/control/permissionsetting/{flag}/mockdeletetoget",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "organization_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/organization/assemble/control/permissionsetting/{flag}/mockputtopost",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "organization_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/organization/assemble/control/personattribute/list/person/{personFlag}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "organization_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/organization/assemble/control/personattribute/list/{flag}/next/{count}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "organization_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/organization/assemble/control/personattribute/list/{flag}/prev/{count}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "organization_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/organization/assemble/control/personattribute/{flag}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "organization_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/organization/assemble/control/personattribute/{flag}/mockdeletetoget",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "organization_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/organization/assemble/control/personattribute/{flag}/mockputtopost",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "organization_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/organization/assemble/control/personcard/createCode/{cardId}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "organization_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/organization/assemble/control/personcard/createQR/{cardId}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "organization_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/organization/assemble/control/personcard/listPersonalVCf/{idList}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "organization_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/organization/assemble/control/personcard/listVCf/{idList}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "organization_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/organization/assemble/control/personcard/listgrouptypes",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "organization_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/organization/assemble/control/personcard/listpaging/page/{page}/size/{size}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "organization_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/organization/assemble/control/personcard/listpaging/page/{page}/size/{size}/mockputtopost",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "organization_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/organization/assemble/control/personcard/listpagingwithgroup/page/{page}/size/{size}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "organization_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/organization/assemble/control/personcard/listpagingwithgroup/page/{page}/size/{size}/mockputtopost",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "organization_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/organization/assemble/control/personcard/mylist",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "organization_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/organization/assemble/control/personcard/{flag}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "organization_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/organization/assemble/control/personcard/{flag}/mockdeletetoget",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "organization_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/organization/assemble/control/role/list/group/{groupFlag}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "organization_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/organization/assemble/control/role/list/like",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "organization_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/organization/assemble/control/role/list/like/mockputtopost",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "organization_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/organization/assemble/control/role/list/like/pinyin",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "organization_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/organization/assemble/control/role/list/like/pinyin/mockputtopost",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "organization_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/organization/assemble/control/role/list/person/{personFlag}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "organization_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/organization/assemble/control/role/list/pinyininitial",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "organization_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/organization/assemble/control/role/list/pinyininitial/mockputtopost",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "organization_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/organization/assemble/control/role/list/{flag}/next/{count}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "organization_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/organization/assemble/control/role/list/{flag}/prev/{count}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "organization_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/organization/assemble/control/role/{flag}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "organization_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/organization/assemble/control/role/{flag}/mockdeletetoget",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "organization_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/organization/assemble/control/role/{flag}/mockputtopost",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "organization_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/organization/assemble/control/unit/list/{flag}/next/{count}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "organization_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/organization/assemble/control/unit/list/{flag}/sub/nested",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "organization_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/organization/assemble/control/unit/list/{flag}/sup/nested",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "organization_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/organization/assemble/control/unit/list/{flag}/sup/nested/type/{type}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "organization_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/organization/assemble/control/unit/{flag}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "organization_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/organization/assemble/control/unitattribute/list/unit/{flag}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "organization_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/organization/assemble/control/unitattribute/list/{flag}/next/{count}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "organization_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/organization/assemble/control/unitattribute/list/{flag}/prev/{count}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "organization_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/organization/assemble/control/unitattribute/{flag}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "organization_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/organization/assemble/control/unitattribute/{flag}/mockdeletetoget",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "organization_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/organization/assemble/control/unitattribute/{flag}/mockputtopost",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "organization_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/organization/assemble/control/unitduty/distinct/name",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "organization_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/organization/assemble/control/unitduty/distinct/name/like/{key}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "organization_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/organization/assemble/control/unitduty/list/identity/{identityFlag}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "organization_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/organization/assemble/control/unitduty/list/like",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "organization_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/organization/assemble/control/unitduty/list/name/{name}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "organization_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/organization/assemble/control/unitduty/list/unit/{unitFlag}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "organization_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/organization/assemble/control/unitduty/list/{flag}/next/{count}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "organization_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/organization/assemble/control/unitduty/list/{flag}/prev/{count}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "organization_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/organization/assemble/control/unitduty/update/member",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "organization_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/organization/assemble/control/unitduty/{flag}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "organization_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/organization/assemble/control/unitduty/{flag}/mockdeletetoget",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "organization_assemble_control",
        method: "GET",
        rust_path: "/jaxrs/organization/assemble/control/unitduty/{flag}/mockputtopost",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "organization_assemble_control",
        method: "POST",
        rust_path: "/jaxrs/organization/assemble/control/person/list/like",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "organization_assemble_express",
        method: "GET",
        rust_path: "/jaxrs/organization/assemble/express/config/get",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "organization_assemble_express",
        method: "GET",
        rust_path: "/jaxrs/organization/assemble/express/data/sync",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "organization_assemble_express",
        method: "GET",
        rust_path: "/jaxrs/organization/assemble/express/status/get",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "organization_assemble_express",
        method: "GET",
        rust_path: "/jaxrs/organization/assemble/express/units/list",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "organization_assemble_personal",
        method: "GET",
        rust_path: "/jaxrs/organization/assemble/personal/{id}/role/list",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "organization_assemble_personal",
        method: "GET",
        rust_path: "/jaxrs/organization/assemble/personal/{id}/setting",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "organization_core_entity",
        method: "GET",
        rust_path: "/jaxrs/organization/bind/list",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "organization_core_entity",
        method: "GET",
        rust_path: "/jaxrs/organization/custom/list/{identityId}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "organization_core_entity",
        method: "GET",
        rust_path: "/jaxrs/organization/definition/list",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "organization_core_entity",
        method: "GET",
        rust_path: "/jaxrs/organization/group/list",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "organization_core_entity",
        method: "GET",
        rust_path: "/jaxrs/organization/identity/list",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "organization_core_entity",
        method: "GET",
        rust_path: "/jaxrs/organization/person/list",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "organization_core_entity",
        method: "POST",
        rust_path: "/jaxrs/organization/bind",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "organization_core_entity",
        method: "POST",
        rust_path: "/jaxrs/organization/custom",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "organization_core_entity",
        method: "POST",
        rust_path: "/jaxrs/organization/definition",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "organization_core_entity",
        method: "POST",
        rust_path: "/jaxrs/organization/group",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "organization_core_entity",
        method: "POST",
        rust_path: "/jaxrs/organization/identity",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "organization_core_entity",
        method: "POST",
        rust_path: "/jaxrs/organization/person",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "organization_core_entity",
        method: "PUT",
        rust_path: "/jaxrs/organization/bind/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "organization_core_entity",
        method: "PUT",
        rust_path: "/jaxrs/organization/custom/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "organization_core_entity",
        method: "PUT",
        rust_path: "/jaxrs/organization/definition/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "organization_core_entity",
        method: "PUT",
        rust_path: "/jaxrs/organization/group/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "organization_core_entity",
        method: "PUT",
        rust_path: "/jaxrs/organization/identity/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "organization_core_entity",
        method: "PUT",
        rust_path: "/jaxrs/organization/person/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "organization_core_express",
        method: "GET",
        rust_path: "/jaxrs/organization/core/express/config",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "organization_core_express",
        method: "GET",
        rust_path: "/jaxrs/organization/core/express/status",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "organization_core_express",
        method: "GET",
        rust_path: "/jaxrs/organization/core/express/sync",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "personal",
        method: "GET",
        rust_path: "/jaxrs/person",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "personal",
        method: "GET",
        rust_path: "/jaxrs/person/icon/{person}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "personal",
        method: "GET",
        rust_path: "/jaxrs/person/regist/check/email/{email}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "personal",
        method: "GET",
        rust_path: "/jaxrs/person/regist/check/mobile/{mobile}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "personal",
        method: "GET",
        rust_path: "/jaxrs/person/regist/check/name/{name}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "personal",
        method: "GET",
        rust_path: "/jaxrs/person/signature/delete/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "personal",
        method: "GET",
        rust_path: "/jaxrs/person/signature/list",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "personal",
        method: "GET",
        rust_path: "/jaxrs/person/signature/manager/list",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "personal",
        method: "GET",
        rust_path: "/jaxrs/reset/check/credential/{credential}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "personal",
        method: "GET",
        rust_path: "/jaxrs/reset/check/password/{password}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "personal",
        method: "GET",
        rust_path: "/jaxrs/reset/code/credential/{credential}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "personal",
        method: "POST",
        rust_path: "/jaxrs/person/icon/upload",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "personal",
        method: "POST",
        rust_path: "/jaxrs/person/regist",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "personal",
        method: "POST",
        rust_path: "/jaxrs/person/regist/code",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "personal",
        method: "POST",
        rust_path: "/jaxrs/person/signature/upload",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "personal",
        method: "POST",
        rust_path: "/jaxrs/reset/password/anonymous",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "personal",
        method: "PUT",
        rust_path: "/jaxrs/person",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "personal",
        method: "PUT",
        rust_path: "/jaxrs/person/password",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "personal",
        method: "PUT",
        rust_path: "/jaxrs/reset",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "personal_extend",
        method: "GET",
        rust_path: "/jaxrs/icon/{person}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "personal_extend",
        method: "GET",
        rust_path: "/jaxrs/person/icon",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "personal_extend",
        method: "GET",
        rust_path: "/jaxrs/personal/detail/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "personal_extend",
        method: "GET",
        rust_path: "/jaxrs/personal/info",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "portal",
        method: "GET",
        rust_path: "/jaxrs/portal/dict/list",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "portal",
        method: "GET",
        rust_path: "/jaxrs/portal/list",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "portal",
        method: "GET",
        rust_path: "/jaxrs/portal/page/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "portal",
        method: "GET",
        rust_path: "/jaxrs/portal/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "portal",
        method: "GET",
        rust_path: "/jaxrs/portalcategory/list",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "portal",
        method: "POST",
        rust_path: "/jaxrs/portal/page/delete/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "portal",
        method: "POST",
        rust_path: "/jaxrs/portal/page/save/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "portal_assemble_designer",
        method: "GET",
        rust_path: "/jaxrs/portal/assemble/designer/dict/list/paging/{page}/{size}/{size}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "portal_assemble_designer",
        method: "GET",
        rust_path: "/jaxrs/portal/assemble/designer/dict/list/portal/{portalId}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "portal_assemble_designer",
        method: "GET",
        rust_path: "/jaxrs/portal/assemble/designer/dict/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "portal_assemble_designer",
        method: "GET",
        rust_path: "/jaxrs/portal/assemble/designer/file/download/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "portal_assemble_designer",
        method: "GET",
        rust_path: "/jaxrs/portal/assemble/designer/file/list/application/{applicationFlag}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "portal_assemble_designer",
        method: "GET",
        rust_path: "/jaxrs/portal/assemble/designer/file/list/{id}/{next}/{count}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "portal_assemble_designer",
        method: "GET",
        rust_path: "/jaxrs/portal/assemble/designer/file/{flag}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "portal_assemble_designer",
        method: "GET",
        rust_path: "/jaxrs/portal/assemble/designer/get/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "portal_assemble_designer",
        method: "GET",
        rust_path: "/jaxrs/portal/assemble/designer/list",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "portal_assemble_designer",
        method: "GET",
        rust_path: "/jaxrs/portal/assemble/designer/list/portal/{page}/{portalId}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "portal_assemble_designer",
        method: "GET",
        rust_path: "/jaxrs/portal/assemble/designer/output/select/file/{flag}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "portal_assemble_designer",
        method: "GET",
        rust_path: "/jaxrs/portal/assemble/designer/output/select/{portalFlag}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "portal_assemble_designer",
        method: "GET",
        rust_path: "/jaxrs/portal/assemble/designer/page/list/{category}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "portal_assemble_designer",
        method: "GET",
        rust_path: "/jaxrs/portal/assemble/designer/page/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "portal_assemble_designer",
        method: "GET",
        rust_path: "/jaxrs/portal/assemble/designer/pageversion/list/{page}/{pageId}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "portal_assemble_designer",
        method: "GET",
        rust_path: "/jaxrs/portal/assemble/designer/pageversion/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "portal_assemble_designer",
        method: "GET",
        rust_path: "/jaxrs/portal/assemble/designer/portal/icon/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "portal_assemble_designer",
        method: "GET",
        rust_path: "/jaxrs/portal/assemble/designer/portal/list/portalcategory/{portalCategory}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "portal_assemble_designer",
        method: "GET",
        rust_path: "/jaxrs/portal/assemble/designer/portal/list/summary/portalcategory/{portalCategory}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "portal_assemble_designer",
        method: "GET",
        rust_path: "/jaxrs/portal/assemble/designer/portal/permission/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "portal_assemble_designer",
        method: "GET",
        rust_path: "/jaxrs/portal/assemble/designer/portal/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "portal_assemble_designer",
        method: "GET",
        rust_path: "/jaxrs/portal/assemble/designer/script/list/paging/{page}/{size}/{size}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "portal_assemble_designer",
        method: "GET",
        rust_path: "/jaxrs/portal/assemble/designer/script/list/portal/{portalId}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "portal_assemble_designer",
        method: "GET",
        rust_path: "/jaxrs/portal/assemble/designer/script/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "portal_assemble_designer",
        method: "GET",
        rust_path: "/jaxrs/portal/assemble/designer/scriptversion/list/script/{scriptId}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "portal_assemble_designer",
        method: "GET",
        rust_path: "/jaxrs/portal/assemble/designer/scriptversion/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "portal_assemble_designer",
        method: "GET",
        rust_path: "/jaxrs/portal/assemble/designer/templatepage/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "portal_assemble_designer",
        method: "GET",
        rust_path: "/jaxrs/portal/assemble/designer/widget/list/portal/{portalId}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "portal_assemble_designer",
        method: "GET",
        rust_path: "/jaxrs/portal/assemble/designer/widget/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "portal_assemble_designer",
        method: "GET",
        rust_path: "/jaxrs/portal/assemble/designer/{id}/{count}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "portal_assemble_designer",
        method: "GET",
        rust_path: "/jaxrs/portal/design/list",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "portal_assemble_designer",
        method: "GET",
        rust_path: "/jaxrs/portal/design/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "portal_assemble_designer",
        method: "POST",
        rust_path: "/jaxrs/portal/assemble/designer/create",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "portal_assemble_designer",
        method: "POST",
        rust_path: "/jaxrs/portal/assemble/designer/file/upload/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "portal_assemble_designer",
        method: "POST",
        rust_path: "/jaxrs/portal/assemble/designer/page/create",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "portal_assemble_designer",
        method: "POST",
        rust_path: "/jaxrs/portal/assemble/designer/page/delete/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "portal_assemble_designer",
        method: "POST",
        rust_path: "/jaxrs/portal/assemble/designer/page/save/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "portal_assemble_designer",
        method: "POST",
        rust_path: "/jaxrs/portal/assemble/designer/save/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "portal_assemble_designer",
        method: "POST",
        rust_path: "/jaxrs/portal/design/save",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "portal_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/portal/assemble/surface/dict/list/portal/{portalFlag}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "portal_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/portal/assemble/surface/dict/portal/data/{dictFlag}/{portalFlag}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "portal_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/portal/assemble/surface/dict/portal/path/data/{dictFlag}/{portalFlag}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "portal_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/portal/assemble/surface/dict/portal/{dictFlag}/{portalFlag}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "portal_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/portal/assemble/surface/file/download/{flag}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "portal_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/portal/assemble/surface/file/list/portal/{portalFlag}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "portal_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/portal/assemble/surface/file/portal/content/{flag}/{portalFlag}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "portal_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/portal/assemble/surface/file/portal/download/{flag}/{portalFlag}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "portal_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/portal/assemble/surface/file/{flag}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "portal_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/portal/assemble/surface/get/layout",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "portal_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/portal/assemble/surface/get/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "portal_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/portal/assemble/surface/list/layouts",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "portal_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/portal/assemble/surface/list/portal/portal/{page}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "portal_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/portal/assemble/surface/list/{category}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "portal_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/portal/assemble/surface/mobile/{page}/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "portal_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/portal/assemble/surface/portal/corner/mark/{flag}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "portal_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/portal/assemble/surface/portal/icon/base64/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "portal_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/portal/assemble/surface/portal/icon/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "portal_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/portal/assemble/surface/portal/mobile/{page}/{flag}/{portalFlag}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "portal_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/portal/assemble/surface/portal/{flag}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "portal_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/portal/assemble/surface/portal/{page}/{flag}/{portalFlag}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "portal_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/portal/assemble/surface/preview/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "portal_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/portal/assemble/surface/script/list/portal/portal",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "portal_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/portal/assemble/surface/script/portal/portal/{name}/{name}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "portal_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/portal/assemble/surface/script/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "portal_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/portal/assemble/surface/v2/mobile/{page}/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "portal_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/portal/assemble/surface/v2/portal/mobile/{page}/{flag}/{portalFlag}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "portal_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/portal/assemble/surface/v2/portal/{page}/{flag}/{portalFlag}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "portal_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/portal/assemble/surface/v2/{page}/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "portal_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/portal/assemble/surface/widget/list/portal/portal",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "portal_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/portal/assemble/surface/widget/mobile/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "portal_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/portal/assemble/surface/widget/portal/mobile/{flag}/{portalFlag}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "portal_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/portal/assemble/surface/widget/portal/{flag}/{portalFlag}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "portal_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/portal/assemble/surface/widget/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "portal_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/portal/assemble/surface/{page}/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "portal_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/portal/surface/list",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "portal_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/portal/surface/{id}/preview",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "portal_assemble_surface",
        method: "POST",
        rust_path: "/jaxrs/portal/assemble/surface/create",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "portal_assemble_surface",
        method: "POST",
        rust_path: "/jaxrs/portal/assemble/surface/delete/layout",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "portal_assemble_surface",
        method: "POST",
        rust_path: "/jaxrs/portal/assemble/surface/dict/portal/path/data/mockdeletetoget/{dictFlag}/{portalFlag}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "portal_assemble_surface",
        method: "POST",
        rust_path: "/jaxrs/portal/assemble/surface/dict/portal/path/data/mockputtopost/{dictFlag}/{portalFlag}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "portal_assemble_surface",
        method: "POST",
        rust_path: "/jaxrs/portal/assemble/surface/publish/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "portal_assemble_surface",
        method: "POST",
        rust_path: "/jaxrs/portal/assemble/surface/save/layout",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "portal_assemble_surface",
        method: "POST",
        rust_path: "/jaxrs/portal/assemble/surface/script/portal/portal/imported/{name}/{name}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "portal_assemble_surface",
        method: "POST",
        rust_path: "/jaxrs/portal/surface/publish",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "portal_core_entity",
        method: "GET",
        rust_path: "/jaxrs/portal/page/get/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "portal_core_entity",
        method: "GET",
        rust_path: "/jaxrs/portal/page/list",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "portal_core_entity",
        method: "GET",
        rust_path: "/jaxrs/portal/portal/list",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "portal_core_entity",
        method: "GET",
        rust_path: "/jaxrs/portal/script/list",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "portal_core_entity",
        method: "GET",
        rust_path: "/jaxrs/portal/widget/list",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "portal_core_entity",
        method: "POST",
        rust_path: "/jaxrs/portal/page/create",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "portal_core_entity",
        method: "POST",
        rust_path: "/jaxrs/portal/page/remove",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "portal_core_entity",
        method: "POST",
        rust_path: "/jaxrs/portal/page/update",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "process_bam",
        method: "GET",
        rust_path: "/jaxrs/process/state/organization",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "process_bam",
        method: "GET",
        rust_path: "/jaxrs/process/state/running",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "process_bam",
        method: "GET",
        rust_path: "/jaxrs/process/state/summary",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "process_designer",
        method: "GET",
        rust_path: "/jaxrs/process/application/list/summary",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "process_designer",
        method: "GET",
        rust_path: "/jaxrs/process/designer/application/list",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "process_designer",
        method: "GET",
        rust_path: "/jaxrs/process/designer/application/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "process_designer",
        method: "GET",
        rust_path: "/jaxrs/process/designer/route/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "process_designer",
        method: "POST",
        rust_path: "/jaxrs/process/designer/application/create",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "process_designer",
        method: "POST",
        rust_path: "/jaxrs/process/designer/application/remove",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "process_designer",
        method: "POST",
        rust_path: "/jaxrs/process/designer/application/update",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "process_express",
        method: "GET",
        rust_path: "/jaxrs/process/application/list",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "process_express",
        method: "GET",
        rust_path: "/jaxrs/process/read/count/{credential}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "process_express",
        method: "GET",
        rust_path: "/jaxrs/process/task/count/{credential}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "process_surface",
        method: "GET",
        rust_path: "/jaxrs/process/list/ids",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "process_surface",
        method: "GET",
        rust_path: "/jaxrs/process/record/list/workorworkcompleted/{workOrWorkCompleted}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "process_surface",
        method: "GET",
        rust_path: "/jaxrs/process/{flag}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_bam",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/bam/get/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_bam",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/bam/list/{category}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_bam",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/bam/period/list/completed/application/by/process/{count}/{work}/{applicationId}/{unit}/{unit}/{person}/{person}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_bam",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/bam/period/list/completed/application/process/by/{count}/{work}/{applicationId}/{processId}/{unit}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_bam",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/bam/period/list/completed/application/process/{count}/{work}/{applicationId}/{processId}/{unit}/{unit}/{person}/{person}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_bam",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/bam/period/list/completed/application/{work}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_bam",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/bam/period/list/completed/by/application/{count}/{work}/{unit}/{unit}/{person}/{person}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_bam",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/bam/period/list/completed/task/application",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_bam",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/bam/period/list/completed/task/application/by/process/{count}/{applicationId}/{unit}/{unit}/{person}/{person}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_bam",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/bam/period/list/completed/task/application/process/activity/by/{count}/{applicationId}/{processId}/{activityId}/{unit}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_bam",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/bam/period/list/completed/task/application/process/activity/{count}/{applicationId}/{processId}/{activityId}/{unit}/{unit}/{person}/{person}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_bam",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/bam/period/list/completed/task/application/process/by/activity/{count}/{applicationId}/{processId}/{unit}/{unit}/{person}/{person}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_bam",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/bam/period/list/completed/task/by/application/{count}/{unit}/{unit}/{person}/{person}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_bam",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/bam/period/list/completed/task/{unit}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_bam",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/bam/period/list/completed/{work}/{unit}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_bam",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/bam/period/list/expired/application/by/process/{count}/{work}/{applicationId}/{unit}/{unit}/{person}/{person}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_bam",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/bam/period/list/expired/application/process/by/{count}/{work}/{applicationId}/{processId}/{unit}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_bam",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/bam/period/list/expired/application/process/{count}/{work}/{applicationId}/{processId}/{unit}/{unit}/{person}/{person}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_bam",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/bam/period/list/expired/application/{work}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_bam",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/bam/period/list/expired/by/application/{count}/{work}/{unit}/{unit}/{person}/{person}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_bam",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/bam/period/list/expired/task/application",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_bam",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/bam/period/list/expired/task/application/by/process/{count}/{applicationId}/{unit}/{unit}/{person}/{person}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_bam",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/bam/period/list/expired/task/application/process/activity/by/{count}/{applicationId}/{processId}/{activityId}/{unit}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_bam",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/bam/period/list/expired/task/application/process/activity/{count}/{applicationId}/{processId}/{activityId}/{unit}/{unit}/{person}/{person}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_bam",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/bam/period/list/expired/task/application/process/by/activity/{count}/{applicationId}/{processId}/{unit}/{unit}/{person}/{person}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_bam",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/bam/period/list/expired/task/by/application/{count}/{unit}/{unit}/{person}/{person}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_bam",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/bam/period/list/expired/task/{unit}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_bam",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/bam/period/list/expired/{work}/{unit}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_bam",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/bam/status/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_bam",
        method: "POST",
        rust_path: "/jaxrs/processplatform/assemble/bam/create",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_bam",
        method: "POST",
        rust_path: "/jaxrs/processplatform/assemble/bam/delete/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_bam",
        method: "POST",
        rust_path: "/jaxrs/processplatform/assemble/bam/period/list/application/by/process/{count}/{start}/{work}/{applicationId}/{unit}/{unit}/{person}/{person}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_bam",
        method: "POST",
        rust_path: "/jaxrs/processplatform/assemble/bam/period/list/application/process/by/{count}/{start}/{work}/{applicationId}/{processId}/{unit}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_bam",
        method: "POST",
        rust_path: "/jaxrs/processplatform/assemble/bam/period/list/application/process/{count}/{start}/{work}/{applicationId}/{processId}/{unit}/{unit}/{person}/{person}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_bam",
        method: "POST",
        rust_path: "/jaxrs/processplatform/assemble/bam/period/list/application/{start}/{work}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_bam",
        method: "POST",
        rust_path: "/jaxrs/processplatform/assemble/bam/period/list/by/application/{count}/{start}/{work}/{unit}/{unit}/{person}/{person}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_bam",
        method: "POST",
        rust_path: "/jaxrs/processplatform/assemble/bam/period/list/task/application/by/process/{count}/{start}/{applicationId}/{unit}/{unit}/{person}/{person}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_bam",
        method: "POST",
        rust_path: "/jaxrs/processplatform/assemble/bam/period/list/task/application/process/activity/by/{count}/{start}/{applicationId}/{processId}/{activityId}/{unit}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_bam",
        method: "POST",
        rust_path: "/jaxrs/processplatform/assemble/bam/period/list/task/application/process/activity/{count}/{start}/{applicationId}/{processId}/{activityId}/{unit}/{unit}/{person}/{person}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_bam",
        method: "POST",
        rust_path: "/jaxrs/processplatform/assemble/bam/period/list/task/application/process/by/activity/{count}/{start}/{applicationId}/{processId}/{unit}/{unit}/{person}/{person}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_bam",
        method: "POST",
        rust_path: "/jaxrs/processplatform/assemble/bam/period/list/task/application/{start}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_bam",
        method: "POST",
        rust_path: "/jaxrs/processplatform/assemble/bam/period/list/task/by/application/{count}/{start}/{unit}/{unit}/{person}/{person}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_bam",
        method: "POST",
        rust_path: "/jaxrs/processplatform/assemble/bam/period/list/task/{start}/{unit}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_bam",
        method: "POST",
        rust_path: "/jaxrs/processplatform/assemble/bam/period/list/{start}/{work}/{unit}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_bam",
        method: "POST",
        rust_path: "/jaxrs/processplatform/assemble/bam/state/trigger/{category}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_designer",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/designer/application/icon/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_designer",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/designer/application/list/applicationcategory/{applicationCategory}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_designer",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/designer/application/list/summary/applicationcategory/{applicationCategory}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_designer",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/designer/application/permission/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_designer",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/designer/application/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_designer",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/designer/applicationdict/list/application/{applicationId}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_designer",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/designer/applicationdict/list/paging/{page}/{size}/{size}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_designer",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/designer/applicationdict/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_designer",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/designer/file/application/{flag}/{applicationFlag}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_designer",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/designer/file/content/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_designer",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/designer/file/download/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_designer",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/designer/file/list/application/{applicationFlag}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_designer",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/designer/file/list/{id}/{next}/{count}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_designer",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/designer/file/{flag}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_designer",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/designer/form/list/application/{applicationId}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_designer",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/designer/form/list/formfield/application/{applicationId}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_designer",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/designer/form/list/formfield/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_designer",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/designer/form/list/{id}/{next}/{count}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_designer",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/designer/form/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_designer",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/designer/formversion/list/form/{formId}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_designer",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/designer/formversion/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_designer",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/designer/get/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_designer",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/designer/item/access/path/path",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_designer",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/designer/item/access/process/path/path/{processId}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_designer",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/designer/item/access/process/{processId}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_designer",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/designer/item/access/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_designer",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/designer/list/{category}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_designer",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/designer/mapping/execute/{flag}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_designer",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/designer/mapping/list/application/{applicationFlag}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_designer",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/designer/mapping/list/{id}/{next}/{count}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_designer",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/designer/mapping/{flag}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_designer",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/designer/output/select/{applicationFlag}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_designer",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/designer/preview/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_designer",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/designer/process/activity/{flag}/{activityType}/{activityType}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_designer",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/designer/process/application/{applicationId}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_designer",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/designer/process/disable/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_designer",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/designer/process/enable/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_designer",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/designer/process/enabled/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_designer",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/designer/process/execute/projection/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_designer",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/designer/process/form/{formId}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_designer",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/designer/process/lead/out/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_designer",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/designer/process/list/element/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_designer",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/designer/process/permission/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_designer",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/designer/process/process/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_designer",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/designer/process/upgrade/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_designer",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/designer/process/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_designer",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/designer/processversion/list/process/{processId}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_designer",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/designer/processversion/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_designer",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/designer/script/application/{applicationId}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_designer",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/designer/script/application/{applicationId}/{name}/{name}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_designer",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/designer/script/list/paging/{page}/{size}/{size}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_designer",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/designer/script/list/{id}/{next}/{count}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_designer",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/designer/script/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_designer",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/designer/scriptversion/list/script/{scriptId}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_designer",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/designer/scriptversion/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_designer",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/designer/templateform/list/{category}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_designer",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/designer/templateform/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_designer",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/designer/{id}/{count}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_designer",
        method: "POST",
        rust_path: "/jaxrs/processplatform/assemble/designer/application/{id}/{onlyRemoveNotCompleted}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_designer",
        method: "POST",
        rust_path: "/jaxrs/processplatform/assemble/designer/create",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_designer",
        method: "POST",
        rust_path: "/jaxrs/processplatform/assemble/designer/delete/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_designer",
        method: "POST",
        rust_path: "/jaxrs/processplatform/assemble/designer/file/upload/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_designer",
        method: "POST",
        rust_path: "/jaxrs/processplatform/assemble/designer/item/access/delete/process/path/path/{processId}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_designer",
        method: "POST",
        rust_path: "/jaxrs/processplatform/assemble/designer/mergeitemplan/list/application/paging/{applicationId}/{page}/{size}/{size}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_designer",
        method: "POST",
        rust_path: "/jaxrs/processplatform/assemble/designer/mergeitemplan/list/paging/{page}/{size}/{size}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_designer",
        method: "POST",
        rust_path: "/jaxrs/processplatform/assemble/designer/mergeitemplan/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_designer",
        method: "POST",
        rust_path: "/jaxrs/processplatform/assemble/designer/process/application/disable/edition/{applicationId}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_designer",
        method: "POST",
        rust_path: "/jaxrs/processplatform/assemble/designer/process/application/edition/edition/{applicationId}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_designer",
        method: "POST",
        rust_path: "/jaxrs/processplatform/assemble/designer/process/edition/{id}/{onlyRemoveNotCompleted}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_designer",
        method: "POST",
        rust_path: "/jaxrs/processplatform/assemble/designer/process/{id}/{onlyRemoveNotCompleted}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_designer",
        method: "POST",
        rust_path: "/jaxrs/processplatform/assemble/designer/save/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_designer",
        method: "POST",
        rust_path: "/jaxrs/processplatform/assemble/designer/workcompleted/application/merge/data/{applicationFlag}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_designer",
        method: "POST",
        rust_path: "/jaxrs/processplatform/assemble/designer/workcompleted/process/merge/data/{processFlag}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/anonymous/read/{count}/{credential}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/anonymous/task/{count}/{credential}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/application/icon/{flag}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/application/is/manager/{flag}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/application/list/complex/manage/person",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/application/list/key/key",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/application/list/terminal/terminal",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/application/{flag}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/application/{flag}/{onlyRemoveNotCompleted}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/applicationdict/application/data/{applicationDictFlag}/{applicationFlag}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/applicationdict/application/path0/data/mockputtopost/{applicationDictFlag}/{applicationFlag}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/applicationdict/application/path0/data/{applicationDictFlag}/{applicationFlag}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/applicationdict/application/path0/path1/data/mockputtopost/{applicationDictFlag}/{applicationFlag}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/applicationdict/application/path0/path1/data/{applicationDictFlag}/{applicationFlag}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/applicationdict/application/path0/path1/path2/data/mockputtopost/{applicationDictFlag}/{applicationFlag}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/applicationdict/application/path0/path1/path2/data/{applicationDictFlag}/{applicationFlag}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/applicationdict/application/path0/path1/path2/path3/data/mockputtopost/{applicationDictFlag}/{applicationFlag}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/applicationdict/application/path0/path1/path2/path3/data/{applicationDictFlag}/{applicationFlag}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/applicationdict/application/path0/path1/path2/path3/path4/data/mockputtopost/{applicationDictFlag}/{applicationFlag}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/applicationdict/application/path0/path1/path2/path3/path4/data/{applicationDictFlag}/{applicationFlag}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/applicationdict/application/path0/path1/path2/path3/path4/path5/data/mockputtopost/{applicationDictFlag}/{applicationFlag}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/applicationdict/application/path0/path1/path2/path3/path4/path5/data/{applicationDictFlag}/{applicationFlag}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/applicationdict/application/path0/path1/path2/path3/path4/path5/path6/data/mockputtopost/{applicationDictFlag}/{applicationFlag}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/applicationdict/application/path0/path1/path2/path3/path4/path5/path6/data/{applicationDictFlag}/{applicationFlag}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/applicationdict/application/path0/path1/path2/path3/path4/path5/path6/path7/data/mockputtopost/{applicationDictFlag}/{applicationFlag}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/applicationdict/application/path0/path1/path2/path3/path4/path5/path6/path7/data/{applicationDictFlag}/{applicationFlag}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/applicationdict/application/{applicationDictFlag}/{applicationFlag}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/applicationdict/list/application/{applicationFlag}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/control/workorworkcompleted/{workOrWorkCompleted}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/correlation/job/job",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/correlation/list/job/job",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/correlation/list/job/job/site/site",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/data/fetch/job/job",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/data/job/job",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/data/job/job/array/data",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/data/job/job/mockputtopost",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/data/job/job/path0",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/data/job/job/path0/mockputtopost",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/data/job/job/path0/path1",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/data/job/job/path0/path1/mockputtopost",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/data/job/job/path0/path1/path2",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/data/job/job/path0/path1/path2/mockputtopost",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/data/job/job/path0/path1/path2/path3",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/data/job/job/path0/path1/path2/path3/mockputtopost",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/data/job/job/path0/path1/path2/path3/path4",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/data/job/job/path0/path1/path2/path3/path4/mockputtopost",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/data/job/job/path0/path1/path2/path3/path4/path5",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/data/job/job/path0/path1/path2/path3/path4/path5/mockputtopost",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/data/job/job/path0/path1/path2/path3/path4/path5/path6",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/data/job/job/path0/path1/path2/path3/path4/path5/path6/mockputtopost",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/data/job/job/path0/path1/path2/path3/path4/path5/path6/path7",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/data/job/job/path0/path1/path2/path3/path4/path5/path6/path7/mockputtopost",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/data/work/mockputtopost/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/data/work/path0/mockputtopost/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/data/work/path0/path1/mockputtopost/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/data/work/path0/path1/path2/mockputtopost/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/data/work/path0/path1/path2/path3/mockputtopost/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/data/work/path0/path1/path2/path3/path4/mockputtopost/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/data/work/path0/path1/path2/path3/path4/path5/mockputtopost/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/data/work/path0/path1/path2/path3/path4/path5/path6/mockputtopost/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/data/work/path0/path1/path2/path3/path4/path5/path6/path7/mockputtopost/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/data/work/path0/path1/path2/path3/path4/path5/path6/path7/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/data/work/path0/path1/path2/path3/path4/path5/path6/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/data/work/path0/path1/path2/path3/path4/path5/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/data/work/path0/path1/path2/path3/path4/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/data/work/path0/path1/path2/path3/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/data/work/path0/path1/path2/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/data/work/path0/path1/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/data/work/path0/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/data/work/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/data/workcompleted/from/data/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/data/workcompleted/from/item/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/data/workcompleted/mockputtopost/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/data/workcompleted/path0/mockputtopost/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/data/workcompleted/path0/path1/mockputtopost/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/data/workcompleted/path0/path1/path2/mockputtopost/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/data/workcompleted/path0/path1/path2/path3/mockputtopost/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/data/workcompleted/path0/path1/path2/path3/path4/mockputtopost/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/data/workcompleted/path0/path1/path2/path3/path4/path5/mockputtopost/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/data/workcompleted/path0/path1/path2/path3/path4/path5/path6/mockputtopost/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/data/workcompleted/path0/path1/path2/path3/path4/path5/path6/path7/mockputtopost/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/data/workcompleted/path0/path1/path2/path3/path4/path5/path6/path7/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/data/workcompleted/path0/path1/path2/path3/path4/path5/path6/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/data/workcompleted/path0/path1/path2/path3/path4/path5/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/data/workcompleted/path0/path1/path2/path3/path4/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/data/workcompleted/path0/path1/path2/path3/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/data/workcompleted/path0/path1/path2/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/data/workcompleted/path0/path1/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/data/workcompleted/path0/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/data/workcompleted/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/datarecord/get/job/job/path/path",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/datarecord/list/job/job",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/documentversion/list/job/job",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/documentversion/list/job/job/{category}/{category}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/documentversion/list/workorworkcompleted/{workOrWorkCompleted}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/documentversion/list/workorworkcompleted/{workOrWorkCompleted}/{category}/{category}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/documentversion/work/work",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/documentversion/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/draft/list/my/paging/{page}/{size}/{size}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/draft/list/next/{id}/{count}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/draft/list/prev/{id}/{count}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/draft/mockputtopost",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/draft/process/{processFlag}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/draft/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/file/application/content/{flag}/{applicationFlag}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/file/application/download/{flag}/{applicationFlag}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/file/list/application/{applicationFlag}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/form/application/mobile/{flag}/{applicationFlag}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/form/application/{flag}/{applicationFlag}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/form/mobile/{flag}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/form/v2/lookup/taskcompleted/taskcompleted",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/form/v2/lookup/taskcompleted/taskcompleted/mobile",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/form/v2/lookup/workorworkcompleted/mobile/{workOrWorkCompleted}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/form/v2/lookup/workorworkcompleted/{workOrWorkCompleted}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/form/v2/mobile/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/form/v2/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/form/{flag}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/get/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/handover/cancel/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/handover/list/paging/{page}/{size}/{size}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/handover/process/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/handover/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/job/job/allow/visit/person/person",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/job/job/find/work/workcompleted",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/job/latest/work/workcompleted/serial/serial",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/job/v2/job/projection",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/keylock/lock",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/keylock/lock/mockputtopost",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/list/{category}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/mode/clear/person/person/manager",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/preview/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/process/activity/activity/{activityType}/{activityType}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/process/allowrerouteto/{flag}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/process/application/{flag}/{applicationFlag}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/process/complex/{flag}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/process/list/application/filter/{applicationFlag}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/process/list/application/{applicationFlag}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/process/list/available/identity/process/{flag}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/process/list/controllable/application/{applicationFlag}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/process/{flag}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/process/{flag}/{onlyRemoveNotCompleted}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/read/filter/attribute",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/read/filter/attribute/filter",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/read/list/application/process/{count}/{applicationFlag}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/read/list/date/date/manage",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/read/list/filter/manage/{page}/{size}/{size}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/read/list/job/job",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/read/list/my/filter/{page}/{size}/{size}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/read/list/my/paging/{page}/{size}/{size}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/read/list/next/application/{id}/{count}/{applicationFlag}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/read/list/next/filter/{id}/{count}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/read/list/next/process/{id}/{count}/{processFlag}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/read/list/next/{id}/{count}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/read/list/person/person/manage",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/read/list/prev/application/{id}/{count}/{applicationFlag}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/read/list/prev/filter/{id}/{count}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/read/list/prev/process/{id}/{count}/{processFlag}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/read/list/prev/{id}/{count}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/read/list/work/work",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/read/list/workorworkcompleted/{workOrWorkCompleted}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/read/manage/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/read/mockputtopost/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/read/v2/list/next/{id}/{count}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/read/v2/list/paging/{page}/{size}/{size}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/read/v2/list/prev/{id}/{count}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/read/work/{workId}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/read/workcompleted/{workCompletedId}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/read/{count}/{credential}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/read/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/readcompleted/filter/attribute",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/readcompleted/filter/attribute/filter",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/readcompleted/list/application/process/{count}/{applicationFlag}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/readcompleted/list/date/date/manage",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/readcompleted/list/filter/manage/{page}/{size}/{size}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/readcompleted/list/job/job",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/readcompleted/list/my/filter/{page}/{size}/{size}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/readcompleted/list/my/paging/{page}/{size}/{size}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/readcompleted/list/next/application/{id}/{count}/{applicationFlag}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/readcompleted/list/next/filter/{id}/{count}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/readcompleted/list/next/process/{id}/{count}/{processFlag}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/readcompleted/list/next/{id}/{count}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/readcompleted/list/prev/application/{id}/{count}/{applicationFlag}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/readcompleted/list/prev/filter/{id}/{count}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/readcompleted/list/prev/process/{id}/{count}/{processFlag}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/readcompleted/list/prev/{id}/{count}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/readcompleted/list/work/work",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/readcompleted/list/workorworkcompleted/{workOrWorkCompleted}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/readcompleted/manage/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/readcompleted/v2/list/next/{id}/{count}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/readcompleted/v2/list/paging/{page}/{size}/{size}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/readcompleted/v2/list/prev/{id}/{count}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/readcompleted/{count}/{credential}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/readcompleted/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/readrecord/list/job/job",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/readrecord/list/workorworkcompleted/{workOrWorkCompleted}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/record/job/job/manage",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/record/list/job/job",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/record/list/job/job/paging/{page}/{size}/{size}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/record/list/workorworkcompleted/paging/{workOrWorkCompleted}/{page}/{size}/{size}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/record/list/workorworkcompleted/{workOrWorkCompleted}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/record/manage/mockputtopost/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/record/manage/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/review/application/manage/{id}/{applicationFlag}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/review/filter/attribute",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/review/filter/entry",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/review/list/job/job",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/review/person/{count}/{credential}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/review/v2/list/next/{id}/{count}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/review/v2/list/paging/manage/{page}/{size}/{size}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/review/v2/list/paging/{page}/{size}/{size}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/review/v2/list/prev/{id}/{count}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/review/v2/search",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/review/workorworkcompleted/{workOrWorkCompleted}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/review/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/route/list/mockputtopost",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/route/selectconfig/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/route/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/script/application/imported/{flag}/{applicationFlag}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/script/application/{flag}/{applicationFlag}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/serialnumber/generate/process/name/name/serial/{processId}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/serialnumber/list/application/{applicationFlag}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/serialnumber/list/paging/{page}/{size}/{size}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/serialnumber/mockputtopost/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/serialnumber/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/sign/download/{scrawlId}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/sign/list/job/job",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/sign/task/{taskId}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/sign/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/task/filter/attribute",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/task/filter/attribute/filter",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/task/list/application/process/{count}/{applicationFlag}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/task/list/date/date/hour/hour/exclude/draft/manage/{isExcludeDraft}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/task/list/filter/manage/{page}/{size}/{size}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/task/list/job/job",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/task/list/my/filter/{page}/{size}/{size}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/task/list/my/paging/{page}/{size}/{size}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/task/list/next/application/{id}/{count}/{applicationFlag}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/task/list/next/filter/manage/{id}/{count}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/task/list/next/filter/{id}/{count}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/task/list/next/manage/{id}/{count}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/task/list/next/process/{id}/{count}/{processFlag}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/task/list/next/{id}/{count}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/task/list/person/person/exclude/draft/manage/{isExcludeDraft}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/task/list/prev/application/{id}/{count}/{applicationFlag}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/task/list/prev/filter/manage/{id}/{count}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/task/list/prev/filter/{id}/{count}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/task/list/prev/manage/{id}/{count}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/task/list/prev/process/{id}/{count}/{processFlag}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/task/list/prev/{id}/{count}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/task/list/work/work",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/task/manage/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/task/mockputtopost/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/task/v2/list/next/{id}/{count}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/task/v2/list/paging/{page}/{size}/{size}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/task/v2/list/prev/{id}/{count}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/task/v2/pause/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/task/v3/pin/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/task/{count}/{credential}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/task/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/taskcompleted/filter/attribute",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/taskcompleted/filter/attribute/filter",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/taskcompleted/list/application/process/{count}/{applicationFlag}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/taskcompleted/list/date/date/hour/hour/manage",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/taskcompleted/list/filter/manage/{page}/{size}/{size}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/taskcompleted/list/job/job",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/taskcompleted/list/my/filter/{page}/{size}/{size}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/taskcompleted/list/my/paging/{page}/{size}/{size}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/taskcompleted/list/next/application/{id}/{count}/{applicationFlag}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/taskcompleted/list/next/filter/{id}/{count}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/taskcompleted/list/next/process/{id}/{count}/{processFlag}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/taskcompleted/list/next/{id}/{count}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/taskcompleted/list/prev/application/{id}/{count}/{applicationFlag}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/taskcompleted/list/prev/filter/{id}/{count}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/taskcompleted/list/prev/manual/{flag}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/taskcompleted/list/prev/process/{id}/{count}/{processFlag}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/taskcompleted/list/prev/{id}/{count}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/taskcompleted/list/work/work",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/taskcompleted/list/workorworkcompleted/{workOrWorkCompleted}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/taskcompleted/manage/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/taskcompleted/v2/list/next/{id}/{count}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/taskcompleted/v2/list/paging/{page}/{size}/{size}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/taskcompleted/v2/list/prev/{id}/{count}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/taskcompleted/{count}/{credential}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/taskcompleted/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/work/application/process/{applicationFlag}/{processFlag}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/work/application/{count}/{credential}/{appId}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/work/assignment/manage/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/work/close/check/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/work/filter/attribute/application/manage/{applicationFlag}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/work/filter/attribute/application/{applicationFlag}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/work/list/application/process/manage/{count}/{applicationFlag}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/work/list/application/process/{count}/{applicationFlag}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/work/list/filter/manage/{page}/{size}/{size}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/work/list/my/paging/{page}/{size}/{size}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/work/list/next/application/filter/manage/{id}/{count}/{applicationFlag}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/work/list/next/application/filter/{id}/{count}/{applicationFlag}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/work/list/next/application/manage/{id}/{count}/{applicationFlag}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/work/list/next/application/{id}/{count}/{applicationFlag}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/work/list/next/creator/current/filter/{id}/{count}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/work/list/next/creator/current/{id}/{count}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/work/list/next/process/{id}/{count}/{processFlag}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/work/list/paging/application/filter/manage/{page}/{size}/{size}/{applicationFlag}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/work/list/prev/application/filter/manage/{id}/{count}/{applicationFlag}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/work/list/prev/application/filter/{id}/{count}/{applicationFlag}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/work/list/prev/application/manage/{id}/{count}/{applicationFlag}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/work/list/prev/application/{id}/{count}/{applicationFlag}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/work/list/prev/creator/current/filter/{id}/{count}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/work/list/prev/creator/current/{id}/{count}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/work/list/prev/process/{id}/{count}/{processFlag}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/work/manage/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/work/process/force/{processFlag}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/work/process/{processFlag}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/work/projection/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/work/refer/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/work/relative/manage/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/work/single/manage/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/work/v2/list/activity/goback/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/work/v2/list/next/{id}/{count}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/work/v2/list/paging/{page}/{size}/{size}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/work/v2/list/prev/{id}/{count}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/work/v2/reroute/mockputtopost/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/work/v2/reroute/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/work/v2/retract/mockputtopost/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/work/v2/retract/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/work/v2/rollback/mockputtopost/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/work/v2/rollback/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/work/v2/terminate/manage/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/work/v2/terminate/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/work/v2/workorworkcompleted/{workOrWorkCompleted}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/work/v3/retract",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/work/v3/retract/stage/job/job",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/work/v3/workorworkcompleted/permission/{workOrWorkCompleted}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/work/workorworkcompleted/{workOrWorkCompleted}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/work/{count}/{credential}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/work/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/workcompleted/assignment/manage/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/workcompleted/filter/attribute/application/manage/{applicationFlag}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/workcompleted/filter/attribute/application/{applicationFlag}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/workcompleted/filter/list/prev/application/{id}/{count}/{applicationFlag}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/workcompleted/list/application/process/manage/{count}/{applicationFlag}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/workcompleted/list/application/process/{count}/{applicationFlag}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/workcompleted/list/filter/manage/{page}/{size}/{size}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/workcompleted/list/next/application/filter/manage/{id}/{count}/{applicationFlag}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/workcompleted/list/next/application/filter/{id}/{count}/{applicationFlag}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/workcompleted/list/next/application/manage/{id}/{count}/{applicationFlag}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/workcompleted/list/next/application/{id}/{count}/{applicationFlag}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/workcompleted/list/paging/application/filter/manage/{page}/{size}/{size}/{applicationFlag}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/workcompleted/list/prev/application/filter/{id}/{count}/{applicationFlag}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/workcompleted/list/prev/application/manage/{id}/{count}/{applicationFlag}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/workcompleted/list/prev/application/{id}/{count}/{applicationFlag}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/workcompleted/manage/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/workcompleted/process/{processFlag}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/workcompleted/rollback/mockputtopost/{flag}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/workcompleted/rollback/{flag}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/workcompleted/shift/time",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/workcompleted/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/worklog/list/job/job",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/worklog/list/rollback/workorworkcompleted/{workOrWorkCompleted}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/processplatform/assemble/surface/worklog/list/workorworkcompleted/{workOrWorkCompleted}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "POST",
        rust_path: "/jaxrs/processplatform/assemble/surface/applicationdict/application/path0/data/mockdeletetoget/{applicationDictFlag}/{applicationFlag}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "POST",
        rust_path: "/jaxrs/processplatform/assemble/surface/applicationdict/application/path0/path1/data/mockdeletetoget/{applicationDictFlag}/{applicationFlag}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "POST",
        rust_path: "/jaxrs/processplatform/assemble/surface/applicationdict/application/path0/path1/path2/data/mockdeletetoget/{applicationDictFlag}/{applicationFlag}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "POST",
        rust_path: "/jaxrs/processplatform/assemble/surface/applicationdict/application/path0/path1/path2/path3/data/mockdeletetoget/{applicationDictFlag}/{applicationFlag}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "POST",
        rust_path: "/jaxrs/processplatform/assemble/surface/applicationdict/application/path0/path1/path2/path3/path4/data/mockdeletetoget/{applicationDictFlag}/{applicationFlag}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "POST",
        rust_path: "/jaxrs/processplatform/assemble/surface/applicationdict/application/path0/path1/path2/path3/path4/path5/data/mockdeletetoget/{applicationDictFlag}/{applicationFlag}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "POST",
        rust_path: "/jaxrs/processplatform/assemble/surface/applicationdict/application/path0/path1/path2/path3/path4/path5/path6/data/mockdeletetoget/{applicationDictFlag}/{applicationFlag}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "POST",
        rust_path: "/jaxrs/processplatform/assemble/surface/applicationdict/application/path0/path1/path2/path3/path4/path5/path6/path7/data/mockdeletetoget/{applicationDictFlag}/{applicationFlag}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "POST",
        rust_path: "/jaxrs/processplatform/assemble/surface/correlation/job/job/delete",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "POST",
        rust_path: "/jaxrs/processplatform/assemble/surface/correlation/update/job/job",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "POST",
        rust_path: "/jaxrs/processplatform/assemble/surface/create",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "POST",
        rust_path: "/jaxrs/processplatform/assemble/surface/data/work/mockdeletetoget/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "POST",
        rust_path: "/jaxrs/processplatform/assemble/surface/data/work/path0/mockdeletetoget/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "POST",
        rust_path: "/jaxrs/processplatform/assemble/surface/data/work/path0/path1/mockdeletetoget/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "POST",
        rust_path: "/jaxrs/processplatform/assemble/surface/data/work/path0/path1/path2/mockdeletetoget/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "POST",
        rust_path: "/jaxrs/processplatform/assemble/surface/data/work/path0/path1/path2/path3/mockdeletetoget/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "POST",
        rust_path: "/jaxrs/processplatform/assemble/surface/data/work/path0/path1/path2/path3/path4/mockdeletetoget/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "POST",
        rust_path: "/jaxrs/processplatform/assemble/surface/data/work/path0/path1/path2/path3/path4/path5/mockdeletetoget/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "POST",
        rust_path: "/jaxrs/processplatform/assemble/surface/data/work/path0/path1/path2/path3/path4/path5/path6/mockdeletetoget/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "POST",
        rust_path: "/jaxrs/processplatform/assemble/surface/data/work/path0/path1/path2/path3/path4/path5/path6/path7/mockdeletetoget/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "POST",
        rust_path: "/jaxrs/processplatform/assemble/surface/delete/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "POST",
        rust_path: "/jaxrs/processplatform/assemble/surface/draft/mockdeletetoget/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "POST",
        rust_path: "/jaxrs/processplatform/assemble/surface/draft/start/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "POST",
        rust_path: "/jaxrs/processplatform/assemble/surface/mode/delete/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "POST",
        rust_path: "/jaxrs/processplatform/assemble/surface/mode/save",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "POST",
        rust_path: "/jaxrs/processplatform/assemble/surface/publish/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "POST",
        rust_path: "/jaxrs/processplatform/assemble/surface/read/manage/mockdeletetoget/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "POST",
        rust_path: "/jaxrs/processplatform/assemble/surface/read/opinion/manage/mockputtopost/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "POST",
        rust_path: "/jaxrs/processplatform/assemble/surface/read/opinion/manage/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "POST",
        rust_path: "/jaxrs/processplatform/assemble/surface/read/processing/manage/mockputtopost/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "POST",
        rust_path: "/jaxrs/processplatform/assemble/surface/read/processing/manage/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "POST",
        rust_path: "/jaxrs/processplatform/assemble/surface/read/processing/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "POST",
        rust_path: "/jaxrs/processplatform/assemble/surface/read/reference/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "POST",
        rust_path: "/jaxrs/processplatform/assemble/surface/read/reset/manage/mockputtopost/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "POST",
        rust_path: "/jaxrs/processplatform/assemble/surface/read/reset/manage/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "POST",
        rust_path: "/jaxrs/processplatform/assemble/surface/read/v2/list/create/next/{id}/{count}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "POST",
        rust_path: "/jaxrs/processplatform/assemble/surface/read/v2/list/create/paging/{page}/{size}/{size}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "POST",
        rust_path: "/jaxrs/processplatform/assemble/surface/read/v2/list/create/prev/{id}/{count}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "POST",
        rust_path: "/jaxrs/processplatform/assemble/surface/readcompleted/manage/mockdeletetoget/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "POST",
        rust_path: "/jaxrs/processplatform/assemble/surface/readcompleted/opinion/manage/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "POST",
        rust_path: "/jaxrs/processplatform/assemble/surface/readcompleted/reference/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "POST",
        rust_path: "/jaxrs/processplatform/assemble/surface/readcompleted/v2/list/create/next/{id}/{count}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "POST",
        rust_path: "/jaxrs/processplatform/assemble/surface/readcompleted/v2/list/create/paging/{page}/{size}/{size}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "POST",
        rust_path: "/jaxrs/processplatform/assemble/surface/readcompleted/v2/list/create/prev/{id}/{count}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "POST",
        rust_path: "/jaxrs/processplatform/assemble/surface/record/manage/mockdeletetoget/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "POST",
        rust_path: "/jaxrs/processplatform/assemble/surface/review/application/manage/mockdeletetoget/{id}/{applicationFlag}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "POST",
        rust_path: "/jaxrs/processplatform/assemble/surface/review/create/work",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "POST",
        rust_path: "/jaxrs/processplatform/assemble/surface/review/create/workcompleted",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "POST",
        rust_path: "/jaxrs/processplatform/assemble/surface/review/filter/create/entry",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "POST",
        rust_path: "/jaxrs/processplatform/assemble/surface/review/v2/list/create/next/{id}/{count}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "POST",
        rust_path: "/jaxrs/processplatform/assemble/surface/review/v2/list/create/paging/{page}/{size}/{size}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "POST",
        rust_path: "/jaxrs/processplatform/assemble/surface/review/v2/list/create/prev/{id}/{count}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "POST",
        rust_path: "/jaxrs/processplatform/assemble/surface/save/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "POST",
        rust_path: "/jaxrs/processplatform/assemble/surface/serialnumber/mockdeletetoget/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "POST",
        rust_path: "/jaxrs/processplatform/assemble/surface/service/work/touch/mockputtopost/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "POST",
        rust_path: "/jaxrs/processplatform/assemble/surface/service/work/touch/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "POST",
        rust_path: "/jaxrs/processplatform/assemble/surface/sign/mockdeletetoget/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "POST",
        rust_path: "/jaxrs/processplatform/assemble/surface/sign/save/task/{taskId}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "POST",
        rust_path: "/jaxrs/processplatform/assemble/surface/sign/task/mockdeletetoget/{taskId}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "POST",
        rust_path: "/jaxrs/processplatform/assemble/surface/task/manage/mockdeletetoget/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "POST",
        rust_path: "/jaxrs/processplatform/assemble/surface/task/opinion/manage/mockputtopost/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "POST",
        rust_path: "/jaxrs/processplatform/assemble/surface/task/opinion/manage/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "POST",
        rust_path: "/jaxrs/processplatform/assemble/surface/task/press/manage/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "POST",
        rust_path: "/jaxrs/processplatform/assemble/surface/task/processing/manage/mockputtopost/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "POST",
        rust_path: "/jaxrs/processplatform/assemble/surface/task/processing/manage/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "POST",
        rust_path: "/jaxrs/processplatform/assemble/surface/task/processing/neural/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "POST",
        rust_path: "/jaxrs/processplatform/assemble/surface/task/processing/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "POST",
        rust_path: "/jaxrs/processplatform/assemble/surface/task/reference/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "POST",
        rust_path: "/jaxrs/processplatform/assemble/surface/task/reset/manage/mockputtopost/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "POST",
        rust_path: "/jaxrs/processplatform/assemble/surface/task/reset/manage/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "POST",
        rust_path: "/jaxrs/processplatform/assemble/surface/task/v2/list/create/next/{id}/{count}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "POST",
        rust_path: "/jaxrs/processplatform/assemble/surface/task/v2/list/create/paging/{page}/{size}/{size}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "POST",
        rust_path: "/jaxrs/processplatform/assemble/surface/task/v2/list/create/prev/{id}/{count}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "POST",
        rust_path: "/jaxrs/processplatform/assemble/surface/task/v2/reset/mockputtopost/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "POST",
        rust_path: "/jaxrs/processplatform/assemble/surface/task/v2/reset/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "POST",
        rust_path: "/jaxrs/processplatform/assemble/surface/task/v2/resume/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "POST",
        rust_path: "/jaxrs/processplatform/assemble/surface/task/v2/trigger/processing/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "POST",
        rust_path: "/jaxrs/processplatform/assemble/surface/task/v3/add/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "POST",
        rust_path: "/jaxrs/processplatform/assemble/surface/task/will/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "POST",
        rust_path: "/jaxrs/processplatform/assemble/surface/taskcompleted/manage/mockdeletetoget/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "POST",
        rust_path: "/jaxrs/processplatform/assemble/surface/taskcompleted/opinion/manage/mockputtopost/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "POST",
        rust_path: "/jaxrs/processplatform/assemble/surface/taskcompleted/opinion/manage/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "POST",
        rust_path: "/jaxrs/processplatform/assemble/surface/taskcompleted/press/work/work",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "POST",
        rust_path: "/jaxrs/processplatform/assemble/surface/taskcompleted/reference/control/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "POST",
        rust_path: "/jaxrs/processplatform/assemble/surface/taskcompleted/reference/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "POST",
        rust_path: "/jaxrs/processplatform/assemble/surface/taskcompleted/v2/list/create/next/{id}/{count}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "POST",
        rust_path: "/jaxrs/processplatform/assemble/surface/taskcompleted/v2/list/create/paging/{page}/{size}/{size}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "POST",
        rust_path: "/jaxrs/processplatform/assemble/surface/taskcompleted/v2/list/create/prev/{id}/{count}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "POST",
        rust_path: "/jaxrs/processplatform/assemble/surface/work/mockdeletetoget/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "POST",
        rust_path: "/jaxrs/processplatform/assemble/surface/work/processing/mockputtopost/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "POST",
        rust_path: "/jaxrs/processplatform/assemble/surface/work/processing/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "POST",
        rust_path: "/jaxrs/processplatform/assemble/surface/work/relative/manage/mockdeletetoget/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "POST",
        rust_path: "/jaxrs/processplatform/assemble/surface/work/single/manage/mockdeletetoget/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "POST",
        rust_path: "/jaxrs/processplatform/assemble/surface/work/v2/add/split/mockputtopost/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "POST",
        rust_path: "/jaxrs/processplatform/assemble/surface/work/v2/add/split/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "POST",
        rust_path: "/jaxrs/processplatform/assemble/surface/work/v2/trigger/processing/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "POST",
        rust_path: "/jaxrs/processplatform/assemble/surface/workcompleted/delete/manage/mockdeletetoget/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "POST",
        rust_path: "/jaxrs/processplatform/assemble/surface/workcompleted/delete/manage/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_assemble_surface",
        method: "POST",
        rust_path: "/jaxrs/processplatform/assemble/surface/worklog/list/add/split/work/{workId}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_core_entity",
        method: "GET",
        rust_path: "/jaxrs/process/task/list",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_core_entity",
        method: "GET",
        rust_path: "/jaxrs/process/task/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_core_entity",
        method: "GET",
        rust_path: "/jaxrs/process/ticket/list",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_core_entity",
        method: "GET",
        rust_path: "/jaxrs/process/work/list",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_core_entity",
        method: "GET",
        rust_path: "/jaxrs/process/work/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_core_entity",
        method: "GET",
        rust_path: "/jaxrs/process/workcompleted/list",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_core_express",
        method: "GET",
        rust_path: "/jaxrs/processplatform/task/count/with/person/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_core_express",
        method: "GET",
        rust_path: "/jaxrs/processplatform/task/processing/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_core_express",
        method: "GET",
        rust_path: "/jaxrs/processplatform/work/count/with/person/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_core_express",
        method: "GET",
        rust_path: "/jaxrs/processplatform/work/processing/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_core_express",
        method: "GET",
        rust_path: "/jaxrs/processplatform/work/retract/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_core_express",
        method: "GET",
        rust_path: "/jaxrs/processplatform/work/terminate/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_service_processing",
        method: "GET",
        rust_path: "/jaxrs/processplatform/service/processing/applicationdict/path0/data/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_service_processing",
        method: "GET",
        rust_path: "/jaxrs/processplatform/service/processing/applicationdict/path0/path1/data/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_service_processing",
        method: "GET",
        rust_path: "/jaxrs/processplatform/service/processing/applicationdict/path0/path1/path2/data/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_service_processing",
        method: "GET",
        rust_path: "/jaxrs/processplatform/service/processing/applicationdict/path0/path1/path2/path3/data/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_service_processing",
        method: "GET",
        rust_path: "/jaxrs/processplatform/service/processing/applicationdict/path0/path1/path2/path3/path4/data/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_service_processing",
        method: "GET",
        rust_path: "/jaxrs/processplatform/service/processing/applicationdict/path0/path1/path2/path3/path4/path5/data/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_service_processing",
        method: "GET",
        rust_path: "/jaxrs/processplatform/service/processing/applicationdict/path0/path1/path2/path3/path4/path5/path6/data/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_service_processing",
        method: "GET",
        rust_path: "/jaxrs/processplatform/service/processing/applicationdict/path0/path1/path2/path3/path4/path5/path6/path7/data/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_service_processing",
        method: "GET",
        rust_path: "/jaxrs/processplatform/service/processing/applicationdict/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_service_processing",
        method: "GET",
        rust_path: "/jaxrs/processplatform/service/processing/attachment/workcompleted/{id}/{workCompletedId}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_service_processing",
        method: "GET",
        rust_path: "/jaxrs/processplatform/service/processing/attachment/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_service_processing",
        method: "GET",
        rust_path: "/jaxrs/processplatform/service/processing/attachment/{id}/{work}/{workId}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_service_processing",
        method: "GET",
        rust_path: "/jaxrs/processplatform/service/processing/data/path/{job}/{job}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_service_processing",
        method: "GET",
        rust_path: "/jaxrs/processplatform/service/processing/data/workcompleted/path/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_service_processing",
        method: "GET",
        rust_path: "/jaxrs/processplatform/service/processing/data/workcompleted/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_service_processing",
        method: "GET",
        rust_path: "/jaxrs/processplatform/service/processing/data/{job}/{job}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_service_processing",
        method: "GET",
        rust_path: "/jaxrs/processplatform/service/processing/documentversion/{work}/{work}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_service_processing",
        method: "GET",
        rust_path: "/jaxrs/processplatform/service/processing/draft/{work}/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_service_processing",
        method: "GET",
        rust_path: "/jaxrs/processplatform/service/processing/form/suitable/activity/{activityId}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_service_processing",
        method: "GET",
        rust_path: "/jaxrs/processplatform/service/processing/get/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_service_processing",
        method: "GET",
        rust_path: "/jaxrs/processplatform/service/processing/instance/{executionId}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_service_processing",
        method: "GET",
        rust_path: "/jaxrs/processplatform/service/processing/list/{category}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_service_processing",
        method: "GET",
        rust_path: "/jaxrs/processplatform/service/processing/manual/append/identity/{work}/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_service_processing",
        method: "GET",
        rust_path: "/jaxrs/processplatform/service/processing/process/{id}/complex",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_service_processing",
        method: "GET",
        rust_path: "/jaxrs/processplatform/service/processing/process/{work}/{processId}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_service_processing",
        method: "GET",
        rust_path: "/jaxrs/processplatform/service/processing/process/{work}/{processId}/{name}/{name}/{serial}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_service_processing",
        method: "GET",
        rust_path: "/jaxrs/processplatform/service/processing/projection/{work}/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_service_processing",
        method: "GET",
        rust_path: "/jaxrs/processplatform/service/processing/readcompleted/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_service_processing",
        method: "GET",
        rust_path: "/jaxrs/processplatform/service/processing/record/terminate/{work}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_service_processing",
        method: "GET",
        rust_path: "/jaxrs/processplatform/service/processing/record/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_service_processing",
        method: "GET",
        rust_path: "/jaxrs/processplatform/service/processing/record/{job}/{job}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_service_processing",
        method: "GET",
        rust_path: "/jaxrs/processplatform/service/processing/review/init/review",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_service_processing",
        method: "GET",
        rust_path: "/jaxrs/processplatform/service/processing/review/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_service_processing",
        method: "GET",
        rust_path: "/jaxrs/processplatform/service/processing/snap/abandoned/{work}/{workId}/{type}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_service_processing",
        method: "GET",
        rust_path: "/jaxrs/processplatform/service/processing/snap/restore/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_service_processing",
        method: "GET",
        rust_path: "/jaxrs/processplatform/service/processing/snap/snap/{work}/{workId}/{type}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_service_processing",
        method: "GET",
        rust_path: "/jaxrs/processplatform/service/processing/snap/suspend/{work}/{workId}/{type}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_service_processing",
        method: "GET",
        rust_path: "/jaxrs/processplatform/service/processing/snap/workcompleted/abandonedworkcompleted/{workCompletedId}/{type}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_service_processing",
        method: "GET",
        rust_path: "/jaxrs/processplatform/service/processing/snap/workcompleted/snapworkcompleted/{workCompletedId}/{type}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_service_processing",
        method: "GET",
        rust_path: "/jaxrs/processplatform/service/processing/snap/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_service_processing",
        method: "GET",
        rust_path: "/jaxrs/processplatform/service/processing/task/expire/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_service_processing",
        method: "GET",
        rust_path: "/jaxrs/processplatform/service/processing/task/pass/expired/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_service_processing",
        method: "GET",
        rust_path: "/jaxrs/processplatform/service/processing/task/replace/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_service_processing",
        method: "GET",
        rust_path: "/jaxrs/processplatform/service/processing/task/urge/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_service_processing",
        method: "GET",
        rust_path: "/jaxrs/processplatform/service/processing/task/v2/pause/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_service_processing",
        method: "GET",
        rust_path: "/jaxrs/processplatform/service/processing/task/v2/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_service_processing",
        method: "GET",
        rust_path: "/jaxrs/processplatform/service/processing/task/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_service_processing",
        method: "GET",
        rust_path: "/jaxrs/processplatform/service/processing/taskcompleted/task/identity/{next}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_service_processing",
        method: "GET",
        rust_path: "/jaxrs/processplatform/service/processing/taskcompleted/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_service_processing",
        method: "GET",
        rust_path: "/jaxrs/processplatform/service/processing/v2/goback/{work}/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_service_processing",
        method: "GET",
        rust_path: "/jaxrs/processplatform/service/processing/v2/projection/{job}/{job}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_service_processing",
        method: "GET",
        rust_path: "/jaxrs/processplatform/service/processing/v2/reroute/{work}/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_service_processing",
        method: "GET",
        rust_path: "/jaxrs/processplatform/service/processing/v2/rollback/{work}/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_service_processing",
        method: "GET",
        rust_path: "/jaxrs/processplatform/service/processing/v2/{job}/{job}/{person}/{person}/{view}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_service_processing",
        method: "GET",
        rust_path: "/jaxrs/processplatform/service/processing/v3/retract/{work}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_service_processing",
        method: "GET",
        rust_path: "/jaxrs/processplatform/service/processing/work/list",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_service_processing",
        method: "GET",
        rust_path: "/jaxrs/processplatform/service/processing/workcompleted/process/{processFlag}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_service_processing",
        method: "GET",
        rust_path: "/jaxrs/processplatform/service/processing/workcompleted/rollback/{flag}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_service_processing",
        method: "GET",
        rust_path: "/jaxrs/processplatform/service/processing/workcompleted/shift/time",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_service_processing",
        method: "GET",
        rust_path: "/jaxrs/processplatform/service/processing/{job}/{job}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_service_processing",
        method: "POST",
        rust_path: "/jaxrs/gateway/{work_id}/{activity_token}/join",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_service_processing",
        method: "POST",
        rust_path: "/jaxrs/processplatform/service/processing/attachment/copy/workcompleted/{workCompletedId}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_service_processing",
        method: "POST",
        rust_path: "/jaxrs/processplatform/service/processing/attachment/copy/{work}/{workId}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_service_processing",
        method: "POST",
        rust_path: "/jaxrs/processplatform/service/processing/attachment/edit/text/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_service_processing",
        method: "POST",
        rust_path: "/jaxrs/processplatform/service/processing/cancel/{executionId}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_service_processing",
        method: "POST",
        rust_path: "/jaxrs/processplatform/service/processing/create",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_service_processing",
        method: "POST",
        rust_path: "/jaxrs/processplatform/service/processing/data/delete/{work}/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_service_processing",
        method: "POST",
        rust_path: "/jaxrs/processplatform/service/processing/data/path/delete/{work}/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_service_processing",
        method: "POST",
        rust_path: "/jaxrs/processplatform/service/processing/execute/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_service_processing",
        method: "POST",
        rust_path: "/jaxrs/processplatform/service/processing/gateway/fork/{gateway_instance_id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_service_processing",
        method: "POST",
        rust_path: "/jaxrs/processplatform/service/processing/manual/after/processing/{work}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_service_processing",
        method: "POST",
        rust_path: "/jaxrs/processplatform/service/processing/record/processing/{work}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_service_processing",
        method: "POST",
        rust_path: "/jaxrs/processplatform/service/processing/record/task/processing",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_service_processing",
        method: "POST",
        rust_path: "/jaxrs/processplatform/service/processing/series/series/activitytoken/processing/signal/{work}/{id}/{activityToken}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_service_processing",
        method: "POST",
        rust_path: "/jaxrs/processplatform/service/processing/service/touch/{work}/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_service_processing",
        method: "POST",
        rust_path: "/jaxrs/processplatform/service/processing/task/press/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_service_processing",
        method: "POST",
        rust_path: "/jaxrs/processplatform/service/processing/task/processing/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_service_processing",
        method: "POST",
        rust_path: "/jaxrs/processplatform/service/processing/task/v2/reset/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_service_processing",
        method: "POST",
        rust_path: "/jaxrs/processplatform/service/processing/task/v2/resume/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_service_processing",
        method: "POST",
        rust_path: "/jaxrs/processplatform/service/processing/task/v3/add/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_service_processing",
        method: "POST",
        rust_path: "/jaxrs/processplatform/service/processing/task/will/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_service_processing",
        method: "POST",
        rust_path: "/jaxrs/processplatform/service/processing/taskcompleted/press/{id}/{work}/{work}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_service_processing",
        method: "POST",
        rust_path: "/jaxrs/processplatform/service/processing/timer/start",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_service_processing",
        method: "POST",
        rust_path: "/jaxrs/processplatform/service/processing/timer/{job_id}/cancel",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_service_processing",
        method: "POST",
        rust_path: "/jaxrs/processplatform/service/processing/touch/deletedraft",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_service_processing",
        method: "POST",
        rust_path: "/jaxrs/processplatform/service/processing/touch/handoverjob",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_service_processing",
        method: "POST",
        rust_path: "/jaxrs/processplatform/service/processing/touch/merge",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_service_processing",
        method: "POST",
        rust_path: "/jaxrs/processplatform/service/processing/touch/mergeitem",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_service_processing",
        method: "POST",
        rust_path: "/jaxrs/processplatform/service/processing/touch/touchdelay",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_service_processing",
        method: "POST",
        rust_path: "/jaxrs/processplatform/service/processing/touch/urge",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_service_processing",
        method: "POST",
        rust_path: "/jaxrs/processplatform/service/processing/v2/add/split/{work}/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_service_processing",
        method: "POST",
        rust_path: "/jaxrs/processplatform/service/processing/workcompleted/merge/{flag}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_service_processing",
        method: "POST",
        rust_path: "/jaxrs/task/{id}/claim",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_service_processing",
        method: "POST",
        rust_path: "/jaxrs/task/{id}/complete",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_service_processing",
        method: "POST",
        rust_path: "/jaxrs/task/{id}/reject",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_service_processing",
        method: "POST",
        rust_path: "/jaxrs/task/{id}/transfer/{person}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_service_processing",
        method: "POST",
        rust_path: "/jaxrs/work/{id}/complete",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_service_processing",
        method: "POST",
        rust_path: "/jaxrs/work/{id}/retract",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_service_processing",
        method: "POST",
        rust_path: "/jaxrs/work/{id}/start",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_service_processing",
        method: "POST",
        rust_path: "/jaxrs/work/{id}/terminate",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "processplatform_service_processing",
        method: "PUT",
        rust_path: "/jaxrs/work/{id}/processing",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "program_center",
        method: "DELETE",
        rust_path: "/jaxrs/program_center/appstyle/image/application/top/erase",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "program_center",
        method: "DELETE",
        rust_path: "/jaxrs/program_center/appstyle/image/launch/logo/erase",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "program_center",
        method: "DELETE",
        rust_path: "/jaxrs/program_center/appstyle/image/login/avatar/erase",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "program_center",
        method: "DELETE",
        rust_path: "/jaxrs/program_center/appstyle/image/menu/logo/blur/erase",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "program_center",
        method: "DELETE",
        rust_path: "/jaxrs/program_center/appstyle/image/menu/logo/focus/erase",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "program_center",
        method: "DELETE",
        rust_path: "/jaxrs/program_center/appstyle/image/process/default/erase",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "program_center",
        method: "DELETE",
        rust_path: "/jaxrs/program_center/appstyle/image/setup/about/logo/erase",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "program_center",
        method: "DELETE",
        rust_path: "/jaxrs/program_center/dict/dictFlag/path/data/mockdeletetoget",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "program_center",
        method: "DELETE",
        rust_path: "/jaxrs/program_center/mpweixin/menu/delete/id",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "program_center",
        method: "GET",
        rust_path: "/jaxrs/program/applications",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "program_center",
        method: "GET",
        rust_path: "/jaxrs/program/appstyle/current/style",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "program_center",
        method: "GET",
        rust_path: "/jaxrs/program/datastructure/modules/all",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "program_center",
        method: "GET",
        rust_path: "/jaxrs/program_center/agent/flag",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "program_center",
        method: "GET",
        rust_path: "/jaxrs/program_center/agent/flag/disable",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "program_center",
        method: "GET",
        rust_path: "/jaxrs/program_center/agent/flag/enable",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "program_center",
        method: "GET",
        rust_path: "/jaxrs/program_center/agent/flag/execute",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "program_center",
        method: "GET",
        rust_path: "/jaxrs/program_center/agent/flag/file",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "program_center",
        method: "GET",
        rust_path: "/jaxrs/program_center/andfx/pull/sync",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "program_center",
        method: "GET",
        rust_path: "/jaxrs/program_center/appstyle/current/style",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "program_center",
        method: "GET",
        rust_path: "/jaxrs/program_center/appstyle/image/application/top",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "program_center",
        method: "GET",
        rust_path: "/jaxrs/program_center/appstyle/image/launch/logo",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "program_center",
        method: "GET",
        rust_path: "/jaxrs/program_center/appstyle/image/login/avatar",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "program_center",
        method: "GET",
        rust_path: "/jaxrs/program_center/appstyle/image/menu/logo/blur",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "program_center",
        method: "GET",
        rust_path: "/jaxrs/program_center/appstyle/image/menu/logo/focus",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "program_center",
        method: "GET",
        rust_path: "/jaxrs/program_center/appstyle/image/process/default",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "program_center",
        method: "GET",
        rust_path: "/jaxrs/program_center/appstyle/image/setup/about/logo",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "program_center",
        method: "GET",
        rust_path: "/jaxrs/program_center/appstyle/index/portal",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "program_center",
        method: "GET",
        rust_path: "/jaxrs/program_center/bar/select1/field/field/value/value/count/count",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "program_center",
        method: "GET",
        rust_path: "/jaxrs/program_center/bar/select2/count/count",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "program_center",
        method: "GET",
        rust_path: "/jaxrs/program_center/bar/select3/field/field/value/value/count/count",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "program_center",
        method: "GET",
        rust_path: "/jaxrs/program_center/bar/select4/field/field/value/value/count/count",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "program_center",
        method: "GET",
        rust_path: "/jaxrs/program_center/captcha/id/validate/answer/answer",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "program_center",
        method: "GET",
        rust_path: "/jaxrs/program_center/captcha/list",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "program_center",
        method: "GET",
        rust_path: "/jaxrs/program_center/center/applications",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "program_center",
        method: "GET",
        rust_path: "/jaxrs/program_center/center/regist/applications",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "program_center",
        method: "GET",
        rust_path: "/jaxrs/program_center/center/version",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "program_center",
        method: "GET",
        rust_path: "/jaxrs/program_center/code/list",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "program_center",
        method: "GET",
        rust_path: "/jaxrs/program_center/code/list/paging/page/size/size",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "program_center",
        method: "GET",
        rust_path: "/jaxrs/program_center/code/validate/mobile/mobile/answer/answer",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "program_center",
        method: "GET",
        rust_path: "/jaxrs/program_center/code/validate/mobile/mobile/answer/answer/cascade",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "program_center",
        method: "GET",
        rust_path: "/jaxrs/program_center/collect/code/mobile/mobile",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "program_center",
        method: "GET",
        rust_path: "/jaxrs/program_center/collect/connect",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "program_center",
        method: "GET",
        rust_path: "/jaxrs/program_center/collect/controllebbs",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "program_center",
        method: "GET",
        rust_path: "/jaxrs/program_center/collect/controllermobile/name/name/mobile/mobile",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "program_center",
        method: "GET",
        rust_path: "/jaxrs/program_center/collect/disconnect",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "program_center",
        method: "GET",
        rust_path: "/jaxrs/program_center/collect/list",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "program_center",
        method: "GET",
        rust_path: "/jaxrs/program_center/collect/login",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "program_center",
        method: "GET",
        rust_path: "/jaxrs/program_center/collect/mobile/check/connect",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "program_center",
        method: "GET",
        rust_path: "/jaxrs/program_center/collect/name/name/exist",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "program_center",
        method: "GET",
        rust_path: "/jaxrs/program_center/collect/name/name/mobile/mobile/code/code",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "program_center",
        method: "GET",
        rust_path: "/jaxrs/program_center/collect/person",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "program_center",
        method: "GET",
        rust_path: "/jaxrs/program_center/collect/remove",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "program_center",
        method: "GET",
        rust_path: "/jaxrs/program_center/collect/resetpassword",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "program_center",
        method: "GET",
        rust_path: "/jaxrs/program_center/collect/sync/area",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "program_center",
        method: "GET",
        rust_path: "/jaxrs/program_center/collect/urlMapping",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "program_center",
        method: "GET",
        rust_path: "/jaxrs/program_center/collect/validate",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "program_center",
        method: "GET",
        rust_path: "/jaxrs/program_center/collect/validate/codeanswer",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "program_center",
        method: "GET",
        rust_path: "/jaxrs/program_center/collect/validate/direct",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "program_center",
        method: "GET",
        rust_path: "/jaxrs/program_center/collect/validate/password",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "program_center",
        method: "GET",
        rust_path: "/jaxrs/program_center/command/execute",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "program_center",
        method: "GET",
        rust_path: "/jaxrs/program_center/command/list/node",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "program_center",
        method: "GET",
        rust_path: "/jaxrs/program_center/config/centerserver",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "program_center",
        method: "GET",
        rust_path: "/jaxrs/program_center/config/change/password",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "program_center",
        method: "GET",
        rust_path: "/jaxrs/program_center/config/collect",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "program_center",
        method: "GET",
        rust_path: "/jaxrs/program_center/config/get",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "program_center",
        method: "GET",
        rust_path: "/jaxrs/program_center/config/license",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "program_center",
        method: "GET",
        rust_path: "/jaxrs/program_center/config/list",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "program_center",
        method: "GET",
        rust_path: "/jaxrs/program_center/config/list/application",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "program_center",
        method: "GET",
        rust_path: "/jaxrs/program_center/config/list/dump/data",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "program_center",
        method: "GET",
        rust_path: "/jaxrs/program_center/config/list/dump/data/current/node",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "program_center",
        method: "GET",
        rust_path: "/jaxrs/program_center/config/list/entity",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "program_center",
        method: "GET",
        rust_path: "/jaxrs/program_center/config/open",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "program_center",
        method: "GET",
        rust_path: "/jaxrs/program_center/config/open/get/disable/export/enable",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "program_center",
        method: "GET",
        rust_path: "/jaxrs/program_center/config/open/run/time/config",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "program_center",
        method: "GET",
        rust_path: "/jaxrs/program_center/config/person",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "program_center",
        method: "GET",
        rust_path: "/jaxrs/program_center/config/portal",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "program_center",
        method: "GET",
        rust_path: "/jaxrs/program_center/config/proxy",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "program_center",
        method: "GET",
        rust_path: "/jaxrs/program_center/config/ternary/management",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "program_center",
        method: "GET",
        rust_path: "/jaxrs/program_center/config/token",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "program_center",
        method: "GET",
        rust_path: "/jaxrs/program_center/datastructure/fileds/all",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "program_center",
        method: "GET",
        rust_path: "/jaxrs/program_center/datastructure/modules/all",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "program_center",
        method: "GET",
        rust_path: "/jaxrs/program_center/datastructure/tables/all",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "program_center",
        method: "GET",
        rust_path: "/jaxrs/program_center/deploy/id",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "program_center",
        method: "GET",
        rust_path: "/jaxrs/program_center/deploy/list/paging/page/size/size",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "program_center",
        method: "GET",
        rust_path: "/jaxrs/program_center/deploy/server/o2",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "program_center",
        method: "GET",
        rust_path: "/jaxrs/program_center/deploy/server/resource",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "program_center",
        method: "GET",
        rust_path: "/jaxrs/program_center/deploy/web/resource/as/new/asNew",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "program_center",
        method: "GET",
        rust_path: "/jaxrs/program_center/designer/search",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "program_center",
        method: "GET",
        rust_path: "/jaxrs/program_center/dict/dictFlag/data",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "program_center",
        method: "GET",
        rust_path: "/jaxrs/program_center/dict/dictFlag/path/data",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "program_center",
        method: "GET",
        rust_path: "/jaxrs/program_center/dict/id",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "program_center",
        method: "GET",
        rust_path: "/jaxrs/program_center/dict/list",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "program_center",
        method: "GET",
        rust_path: "/jaxrs/program_center/dict/list/paging/page/size/size",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "program_center",
        method: "GET",
        rust_path: "/jaxrs/program_center/dingding/get/callback/aes",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "program_center",
        method: "GET",
        rust_path: "/jaxrs/program_center/dingding/pull/sync",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "program_center",
        method: "GET",
        rust_path: "/jaxrs/program_center/dingding/request/pull/sync",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "program_center",
        method: "GET",
        rust_path: "/jaxrs/program_center/dingding/sync/organization/callback",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "program_center",
        method: "GET",
        rust_path: "/jaxrs/program_center/dingding/sync/organization/register/callback/enable",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "program_center",
        method: "GET",
        rust_path: "/jaxrs/program_center/distribute/assemble/source/source",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "program_center",
        method: "GET",
        rust_path: "/jaxrs/program_center/distribute/webserver/assemble/source/source",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "program_center",
        method: "GET",
        rust_path: "/jaxrs/program_center/invoke/flag",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "program_center",
        method: "GET",
        rust_path: "/jaxrs/program_center/invoke/flag/client/client/token/token/execute",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "program_center",
        method: "GET",
        rust_path: "/jaxrs/program_center/invoke/flag/execute",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "program_center",
        method: "GET",
        rust_path: "/jaxrs/program_center/invoke/flag/execute/get",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "program_center",
        method: "GET",
        rust_path: "/jaxrs/program_center/invoke/flag/file",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "program_center",
        method: "GET",
        rust_path: "/jaxrs/program_center/invoke/list/category",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "program_center",
        method: "GET",
        rust_path: "/jaxrs/program_center/invoke/list/with/category/category",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "program_center",
        method: "GET",
        rust_path: "/jaxrs/program_center/invoke/token",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "program_center",
        method: "GET",
        rust_path: "/jaxrs/program_center/jest/clear/cache/source",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "program_center",
        method: "GET",
        rust_path: "/jaxrs/program_center/jest/list",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "program_center",
        method: "GET",
        rust_path: "/jaxrs/program_center/jest/version",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "program_center",
        method: "GET",
        rust_path: "/jaxrs/program_center/market/cloud/unit/is/vip",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "program_center",
        method: "GET",
        rust_path: "/jaxrs/program_center/market/flag",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "program_center",
        method: "GET",
        rust_path: "/jaxrs/program_center/market/flag/cover/pic",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "program_center",
        method: "GET",
        rust_path: "/jaxrs/program_center/market/flag/install/log",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "program_center",
        method: "GET",
        rust_path: "/jaxrs/program_center/market/flag/installed/version",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "program_center",
        method: "GET",
        rust_path: "/jaxrs/program_center/market/flag/uninstall",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "program_center",
        method: "GET",
        rust_path: "/jaxrs/program_center/market/id/download",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "program_center",
        method: "GET",
        rust_path: "/jaxrs/program_center/market/install/offline",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "program_center",
        method: "GET",
        rust_path: "/jaxrs/program_center/market/list/category",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "program_center",
        method: "GET",
        rust_path: "/jaxrs/program_center/market/list/install/log/paging/page/size/size",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "program_center",
        method: "GET",
        rust_path: "/jaxrs/program_center/market/list/paging/page/size/size",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "program_center",
        method: "GET",
        rust_path: "/jaxrs/program_center/market/list/paging/page/size/size/category/category",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "program_center",
        method: "GET",
        rust_path: "/jaxrs/program_center/market/list/top/three",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "program_center",
        method: "GET",
        rust_path: "/jaxrs/program_center/module/id/compare",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "program_center",
        method: "GET",
        rust_path: "/jaxrs/program_center/module/list",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "program_center",
        method: "GET",
        rust_path: "/jaxrs/program_center/module/list/category",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "program_center",
        method: "GET",
        rust_path: "/jaxrs/program_center/module/remove/structure/id",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "program_center",
        method: "GET",
        rust_path: "/jaxrs/program_center/module/write/flag",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "program_center",
        method: "GET",
        rust_path: "/jaxrs/program_center/mpweixin/check",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "program_center",
        method: "GET",
        rust_path: "/jaxrs/program_center/mpweixin/menu/subscribe",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "program_center",
        method: "GET",
        rust_path: "/jaxrs/program_center/mpweixin/message/template/send",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "program_center",
        method: "GET",
        rust_path: "/jaxrs/program_center/prompterrorlog/count/exceptionclass",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "program_center",
        method: "GET",
        rust_path: "/jaxrs/program_center/prompterrorlog/count/loggername",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "program_center",
        method: "GET",
        rust_path: "/jaxrs/program_center/prompterrorlog/id",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "program_center",
        method: "GET",
        rust_path: "/jaxrs/program_center/prompterrorlog/list/id/next/count",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "program_center",
        method: "GET",
        rust_path: "/jaxrs/program_center/prompterrorlog/list/id/next/count/date/date",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "program_center",
        method: "GET",
        rust_path: "/jaxrs/program_center/prompterrorlog/list/id/next/count/exceptionclass/exceptionClass",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "program_center",
        method: "GET",
        rust_path: "/jaxrs/program_center/prompterrorlog/list/id/next/count/loggername/loggerName",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "program_center",
        method: "GET",
        rust_path: "/jaxrs/program_center/prompterrorlog/list/id/prev/count",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "program_center",
        method: "GET",
        rust_path: "/jaxrs/program_center/prompterrorlog/list/id/prev/count/date/date",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "program_center",
        method: "GET",
        rust_path: "/jaxrs/program_center/prompterrorlog/list/id/prev/count/exceptionclass/exceptionClass",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "program_center",
        method: "GET",
        rust_path: "/jaxrs/program_center/prompterrorlog/list/id/prev/count/loggername/loggerName",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "program_center",
        method: "GET",
        rust_path: "/jaxrs/program_center/qiyeweixin/get/callback/aes",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "program_center",
        method: "GET",
        rust_path: "/jaxrs/program_center/qiyeweixin/pull/sync",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "program_center",
        method: "GET",
        rust_path: "/jaxrs/program_center/qiyeweixin/request/pull/sync",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "program_center",
        method: "GET",
        rust_path: "/jaxrs/program_center/qiyeweixin/send/getprivateinfo/message",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "program_center",
        method: "GET",
        rust_path: "/jaxrs/program_center/schedule/list/schedule",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "program_center",
        method: "GET",
        rust_path: "/jaxrs/program_center/schedule/list/schedulelocal",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "program_center",
        method: "GET",
        rust_path: "/jaxrs/program_center/schedule/list/schedulelog/application/application",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "program_center",
        method: "GET",
        rust_path: "/jaxrs/program_center/schedule/report",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "program_center",
        method: "GET",
        rust_path: "/jaxrs/program_center/schedule/schedule/fire",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "program_center",
        method: "GET",
        rust_path: "/jaxrs/program_center/script/flag",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "program_center",
        method: "GET",
        rust_path: "/jaxrs/program_center/script/id",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "program_center",
        method: "GET",
        rust_path: "/jaxrs/program_center/script/list",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "program_center",
        method: "GET",
        rust_path: "/jaxrs/program_center/script/list/paging/page/size/size",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "program_center",
        method: "GET",
        rust_path: "/jaxrs/program_center/script/name/name",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "program_center",
        method: "GET",
        rust_path: "/jaxrs/program_center/script/name/name/imported",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "program_center",
        method: "GET",
        rust_path: "/jaxrs/program_center/test/test1",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "program_center",
        method: "GET",
        rust_path: "/jaxrs/program_center/test/test2",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "program_center",
        method: "GET",
        rust_path: "/jaxrs/program_center/unexpectederrorlog/id",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "program_center",
        method: "GET",
        rust_path: "/jaxrs/program_center/unexpectederrorlog/list/id/next/count",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "program_center",
        method: "GET",
        rust_path: "/jaxrs/program_center/unexpectederrorlog/list/id/next/count/date/date",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "program_center",
        method: "GET",
        rust_path: "/jaxrs/program_center/unexpectederrorlog/list/id/prev/count",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "program_center",
        method: "GET",
        rust_path: "/jaxrs/program_center/unexpectederrorlog/list/id/prev/count/date/date",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "program_center",
        method: "GET",
        rust_path: "/jaxrs/program_center/validation/meta",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "program_center",
        method: "GET",
        rust_path: "/jaxrs/program_center/validation/scripting/benchmark",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "program_center",
        method: "GET",
        rust_path: "/jaxrs/program_center/validation/timeout/timeout",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "program_center",
        method: "GET",
        rust_path: "/jaxrs/program_center/zhengwudingding/pull/sync",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "program_center",
        method: "GET",
        rust_path: "/jaxrs/program_center/zhengwudingding/regist/callback",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "program_center",
        method: "GET",
        rust_path: "/jaxrs/program_center/zhengwudingding/sync/organization/callback",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "program_center",
        method: "POST",
        rust_path: "/jaxrs/program_center/agent/create",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "program_center",
        method: "POST",
        rust_path: "/jaxrs/program_center/agent/save/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "program_center",
        method: "POST",
        rust_path: "/jaxrs/program_center/application/create",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "program_center",
        method: "POST",
        rust_path: "/jaxrs/program_center/application/save/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "program_center",
        method: "POST",
        rust_path: "/jaxrs/program_center/appstyle/current/update",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "program_center",
        method: "POST",
        rust_path: "/jaxrs/program_center/bar/create/mass/from/count",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "program_center",
        method: "POST",
        rust_path: "/jaxrs/program_center/captcha/v2/create/width/width/height/height",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "program_center",
        method: "POST",
        rust_path: "/jaxrs/program_center/code/create/mobile/mobile",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "program_center",
        method: "POST",
        rust_path: "/jaxrs/program_center/collect/add",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "program_center",
        method: "POST",
        rust_path: "/jaxrs/program_center/collect/updateUnit",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "program_center",
        method: "POST",
        rust_path: "/jaxrs/program_center/config/save",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "program_center",
        method: "POST",
        rust_path: "/jaxrs/program_center/dict/dictFlag/path/data/mockputtopost",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "program_center",
        method: "POST",
        rust_path: "/jaxrs/program_center/foo/create/mass/from/count",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "program_center",
        method: "POST",
        rust_path: "/jaxrs/program_center/input/compare",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "program_center",
        method: "POST",
        rust_path: "/jaxrs/program_center/input/cover",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "program_center",
        method: "POST",
        rust_path: "/jaxrs/program_center/input/create",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "program_center",
        method: "POST",
        rust_path: "/jaxrs/program_center/input/prepare/cover",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "program_center",
        method: "POST",
        rust_path: "/jaxrs/program_center/input/prepare/create",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "program_center",
        method: "POST",
        rust_path: "/jaxrs/program_center/market/flag/install/or/update",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "program_center",
        method: "POST",
        rust_path: "/jaxrs/program_center/module/compare/upload",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "program_center",
        method: "POST",
        rust_path: "/jaxrs/program_center/module/output",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "program_center",
        method: "POST",
        rust_path: "/jaxrs/program_center/module/output/flag/file",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "program_center",
        method: "POST",
        rust_path: "/jaxrs/program_center/module/output/structure",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "program_center",
        method: "POST",
        rust_path: "/jaxrs/program_center/mpweixin/media/add/forever",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "program_center",
        method: "POST",
        rust_path: "/jaxrs/program_center/mpweixin/menu/add",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "program_center",
        method: "POST",
        rust_path: "/jaxrs/program_center/mpweixin/menu/create/to/weixin",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "program_center",
        method: "POST",
        rust_path: "/jaxrs/program_center/mpweixin/menu/update/id",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "program_center",
        method: "POST",
        rust_path: "/jaxrs/program_center/output/appInfoFlag/select",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "program_center",
        method: "POST",
        rust_path: "/jaxrs/program_center/output/flag/select/file",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "program_center",
        method: "POST",
        rust_path: "/jaxrs/program_center/output/list",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "program_center",
        method: "POST",
        rust_path: "/jaxrs/program_center/tokenthreshold/update",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "program_init",
        method: "GET",
        rust_path: "/jaxrs/secret/check",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "program_init",
        method: "GET",
        rust_path: "/jaxrs/secret/set/cancel",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "program_init",
        method: "POST",
        rust_path: "/jaxrs/secret/set",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "query_assemble_designer",
        method: "GET",
        rust_path: "/jaxrs/query/assemble/designer/bundle/{view}/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "query_assemble_designer",
        method: "GET",
        rust_path: "/jaxrs/query/assemble/designer/entity/entity/properties/{query}/{category}/{entityCategory}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "query_assemble_designer",
        method: "GET",
        rust_path: "/jaxrs/query/assemble/designer/get/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "query_assemble_designer",
        method: "GET",
        rust_path: "/jaxrs/query/assemble/designer/icon/{query}/{flag}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "query_assemble_designer",
        method: "GET",
        rust_path: "/jaxrs/query/assemble/designer/list/querycategory/{query}/{queryCategory}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "query_assemble_designer",
        method: "GET",
        rust_path: "/jaxrs/query/assemble/designer/list/summary/querycategory/{query}/{queryCategory}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "query_assemble_designer",
        method: "GET",
        rust_path: "/jaxrs/query/assemble/designer/list/{category}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "query_assemble_designer",
        method: "GET",
        rust_path: "/jaxrs/query/assemble/designer/list/{view}/{id}/{next}/{count}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "query_assemble_designer",
        method: "GET",
        rust_path: "/jaxrs/query/assemble/designer/list/{view}/{query}/{flag}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "query_assemble_designer",
        method: "GET",
        rust_path: "/jaxrs/query/assemble/designer/neural/generate/model/{modelFlag}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "query_assemble_designer",
        method: "GET",
        rust_path: "/jaxrs/query/assemble/designer/neural/learn/model/{modelFlag}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "query_assemble_designer",
        method: "GET",
        rust_path: "/jaxrs/query/assemble/designer/neural/model/{modelFlag}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "query_assemble_designer",
        method: "GET",
        rust_path: "/jaxrs/query/assemble/designer/neural/stop/generating/model/{modelFlag}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "query_assemble_designer",
        method: "GET",
        rust_path: "/jaxrs/query/assemble/designer/neural/stop/learn/model/{modelFlag}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "query_assemble_designer",
        method: "GET",
        rust_path: "/jaxrs/query/assemble/designer/output/select/file/{flag}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "query_assemble_designer",
        method: "GET",
        rust_path: "/jaxrs/query/assemble/designer/output/select/{queryFlag}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "query_assemble_designer",
        method: "GET",
        rust_path: "/jaxrs/query/assemble/designer/permission/{query}/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "query_assemble_designer",
        method: "GET",
        rust_path: "/jaxrs/query/assemble/designer/simulate/{view}/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "query_assemble_designer",
        method: "GET",
        rust_path: "/jaxrs/query/assemble/designer/stat/list/{id}/{next}/{count}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "query_assemble_designer",
        method: "GET",
        rust_path: "/jaxrs/query/assemble/designer/stat/list/{query}/{flag}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "query_assemble_designer",
        method: "GET",
        rust_path: "/jaxrs/query/assemble/designer/stat/permission/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "query_assemble_designer",
        method: "GET",
        rust_path: "/jaxrs/query/assemble/designer/stat/simulate/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "query_assemble_designer",
        method: "GET",
        rust_path: "/jaxrs/query/assemble/designer/stat/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "query_assemble_designer",
        method: "GET",
        rust_path: "/jaxrs/query/assemble/designer/table/build/dispatch/{query}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "query_assemble_designer",
        method: "GET",
        rust_path: "/jaxrs/query/assemble/designer/table/build/{flag}/{status}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "query_assemble_designer",
        method: "GET",
        rust_path: "/jaxrs/query/assemble/designer/table/draft/{flag}/{status}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "query_assemble_designer",
        method: "GET",
        rust_path: "/jaxrs/query/assemble/designer/table/export/{tableFlag}/{count}/{count}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "query_assemble_designer",
        method: "GET",
        rust_path: "/jaxrs/query/assemble/designer/table/list/row/select/where/where/{tableFlag}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "query_assemble_designer",
        method: "GET",
        rust_path: "/jaxrs/query/assemble/designer/table/list/row/{tableFlag}/{id}/{next}/{count}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "query_assemble_designer",
        method: "GET",
        rust_path: "/jaxrs/query/assemble/designer/table/list/{query}/{flag}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "query_assemble_designer",
        method: "GET",
        rust_path: "/jaxrs/query/assemble/designer/table/permission/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "query_assemble_designer",
        method: "GET",
        rust_path: "/jaxrs/query/assemble/designer/table/row/where/where/{tableFlag}/{count}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "query_assemble_designer",
        method: "GET",
        rust_path: "/jaxrs/query/assemble/designer/table/row/{tableFlag}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "query_assemble_designer",
        method: "GET",
        rust_path: "/jaxrs/query/assemble/designer/table/row/{tableFlag}/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "query_assemble_designer",
        method: "GET",
        rust_path: "/jaxrs/query/assemble/designer/table/{flag}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "query_assemble_designer",
        method: "GET",
        rust_path: "/jaxrs/query/assemble/designer/{id}/{count}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "query_assemble_designer",
        method: "POST",
        rust_path: "/jaxrs/query/assemble/designer/create",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "query_assemble_designer",
        method: "POST",
        rust_path: "/jaxrs/query/assemble/designer/delete/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "query_assemble_designer",
        method: "POST",
        rust_path: "/jaxrs/query/assemble/designer/importmodel/list/{query}/{flag}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "query_assemble_designer",
        method: "POST",
        rust_path: "/jaxrs/query/assemble/designer/importmodel/permission/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "query_assemble_designer",
        method: "POST",
        rust_path: "/jaxrs/query/assemble/designer/importmodel/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "query_assemble_designer",
        method: "POST",
        rust_path: "/jaxrs/query/assemble/designer/neural/model/reset/{modelFlag}/{status}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "query_assemble_designer",
        method: "POST",
        rust_path: "/jaxrs/query/assemble/designer/save/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "query_assemble_designer",
        method: "POST",
        rust_path: "/jaxrs/query/assemble/designer/table/execute/{flag}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "query_assemble_designer",
        method: "POST",
        rust_path: "/jaxrs/query/assemble/designer/table/row/delete/all/{tableFlag}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "query_assemble_designer",
        method: "POST",
        rust_path: "/jaxrs/query/assemble/designer/table/row/save/{tableFlag}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "query_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/query/assemble/surface/get/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "query_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/query/assemble/surface/list/{category}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "query_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/query/assemble/surface/preview/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "query_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/queryview/bundle/v2/{view}/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "query_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/queryview/bundle/{view}/{flag}/{flag}/{query}/{queryFlag}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "query_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/queryview/bundle/{view}/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "query_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/queryview/excel/result/{view}/{flag}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "query_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/queryview/excel/{view}/{flag}/{flag}/{query}/{queryFlag}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "query_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/queryview/excel/{view}/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "query_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/queryview/execute/v2/{view}/{id}/{page}/{page}/{size}/{size}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "query_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/queryview/execute/{view}/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "query_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/queryview/flag/{view}/application/flag/{app}/execute",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "query_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/queryview/flag/{view}/application/flag/{app}/execute/page/{page}/size/{size}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "query_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/queryview/list/{query}/{key}/{key}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "query_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/queryview/neural/list/calculate/model/{modelFlag}/{work}/{workId}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "query_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/queryview/table/list/paging/{page}/{size}/{size}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "query_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/queryview/table/list/row/select/where/where/{tableFlag}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "query_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/queryview/table/list/row/select/{tableFlag}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "query_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/queryview/table/list/row/{tableFlag}/{id}/{next}/{count}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "query_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/queryview/table/list/table/row/paging/{tableFlag}/{page}/{size}/{size}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "query_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/queryview/table/list/{id}/{next}/{count}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "query_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/queryview/table/row/one/{tableFlag}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "query_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/queryview/table/row/where/where/{tableFlag}/{count}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "query_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/queryview/table/row/{tableFlag}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "query_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/queryview/table/row/{tableFlag}/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "query_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/queryview/table/{flag}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "query_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/queryview/{query}/{flag}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "query_assemble_surface",
        method: "GET",
        rust_path: "/jaxrs/queryview/{view}/{flag}/{flag}/{query}/{queryFlag}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "query_assemble_surface",
        method: "POST",
        rust_path: "/jaxrs/importmodel/id/{id}/execute",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "query_assemble_surface",
        method: "POST",
        rust_path: "/jaxrs/query/assemble/surface/create",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "query_assemble_surface",
        method: "POST",
        rust_path: "/jaxrs/query/assemble/surface/delete/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "query_assemble_surface",
        method: "POST",
        rust_path: "/jaxrs/query/assemble/surface/save/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "query_assemble_surface",
        method: "POST",
        rust_path: "/jaxrs/queryview/bundle/mockputtopost/{view}/{flag}/{flag}/{query}/{queryFlag}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "query_assemble_surface",
        method: "POST",
        rust_path: "/jaxrs/queryview/bundle/mockputtopost/{view}/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "query_assemble_surface",
        method: "POST",
        rust_path: "/jaxrs/queryview/excel/mockputtopost/{view}/{flag}/{flag}/{query}/{queryFlag}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "query_assemble_surface",
        method: "POST",
        rust_path: "/jaxrs/queryview/excel/mockputtopost/{view}/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "query_assemble_surface",
        method: "POST",
        rust_path: "/jaxrs/queryview/execute/mockputtopost/{view}/{flag}/{flag}/{query}/{queryFlag}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "query_assemble_surface",
        method: "POST",
        rust_path: "/jaxrs/queryview/execute/mockputtopost/{view}/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "query_assemble_surface",
        method: "POST",
        rust_path: "/jaxrs/queryview/importmodel/execute/record/{recordId}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "query_assemble_surface",
        method: "POST",
        rust_path: "/jaxrs/queryview/importmodel/list/record/item/paging/{page}/{size}/{size}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "query_assemble_surface",
        method: "POST",
        rust_path: "/jaxrs/queryview/importmodel/list/record/paging/{page}/{size}/{size}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "query_assemble_surface",
        method: "POST",
        rust_path: "/jaxrs/queryview/importmodel/list/{query}/{queryFlag}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "query_assemble_surface",
        method: "POST",
        rust_path: "/jaxrs/queryview/importmodel/record/mockdeletetoget/{recordId}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "query_assemble_surface",
        method: "POST",
        rust_path: "/jaxrs/queryview/importmodel/record/{recordId}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "query_assemble_surface",
        method: "POST",
        rust_path: "/jaxrs/queryview/importmodel/record/{recordId}/{status}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "query_assemble_surface",
        method: "POST",
        rust_path: "/jaxrs/queryview/importmodel/{flag}/{flag}/{query}/{queryFlag}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "query_assemble_surface",
        method: "POST",
        rust_path: "/jaxrs/queryview/importmodel/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "query_assemble_surface",
        method: "POST",
        rust_path: "/jaxrs/queryview/table/row/delete/all/mockdeletetoget/{tableFlag}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "query_assemble_surface",
        method: "POST",
        rust_path: "/jaxrs/queryview/table/row/delete/all/{tableFlag}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "query_assemble_surface",
        method: "POST",
        rust_path: "/jaxrs/queryview/table/row/mockdeletetoget/{tableFlag}/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "query_assemble_surface",
        method: "POST",
        rust_path: "/jaxrs/queryview/table/row/mockputtopost/{tableFlag}/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "query_assemble_surface",
        method: "POST",
        rust_path: "/jaxrs/queryview/table/row/part/update/{tableFlag}/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "query_core_entity",
        method: "GET",
        rust_path: "/jaxrs/query/import/list",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "query_core_entity",
        method: "GET",
        rust_path: "/jaxrs/query/item/list",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "query_core_entity",
        method: "GET",
        rust_path: "/jaxrs/query/view/list",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "query_core_entity",
        method: "GET",
        rust_path: "/jaxrs/query/view/{id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "query_core_entity",
        method: "POST",
        rust_path: "/jaxrs/query/view/create",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "query_core_express",
        method: "GET",
        rust_path: "/jaxrs/query/core/express/cache/status/{queryId}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "query_core_express",
        method: "GET",
        rust_path: "/jaxrs/query/core/express/history/{limit}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "query_core_express",
        method: "POST",
        rust_path: "/jaxrs/query/core/express/cache/{queryId}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "query_core_express",
        method: "POST",
        rust_path: "/jaxrs/query/core/express/execute",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "query_express",
        method: "GET",
        rust_path: "/jaxrs/query/list",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "query_express",
        method: "POST",
        rust_path: "/jaxrs/query/create",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "query_service_processing",
        method: "GET",
        rust_path: "/jaxrs/query/service/processing/status",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "query_service_processing",
        method: "POST",
        rust_path: "/jaxrs/query/service/processing/batch",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "query_service_processing",
        method: "POST",
        rust_path: "/jaxrs/query/service/processing/process",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "query_service_processing",
        method: "POST",
        rust_path: "/jaxrs/query/service/processing/reset",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "realtime",
        method: "GET",
        rust_path: "/ws/realtime",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "realtime",
        method: "GET",
        rust_path: "/ws/realtime/room/{room_id}",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

    EndpointDef {
        crate_name: "realtime",
        method: "GET",
        rust_path: "/ws/realtime/room/{room_id}/stats",
        java_war: "",
        java_action: "",
        body: None,
        requires_auth: false,
    },

];
