---
title: "feat: Express 模块 POST list 端点补全"
type: feat
status: superseded
date: 2026-08-26
origin: docs/plans/2026-08-26-001-feat-parity-convergence-phase2-plan.md
superseded_by: docs/plans/2026-08-26-002-oa4rust-fail-closure-master-plan.md
---

# Express 模块 POST list 端点补全

## Summary

补全 `organization_assemble_express` crate 中约50个未实现的 POST list 端点。这些端点在 Rust 侧返回 `prompt`（错误/未实现），Java 侧返回 `data`（成功）。补全后可消除约50条 FAIL（全部属于"深层逻辑缺口"类）。

---

## Problem Frame

`organization_assemble_express` 是组织模块的"快捷查询"层，提供 POST 方式的批量/复杂查询接口。当前该 crate 的 POST list 端点（~50条）全部未实现——Rust 路由注册了但 handler 返回错误或空响应。

这些端点主要分三类：
1. **人员查询**（~20条）：按组/身份/角色/单位/登录状态查人员列表
2. **单位查询**（~15条）：按身份/层级/人员/类型查单位列表
3. **辅助查询**（~15条）：组列表、角色列表、属性操作、检查接口

---

## Requirements

- R1. 所有50个 POST list 端点返回与 Java 一致的 `java_success(data, count, size)` 信封
- R2. 查询逻辑复用 `organization_assemble_control` crate 的现有查询函数
- R3. 每个端点有对应的单元测试
- R4. 改后全量 compare PASS 数增长 ≥40

---

## Scope Boundaries

- 不修改 `organization_assemble_control` crate 的现有查询函数
- 不实现 `personattribute` / `unitattribute` 的写操作（append/set）——仅实现查询端点
- 不处理 `unit/check/*` 检查端点——返回布尔值，信封差异需单独分析

---

## Context & Research

### Relevant Code and Patterns

- `crates/organization_assemble_express/src/lib.rs`：Express 模块路由注册
- `crates/organization_assemble_control/src/lib.rs`：Control 模块的查询函数（复用目标）
- `crates/shared/src/pagination.rs`：分页辅助函数 `page_result()`
- Java 侧对应：`x_organization_assemble_express` war 的 Action 类

### Institutional Learnings

- Express 模块是 Control 模块的"快捷层"，查询逻辑应复用 Control 的实现
- Phase 1 已处理 Control 模块的类似端点，Express 参照同样模式

---

## Key Technical Decisions

- **复用 Control 查询函数**：Express 的 POST list 端点本质上是 Control 模块查询的批量版本，直接调用 Control 的查询函数
- **POST body 解析**：Java 的 POST list 端点接收 JSON body（如 `{"unitNameList":["a","b"]}`），Rust 侧需定义对应的 `Deserialize` 结构体
- **返回格式**：统一使用 `ActionResult::java_success(Value::Array(data), count, size)`

---

## Implementation Units

### U1. 人员查询端点（~20条）

**Goal:** 实现所有按人员维度查询的 POST list 端点

**Requirements:** R1, R2, R3

**Dependencies:** 无

**Files:**
- Modify: `crates/organization_assemble_express/src/lib.rs`
- Create: `crates/organization_assemble_express/src/person_queries.rs`（可选，按复杂度决定是否提取）
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

### U2. 单位查询端点（~15条）

**Goal:** 实现所有按单位维度查询的 POST list 端点

**Requirements:** R1, R2, R3

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

### U3. 组/角色/辅助查询端点（~15条）

**Goal:** 实现组列表、角色列表、属性操作等辅助端点

**Requirements:** R1, R2, R3

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

### U4. 全量验证

**Goal:** 全量 compare 重跑，验证 PASS 增量

**Requirements:** R4

**Dependencies:** U1-U3

**Files:**
- Test: `cargo test --test behavior_compare` 全量

**Verification:**
- PASS 数增长 ≥40
- Express crate 的 FAIL 数从 ~50 降至 ≤5

---

## Risks & Dependencies

| Risk | Mitigation |
|------|------------|
| POST body 格式不确定 | 从 Java 源码推断或 curl 验证 |
| Control 查询函数可能不支持某些查询维度 | 扩展 Control 查询函数（最小改动） |
| 部分端点可能需要特殊处理（如模糊搜索） | 逐个确认，复杂度高的记入 backlog |

---

## Sources & References

- `crates/organization_assemble_express/src/lib.rs`
- `crates/organization_assemble_control/src/lib.rs`
- `oa4rust/target/debug/behavior-report.md`（Express 端 FAIL 清单）
