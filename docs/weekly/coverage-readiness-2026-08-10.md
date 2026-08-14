# 覆盖率测量可行性诊断（2026-08-10）

> 背景：用户要求“跑一次全量覆盖率以量化实际达成度”。
> 结论：**当前覆盖率工具链无法产出真实数字**，须先修复再运行。

---

## 1. 环境可行性

| 项 | 结果 |
|----|------|
| 原生构建（MINGW64 / Windows） | ❌ 仅 MinGW gcc 13.2.0，目标是 Windows PE，无法编译 Linux 内核；`make`/`lcov`/`gcovr` 均不存在 |
| WSL | ⚠️ 仅有 `docker-desktop` utility VM，无 bash/gcc/make，非开发用 Linux |
| Docker | ✅ Docker 29.6.2 可用，Desktop 运行中 |
| 项目自带容器化流程 | ✅ `Dockerfile.baseline`（Debian + gcc + lcov + python3）+ `run_baseline_docker.sh` 即为其设计运行环境 |
| 既往 gcov 编译 | ✅ `build-baseline/` 含 **351 个 `.gcno`**（UML 驱动等），证明 gcov 编译路径曾跑通 |

**结论**：唯一可行路径是 Docker 内运行项目自带流程。

---

## 2. 工具链缺陷（导致结果恒为 0% / 乱码）

### 缺陷 A — 采集步骤缺失
- 位置：`tools/testing/coverage/coverage_harness.py:102-107`（`collect_gcov_data`）
- 现状：仅 `os.walk(build_dir)` 找 `.gcda`。
- 问题：内核 gcov 的运行时 `.gcda` 写在**已启动 UML 内核的 debugfs**（`/sys/kernel/debug/gcov/`），必须先从 debugfs 拷回 build 目录。脚本从未拷贝 → 永远找不到 `.gcda`。

### 缺陷 B — 解析了错误的文件类型
- 位置：`tools/testing/coverage/gcov_parser.py:114-133`（`collect_coverage`）
- 现状：遍历 `.gcno`（GCC 二进制 notes 文件）并当文本 `parse_gcov_file`。
- 问题：`.gcno` 是二进制；真正的 gcov 文本是运行 `gcov -b` 后生成的 `.gcov` 文件。`parse_gcov_file` 的解析逻辑本身适配 `.gcov` 文本格式（`count:line:content` / `branch ... taken %`），但 `collect_coverage` 搜错了扩展名 → 解析二进制得到空/乱码。

### 缺陷 C — 编排未触发采集
- 位置：`tools/testing/coverage/baseline_measurement.py:317-318`
- 现状：`run_kunit_tests()` + `run_kselftest_tests()` 之后直接 `collect_and_classify_coverage()`。
- 问题：中间没有 `make ARCH=um gcov`（或 debugfs 拷贝 + `gcov` 生成）步骤。UML 进程退出后内存中覆盖率即丢失。

### 佐证
`build-baseline/` 仅有 `.gcno`、**无 `.gcda`**，与缺陷 A/C 的推论完全一致——编译通过但从未启动采集。

---

## 3. 最小修复方案

1. **`collect_gcov_data()`**：测试运行后，从 `/sys/kernel/debug/gcov` 拷贝 `.gcda` 回 build 目录；对每个 `.gcno` 运行 `gcov -b -o <builddir> <file>.gcno` 生成 `.gcov` 文本。
2. **`GcovParser.collect_coverage()`**：遍历对象改为 `.gcov` 文件（现有 `parse_gcov_file` 逻辑无需改，仅改搜索扩展名与入口）。
3. **`baseline_measurement.py`**：在 `run_kunit_tests` 之后插入上述采集调用（或 `make ARCH=um gcov`）。

---

## 4. 修复后执行路径与风险

- 执行：`bash tools/testing/coverage/run_baseline_docker.sh`（内部 `docker build` + 运行 `baseline_measurement.py --arch um --tool gcov`）。
- 耗时：UML 内核全量 gcov 编译约 15–25 分钟 + 测试 + 采集。
- 风险：
  - UML 在容器内运行需 ptrace/skas 与 debugfs 挂载，可能需 `--privileged` 或额外权限。
  - Debian 镜像源已改为 `mirrors.aliyun.com`，若容器内网络不可达会拉取失败。
  - gcov 采集时序：必须在 UML 进程退出前完成 `.gcda` 拷贝。

---

## 5. 建议

目标为“量化覆盖率达成度”，应**先修复 3 处缺陷再运行**，而非直接跑损坏的流程（将得到 0%/乱码，误导判断）。修复工作量小、定位明确，建议直接实施后在 Docker 内实跑。
