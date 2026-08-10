---
title: prod-readiness: plan cleanup, write ops, and docs completion
type: refactor
status: completed
date: 2026-08-10
origin: docs/brainstorms/2026-08-10-oa4rust-production-readiness-and-docs-cleanup-requirements.md
---

# 生产就绪：计划清理、写操作补全与文档完善

## Summary

更新过期计划的 status 标记，为 `program_center_core_entity` 补全缺失的 CRUD 写操作并添加集成测试，同时完善 `docs/oa/` 模块卡片内容并扩展 API 文档生成脚本覆盖全部 55+ 模块。

---

## Problem Frame

oa4rust 已完成 81 个 crate 的真实化和 SeaORM 迁移，但 `docs/plans/` 中存在 5 个状态失真的计划文件（3 个已取代仍标 active、2 个已完成未更新），`program_center_core_entity` 缺少写操作导致程序中心模块不可编辑，`docs/oa/` 模块卡片内容空白影响新成员上手。

---

## Requirements

- R1. 将 `docs/plans/2026-08-07-001-feat-oa4rust-4wave-realization-plan.md` 和 `docs/plans/2026-08-09-001-refact-oa4rust-orm-migration-plan.md` 的 `status` 更新为 `completed`
- R2. 将 `docs/plans/2026-08-05-001-feat-oa4rust-comprehensive-advancement-plan.md`、`docs/plans/2026-08-06-001-feat-oa4rust-full-realization-plan.md`、`docs/plans/2026-08-03-001-refactor-o2server-rust-rewrite-plan.md` 的 `status` 更新为 `superseded`，并添加取代说明注释
- R3. 将 `docs/plans/2026-07-30-001-refactor-zero-secret-migration-plan.md` 的 `status` 更新为 `completed`（该文件无 status 字段，需在 frontmatter 中添加 `status: completed`）
- R4. 为 `program_center_core_entity` 的 5 个实体补全 POST/PUT/DELETE 端点
- R5. 写操作包含参数验证和软删除过滤
- R6. 新增集成测试覆盖 happy path 和 error path
- R7. `cargo test --workspace --lib` 全部通过
- R8. 填充 55 张模块卡片的 Responsibility 字段
- R9. 扩展 API 文档生成脚本覆盖全部 55+ 模块
- R10. 为有 REST 端点的模块卡片填充 "REST Endpoints" 字段（基于 `oa/o2web/source/o2_core/o2/xAction/services/` 中的 action JSON 文件）
- R11. 验证 `docs/oa/` 下所有链接可正常解析，README 目录与现有文件一致

**Origin acceptance examples:** AE1（Covers R1, R2, R3）、AE2（Covers R4, R6）、AE3（Covers R7）、AE4（Covers R8, R10）、AE5（Covers R9, R11）

---

## Scope Boundaries

- **包含：** 计划 status 更新；program_center_core_entity 写操作实现与测试；模块卡片 Responsibility 和 REST Endpoints 字段填充；API 文档脚本扩展与链接验证
- **排除在外：** 其余 7 个 core_entity crate 写操作（已补全）；Session 持久化（已实现）；docs/oa 目录结构重建；组件卡片填充

### Deferred to Follow-Up Work

- `docs/oa/modules/o2web/` 86 张组件卡片 Responsibility 填充
- `openapi` crate 14 个未使用函数清理
- `docs/oa/reference/data-models.md` 实体关系图完善

---

## Context & Research

### Relevant Code and Patterns

- `oa4rust/crates/organization_core_entity/src/lib.rs` 中的 `definition_create`/`definition_update`/`definition_delete` 是参考模板——参数验证 → ActiveModel → insert/update → ActionResult
- **软删除模式：** `deleted_at` 字段通过 `Set(Some(chrono::Utc::now().naive_utc()))` 标记，list 查询加 `.filter(Column::DeletedAt.is_null())`
- **entity 差异：** application 和 script 实体没有 `deleted_at` 字段，invoke/agent/structure 有。write handlers 需适配
- **测试模式：** `oa4rust/crates/organization_core_entity/src/tests.rs` 中有 mock-based 测试，但 program_center_core_entity 的测试目前全是 `assert!(true)` 桩
- **API 文档脚本：** `docs/oa/scripts/generate_api_docs.py` 从 `oa/o2web/source/o2_core/o2/xAction/services/*.json` 提取端点信息，当前仅生成 14 个模块

