---
date: 2026-07-07
topic: kernel-branch-coverage-90
---

# Linux Kernel Full-Codebase Branch Coverage ≥90%

## Summary

建立系统化的 Linux 内核分支覆盖率测试工程，覆盖全部代码树（`kernel/`、`mm/`、`fs/`、`net/`、`drivers/`、`arch/*/`、`lib/`、`include/`），通过组合 KUnit、kselftest、syzkaller 和 Fault Injection 多个测试引擎，按子系统分层构建，最终达到审计可接受的分支覆盖率 90%+。

---

## Problem Frame

合规审计方要求 Linux 内核项目提供分支覆盖率 ≥90% 的证明，覆盖全部代码树。当前内核现有测试（KUnit + kselftest）仅覆盖约 15-25% 的行覆盖率，分支覆盖率更低。审计方理解内核规模，不接受豁免或替代指标。项目有预算，需要建立可重复、可审计的测试工程。

---

## Assumptions

*This requirements doc was authored with user confirmation on scope. The items below are technical assumptions that fill implementation gaps — un-validated bets for planning to scrutinize.*

- 审计方可接受的覆盖率测量工具为 gcov 或 kcov，测量结果需可复现、可审计
- 测试框架组合（KUnit + kselftest + syzkaller + Fault Injection）是可达 90% 的最优路径
- 每个子系统的"完成标准"为该子系统分支覆盖率达到 90% 且测试可复现
- 项目需要团队包括：测试工程师、内核开发者、基础设施/自动化工程师
- 测试代码和工具代码放置在现有 `tools/testing/` 目录结构下

---

## Actors

- A1. 合规审计方：要求覆盖率证明，审核测试过程和结果
- A2. 内核测试工程师：编写和维护测试用例
- A3. 内核开发者：配合测试编写，修复测试发现的缺陷
- A4. 基础设施工程师：搭建和维护覆盖率采集、报告流水线
- A5. 项目经理：跟踪里程碑、协调资源、向审计方汇报进度

---

## Key Flows

- F1. 覆盖率测量流水线
  - **Trigger:** 新代码提交或定期审计检查
  - **Actors:** A2, A4
  - **Steps:** 配置内核启用覆盖率采集 → 构建测试环境 → 运行测试套件 → 采集覆盖率数据 → 生成报告 → 存档审计
  - **Outcome:** 可审计的覆盖率报告，标注未覆盖分支
  - **Covered by:** R1, R2, R3, R4

- F2. 子系统测试开发
  - **Trigger:** 进入新的子系统测试阶段
  - **Actors:** A2, A3
  - **Steps:** 分析子系统代码结构 → 识别关键路径和分支 → 设计测试用例 → 编写测试 → 运行并验证覆盖率 → 修复缺陷 → 提交审计
  - **Outcome:** 子系统分支覆盖率达到 90%，测试代码入库
  - **Covered by:** R5, R6, R7, R8

- F3. 审计检查点
  - **Trigger:** 每个子系统完成或季度审计
  - **Actors:** A1, A5
  - **Steps:** 提交覆盖率报告 → 审计方验证测试完整性 → 签署阶段性证明 → 推进下一子系统
  - **Outcome:** 审计方认可或提出补充要求
  - **Covered by:** R9, R10

- F4. 回归防护
  - **Trigger:** 新代码合并到主线
  - **Actors:** A2, A4
  - **Steps:** 自动运行相关子系统测试 → 检查覆盖率是否下降 → 触发告警或阻断合并
  - **Outcome:** 覆盖率不因新代码而下降
  - **Covered by:** R11

---

## Requirements

**[Coverage measurement infrastructure]**

- R1. 建立统一的覆盖率采集流水线，支持 gcov 和 kcov 两种工具，输出标准格式的覆盖率报告（lcov/html 或等效格式）
- R2. 覆盖率报告必须包含分支级别粒度（哪些分支被覆盖、哪些未覆盖），不得仅报告行覆盖率
- R3. 覆盖率数据可复现：相同测试输入在相同环境下运行，覆盖率数字差异不超过 1%
- R4. 覆盖率报告可追溯：每个覆盖率数据点关联到具体的测试用例和代码提交

**[Test engine integration]**

- R5. 统一编排 KUnit、kselftest、syzkaller、Fault Injection 多个测试引擎，支持一键运行全部测试套件
- R6. 每个测试引擎独立可运行，支持单独运行特定子系统的测试
- R7. 测试失败时自动重试机制，区分偶发性失败和确定性失败
- R8. 测试环境可快速重建（从配置到运行 ≤30 分钟）

