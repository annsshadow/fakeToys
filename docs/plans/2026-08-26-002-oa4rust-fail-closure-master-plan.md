---
title: "oa4rust FAIL 闭环总计划"
type: refactor
status: completed
date: 2026-08-26
origin: docs/plans/2026-08-26-001-feat-parity-convergence-phase2-plan.md
---

# oa4rust FAIL 闭环总计划

## Summary

将836条 FAIL 降至≤200。三类差距（信封差异、Express POST 未实现、深层逻辑缺口）统一管理，分3条执行线并行推进，最终统一验收。预计消除~636条 FAIL。

---

## Problem Frame

Phase 2 完成后基线：1212 PASS / 836 FAIL / 1996 SKIP（4044端点）。836条 FAIL 分三类：

| 类型 | 数量 | 根因 | 消除手段 |
|------|------|------|----------|
| 信封差异 | ~620 | `success()` 硬编码 count=0, size=0 vs Java `java_success(data, count, size)` | 统一响应构造函数 |
| Express POST 未实现 | ~50 | handler 路由注册但未实现 | 实现查询逻辑 |
| 深层逻辑缺口 | ~165 | 其余 crate handler 返回 `prompt`（未实现） | 实现查询逻辑 |
| 数据不对称 | ~100 | 双方都有 `data` 但数组长度不同 | 共享种子扩展（后续） |

`success()` vs `java_success()` 的关键区别：

```rust
// success() — 当前 ~600 个 handler 使用
ActionResult::success(data)
// → { data: Some(data), count: Some(0), size: Some(0), ... }

// java_success() — Java 兼容格式
ActionResult::java_success(data, count, size)
// → { data: Some(data), count: Some(N), size: Some(M), ... }
```

---

## Requirements

- R1. 信封统一：所有列表端点使用 `java_success(data, count, size)` 而非 `success(data)`
- R2. 端点补全：Express POST list + 其余 crate 缺口端点实现对应查询逻辑
- R3. 返回格式：所有实现的端点使用 `ActionResult::java_success(data, count, size)` 信封
- R4. 全量 compare PASS 数从1212增长至≥1600（消除~400条可确定的 FAIL）
- R5. 不改变任何端点的业务语义
- R6. 不修改 comparator.rs、endpoints.rs、Java 侧代码

---

## Scope Boundaries

- **包含：** 信封统一、Express POST 端点实现、深层逻辑缺口实现
- **排除：** 数据不对称类（~100条）——需共享种子或真实数据，进后续计划
- **排除：** 上传响应信封差异（Java `{position,count,spent}` vs Rust `{status,servlet,url}`）——独立问题
- **排除：** 工作流引擎核心逻辑（processplatform_service_processing 的信号/回滚/事件）——进 backlog
- **排除：** CMS 的 `mockdeletetoget` 模式——语义待确认
- **排除：** 生产切流（ops 环境 + ≥2周观察期）

---

## Context & Research

### Relevant Code and Patterns

- `crates/shared/src/response.rs`：`ActionResult` 结构体 + `success()` + `java_success()` + `error()`
- `crates/shared/src/error.rs`：`AppError` 枚举（9变体），`IntoResponse` 实现
- `tests/behavior_comparison/comparator.rs`：比较规则（冻结不变）
- `tests/behavior_comparison/endpoints.rs`：4688条自动生成端点定义
- `tests/behavior_comparison/allowlist.yaml`：26+65条字段映射 allowlist
- `docs/audits/behavior-divergence-backlog.md`：215条深层缺口完整清单

### Institutional Learnings

- `docs/solutions/best-practices/oa4rust-o2server-parity-closure-campaign-2026-08-25.md`：包装模式战役方法论
- Phase 1 提交 `9d81b8ca`：214个 handler `{count,data}`→`java_success` 的成功经验
- `docs/solutions/architecture-patterns/actionresult-9-field-contract.md`：9字段契约定义

---

## Key Technical Decisions

