<!--
Copyright (C) 2026 annsshadow
SPDX-License-Identifier: AGPL-3.0-or-later
-->

# oa4rust 三端全面优化 100 轮账本（2026-09-28 启动）

> 目标：oa4rust 三端（Rust 后端 + desktop + mobile）的**功能扩展**与**性能质量提升**全面优化 100 轮。
> 每轮 = 真实扫描 → 处置 → 验证 → 记账。结果三类：**FEAT**（功能扩展）、**FIX**（发现并修复）、
> **PERF/IMPROVE**（性能/不改行为的质量提升）。
> 节奏：每轮轻量门禁（biome lint + tsc + 受影响测试）；每 10 轮中型门禁（全量 vitest + 双端 build）；
> 第 50/100 轮完整门禁（另加 cargo fmt/clippy --all-targets -D warnings/parity/reconcile）。

## 前置资产（如实记档，不计入本轮 100 轮）

- **审计优化循环 100 轮**（`docs/audits/optimization-loop-100.md`，2026-09-27 收官）：FIX 14 /
  IMPROVE 3 / CLEAN 其余，新增真实现 html→image，501 桩全仓清零，终态门禁 8/8 绿。
- **2026-09-27 审计批次** 7 提交：契约物漂移清零、63 表 xid 索引、42 缺列 + 50 个 500 修复、zip×3 真实现。
- **2026-09-28 性能增量** 7 提交：`[profile.release]` thin-LTO/strip、发布档/延迟 A/B 基准脚本与实测、
  CPU 微基准、clippy 门禁（--all-targets -D warnings）落地、前端 lint 收紧 --error-on-warnings。
- 真实消费收官（`docs/audits/three-ends-2026-09-20/REAL_CONSUMPTION_CLOSURE.md`）：216 条未消费路由
  三类裁决（116 仅运行时可验证 / 73 仅真实用户写动作 / 27 结构不可计入），名义接桩已被用户否决并 revert。

## 状态：进行中（轮 1/100）

| 轮 | 维度 | 结果 | 处置 |
|---|---|---|---|
| 1 | FEAT+门禁修复（desktop） | FEAT | **CSV 导入从占位转真实现**（commit `bfad15676`）：`importData` 占位 toast → `utils/csv.ts` RFC4180 子集解析（引号/`""` 转义/CRLF·CR·LF/BOM/字面 `\t` 映射）+ 逐行 POST 真实表行插入端点 `/api/query/assemble/designer/table/{flag}/row`（handler `table_tableFlag_row_insert` 整包存 body 为行 data，实读核实）。四道闸：文件在位/Excel 拒收（accept 收窄 `.csv,.tsv,.txt`）/空解析拦截/500 行上限+截断披露；confirmMsg 确认 + 每 100 行进度 + 失败收集汇总。测试 11 例（csv 运行时 7 + 视图源码断言 4）。**顺修收紧门禁两处既有误伤**：vue override 关 `useVueMultiWordComponentNames`（页面级路由组件无模板标签冲突面）+ `deepen_form.cjs` 无用 `String.raw` 改普通模板字面量。门禁：vitest 964/964（953 基线 + 11 新增）、biome 238 文件 0 错、tsc 4 包 0 错、desktop build 2.63s |

## 记账纪律

- 提交信息带「（优化轮 N）」尾注；只暂存本轮文件（工作区有并行目标会话的 laiyipao 在途改动，绝不越界暂存）。
- 本仓工作树 CRLF/LF 混住、且存在工具回显损坏风险：**跨工具核实一律以 od/python 字节级输出为权威**，
  编辑用 `read_bytes`/`write_bytes`；Edit 工具的 old_string 必须先经字节级核实（本轮轮1 曾因一次
  空 Edit 误删 `}` 后换行造成 `}function` 粘连，靠 git diff 抓回）。
- 断言 needle 必须从真实源码字节抄，不凭记忆拼（轮1 的 `.xlsx|xls` needle 撞上 `\.(xlsx|xls)` 的括号假红一次）。
