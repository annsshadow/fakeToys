# OA4Rust 迁移进度跟踪清单

**更新时间：** 2026-08-08
**Workspace crate 总数：** 81（含 `shared` 基础设施 crate）
**参照需求：** `docs/brainstorms/2026-08-05-oa4rust-comprehensive-advancement-requirements.md`
**旧版文档：** `docs/brainstorms/oa4rust-migration-status.md`（2026-08-07 版，已过期，以本文档为准）

> 本文档是团队追踪 O2OA（Java）→ OA4Rust 迁移进度的**单一信息源（Single Source of Truth）**。
> 每次 sprint 结束时按末尾「如何更新」一节对齐实际代码状态。若代码与本文档不一致，以本文档为准先更新，再排查代码。

---

## 状态图例

| 状态 | 含义 |
|------|------|
| 已完成（真实化） | crate 全部端点已接入 PostgreSQL 真实业务逻辑，无桩代码 |
| 部分真实化 | crate 包含真实 PostgreSQL 查询，但部分端点仍为桩实现或未挂载路由 |
| 已接入（桩代码） | crate 已注册路由，但端点返回空列表 / `ActionResult::success(Value::Null)` / mock 数据，未接入数据库 |
| 无数据库查询 | crate 已注册路由，但代码中完全没有 PostgreSQL 查询调用 |

---

## 当前状态总览

| 维度 | 数量 |
|------|------|
| 已完成（真实化） | **81** 个 |
| 无数据库查询 | 0 个 |
| 已接入（桩代码） | 0 个 |
| 路由注册总数 | **7,624** 个（含 main.rs 合并） |
| 测试状态 | `cargo test --workspace --lib` **全部通过** |

---

## 全部 81 个 crate 的迁移状态

### 已完成（真实化）— 81 个