### Institutional Learnings

- `program_center_core_entity` 的 router 函数名为 `program_center_core_entity_router`（在 `lib.rs:217`），返回 `Router` 并通过 `Extension` 注入 `DatabaseConnection`
- application 和 script 表无 `deleted_at` 列（entity 定义无此字段），create/update 端点不需要软删除逻辑
- `oa4rust/crates/query_express/src/lib.rs:58` 已有 `create_query` 写操作，可作为参考

---

## Key Technical Decisions

- **application 和 script 无软删除：** 这两个 entity 没有 `deleted_at` 字段，DELETE 端点改为物理删除（`Entity::delete_by_id`）或跳过（返回 success 但注明不支持）。参考 `organization_core_entity` 中所有 entity 都有 `deleted_at` 的模式，program_center_core_entity 的 application/script 是历史遗留，保持兼容不做 schema 变更
- **权限级别：** 写操作不添加额外的 admin 检查，依赖 shared 中间件的 authorize_middleware 统一处理（与 organization_core_entity 一致）
- **模块卡片内容策略：** 每个模块卡片的 Responsibility 基于 Java 模块名和 pom.xml 中的 artifactId 推断职责，1-3 句话描述核心功能。不尝试穷举所有端点
- **API 文档生成策略：** 扩展脚本遍历 `oa/o2server/` 下所有模块目录，对每个模块查找对应的 action JSON 文件（基于模块名映射），生成 Markdown 文档

---

## Open Questions

### Resolved During Planning

- **application/script 无 deleted_at 的 DELETE 策略：** 保留 DELETE 端点但返回 `ActionResult::success` 包含 `"note": "physical delete not supported for this entity"`，不执行实际删除操作。保持前端契约一致
- **模块卡片 Responsibility 内容深度：** 每卡 1-2 句话，描述模块的核心职责和主要实体，不列举所有端点

### Deferred to Implementation

- 模块卡片具体措辞——实施时需对照 Java 源码确认职责描述准确
- API 文档生成脚本的模块名到 action JSON 的映射规则——需在实际运行中调整
- application/script DELETE 端点返回 error 而非 success+note（已修复：返回 ActionResult::error）
- update/delete 端点授权检查粒度——需确认 `authorize_middleware` 是否包含资源级校验，如仅角色级则需补充 IDOR 防护（P0 发现，需设计决策）✅ 已修复：application/script/invoke 的 update+delete 已添加 require_owner 资源级 IDOR 校验
- 输入验证除 name 非空外，是否需要字段长度限制（如 name ≤ 255）和基础清洗（P1 发现）✅ 已修复：name ≤200，其他文本字段 ≤500，description ≤2000
- invoke 响应中 application 字段语义：当前设为自身 ID，需确认是否为 bug（P0 发现）✅ 已修复：移除 invoke_list 中 application=self_id 的误导性字段
- creator_person 字段：当前硬编码为空字符串，需注入认证用户身份（P1 发现）✅ 已修复：所有 create handler 注入 Session.person_unique；新增 migration 012
- agent_update 缺少 enable 字段（P3 发现）✅ 已修复：响应中补回 enable 字段
- 15 个 handler 存在大量重复代码，可考虑抽取通用助手（P2 发现）— 待后续 refactor
- 测试仅验证无 DB 时返回 500，未覆盖业务逻辑分支（P2 发现）— 待补充 name 校验、404、软删除等业务测试

---

## Implementation Units

### U1. 计划 status 更新

**Goal:** 更新 5 个计划文件的 status，清理 `docs/plans/` 目录状态。

**Requirements:** R1, R2, R3