**[Subsystem coverage rollout]**

- R9. 按子系统顺序推进测试覆盖：`kernel/` → `mm/` → `fs/` → `net/` → `drivers/` → `arch/*/`，每个子系统达到 90% 分支覆盖率后进入下一子系统
- R10. 每个子系统的测试代码通过代码审查后合并，不得合并未达标的测试
- R11. 建立覆盖率回归防护：新代码合并不得导致已达标子系统的覆盖率下降

**[Audit and reporting]**

- R12. 生成季度审计报告，包含整体覆盖率趋势、各子系统覆盖率、未覆盖分支分析
- R13. 审计报告包含测试完整性证明：每个测试用例的执行记录、覆盖率贡献、关联的需求/代码路径
- R14. 支持审计方独立运行测试套件验证覆盖率数据（可重复运行环境）

---

## Acceptance Examples

- AE1. **Covers R1, R2.** 给定配置了 gcov 的内核，当运行完整的测试套件后，覆盖率报告显示每个源文件的每个函数内每个分支的覆盖状态（覆盖/未覆盖/部分覆盖）。
- AE2. **Covers R3.** 给定相同的测试输入和环境，当连续运行 3 次覆盖率采集后，3 次结果的分支覆盖率数字差异 ≤1%。
- AE3. **Covers R9.** 给定 `kernel/` 子系统的测试套件，当运行完成后，`kernel/` 下所有 `.c` 文件的分支覆盖率达到 90%+，然后才进入 `mm/` 子系统。
- AE4. **Covers R14.** 给定审计方提供的空白构建环境，当审计方按照文档独立运行测试套件后，得到的覆盖率数据与项目报告一致（差异 ≤2%）。

---

## Success Criteria

- 全代码树分支覆盖率达到 90%+，审计方可接受
- 每个子系统的测试代码通过代码审查，测试可复现
- 覆盖率数据可审计：报告包含未覆盖分支分析、测试用例关联、执行记录
- 新代码合并不导致已达标子系统覆盖率下降（回归防护有效）
- 项目过程可追溯：每个覆盖率数据点关联到测试用例和代码提交

---

## Scope Boundaries

- 仅关注分支覆盖率，不要求其他覆盖率指标（行覆盖率、函数覆盖率）达标
- 不修改内核源码以提高可测试性，除非测试必需且经过评审
- 不构建新的测试框架，复用现有 KUnit、kselftest、syzkaller、Fault Injection
- 不包括性能测试、基准测试、安全导向的模糊测试（syzkaller 仅用于覆盖率）
- 不包括实时覆盖率仪表盘（审计不要求）
- 测试代码放置在现有 `tools/testing/` 目录结构下

---

## Key Decisions

- 分支覆盖率作为唯一指标：审计方不接受行覆盖率或函数覆盖率作为充分证据
- 全代码树无豁免：审计方理解内核规模但不接受豁免
- 分层构建法：按子系统顺序推进，每个子系统达标后再进入下一子系统，确保质量可控
- 多引擎组合：单一工具无法达到 90%，需要 KUnit + kselftest + syzkaller + Fault Injection 组合

---

## Dependencies / Assumptions

- 审计方可接受的覆盖率测量工具为 gcov 或 kcov，测量结果需可复现、可审计
- 测试框架组合（KUnit + kselftest + syzkaller + Fault Injection）是可达 90% 的最优路径
- 项目需要团队包括：测试工程师、内核开发者、基础设施/自动化工程师
- 测试代码和工具代码放置在现有 `tools/testing/` 目录结构下

---

## Outstanding Questions

### Resolve Before Planning

- 审计方是否有覆盖率报告的格式要求（HTML、XML、自定义格式）？
- 审计方是否要求在覆盖率计算中排除某些文件类型（如 `*.lds` 链接脚本、生成的头文件）？
- 项目的时间期望是什么？是否有硬截止日期？

### Deferred to Planning

- [Needs research] 各子系统的基线覆盖率是多少？需要先运行现有测试套件测量
- [Needs research] 哪些子系统最难覆盖（如 `arch/` 下的特定架构代码），需要多少定制化工作？
- [Technical] 覆盖率采集对内核性能的影响有多大？是否需要采样模式？
- [Technical] 如何处理条件编译（`#ifdef`）导致的代码路径差异？
- [Needs research] 现有 KUnit 和 kselftest 的测试用例中，有多少可以复用于覆盖率测量，有多少需要重写？
