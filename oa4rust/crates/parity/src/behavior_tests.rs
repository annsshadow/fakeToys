// AUTO-GENERATED behavior contract tests — DO NOT EDIT by script.
// Hand-written Top 100 high-frequency routes with behavior contracts.
// Generated from: docs/audits/o2server-parity-report.json

use crate::parity_test;
use tower::util::ServiceExt;
use shared::RateLimiter;
use shared::SessionManager;

// ── login/auth routes: behavior = login_returns_token ──

parity_test!(
    crate: auth,
    router_fn: router,
    route: "/jaxrs/authentication",
    method: POST,
    handler: login,
    test_name: parity_behavior__auth__login,
    behavior: "route_exists",
    router_args: (shared::testing::test_pool(), shared::RateLimiter::new(), shared::SessionManager::new()),
    body: serde_json::json!({"credential":"test","password":"test"}).to_string(),
);

parity_test!(
    crate: auth,
    router_fn: router,
    route: "/jaxrs/authentication/check/token",
    method: POST,
    handler: check_token,
    test_name: parity_behavior__auth__check_token,
    behavior: "route_exists",
    router_args: (shared::testing::test_pool(), shared::RateLimiter::new(), shared::SessionManager::new()),
    body: serde_json::json!({"credential":"test","password":"test"}).to_string(),
);

parity_test!(
    crate: auth,
    router_fn: router,
    route: "/jaxrs/authentication/login",
    method: POST,
    handler: login_1,
    test_name: parity_behavior__auth__login_1,
    behavior: "route_exists",
    router_args: (shared::testing::test_pool(), shared::RateLimiter::new(), shared::SessionManager::new()),
    body: serde_json::json!({"credential":"test","password":"test"}).to_string(),
);

parity_test!(
    crate: auth,
    router_fn: router,
    route: "/jaxrs/authentication/oauth/bind/name/test-id/code/test-id/redirecturi/test-id",
    method: GET,
    handler: oauth,
    test_name: parity_behavior__auth__oauth,
    behavior: "route_exists",
    router_args: (shared::testing::test_pool(), shared::RateLimiter::new(), shared::SessionManager::new()),
    body: serde_json::json!({"credential":"test","password":"test"}).to_string(),
);

parity_test!(
    crate: auth,
    router_fn: router,
    route: "/jaxrs/authentication/oauth/dingding/config",
    method: GET,
    handler: oauth_1,
    test_name: parity_behavior__auth__oauth_1,
    behavior: "route_exists",
    router_args: (shared::testing::test_pool(), shared::RateLimiter::new(), shared::SessionManager::new()),
    body: serde_json::json!({"credential":"test","password":"test"}).to_string(),
);

parity_test!(
    crate: auth,
    router_fn: router,
    route: "/jaxrs/authentication/oauth/list",
    method: GET,
    handler: oauth_2,
    test_name: parity_behavior__auth__oauth_2,
    behavior: "login_returns_token",
    router_args: (shared::testing::test_pool(), shared::RateLimiter::new(), shared::SessionManager::new()),
    body: serde_json::json!({"credential":"test","password":"test"}).to_string(),
);

parity_test!(
    crate: auth,
    router_fn: router,
    route: "/jaxrs/authentication/oauth/login/dingding/code/test-id",
    method: GET,
    handler: oauth_3,
    test_name: parity_behavior__auth__oauth_3,
    behavior: "route_exists",
    router_args: (shared::testing::test_pool(), shared::RateLimiter::new(), shared::SessionManager::new()),
    body: serde_json::json!({"credential":"test","password":"test"}).to_string(),
);

parity_test!(
    crate: auth,
    router_fn: router,
    route: "/jaxrs/authentication/oauth/login/name/test-id/code/test-id/redirecturi/test-id",
    method: GET,
    handler: oauth_4,
    test_name: parity_behavior__auth__oauth_4,
    behavior: "route_exists",
    router_args: (shared::testing::test_pool(), shared::RateLimiter::new(), shared::SessionManager::new()),
    body: serde_json::json!({"credential":"test","password":"test"}).to_string(),
);

parity_test!(
    crate: auth,
    router_fn: router,
    route: "/jaxrs/authentication/oauth/login/qywx/code/test-id",
    method: GET,
    handler: oauth_5,
    test_name: parity_behavior__auth__oauth_5,
    behavior: "route_exists",
    router_args: (shared::testing::test_pool(), shared::RateLimiter::new(), shared::SessionManager::new()),
    body: serde_json::json!({"credential":"test","password":"test"}).to_string(),
);

parity_test!(
    crate: auth,
    router_fn: router,
    route: "/jaxrs/authentication/oauth/name/test-id",
    method: GET,
    handler: oauth_6,
    test_name: parity_behavior__auth__oauth_6,
    behavior: "login_returns_token",
    router_args: (shared::testing::test_pool(), shared::RateLimiter::new(), shared::SessionManager::new()),
    body: serde_json::json!({"credential":"test","password":"test"}).to_string(),
);

