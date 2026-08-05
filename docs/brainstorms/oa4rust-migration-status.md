# OA4Rust 迁移进度跟踪清单

**更新时间：** 2026-08-05
**参照计划：** `docs/plans/2026-08-05-001-feat-oa4rust-comprehensive-advancement-plan.md`（U7 单元）

> 本文档是团队追踪 O2OA（Java）→ OA4Rust 迁移进度的**单一信息源（Single Source of Truth）**。
> 每次 sprint 结束时按末尾「如何更新」一节对齐实际代码状态。若代码与本文档不一致，以本文档为准先更新，再排查代码。

## 状态图例

| 状态 | 含义 |
|------|------|
| 待迁移 | crate 尚未创建，或尚未注册任何路由 |
| 已接入（桩代码） | crate 已注册路由，但端点为桩实现（返回空列表 / `ActionResult::success(Value::Null)`），未接入数据库真实逻辑 |
| 真实化中 | 端点开始接入 PostgreSQL / 真实业务逻辑，部分完成，尚未全量验证 |
| 已完成 | 核心端点已按 Java Action 契约路径实现真实逻辑并通过验证 |

接入 `main.rs` 的版本：U1 起 `main.rs` 真正参与编译（ac279a0f），6 个 router 已挂载；其余桩 crate 待 U2 接入。

## 全部 80 个 crate 的迁移状态

**状态分布：** 已完成（真实化）4 个 · 已完成（基础设施）1 个 · 已接入（桩代码，已挂载）1 个 · 已接入（桩代码，未挂载，待 U2）74 个