- **三线并行：** 信封统一、Express POST、深层逻辑三条线可并行推进，互不冲突
- **仅改列表端点信封：** 单对象端点的 `success(data)` 与 Java 行为一致（count=0, size=0），无需改动
- **复用 Control 查询函数：** Express 的 POST list 端点本质上是 Control 模块查询的批量版本
- **分批验证：** 每个 crate 改完后跑全量 compare 验证 PASS 增量，避免累积风险
- **保留 `success()` 函数：** 单对象端点仍使用它，不删除

---

## Implementation Units

### Track A: 信封统一（~620 FAIL 消除）

#### A1. 核心组织模块——organization_assemble_control + organization_assemble_express（信封部分）

**Goal:** 统一组织模块的列表端点信封

**Requirements:** R1, R4

**Dependencies:** 无

**Files:**
- Modify: `crates/organization_assemble_control/src/lib.rs`（~23处 `success`→`java_success`）
- Modify: `crates/organization_assemble_express/src/lib.rs`（已实现的 GET handler ~50处）
- Test: 全量 compare 重跑

**Approach:**
- grep `Ok(Json(ActionResult::success(` 定位所有列表端点
- 对每个返回 `Value::Array(...)` 或 `Vec<...>` 的 handler，改为 `ActionResult::java_success(data, count, size)`
- count 从查询总数获取（已有 `count` 变量或 `len()`），size 从实际返回数量获取
- Express 的 POST list 端点（~50条未实现）不在本 unit 范围——由 Track B 处理

**Test scenarios:**
- Happy path: 某列表端点改后从 FAIL→PASS
- Edge case: 空列表返回 `java_success([], 0, 0)` 与 Java 空数组行为一致
- Regression: 改后不引入新的 FAIL

**Verification:**
- 全量 compare PASS 数增长 ≥20
- 受影响 crate 既有单元测试全绿

---

#### A2. CMS 模块——cms_assemble_control（信封部分）

**Goal:** 统一 CMS 模块的列表端点信封

**Requirements:** R1, R4

**Dependencies:** 无

**Files:**
- Modify: `crates/cms_assemble_control/src/lib.rs`（~58处）
- Test: 全量 compare 重跑

**Approach:**
- CMS 模块几乎全是 Pattern A（Rust→`data`, Java→`prompt`），说明 CMS handler 统一使用了 `success()` 而 Java 用 `prompt` 包装
- 需要逐个确认：哪些是列表端点（改 `java_success`），哪些是单对象端点（保持 `success`），哪些是错误端点（保持 `error`）
- CMS 的 `mockdeletetoget` 模式需特殊处理——标记为 backlog

**Test scenarios:**
- Happy path: CMS 列表端点改后 PASS
- Edge case: CMS 的 mock 操作端点确认是否为信封差异还是语义差异
- Regression: CMS 既有测试全绿

**Verification:**
- 全量 compare PASS 数增长 ≥30
- CMS crate 单元测试全绿

---

#### A3. 流程平台模块——processplatform（信封部分）

**Goal:** 统一流程平台的列表端点信封

**Requirements:** R1, R4

**Dependencies:** 无

**Files:**
- Modify: `crates/processplatform_assemble_surface/src/lib.rs`（~70处）
- Modify: `crates/processplatform_assemble_designer/src/lib.rs`（~12处）
- Test: 全量 compare 重跑

**Approach:**
- processplatform 是最大的信封差异 crate（70条），主要集中在 surface 的工作流查询端点
- 该模块 handler 密度高（461处 `success`），需要仔细区分列表 vs 单对象端点
- 附件端点的上传响应信封差异（45条）不在本 unit 范围

**Test scenarios:**
- Happy path: 流程平台列表端点改后 PASS
- Edge case: 工作流引擎相关端点确认信封差异 vs 语义差异
- Regression: processplatform 单元测试全绿

**Verification:**
- 全量 compare PASS 数增长 ≥40
- processplatform crate 测试全绿

---

#### A4. 查询/考勤/消息模块（信封部分）

**Goal:** 统一 query、attendance、message 模块的列表端点信封

**Requirements:** R1, R4

**Dependencies:** 无