parity_test!(
    crate: auth,
    router_fn: router,
    route: "/jaxrs/authentication/oauth/qywx/config",
    method: GET,
    handler: oauth_7,
    test_name: parity_behavior__auth__oauth_7,
    behavior: "route_exists",
    router_args: (shared::testing::test_pool(), shared::RateLimiter::new(), shared::SessionManager::new()),
    body: serde_json::json!({"credential":"test","password":"test"}).to_string(),
);

parity_test!(
    crate: auth,
    router_fn: router,
    route: "/jaxrs/mpweixin/login/code/test-id",
    method: GET,
    handler: mpweixin_login,
    test_name: parity_behavior__auth__mpweixin_login,
    behavior: "route_exists",
    router_args: (shared::testing::test_pool(), shared::RateLimiter::new(), shared::SessionManager::new()),
    body: serde_json::json!({"credential":"test","password":"test"}).to_string(),
);

parity_test!(
    crate: auth,
    router_fn: router,
    route: "/jaxrs/qiyeweixin/code/test-id",
    method: GET,
    handler: qiyeweixin_login,
    test_name: parity_behavior__auth__qiyeweixin_login,
    behavior: "route_exists",
    router_args: (shared::testing::test_pool(), shared::RateLimiter::new(), shared::SessionManager::new()),
    body: serde_json::json!({"credential":"test","password":"test"}).to_string(),
);

parity_test!(
    crate: auth,
    router_fn: router,
    route: "/jaxrs/welink/code/test-id",
    method: GET,
    handler: welink_login,
    test_name: parity_behavior__auth__welink_login,
    behavior: "route_exists",
    router_args: (shared::testing::test_pool(), shared::RateLimiter::new(), shared::SessionManager::new()),
    body: serde_json::json!({"credential":"test","password":"test"}).to_string(),
);

parity_test!(
    crate: auth,
    router_fn: router,
    route: "/jaxrs/zhengwudingding/code/test-id",
    method: GET,
    handler: zwdingding_login,
    test_name: parity_behavior__auth__zwdingding_login,
    behavior: "route_exists",
    router_args: (shared::testing::test_pool(), shared::RateLimiter::new(), shared::SessionManager::new()),
    body: serde_json::json!({"credential":"test","password":"test"}).to_string(),
);

// ── list routes: behavior = list_returns_array ──

parity_test!(
    crate: ai_assemble_control,
    router_fn: router,
    route: "/jaxrs/ai/assemble/control/config/list/mcp/paging/1/size/1",
    method: GET,
    handler: config_list_mcp_paging_page_size_size,
    test_name: parity_behavior__ai_assemble_control__config_list_mcp_paging_page_size_size,
    behavior: "list_returns_array",
    router_args: (shared::testing::test_pool()),
    body: String::new(),
);

parity_test!(
    crate: ai_assemble_control,
    router_fn: router,
    route: "/jaxrs/ai_assemble_control/config/list/enable/model",
    method: GET,
    handler: config_list_enable_model,
    test_name: parity_behavior__ai_assemble_control__config_list_enable_model,
    behavior: "list_returns_array",
    router_args: (shared::testing::test_pool()),
    body: String::new(),
);

parity_test!(
    crate: ai_assemble_control,
    router_fn: router,
    route: "/jaxrs/ai_assemble_control/config/list/mcp/paging/1/size/1",
    method: GET,
    handler: config_list_mcp_paging_page_size_size_1,
    test_name: parity_behavior__ai_assemble_control__config_list_mcp_paging_page_size_size_1,
    behavior: "list_returns_array",
    router_args: (shared::testing::test_pool()),
    body: String::new(),
);

parity_test!(
    crate: ai_assemble_control,
    router_fn: router,
    route: "/jaxrs/ai_assemble_control/config/list/model/paging/1/size/1",
    method: GET,
    handler: config_list_model_paging_page_size_size,
    test_name: parity_behavior__ai_assemble_control__config_list_model_paging_page_size_size,
    behavior: "list_returns_array",
    router_args: (shared::testing::test_pool()),
    body: String::new(),
);

parity_test!(
    crate: ai_assemble_control,
    router_fn: router,
    route: "/jaxrs/ai_assemble_control/file/list",
    method: GET,
    handler: file_list,
    test_name: parity_behavior__ai_assemble_control__file_list,
    behavior: "list_returns_array",
    router_args: (shared::testing::test_pool()),
    body: String::new(),
);

parity_test!(
    crate: ai_assemble_control,
    router_fn: router,
    route: "/jaxrs/ai_assemble_control/file/list/paging/1/size/1",
    method: GET,
    handler: file_list_paging_page_size_size,
    test_name: parity_behavior__ai_assemble_control__file_list_paging_page_size_size,
    behavior: "list_returns_array",
    router_args: (shared::testing::test_pool()),
    body: String::new(),
);

