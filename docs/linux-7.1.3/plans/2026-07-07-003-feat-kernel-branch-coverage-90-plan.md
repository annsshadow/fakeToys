---
title: feat: Kernel full-codebase branch coverage ≥90%
type: feat
status: active
date: 2026-07-07
origin: docs/brainstorms/kernel-branch-coverage-90-requirements.md
---

# feat: Kernel Full-Codebase Branch Coverage ≥90%

## Summary

建立系统化的 Linux 内核分支覆盖率测试工程，覆盖全部代码树（`kernel/`、`mm/`、`fs/`、`net/`、`drivers/`、`arch/*/`、`lib/`、`include/`），通过组合 KUnit、kselftest、syzkaller 和 Fault Injection 多个测试引擎，按子系统分层构建，最终达到审计可接受的分支覆盖率 90%+。

---

## Problem Frame

合规审计方要求 Linux 内核项目提供分支覆盖率 ≥90% 的证明，覆盖全部代码树。当前内核现有测试（KUnit + kselftest）仅覆盖约 15-25% 的行覆盖率，分支覆盖率更低。审计方理解内核规模，不接受豁免或替代指标。项目有预算，需要建立可重复、可审计的测试工程。

---

## Requirements

**Origin actors:** A1 (合规审计方), A2 (内核测试工程师), A3 (内核开发者), A4 (基础设施工程师), A5 (项目经理)
**Origin flows:** F1 (覆盖率测量流水线), F2 (子系统测试开发), F3 (审计检查点), F4 (回归防护)
**Origin acceptance examples:** AE1 (Covers R1, R2), AE2 (Covers R3), AE3 (Covers R9), AE4 (Covers R14)

- R1. 建立统一的覆盖率采集流水线，支持 gcov 和 kcov 两种工具，输出标准格式的覆盖率报告（lcov/html 或等效格式）
- R2. 覆盖率报告必须包含分支级别粒度（哪些分支被覆盖、哪些未覆盖），不得仅报告行覆盖率
- R3. 覆盖率数据可复现：相同测试输入在相同环境下运行，覆盖率数字差异不超过 1%
- R4. 覆盖率报告可追溯：每个覆盖率数据点关联到具体的测试用例和代码提交
- R5. 统一编排 KUnit、kselftest、syzkaller、Fault Injection 多个测试引擎，支持一键运行全部测试套件
- R6. 每个测试引擎独立可运行，支持单独运行特定子系统的测试
- R7. 测试失败时自动重试机制，区分偶发性失败和确定性失败
- R8. 测试环境可快速重建（从配置到运行 ≤30 分钟）
- R9. 按子系统顺序推进测试覆盖：`kernel/` → `mm/` → `fs/` → `net/` → `drivers/` → `arch/*/`，每个子系统达到 90% 分支覆盖率后进入下一子系统
- R10. 每个子系统的测试代码通过代码审查后合并，不得合并未达标的测试
- R11. 建立覆盖率回归防护：新代码合并不得导致已达标子系统的覆盖率下降
- R12. 生成季度审计报告，包含整体覆盖率趋势、各子系统覆盖率、未覆盖分支分析
- R13. 审计报告包含测试完整性证明：每个测试用例的执行记录、覆盖率贡献、关联的需求/代码路径
- R14. 支持审计方独立运行测试套件验证覆盖率数据（可重复运行环境）

---

## Scope Boundaries

- 仅关注分支覆盖率，不要求其他覆盖率指标（行覆盖率、函数覆盖率）达标
- 不修改内核源码以提高可测试性，除非测试必需且经过评审
- 不构建新的测试框架，复用现有 KUnit、kselftest、syzkaller、Fault Injection
- 不包括性能测试、基准测试、安全导向的模糊测试（syzkaller 仅用于覆盖率）
- 不包括实时覆盖率仪表盘（审计不要求）
- 测试代码放置在现有 `tools/testing/` 目录结构下

---

## Context & Research

### Relevant Code and Patterns

