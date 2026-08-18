# OA4Rust 迁移进度跟踪清单

> **⚠️ 已过期** — 此文档（2026-08-07 版）已被 `docs/brainstorms/oa4rust-migration-status-2026-08-08.md` 取代。
> 新文档追踪 81 个 crate 全部完成 SeaORM 迁移和写操作补齐。请勿引用本文档。

**更新时间：** 2026-08-07（已过期）
**参照需求：** `docs/brainstorms/2026-08-05-oa4rust-comprehensive-advancement-requirements.md`
**取代文档：** `docs/brainstorms/oa4rust-migration-status-2026-08-08.md`

> 本文档是团队追踪 O2OA（Java）→ OA4Rust 迁移进度的**单一信息源（Single Source of Truth）**。
> 每次 sprint 结束时按末尾「如何更新」一节对齐实际代码状态。若代码与本文档不一致，以本文档为准先更新，再排查代码。

## 状态图例

| 状态 | 含义 |
|------|------|
| 已完成（真实化） | crate 全部或核心端点已接入 PostgreSQL 真实业务逻辑，返回真实数据 |
| 部分真实化 | crate 包含真实 PostgreSQL 查询代码，但部分端点仍为桩实现或未挂载路由 |
| 已接入（桩代码） | crate 已注册路由，但端点为桩实现（返回空列表 / `ActionResult::success(Value::Null)` / mock 数据），未接入数据库真实逻辑 |

## 全部 80 个 crate 的迁移状态

**状态分布：** 已完成（真实化）10 个 · 部分真实化 20 个 · 已接入（桩代码）50 个

