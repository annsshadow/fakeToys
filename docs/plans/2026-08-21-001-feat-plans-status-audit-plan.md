---
title: feat: 计划文档状态全面审计与剩余工作汇总
type: feat
status: completed
date: 2026-08-21
origin: docs/brainstorms/2026-08-21-plans-status-audit-and-consolidation-requirements.md
---

# 计划文档状态全面审计与剩余工作汇总计划

## Summary

按年代分三批对 `docs/plans/` 全部 18 份计划文档做实施单元级审计，以静态证据（路由挂载、测试文件、git 历史、反向索引）对照代码库验证实际状态；原地更新各文档 frontmatter `status`（严格三态制）并追加"实现情况"小节；将全部未实现功能单元、未完成 Deferred 项与 `docs/todo.md` 未完成项汇总为一份新计划文档（编号预留 2026-08-21-002）；随后清空 `todo.md` 并执行终检断言。

---

## Problem Frame

`docs/plans/` 中 18 份计划的 frontmatter 状态与代码库实际状态脱节：16 份有 frontmatter 的文档中 4 份仍标 `active`，但对应工作多数已完成；2 份完全没有 frontmatter。各计划散落的未完成项（含两类 Deferred 小节）与 `docs/todo.md` 的 4 个 Linux 文档待办项没有统一视图，后续规划要么重复审计、要么基于过期状态决策。（详见 origin 文档 Problem Frame）

---

## Requirements

- R1. 对 18 份计划逐实施单元审计，静态验证为主（文件存在性、路由挂载、测试覆盖、git 历史），争议点针对性验证
- R2. 各计划 "Deferred to Follow-Up Work" 与 "Deferred to Implementation" 小节的延期项纳入审计
- R3. 原地更新 frontmatter `status`，采用严格三态：`active` / `completed` / `superseded`；现有 1 处规范外的 `partially_completed` 归一为 `active`
- R4. 每份被更新的计划追加简短"实现情况"小节：已验证完成的单元（附证据）、未完成单元及原因、无法验证的单元显式标注
- R5. 所有计划保留在 `docs/plans/`，不移入归档目录，永不删除计划文件
- R6. 所有未实现项汇总为**一份**新计划文档，放在 `docs/plans/`，遵循命名约定
- R7. 新计划中每个遗留项注明来源计划与原始单元编号，可追溯
- R8. `docs/todo.md` 的未完成项（Linux 文档翻译校正等 4 项）纳入新计划
- R9. 新计划落地后清空 `todo.md`（保留空文件或占位说明），纯指引行随清空移除

**Origin actors / flows / acceptance examples:** origin 未定义（文档审计任务，无行为面）。

---

## Scope Boundaries

- 不修改任何业务代码、不执行未实现的功能——纯文档工作
- 不重写历史计划正文内容，只更新 frontmatter 状态与追加实现情况小节
- 不处理 `docs/archived-plans/` 已归档计划
- 审计不强制运行完整 `cargo test --workspace`

---

## Context & Research

### Relevant Code and Patterns

- **frontmatter 约定**：字段序 title→type→status→date→origin；`type` 取值 feat/fix/refactor/analysis/plan；2 份无 frontmatter 文档需补建：`docs/plans/2026-08-17-bpmn-gate.md`、`docs/plans/2026-08-19-002-unified-completion-plan.md`
- **当前状态分布**：completed×11、active×4、partially_completed×1（`2026-08-12-002`）、无 frontmatter×2
- **验证证据面**：
  - 路由挂载：`oa4rust/src/main.rs` 全文件约 88 处 `.merge()`（`create_app()` 为主体；empower 特例为双重模块路径，354 行起）
  - 测试布局：`crates/*/src/tests_generated.rs`（90 个）、`crates/*/src/tests.rs`（86 个）、`oa4rust/tests/behavior_compare.rs` + `tests/behavior_comparison/endpoints.rs`（14,576 行端点清单）
  - 脚本：`oa4rust/scripts/` 下 `generate_handler_tests.py`（注意：非 gen_handler_tests.py）、`audit_router_wiring.py`、`count_routes.py`、`populate_endpoints_docs.py` 等
  - git 历史：2026-08-17 至 08-21 提交密集覆盖近期计划范围；**08-21 当天仍有提交落在 08-20 计划范围内**，验证以工作树 HEAD 为准
  - 反向索引：`docs/solutions/` 多篇文档末尾 `Related → Plan:` 字段指向具体计划的具体系，可作为"已验证实现"旁证