| crate 名称 | 对应 Java 模块 | 当前状态 | 接入 main.rs 的版本 | 已接入的端点列表 |
|------------|----------------|----------|----------------------|------------------|
| ai | x_ai_assemble_control | 已接入（桩代码，未挂载 main.rs，待 U2） | —（待 U2） | 桩 |
| ai_assemble_control | x_ai_assemble_control | 已接入（桩代码，未挂载 main.rs，待 U2） | —（待 U2） | 桩 |
| ai_core_entity | x_ai_core_entity | 已接入（桩代码，未挂载 main.rs，待 U2） | —（待 U2） | 桩 |
| attendance | x_attendance_assemble_control | 已接入（桩代码，未挂载 main.rs，待 U2） | —（待 U2） | 桩 |
| attendance_assemble_control | x_attendance_assemble_control | 已接入（桩代码，未挂载 main.rs，待 U2） | —（待 U2） | 桩 |
| attendance_core_entity | x_attendance_core_entity | 已接入（桩代码，未挂载 main.rs，待 U2） | —（待 U2） | 桩 |
| auth | x_organization_assemble_authentication | 已完成（真实化） | ac279a0f（U1） | POST /jaxrs/authentication/login、POST /jaxrs/authentication/logout、GET /jaxrs/authentication/who、GET /jaxrs/authentication/captcha（占位）、POST /jaxrs/authentication/bind、POST /jaxrs/authentication/oauth（占位）、POST /jaxrs/authentication/refresh、POST /jaxrs/authentication/code（占位）、POST /jaxrs/secret/check、POST /jaxrs/secret/set、POST /jaxrs/secret/cancel（secret 三端点目前由 auth 侧 `secret::router` 提供，U4 归属 program_init） |
| base | x_base_core_project | 已接入（桩代码，未挂载 main.rs，待 U2） | —（待 U2） | 桩 |
| bbs | x_bbs_assemble_control | 已接入（桩代码，未挂载 main.rs，待 U2） | —（待 U2） | 桩 |
| bbs_assemble_control | x_bbs_assemble_control | 已接入（桩代码，未挂载 main.rs，待 U2） | —（待 U2） | 桩 |
| bbs_core_entity | x_bbs_core_entity | 已接入（桩代码，未挂载 main.rs，待 U2） | —（待 U2） | 桩 |
| calendar | x_calendar_assemble_control | 已接入（桩代码，未挂载 main.rs，待 U2） | —（待 U2） | 桩 |
| calendar_assemble_control | x_calendar_assemble_control | 已接入（桩代码，未挂载 main.rs，待 U2） | —（待 U2） | 桩 |
| calendar_core_entity | x_calendar_core_entity | 已接入（桩代码，未挂载 main.rs，待 U2） | —（待 U2） | 桩 |
| cms_assemble_control | x_cms_assemble_control | 已接入（桩代码，未挂载 main.rs，待 U2） | —（待 U2） | 桩 |
| cms_control | x_cms_assemble_control | 已接入（桩代码，已挂载） | ac279a0f（U1） | GET /jaxrs/cms/view/list/all |
| cms_core_entity | x_cms_core_entity | 已接入（桩代码，未挂载 main.rs，待 U2） | —（待 U2） | 桩 |
| cms_core_express | x_cms_core_express | 已接入（桩代码，未挂载 main.rs，待 U2） | —（待 U2） | 桩 |
| cms_express | x_cms_core_express | 已接入（桩代码，未挂载 main.rs，待 U2） | —（待 U2） | 桩 |
| component | x_component_assemble_control | 已接入（桩代码，未挂载 main.rs，待 U2） | —（待 U2） | 桩 |
| component_assemble_control | x_component_assemble_control | 已接入（桩代码，未挂载 main.rs，待 U2） | —（待 U2） | 桩 |
| component_core_entity | x_component_core_entity | 已接入（桩代码，未挂载 main.rs，待 U2） | —（待 U2） | 桩 |
| console | x_console | 已接入（桩代码，未挂载 main.rs，待 U2） | —（待 U2） | 桩 |
| control | x_organization_assemble_control | 已完成（真实化） | ac279a0f（U1）；契约路径真实化 97fabf1c（U3） | POST /jaxrs/person、GET\|PUT\|DELETE /jaxrs/person/{flag}、GET /jaxrs/person/list/{flag}/next/{count}、GET /jaxrs/person/list/{flag}/prev/{count}；POST /jaxrs/group、GET\|PUT\|DELETE /jaxrs/group/{flag}、GET /jaxrs/group/list/{flag}/next\|prev/{count}；POST /jaxrs/role、GET\|PUT\|DELETE /jaxrs/role/{flag}、GET /jaxrs/role/list/{flag}/next\|prev/{count}；POST /jaxrs/unit、GET /jaxrs/unit/list、GET\|PUT\|DELETE /jaxrs/unit/{flag}、GET /jaxrs/unit/list/{flag}/next\|prev/{count} |
| correlation | x_correlation_core_entity | 已接入（桩代码，未挂载 main.rs，待 U2） | —（待 U2） | 桩 |
| correlation_core_entity | x_correlation_core_entity | 已接入（桩代码，未挂载 main.rs，待 U2） | —（待 U2） | 桩 |
| correlation_core_express | x_correlation_core_express | 已接入（桩代码，未挂载 main.rs，待 U2） | —（待 U2） | 桩 |
| correlation_service_processing | x_correlation_service_processing | 已接入（桩代码，未挂载 main.rs，待 U2） | —（待 U2） | 桩 |
| express | 快递查询应用（Java 侧无独立模块，待确认） | 已接入（桩代码，未挂载 main.rs，待 U2） | —（待 U2） | 桩 |
| file | x_file_assemble_control | 已接入（桩代码，未挂载 main.rs，待 U2） | —（待 U2） | 桩 |
| file_assemble_control | x_file_assemble_control | 已接入（桩代码，未挂载 main.rs，待 U2） | —（待 U2） | 桩 |
| file_core_entity | x_file_core_entity | 已接入（桩代码，未挂载 main.rs，待 U2） | —（待 U2） | 桩 |
| general | x_general_assemble_control | 已接入（桩代码，未挂载 main.rs，待 U2） | —（待 U2） | 桩 |
| general_assemble_control | x_general_assemble_control | 已接入（桩代码，未挂载 main.rs，待 U2） | —（待 U2） | 桩 |
| general_core_entity | x_general_core_entity | 已接入（桩代码，未挂载 main.rs，待 U2） | —（待 U2） | 桩 |
| hotpic | x_hotpic_assemble_control | 已接入（桩代码，未挂载 main.rs，待 U2） | —（待 U2） | 桩 |
| hotpic_assemble_control | x_hotpic_assemble_control | 已接入（桩代码，未挂载 main.rs，待 U2） | —（待 U2） | 桩 |
| hotpic_core_entity | x_hotpic_core_entity | 已接入（桩代码，未挂载 main.rs，待 U2） | —（待 U2） | 桩 |
| jpush | x_jpush_assemble_control | 已接入（桩代码，未挂载 main.rs，待 U2） | —（待 U2） | 桩 |
| jpush_assemble_control | x_jpush_assemble_control | 已接入（桩代码，未挂载 main.rs，待 U2） | —（待 U2） | 桩 |
| jpush_core_entity | x_jpush_core_entity | 已接入（桩代码，未挂载 main.rs，待 U2） | —（待 U2） | 桩 |
| meeting | x_meeting_assemble_control | 已接入（桩代码，未挂载 main.rs，待 U2） | —（待 U2） | 桩 |
| meeting_assemble_control | x_meeting_assemble_control | 已接入（桩代码，未挂载 main.rs，待 U2） | —（待 U2） | 桩 |
| meeting_core_entity | x_meeting_core_entity | 已接入（桩代码，未挂载 main.rs，待 U2） | —（待 U2） | 桩 |
| message | x_message_assemble_communicate | 已接入（桩代码，未挂载 main.rs，待 U2） | —（待 U2） | 桩 |
| message_assemble_communicate | x_message_assemble_communicate | 已接入（桩代码，未挂载 main.rs，待 U2） | —（待 U2） | 桩 |
| message_core_entity | x_message_core_entity | 已接入（桩代码，未挂载 main.rs，待 U2） | —（待 U2） | 桩 |
| mind | x_mind_assemble_control | 已接入（桩代码，未挂载 main.rs，待 U2） | —（待 U2） | 桩 |
| mind_assemble_control | x_mind_assemble_control | 已接入（桩代码，未挂载 main.rs，待 U2） | —（待 U2） | 桩 |
| mind_core_entity | x_mind_core_entity | 已接入（桩代码，未挂载 main.rs，待 U2） | —（待 U2） | 桩 |
| organization_assemble_express | x_organization_assemble_express | 已接入（桩代码，未挂载 main.rs，待 U2） | —（待 U2） | 桩 |
| organization_core_entity | x_organization_core_entity | 已接入（桩代码，未挂载 main.rs，待 U2） | —（待 U2） | 桩 |
| organization_core_express | x_organization_core_express | 已接入（桩代码，未挂载 main.rs，待 U2） | —（待 U2） | 桩 |
| personal | x_organization_assemble_personal | 已完成（真实化） | ac279a0f（U1） | GET /jaxrs/person、PUT /jaxrs/person、POST /jaxrs/person/mockputtopost、PUT /jaxrs/password、POST /jaxrs/password/mockputtopost、POST /jaxrs/reset/code、POST /jaxrs/reset/check、POST /jaxrs/reset/set |
| personal_extend | x_organization_assemble_personal | 已完成（真实化） | ac279a0f（U1） | GET /jaxrs/personal/info、PUT /jaxrs/personal/update、GET /jaxrs/personal/detail/{id}、POST /jaxrs/password/change、POST /jaxrs/password/reset、POST /jaxrs/password/verify、POST /jaxrs/personal/avatar/upload、GET /jaxrs/personal/avatar/{id} |
| portal | x_portal_assemble_surface | 已接入（桩代码，未挂载 main.rs，待 U2） | —（待 U2） | 桩 |
| portal_assemble_designer | x_portal_assemble_designer | 已接入（桩代码，未挂载 main.rs，待 U2） | —（待 U2） | 桩 |
| portal_assemble_surface | x_portal_assemble_surface | 已接入（桩代码，未挂载 main.rs，待 U2） | —（待 U2） | 桩 |
| portal_core_entity | x_portal_core_entity | 已接入（桩代码，未挂载 main.rs，待 U2） | —（待 U2） | 桩 |
| process_bam | x_processplatform_assemble_bam | 已接入（桩代码，未挂载 main.rs，待 U2） | —（待 U2） | 桩 |
| process_designer | x_processplatform_assemble_designer | 已接入（桩代码，未挂载 main.rs，待 U2） | —（待 U2） | 桩 |
| process_express | x_processplatform_core_express | 已接入（桩代码，未挂载 main.rs，待 U2） | —（待 U2） | 桩 |
| process_surface | x_processplatform_assemble_surface | 已接入（桩代码，未挂载 main.rs，待 U2） | —（待 U2） | 桩 |
| processplatform_assemble_bam | x_processplatform_assemble_bam | 已接入（桩代码，未挂载 main.rs，待 U2） | —（待 U2） | 桩 |
| processplatform_assemble_designer | x_processplatform_assemble_designer | 已接入（桩代码，未挂载 main.rs，待 U2） | —（待 U2） | 桩 |
| processplatform_assemble_surface | x_processplatform_assemble_surface | 已接入（桩代码，未挂载 main.rs，待 U2） | —（待 U2） | 桩 |
| processplatform_core_entity | x_processplatform_core_entity | 已接入（桩代码，未挂载 main.rs，待 U2） | —（待 U2） | 桩 |
| processplatform_core_express | x_processplatform_core_express | 已接入（桩代码，未挂载 main.rs，待 U2） | —（待 U2） | 桩 |
| processplatform_service_processing | x_processplatform_service_processing | 已接入（桩代码，未挂载 main.rs，待 U2） | —（待 U2） | 桩 |
| program_center | x_program_center | 已接入（桩代码，未挂载 main.rs，待 U2） | —（待 U2） | 桩 |
| program_center_core_entity | x_program_center_core_entity | 已接入（桩代码，未挂载 main.rs，待 U2） | —（待 U2） | 桩 |
| program_init | x_program_init | 已接入（桩代码，未挂载 main.rs，待 U2） | —（待 U2） | 桩 |
| query_assemble_designer | x_query_assemble_designer | 已接入（桩代码，未挂载 main.rs，待 U2） | —（待 U2） | 桩 |
| query_assemble_surface | x_query_assemble_surface | 已接入（桩代码，未挂载 main.rs，待 U2） | —（待 U2） | 桩 |
| query_core_entity | x_query_core_entity | 已接入（桩代码，未挂载 main.rs，待 U2） | —（待 U2） | 桩 |
| query_core_express | x_query_core_express | 已接入（桩代码，未挂载 main.rs，待 U2） | —（待 U2） | 桩 |
| query_express | x_query_core_express | 已接入（桩代码，未挂载 main.rs，待 U2） | —（待 U2） | 桩 |
| query_service | x_query_service_processing | 已接入（桩代码，未挂载 main.rs，待 U2） | —（待 U2） | 桩 |
| query_service_processing | x_query_service_processing | 已接入（桩代码，未挂载 main.rs，待 U2） | —（待 U2） | 桩 |
| shared | x_base_core_project（基础设施） | 已完成（基础设施） | ac279a0f（U1） | GET /health |