| crate 名称 | 对应 Java 模块 | 当前状态 | 接入 main.rs 的版本 | 已接入的端点列表 |
|------------|----------------|----------|----------------------|------------------|
| ai | x_ai_assemble_control | 部分真实化 | — | 部分真实（app_list、model_list、conversation_list 等 PostgreSQL 查询已实现；config 类端点仍为 mock） |
| ai_assemble_control | x_ai_assemble_control | 已接入（桩代码） | — | 桩 |
| ai_core_entity | x_ai_core_entity | 已接入（桩代码） | — | 桩 |
| attendance | x_attendance_assemble_control | 部分真实化 | — | 部分真实（list_admins、list_employee_configs、list_statistical_cycles 等 PostgreSQL 查询已实现） |
| attendance_assemble_control | x_attendance_assemble_control | 部分真实化 | — | 部分真实（rule/list/toggle、admin list 等 PostgreSQL 查询已实现；97 个路由中约 10 个已真实） |
| attendance_core_entity | x_attendance_core_entity | 部分真实化 | — | 部分真实（record_list、rule_list、appeal_list 等 PostgreSQL 查询已实现） |
| auth | x_organization_assemble_authentication | 已完成（真实化） | ac279a0f（U1） | POST /jaxrs/authentication/login、DELETE /jaxrs/authentication/logout、GET /jaxrs/authentication/who、GET /jaxrs/authentication/captcha（真实 PNG 生成）、POST /jaxrs/authentication/oauth（微信/钉钉 OAuth 已实现）、POST /jaxrs/authentication/refresh、POST /jaxrs/authentication/code、POST /jaxrs/secret/check、POST /jaxrs/secret/set、POST /jaxrs/secret/cancel |
| base | x_base_core_project | 部分真实化 | — | 部分真实（cache_detail 查询 pg_class 已真实；echo/openapi 为固定响应） |
| bbs | x_bbs_assemble_control | 部分真实化 | — | 部分真实（forum/view/section/subject 的 list/view/create/search 等 PostgreSQL 查询已实现） |
| bbs_assemble_control | x_bbs_assemble_control | 部分真实化 | — | 部分真实（config/sections/forum/topic/reply 等 PostgreSQL 查询已实现；约 5 个端点仍为桩） |
| bbs_core_entity | x_bbs_core_entity | 部分真实化 | — | 部分真实（forum/section/subject 的 CRUD 已实现） |
| calendar | x_calendar_assemble_control | 部分真实化 | — | 部分真实（calendar_list_public、calendar_list_my、calendar_get 等 PostgreSQL 查询已实现） |
| calendar_assemble_control | x_calendar_assemble_control | 部分真实化 | — | 部分真实（config/calendars 查询已实现） |
| calendar_core_entity | x_calendar_core_entity | 部分真实化 | — | 部分真实（calendar CRUD 已实现） |
| cms_assemble_control | x_cms_assemble_control | 已完成（真实化） | — | GET /jaxrs/cms/assemble/control/config/get、GET /jaxrs/cms/assemble/control/sections、POST /jaxrs/cms/assemble/control/config/update |
| cms_control | x_cms_assemble_control | 已完成（真实化） | — | GET /jaxrs/cms_control/get/control/config、GET /jaxrs/cms_control/list/control/sections |
| cms_core_entity | x_cms_core_entity | 部分真实化 | — | 部分真实（category/app/config 查询已实现） |
| cms_core_express | x_cms_core_express | 已接入（桩代码） | — | 桩 |
| cms_express | x_cms_core_express | 已接入（桩代码） | — | 桩 |
| component | x_component_assemble_control | 部分真实化 | — | 部分真实（list/count/get 等 PostgreSQL 查询已实现） |
| component_assemble_control | x_component_assemble_control | 部分真实化 | — | 部分真实（config/categories/component CRUD 已实现） |
| component_core_entity | x_component_core_entity | 已接入（桩代码） | — | 桩 |
| console | x_console | 已接入（桩代码） | — | 桩 |
| control | x_organization_assemble_control | 已完成（真实化） | ac279a0f（U1） | POST /jaxrs/person、GET\|PUT\|DELETE /jaxrs/person/{flag}、GET /jaxrs/person/list/{flag}/next/{count}、GET /jaxrs/person/list/{flag}/prev/{count}；POST /jaxrs/group、GET\|PUT\|DELETE /jaxrs/group/{flag}、GET /jaxrs/group/list/{flag}/next\|prev/{count}；POST /jaxrs/role、GET\|PUT\|DELETE /jaxrs/role/{flag}、GET /jaxrs/role/list/{flag}/next\|prev/{count}；POST /jaxrs/unit、GET /jaxrs/unit/list、GET\|PUT\|DELETE /jaxrs/unit/{flag}、GET /jaxrs/unit/list/{flag}/next\|prev/{count} |
| correlation | x_correlation_core_entity | 部分真实化 | — | 部分真实（type cms/processplatform 查询、readable 检查等已实现） |
| correlation_core_entity | x_correlation_core_entity | 部分真实化 | — | 部分真实（list/list_by_source 等已实现） |
| correlation_core_express | x_correlation_core_express | 部分真实化 | — | 部分真实（status/sync 等已实现） |
| correlation_service_processing | x_correlation_service_processing | 部分真实化 | — | 部分真实（list/get/create/save/delete 等已实现；link/unlink 仍为桩） |
| express | 快递查询应用 | 部分真实化 | — | 部分真实（区域/安全 clearance 查询已实现；快递查询/订阅仍为 mock） |
| file | x_file_assemble_control | 部分真实化 | — | 部分真实（folder_list_top、folder_list_with_folder、complex_top 等 PostgreSQL 查询已实现） |
| file_assemble_control | x_file_assemble_control | 部分真实化 | — | 部分真实（config/storage/categories 查询已实现） |
| file_core_entity | x_file_core_entity | 部分真实化 | — | 部分真实（folder/file CRUD 已实现） |
| general | x_general_assemble_control | 部分真实化 | — | 部分真实（area_list 等已实现；security_clearance/worktime 仍为 mock） |
| general_assemble_control | x_general_assemble_control | 部分真实化 | — | 部分真实（status/permissions/attendscope/area/qrcode/securityclearance 等大量 PostgreSQL 查询已实现；71 个路由中约 30+ 已真实） |
| general_core_entity | x_general_core_entity | 部分真实化 | — | 部分真实（dict/file/invoice CRUD 已实现） |
| hotpic | x_hotpic_assemble_control | 部分真实化 | — | 部分真实（exists_check/get/list 等 PostgreSQL 查询已实现） |
| hotpic_assemble_control | x_hotpic_assemble_control | 部分真实化 | — | 部分真实（config/panels/applications/hotpic CRUD 已实现） |
| hotpic_core_entity | x_hotpic_core_entity | 部分真实化 | — | 部分真实（list/exists_check 等已实现） |
| jpush | x_jpush_assemble_control | 部分真实化 | — | 部分真实（device/template CRUD 已实现） |
| jpush_assemble_control | x_jpush_assemble_control | 部分真实化 | — | 部分真实（config/apps/message CRUD 已实现；设备管理路由部分为桩） |
| jpush_core_entity | x_jpush_core_entity | 部分真实化 | — | 部分真实（device/template CRUD 已实现） |
| meeting | x_meeting_assemble_control | 部分真实化 | — | 部分真实（create/get/list/add_participant/list_participants/list_schedule 等 PostgreSQL 查询已实现；room/building/openmeeting 仍为 mock） |
| meeting_assemble_control | x_meeting_assemble_control | 部分真实化 | — | 部分真实（list/create/delete/save 等 PostgreSQL 查询已实现；其余为桩） |
| meeting_core_entity | x_meeting_core_entity | 部分真实化 | — | 部分真实（room/meeting CRUD 已实现） |
| message | x_message_assemble_communicate | 已完成（真实化） | ac279a0f（U1） | GET /jaxrs/message/consume/list/{consume}/count/{count}、GET /jaxrs/message/consume/{id}/type/{type}、POST /jaxrs/message/custom/create、POST /jaxrs/message/mark_read/{id}、GET /jaxrs/message/unread/count/{consume} |
| message_assemble_communicate | x_message_assemble_communicate | 部分真实化 | — | 部分真实（send/receive/mark_read 等部分真实；118 个路由中仅少量已实现） |
| message_core_entity | x_message_core_entity | 部分真实化 | — | 部分真实（list/list_by_consume/unread_count 等已实现） |
| mind | x_mind_assemble_control | 部分真实化 | — | 部分真实（mind/folder/version CRUD 已实现） |
| mind_assemble_control | x_mind_assemble_control | 部分真实化 | — | 部分真实（config/folder 操作已实现；move/force 仍为桩） |
| mind_core_entity | x_mind_core_entity | 部分真实化 | — | 部分真实（mind/folder/version CRUD 已实现） |
| organization_assemble_express | x_organization_assemble_express | 部分真实化 | — | 部分真实（config/units/sync/status 等已实现） |
| organization_core_entity | x_organization_core_entity | 部分真实化 | — | 部分真实（definition/group/identity/person/custom/bind 列表已实现） |
| organization_core_express | x_organization_core_express | 部分真实化 | — | 部分真实（status/sync/config 等已实现） |
| personal | x_organization_assemble_personal | 已完成（真实化） | ac279a0f（U1） | GET /jaxrs/person、PUT /jaxrs/person、POST /jaxrs/person/mockputtopost、PUT /jaxrs/password、POST /jaxrs/password/mockputtopost、POST /jaxrs/reset/code、POST /jaxrs/reset/check、POST /jaxrs/reset/set |
| personal_extend | x_organization_assemble_personal | 已完成（真实化） | ac279a0f（U1） | GET /jaxrs/personal/info、PUT /jaxrs/personal/update、GET /jaxrs/personal/detail/{id}、POST /jaxrs/password/change、POST /jaxrs/password/reset、POST /jaxrs/password/verify、POST /jaxrs/personal/avatar/upload、GET /jaxrs/personal/avatar/{id} |
| portal | x_portal_assemble_surface | 部分真实化 | — | 部分真实（page/dict/widget/script 查询已实现；portal 本身仍为 mock） |
| portal_assemble_designer | x_portal_assemble_designer | 部分真实化 | — | 部分真实（page CRUD 已实现；design 相关仍为桩） |
| portal_assemble_surface | x_portal_assemble_surface | 部分真实化 | — | 部分真实（surface CRUD 已实现；preview/publish 仍为桩） |
| portal_core_entity | x_portal_core_entity | 部分真实化 | — | 部分真实（page CRUD 已实现） |
| process_bam | x_processplatform_assemble_bam | 已完成（真实化） | — | GET /jaxrs/process/state/summary、GET /jaxrs/process/state/running、GET /jaxrs/process/state/organization |
| process_designer | x_processplatform_assemble_designer | 部分真实化 | — | 部分真实（application/summary、designer/route 等查询已实现） |
| process_express | x_processplatform_core_express | 部分真实化 | — | 部分真实（task/read/application count 等查询已实现） |
| process_surface | x_processplatform_assemble_surface | 部分真实化 | — | 部分真实（list_ids/get/record 等查询已实现） |
| processplatform_assemble_bam | x_processplatform_assemble_bam | 部分真实化 | — | 部分真实（bam config/list 等查询已实现；20+ 统计路由为桩） |
| processplatform_assemble_designer | x_processplatform_assemble_designer | 部分真实化 | — | 部分真实（flow CRUD 已实现；preview 等仍为桩） |
| processplatform_assemble_surface | x_processplatform_assemble_surface | 部分真实化 | — | 部分真实（surface CRUD 已实现；preview/publish 等仍为桩） |
| processplatform_core_entity | x_processplatform_core_entity | 部分真实化 | — | 部分真实（work/task/ticket list 等查询已实现；workcompleted 为桩） |
| processplatform_core_express | x_processplatform_core_express | 部分真实化 | — | 部分真实（work/task terminate/retract/processing 等已实现） |
| processplatform_service_processing | x_processplatform_service_processing | 部分真实化 | — | 部分真实（get/create/execute/instance/cancel 等已实现；list 等仍为桩） |
| program_center | x_program_center | 部分真实化 | — | 部分真实（applications/style/collect/config 等已实现；datastructure/modules 为桩） |
| program_center_core_entity | x_program_center_core_entity | 部分真实化 | — | 部分真实（application/script/list 等已实现；invoke/agent/structure 为桩） |
| program_init | x_program_init | 已完成（真实化） | ac279a0f（U1） | GET /jaxrs/secret/check、POST /jaxrs/secret/set、GET /jaxrs/secret/set/cancel |
| query_assemble_designer | x_query_assemble_designer | 已接入（桩代码） | — | 桩 |
| query_assemble_surface | x_query_assemble_surface | 已接入（桩代码） | — | 桩 |
| query_core_entity | x_query_core_entity | 部分真实化 | — | 部分真实（item/view/import 查询已实现） |
| query_core_express | x_query_core_express | 已接入（桩代码） | — | 桩 |
| query_express | x_query_core_express | 已接入（桩代码） | — | 桩 |
| query_service | x_query_service_processing | 已完成（真实化） | — | POST /jaxrs/query/service/neural/generate/{model_flag}、GET /jaxrs/query/service/neural/list |
| query_service_processing | x_query_service_processing | 已接入（桩代码） | — | 桩 |
| shared | x_base_core_project（基础设施） | 已完成（基础设施） | ac279a0f（U1） | GET /health |