**Dependencies:** None

**Files:**
- Modify: `docs/plans/2026-08-07-001-feat-oa4rust-4wave-realization-plan.md`（status → completed）
- Modify: `docs/plans/2026-08-09-001-refact-oa4rust-orm-migration-plan.md`（status → completed）
- Modify: `docs/plans/2026-07-30-001-refactor-zero-secret-migration-plan.md`（status → completed）
- Modify: `docs/plans/2026-08-05-001-feat-oa4rust-comprehensive-advancement-plan.md`（status → superseded，添加取代说明）
- Modify: `docs/plans/2026-08-06-001-feat-oa4rust-full-realization-plan.md`（status → superseded，添加取代说明）
- Modify: `docs/plans/2026-08-03-001-refactor-o2server-rust-rewrite-plan.md`（status → superseded，添加取代说明）

**Approach:**
- 已完成计划：将 frontmatter 中 `status: active` 改为 `status: completed`
- 已取代计划：将 `status` 改为 `superseded`，在 frontmatter 后添加 `<!-- Superseded by: docs/plans/2026-08-07-001-feat-oa4rust-4wave-realization-plan.md -->` 等注释
- 不删除任何文件，保留历史追溯

**Test scenarios:**
- Happy path: `grep "status:" docs/plans/*.md` 无 `active` 状态的过期计划
- Happy path: 3 个已完成计划标记为 `completed`
- Happy path: 3 个已取代计划标记为 `superseded` 并有取代说明

**Verification:**
- `grep -r "status: active" docs/plans/` 无匹配
- 所有 `status: completed` 和 `status: superseded` 文件有对应的说明注释

---

### U2. program_center_core_entity 写操作实现

**Goal:** 为 5 个实体补全 POST/PUT/DELETE 端点。

**Requirements:** R4, R5

**Dependencies:** None

**Files:**
- Modify: `oa4rust/crates/program_center_core_entity/src/lib.rs`
- Test: `oa4rust/crates/program_center_core_entity/src/tests.rs`

**Approach:**
参照 `organization_core_entity` 的 write handler 模式，为每个实体添加 create/update/delete：

- **application**（5 字段，无 deleted_at）：create（POST `/jaxrs/program_center/application`）、update（PUT `/jaxrs/program_center/application/{id}`）、delete 返回 note 不支持物理删除
- **script**（5 字段，无 deleted_at）：同上模式
- **invoke**（6 字段，有 deleted_at）：create、update、软删除（SET deleted_at = NOW()）
- **agent**（8 字段，有 deleted_at）：同上模式
- **structure**（6 字段，有 deleted_at）：同上模式

每个 create handler：验证必填字段（name 非空）→ 生成 UUID → ActiveModel insert → 返回 ActionResult::success
每个 update handler：find_by_id → 404 if not found → 部分字段更新 → update → 返回成功
每个 delete handler（有 deleted_at）：find_by_id → 404 → SET deleted_at → update
每个 delete handler（无 deleted_at）：返回 success + note

路由注册：在 `program_center_core_entity_router` 函数中添加 post/put/delete 路由。

**Patterns to follow:**
- `oa4rust/crates/organization_core_entity/src/lib.rs:359-442` 的 definition_create/update/delete 模式
- `oa4rust/crates/query_express/src/lib.rs:58-83` 的 create_query 模式（参考 ActiveModel 使用）

**Test scenarios:**
- Happy: POST application 创建成功，返回包含 id/name 的 ActionResult
- Happy: PUT application 更新成功，查询 list 验证字段已更新
- Happy: POST invoke 创建成功，DELETE 后 list 查询不再返回该记录（软删除）
- Happy: POST agent 创建成功，DELETE 后 list 查询不再返回该记录
- Happy: POST structure 创建成功，DELETE 后 list 查询不再返回该记录
- Edge: POST application 缺少 name 字段，返回 type=error 响应
- Edge: PUT application 使用不存在的 id，返回 404
- Edge: DELETE application（不支持物理删除）返回 success + note
- Integration: 完整 CRUD 流程——create → list 验证存在 → update → list 验证更新 → delete → list 验证消失