- `lib/kunit/` — KUnit 框架实现（test.c, executor.c, assert.c），测试注册模式为 `kunit_test_suites()`
- `tools/testing/kunit/kunit.py` — KUnit Python 运行器，支持 UML/QEMU
- `tools/testing/selftests/` — kselftest 目录，123 个测试子目录，使用 `kselftest_harness.h`
- `kernel/gcov/` — gcov 覆盖率采集实现（base.c, fs.c, gcc_4_7.c, clang.c）
- `lib/Kconfig.debug` — KCOV 配置（`CONFIG_KCOV`、`CONFIG_KCOV_INSTRUMENT_ALL`）
- `lib/fault-inject.c` — Fault Injection 框架（`should_fail()` / `should_fail_ex()`）
- `drivers/gpu/drm/ci/kunit.yml` — GitLab CI KUnit 测试示例
- `tools/testing/selftests/kselftest/runner.sh` — kselftest 运行器

### Institutional Learnings

无直接相关的 `docs/solutions/` 条目。

### External References

- gcov 文档：`Documentation/dev-tools/gcov.rst`
- kcov 文档：`Documentation/dev-tools/kcov.rst`
- Fault Injection 文档：`Documentation/fault-injection/fault-injection.rst`
- KUnit 文档：`Documentation/dev-tools/kunit/index.rst`
- kselftest 文档：`Documentation/dev-tools/kselftest.rst`

---

## Key Technical Decisions

- 分支覆盖率作为唯一指标：审计方不接受行覆盖率或函数覆盖率作为充分证据 (see origin: docs/brainstorms/kernel-branch-coverage-90-requirements.md)
- 全代码树无豁免：审计方理解内核规模但不接受豁免 (see origin)
- 分层构建法：按子系统顺序推进，每个子系统达标后再进入下一子系统，确保质量可控 (see origin)
- 多引擎组合：单一工具无法达到 90%，需要 KUnit + kselftest + syzkaller + Fault Injection 组合 (see origin)
- 覆盖率采集方案：gcov 为主（提供行/分支粒度），kcov 为辅（用于 fuzzing 场景），统一输出为 lcov 格式
- 测试编排：扩展 `tools/testing/kunit/kunit.py` 作为统一入口，集成 syzkaller 和 kselftest

---

## Open Questions

### Resolved During Planning

- 覆盖率测量工具选择：gcov（GCC 内置，支持分支分析）+ kcov（Clang sanitizer，用于 fuzzing）
- 测试编排入口：扩展 `tools/testing/kunit/kunit.py` 作为统一运行器

### Deferred to Implementation

- 各子系统的基线覆盖率：需要先运行现有测试套件测量
- 哪些子系统最难覆盖（如 `arch/` 下的特定架构代码）：需要实际测量后评估
- 覆盖率采集对内核性能的影响：需要基准测试验证
- 条件编译（`#ifdef`）导致的代码路径差异：需要在实现时处理
- 现有测试用例中有多少可以复用：需要逐子系统审计

---

## Implementation Units

### U1. 建立覆盖率采集基础设施

**Goal:** 建立统一的覆盖率采集流水线，支持 gcov 和 kcov，输出分支级别覆盖率报告。

**Requirements:** R1, R2, R3, R4

**Dependencies:** 无

**Files:**
- Create: `tools/testing/coverage/coverage_harness.py` — 覆盖率采集主脚本
- Create: `tools/testing/coverage/gcov_parser.py` — gcov 数据解析模块
- Create: `tools/testing/coverage/kcov_parser.py` — kcov 数据解析模块
- Create: `tools/testing/coverage/report_generator.py` — lcov/HTML 报告生成模块
- Create: `tools/testing/coverage/configs/` — 覆盖率采集所需的内核配置片段
- Modify: `tools/testing/kunit/kunit.py` — 集成覆盖率采集参数

**Approach:**
- 扩展 `tools/testing/kunit/kunit.py`，添加 `--coverage` 参数，自动配置内核启用 gcov/kcov
- 新建 `tools/testing/coverage/` 目录，包含覆盖率数据解析和报告生成工具
- 覆盖率采集流程：配置内核（启用 `CONFIG_GCOV_KERNEL` + `CONFIG_GCOV_PROFILE_ALL`）→ 构建 → 运行测试 → 从 debugfs 采集数据 → 解析 → 生成 lcov 报告
- 支持分支覆盖率模式：GCC 的 `--coverage` 配合 lcov 的 `--rc lcov_branch_coverage=1`
- 输出格式：lcov（用于详细分析）+ HTML（用于审计展示）