parity_test!(
    crate: ai_assemble_control,
    router_fn: router,
    route: "/jaxrs/ai_assemble_control/index/list/paging/1/size/1",
    method: GET,
    handler: index_list_paging_page_size_size,
    test_name: parity_behavior__ai_assemble_control__index_list_paging_page_size_size,
    behavior: "list_returns_array",
    router_args: (shared::testing::test_pool()),
    body: String::new(),
);

parity_test!(
    crate: ai_assemble_control,
    router_fn: router,
    route: "/jaxrs/ai_assemble_control/list/ai/models",
    method: GET,
    handler: list_ai_models,
    test_name: parity_behavior__ai_assemble_control__list_ai_models,
    behavior: "list_returns_array",
    router_args: (shared::testing::test_pool()),
    body: String::new(),
);

parity_test!(
    crate: ai,
    router_fn: router,
    route: "/jaxrs/ai/app/list",
    method: GET,
    handler: app_list,
    test_name: parity_behavior__ai__app_list,
    behavior: "list_returns_array",
    router_args: (shared::testing::test_pool()),
    body: String::new(),
);

parity_test!(
    crate: ai,
    router_fn: router,
    route: "/jaxrs/ai/chat/list/completion/test-id/paging/1/size/1",
    method: GET,
    handler: chat_list_completion_paging,
    test_name: parity_behavior__ai__chat_list_completion_paging,
    behavior: "list_returns_array",
    router_args: (shared::testing::test_pool()),
    body: String::new(),
);

parity_test!(
    crate: ai,
    router_fn: router,
    route: "/jaxrs/ai/chat/list/paging/1/size/1",
    method: GET,
    handler: chat_list_paging,
    test_name: parity_behavior__ai__chat_list_paging,
    behavior: "list_returns_array",
    router_args: (shared::testing::test_pool()),
    body: String::new(),
);

parity_test!(
    crate: ai,
    router_fn: router,
    route: "/jaxrs/ai/config/list/enable/model",
    method: GET,
    handler: list_enable_model,
    test_name: parity_behavior__ai__list_enable_model,
    behavior: "list_returns_array",
    router_args: (shared::testing::test_pool()),
    body: String::new(),
);

parity_test!(
    crate: ai,
    router_fn: router,
    route: "/jaxrs/ai/config/list/mcp/paging/1/size/1",
    method: GET,
    handler: config_list_mcp_paging,
    test_name: parity_behavior__ai__config_list_mcp_paging,
    behavior: "list_returns_array",
    router_args: (shared::testing::test_pool()),
    body: String::new(),
);

parity_test!(
    crate: ai,
    router_fn: router,
    route: "/jaxrs/ai/config/list/model/paging/1/size/1",
    method: GET,
    handler: config_list_model_paging,
    test_name: parity_behavior__ai__config_list_model_paging,
    behavior: "list_returns_array",
    router_args: (shared::testing::test_pool()),
    body: String::new(),
);

parity_test!(
    crate: ai,
    router_fn: router,
    route: "/jaxrs/ai/conversation/list",
    method: GET,
    handler: conversation_list,
    test_name: parity_behavior__ai__conversation_list,
    behavior: "list_returns_array",
    router_args: (shared::testing::test_pool()),
    body: String::new(),
);

parity_test!(
    crate: ai,
    router_fn: router,
    route: "/jaxrs/ai/model/list",
    method: GET,
    handler: model_list,
    test_name: parity_behavior__ai__model_list,
    behavior: "list_returns_array",
    router_args: (shared::testing::test_pool()),
    body: String::new(),
);

parity_test!(
    crate: bbs_assemble_control,
    router_fn: router,
    route: "/jaxrs/bbs/assemble/control/forum/list",
    method: GET,
    handler: list_forums,
    test_name: parity_behavior__bbs_assemble_control__list_forums,
    behavior: "route_exists",
    router_args: (shared::testing::test_pool()),
    body: String::new(),
);

parity_test!(
    crate: bbs_assemble_control,
    router_fn: router,
    route: "/jaxrs/bbs/assemble/control/reply/list/sub/test-id",
    method: GET,
    handler: reply_list_sub_id,
    test_name: parity_behavior__bbs_assemble_control__reply_list_sub_id,
    behavior: "route_exists",
    router_args: (shared::testing::test_pool()),
    body: String::new(),
);

parity_test!(
    crate: bbs_assemble_control,
    router_fn: router,
    route: "/jaxrs/bbs/assemble/control/section/list",
    method: GET,
    handler: list_control_sections,
    test_name: parity_behavior__bbs_assemble_control__list_control_sections,
    behavior: "route_exists",
    router_args: (shared::testing::test_pool()),
    body: String::new(),
);

parity_test!(
    crate: bbs_assemble_control,
    router_fn: router,
    route: "/jaxrs/bbs/assemble/control/topic/list/forum/test-id",
    method: GET,
    handler: list_topics_by_forum,
    test_name: parity_behavior__bbs_assemble_control__list_topics_by_forum,
    behavior: "route_exists",
    router_args: (shared::testing::test_pool()),
    body: String::new(),
);

