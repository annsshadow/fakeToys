---
title: feat: Multi-engine full-codebase branch coverage 鈮?0%
type: feat
status: active
date: 2026-07-08
origin: docs/brainstorms/2026-07-08-multi-engine-full-coverage-requirements.md
---

# Linux Kernel Multi-Engine Full-Codebase Branch Coverage 鈮?0%

## Summary

在现KUnit、kselftest、覆盖率工具链和 GitLab CI 流水线基础上，扩展纳入 syzkaller Fault Injection 作为补充测试引擎，建立全代码树同步推进的分支覆盖率测试工程。通过"基线测量先行、四引擎组合覆盖、CI 矩阵扩展、审计就绪报四步，达到审计方可接受的分支覆盖0%
---

## Problem Frame

合规审计方要Linux 内核项目提供全代码树分支覆盖0% 的证明，不接受豁免或替代指标。当前内核已KUnit、kselftest 两套测试框架，以gcov/kcov 覆盖率工具和 GitLab CI 流水线，但覆盖分布极不均衡，且缺syzkaller Fault Injection 的系统化运用。项目没有现成的全代码树分支覆盖率基线数据，无法评估当前缺口和制定精准推进计划。改善测试覆盖需要先建立可复现的基线测量，再按优先级组织四引擎组合推进，并配合审计就绪的报告和回归防护确保新增测试持续有效
---

## Requirements

**Origin actors:** A1（合规审计方 A2（内核测试工程师 A3（内核开发者）, A4（基础设施工程师）, A5（项目经理）

**Origin flows:** F1（基线测量与缺口分析 F2（四引擎组合测试开发）, F3（审计检查点 F4（回归防护）

**Origin acceptance examples:** AE1（R1, R2 AE2（R3 AE3（R5 AE4（R9 AE5（R14
- R1. 建立统一的覆盖率采集流水线，支持 gcov kcov 两种工具，输出标准格式的覆盖率报告（lcov/html 或等效格式）
- R2. 覆盖率报告必须包含分支级别粒度（哪些分支被覆盖、哪些未覆盖），不得仅报告行覆盖- R3. 覆盖率数据可复现：相同测试输入在相同环境下运行，覆盖率数字差异不超过 1%
- R4. 覆盖率报告可追溯：每个覆盖率数据点关联到具体的测试用例和代码提交
- R5. 统一编排 KUnit、kselftest、syzkaller、Fault Injection 四个测试引擎，支持一键运行全部测试套- R6. 每个测试引擎独立可运行，支持单独运行特定子系统的测试
- R7. 测试失败时自动重试机制，区分偶发性失败和确定性失- R8. 测试环境可快速重建（从配置到运行 0 分钟- R9. 全代码树同步推进测试覆盖：`kernel/`、`mm/`、`fs/`、`net/`、`drivers/`、`arch/*/`、`lib/`、`include/` 同时建设，不接受分阶段豁- R10. 每个子系统的测试代码通过代码审查后合并，确保新增测试确实提升目标覆盖- R11. 建立覆盖率回归防护：新代码合并不得导致整体覆盖率下降
- R12. 生成审计就绪报告，包含整体覆盖率趋势、各子系统覆盖率、未覆盖分支分析
- R13. 审计报告包含测试完整性证明：每个测试用例的执行记录、覆盖率贡献、关联的代码路径
- R14. 支持审计方独立运行测试套件验证覆盖率数据（可重复运行环境
---

## Scope Boundaries

- 仅关注分支覆盖率，不要求其他覆盖率指标（行覆盖率、函数覆盖率）达- 不修改内核源码以提高可测试性，除非测试必需且经过评- 不构建新的测试框架，复用现有 KUnit、kselftest、syzkaller、Fault Injection
- 不包括性能测试、基准测试、安全导向的模糊测试（syzkaller 仅用于覆盖率- 不包括实时覆盖率仪表盘（审计不要求）
- 测试代码放置在现`tools/testing/` 目录结构
---

## Context & Research

### Relevant Code and Patterns