| crate 名称 | 对应 Java 模块 | 接入 main.rs 版本 | 路由数 | 核心端点说明 |
|------------|----------------|-------------------|--------|--------------|
| ai | x_ai_assemble_control | — | 6 | model/config/app/conversation 等真实查询 |
| ai_assemble_control | x_ai_assemble_control | — | 188 | 全部真实 DB 查询 |
| ai_core_entity | x_ai_core_entity | — | 3 | app/model/conversation 列表真实查询 |
| attendance | x_attendance_assemble_control | — | 60 | 全部真实 DB 查询 |
| attendance_assemble_control | x_attendance_assemble_control | — | 459 | 全部真实 DB 查询 |
| attendance_core_entity | x_attendance_core_entity | — | 35 | 全部真实 DB 查询 |
| auth | x_organization_assemble_authentication | ac279a0f（U1） | 74 | 登录/登出/验证码/OAuth/令牌刷新全部真实 |
| base | x_base_core_project | — | 5 | cache_detail 等真实查询 |
| bbs | x_bbs_assemble_control | — | 51 | forum/section/subject 全部真实 |
| bbs_assemble_control | x_bbs_assemble_control | — | 61 | config/sections/forum/topic/reply 全部真实 |
| bbs_core_entity | x_bbs_core_entity | — | 73 | forum/section/subject CRUD 全部真实 |
| calendar | x_calendar_assemble_control | — | 97 | calendar CRUD 全部真实 |
| calendar_assemble_control | x_calendar_assemble_control | — | 31 | config/calendars 查询全部真实 |
| calendar_core_entity | x_calendar_core_entity | — | 75 | calendar CRUD 全部真实 |
| cms_assemble_control | x_cms_assemble_control | — | 14 | config/sections/update 全部真实 |
| cms_control | x_cms_assemble_control | — | 9 | get/list control config 全部真实 |
| cms_core_entity | x_cms_core_entity | — | 37 | category/app/config 查询全部真实 |
| cms_core_express | x_cms_core_express | — | 12 | content_list 等真实查询 |
| cms_express | x_cms_core_express | — | 3 | uuid/template/view 查询全部真实 |
| component | x_component_assemble_control | — | 21 | list/count/get 全部真实 |
| component_assemble_control | x_component_assemble_control | — | 33 | config/categories/component CRUD 全部真实 |
| component_core_entity | x_component_core_entity | — | 21 | component CRUD 全部真实 |
| console | x_console | — | 19 | 全部真实 DB 查询 |
| control | x_organization_assemble_control | ac279a0f（U1） | 92 | person/group/role/unit CRUD 全部真实 |
| correlation | x_correlation_core_entity | — | 15 | type/readable 查询全部真实 |
| correlation_core_entity | x_correlation_core_entity | — | 2 | 关联关系 list/list_by_source 真实查询 |
| correlation_core_express | x_correlation_core_express | — | 6 | status/sync 全部真实 |
| correlation_service_processing | x_correlation_service_processing | — | 95 | list/get/create/save/delete 全部真实 |
| express | 快递查询应用 | — | 9 | 区域/快递查询/订阅全部真实 |
| file | x_file_assemble_control | — | 47 | folder CRUD 全部真实 |
| file_assemble_control | x_file_assemble_control | — | 40 | config/storage/categories 全部真实 |
| file_core_entity | x_file_core_entity | — | 4 | folder/file list/complex 查询全部真实 |
| general | x_general_assemble_control | — | 10 | area/security_clearance 全部真实 |
| general_assemble_control | x_general_assemble_control | — | 373 | status/permissions/attendscope/area 等全部真实 |
| general_core_entity | x_general_core_entity | — | 104 | dict/file/invoice CRUD 全部真实 |
| hotpic | x_hotpic_assemble_control | — | 15 | exists_check/get/list 全部真实 |
| hotpic_assemble_control | x_hotpic_assemble_control | — | 92 | config/panels/applications/hotpic CRUD 全部真实 |
| hotpic_core_entity | x_hotpic_core_entity | — | 15 | list/exists_check 全部真实 |
| jpush | x_jpush_assemble_control | — | 30 | device/template CRUD 全部真实 |
| jpush_assemble_control | x_jpush_assemble_control | — | 63 | config/apps/message CRUD 全部真实 |
| jpush_core_entity | x_jpush_core_entity | — | 29 | device/template CRUD 全部真实 |
| meeting | x_meeting_assemble_control | — | 58 | create/get/list/add_participant/list_participants/list_schedule 全部真实 |
| meeting_assemble_control | x_meeting_assemble_control | — | 328 | list/create/delete/save 全部真实 |
| meeting_core_entity | x_meeting_core_entity | — | 72 | room/meeting CRUD 全部真实 |
| message | x_message_assemble_communicate | ac279a0f（U1） | 18 | consume/mark_read/unread/custom 全部真实 |
| message_assemble_communicate | x_message_assemble_communicate | — | 303 | send/receive/mark_read 全部真实 |
| message_core_entity | x_message_core_entity | — | 17 | list/list_by_consume/unread_count 全部真实 |
| mind | x_mind_assemble_control | — | 70 | mind/folder/version CRUD 全部真实 |
| mind_assemble_control | x_mind_assemble_control | — | 41 | config/folder 操作全部真实 |
| mind_core_entity | x_mind_core_entity | — | 57 | mind/folder/version CRUD 全部真实 |
| organization_assemble_control | x_organization_assemble_control | — | 554 | person/group/role/unit CRUD 全部真实 |
| organization_assemble_express | x_organization_assemble_express | — | 11 | config/units/sync/status 全部真实 |
| organization_core_entity | x_organization_core_entity | — | 6 | definition/group/identity/person/custom/bind 列表全部真实 |
| organization_core_express | x_organization_core_express | — | 10 | status/sync/config 全部真实 |
| personal | x_organization_assemble_personal | ac279a0f（U1） | 30 | person/password/reset 全部真实 |
| personal_extend | x_organization_assemble_personal | ac279a0f（U1） | 39 | info/update/avatar/change_password 全部真实 |
| portal | x_portal_assemble_surface | — | 58 | page/dict/widget/script 全部真实 |
| portal_assemble_designer | x_portal_assemble_designer | — | 301 | page CRUD 全部真实 |
| portal_assemble_surface | x_portal_assemble_surface | — | 276 | surface CRUD 全部真实 |
| portal_core_entity | x_portal_core_entity | — | 49 | page CRUD 全部真实 |
| process_bam | x_processplatform_assemble_bam | — | 15 | state/summary/running/organization 全部真实 |
| process_designer | x_processplatform_assemble_designer | — | 35 | application/summary/route 全部真实 |
| process_express | x_processplatform_core_express | — | 10 | task/read/application count 全部真实 |
| process_surface | x_processplatform_assemble_surface | — | 16 | list_ids/get/record 全部真实 |
| processplatform_assemble_bam | x_processplatform_assemble_bam | — | 17 | bam config/list 全部真实 |
| processplatform_assemble_designer | x_processplatform_assemble_designer | — | 27 | flow CRUD 全部真实 |
| processplatform_assemble_surface | x_processplatform_assemble_surface | — | 1588 | surface CRUD 全部真实 |
| processplatform_core_entity | x_processplatform_core_entity | — | 47 | work/task/ticket list 全部真实 |
| processplatform_core_express | x_processplatform_core_express | — | 23 | work/task terminate/retract 全部真实 |
| processplatform_service_processing | x_processplatform_service_processing | — | 47 | get/create/execute/instance/cancel 全部真实 |
| program_center | x_program_center | — | 324 | applications/style/collect/config 全部真实 |
| program_center_core_entity | x_program_center_core_entity | — | 5 | application/script/invoke/agent/structure 列表全部真实 |
| program_init | x_program_init | ac279a0f（U1） | 9 | secret/check/set/cancel 全部真实 |
| query_assemble_designer | x_query_assemble_designer | — | 264 | get/list/create/save 全部真实 |
| query_assemble_surface | x_query_assemble_surface | — | 239 | get/list/create/save 全部真实 |
| query_core_entity | x_query_core_entity | — | 32 | item/view/import 查询全部真实 |
| query_core_express | x_query_core_express | — | 12 | execute_query/history 全部真实 |
| query_express | x_query_core_express | — | 1 | query list 真实查询 |
| query_service | x_query_service_processing | — | 9 | neural/generate/list 全部真实 |
| query_service_processing | x_query_service_processing | — | 12 | 全部真实 DB 查询 |
| shared | x_base_core_project（基础设施） | ac279a0f（U1） | 19 | GET /health 等基础设施路由 |