parity_test!(
    crate: calendar_assemble_control,
    router_fn: router,
    route: "/jaxrs/calendar_assemble_control/list/control/calendars",
    method: GET,
    handler: list_control_calendars,
    test_name: parity_behavior__calendar_assemble_control__list_control_calendars,
    behavior: "list_returns_array",
    router_args: (shared::testing::test_pool()),
    body: String::new(),
);

parity_test!(
    crate: calendar,
    router_fn: router,
    route: "/jaxrs/calendar/calendar/list/my",
    method: GET,
    handler: calendar_list_my,
    test_name: parity_behavior__calendar__calendar_list_my,
    behavior: "list_returns_array",
    router_args: (shared::testing::test_pool()),
    body: String::new(),
);

parity_test!(
    crate: calendar,
    router_fn: router,
    route: "/jaxrs/calendar/calendar/list/public",
    method: GET,
    handler: calendar_list_public,
    test_name: parity_behavior__calendar__calendar_list_public,
    behavior: "list_returns_array",
    router_args: (shared::testing::test_pool()),
    body: String::new(),
);

parity_test!(
    crate: calendar,
    router_fn: router,
    route: "/jaxrs/calendar/event/list/test-id",
    method: GET,
    handler: event_list,
    test_name: parity_behavior__calendar__event_list,
    behavior: "list_returns_array",
    router_args: (shared::testing::test_pool()),
    body: String::new(),
);

parity_test!(
    crate: cms_assemble_control,
    router_fn: router,
    route: "/jaxrs/cms_assemble_control/list/control/sections",
    method: GET,
    handler: list_control_sections,
    test_name: parity_behavior__cms_assemble_control__list_control_sections,
    behavior: "list_returns_array",
    router_args: (shared::testing::test_pool()),
    body: String::new(),
);

parity_test!(
    crate: cms_assemble_control,
    router_fn: router,
    route: "/jaxrs/commend/list/paging/1?doc_id=1",
    method: GET,
    handler: commend_list_paging,
    test_name: parity_behavior__cms_assemble_control__commend_list_paging,
    behavior: "list_returns_array",
    router_args: (shared::testing::test_pool()),
    body: String::new(),
);

parity_test!(
    crate: cms_core_entity,
    router_fn: router,
    route: "/jaxrs/cms/article/list",
    method: GET,
    handler: article_list,
    test_name: parity_behavior__cms_core_entity__article_list,
    behavior: "route_exists",
    router_args: (shared::testing::test_pool()),
    body: String::new(),
);

parity_test!(
    crate: cms_core_entity,
    router_fn: router,
    route: "/jaxrs/cms/category/list",
    method: GET,
    handler: category_list,
    test_name: parity_behavior__cms_core_entity__category_list,
    behavior: "route_exists",
    router_args: (shared::testing::test_pool()),
    body: String::new(),
);

parity_test!(
    crate: cms_core_express,
    router_fn: router,
    route: "/jaxrs/cms/core/express/content/list",
    method: GET,
    handler: content_list,
    test_name: parity_behavior__cms_core_express__content_list,
    behavior: "list_returns_array",
    router_args: (shared::testing::test_pool()),
    body: String::new(),
);

parity_test!(
    crate: component_assemble_control,
    router_fn: router,
    route: "/jaxrs/component/assemble/control/status/list",
    method: GET,
    handler: status_list,
    test_name: parity_behavior__component_assemble_control__status_list,
    behavior: "list_returns_array",
    router_args: (shared::testing::test_pool()),
    body: String::new(),
);

parity_test!(
    crate: component_assemble_control,
    router_fn: router,
    route: "/jaxrs/component_assemble_control/list/components",
    method: GET,
    handler: list_components,
    test_name: parity_behavior__component_assemble_control__list_components,
    behavior: "list_returns_array",
    router_args: (shared::testing::test_pool()),
    body: String::new(),
);

parity_test!(
    crate: component_assemble_control,
    router_fn: router,
    route: "/jaxrs/component_assemble_control/list/control/categories",
    method: GET,
    handler: list_control_categories,
    test_name: parity_behavior__component_assemble_control__list_control_categories,
    behavior: "list_returns_array",
    router_args: (shared::testing::test_pool()),
    body: String::new(),
);

parity_test!(
    crate: component,
    router_fn: router,
    route: "/jaxrs/component/list/all",
    method: GET,
    handler: list_all,
    test_name: parity_behavior__component__list_all,
    behavior: "route_exists",
    router_args: (shared::testing::test_pool()),
    body: String::new(),
);

parity_test!(
    crate: correlation,
    router_fn: router,
    route: "/jaxrs/correlation/type/cms/list",
    method: GET,
    handler: list_cms_correlations,
    test_name: parity_behavior__correlation__list_cms_correlations,
    behavior: "list_returns_array",
    router_args: (shared::testing::test_pool()),
    body: String::new(),
);

