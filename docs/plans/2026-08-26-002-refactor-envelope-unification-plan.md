---
title: "refactor: 信封统一——消除 Rust/Java 响应结构差异"
type: refactor
status: superseded
date: 2026-08-26
origin: docs/plans/2026-08-26-001-feat-parity-convergence-phase2-plan.md
superseded_by: docs/plans/2026-08-26-002-oa4rust-fail-closure-master-plan.md
---

# 信封统一——消除 Rust/Java 响应结构差异

## Summary

将 Rust handler 的响应信封从 `success()` 统一为 `java_success()`，使 `{data, type, message, date, spent, size, count, position}` 9 字段与 Java Gson 序列化行为完全对齐。当前 ~600 个 handler 使用 `success()`（硬编码 count=0, size=0），而 Java 端对列表端点返回 `{data:[...], count:N, size:M}`。此重构预计可将 FAIL 从836降至~200（消除约620条信封差异类假阳性）。

---

## Problem Frame

Phase 2 U3 聚类分析发现：844 条 FAIL 中，约620条为**信封/包装结构性差异**，而非字段改名问题。核心差异：

| 差异类型 | 数量 | 根因 |
|---------|------|------|
| `data` vs `prompt` 互换 | 347 | Rust 成功用 `data` 包装，Java 部分端点用 `prompt` 包装 |
| `count` 包装差异 | 78 | Rust `{count:N, data:[...]}` vs Java 裸数组 `[{...}]` |
| 上传响应信封 | 45 | Java `{position,count,spent,type,size,date,prompt}` vs Rust `{status,servlet,url}` |
| 数组长度差异 | 49 | 双方都有 `data` 但数组长度不同（数据不对称） |
| 其他结构差异 | ~100 | 字段名不同、类型不同等 |

`success()` vs `java_success()` 的关键区别：

```rust
// success() — 当前多数 handler 使用
ActionResult::success(data)
// → { data: Some(data), count: Some(0), size: Some(0), ... }

// java_success() — Java 兼容格式
ActionResult::java_success(data, count, size)
// → { data: Some(data), count: Some(N), size: Some(M), ... }
```

---

## Requirements

- R1. 所有列表端点（返回数组的 handler）统一使用 `java_success(data, count, size)` 而非 `success(data)`
- R2. 单对象端点保持 `success(data)` 不变（count=0, size=0 与 Java 行为一致）
- R3. 错误端点保持 `AppError` / `ActionResult::error()` 不变
- R4. 重构后全量 compare 的 PASS 数增长 ≥300（从1212到≥1500）
- R5. 不改变任何端点的业务语义，仅调整响应信封形状

---

## Scope Boundaries

- 不修改 `comparator.rs` 的比较规则
- 不修改 `endpoints.rs` 的端点清单
- 不修改业务逻辑（handler 的输入处理、数据库查询）
- 不处理"上传响应信封"差异（Java 的 `{position,count,spent,type,size}` vs Rust 的 `{status,servlet,url}`）——这是第四层差异，需独立处理
- 不处理"数组长度差异"（数据不对称类）——需共享种子或真实数据解决

---

## Context & Research

### Relevant Code and Patterns

- `crates/shared/src/response.rs`：`ActionResult` 结构体 + `success()` + `java_success()` + `error()`
- `crates/shared/src/error.rs`：`AppError` 枚举（9变体），`IntoResponse` 实现
- ~600 处 `success()` 调用，~133 处 `java_success()` 调用
- `tests/behavior_comparison/comparator.rs`：比较规则（冻结不变）

### Institutional Learnings

- `docs/solutions/best-practices/oa4rust-o2server-parity-closure-campaign-2026-08-25.md`：包装模式战役方法论
- Phase 1 提交 `9d81b8ca`：214 个 handler `{count,data}`→`java_success` 的经验

---

## Key Technical Decisions

- **仅改列表端点的信封**：单对象端点的 `success(data)` 与 Java 行为一致（count=0, size=0），无需改动
- **分批改、分批测**：按 crate 分批，每批 ≤50 个 handler，改后立即跑全量 compare 验证 PASS 增量
- **保留 `success()` 函数**：单对象端点仍使用它，不删除
- **不引入新的构造函数**：当前 `success()` + `java_success()` + `error()` 三个构造函数足够

