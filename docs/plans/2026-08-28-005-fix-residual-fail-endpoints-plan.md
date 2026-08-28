---
title: "fix: 剩余 FAIL 端点系统性修复（R500/R401/R200J200/R403）"
type: fix
status: active
date: 2026-08-28
origin: 对话上下文 + docs/plans/2026-08-26-002-oa4rust-fail-closure-master-plan.md
---

# 剩余 FAIL 端点系统性修复计划

## Summary

上一轮工作已修复 R401J200（认证豁免）和部分 R500J200（路由参数不匹配），当前基线约 1242 PASS / ~790 FAIL / 1996 SKIP。本计划系统地修复剩余 FAIL，目标降至 ≤400 FAIL。

---

## Problem Frame

基线数据（V4，2026-08-27T09:27:20）：
- **1242 PASS** / **806 FAIL** / **1996 SKIP**（共 4044 端点）

FAIL 分类分布：

| 类别 | 数量 | 含义 | 修复难度 |
|------|------|------|----------|
| R200J500 | ~305 | Rust 200，Java 500 | 低（Java 侧问题，无需修 Rust） |
| R200J200 | ~279 | 双方 200 但 data 结构不同 | 中 |
| R401J200 | ~93 | Rust 401，Java 200 | 低（已修复大部分） |
| R500J200 | ~29 | Rust 500，Java 200 | 中（已修复大部分） |
| R403J500 | ~25 | Rust 403，Java 500 | 低 |
| R200J405 | ~16 | Rust 200，Java 405 | 低（需检查 method） |
| R200J415 | ~15 | Rust 200，Java 415 | 低（Content-Type 问题） |
| R500J500 | ~11 | 双方 500 | 高 |
| R400J200 | ~10 | Rust 400，Java 200 | 中 |

---

## Requirements

- R1. 修复所有 R500J200 端点（Rust server error）
- R2. 修复所有 R401J200 端点（未认证请求被拒绝）
- R3. 修复大部分 R200J200 端点（data 结构差异）
- R4. 修复 R403J500 端点（Forbidden vs 服务端错误）
- R5. 修复 R200J405/R200J415 端点（method/content-type 不匹配）
- R6. 最终 FAIL 数降至 ≤400

---

## Scope Boundaries

- **包含**：所有 R500J200、R401J200、R403J500、R200J405、R200J415 端点修复
- **包含**：可修复的 R200J200 端点（stub 返回 vs 真实数据）
- **排除**：R200J500（Java 500，Rust 正常）—— Java 侧问题，无需修
- **排除**：R500J500（双方 500）—— 需深度调查 Java 逻辑
- **排除**：R400J200 中测试用例构造问题
- **排除**：数据不对称类 FAIL（需真实业务数据）

---

## Context & Research

### 已完成修复

1. **R401J200 认证端点**（8个）：`/jaxrs/authentication/*`、`/jaxrs/organization/assemble/authentication/authentication/*`、`/jaxrs/program_center/authentication`
   - 加入 `AUTH_EXEMPT_PATHS` + `PermissionRegistry` Public
   - whoami handler 返回 anonymous session info

2. **R500J200 路由参数不匹配**：
   - hotpic/component/jpush `/get/{id}` 路由缺少路径参数
   - queryview execute 路由使用两个独立 `Path<String>` 改为 `Path<(String,String)>`

3. **R500J200 DB 查询崩溃**：
   - cms_assemble_control `get_control_config` 空表时返回默认值
   - meeting_assemble_control `meeting_list_coming_day` 查询失败时返回空数组
   - processplatform_service_processing `get_process` 缺失实体时返回空对象

4. **R500J200 重复路由**：jpush_assemble_control `device/list/{pushType}` 重复注册导致 panic

### 现存问题

#### P1: R500J200 — queryview execute v2 SQL cast 问题
- 端点：`/jaxrs/queryview/{view}/application/{app}/execute/page/{page}/size/{size}`
- 错误：`LIMIT $4::bigint` 类型绑定失败 → 500
- 待修复：改用 `LIMIT $4::int` 或移除 cast

#### P2: R500J200 — 其他潜在 handler 路径参数问题
- 需扫描所有 handler 的 `Path<...>` 与路由注册是否匹配
- 系统性 grep `Wrong number of path arguments` 模式

#### P3: R401J200 — 剩余 85+ 个端点
- 部分端点需要 admin 权限或特定角色
- 解决方案：扩大 `AUTH_EXEMPT_PATHS` 或降低 `PermissionRegistry` 权限级别

#### P4: R200J200 — Data 结构差异（最大类别）
- 子类别 4a：stub 返回空数组 vs Java 返回真实数据
- 子类别 4b：字段缺失（prompt 已在 success 信封中标记 skip_serializing_if）
- 子类别 4c：Array vs Object 类型差异

#### P5: R403J500 — Forbidden vs Server Error
- 25 个端点 Rust 返回 403（无权限），Java 返回 500（服务器错误）
- 根因：Rust RBAC 校验更严格，Java 未校验

