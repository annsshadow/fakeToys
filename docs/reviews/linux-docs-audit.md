# Linux 内核 v7.1.3 文档中文化 — 待办清单（TODO）

> 更新时间：2026-08-14
> 范围：`docs/系统文档/`（Linux 内核 v7.1.3 英文文档全量中文化，3382 个 .md）
> 当前基线：覆盖率 196/196（100%）、围栏全偶数、marginal 194→92

## 一、已完成基线（避免重复推进）

- [x] **全量中文化**：196 单元 / 3382 文件 100% 覆盖（`_fullscan.py`：PARTIAL=0、ENGLISH=0）。
- [x] **奇数围栏精修（Task28）**：3413/3414 文件偶数，`module-signing` 残留为上游固有例外。
- [x] **低 pr 文件重译**：277 个低 pr 文件分 10 块并行重译，0 失败；抽样核验译文通顺，低分由不可译技术符号（标题/十六进制/字段名/代码/URL）稀释。
- [x] **marginal 可读性精修**：193 个 marginal 文件（pr∈(0.03,0.10)）在顶部补针对性中文导言，0 失败；marginal 由 194 降至 92；193 个改动文件 ``` 围栏奇偶全偶数，无结构破坏。

## 二、待办清单

### P0 — 已完成（翻译成果入库）

- [x] **翻译成果纳入版本控制**
  - 执行（2026-08-14）：在 `feat/test-coverage-expansion` 分支上以 `master` 为父提交，单笔提交纳入 `docs/系统文档/` 下 **3414 个翻译后 `.md`**（3382 翻译 + 32 核心已译 + START_HERE 等）。
  - 提交：`541231754 docs(zh-cn): 纳入 Linux v7.1.3 文档全量中文化成果`（含 `Assisted-by: _fullscan.py, _prdist.py`，无 `Signed-off-by`）。
  - 排除项（保持未跟踪，留待 P2 清理）：`translations/` 原样中文、`_*.py` 辅助脚本、`.translate_*` 运行产物、`.prose.txt`/`.seg`/`.masked` 等流水线中间产物（共 **770 个未跟踪文件**）。
  - 校验：提交后 `docs/系统文档` 跟踪 = 3414；`translations/` 误入 = 0；`arch/`、`kernel/`、脚本、产物暂存均为 0；`master` 未改动；工作区其他改动（`.gitignore`、`arch/um/Makefile`、`docs/PCI/*` 删除）未进入本次提交。
  - 状态：已完成

### P1 — 建议做（质量 / 一致性）

- [ ] **全局术语一致性复核**
  - 现状：全量由多代理多轮翻译完成，术语可能存在不统一（如「调度器 / 调度程序」「帧缓冲 / 帧缓存」混用）；前序曾反馈「术语表机械译法非流畅」。
  - 建议动作：以 `translations/zh_CN` 与 `START_HERE.md` 为内核中文术语基准，抽取高频术语，扫描各文件译法并统一；输出术语偏差清单后批量修正。
  - 状态：建议

- [ ] **翻译与上游 rst 源同步策略**
  - 现状：`docs/系统文档/` 是 `Documentation/` 的 rst-to-md 产物。内核 v7.1.3 后续若更新 `Documentation/`，翻译如何增量同步尚未定义。
  - 建议动作：定义同步流程（重新生成 md → diff → 仅对变更文件重译 → 复扫校验），避免未来全量重做。
  - 状态：规划性，非紧急

### P2 — 可选（收益递减 / 收尾）

- [ ] **残余 92 个 marginal 二次精修**
  - 现状：cardlist / 寄存器表 / C 源码（`.c.md`）类文件 pr 仍 <0.10，受 pr 算法限制难以越过；但均已加导言且 pr 较原值明显上移，属结构性例外。
  - 建议动作（可选）：给 cardlist 条目补中文分类标签、给 C 源码加中文概要；或维持现状作为已知例外。
  - 状态：可选，建议维持现状

- [ ] **辅助脚本与运行产物清理**
  - 现状：散落临时脚本（`_dispatch_marginal.py`、`_dispatch_qual.py`、`_audit_fences.py`、`_fence_dump.py`、`_fix_odd_fences.py`、`_check141.py`、`_list_odd.py`、`_gitcmp.py`、`_showtails.py`、`detect_tmp.py` 等）及 `.translate_*.json`、`.translate_results/` 运行产物。
  - 建议动作：保留 `_fullscan.py` / `_prdist.py`（校验工具）；归档或删除派发 / 审计临时脚本；`.translate_*` 运行产物移入归档目录或清理。
  - 状态：收尾

- [ ] **长期记忆 / 计划文档状态同步**
  - 现状：`.workbuddy/memory/MEMORY.md` 仍记「全量流水线进行中 / 剩余 189 单元待译」，已过时；`docs/plans/` 或 `docs/solutions/` 若有相关计划文档需标记 `completed`（`AGENTS.md` plan 生命周期）。
  - 建议动作：更新 MEMORY.md 项目状态为「已完成 196/196」；检查并标记相关计划文档 `status: completed`。
  - 状态：元工作

### 附：工程验证（可选）

- [ ] **翻译 md 链接 / 格式有效性验证**
  - 建议动作：对 `docs/系统文档/` 跑死链检查与 md 合法性校验，确保精修未引入损坏链接或格式错误。
  - 状态：可选

## 三、优先级说明

- **P0** 已完成：翻译成果已于 2026-08-14 入库（`feat/test-coverage-expansion` 分支、`master` 之子，3414 个 `.md`），安全网已建立。
- **P1** 提升长期可维护性（术语统一、上游同步），建议在入库后推进。
- **P2** 为可选收尾与元工作，收益递减，可按需安排。