**Files:**
- Modify: `crates/query_assemble_designer/src/lib.rs`（~13处）
- Modify: `crates/query_assemble_surface/src/lib.rs`（~12处）
- Modify: `crates/attendance_assemble_control/src/lib.rs`（~20处）
- Modify: `crates/message_assemble_communicate/src/lib.rs`（~9处）
- Test: 全量 compare 重跑

**Approach:**
- 四个 crate 合计44条，每个 crate 独立改、独立测
- query 模块的 `search` 端点需确认返回格式（可能是分页结果）
- attendance 模块的 `analyse` 端点可能返回复杂结构，需逐个确认

**Test scenarios:**
- Happy path: 各模块列表端点改后 PASS
- Edge case: 分页端点的 count/size 取值
- Regression: 各模块单元测试全绿

**Verification:**
- 全量 compare PASS 数增长 ≥20
- 各模块单元测试全绿

---

#### A5. 其余小模块（信封部分）

**Goal:** 统一剩余模块的列表端点信封

**Requirements:** R1, R4

**Dependencies:** 无

**Files:**
- Modify: 各模块的 `src/lib.rs`（每个 ~3-20处）
- Test: 全量 compare 重跑

**Approach:**
- 小模块批量处理，每个模块改完即测
- `program_center` 有20条差异，需优先处理
- `portal` 8条，`meeting` 5条，`general` 5条，`file` 8条
- `ai`、`personal`、`mind`、`calendar`、`auth` 各 2-4 条

**Test scenarios:**
- Happy path: 各模块列表端点改后 PASS
- Regression: 各模块单元测试全绿

**Verification:**
- 全量 compare PASS 数增长 ≥30
- 所有受影响模块测试全绿

---

### Track B: Express POST 端点补全（~50 FAIL 消除）

#### B1. 人员查询端点（~20条）

**Goal:** 实现所有按人员维度查询的 POST list 端点

**Requirements:** R2, R3

**Dependencies:** 无

**Files:**
- Modify: `crates/organization_assemble_express/src/lib.rs`
- Test: `crates/organization_assemble_express/src/tests.rs`

**Approach:**
- 端点清单（按 body 参数分组）：
  - `POST /jaxrs/person/list` — 全量人员列表
  - `POST /jaxrs/person/list/group` — 按组查人员
  - `POST /jaxrs/person/list/identity` — 按身份查人员
  - `POST /jaxrs/person/list/role` — 按角色查人员
  - `POST /jaxrs/person/list/unit/sub/direct` — 直属下级单位人员
  - `POST /jaxrs/person/list/unit/sub/nested` — 嵌套下级单位人员
  - `POST /jaxrs/person/list/person/sub/direct` — 直属下级人员
  - `POST /jaxrs/person/list/person/sub/nested` — 嵌套下级人员
  - `POST /jaxrs/person/list/person/sup/direct` — 直属上级人员
  - `POST /jaxrs/person/list/person/sup/nested` — 嵌套上级人员
  - `POST /jaxrs/person/list/login/after` — 登录后人员
  - `POST /jaxrs/person/list/login/recent` — 近期登录人员
  - `POST /jaxrs/person/list/pair/identity` — 身份配对列表
  - `POST /jaxrs/person/list/group/object` — 按组查人员（对象格式）
  - `POST /jaxrs/person/list/identity/object` — 按身份查人员（对象格式）
  - `POST /jaxrs/person/list/unit/sub/direct/like` — 模糊搜索直属下级
  - `POST /jaxrs/person/list/unit/sub/nested/like` — 模糊搜索嵌套下级
  - `POST /jaxrs/person/detail/{flag}` — 人员详情
- 每个端点：解析 POST body → 调用 Control 查询函数 → 返回 `java_success`
- 用宏或辅助函数减少重复代码

**Test scenarios:**
- Happy path: 按组查询返回正确人员列表
- Edge case: 空查询条件返回空列表
- Edge case: 不存在的组/身份返回空列表（不报错）
- Integration: 与 Control 模块查询结果一致

**Verification:**
- 所有20个端点从 FAIL→PASS
- 单元测试覆盖每个端点

---

#### B2. 单位查询端点（~15条）