**四引擎编排器已存*（`tools/testing/orchestrator/`）：
- `test_orchestrator.py` 统一入口，支`--engines`、`--subsystem`、`--retry`、`--coverage` 参数
- `base_runner.py` 抽象基类，提`run_with_retry` `SuiteResult` 数据结构
- `kunit_runner.py` 包装 `tools/testing/kunit/kunit.py`，解TAP 输出
- `kselftest_runner.py` 包装 `tools/testing/selftests/`，通过 `make TARGETS=<subsystem>` 运行
- `syzkaller_runner.py` **stub**：目录不存在时返SKIP，`collect_coverage` 返回dict
- `faultinj_runner.py` 通过 debugfs 启用/禁用故障注入，但**不实际运行子系统测试**
- `uml_runner.py` / `qemu_runner.py` 环境 runner

**覆盖率工具链已存*（`tools/testing/coverage/`）：
- `coverage_harness.py` 主入口，支持 gcov/kcov 配置、构建、测试执行、数据收集、报告生- `gcov_parser.py` / `kcov_parser.py` 解析- `report_generator.py` lcov/html 报告生成

**审计工具链已存在**（`tools/testing/audit/`）：
- `coverage_regression.py` 回归检- `report_generator.py` 审计报告生成

**CI 流水线已存在**（`.gitlab-ci-coverage.yml`）：
- 4 阶段：build test coverage audit
- 当前 test 阶段仅运行：kunit、net_core_kunit、fs_super_kunit、kselftest
- coverage 阶段调用 `coverage_harness.py`
- audit 阶段调用 `report_generator.py` `coverage_regression.py`

**现有 KUnit 测试分布**- `kernel/` 个测试文件（sysctl-test.c、kallsyms_selftest.c、crash_core_test.c、backtracetest.c- `mm/`0+ 个测试文件（page_alloc、vma、vmalloc、swap、shmem 等）
- `fs/` 个测试文件（super、inode、dcache、namei、readdir、statfs、libfs、ext4- `net/` 个测试文件（core_kunit_test.c、socket_kunit_test.c- `drivers/`：多个测试文件（i2c、gpio、clk、tty、spi 等）

**现有 kselftest 分布**- `tools/testing/selftests/` 90+ 个子系统目录

**Fault Injection 框架**- 内核内置：`CONFIG_FAULT_INJECTION`、`CONFIG_FAULT_INJECTION_DEBUG_FS`
- debugfs 接口：`/sys/kernel/debug/failslab`、`/sys/kernel/debug/fail_page_alloc`
- 工具：`tools/testing/fault-injection/failcmd.sh`

**syzkaller 状*- `tools/testing/syzkaller/` **不存*
- `syzkaller_runner.py` stub，始终返SKIP

### Institutional Learnings

`docs/solutions/` 目录不存在，`STRATEGY.md` 不存在。本次工作属于该仓库首次系统化四引擎测试覆盖工程
### External References

- `Documentation/dev-tools/testing-strategy.rst` 测试策略文档（KUnit/kselftest 选型、优先级、checklist- `Documentation/dev-tools/testing-overview.rst` 框架概述
- `Documentation/fault-injection/fault-injection.rst` 内核故障注入框架文档
- `tools/testing/selftests/kselftest_harness.h` 鈥?kselftest  harness

---

## Key Technical Decisions

- **以现orchestrator 为基础扩展**：`tools/testing/orchestrator/` 已有四引擎适配器骨架，补全 syzkaller Fault Injection stub 实现，而非重建
- **syzkaller 采用外部部署模式**：`tools/testing/syzkaller/` 目录不存在，通过外部 syzkaller 仓库部署，orchestrator 通过配置文件路径关联
- **Fault Injection 作为测试修饰*：不单独运行，而是KUnit/kselftest 执行期间启用故障注入，复用现有测试套- **覆盖率合并策*：gcov 用于 KUnit kselftest 的全局覆盖率，kcov 用于 syzkaller per-task 覆盖率，合并时以 gcov 为主
- **基线测量先行**：在写任何新测试前，先用现有 gcov + 现有测试套件跑出全代码树分支覆盖率基- **CI 扩展而非重建**：`.gitlab-ci-coverage.yml` 已有 4 阶段结构，在其上扩展 job 矩阵

---

## Open Questions

### Resolved During Planning