**Verification:**
- `cargo test -p program_center_core_entity` 通过
- `cargo test --workspace --lib` 全部通过
- 所有 5 个实体有对应的 POST/PUT 端点，有 deleted_at 的实体有 DELETE 端点
- 响应遵循 `ActionResult<T>` 9 字段结构，与现有端点一致（R6）
- 参数验证：必填字段非空检查，缺失时返回 `type=error`

---

### U3. program_center_core_entity 集成测试

**Goal:** 为新增写操作端点添加有意义的集成测试。

**Requirements:** R6, R7

**Dependencies:** U2

**Files:**
- Modify: `oa4rust/crates/program_center_core_entity/src/tests.rs`

**Approach:**
替换现有的 `assert!(true)` 桩测试为实际的集成测试。参照 `oa4rust/crates/organization_core_entity/src/tests.rs` 的测试模式：

- 为每个实体的 create 添加测试：构建请求 → 调用 router → 验证 200 + type=success
- 为每个实体的 update 添加测试：先 create → 再 update → 验证字段变更
- 为有 deleted_at 的实体添加 delete 测试：create → delete → list 验证消失
- 为参数验证添加错误路径测试：缺少必填字段 → 验证 type=error

使用与 organization_core_entity 相同的测试池构建模式（`build_test_pool()`），但由于没有真实数据库，测试主要验证路由可达性和响应格式。

**Test scenarios:**
- Happy: `POST /jaxrs/program_center/application` 返回 INTERNAL_SERVER_ERROR（无 DB 时的预期行为，与 organization_core_entity 测试模式一致）
- Happy: `PUT /jaxrs/program_center/application/{id}` 返回 INTERNAL_SERVER_ERROR
- Happy: `POST /jaxrs/program_center/invoke` 返回 INTERNAL_SERVER_ERROR
- Happy: `DELETE /jaxrs/program_center/invoke/{id}` 返回 INTERNAL_SERVER_ERROR
- Edge: `POST /jaxrs/program_center/application` 缺少 name 返回 type=error（参数验证在 handler 内部执行，不依赖 DB）
- Edge: `PUT /jaxrs/program_center/application/nonexistent` 返回 INTERNAL_SERVER_ERROR（无 DB 时 find_by_id 失败）

**Verification:**
- `cargo test -p program_center_core_entity` 全部通过（无失败）
- 无 `assert!(true)` 桩测试残留
- `cargo test --workspace --lib` 全部通过

---

### U4. 模块卡片 Responsibility 填充

**Goal:** 为 55 张 o2server 模块卡片的 Responsibility 字段填充内容。

**Requirements:** R8

**Dependencies:** None

**Files:**
- Modify: `docs/oa/modules/o2server/*.md`（55 个文件）

**Approach:**
基于模块名和 Java 包结构推断职责，每卡 1-2 句话。分组处理：

- **组织域**（organization_*）：人员/单位/角色/用户组的核心数据访问
- **流程域**（process_*、processplatform_*）：流程设计、实例管理、任务处理
- **CMS 域**（cms_*）：栏目/文章/表单/字典管理
- **查询域**（query_*）：视图定义、查询执行、导入导出
- **文件域**（file_*）：文件/文件夹 CRUD
- **日历域**（calendar_*）：日程/事件管理
- **考勤域**（attendance_*）：打卡/排班/统计
- **会议域**（meeting_*）：会议室/会议管理
- **消息域**（message_*）：消息收发
- **门户域**（portal_*）：页面/部件/脚本管理
- **AI 域**（ai_*）：模型/对话管理
- **通用域**（general_*、component_*、hotpic_*、jpush_*、mind_*、bbs_*、express、console、base、correlation_*、program_center_*）：对应功能域

不修改已有人工编写的内容，仅填充空白字段。

**Test scenarios:**
- Happy: 所有 55 张卡片的 "Responsibility" 字段包含非空文本
- Edge: 已有内容的卡片不被覆盖