**Goal:** 实现所有按单位维度查询的 POST list 端点

**Requirements:** R2, R3

**Dependencies:** 无

**Files:**
- Modify: `crates/organization_assemble_express/src/lib.rs`
- Test: `crates/organization_assemble_express/src/tests.rs`

**Approach:**
- 端点清单：
  - `POST /jaxrs/unit/list/identity` — 按身份查单位
  - `POST /jaxrs/unit/list/identity/sup/nested` — 嵌套上级身份单位
  - `POST /jaxrs/unit/list/level` — 按层级查单位
  - `POST /jaxrs/unit/list/person` — 按人员查单位
  - `POST /jaxrs/unit/list/person/sup/nested` — 嵌套上级人员单位
  - `POST /jaxrs/unit/list/types` — 单位类型列表
  - `POST /jaxrs/unit/list/unitduty` — 单位职责列表
  - `POST /jaxrs/unit/identity/level` — 身份层级列表
  - `POST /jaxrs/unit/identity/type` — 身份类型列表
  - `POST /jaxrs/unit/check/unit/has/identity` — 单位身份检查
  - `POST /jaxrs/unit/check/unit/has/person` — 单位人员检查
  - `POST /jaxrs/unit/check/unit/has/unit` — 单位隶属检查
- 检查端点返回布尔值，需确认 Java 返回格式

**Test scenarios:**
- Happy path: 按层级查询返回正确单位列表
- Edge case: 空层级返回空列表
- Integration: 与 Control 模块查询结果一致

**Verification:**
- 所有15个端点从 FAIL→PASS

---

#### B3. 组/角色/辅助查询端点（~15条）

**Goal:** 实现组列表、角色列表、属性操作等辅助端点

**Requirements:** R2, R3

**Dependencies:** 无

**Files:**
- Modify: `crates/organization_assemble_express/src/lib.rs`
- Test: `crates/organization_assemble_express/src/tests.rs`

**Approach:**
- 端点清单：
  - `POST /jaxrs/group/list` — 全量组列表
  - `POST /jaxrs/group/list/group/sub/direct` — 直属下级组
  - `POST /jaxrs/group/list/group/sub/nested` — 嵌套下级组
  - `POST /jaxrs/group/list/group/sup/direct` — 直属上级组
  - `POST /jaxrs/group/list/group/sup/nested` — 嵌套上级组
  - `POST /jaxrs/group/list/identity` — 按身份查组
  - `POST /jaxrs/group/list/person` — 按人员查组
  - `POST /jaxrs/group/has/role` — 角色存在检查
  - `POST /jaxrs/person/has/role` — 角色存在检查
  - `POST /jaxrs/role/list` — 全量角色列表
  - `POST /jaxrs/role/list/person` — 按人员查角色
  - `POST /jaxrs/personattribute/append/person/name` — 追加人员属性
  - `POST /jaxrs/personattribute/set/person/name` — 设置人员属性
  - `POST /jaxrs/unitattribute/append/unit/name` — 追加单位属性
  - `POST /jaxrs/unitattribute/set/unit/name` — 设置单位属性

**Test scenarios:**
- Happy path: 组列表返回正确数据
- Edge case: 属性操作的幂等性
- Integration: 与 Control 模块查询结果一致

**Verification:**
- 所有15个端点从 FAIL→PASS

---

### Track C: 深层逻辑缺口补全（~165 FAIL 消除）

#### C1. organization_assemble_control——组织树层级查询（13条）

**Goal:** 实现组织树的层级查询端点

**Requirements:** R2, R3

**Dependencies:** 无

**Files:**
- Modify: `crates/organization_assemble_control/src/lib.rs`
- Test: `crates/organization_assemble_control/src/tests.rs`