- syzkaller 集成方式：`tools/testing/syzkaller/` 不存在，采用外部部署 + 配置路径关联
- Fault Injection 运行模式：作KUnit/kselftest 的修饰器，在测试执行期间启用 debugfs 故障注入
- 覆盖率合并策略：gcov 为主（KUnit/kselftest），kcov 为辅（syzkaller per-task
### Deferred to Implementation

- 基线覆盖率具体数值：需U1 运行测量后获- 各子系统缺口优先级排序：需U1 基线报告后根据实际数据确- syzkaller 具体配置（corpus 目录、poc 数量、超时）：需U2 实现时根据环境调- 条件编译（`#ifdef`）导致的覆盖率差异处理策略：需U1 数据分析后确
---

## Implementation Units

### U1. 基线测量与缺口分
**Goal:** 建立可复现的全代码树分支覆盖率基线，识别各子系统缺口分布

**Requirements:** R1, R2, R3, R4, R9

**Dependencies:** None

**Files:**
- Create: `tools/testing/coverage/baseline_report.json`
- Create: `tools/testing/coverage/baseline/`（存储基线数据）
- Modify: `tools/testing/coverage/coverage_harness.py`（如需要）

**Approach:**
- 配置内核启用 gcov 覆盖率采集（`CONFIG_GCOV_KERNEL=y`、`CONFIG_GCOV_PROFILE_ALL=y`- 构建测试环境（UML QEMU- 运行现有 KUnit + kselftest 套件
- 采集全代码树分支覆盖率数- 生成基线报告，按子系统（`kernel/`、`mm/`、`fs/`、`net/`、`drivers/`、`arch/*/`、`lib/`、`include/`）标注缺口分- 验证覆盖率数据可复现性（连续运行 3 次，差异 %
**Patterns to follow:**
- `tools/testing/coverage/coverage_harness.py` 覆盖率收集入- `tools/testing/coverage/gcov_parser.py` gcov 数据解析
- `tools/testing/audit/report_generator.py` 报告生成

**Test scenarios:**
- Happy path: gcov 配置后构建成功，覆盖率数据采集完成，报告生成
- Edge case: 某些子系统无任何测试时，基线报告正确标注0%
- Error path: gcov 配置失败时，harness 返回明确错误而非崩溃
- Integration: 基线数据可被 `coverage_regression.py` 消费

**Verification:**
- `tools/testing/coverage/coverage_harness.py` 成功生成全代码树基线报告
- 报告包含每个子系统的分支覆盖率数字和未覆盖分支列- 连续 3 次运行结果差%

---

### U2. syzkaller 集成部署

**Goal:** 完成 syzkaller_runner.py stub 实现，集syzkaller orchestrator

**Requirements:** R5, R6

**Dependencies:** U1

**Files:**
- Create: `tools/testing/syzkaller/`（部syzkaller 二进制和配置模板- Modify: `tools/testing/orchestrator/syzkaller_runner.py`（完善实现）
- Modify: `tools/testing/orchestrator/test_orchestrator.py`（如果需要）

**Approach:**
- `tools/testing/syzkaller/` 下部syzkaller 二进制（或提供下载脚本）
- 提供默认配置文件模板（`tools/testing/syzkaller/cfg/`- 完善 `syzkaller_runner.py`  - `configure()`：为指定子系统选择或生syzkaller 配置
  - `build()`：构syzkaller 目标（如果源码存在）
  - `run()`：启syzkaller fuzzing 会话，支持超时控  - `collect_coverage()`：收kcov 覆盖率数- `test_orchestrator.py` 中确syzkaller 引擎可被 `--engines` 参数选中

**Patterns to follow:**
- `tools/testing/orchestrator/syzkaller_runner.py` 鈥，鐜版湁 stub 缁撴瀯
- `tools/testing/orchestrator/base_runner.py` 基类接口
- `tools/testing/orchestrator/kunit_runner.py` 成熟 runner 参
**Test scenarios:**
- Happy path: syzkaller 目录存在时，runner 成功启动 fuzzing 会话
- Edge case: syzkaller 目录不存在时，优雅降级为 SKIP
- Error path: syzkaller 配置无效时，返回 ERROR 而非崩溃
- Integration: test_orchestrator.py `--engines syzkaller` 参数正常工作

**Verification:**
- `python tools/testing/orchestrator/test_orchestrator.py --engines syzkaller --subsystem net` 正常运行
- syzkaller 配置模板可被 fuzzing 会话使用