**Patterns to follow:**
- `tools/testing/kunit/kunit.py` 的 argparse + 子命令模式
- `kernel/gcov/fs.c` 的 debugfs 导出机制
- `tools/testing/kunit/configs/` 的配置片段组织方式

**Test scenarios:**
- Happy path: 配置内核启用 gcov，运行 KUnit 测试套件，成功生成 lcov 报告
- Happy path: 配置内核启用 kcov，运行 syzkaller 会话，成功采集覆盖率数据
- Edge case: 内核配置中部分文件禁用 gcov（通过 `GCOV_PROFILE_*.o := n`），报告正确反映排除文件
- Error path: debugfs 未挂载时，脚本自动挂载或报错退出
- Error path: 覆盖率数据文件损坏时，脚本跳过损坏文件并继续处理
- Integration: 覆盖率采集与实际测试运行流水线集成，测试结果和覆盖率数据关联

**Verification:**
- `tools/testing/coverage/` 目录下的脚本可以独立运行
- 运行 `python tools/testing/kunit/kunit.py run --coverage` 可以生成有效的 lcov 报告
- lcov 报告中包含分支覆盖率数据（`BRDA` 记录）
- 相同测试输入运行两次，分支覆盖率数字差异 ≤1%

---

### U2. 建立统一测试编排框架

**Goal:** 将 KUnit、kselftest、syzkaller、Fault Injection 统一编排为可一键运行的测试套件。

**Requirements:** R5, R6, R7, R8

**Dependencies:** U1

**Files:**
- Create: `tools/testing/orchestrator/test_orchestrator.py` — 统一测试编排器
- Create: `tools/testing/orchestrator/kunit_runner.py` — KUnit 运行适配器
- Create: `tools/testing/orchestrator/kselftest_runner.py` — kselftest 运行适配器
- Create: `tools/testing/orchestrator/syzkaller_runner.py` — syzkaller 运行适配器
- Create: `tools/testing/orchestrator/faultinj_runner.py` — Fault Injection 运行适配器
- Create: `tools/testing/orchestrator/configs/` — 各引擎的配置模板

**Approach:**
- 扩展 `tools/testing/kunit/kunit.py`，添加 `run_all` 子命令，按子系统顺序运行全部测试引擎
- 每个测试引擎通过适配器封装，提供统一的接口：`configure()` → `build()` → `run()` → `collect_coverage()`
- 支持单独运行特定子系统的测试：`--subsystem mm/`
- 自动重试机制：测试失败时重试最多 3 次，区分偶发性失败（重试后通过）和确定性失败（始终失败）
- 测试环境缓存：使用 `make O=build` 的 out-of-tree 构建，配置和构建结果缓存，支持快速重建

**Patterns to follow:**
- `tools/testing/kunit/kunit.py` 的配置/构建/运行三阶段模式
- `tools/testing/selftests/kselftest/runner.sh` 的 TAP 输出和超时处理
- `drivers/gpu/drm/ci/kunit.sh` 的 CI 集成模式

**Test scenarios:**
- Happy path: `python tools/testing/kunit/kunit.py run_all` 一键运行全部测试引擎
- Happy path: `python tools/testing/kunit/kunit.py run_all --subsystem mm/` 仅运行 mm/ 子系统测试
- Edge case: 某个测试引擎不可用（如 syzkaller 未安装），跳过该引擎并继续运行其他引擎
- Error path: 测试超时时，记录超时信息并继续下一测试
- Error path: 偶发性测试失败时，自动重试并在报告中标注
- Integration: 多个测试引擎的输出合并为统一的 TAP/JSON 格式

**Verification:**
- `tools/testing/kunit/kunit.py run_all` 可以一键运行全部测试引擎
- 支持 `--subsystem` 参数单独运行特定子系统
- 偶发性失败自动重试，确定性失败直接报告
- 测试环境配置和构建结果可缓存，重建时间 ≤30 分钟

---

### U3. kernel/ 子系统分支覆盖率达到 90%

