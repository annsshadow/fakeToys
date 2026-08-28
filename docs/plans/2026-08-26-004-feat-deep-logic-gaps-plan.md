---
title: "feat: 深层逻辑缺口补全（非 Express 模块）"
type: feat
status: superseded
date: 2026-08-26
origin: docs/plans/2026-08-26-001-feat-parity-convergence-phase2-plan.md
superseded_by: docs/plans/2026-08-26-002-oa4rust-fail-closure-master-plan.md
---

# 深层逻辑缺口补全（非 Express 模块）

## Summary

补全除 `organization_assemble_express` 外的所有 crate 的深层逻辑缺口（~165个端点）。这些端点在 Rust 侧返回 `prompt`（未实现），Java 侧返回 `data`（成功）。按 crate 分批实现，覆盖 CMS、流程平台、查询、考勤、消息、日历、文件、门户、AI、个人中心、脑图等模块。

---

## Problem Frame

Phase 2 U6 backlog 分析识别了215个深层逻辑缺口，其中~50个在 Express 模块（由 Plan 003 处理），剩余~165个分布在15个 crate 中。这些端点的共同特征：Rust 路由已注册但 handler 返回错误（`AppError::NotImplemented` 或类似），而 Java 有完整实现。

按 crate 分组的缺口数：

| Crate | 缺口数 | 核心缺失 |
|-------|--------|---------|
| cms_assemble_control | 13 | 权限查询、文件下载、mock 操作 |
| processplatform_assemble_surface | 13 | 工作流附件/意见/日志 |
| organization_assemble_control | 13 | 组织树层级查询 |
| query_assemble_designer | 10 | 查询语句/视图/表定义 |
| program_center | 9 | 应用打包/认证/配置 |
| query_assemble_surface | 8 | 查询前端视图/语句 |
| message_assemble_communicate | 7 | IM 会话/消息操作 |
| processplatform_service_processing | 5 | 工作流处理信号/回滚 |
| attendance_assemble_control | 5 | 考勤分析查询 |
| calendar_assemble_control | 6 | 日历关注/管理员/事件 |
| file_assemble_control | 5 | 文件引用查询/上传 |
| portal_assemble_surface | 3 | 门户字典/文件/页面 |
| portal_assemble_designer | 5 | 门户设计器 |
| meeting_assemble_control | 3 | 会议确认操作 |
| general_assemble_control | 4 | 地区/发票/通用文件 |
| 其他（ai/personal/mind） | 8 | 分散功能 |

---

## Requirements

- R1. 每个 crate 的缺口端点实现对应的查询/操作逻辑
- R2. 返回格式统一使用 `ActionResult::java_success(data, count, size)`
- R3. 每个 crate 改完后既有单元测试全绿
- R4. 全量 compare PASS 数增长 ≥80（对应~165条缺口中的可转化部分）
- R5. 不实现工作流引擎深层逻辑（processplatform_service_processing 的复杂端点进 backlog）

---

## Scope Boundaries

- 不实现工作流引擎核心逻辑（processplatform_service_processing 的信号/回滚/事件端点）——仅进 backlog
- 不实现 CMS 的 mock 操作模式（`mockdeletetoget`）——需确认语义
- 不修改 Java 侧的任何代码
- 不修改比较器规则

---

## Implementation Units

### U1. organization_assemble_control——组织树层级查询（13条）

**Goal:** 实现组织树的层级查询端点（直属下级、嵌套下级、上级、角色关联）

**Requirements:** R1, R2, R3

**Dependencies:** 无

**Files:**
- Modify: `crates/organization_assemble_control/src/lib.rs`
- Test: `crates/organization_assemble_control/src/tests.rs`

**Approach:**
- 端点清单：
  - `GET /jaxrs/organization/assemble/control/group/list/{flag}/sub/direct` — 直属下级组
  - `GET /jaxrs/organization/assemble/control/group/list/{flag}/sub/nested` — 嵌套下级组
  - `GET /jaxrs/organization/assemble/control/group/list/{flag}/sup/direct` — 直属上级组
  - `GET /jaxrs/organization/assemble/control/group/list/{flag}/sup/nested` — 嵌套上级组
  - `GET /jaxrs/organization/assemble/control/group/list/person/{personFlag}/sup/direct` — 人员直属上级组
  - `GET /jaxrs/organization/assemble/control/group/list/person/{personFlag}/sup/nested` — 人员嵌套上级组
  - `GET /jaxrs/organization/assemble/control/group/list/role/{roleFlag}` — 按角色查组
  - `GET /jaxrs/organization/assemble/control/identity/list/person/{personFlag}` — 按人员查身份
  - `GET /jaxrs/organization/assemble/control/identity/list/unit/{unitFlag}` — 按单位查身份
  - `GET /jaxrs/organization/assemble/control/person/check/password/{password}` — 密码检查
  - `GET /jaxrs/organization/assemble/control/personattribute/list/person/{personFlag}` — 人员属性
  - `GET /jaxrs/organization/assemble/control/personcard/listPersonalVCf/{idList}` — 个人名片VCF
  - `GET /jaxrs/organization/assemble/control/personcard/listVCf/{idList}` — 名片VCF
  - `GET /jaxrs/organization/assemble/control/role/list/group/{groupFlag}` — 按组查角色