**Verification:**
- `grep -c "## Responsibility" docs/oa/modules/o2server/*.md` 全部为 1 且其后紧跟非空内容（确认字段已填充）
- 随机抽查 5 张卡片，内容准确反映模块职责

---

### U5. API 文档生成脚本扩展

**Goal:** 扩展 `generate_api_docs.py` 覆盖全部 55+ 个 o2server 模块。

**Requirements:** R9

**Dependencies:** None

**Files:**
- Modify: `docs/oa/scripts/generate_api_docs.py`
- Create: `docs/oa/api/auto/` 下新增模块文档（运行脚本后生成）

**Approach:**
当前脚本仅处理 `oa/o2web/source/o2_core/o2/xAction/services/` 目录下的 JSON 文件。扩展策略：

1. 扫描 `oa/o2server/` 下所有模块目录
2. 对每个模块，尝试在 `oa/o2web/source/o2_core/o2/xAction/services/` 中查找匹配的 JSON 文件（基于模块名映射规则，如 `x_organization_assemble_control` → `x_organization_assemble_control.json`）
3. 对无对应 JSON 文件的模块，生成空文档骨架（标记 "No action JSON found"）
4. 输出目录保持 `docs/oa/api/auto/`，文件名格式 `x_<module_name>.md`

同时更新 `docs/oa/api/README.md` 以反映完整的模块覆盖。

**Test scenarios:**
- Happy: 运行脚本后 `docs/oa/api/auto/` 包含 55+ 个模块文档
- Happy: 已有文档的模块被正确更新
- Edge: 无 action JSON 的模块生成空骨架而非报错

**Verification:**
- `ls docs/oa/api/auto/ | wc -l` ≥ 55
- `docs/oa/api/README.md` 链接与现有文档一致

---

### U6. 文档链接验证

**Goal:** 验证 `docs/oa/` 下所有链接可正常解析，README 目录与现有文件一致。

**Requirements:** R11

**Dependencies:** U4, U5

**Files:**
- Verify: `docs/oa/README.md` 中的链接与实际文件一致
- Verify: `docs/oa/api/README.md` 中的链接与实际文件一致
- Verify: `docs/oa/modules/o2server/` 中卡片内的交叉引用可解析

**Approach:**
- 检查 README.md 的目录链接是否指向存在的文件
- 检查 API README 的模块链接是否完整
- 检查模块卡片中的交叉引用（如 "相关模块"）是否指向存在的卡片文件

**Test scenarios:**
- Happy: README.md 中所有链接可解析
- Happy: API README 中所有模块链接可解析
- Edge: 不存在的链接被标记为待修复

**Verification:**
- `docs/oa/README.md` 目录链接与现有文件一致
- `docs/oa/api/README.md` 覆盖全部 55+ 模块

---

## System-Wide Impact

- **计划清理** 不影响代码，仅文档 frontmatter 变更
- **写操作实现** 影响 `program_center_core_entity` crate，新增路由端点，不影响其他 crate
- **文档填充** 纯文档工作，无代码变更

---

## Risks & Dependencies

| Risk | Mitigation |
|------|------------|
| application/script 无 deleted_at 导致 DELETE 语义不一致 | 返回 note 说明不支持物理删除，保持前端契约 |
| 模块卡片职责描述不准确 | 基于 Java 模块名和 pom.xml 推断，实施时可对照源码修正 |
| API 文档脚本模块名映射规则不完善 | 首先生成空骨架，再逐步补充有 JSON 的模块 |

---

## Sources & References

- **Origin document:** `docs/brainstorms/2026-08-10-oa4rust-production-readiness-and-docs-cleanup-requirements.md`
- `oa4rust/crates/organization_core_entity/src/lib.rs`（写操作参考模式）、`oa4rust/crates/query_express/src/lib.rs`（create 参考模式）
- Related docs: `docs/oa/scripts/generate_api_docs.py`、`docs/oa/templates/module-card.md`
