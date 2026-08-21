---
date: 2026-08-21
topic: plans-status-audit-and-consolidation
---

# 计划文档状态审计与未实现功能汇总

## Summary

对 `docs/plans/` 全部 18 份计划文档逐实施单元做全面审计，对照代码库验证实际实现状态并原地更新各文档的状态标记与实现情况说明；将所有未实现功能（含 Deferred 延期项）连同 `docs/todo.md` 的未完成项汇总为一份新的独立计划文档；新计划落地后清空 `docs/todo.md`。

---

## Problem Frame

`docs/plans/` 积累了 2026-08-08 至 2026-08-20 的 18 份计划文档。期间大量缺口补全工作已完成（crate 挂载修复、handler 测试补全、端点对齐、行为对比测试等），但多数计划文档的 frontmatter `status` 仍停留在写入时的值，与代码库实际状态脱节。读者无法从计划目录判断哪些工作已落地、哪些仍是缺口。

同时，各计划中明确延期的项（"Deferred to Follow-Up Work"）和 `docs/todo.md` 中的未完成项散落多处，没有一份统一的"剩余工作"视图。若不做一次收敛，后续规划要么重复审计，要么基于过期状态做决策。

---

## Requirements

**计划文档审计**

- R1. 对 `docs/plans/` 全部 18 份计划文档逐实施单元（Implementation Units）审计，对照代码库验证实际实现状态。验证方式以静态检查为主：文件存在性、路由挂载、测试文件覆盖、git 提交历史；关键争议点做针对性验证
- R2. 各计划中 "Deferred to Follow-Up Work" 延期项一并纳入审计，判定其当前完成状态

**状态更新**

- R3. 原地更新每份计划的 frontmatter `status` 字段：完全实现的标 `completed`，部分实现的用明确的部分完成标记，未实现的保持未完成标记
- R4. 每份被更新的计划附简短"实现情况"小节：哪些单元已验证完成（附验证证据）、哪些未完成及原因
- R5. 所有计划文档保留在 `docs/plans/`，不移入 `docs/archived-plans/`

**新计划文档**

- R6. 将所有未实现的功能单元与未完成的 Deferred 项汇总为**一份**新计划文档，放在 `docs/plans/` 下，遵循现有命名约定（日期前缀 + 类型前缀）
- R7. 新计划文档中每个遗留项注明来源计划文档与原始单元编号，保证可追溯
- R8. `docs/todo.md` 的未完成项（Linux 文档翻译校正、链接完整性审计、格式统一等）纳入新计划文档

**todo.md 清理**

- R9. 新计划落地后清空 `docs/todo.md` 内容（保留空文件或占位说明）；其中纯指引性内容（如指向 `docs/reviews/` 的说明）视为非待办项，随清空一并移除

---

## Success Criteria

- 每份计划的 `status` 与代码库实际状态一致，且有可查的验证证据
- 一份新计划文档完整覆盖：全部未实现功能单元 + 未完成 Deferred 项 + `todo.md` 未完成项，无遗漏
- `todo.md` 已清空，且其原有内容都能在新计划文档中找到对应条目
- 下游规划可直接基于新计划文档开展工作，无需重新审计历史计划

---

## Scope Boundaries

- 不修改任何业务代码、不执行未实现的功能——本任务是纯文档工作
- 不重写历史计划的内容本身，只更新状态与实现情况说明
- 不处理 `docs/archived-plans/` 中已归档的历史计划
- 审计以静态验证为主，不强制运行完整 `cargo test --workspace`

---

## Key Decisions

- **全面审计而非抽查**：用户要求逐单元核对代码库，接受较高时间成本换取结果可靠性
- **汇总为一份新计划**：便于总览剩余工作；单份文档偏大是已知取舍
- **仅原地更新、不归档**：保留 `docs/plans/` 完整历史视图，用户明确选择
- **静态验证为主**：避免长时间测试套件运行；仅在关键争议点做针对性运行验证
- **Deferred 项纳入**：延期项属于真实未完成工作，纳入新计划避免遗漏

---

## Dependencies / Assumptions

- 代码库状态以当前工作区 HEAD 为审计基准
- 沿用现有 frontmatter `status` 字段约定（如 `completed`）

---

## Outstanding Questions

### Resolve Before Planning

（无）

### Deferred to Planning

- [Affects R3][Technical] 部分完成状态的具体标记值（如 `partial` / `in-progress`）需在执行时确定并在全部文档间统一
- [Affects R1][Needs research] 个别单元可能需要运行针对性测试才能判定（如行为对比测试的条件执行是否生效）