---

### U3. Fault Injection 测试扩展

**Goal:** 完善 faultinj_runner.py，使其能实际运行子系统测试并注入故障

**Requirements:** R5, R6

**Dependencies:** U1

**Files:**
- Modify: `tools/testing/orchestrator/faultinj_runner.py`
- Create: `tools/testing/fault-injection/subsystem_profiles/`（各子系统故障注入配置）

**Approach:**
- 完善 `faultinj_runner.py`  - `run()`：不再仅返回 PASS，而是实际运行目标子系统的 KUnit/kselftest 套件，同时启用故障注  - 支持配置故障注入概率和类型（slab、page_alloc 等）
  - 收集故障注入触发的测试结- 创建子系统配置文件：
  - `tools/testing/fault-injection/subsystem_profiles/net.yaml`
  - `tools/testing/fault-injection/subsystem_profiles/fs.yaml`
  - `tools/testing/fault-injection/subsystem_profiles/mm.yaml`
- 每个配置文件定义：目标内核模块、故障类型、注入概率、预期测
**Patterns to follow:**
- `tools/testing/orchestrator/faultinj_runner.py` 鈥，鐜版湁缁撴瀯
- `Documentation/fault-injection/fault-injection.rst` 故障注入框架文档
- `tools/testing/fault-injection/failcmd.sh` 命令行工具参
**Test scenarios:**
- Happy path: 运行 net/ 子系统的 KUnit 测试时，slab 故障注入按配置概率触- Edge case: 故障注入概率0 时，测试行为与无故障注入一- Error path: debugfs 不可用时，runner 返回 SKIP 而非崩溃
- Integration: fault injection 结果KUnit/kselftest 结果合并到同一 SuiteResult

**Verification:**
- `python tools/testing/orchestrator/test_orchestrator.py --engines fault_injection --subsystem net` 正常运行
- 故障注入期间至少有一个测试因注入的故障而失败（证明注入生效
---

### U4. 四引擎覆盖率数据合并

**Goal:** 完善 test_orchestrator.py `--coverage` 实现，支持四引擎覆盖率数据合并输
**Requirements:** R1, R2, R5

**Dependencies:** U2, U3

**Files:**
- Modify: `tools/testing/orchestrator/test_orchestrator.py`
- Modify: `tools/testing/orchestrator/base_runner.py`（如需要）
- Modify: `tools/testing/coverage/coverage_harness.py`（如需要）

**Approach:**
- 实现 `test_orchestrator.py` `--coverage` 功能  - 每个 engine runner 运行后调`collect_coverage()`
  - 合并四引擎的覆盖率数据（gcov 为主，kcov 为辅  - 去重：同一代码路径被多个引擎覆盖时，只计一- 扩展 `base_runner.py` `SuiteResult`  - 添加 `coverage_summary` 字段
  - 提供覆盖率合并方- 输出标准格式报告（lcov info 文件 + HTML 摘要
**Patterns to follow:**
- `tools/testing/coverage/coverage_harness.py` 覆盖率收集和报告生成
- `tools/testing/coverage/gcov_parser.py` gcov 数据解析
- `tools/testing/orchestrator/base_runner.py` 鈥?SuiteResult 鏁版嵁缁撴瀯

**Test scenarios:**
- Happy path: `--coverage` 运行时，四引擎数据合并输出为单个 lcov 文件
- Edge case: 某引擎返回空覆盖率时，合并逻辑正确处理
- Error path: 覆盖率数据格式异常时，harness 返回明确错误
- Integration: 合并后的覆盖率数据可`report_generator.py` 生成 HTML 报告

**Verification:**
- `python tools/testing/orchestrator/test_orchestrator.py --engines kunit kselftest --coverage` 成功生成合并覆盖率报- 报告包含分支级别粒度的覆盖状
---

### U5. CI 全代码树测试矩阵扩展

**Goal:** 扩展 `.gitlab-ci-coverage.yml`，支持全代码KUnit + kselftest + syzkaller + Fault Injection 矩阵

**Requirements:** R8, R9, R12

**Dependencies:** U1, U2, U3, U4

**Files:**
- Modify: `.gitlab-ci-coverage.yml`
- Create: `tools/testing/ci/full_matrix.yml`（如需要）