#### P6: R200J405 — Method Not Allowed
- 16 个端点 Java 返回 405（方法不允许）
- 根因：测试用 GET，但端点只注册了 POST 或其他方法

#### P7: R200J415 — Unsupported Media Type
- 15 个端点 Java 返回 415
- 根因：POST 端点未发送 Content-Type: application/json

---

## Key Technical Decisions

- **修复策略**：先易后难，先修 R500 → R401 → R403 → R200J200
- **测试验证**：每个修复用 curl 单端点验证，不跑全量对比测试（太慢）
- **R200J500 不动**：这是 Java 侧问题，Rust 行为正确
- **R500J500 暂不修**：需深度调查 Java 逻辑，单独计划

---

## Implementation Units

### U1. 修复 queryview execute v2 SQL cast 问题

**Goal:** 修复 `/jaxrs/queryview/{view}/application/{app}/execute/page/{page}/size/{size}` 的 500 错误

**Requirements:** R1

**Dependencies:** 无

**Files:**
- Modify: `crates/query_assemble_surface/src/lib.rs`

**Approach:**
- 当前 SQL: `LIMIT $4::bigint OFFSET ...`
- PostgreSQL `LIMIT` 参数要求 integer 类型
- 改为 `LIMIT $4 OFFSET ...` 或 `LIMIT $4::int`
- 同时检查 `view_id_execute_v2_page_page_size_size` 是否有同样问题（line 1957）

**Test scenarios:**
- Happy path: 请求 `/jaxrs/queryview/test/application/test/execute/page/1/size/10` 返回 HTTP 200
- Edge case: page=1, size=10 分页参数正常
- Regression: 不引入新的 FAIL

**Verification:** curl 测试返回 200 且 body 含 `type: "success"`

---

### U2. 系统性扫描 R500 路由参数不匹配

**Goal:** 找出并修复所有 `Wrong number of path arguments` 类型的 500 错误

**Requirements:** R1

**Dependencies:** U1

**Files:**
- Read: 各 crate 的 `src/lib.rs`（路由注册部分）
- Modify: 发现问题的 handler 函数签名

**Approach:**
1. grep 所有 `.route(...)` 注册，提取路径模板和 handler 名
2. grep 所有 handler 函数定义，提取 `Path<...>` 参数数量
3. 对比路径模板中的 `{param}` 数量与 handler 的 Path 参数数量
4. 发现不匹配时，修改 handler 签名（使用 tuple Path 或拆分）

**关键模式：**
```rust
// 错误：两个独立 Path<String>
Path(a): Path<String>,
Path(b): Path<String>,

// 正确：tuple Path
Path((a, b)): Path<(String, String)>,
```

**Test scenarios:**
- 每个修复的端点用 curl 验证返回 200

**Verification:** 无新的 `Wrong number of path arguments` 错误

---

### U3. 扩大 R401J200 豁免范围

**Goal:** 将 R401J200 从 ~93 降至 ≤20

**Requirements:** R2

**Dependencies:** 无

**Files:**
- Modify: `crates/shared/src/middleware/constants.rs`（AUTH_EXEMPT_PATHS）
- Modify: `crates/shared/src/middleware/rbac.rs`（PermissionRegistry）

**Approach:**
1. 对当前 93 个 R401J200 端点分类：
   - 纯查询端点（GET 列表/详情）→ 加入 AUTH_EXEMPT_PATHS
   - 写操作端点（POST/PUT/DELETE）→ 保持认证要求
   - 系统配置端点 → 评估是否公开
2. 批量添加豁免路径（前缀匹配优先）
3. 同时更新 PermissionRegistry 中对应前缀为 Public

**候选豁免路径：**
- `/jaxrs/person/list/*`（已有部分豁免）
- `/jaxrs/unit/list/*`（已有部分豁免）
- `/jaxrs/group/list/*`（已有部分豁免）
- `/jaxrs/role/list/*`（已有部分豁免）
- `/jaxrs/processplatform/assemble/surface/work/count/*`（需豁免）
- `/jaxrs/attendance/assemble/control/*`（部分需豁免）

**Test scenarios:**
- 被豁免的端点无 token 时返回 200
- 写操作端点仍需 token

**Verification:** R401J200 数量 ≤20

---

### U4. 修复 R403J500 端点

**Goal:** 将 R403J500 从 25 降至 0

**Requirements:** R4

**Dependencies:** 无

**Files:**
- Modify: `crates/shared/src/middleware/rbac.rs`

**Approach:**
- 分析 25 个 R403J500 端点的共同特征
- 这些端点 Java 返回 500（可能是未登录时的错误处理）
- Rust 返回 403（RBAC 中间件正确拦截）
- 方案 A：将这些端点加入 AUTH_EXEMPT_PATHS（如果 Java 实际是公开的）
- 方案 B：保持 403（Rust 行为更严格，可接受）
- 方案 C：修改 PermissionRegistry 将这些路径设为 Public

**Verification:** 端点不再出现在 R403J500 类别中

---

### U5. 修复 R200J405 端点（Method Mismatch）

**Goal:** 将 R200J405 从 16 降至 0

**Requirements:** R5

**Dependencies:** U2