**Goal:** 为 `kernel/` 子系统编写 KUnit 和 kselftest 测试，使该子系统的分支覆盖率达到 90%。

**Requirements:** R9, R10, R11

**Dependencies:** U1, U2

**Files:**
- Create: `tools/testing/coverage/baseline/` — 基线覆盖率数据目录
- Create: `kernel/test/` — `kernel/` 子系统的 KUnit 测试（新建目录）
- Modify: 各目标文件的 Makefile 添加 `GCOV_PROFILE_*.o := y`

**Approach:**
- 先用 U1 的基础设施测量 `kernel/` 的基线分支覆盖率
- 分析未覆盖分支，识别可通过 KUnit 覆盖的路径（调度器、printk、irq、time、locking、RCU、BPF 等）
- 编写 KUnit 测试用例覆盖关键路径，特别关注错误处理分支
- 对需要用户空间交互的路径，编写 kselftest 测试
- 使用 Fault Injection 触发错误处理路径（如内存分配失败）
- 每个测试用例关联到具体的未覆盖分支，确保测试有明确的覆盖率贡献
- 达标后提交审计，审计通过后进入下一子系统

**Patterns to follow:**
- `lib/kunit/` 的 KUnit 测试注册模式
- `tools/testing/selftests/` 的 kselftest  harness 模式
- `lib/fault-inject.c` 的 `should_fail()` 用法

**Test scenarios:**
- Happy path: `kernel/sched/` 核心路径的 KUnit 测试覆盖率达到 90%
- Happy path: `kernel/printk/` 的 printk 路径通过 kselftest 覆盖率达到 90%
- Edge case: 调度器在不同优先级下的分支覆盖
- Error path: 通过 Fault Injection 触发 kmalloc 失败，覆盖错误处理分支
- Integration: KUnit + kselftest + Fault Injection 组合覆盖 `kernel/` 的完整路径

**Verification:**
- `kernel/` 子系统的分支覆盖率 ≥90%
- 覆盖率报告中未覆盖分支 <10%
- 每个测试用例有明确的代码路径覆盖目标
- 测试代码通过代码审查

---

### U4. mm/ 子系统分支覆盖率达到 90%

**Goal:** 为 `mm/` 子系统编写 KUnit 和 kselftest 测试，使该子系统的分支覆盖率达到 90%。

**Requirements:** R9, R10, R11

**Dependencies:** U3

**Files:**
- Create: `mm/test/` — `mm/` 子系统的 KUnit 测试
- Modify: `mm/` 下各目标文件的 Makefile 添加 `GCOV_PROFILE_*.o := y`

**Approach:**
- 分析 `mm/` 子系统的关键路径：page allocator、slab、vmalloc、hugetlb、swap、mmap、madvise、mprotect 等
- 编写 KUnit 测试覆盖内存分配器的基础路径（ buddy system、slab allocator）
- 编写 kselftest 测试覆盖用户空间可见的内存管理接口（mmap、madvise、mprotect、brk）
- 使用 Fault Injection 触发内存分配失败，覆盖错误处理路径
- 重点覆盖竞争条件路径（通过 KCSAN 检测）
- 达标后提交审计，审计通过后进入下一子系统

**Patterns to follow:**
- U3 中建立的 `kernel/` 测试模式和覆盖率流程
- `mm/damon/` 现有的 KUnit 测试（`tools/testing/kunit/configs/damon`）

**Test scenarios:**
- Happy path: page allocator 的分配/释放路径覆盖率达到 90%
- Happy path: mmap/munmap 的用户空间路径覆盖率达到 90%
- Edge case: 不同内存区域（DMA、Normal、HighMem）的分配路径
- Error path: 通过 failslab 注入 kmalloc 失败，覆盖错误处理分支
- Error path: 通过 fail_page_alloc 注入 page alloc 失败
- Integration: 内存压力下的分配路径（结合 fault injection）

**Verification:**
- `mm/` 子系统的分支覆盖率 ≥90%
- 覆盖率报告中未覆盖分支 <10%
- 错误处理路径通过 fault injection 覆盖

---

### U5. fs/ + net/ 子系统分支覆盖率达到 90%

**Goal:** 为 `fs/` 和 `net/` 子系统编写测试，使两个子系统的分支覆盖率达到 90%。