- **todo.md 目标目录**：`docs/linux-7.1.3/` 实际存在（完整内核文档树）

### Institutional Learnings

- `docs/solutions/development-workflow/plan-status-lifecycle.md`：三态生命周期权威规范；superseded 需附 `<!-- Superseded by: ... -->` 注释；永不删除计划文件；终检命令 `grep -r "status: active" docs/plans/`
- `docs/solutions/best-practices/single-source-of-truth-migration-status.md`：状态单一来源、机器可解析；文档与代码不一致时强制调查并以实测为准；其"81 crates 全部真实化"结论已时效 11 天，不可直接采信
- `docs/solutions/tooling-decisions/oa-component-card-generation.md`：批量 markdown 修改后用计数断言收尾，防止漏改误改

### External References

- 无（纯内部文档工作，未做外部研究）

---

## Key Technical Decisions

- **严格三态制**（用户确认）：有剩余工作的计划一律 `active`，部分完成细节写入"实现情况"小节；`partially_completed` 归一为 `active`。保证 `grep "status: active"` 终检语义干净
- **按年代分三批审计**：早期计划多标 completed，用 git 历史交叉验证 + 抽查即可；近期计划需代码级核对。证据密度不同，分批控制单批规模
- **静态验证优先**：以文件存在性、路由注册、测试文件、git 历史、反向索引为主要证据；仅在关键争议点做针对性运行验证
- **两类 Deferred 小节都收集**："Deferred to Follow-Up Work" 与 "Deferred to Implementation" 均为延期项来源
- **新汇总计划预留编号 2026-08-21-002**：本审计计划自身占用 001，避免执行期编号冲突
- **fail loud**：无法验证的单元显式标注 unverified 及原因，不默认 completed（CLAUDE.md Rule 12）
- **不一致处理**：计划自述与代码不符时，以代码实测为准修正状态，并在实现情况小节记录差异

---

## Open Questions

### Resolved During Planning

- 状态取值体系？→ 严格三态（用户确认，规范优先于实践中的偏离值）
- 无 frontmatter 的 2 份文档如何处理？→ 按 `title/type/status/date(/origin)` 约定补建
- 生成脚本名？→ 更正为 `generate_handler_tests.py`（研究核实）

### Deferred to Implementation

- [Affects U1] `2026-08-17-bpmn-gate.md` 属进展/门禁笔记性质，其"完成"判定标准需执行时按内容实质确定
- [Affects U1] 是否运行一次 `cargo test --workspace --lib` 作为全局健康佐证——视静态验证的争议程度决定
- [Affects U2/U3] 个别单元若静态证据不足，是否升级为针对性运行验证——执行时逐案判断

---

## Implementation Units

### U1. 审计近期计划批次（2026-08-17 ~ 2026-08-20，4 份）

**Goal:** 对最可能含未完成工作的 4 份近期计划完成单元级验证并原地更新

**Requirements:** R1, R2, R3, R4

**Dependencies:** None

**Files:**
- Modify: `docs/plans/2026-08-17-bpmn-gate.md`（补建 frontmatter）
- Modify: `docs/plans/2026-08-19-001-fix-close-all-blocking-gaps-plan.md`（现 active）
- Modify: `docs/plans/2026-08-19-002-unified-completion-plan.md`（补建 frontmatter）
- Modify: `docs/plans/2026-08-20-001-feat-oa4rust-remaining-gap-closure-plan.md`

**Approach:**
- 逐单元对照代码库：U1 行为对比测试条件执行（查 `tests/behavior_compare.rs` 的环境变量守卫）、U2 crate 挂载（查 `src/main.rs` merge 列表）、U3 handler 测试（查 `tests_generated.rs` 数量）、U4 端点对齐度（git log 显示 08-21 达 36.6%，目标 ≥70% 未达）、U5 文档填充（抽查模块卡片 REST Endpoints 字段）
- 收集各计划 Deferred 小节中未完成项
- 每份追加"实现情况（2026-08-21 审计）"小节并更新 status

**Patterns to follow:**
- `docs/solutions/development-workflow/plan-status-lifecycle.md` 的状态操作规则