**Approach:**
- 扩展 test 阶段  - 添加所有现KUnit 子系统的专用 job（kernel/、mm/、fs/、net/、drivers/  - 添加 kselftest 全量运行 job
  - 添加 syzkaller job（标记为 `allow_failure: true`，因syzkaller 需VM  - 添加 fault injection job
- 扩展 coverage 阶段  - 集成四引擎覆盖率合并
  - 生成按子系统的覆盖率报告
- 保持 audit 阶段不变（复用现`coverage_regression.py`
**Patterns to follow:**
- `.gitlab-ci-coverage.yml` 鈥，鐜版湁 stages 鍜?extends 妯℃澘
- `tools/testing/orchestrator/test_orchestrator.py` CLI 接口

**Test scenarios:**
- Happy path: CI pipeline 完整执行 build test coverage audit 四阶- Edge case: syzkaller job 失败时不影响整体 pipeline（`allow_failure: true`- Error path: 某子系统 KUnit 测试失败时，coverage 阶段仍运行但标记 FAIL
- Integration: 覆盖率报告成功生成并作为 artifact 保存

**Verification:**
- `.gitlab-ci-coverage.yml` 通过 GitLab CI lint 检- 新增job GitLab CI 中正确触
---

### U6. 子系KUnit 测试扩展

**Goal:** 按优先级补充各子系统KUnit 测试，提升分支覆盖率

**Requirements:** R9, R10

**Dependencies:** U1（基线测量完成后，根据缺口确定优先级
**Files:**
- Modify: `net/core_kunit_test.c`（扩net_device 测试- Modify: `net/socket_kunit_test.c`（扩socket 测试- Modify: `fs/super_kunit_test.c`（扩super_block 测试- Modify: `fs/inode_kunit_test.c`（扩inode 测试- Modify: `kernel/sysctl-test.c`（扩sysctl 测试- Modify: `mm/page_alloc_kunit_test.c`（扩page alloc 测试- 可能新增：`net/neighbour_kunit_test.c`、`fs/dentry_kunit_test.c` 
**Approach:**
- 基于 U1 的基线报告，确定各子系统的缺口优先级
- 优先覆盖 P1 核心数据结构（net_device、super_block、inode、mm_struct、task_struct- 每个新测试遵循现`-test.c` 命名约定`kunit_test_suite()` 注册模式
- 所有测试通过 `scripts/checkpatch.pl` `scripts/spdxcheck.py` 检
**Patterns to follow:**
- `net/core_kunit_test.c` 现有 net/ KUnit 测试模式
- `fs/inode_kunit_test.c` inode 生命周期测试参- `mm/page_alloc_kunit_test.c` 复杂结构初始化测试参
**Test scenarios:**
- Happy path: 新增测试`make kunit` PASS
- Edge case: 边界条件（空指针、零大小、最大值）正确触发 EXPECT/ASSERT
- Error path: 错误输入触发 EXPECT_FAIL 而非崩溃
- Integration: 新测试在 CI 中自动运行并通过

**Verification:**
- `make O=build kunit` 运行并通过所有新增测- 新测试在 TAP 输出中显PASS
- 分支覆盖率基线报告显示对应子系统覆盖率提
---

### U7. 审计就绪报告与回归防
**Goal:** 建立审计就绪报告格式和覆盖率回归防护机制

**Requirements:** R11, R12, R13, R14

**Dependencies:** U4, U5

**Files:**
- Modify: `tools/testing/audit/report_generator.py`
- Modify: `tools/testing/audit/coverage_regression.py`
- Create: `tools/testing/audit/audit_readme.md`（审计方运行指南
**Approach:**
- 扩展 `report_generator.py`  - 生成审计就绪报告（Markdown + HTML），包含整体覆盖率趋势、各子系统覆盖率、未覆盖分支分析
  - 每个测试用例关联覆盖率贡- 扩展 `coverage_regression.py`  - 支持全代码树回归检  - 新代码合并时自动检查是否导致覆盖率下降
- 创建审计方运行指南：
  - 环境准备步骤
  - 独立运行测试套件的方  - 验证覆盖率数据的步骤