**Requirements:** R9, R10, R11

**Dependencies:** U4

**Files:**
- Create: `fs/test/` — `fs/` 子系统的 KUnit 和 kselftest 测试
- Create: `net/test/` — `net/` 子系统的 KUnit 和 kselftest 测试
- Modify: `fs/` 和 `net/` 下各目标文件的 Makefile 添加 `GCOV_PROFILE_*.o := y`

**Approach:**
- `fs/`：重点覆盖 VFS 层（superblock/inode/dentry 操作）、ext4 核心路径、path lookup、mount 流程
- `fs/`：使用 kselftest 覆盖文件系统操作的完整路径（open/read/write/close、ioctl、fcntl）
- `net/`：重点覆盖 sk_buff 生命周期、NAPI、netdevice 模型、socket 层
- `net/`：使用 kselftest 覆盖网络协议栈（IPv4、IPv6、TCP、netfilter）
- 使用 syzkaller 对 VFS 和网络栈进行模糊测试，覆盖罕见路径
- 使用 Fault Injection 触发块设备 IO 错误、网络包丢失等场景
- 达标后提交审计，审计通过后进入下一子系统

**Patterns to follow:**
- `fs/ext4/` 现有的 KUnit 测试（`.kunitconfig` 在 `fs/ext4/`）
- `tools/testing/selftests/net/` 现有的网络测试
- `tools/testing/selftests/filesystems/` 现有的文件系统测试

**Test scenarios:**
- Happy path: VFS 层的路径查找（path lookup）覆盖率达到 90%
- Happy path: ext4 核心路径（inode 操作、块分配、journal）覆盖率达到 90%
- Happy path: 网络栈的 sk_buff 分配/释放路径覆盖率达到 90%
- Happy path: TCP 连接建立/断开路径覆盖率达到 90%
- Edge case: 不同文件系统类型的 VFS 操作差异
- Error path: 通过 fail_make_request 注入块设备 IO 错误
- Error path: 通过 fail_skb_realloc 注入网络 skb 重分配失败
- Integration: syzkaller 模糊测试 VFS 和网络栈的边界路径

**Verification:**
- `fs/` 子系统的分支覆盖率 ≥90%
- `net/` 子系统的分支覆盖率 ≥90%
- syzkaller 发现的边界路径有对应的回归测试

---

### U6. drivers/ 子系统分支覆盖率达到 90%

**Goal:** 为 `drivers/` 子系统编写测试，使该子系统的分支覆盖率达到 90%。

**Requirements:** R9, R10, R11

**Dependencies:** U5

**Files:**
- Create: `drivers/test/` — `drivers/` 子系统的测试框架和公共测试
- Create: `drivers/base/test/` — 驱动核心（device/driver/bus 模型）的 KUnit 测试
- Create: `drivers/gpu/drm/tests/` — DRM 子系统的 KUnit 测试扩展
- Modify: `drivers/` 下各目标文件的 Makefile 添加 `GCOV_PROFILE_*.o := y`

**Approach:**
- 由于驱动代码需要硬件或完整平台模拟，优先使用 QEMU 作为测试平台
- 重点覆盖驱动核心框架（`drivers/base/`）：kobject、device、driver、bus 层次结构
- 按驱动类型分组，优先覆盖通用驱动（block、char、net、sound、gpu）
- 使用 QEMU 启动完整系统，运行 kselftest 和 KUnit 测试
- 使用 syzkaller 对驱动接口进行模糊测试
- 对于需要特定硬件的驱动，使用 QEMU 的设备模拟或编写 mock 测试
- 达标后提交审计，审计通过后进入下一子系统

**Patterns to follow:**
- `drivers/gpu/drm/ci/kunit.sh` 的 QEMU + KUnit 模式
- `drivers/gpu/drm/tests/` 现有的 DRM KUnit 测试

**Test scenarios:**
- Happy path: 驱动核心框架（device_register/driver_register/bus_register）覆盖率达到 90%
- Happy path: DRM 核心路径（drm_mode_create、gem_create、dma_resv）覆盖率达到 90%
- Edge case: 不同总线类型（PCI、USB、platform）的驱动绑定流程
- Error path: 通过 fault injection 触发驱动 probe 失败
- Error path: 模拟设备热插拔（device_add/remove）
- Integration: QEMU 完整系统启动后运行 KUnit 和 kselftest

