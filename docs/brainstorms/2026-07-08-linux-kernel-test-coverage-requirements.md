---
date: 2026-07-08
topic: linux-kernel-test-coverage
---

# Linux Kernel 测试覆盖率全面改进

## Summary

围绕 Linux 内核现有 KUnit 和 kselftest 框架，以"测试策略文档先行、子系统 KUnit 补全、CI 基础能力"三步组合推进测试覆盖率改善。优先填补 `net/`（当前 KUnit 为 0）和 `fs/`（当前 KUnit 稀疏）的覆盖空白，输出可复用的测试策略规范和可自动化执行的持续测试基础设施。

---

## Problem Frame

当前 Linux 内核已有两套成熟的测试框架（KUnit 和 kselftest），但覆盖分布极不均衡。`kernel/`、`mm/`、`drivers/` 等子系统的 KUnit 测试已较为完善，而 `net/` 完全依赖 kselftest（无任何在树 KUnit），`fs/` 仅覆盖了通用 VFS 和少数文件系统。同时，项目缺少统一的测试策略文档、覆盖率度量流程和自动化 CI，导致新增测试缺乏选型标准和验证闭环。改善测试覆盖需要先建立策略和标准，再按标准补齐关键子系统，并配合基础设施确保新增测试持续有效。

---

## Requirements

**测试策略文档**
- R1. 定义 KUnit vs kselftest 的选型准则，明确各自适用场景和边界
- R2. 定义各子系统的覆盖优先级（核心数据结构、关键路径的定性排序），不设数值硬性指标
- R3. 提供新增测试的 checklist，包括命名规范、配置选项、运行方式等
- R4. 定义覆盖率度量的方法（kcov/gcov）和度量方向（如优先覆盖 net_device 和 neighbour 等核心数据结构），不设数值硬性指标

**子系统 KUnit 补全**
- R5. 为 `net/` 子系统补充 KUnit 测试，覆盖核心数据结构（如 `struct net_device`、socket、路由表、neighbour 等）
- R6. 为 `fs/` 子系统中缺失 KUnit 的主要文件系统补充测试
- R7. 所有新增 KUnit 测试遵循现有框架约定（`-test.c` 文件，如 foo-test.c）

**CI 基础能力**
- R8. 建立基于 UML 或 QEMU 的自动化测试流水线，可自动运行 KUnit + kselftest
- R9. 测试结果以标准格式（TAP）输出，可被工具解析

---

## Success Criteria

- 测试策略文档输出并被审阅通过，可被后续贡献者引用为新测试的选型依据
- `net/` 至少新增一个 KUnit 测试套件并成功运行
- `fs/` 至少一个主要文件系统新增 KUnit 测试并成功运行
- 新增的 CI 流程可自动完成构建和测试，输出可读结果

---

## Scope Boundaries

- 不做 kselftest 套件的大规模扩展（现有 93 个套件已覆盖大部分用户态场景）
- 暂不纳入 `tools/testing/orchestrator/` 统一运行器的开发和完善
- 不在本次范围内制定覆盖率硬性指标（数值目标），由策略文档定义度量方法
- 暂不集成模糊测试（syzkaller）