**Test scenarios:**
- Test expectation: none -- 纯文档更新，验证以终检断言承担（见 U5）

**Verification:**
- 4 份文档均有合法三态 status 与实现情况小节；每个判定附证据引用（文件路径或提交号）

---

### U2. 审计中期计划批次（2026-08-12 ~ 2026-08-13，6 份）

**Goal:** 对 6 份中期计划完成单元级验证并原地更新，含 `partially_completed` 归一

**Requirements:** R1, R2, R3, R4

**Dependencies:** None

**Files:**
- Modify: `docs/plans/2026-08-12-001-fix-oa4rust-final-gap-closure-plan.md`（现 active）
- Modify: `docs/plans/2026-08-12-002-feat-oa4rust-audit-and-test-coverage-plan.md`（partially_completed → 归一）
- Modify: `docs/plans/2026-08-13-001-feat-handler-test-coverage-99-plan.md`
- Modify: `docs/plans/2026-08-13-001-feat-oa4rust-seaorm-entity-parity.md`
- Modify: `docs/plans/2026-08-13-002-oa4rust-o2server-parity-analysis.md`
- Modify: `docs/plans/2026-08-13-003-oa4rust-completion-plan.md`（现 active）

**Approach:**
- 单元级静态核查 + `docs/solutions/` 的 `Related → Plan:` 反向索引旁证
- 重点核验两份 active 计划的真实剩余面：`08-12-001` 与 `08-13-003` 可能已被后续计划覆盖完成，也可能确有遗留
- `08-12-002` 的部分完成细节从原状态迁入实现情况小节后再归一状态值

**Patterns to follow:**
- 同 U1

**Test scenarios:**
- Test expectation: none -- 纯文档更新

**Verification:**
- 6 份文档状态合法且与证据一致；`partially_completed` 在全目录归零

---

### U3. 审计早期计划批次（2026-08-08 ~ 2026-08-11，8 份）

**Goal:** 对 8 份早期计划（7 份已标 completed、1 份 active）做轻量验证并原地更新

**Requirements:** R1, R2, R3, R4

**Dependencies:** None

**Files:**
- Modify: `docs/plans/2026-08-08-001-feat-multi-axis-quality-plan.md`（现 active）
- Modify: `docs/plans/2026-08-10-001-feat-oa4rust-gap-audit-plan.md`
- Modify: `docs/plans/2026-08-10-002-feat-oa4rust-auth-provider-closure-plan.md`
- Modify: `docs/plans/2026-08-10-002-fix-oa4rust-gap-closure-plan.md`
- Modify: `docs/plans/2026-08-11-001-feat-oa4rust-full-gap-closure-plan.md`
- Modify: `docs/plans/2026-08-11-002-feat-oa4rust-core-modules-gap-closure-plan.md`
- Modify: `docs/plans/2026-08-11-003-feat-oa4rust-api-gap-closure-plan.md`
- Modify: `docs/plans/2026-08-11-004-feat-oa4rust-remaining-gap-closure-plan.md`

**Approach:**
- 已标 completed 的 7 份：git 历史交叉验证 + 每份抽查 1-2 个代表性交付物（如认证模块文件、MCP 工具桥接产物）；发现不符时升级为代码级核查并记录差异
- `08-08-001`（active）：核验其质量轴目标当前达成度，判定 completed 或 active
- 收集未完成的 Deferred 项

**Patterns to follow:**
- 同 U1

**Test scenarios:**
- Test expectation: none -- 纯文档更新

**Verification:**
- 8 份文档状态合法；completed 判定均有至少一条证据支撑

---

### U4. 汇总未实现项并撰写新计划文档

**Goal:** 将三批审计发现的全部未实现项收敛为一份可执行的新计划

**Requirements:** R6, R7, R8

**Dependencies:** U1, U2, U3

**Files:**
- Create: `docs/plans/2026-08-21-002-feat-remaining-work-consolidation-plan.md`

**Approach:**
- 内容三大来源：① 三批审计发现的未实现功能单元；② 各计划两类 Deferred 小节中未完成项；③ `docs/todo.md` 的 4 个 Linux 文档项（翻译校正、残余复核、链接审计、格式统一）
- 每个条目注明来源计划路径与原始单元编号（如 `来源: docs/plans/2026-08-20-001-... U4`）
- 按 `single-source-of-truth-migration-status.md` 采用机器可解析格式：图例 + 汇总表 + 明细分组（oa4rust 组 / Linux 文档组）
- 新计划遵循标准计划模板（frontmatter、status: active、Summary、Implementation Units 等），使后续可直接进入 ce-work 执行