- 组织树查询需要递归遍历 `org_unit` 表的 `superior` 字段
- 复用现有 `org_unit` 查询函数，扩展层级遍历逻辑

**Test scenarios:**
- Happy path: 查询直属下级返回正确组列表
- Edge case: 叶子节点（无下级）返回空列表
- Edge case: 循环引用（superior 链成环）的安全处理
- Integration: 查询结果与数据库实际层级一致

**Verification:**
- 13个端点从 FAIL→PASS
- 组织树查询的单元测试覆盖

---

### U2. cms_assemble_control——CMS 核心功能（13条）

**Goal:** 实现 CMS 的权限查询、文件操作、关联查询等端点

**Requirements:** R1, R2, R3

**Dependencies:** 无

**Files:**
- Modify: `crates/cms_assemble_control/src/lib.rs`
- Test: `crates/cms_assemble_control/src/tests.rs`

**Approach:**
- 端点清单：
  - `DELETE /jaxrs/categoryinfo/erase/category/{id}` — 分类物理删除
  - `DELETE /jaxrs/document/batch/{id}` — 批量删除文档
  - `GET /jaxrs/categoryinfo/list/manage/app/{appId}` — 应用下分类管理列表
  - `GET /jaxrs/correlation/list/doc/{docId}` — 文档关联列表
  - `GET /jaxrs/correlation/list/doc/{docId}/site/{site}` — 文档站点关联
  - `GET /jaxrs/design/appdict/list/appInfo/{appId}` — 应用字典设计列表
  - `GET /jaxrs/file/list/appInfo/{appInfoFlag}` — 应用文件列表
  - `GET /jaxrs/fileinfo/batch/download/doc/{docId}/site/{site}` — 批量下载
  - `GET /jaxrs/fileinfo/download/document/{id}` — 文件下载
  - `GET /jaxrs/fileinfo/download/document/{id}/stream` — 文件流下载
  - `GET /jaxrs/form/list/formfield/appInfo/{appId}` — 应用表单字段列表
  - `GET /jaxrs/form/list/{id}/formfield` — 表单字段列表
  - `POST /jaxrs/file` — 文件上传
- 权限查询需关联 `x_cms_appinfo` + 权限表
- 文件下载需实现 StreamingBody 响应

**Test scenarios:**
- Happy path: 分类管理列表返回正确数据
- Edge case: 不存在的 appId 返回空列表
- Integration: 权限查询结果与数据库一致

**Verification:**
- 13个端点从 FAIL→PASS

---

### U3. processplatform_assemble_surface——工作流前端（13条）

**Goal:** 实现流程平台前端的附件/字典/文件/流程列表查询

**Requirements:** R1, R2, R3

**Dependencies:** 无

**Files:**
- Modify: `crates/processplatform_assemble_surface/src/lib.rs`
- Test: `crates/processplatform_assemble_surface/src/tests.rs`

**Approach:**
- 端点清单：
  - `DELETE /jaxrs/processplatform/assemble/surface/data/work/{id}` — 删除工作数据
  - `GET /jaxrs/processplatform/assemble/surface/applicationdict/list/application/{applicationFlag}` — 应用字典列表
  - `GET /jaxrs/processplatform/assemble/surface/attachment/list/work/{workId}` — 工作附件
  - `GET /jaxrs/processplatform/assemble/surface/attachment/list/workcompleted/{workCompletedId}` — 已完成工作附件
  - `GET /jaxrs/processplatform/assemble/surface/data/workcompleted/{id}` — 已完成工作数据
  - `GET /jaxrs/processplatform/assemble/surface/data/workcompleted/{id}/from/data` — 从数据获取
  - `GET /jaxrs/processplatform/assemble/surface/data/workcompleted/{id}/from/item` — 从条目获取
  - `GET /jaxrs/processplatform/assemble/surface/file/list/application/{applicationFlag}` — 应用文件列表
  - `GET /jaxrs/processplatform/assemble/surface/process/list/application/{applicationFlag}` — 应用流程列表
  - `GET /jaxrs/processplatform/assemble/surface/process/list/available/identity/process/{flag}` — 可用身份流程
  - `GET /jaxrs/processplatform/assemble/surface/process/list/controllable/application/{applicationFlag}` — 可控流程
  - `GET /jaxrs/processplatform/assemble/surface/serialnumber/list/application/{applicationFlag}` — 流水号列表
  - `GET /jaxrs/processplatform/assemble/surface/taskcompleted/list/prev/manual/{flag}` — 前一个手动已完成任务