**Verification:**
- `drivers/` 子系统的分支覆盖率 ≥90%
- 测试可以在 QEMU 中自动运行
- 驱动核心框架的测试覆盖率 ≥95%（驱动核心相对稳定，覆盖率应更高）

---

### U7. arch/*/ 子系统分支覆盖率达到 90%

**Goal:** 为 `arch/*/` 子系统编写测试，使该子系统的分支覆盖率达到 90%。

**Requirements:** R9, R10, R11

**Dependencies:** U6

**Files:**
- Create: `arch/test/` — 架构相关测试的公共框架
- Create: `arch/x86/` 下的架构特定测试
- Create: `arch/arm64/` 下的架构特定测试
- Modify: `arch/*/` 下各目标文件的 Makefile 添加 `GCOV_PROFILE_*.o := y`

**Approach:**
- 架构代码高度依赖具体硬件，使用 QEMU 模拟多种架构（x86_64、arm64、riscv）
- 重点覆盖架构无关的公共路径（entry common、signal handling、syscall dispatch、context switch）
- 按架构分组，优先覆盖主流架构（x86_64、arm64）
- 使用 KUnit 测试架构特定的辅助函数和数据结构的逻辑
- 使用 kselftest 测试架构特定的系统调用和 ABI
- 对于异常处理路径，使用 QEMU 的 fault injection 机制触发
- 达标后提交审计，审计通过后进入下一子系统

**Patterns to follow:**
- `arch/x86/` 现有的测试（`tools/testing/selftests/x86/`）
- `arch/arm64/` 现有的测试
- U6 中建立的 QEMU 测试模式

**Test scenarios:**
- Happy path: 系统调用分发路径（syscall entry/exit）覆盖率达到 90%
- Happy path: 信号处理路径（signal delivery/return）覆盖率达到 90%
- Happy path: 上下文切换路径覆盖率达到 90%
- Edge case: 不同系统调用号的 syscall 处理差异
- Error path: 通过 QEMU 注入页错误、段错误等异常
- Error path: 模拟系统调用参数无效的错误处理路径
- Integration: 完整系统启动后运行架构特定的 kselftest

**Verification:**
- `arch/*/` 子系统的分支覆盖率 ≥90%
- 主流架构（x86_64、arm64）的覆盖率 ≥95%
- 测试可以在 QEMU 中自动运行

---

### U8. 审计报告、回归防护和 CI 集成

**Goal:** 建立完整的审计报告系统、覆盖率回归防护机制和 CI 集成。

**Requirements:** R12, R13, R14, R11

**Dependencies:** U7

**Files:**
- Create: `tools/testing/audit/report_generator.py` — 季度审计报告生成器
- Create: `tools/testing/audit/coverage_regression.py` — 覆盖率回归检测工具
- Create: `tools/testing/audit/ci_configs/` — CI 配置模板
- Modify: `.gitlab-ci.yml`（或新建）— CI 流水线配置
- Create: `docs/dev-tools/coverage-audit.rst` — 覆盖率审计文档

**Approach:**
- 审计报告生成器：汇总各子系统的覆盖率数据，生成趋势图、未覆盖分支分析、测试完整性证明
- 覆盖率回归检测：在 CI 中集成覆盖率门控（coverage gate），新代码合并不导致已达标子系统覆盖率下降
- CI 集成：建立 GitLab CI 流水线，支持自动运行测试、采集覆盖率、生成报告、触发覆盖率门控
- 审计文档：编写详细的覆盖率采集和验证文档，支持审计方独立运行测试套件
- 支持审计方导出原始覆盖率数据，独立验证报告结果

**Patterns to follow:**
- `drivers/gpu/drm/ci/kunit.yml` 的 GitLab CI 模式
- `tools/testing/selftests/kselftest/runner.sh` 的 TAP 输出模式