**Patterns to follow:**
- `docs/plans/2026-08-20-001-feat-oa4rust-remaining-gap-closure-plan.md` 的模板结构
- `docs/solutions/best-practices/single-source-of-truth-migration-status.md` 的表格格式

**Test scenarios:**
- Test expectation: none -- 纯文档创建

**Verification:**
- 对照三批审计的实现情况小节逐条核对：所有"未完成/unverified"条目在新计划中均有对应条目，无遗漏
- todo.md 的 4 个待办项全部出现在新计划中

---

### U5. 清空 todo.md 与终检断言

**Goal:** 消除冲突状态文件，并对整个审计做机器可校验的收尾

**Requirements:** R9, R5

**Dependencies:** U4

**Files:**
- Modify: `docs/todo.md`
- Modify: `docs/plans/2026-08-21-001-feat-plans-status-audit-plan.md`（收尾翻转自身状态）

**Approach:**
- 内容迁出后将 `todo.md` 置为占位说明（指向新汇总计划），删除原有全部条目
- 终检断言（PowerShell 环境，用 ripgrep；无 rg 时可用 `Select-String` 替代）：
  - `rg -n "^status: active" docs/plans/` 仅命中真实进行中的计划（行首锚定，排除正文引用噪声；预期含新 002 计划及审计后仍 active 者）
  - `rg -n "^status: (partially_completed|in-progress|partial)" docs/plans/` 零命中
  - 18 份计划均存在"实现情况"小节且 frontmatter 合法
  - `docs/archived-plans/` 无改动
- 四项断言全部通过后，将本审计计划（001）frontmatter status 置为 completed，与终检收尾在同一提交完成

**Patterns to follow:**
- `docs/solutions/tooling-decisions/oa-component-card-generation.md` 的计数断言收尾法

**Test scenarios:**
- Test expectation: none -- 断言即验证

**Verification:**
- 四项终检断言全部通过；任何失败项回溯对应批次修正

---

## System-Wide Impact

- **Interaction graph:** 新 002 计划成为剩余工作唯一真相源；`docs/todo.md` 清空后其历史内容依赖 002 计划的来源标注保持可追溯
- **Error propagation:** 若某计划状态误判为 completed，会通过 002 计划的遗漏传导到后续执行——由 U4 的逐条核对（对照各计划实现情况小节核对 002 计划条目无遗漏）拦截
- **State lifecycle risks:** `partially_completed` 归一后，若有外部流程依赖该值将失效——全仓 grep 确认仅 1 处使用，风险可控
- **Unchanged invariants:** 历史计划正文不改写；`docs/archived-plans/` 不动；永不删除计划文件（plan-status-lifecycle 规范）

---

## Risks & Dependencies

| Risk | Mitigation |
|------|------------|
| 计划自述与代码不一致造成误判 | 以代码实测为准修正状态，差异显式记录在实现情况小节 |
| 18 份 × 逐单元验证工作量超预期 | 反向索引旁证减少核对量；分批推进；无法验证的标 unverified 而非强行验证 |
| 08-21 当天仍有提交落在计划范围 | 审计基准固定为工作树 HEAD，实现情况小节注明审计时点 |
| 状态归一造成部分完成信息丢失 | 细节迁入实现情况小节后再改状态值 |

---

## Documentation / Operational Notes

- 审计完成后可考虑用 `/ce-compound` 沉淀"计划状态全面审计"经验（`docs/solutions/` 中目前无此类文档）
- 后续新增计划时应即时维护 status，避免再次积累漂移

---

## Sources & References

- **Origin document:** [docs/brainstorms/2026-08-21-plans-status-audit-and-consolidation-requirements.md](../brainstorms/2026-08-21-plans-status-audit-and-consolidation-requirements.md)
- Related learnings: `docs/solutions/development-workflow/plan-status-lifecycle.md`, `docs/solutions/best-practices/single-source-of-truth-migration-status.md`, `docs/solutions/tooling-decisions/oa-component-card-generation.md`
- Related code: `oa4rust/src/main.rs`, `oa4rust/tests/behavior_compare.rs`, `docs/todo.md`