- 附件/文件列表查询相对简单（查表返回数组）
- 工作流数据查询需要关联多个表

**Test scenarios:**
- Happy path: 附件列表返回正确数据
- Edge case: 无附件的工作返回空列表
- Integration: 查询结果与数据库一致

**Verification:**
- 13个端点从 FAIL→PASS

---

### U4. query 模块——查询设计器+前端（18条）

**Goal:** 实现查询设计器和前端的视图/语句/表/搜索端点

**Requirements:** R1, R2, R3

**Dependencies:** 无

**Files:**
- Modify: `crates/query_assemble_designer/src/lib.rs`
- Modify: `crates/query_assemble_surface/src/lib.rs`
- Test: 各模块 tests.rs

**Approach:**
- query_assemble_designer（10条）：
  - `DELETE /jaxrs/query/assemble/designer/table/{flag}/row/delete/all` — 删除所有行
  - `GET /jaxrs/query/assemble/designer/importmodel/list/query/{flag}` — 导入模型列表
  - `GET /jaxrs/query/assemble/designer/stat/list/query/{queryFlag}` — 统计列表
  - `GET /jaxrs/query/assemble/designer/table/list/query/{flag}` — 表列表
  - `GET /jaxrs/query/assemble/designer/table/{flag}/build/dispatch` — 表构建调度
  - `GET /jaxrs/query/assemble/designer/view/list/query/{queryFlag}` — 视图列表
  - `POST /jaxrs/query/assemble/designer/search` — 搜索
  - `POST /jaxrs/query/assemble/designer/statement/list/query/{queryFlag}` — SQL语句列表
  - `POST /jaxrs/query/assemble/designer/table/{flag}/row` — 表行数据
  - `POST /jaxrs/query/assemble/designer/table/{flag}/row/save` — 保存行数据
- query_assemble_surface（8条）：类似结构的前端查询端点

**Test scenarios:**
- Happy path: 视图列表返回正确数据
- Edge case: 空查询返回空列表
- Integration: 设计器与前端查询结果一致

**Verification:**
- 18个端点从 FAIL→PASS

---

### U5. 其余模块（program_center / message / calendar / file / portal / meeting / general / ai / personal / mind）（~45条）

**Goal:** 实现剩余模块的分散缺口端点

**Requirements:** R1, R2, R3

**Dependencies:** 无

**Files:**
- Modify: 各模块的 `src/lib.rs`
- Test: 各模块 tests.rs

**Approach:**
- program_center（9条）：应用打包/认证/配置查询
- message_assemble_communicate（7条）：IM 会话/消息操作
- calendar_assemble_control（6条）：日历关注/管理员/事件
- file_assemble_control（5条）：文件引用查询/上传
- portal_assemble_surface（3条）+ portal_assemble_designer（5条）：门户功能
- meeting_assemble_control（3条）：会议确认操作
- general_assemble_control（4条）：地区/发票/通用文件
- ai（2条）：AI 文件下载/索引删除
- personal（4条）：授权/邮件/注册检查
- mind_assemble_control（2条）：脑图保存
- processplatform_service_processing（5条）：工作流处理（标记为 backlog，仅实现简单的3条）

**Test scenarios:**
- Happy path: 各端点返回正确数据
- Regression: 各模块单元测试全绿

**Verification:**
- 可实现的~40个端点从 FAIL→PASS

---

### U6. 全量验证与报告

**Goal:** 全量 compare 重跑，记录增量，更新终扫文档

**Requirements:** R4

**Dependencies:** U1-U5

**Files:**
- Modify: `docs/audits/final-coverage-sweep.md`
- Test: `cargo test --test behavior_compare` 全量

**Verification:**
- PASS 数增长 ≥80（较基线1212）
- 各 crate 的 FAIL 数对应下降

---

## Risks & Dependencies

| Risk | Mitigation |
|------|------------|
| 部分端点的 Java 语义不确定 | 从 Java 源码推断或 curl 验证 |
| 组织树递归查询性能 | 限制递归深度或用 CTE |
| CMS mock 操作模式语义不明 | 标记为 backlog，不实现 |
| 工作流引擎核心逻辑复杂 | 仅实现简单查询，复杂逻辑进 backlog |

---

## Sources & References

- `docs/audits/behavior-divergence-backlog.md`（完整端点清单）
- `oa4rust/target/debug/behavior-report.md`（FAIL 证据）
- 各 crate 的 `src/lib.rs`（现有 handler 实现）