## 如何更新

每次 sprint 结束时，按实际代码状态更新本清单：

1. **更新表头**：将「更新时间」改为当天日期。
2. **核对 crate 清单**：运行 `Get-ChildItem crates -Directory`（在 `oa4rust` 目录）比对表内 crate，数量应与 workspace 成员一致（当前 80 个）。新增 / 删除 crate 时同步更新表格与顶部状态分布统计。
3. **更新状态列**：仅在实际完成后改动状态（桩 → 真实化中 → 已完成），依据：端点是否已接入 PostgreSQL 真实逻辑并通过 `cargo test`。
4. **更新端点列表**：对状态为「已完成」或「真实化中」的 crate，以实际路由注册为准更新「已接入的端点列表」（对照 `crates/<name>/src/routes.rs` 或 `lib.rs` 的 `route(...)` 调用；路径必须对齐 Java Action 契约 `/jaxrs/*`，非 Rust 自造路径）。
5. **更新「接入 main.rs 的版本」**：crate 首次挂载 `main.rs` 时记录对应 git commit（U2 完成后应无「—（待 U2）」条目）。
6. **约定说明**：
   - 对应 Java 模块列中，短名 crate（如 `ai`）与同名 `_assemble_control` / `_core_entity` crate 可能共享一个 Java 模块（一个 Java 模块被拆分为多个 crate）；映射有歧义时以 U2 路由归属 / `main.rs` 实际挂载为准，并在表格备注。
   - 桩代码端点统一标记「桩」，不逐条枚举，待真实化后按第 4 条补充。