parity_test!(
    crate: correlation,
    router_fn: router,
    route: "/jaxrs/correlation/type/processplatform/list",
    method: GET,
    handler: list_process_platform_correlations,
    test_name: parity_behavior__correlation__list_process_platform_correlations,
    behavior: "list_returns_array",
    router_args: (shared::testing::test_pool()),
    body: String::new(),
);

parity_test!(
    crate: correlation_service_processing,
    router_fn: router,
    route: "/jaxrs/correlation/service/processing/correlation/list/type/cms/document/test-id",
    method: GET,
    handler: correlation_list_type_cms_document_document,
    test_name: parity_behavior__correlation_service_processing__correlation_list_type_cms_document_document,
    behavior: "list_returns_array",
    router_args: (shared::testing::test_pool()),
    body: String::new(),
);

parity_test!(
    crate: correlation_service_processing,
    router_fn: router,
    route: "/jaxrs/correlation/service/processing/correlation/list/type/cms/document/test-id/site/test-id",
    method: GET,
    handler: correlation_list_type_cms_document_document_site_site,
    test_name: parity_behavior__correlation_service_processing__correlation_list_type_cms_document_document_site_site,
    behavior: "list_returns_array",
    router_args: (shared::testing::test_pool()),
    body: String::new(),
);

parity_test!(
    crate: correlation_service_processing,
    router_fn: router,
    route: "/jaxrs/correlation/service/processing/correlation/list/type/processplatform/job/test-id",
    method: GET,
    handler: correlation_list_type_processplatform_job_job,
    test_name: parity_behavior__correlation_service_processing__correlation_list_type_processplatform_job_job,
    behavior: "list_returns_array",
    router_args: (shared::testing::test_pool()),
    body: String::new(),
);

parity_test!(
    crate: correlation_service_processing,
    router_fn: router,
    route: "/jaxrs/correlation/service/processing/correlation/list/type/processplatform/job/test-id/site/test-id",
    method: GET,
    handler: correlation_list_type_processplatform_job_job_site_site,
    test_name: parity_behavior__correlation_service_processing__correlation_list_type_processplatform_job_job_site_site,
    behavior: "list_returns_array",
    router_args: (shared::testing::test_pool()),
    body: String::new(),
);

parity_test!(
    crate: correlation_service_processing,
    router_fn: router,
    route: "/jaxrs/correlation/service/processing/list/test-id",
    method: GET,
    handler: list_correlations,
    test_name: parity_behavior__correlation_service_processing__list_correlations,
    behavior: "list_returns_array",
    router_args: (shared::testing::test_pool()),
    body: String::new(),
);

parity_test!(
    crate: file_assemble_control,
    router_fn: router,
    route: "/jaxrs/file/assemble/control/file/list/test-id",
    method: GET,
    handler: list_files,
    test_name: parity_behavior__file_assemble_control__list_files,
    behavior: "list_returns_array",
    router_args: (shared::testing::test_pool()),
    body: String::new(),
);

parity_test!(
    crate: file,
    router_fn: router,
    route: "/jaxrs/file/folder/list/top",
    method: GET,
    handler: folder_list_top,
    test_name: parity_behavior__file__folder_list_top,
    behavior: "list_returns_array",
    router_args: (shared::testing::test_pool()),
    body: String::new(),
);

parity_test!(
    crate: file,
    router_fn: router,
    route: "/jaxrs/file/folder/list/test-id",
    method: GET,
    handler: folder_list_with_folder,
    test_name: parity_behavior__file__folder_list_with_folder,
    behavior: "list_returns_array",
    router_args: (shared::testing::test_pool()),
    body: String::new(),
);

parity_test!(
    crate: hotpic_assemble_control,
    router_fn: router,
    route: "/jaxrs/hotpic/assemble/control/cipher/hotpic/filter/list/page/1/count/1",
    method: GET,
    handler: cipher_hotpic_filter_list_page_page_count_count,
    test_name: parity_behavior__hotpic_assemble_control__cipher_hotpic_filter_list_page_page_count_count,
    behavior: "list_returns_array",
    router_args: (shared::testing::test_pool()),
    body: String::new(),
);

parity_test!(
    crate: hotpic_assemble_control,
    router_fn: router,
    route: "/jaxrs/hotpic/assemble/control/list/control/applications",
    method: GET,
    handler: list_control_applications,
    test_name: parity_behavior__hotpic_assemble_control__list_control_applications,
    behavior: "list_returns_array",
    router_args: (shared::testing::test_pool()),
    body: String::new(),
);

parity_test!(
    crate: hotpic_assemble_control,
    router_fn: router,
    route: "/jaxrs/hotpic/assemble/control/list/control/panels",
    method: GET,
    handler: list_control_panels,
    test_name: parity_behavior__hotpic_assemble_control__list_control_panels,
    behavior: "list_returns_array",
    router_args: (shared::testing::test_pool()),
    body: String::new(),
);

