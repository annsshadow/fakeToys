---
title: "账本表格的 GFM 竖线口径：转义竖线不是分隔符（A372 / L301）"
date: 2026-11-10
last_refreshed: 2026-11-10
category: testing
module: augmentor/tests/unit/test_backlog_mark_vocab_l253.py
problem_type: gfm_table_pipe_escaping
component: ledger_table_column_count
severity: small
symptoms:
  - "A325 行有 15 枚竖线，GitHub 渲染时与四列表头错位"
  - "A344 行只有 4 枚竖线，少一级分隔，定级格落在「内容」列"
  - "l253 的末格判据按裸 split(\"|\") 数格，把 GFM 合法的转义竖线也算成异形行"
root_cause: raw_pipe_split_conflates_escaped_and_unescaped
resolution_type: gfm_aware_single_authority
tags: [gfm, markdown-table, pipe-escaping, column-count, ledger, single-authority]
applies_when:
  - "Markdown 表格单元格里出现竖线（绝对值记号、正则、代码示例）"
  - "用裸 split(\"|\") 数格或切单元格"
  - "渲染口径与判据口径对不上"
related_components:
  - augmentor/tests/unit/test_backlog_mark_vocab_l253.py
  - augmentor/tests/unit/test_api_doc_endpoint_coverage_l210.py
  - augmentor/OPTIMIZATION_LOOP_2.md
---

# 账本表格的 GFM 竖线口径：转义竖线不是分隔符

## 现象

L278 把 `docs/augmentor/API.md` 的表行竖线改按 GFM 转义，并把「数格子」收进
`test_api_doc_endpoint_coverage_l210.py` 的 `column_count` / `split_cells` 单一权威。
同一把尺普查两卷账本后在 A372 立账，量到三处余面：

1. **A325 行 15 枚竖线**：6 枚在行内码、9 枚在正文（`|b|`、`|a|` 这类绝对值记号）。
   GitHub 按未转义竖线切列 ⇒ 这一行被切成 16 列，与四列表头错位。
2. **A344 行只有 4 枚竖线**：少一级分隔，定级格落在「内容」列的位置上。
3. **l253 的「末格定位不靠格数」按裸 `split("|")` 数格**：现量 9 行格数不等于 6，
   其中 7 行带的全是**已转义**竖线（GFM 合法），真正与表头错位的只有 A325 与 A344。

## 根因

GFM 表格只按**未转义**竖线切列；`` `a\|b` `` 里的 `\|` 是内容，`` `a|b` `` 里的裸 `|`
才是分隔符。裸 `split("|")` 看不出这个区别：反斜杠对它不可见，于是合法的转义写法被
判成异形行，而真正错位的行反而因为「最后一格仍是定级格」显得无害。

## 处置

**产品码零改动**：

1. **A325 行**：把 10 枚未转义竖线按 GFM 转义成 `\|`（绝对值记号 `|b|`、`|a|` 等）。
2. **A344 行**：补上缺失的一级分隔，位置格填
   `` `OPTIMIZATION_LOOP.md` / `OPTIMIZATION_LOOP_2.md` ``。
3. **l253 判据**：`grade_cell` 改用 l210 的 `split_cells`（先把 `\|` 换成占位符再切、
   切完还原），末格测试改用 `column_count`（`(?<!\\)\|`）数未转义竖线：
   - 全部表行必须恰好 5 枚未转义竖线（四列）；
   - 带转义竖线的 8 行（A70 / B2 / A288 / A291 / A295 / A321 / A325 / A372）仍由名册
     逐格点名，证明转义竖线不破坏末格定位。

现量：异形行名册从 9 行收成 **0**；带转义竖线的行从 7 行走到 **8 行**（A325 修完后
自己加入）。

## 守卫与证据

- 守卫：`augmentor/tests/unit/test_backlog_mark_vocab_l253.py` 的
  `test_the_grade_cell_is_always_the_last_column`；`grade_cell` 改引 l210 权威。
- 常数重钉：l253 的 `MEASURED_LETTERS`（S 72 / ~~S~~ 70）、`MEASURED_CLOSURE_CELLS` 148、
  `MEASURED_WIDE_ROWS` 151、`MEASURED_SAME_VOLUME` 135、`MEASURED_WORDS`（已收口 74）、
  `MEASURED_COMBOS`（`~~S~~ / 已收口 / L / 不粗体` 15）；l242 `MEASURED_MARKED` 148；
  l263 `MEASURED_NO_VERDICT_ROWS` 126。
- 受影响守卫批：l253 / l242 / l263 / l264 / l270 / l268 / l177 / l203 / markdown 结构
  ⇒ **237 passed**（124.23 s）；a184 复跑仍是基线 6 红。
- 全量定标：`python -m pytest tests -q -p no:cacheprovider` ⇒ `37 failed, 10272 passed,
  6 skipped, 6 warnings, 649.65s`，覆盖率 `99.62%`。37 项红全部落在既有基线族。

## 教训

1. **判据要跟渲染器同口径**：GFM 按未转义竖线切列，判据就不能按裸 `split("|")` 数格。
   否则合法的转义写法被误判成缺陷，而真正的错位行反而隐身。
2. **单一权威**：`column_count` / `split_cells` 由 l210 持有，l223 / l253 共引；
   两把尺子各写一份 `split("|")` 就是 L278 修过的同一形状。
3. **转义竖线是内容不是分隔符**：名册要逐格点名带转义竖线的行，证明末格定位在
   「内容里有竖线」时仍然成立——这一格是设计成会红的。
4. **历史表行的排版订正是允许的**：改的是渲染口径，不是叙事内容；改完由判据的
   未转义竖线计数当场验收。

## 参考

- 账本：`augmentor/OPTIMIZATION_LOOP_2.md` L301 清单行与日志块，A372 行
- 提交：`a4c93b729`（`test,docs(augmentor): L301 A372 两卷账本 GFM 竖线口径收口`）
- 判据：`augmentor/tests/unit/test_backlog_mark_vocab_l253.py`、
  `augmentor/tests/unit/test_api_doc_endpoint_coverage_l210.py`
- 同族：L278（API.md 竖线转义与列数权威）、A372（本格）