---

## Implementation Units

### U1. 核心组织模块——organization_assemble_control + organization_assemble_express

**Goal:** 统一组织模块的列表端点信封，消除73条信封差异（control 23 + express 50）

**Requirements:** R1, R4

**Dependencies:** 无

**Files:**
- Modify: `crates/organization_assemble_control/src/lib.rs`（~23 处 `success`→`java_success`）
- Modify: `crates/organization_assemble_express/src/lib.rs`（~50 处 `success`→`java_success`）
- Test: 全量 compare 重跑

**Approach:**
- grep `Ok(Json(ActionResult::success(` 定位所有列表端点
- 对每个返回 `Value::Array(...)` 或 `Vec<...>` 的 handler，改为 `ActionResult::java_success(data, count, size)`
- count 从查询总数获取（已有 `count` 变量或 `len()`），size 从实际返回数量获取
- 对 `organization_assemble_express` 的 POST list 端点（~50条），这些是完全未实现的路由——不在本 unit 范围，记入 backlog

**Test scenarios:**
- Happy path: 某列表端点改后从 FAIL→PASS
- Edge case: 空列表返回 `java_success([], 0, 0)` 与 Java 空数组行为一致
- Regression: 改后不引入新的 FAIL

**Verification:**
- 全量 compare PASS 数增长 ≥40（对应 express 43 + control 23 的信封差异）
- 受影响 crate 既有单元测试全绿

---

### U2. CMS 模块——cms_assemble_control

**Goal:** 统一 CMS 模块的列表端点信封，消除58条信封差异

**Requirements:** R1, R4

**Dependencies:** 无

**Files:**
- Modify: `crates/cms_assemble_control/src/lib.rs`（~58 处）
- Test: 全量 compare 重跑

**Approach:**
- CMS 模块几乎全是 Pattern A（Rust→`data`, Java→`prompt`），说明 CMS handler 统一使用了 `success()` 而 Java 用 `prompt` 包装
- 需要逐个确认：哪些是列表端点（改 `java_success`），哪些是单对象端点（保持 `success`），哪些是错误端点（保持 `error`）
- CMS 的 `mockdeletetoget` 模式（mock delete-to-get）需要特殊处理：Java 返回 `{prompt: "message"}` 而 Rust 返回 `{data: result}`——这可能是业务语义差异，需逐个确认

**Test scenarios:**
- Happy path: CMS 列表端点改后 PASS
- Edge case: CMS 的 mock 操作端点确认是否为信封差异还是语义差异
- Regression: CMS 既有测试全绿

**Verification:**
- 全量 compare PASS 数增长 ≥30
- CMS crate 单元测试全绿

---

### U3. 流程平台模块——processplatform_assemble_surface + processplatform_assemble_designer

**Goal:** 统一流程平台的列表端点信封，消除82条信封差异

**Requirements:** R1, R4

**Dependencies:** 无

**Files:**
- Modify: `crates/processplatform_assemble_surface/src/lib.rs`（~70 处）
- Modify: `crates/processplatform_assemble_designer/src/lib.rs`（~12 处）
- Test: 全量 compare 重跑

**Approach:**
- processplatform 是最大的信封差异 crate（70条），主要集中在 surface 的工作流查询端点
- 该模块 handler 密度高（461处 `success`），需要仔细区分列表 vs 单对象端点
- 附件端点的 `{position,count,spent,type,size,date,prompt}` 差异（45条）不在本 unit 范围——这是独立的上传响应结构问题

**Test scenarios:**
- Happy path: 流程平台列表端点改后 PASS
- Edge case: 工作流引擎相关端点确认信封差异 vs 语义差异
- Regression: processplatform 单元测试全绿

**Verification:**
- 全量 compare PASS 数增长 ≥40
- processplatform crate 测试全绿

---

### U4. 查询/考勤/消息模块

**Goal:** 统一 query、attendance、message 模块的列表端点信封，消除44条信封差异

**Requirements:** R1, R4

**Dependencies:** 无