**Test scenarios:**
- Happy path: 提交 PR 后，CI 自动运行相关子系统测试，覆盖率不下降
- Happy path: 生成季度审计报告，包含所有必需字段
- Edge case: 新代码覆盖了新的分支，覆盖率上升
- Error path: 新代码引入了未覆盖的分支，覆盖率下降，CI 阻断合并
- Integration: 审计方按照文档独立运行测试套件，得到与项目报告一致的覆盖率数据

**Verification:**
- 季度审计报告包含所有必需字段（覆盖率趋势、未覆盖分支分析、测试完整性证明）
- CI 流水线可以自动运行测试、采集覆盖率、执行回归检测
- 覆盖率门控有效：新代码合并不导致覆盖率下降
- 审计方可以独立运行测试套件并复现覆盖率数据

---

## System-Wide Impact

- **Interaction graph:** 测试编排器与所有测试引擎（KUnit、kselftest、syzkaller、Fault Injection）交互；覆盖率采集与内核构建系统（Kbuild）交互；CI 流水线与 GitLab 交互
- **Error propagation:** 覆盖率采集失败不应阻断测试运行；测试引擎故障应隔离影响
- **State lifecycle risks:** 覆盖率数据文件可能很大（GB 级），需要定期清理和归档；测试环境配置需要版本控制
- **API surface parity:** 覆盖率报告格式需要向后兼容，支持历史数据对比
- **Integration coverage:** 测试编排器需要与现有构建系统（`make kselftest`、`make kunit`）集成，不破坏现有工作流
- **Unchanged invariants:** 内核源码不因测试工程而修改（除必要的 `GCOV_PROFILE_*.o` 标记外）；现有测试套件继续正常工作

---

## Risks & Dependencies

| Risk | Likelihood | Impact | Mitigation |
|------|-----------|--------|------------|
| 部分架构代码（如 arch/*/ 的异常处理）在软件层面无法覆盖 | High | High | 与审计方协商，对无法覆盖的代码提供替代证明（代码审查、静态分析） |
| 覆盖率采集导致内核性能大幅下降，影响测试效率 | Medium | Medium | 使用采样模式或分阶段采集；在非性能关键路径上使用 kcov |
| syzkaller 对某些子系统的覆盖效率低 | Medium | Medium | 针对性编写 seed corpus；结合 kselftest 补充覆盖 |
| 测试用例维护成本高，随内核演化需要持续更新 | Medium | Medium | 建立测试维护流程；将测试用例纳入内核审查流程 |
| 覆盖率数据量大（GB 级），存储和传输成本高 | Medium | Low | 使用 lcov 的压缩格式；仅保留汇总数据，详细数据按需生成 |
| 团队技能缺口：测试工程师需要熟悉内核内部 | Medium | Medium | 培训计划；与内核开发者结对编程 |

---

## Documentation / Operational Notes

- 覆盖率审计文档：`docs/dev-tools/coverage-audit.rst`
- 测试编写指南：`docs/dev-tools/testing-coverage-guide.rst`
- CI 流水线文档：`.gitlab-ci.yml` 注释
- 每个子系统的覆盖率基线数据存档在 `tools/testing/coverage/baseline/`
- 审计报告存档在 `tools/testing/audit/reports/`

---

## Sources & References

- **Origin document:** [docs/brainstorms/kernel-branch-coverage-90-requirements.md](../brainstorms/kernel-branch-coverage-90-requirements.md)
- **KUnit 框架:** [lib/kunit/](../../lib/kunit/)
- **KUnit 运行器:** [tools/testing/kunit/kunit.py](../../tools/testing/kunit/kunit.py)
- **kselftest 目录:** [tools/testing/selftests/](../../tools/testing/selftests/)
- **gcov 实现:** [kernel/gcov/](../../kernel/gcov/)
- **kcov 配置:** [lib/Kconfig.debug](../../lib/Kconfig.debug)
- **Fault Injection:** [lib/fault-inject.c](../../lib/fault-inject.c)
- **DRM CI 示例:** [drivers/gpu/drm/ci/](../../drivers/gpu/drm/ci/)
- **gcov 文档:** [Documentation/dev-tools/gcov.rst](../../Documentation/dev-tools/gcov.rst)
- **kcov 文档:** [Documentation/dev-tools/kcov.rst](../../Documentation/dev-tools/kcov.rst)