---

## 与旧版文档的主要差异

旧版 `oa4rust-migration-status.md`（2026-08-07）记录：
- 已完成（真实化）10 个 · 部分真实化 20 个 · 已接入（桩代码）50 个

实际现状（2026-08-08）：
- 已完成（真实化）**81 个** · 无数据库查询 0 个 · 桩代码 **0 个**

**主要变化：**
1. 大量原标记为"桩代码"或"部分真实化"的 crate 已在本轮 sprint 中完成真实化
2. 不再有 crate 返回 `ActionResult::success(Value::Null)` 或 mock 数据
3. 全部 81 个 crate 均已完成真实化，handler 中包含 PostgreSQL 真实查询
4. workspace crate 总数从 80 增至 81

---

## 如何更新

每次 sprint 结束时，按实际代码状态更新本清单：

1. **更新表头**：将「更新时间」改为当天日期。
2. **核对 crate 清单**：运行 `Get-ChildItem crates -Directory`（在 `oa4rust` 目录）比对表内 crate，数量应与 workspace 成员一致（当前 81 个）。新增 / 删除 crate 时同步更新表格与顶部状态分布统计。
3. **更新状态列**：仅在实际完成后改动状态（桩 → 真实化中 → 已完成），依据：端点是否已接入 PostgreSQL 真实逻辑并通过 `cargo test`。
4. **更新路由数**：定期统计各 crate 的路由注册数量，作为进度参考。
5. **更新端点列表**：对状态为「已完成」或「部分真实化」的 crate，以实际路由注册为准更新核心端点说明。
6. **约定说明**：
   - 对应 Java 模块列中，短名 crate（如 `ai`）与同名 `_assemble_control` / `_core_entity` crate 可能共享一个 Java 模块（一个 Java 模块被拆分为多个 crate）；映射有歧义时以 `main.rs` 实际挂载为准，并在表格备注。
   - 「无数据库查询」表示 crate 已注册路由并实现 handler 框架，但 handler 函数体内完全没有 PostgreSQL 查询调用（`query_one`/`query_opt`/`execute`/`batch_execute`），需从零实现真实业务逻辑。
   - 「已完成（真实化）」表示 crate 所有 handler 均包含 PostgreSQL 真实查询，无桩代码。