**Patterns to follow:**
- `tools/testing/audit/report_generator.py` 现有报告生成
- `tools/testing/audit/coverage_regression.py` 现有回归检
**Test scenarios:**
- Happy path: 审计方按照指南独立运行测试，得到的覆盖率数据与项目报告一致（差异 %- Edge case: 基线数据缺失时，报告提示需要先运行基线测量
- Error path: 覆盖率下降时，regression 检测正确标记并阻止合并
- Integration: 报告数据可被审计方解析和验证

**Verification:**
- `tools/testing/audit/report_generator.py` 成功生成审计就绪报告
- `tools/testing/audit/coverage_regression.py` 正确检测覆盖率回归
- 审计方可在空白环境中复现测试结果

---

## System-Wide Impact

- **Interaction graph:** U2 U3 新增engine runner `test_orchestrator.py` 调用；U4 的覆盖率合并逻辑影响所engine runner 的输出格式；U5 CI 矩阵merge request 时自动触- **Error propagation:** CI 流水线的 test 阶段失败阻止 coverage audit 阶段执行（现stages 依赖关系）；syzkaller job 标记`allow_failure`，不阻断 pipeline
- **State lifecycle risks:** syzkaller 需要外VM/QEMU 环境，CI 中可能不可用；Fault Injection 需debugfs 挂载，UML 环境可能有限- **API surface parity:** 新增测试不修改任何内API，仅增加测试覆盖
- **Integration coverage:** U4 的覆盖率合并需验证gcov + kcov 混合场景下的准确- **Unchanged invariants:** 现有 KUnit/kselftest 的测试接口和 CI 阶段结构保持不变

---

## Risks & Dependencies

| Risk | Likelihood | Impact | Mitigation |
|------|-----------|--------|------------|
| syzkaller 目录不存在，需外部部署 | High | Medium | U2 提供下载脚本和配置模板，CI 中标记为 allow_failure |
| Fault Injection UML 环境debugfs 不可| Medium | Medium | U3 检debugfs 可用性，不可用时优雅降级SKIP |
| 基线测量时间过长（全代码树构+ 测试| High | Low | 使用 `make O=build` 树外构建，利ccache 加速重复构|
| 覆盖率数据合并不一致（gcov vs kcov| Medium | High | U4 gcov 为主，kcov 为辅，合并时去重 |
| 全代码树同步推进资源消耗大 | High | High | U1 基线测量后，根据实际缺口调整资源分配优先|
| 审计方对报告格式有额外要| Medium | High | U7 提供多种格式输出（Markdown、HTML、JSON），预留自定义接|

---

## Documentation / Operational Notes

- U1 产出物为基线覆盖率报告，按子系统分类
- U2 产出物为 syzkaller 部署指南和配置模- U3 产出物为 fault injection  subsystem profiles
- U7 产出物为审计方运行指- 所有测试代码需通过 `scripts/checkpatch.pl` `scripts/spdxcheck.py` 检- CI 修改需GitLab CI 中验pipeline 通过

---

## Sources & References

- **Origin document:** [docs/brainstorms/2026-07-08-multi-engine-full-coverage-requirements.md](docs/brainstorms/2026-07-08-multi-engine-full-coverage-requirements.md)
- Related code: [tools/testing/orchestrator/test_orchestrator.py](tools/testing/orchestrator/test_orchestrator.py), [tools/testing/orchestrator/syzkaller_runner.py](tools/testing/orchestrator/syzkaller_runner.py), [tools/testing/orchestrator/faultinj_runner.py](tools/testing/orchestrator/faultinj_runner.py), [tools/testing/coverage/coverage_harness.py](tools/testing/coverage/coverage_harness.py), [.gitlab-ci-coverage.yml](.gitlab-ci-coverage.yml)
- Related docs: [Documentation/dev-tools/testing-strategy.rst](Documentation/dev-tools/testing-strategy.rst), [Documentation/fault-injection/fault-injection.rst](Documentation/fault-injection/fault-injection.rst)
- Existing tests: [net/core_kunit_test.c](net/core_kunit_test.c), [fs/super_kunit_test.c](fs/super_kunit_test.c), [kernel/sysctl-test.c](kernel/sysctl-test.c), [mm/page_alloc_kunit_test.c](mm/page_alloc_kunit_test.c)