**Approach:**
- 端点清单：
  - `GET .../group/list/{flag}/sub/direct` — 直属下级组
  - `GET .../group/list/{flag}/sub/nested` — 嵌套下级组
  - `GET .../group/list/{flag}/sup/direct` — 直属上级组
  - `GET .../group/list/{flag}/sup/nested` — 嵌套上级组
  - `GET .../group/list/person/{personFlag}/sup/direct` — 人员直属上级组
  - `GET .../group/list/person/{personFlag}/sup/nested` — 人员嵌套上级组
  - `GET .../group/list/role/{roleFlag}` — 按角色查组
  - `GET .../identity/list/person/{personFlag}` — 按人员查身份
  - `GET .../identity/list/unit/{unitFlag}` — 按单位查身份
  - `GET .../person/check/password/{password}` — 密码检查
  - `GET .../personattribute/list/person/{personFlag}` — 人员属性
  - `GET .../personcard/listPersonalVCf/{idList}` — 个人名片VCF
  - `GET .../personcard/listVCf/{idList}` — 名片VCF
  - `GET .../role/list/group/{groupFlag}` — 按组查角色
- 组织树查询需递归遍历 `org_unit` 表的 `superior` 字段
- 复用现有 `org_unit` 查询函数，扩展层级遍历逻辑

**Test scenarios:**
- Happy path: 查询直属下级返回正确组列表
- Edge case: 叶子节点（无下级）返回空列表
- Edge case: 循环引用（superior 链成环）的安全处理
- Integration: 查询结果与数据库实际层级一致

**Verification:**
- 13个端点从 FAIL→PASS

---

#### C2. cms_assemble_control——CMS 核心功能（13条）

**Goal:** 实现 CMS 的权限查询、文件操作、关联查询等端点

**Requirements:** R2, R3

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

#### C3. processplatform_assemble_surface——工作流前端（13条）

**Goal:** 实现流程平台前端的附件/字典/文件/流程列表查询

**Requirements:** R2, R3

**Dependencies:** 无

**Files:**
- Modify: `crates/processplatform_assemble_surface/src/lib.rs`
- Test: `crates/processplatform_assemble_surface/src/tests.rs`

**Approach:**
- 端点清单：
  - `DELETE /jaxrs/processplatform/assemble/surface/data/work/{id}` — 删除工作数据
  - `GET .../applicationdict/list/application/{applicationFlag}` — 应用字典列表
  - `GET .../attachment/list/work/{workId}` — 工作附件
  - `GET .../attachment/list/workcompleted/{workCompletedId}` — 已完成工作附件
  - `GET .../data/workcompleted/{id}` — 已完成工作数据
  - `GET .../data/workcompleted/{id}/from/data` — 从数据获取
  - `GET .../data/workcompleted/{id}/from/item` — 从条目获取
  - `GET .../file/list/application/{applicationFlag}` — 应用文件列表
  - `GET .../process/list/application/{applicationFlag}` — 应用流程列表
  - `GET .../process/list/available/identity/process/{flag}` — 可用身份流程
  - `GET .../process/list/controllable/application/{applicationFlag}` — 可控流程
  - `GET .../serialnumber/list/application/{applicationFlag}` — 流水号列表
  - `GET .../taskcompleted/list/prev/manual/{flag}` — 前一个手动已完成任务
- 附件/文件列表查询相对简单（查表返回数组）
- 工作流数据查询需关联多个表

**Test scenarios:**
- Happy path: 附件列表返回正确数据
- Edge case: 无附件的工作返回空列表
- Integration: 查询结果与数据库一致

**Verification:**
- 13个端点从 FAIL→PASS

---

#### C4. query 模块——查询设计器+前端（18条）

**Goal:** 实现查询设计器和前端的视图/语句/表/搜索端点

**Requirements:** R2, R3

**Dependencies:** 无

**Files:**
- Modify: `crates/query_assemble_designer/src/lib.rs`
- Modify: `crates/query_assemble_surface/src/lib.rs`
- Test: 各模块 tests.rs

**Approach:**
- query_assemble_designer（10条）：
  - `DELETE .../table/{flag}/row/delete/all` — 删除所有行
  - `GET .../importmodel/list/query/{flag}` — 导入模型列表
  - `GET .../stat/list/query/{queryFlag}` — 统计列表
  - `GET .../table/list/query/{flag}` — 表列表
  - `GET .../table/{flag}/build/dispatch` — 表构建调度
  - `GET .../view/list/query/{queryFlag}` — 视图列表
  - `POST .../search` — 搜索
  - `POST .../statement/list/query/{queryFlag}` — SQL语句列表
  - `POST .../table/{flag}/row` — 表行数据
  - `POST .../table/{flag}/row/save` — 保存行数据