---

## 如何更新

每次 sprint 结束时，按实际代码状态更新本清单：

1. **更新表头**：将「更新时间」改为当天日期。
2. **核对 crate 清单**：运行 `Get-ChildItem crates -Directory`（在 `oa4rust` 目录）比对表内 crate，数量应与 workspace 成员一致（当前 80 个）。新增 / 删除 crate 时同步更新表格与顶部状态分布统计。
3. **更新状态列**：仅在实际完成后改动状态（桩 → 真实化中 → 已完成），依据：端点是否已接入 PostgreSQL 真实逻辑并通过 `cargo test`。
4. **更新端点列表**：对状态为「已完成」或「部分真实化」的 crate，以实际路由注册为准更新「已接入的端点列表」（对照 `crates/<name>/src/routes.rs` 或 `lib.rs` 的 `route(...)` 调用；路径必须对齐 Java Action 契约 `/jaxrs/*`，非 Rust 自造路径）。
5. **更新「接入 main.rs 的版本」**：crate 首次挂载 `main.rs` 时记录对应 git commit。
6. **约定说明**：
   - 对应 Java 模块列中，短名 crate（如 `ai`）与同名 `_assemble_control` / `_core_entity` crate 可能共享一个 Java 模块（一个 Java 模块被拆分为多个 crate）；映射有歧义时以 U2 路由归属 / `main.rs` 实际挂载为准，并在表格备注。
   - 桩代码端点统一标记「桩」，不逐条枚举，待真实化后按第 4 条补充。
   - 「部分真实化」表示 crate 包含真实 PostgreSQL 查询代码，但部分端点仍为桩实现或未挂载路由，需要补充完善。