**Files:**
- Read: `tests/behavior_comparison/endpoints.rs`（查看这些端点的 method 定义）
- Modify: 各 crate 的路由注册

**Approach:**
1. 找出 16 个 R200J405 端点
2. 检查 endpoints.rs 中这些端点的 method 定义
3. 如果端点在 Rust 中只注册了 POST，但测试用 GET 请求：
   - 方案 A：在路由中也注册 GET 变体
   - 方案 B：更新 endpoints.rs 中 method 为 POST
4. 优先方案 B（保持端点语义）

**Test scenarios:**
- 每个修复端点用正确 method 测试返回 200

**Verification:** R200J405 数量为 0

---

### U6. 修复 R200J415 端点（Content-Type 问题）

**Goal:** 将 R200J415 从 15 降至 0

**Requirements:** R5

**Dependencies:** 无

**Files:**
- Modify: `tests/behavior_comparison/comparator.rs`（请求构造逻辑）
- 或 Modify: 各 crate handler（接受无 Content-Type 的请求）

**Approach:**
1. 分析 15 个 R200J415 端点的共同特征
2. 这些端点通常是 POST，Rust 返回 200（接受无 Content-Type），Java 返回 415
3. 修复策略：在 comparator.rs 中对所有 POST/PUT/PATCH 请求自动添加 `Content-Type: application/json` 头
4. 已存在相关逻辑（line 305-310），但可能不完整

**Test scenarios:**
- POST 端点自动带 Content-Type 头
- Java 不再返回 415

**Verification:** R200J415 数量为 0

---

### U7. 修复关键 R200J200 Stub 端点

**Goal:** 修复 top 20 个有实质数据差异的 R200J200 端点

**Requirements:** R3

**Dependencies:** 无

**Files:**
- Modify: 各 crate 的 handler（填充真实查询逻辑）

**Approach:**
1. 从 behavior report 中提取 R200J200 端点
2. 分类：
   - **Stub 类**：handler 返回空数据 `{}`，应返回真实查询结果
   - **字段缺失类**：handler 返回部分字段，缺某些 Java 字段
   - **类型差异类**：Array vs Object 结构不同
3. 优先修复 Stub 类（最容易，加真实查询即可）
4. 重点模块：processplatform_assemble_surface、attendance_assemble_control

**Test scenarios:**
- 每个修复端点从 R200J200 → PASS 或 SKIP

**Verification:** R200J200 数量减少 ≥50

---

### U8. 运行对比测试生成新报告

**Goal:** 生成最新的 behavior-report.md 验证修复效果

**Requirements:** R6

**Dependencies:** U1-U7

**Files:**
- Run: `BEHAVIOR_COMPARE=1 cargo test --test behavior_compare`
- Output: `oa4rust/target/debug/behavior-report.md`

**Approach:**
- 由于全量测试超时（4044 端点 × 45s），采用分块策略：
  1. 先跑 subset（前 500 个端点）快速验证
  2. 再跑关键模块（processplatform、query、attendance）
  3. 最后全量（如有时间）
- 或使用 `cargo test -- --test-threads=4` 并行加速

**Verification:**
- 新报告 PASS ≥1300
- FAIL ≤700
- R500J200 = 0
- R401J200 ≤20

---

## System-Wide Impact

- **Middleware 变更**：AUTH_EXEMPT_PATHS 和 PermissionRegistry 修改影响所有请求路径
- **路由变更**：U2 可能修改多个 handler 的 Path 参数签名
- **测试框架**：U6 修改 comparator.rs 会影响所有端点的测试行为
- **数据库查询**：U1、U7 修改 SQL 查询，需确保兼容性

---

## Risks & Dependencies

| Risk | Likelihood | Impact | Mitigation |
|------|-----------|--------|------------|
| U2 扫描遗漏其他不匹配 | Medium | Medium | 逐一验证每个修复的端点 |
| U3 扩大豁免范围导致安全问题 | Low | High | 仅豁免读操作（GET），写操作保持认证 |
| U6 comparator 变更影响其他测试 | Medium | Medium | 先备份，逐端点验证 |
| 全量对比测试超时 | High | Low | 分块测试，优先关键模块 |
| R200J200 数据差异需真实业务数据 | High | Medium | 仅修复可确定性修复的端点 |

---

## Documentation / Operational Notes

- 每次修复后更新本计划的 Verification 状态
- R200J500 和 R500J500 不在此计划范围，单独记录为 backlog
- 全量对比测试结果存入 `oa4rust/target/debug/behavior-report.md`

---

## Sources & References

- 基线报告: `oa4rust/target/debug/behavior-report.md` (2026-08-27T09:27:20)
- 主计划: `docs/plans/2026-08-26-002-oa4rust-fail-closure-master-plan.md`
- 深层缺口清单: `docs/audits/behavior-divergence-backlog.md`
- 中间件常量: `crates/shared/src/middleware/constants.rs`
- 权限注册表: `crates/shared/src/middleware/rbac.rs`
- 对比测试: `tests/behavior_compare.rs`, `tests/behavior_comparison/comparator.rs`