- query_assemble_surface（8条）：类似结构的前端查询端点

**Test scenarios:**
- Happy path: 视图列表返回正确数据
- Edge case: 空查询返回空列表
- Integration: 设计器与前端查询结果一致

**Verification:**
- 18个端点从 FAIL→PASS

---

#### C5. 其余模块（~45条）

**Goal:** 实现剩余模块的分散缺口端点

**Requirements:** R2, R3

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

### Final: 全量验证与报告

#### V1. 全量验证

**Goal:** 全量 compare 重跑，记录 PASS/FAIL/SKIP 前后对照，更新终扫文档

**Requirements:** R4

**Dependencies:** A1-A5, B1-B3, C1-C5

**Files:**
- Modify: `docs/audits/final-coverage-sweep.md`（§六追加闭环小节）
- Test: `cargo test --test behavior_compare` 全量

**Approach:**
- 启动 Rust 服务 + Java 服务，应用种子，跑全量 compare
- 记录 PASS/FAIL/SKIP 数字，与基线（1212/836/1996）对比
- 生成新的 behavior-report.md
- 更新终扫文档，标注各 Track 的贡献

**Test scenarios:**
- Test expectation: none — 纯验证与文档更新

**Verification:**
- PASS ≥1600（较基线 +32%）
- FAIL ≤250（较基线 -70%）
- 信封差异类 FAIL（`data`/`prompt` 互换）数量 ≤50

---

## 依赖关系图

```
Track A (信封统一)          Track B (Express POST)      Track C (深层逻辑)
  A1 ─┐                      B1 ─┐                      C1 ─┐
  A2 ─┤                      B2 ─┤                      C2 ─┤
  A3 ─┤ 可并行               B3 ─┤ 可并行               C3 ─┤ 可并行
  A4 ─┤                      ─────                      C4 ─┤
  A5 ─┘                                                 C5 ─┘
       │                          │                          │
       └──────────────┬───────────┘──────────────────────────┘
                      │
                      ▼
                   V1 (全量验证)
```

---

## System-Wide Impact

- **Interaction graph:** 信封形状变更影响所有消费 Rust API 的客户端（前端、测试脚本、影子流量脚本）
- **Error propagation:** 错误端点不变，仅成功端点的信封字段值变化
- **State lifecycle risks:** 无状态变更，纯响应格式调整 + 查询端点新增
- **API surface parity:** 信封统一后，Rust API 与 Java API 在 JSON 结构层面完全对齐
- **Unchanged invariants:** 业务逻辑不变（已有端点）、路由不变、数据库 schema 不变

---

## Risks & Dependencies

| Risk | Mitigation |
|------|------------|
| 改动 handler 数量大（~600处信封 + ~215处新实现） | 分 Track 分 Unit，每批测后确认 |
| 部分端点 count 取值不确定 | 从查询总数获取，无总数的用 `len()` |
| Express POST body 格式不确定 | 从 Java 源码推断或 curl 验证 |
| 组织树递归查询性能 | 限制递归深度或用 CTE |
| CMS mock 操作模式语义不明 | 标记为 backlog，不实现 |
| 工作流引擎核心逻辑复杂 | 仅实现简单查询，复杂逻辑进 backlog |
| 信封统一后前端可能受影响 | 信封字段是超集，向后兼容 |

---

## Sources & References

- Phase 2 plan: `docs/plans/2026-08-26-001-feat-parity-convergence-phase2-plan.md`
- 包装模式战役: `docs/solutions/best-practices/oa4rust-o2server-parity-closure-campaign-2026-08-25.md`
- ActionResult 契约: `docs/solutions/architecture-patterns/actionresult-9-field-contract.md`
- 深层缺口清单: `docs/audits/behavior-divergence-backlog.md`
- 信封分析数据: `oa4rust/target/debug/behavior-report.md`（2026-08-26 运行）