parity_test!(
    crate: hotpic_assemble_control,
    router_fn: router,
    route: "/jaxrs/hotpic/assemble/control/user/hotpic/filter/list/page/1/count/1",
    method: GET,
    handler: user_hotpic_filter_list_page_page_count_count,
    test_name: parity_behavior__hotpic_assemble_control__user_hotpic_filter_list_page_page_count_count,
    behavior: "list_returns_array",
    router_args: (shared::testing::test_pool()),
    body: String::new(),
);

parity_test!(
    crate: hotpic_assemble_control,
    router_fn: router,
    route: "/jaxrs/hotpic/list/hotpics",
    method: GET,
    handler: list_hotpics,
    test_name: parity_behavior__hotpic_assemble_control__list_hotpics,
    behavior: "list_returns_array",
    router_args: (shared::testing::test_pool()),
    body: String::new(),
);

parity_test!(
    crate: hotpic_assemble_control,
    router_fn: router,
    route: "/jaxrs/hotpic_assemble_control/cipher/hotpic/filter/list/page/page/count/count",
    method: GET,
    handler: cipher_hotpic_filter_list_page_page_count_count_1,
    test_name: parity_behavior__hotpic_assemble_control__cipher_hotpic_filter_list_page_page_count_count_1,
    behavior: "route_exists",
    router_args: (shared::testing::test_pool()),
    body: String::new(),
);

parity_test!(
    crate: hotpic_assemble_control,
    router_fn: router,
    route: "/jaxrs/hotpic_assemble_control/list/control/applications",
    method: GET,
    handler: list_control_applications_1,
    test_name: parity_behavior__hotpic_assemble_control__list_control_applications_1,
    behavior: "list_returns_array",
    router_args: (shared::testing::test_pool()),
    body: String::new(),
);

// ── other routes: behavior = route_exists ──

parity_test!(
    crate: ai_assemble_control,
    router_fn: router,
    route: "/jaxrs/ai/assemble/control/config/create/mcp",
    method: POST,
    handler: config_create_mcp,
    test_name: parity_behavior__ai_assemble_control__config_create_mcp,
    behavior: "route_exists",
    router_args: (shared::testing::test_pool()),
    body: String::new(),
);

parity_test!(
    crate: ai_assemble_control,
    router_fn: router,
    route: "/jaxrs/ai/assemble/control/config/delete/mcp/test-id",
    method: POST,
    handler: config_delete_mcp_flag,
    test_name: parity_behavior__ai_assemble_control__config_delete_mcp_flag,
    behavior: "route_exists",
    router_args: (shared::testing::test_pool()),
    body: String::new(),
);

parity_test!(
    crate: ai_assemble_control,
    router_fn: router,
    route: "/jaxrs/ai/assemble/control/config/get/mcp/test-id",
    method: GET,
    handler: config_get_mcp_flag,
    test_name: parity_behavior__ai_assemble_control__config_get_mcp_flag,
    behavior: "route_exists",
    router_args: (shared::testing::test_pool()),
    body: String::new(),
);

parity_test!(
    crate: ai_assemble_control,
    router_fn: router,
    route: "/jaxrs/ai/assemble/control/config/update/mcp/test-id",
    method: POST,
    handler: config_update_mcp_flag,
    test_name: parity_behavior__ai_assemble_control__config_update_mcp_flag,
    behavior: "route_exists",
    router_args: (shared::testing::test_pool()),
    body: String::new(),
);

parity_test!(
    crate: ai_assemble_control,
    router_fn: router,
    route: "/jaxrs/ai_assemble_control/chat/completion",
    method: POST,
    handler: chat_completion,
    test_name: parity_behavior__ai_assemble_control__chat_completion,
    behavior: "route_exists",
    router_args: (shared::testing::test_pool()),
    body: String::new(),
);

parity_test!(
    crate: ai_assemble_control,
    router_fn: router,
    route: "/jaxrs/ai_assemble_control/config/base/config",
    method: GET,
    handler: config_base_config,
    test_name: parity_behavior__ai_assemble_control__config_base_config,
    behavior: "route_exists",
    router_args: (shared::testing::test_pool()),
    body: String::new(),
);

parity_test!(
    crate: ai_assemble_control,
    router_fn: router,
    route: "/jaxrs/ai_assemble_control/config/create/mcp",
    method: GET,
    handler: config_create_mcp_1,
    test_name: parity_behavior__ai_assemble_control__config_create_mcp_1,
    behavior: "route_exists",
    router_args: (shared::testing::test_pool()),
    body: String::new(),
);

parity_test!(
    crate: ai_assemble_control,
    router_fn: router,
    route: "/jaxrs/ai_assemble_control/config/create/model",
    method: GET,
    handler: config_create_model,
    test_name: parity_behavior__ai_assemble_control__config_create_model,
    behavior: "route_exists",
    router_args: (shared::testing::test_pool()),
    body: String::new(),
);