**Files:**
- Modify: `crates/query_assemble_designer/src/lib.rs`（~13 处）
- Modify: `crates/query_assemble_surface/src/lib.rs`（~12 处）
- Modify: `crates/attendance_assemble_control/src/lib.rs`（~20 处）
- Modify: `crates/message_assemble_communicate/src/lib.rs`（~9 处）
- Test: 全量 compare 重跑

**Approach:**
- 四个 crate 合计44条，每个 crate 独立改、独立测
- query 模块的 `search` 端点需要确认返回格式（可能是分页结果）
- attendance 模块的 `analyse` 端点可能返回复杂结构，需逐个确认

**Test scenarios:**
- Happy path: 各模块列表端点改后 PASS
- Edge case: 分页端点的 count/size 取值
- Regression: 各模块单元测试全绿

**Verification:**
- 全量 compare PASS 数增长 ≥20
- 各模块单元测试全绿

---

### U5. 其余模块（program_center / portal / meeting / general / file / personal / ai / mind / calendar / auth）

**Goal:** 统一剩余模块的列表端点信封，消除~80条信封差异

**Requirements:** R1, R4

**Dependencies:** 无

**Files:**
- Modify: 各模块的 `src/lib.rs`（每个 ~3-20 处）
- Test: 全量 compare 重跑

**Approach:**
- 小模块批量处理，每个模块改完即测
- `program_center` 有20条差异，需优先处理
- `portal` 有8条，`meeting` 有5条，`general` 有5条，`file` 有8条
- `ai`、`personal`、`mind`、`calendar`、`auth` 各 2-4 条

**Test scenarios:**
- Happy path: 各模块列表端点改后 PASS
- Regression: 各模块单元测试全绿

**Verification:**
- 全量 compare PASS 数增长 ≥30
- 所有受影响模块测试全绿

---

### U6. 全量验证与报告

**Goal:** 全量 compare 重跑，记录 PASS/FAIL/SKIP 前后对照，更新终扫文档

**Requirements:** R4, R5

**Dependencies:** U1-U5

**Files:**
- Modify: `docs/audits/final-coverage-sweep.md`（§六追加信封统一小节）
- Test: `cargo test --test behavior_compare` 全量

**Appro:**
- 启动 Rust 服务 + Java 服务，应用种子，跑全量 compare
- 记录 PASS/FAIL/SKIP 数字，与基线（1212/836/1996）对比
- 生成新的 behavior-report.md，确认信封差异类 FAIL 已消除

**Test scenarios:**
- Test expectation: none — 纯验证与文档更新

**Verification:**
- PASS ≥1500（较基线 +25%）
- FAIL 中信封差异类（`data`/`prompt` 互换）数量 ≤50

---

## System-Wide Impact

- **Interaction graph:** 信封形状变更影响所有消费 Rust API 的客户端（前端、测试脚本、影子流量脚本）
- **Error propagation:** 错误端点不变，仅成功端点的信封字段值变化
- **State lifecycle risks:** 无状态变更，纯响应格式调整
- **API surface parity:** 信封统一后，Rust API 与 Java API 在 JSON 结构层面完全对齐
- **Unchanged invariants:** 业务逻辑不变、路由不变、数据库 schema 不变

---

## Risks & Dependencies

| Risk | Mitigation |
|------|------------|
| 改动 handler 数量大（~600处） | 分 crate 分批，每批测后确认 |
| 部分端点 count 取值不确定 | 从查询总数获取，无总数的用 `len()` |
| 信封统一后前端可能受影响 | 信封字段是超集，向后兼容 |
| Java 的 `prompt` 包装差异 | 逐个确认是信封差异还是语义差异 |

---

## Sources & References

- Phase 2 plan: `docs/plans/2026-08-26-001-feat-parity-convergence-phase2-plan.md`
- 包装模式战役: `docs/solutions/best-practices/oa4rust-o2server-parity-closure-campaign-2026-08-25.md`
- ActionResult 契约: `docs/solutions/architecture-patterns/actionresult-9-field-contract.md`
- 信封分析数据: `oa4rust/target/debug/behavior-report.md`（2026-08-26 运行）