parity_test!(
    crate: ai_assemble_control,
    router_fn: router,
    route: "/jaxrs/ai_assemble_control/config/delete/mcp/flag",
    method: GET,
    handler: config_delete_mcp_flag_1,
    test_name: parity_behavior__ai_assemble_control__config_delete_mcp_flag_1,
    behavior: "route_exists",
    router_args: (shared::testing::test_pool()),
    body: String::new(),
);

parity_test!(
    crate: ai_assemble_control,
    router_fn: router,
    route: "/jaxrs/ai_assemble_control/config/delete/model/flag",
    method: GET,
    handler: config_delete_model_flag,
    test_name: parity_behavior__ai_assemble_control__config_delete_model_flag,
    behavior: "route_exists",
    router_args: (shared::testing::test_pool()),
    body: String::new(),
);

parity_test!(
    crate: ai_assemble_control,
    router_fn: router,
    route: "/jaxrs/ai_assemble_control/config/get/mcp/ext/flag",
    method: GET,
    handler: config_get_mcp_ext_flag,
    test_name: parity_behavior__ai_assemble_control__config_get_mcp_ext_flag,
    behavior: "route_exists",
    router_args: (shared::testing::test_pool()),
    body: String::new(),
);

parity_test!(
    crate: ai_assemble_control,
    router_fn: router,
    route: "/jaxrs/ai_assemble_control/config/get/mcp/flag",
    method: GET,
    handler: config_get_mcp_flag_1,
    test_name: parity_behavior__ai_assemble_control__config_get_mcp_flag_1,
    behavior: "route_exists",
    router_args: (shared::testing::test_pool()),
    body: String::new(),
);

parity_test!(
    crate: ai_assemble_control,
    router_fn: router,
    route: "/jaxrs/ai_assemble_control/config/get/model/flag",
    method: GET,
    handler: config_get_model_flag,
    test_name: parity_behavior__ai_assemble_control__config_get_model_flag,
    behavior: "route_exists",
    router_args: (shared::testing::test_pool()),
    body: String::new(),
);

parity_test!(
    crate: ai_assemble_control,
    router_fn: router,
    route: "/jaxrs/ai_assemble_control/config/save",
    method: GET,
    handler: config_save,
    test_name: parity_behavior__ai_assemble_control__config_save,
    behavior: "route_exists",
    router_args: (shared::testing::test_pool()),
    body: String::new(),
);

parity_test!(
    crate: ai_assemble_control,
    router_fn: router,
    route: "/jaxrs/ai_assemble_control/config/update/mcp/flag",
    method: GET,
    handler: config_update_mcp_flag_1,
    test_name: parity_behavior__ai_assemble_control__config_update_mcp_flag_1,
    behavior: "route_exists",
    router_args: (shared::testing::test_pool()),
    body: String::new(),
);

parity_test!(
    crate: ai_assemble_control,
    router_fn: router,
    route: "/jaxrs/ai_assemble_control/config/update/model/flag",
    method: GET,
    handler: config_update_model_flag,
    test_name: parity_behavior__ai_assemble_control__config_update_model_flag,
    behavior: "route_exists",
    router_args: (shared::testing::test_pool()),
    body: String::new(),
);

parity_test!(
    crate: ai_assemble_control,
    router_fn: router,
    route: "/jaxrs/ai_assemble_control/file/copy/file",
    method: GET,
    handler: file_copy_file,
    test_name: parity_behavior__ai_assemble_control__file_copy_file,
    behavior: "route_exists",
    router_args: (shared::testing::test_pool()),
    body: String::new(),
);

parity_test!(
    crate: ai_assemble_control,
    router_fn: router,
    route: "/jaxrs/ai_assemble_control/file/delete/flag",
    method: GET,
    handler: file_delete_flag,
    test_name: parity_behavior__ai_assemble_control__file_delete_flag,
    behavior: "route_exists",
    router_args: (shared::testing::test_pool()),
    body: String::new(),
);

parity_test!(
    crate: ai_assemble_control,
    router_fn: router,
    route: "/jaxrs/ai_assemble_control/file/flag",
    method: GET,
    handler: file_flag,
    test_name: parity_behavior__ai_assemble_control__file_flag,
    behavior: "route_exists",
    router_args: (shared::testing::test_pool()),
    body: String::new(),
);

parity_test!(
    crate: ai_assemble_control,
    router_fn: router,
    route: "/jaxrs/ai_assemble_control/file/id/download",
    method: GET,
    handler: file_id_download,
    test_name: parity_behavior__ai_assemble_control__file_id_download,
    behavior: "route_exists",
    router_args: (shared::testing::test_pool()),
    body: String::new(),
);

parity_test!(
    crate: ai_assemble_control,
    router_fn: router,
    route: "/jaxrs/ai_assemble_control/file/id/download/scale",
    method: GET,
    handler: file_id_download_scale,
    test_name: parity_behavior__ai_assemble_control__file_id_download_scale,
    behavior: "route_exists",
    router_args: (shared::testing::test_pool()),
    body: String::new(),
);

parity_test!(
    crate: ai_assemble_control,
    router_fn: router,
    route: "/jaxrs/ai_assemble_control/file/upload",
    method: GET,
    handler: file_upload,
    test_name: parity_behavior__ai_assemble_control__file_upload,
    behavior: "route_exists",
    router_args: (shared::testing::test_pool()),
    body: String::new(),
);

parity_test!(
    crate: ai_assemble_control,
    router_fn: router,
    route: "/jaxrs/ai_assemble_control/get/ai/control/config",
    method: GET,
    handler: get_ai_control_config,
    test_name: parity_behavior__ai_assemble_control__get_ai_control_config,
    behavior: "route_exists",
    router_args: (shared::testing::test_pool()),
    body: String::new(),
);

parity_test!(
    crate: ai_assemble_control,
    router_fn: router,
    route: "/jaxrs/ai_assemble_control/get/usage/stats",
    method: GET,
    handler: get_usage_stats,
    test_name: parity_behavior__ai_assemble_control__get_usage_stats,
    behavior: "route_exists",
    router_args: (shared::testing::test_pool()),
    body: String::new(),
);

parity_test!(
    crate: ai_assemble_control,
    router_fn: router,
    route: "/jaxrs/ai_assemble_control/index/cms/doc/docId",
    method: GET,
    handler: index_cms_doc_docId,
    test_name: parity_behavior__ai_assemble_control__index_cms_doc_docId,
    behavior: "route_exists",
    router_args: (shared::testing::test_pool()),
    body: String::new(),
);

parity_test!(
    crate: ai_assemble_control,
    router_fn: router,
    route: "/jaxrs/ai_assemble_control/index/cms/doc/with/app/appId",
    method: GET,
    handler: index_cms_doc_with_app_appId,
    test_name: parity_behavior__ai_assemble_control__index_cms_doc_with_app_appId,
    behavior: "route_exists",
    router_args: (shared::testing::test_pool()),
    body: String::new(),
);

parity_test!(
    crate: ai_assemble_control,
    router_fn: router,
    route: "/jaxrs/ai_assemble_control/index/delete/flag",
    method: GET,
    handler: index_delete_flag,
    test_name: parity_behavior__ai_assemble_control__index_delete_flag,
    behavior: "route_exists",
    router_args: (shared::testing::test_pool()),
    body: String::new(),
);

parity_test!(
    crate: ai_assemble_control,
    router_fn: router,
    route: "/jaxrs/ai_assemble_control/index/sync/to/knowledge",
    method: GET,
    handler: index_sync_to_knowledge,
    test_name: parity_behavior__ai_assemble_control__index_sync_to_knowledge,
    behavior: "route_exists",
    router_args: (shared::testing::test_pool()),
    body: String::new(),
);

parity_test!(
    crate: ai_assemble_control,
    router_fn: router,
    route: "/jaxrs/ai_assemble_control/update/ai/control/config",
    method: GET,
    handler: update_ai_control_config,
    test_name: parity_behavior__ai_assemble_control__update_ai_control_config,
    behavior: "route_exists",
    router_args: (shared::testing::test_pool()),
    body: String::new(),
);

parity_test!(
    crate: ai,
    router_fn: router,
    route: "/jaxrs/ai/chat/delete/test-id",
    method: GET,
    handler: chat_delete,
    test_name: parity_behavior__ai__chat_delete,
    behavior: "route_exists",
    router_args: (shared::testing::test_pool()),
    body: String::new(),
);

parity_test!(
    crate: ai,
    router_fn: router,
    route: "/jaxrs/ai/config/base/config",
    method: GET,
    handler: config_base_config,
    test_name: parity_behavior__ai__config_base_config,
    behavior: "route_exists",
    router_args: (shared::testing::test_pool()),
    body: String::new(),
);

parity_test!(
    crate: ai,
    router_fn: router,
    route: "/jaxrs/ai/config/get",
    method: GET,
    handler: config_get,
    test_name: parity_behavior__ai__config_get,
    behavior: "route_exists",
    router_args: (shared::testing::test_pool()),
    body: String::new(),
);

parity_test!(
    crate: ai,
    router_fn: router,
    route: "/jaxrs/ai/config/get/mcp/test-id",
    method: GET,
    handler: config_get_mcp,
    test_name: parity_behavior__ai__config_get_mcp,
    behavior: "route_exists",
    router_args: (shared::testing::test_pool()),
    body: String::new(),
);

parity_test!(
    crate: ai,
    router_fn: router,
    route: "/jaxrs/ai/config/get/model/test-id",
    method: GET,
    handler: config_get_model,
    test_name: parity_behavior__ai__config_get_model,
    behavior: "route_exists",
    router_args: (shared::testing::test_pool()),
    body: String::new(),
);

parity_test!(
    crate: ai,
    router_fn: router,
    route: "/jaxrs/ai/file/delete/test-id",
    method: GET,
    handler: file_delete,
    test_name: parity_behavior__ai__file_delete,
    behavior: "route_exists",
    router_args: (shared::testing::test_pool()),
    body: String::new(),
);

