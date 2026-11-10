---
title: "账本日志要点里 Backlog 编号的「编号 → 允许的说法」名册（A350① / L303）"
date: 2026-11-10
last_refreshed: 2026-11-10
category: testing
module: augmentor/tests/unit/test_backlog_claim_roster_l303.py
problem_type: ledger_backlog_id_prose_drift
component: backlog_id_claim_roster
severity: medium
symptoms:
  - "同一个 Backlog 编号在日志要点里被写成三件不同的事，全账无一处红"
  - "按一个不存在的账做归并：后两轮的待拍清单引用了 A344 从未有过的「写盘端点失败形状」"
  - "语义对账要读散文，确定性判据做不到，历轮只能靠人眼"
root_cause: prose_claim_on_backlog_id_with_no_reconciliation_ruler
resolution_type: per_id_allowed_claim_roster
tags: [ledger, backlog-id, claim-roster, prose-consistency, blind-spot, teeth, correction-record]
applies_when:
  - "同一个编号在账本散文里承担了内容断言，而表行才是权威口径"
  - "想把「允许怎么说」钉成确定性判据，又不想让尺子读整段散文做语义判断"
  - "订正记录本身必须引用旧说法，需要把它排除在活说法面之外"
related_components:
  - augmentor/tests/unit/test_backlog_claim_roster_l303.py
  - augmentor/OPTIMIZATION_LOOP_2.md
  - augmentor/tests/unit/test_backlog_mark_vocab_l253.py
---

# 账本日志要点里 Backlog 编号的「编号 → 允许的说法」名册

## 现象

A350（L270 立账）量到同一个编号 A344 在日志要点里被写成三件不同的事：

- A344 的表行（权威口径）写的是「点名的提交哈希只验解析、没量可达性」；
- L266 块 ⑮ 写成「共享工作树吞提交 / 换机器部署」；
- L268 块 ⑫ 写成「`api/routes/*` 的写盘端点失败形状」，L269 块 ⑫ 沿用后者。

后果不是笔癖：全账逐条比对 Backlog 表行，讲写盘端点的两格是 A340 的批量导出咽异常与
A347 的写面匿名可达，都不是「失败形状」那一维。后两轮的待拍清单于是按一个不存在的账
做了归并。

为什么没有尺子：l253 的「同卷可查」只判编号在不在、不判内容；语义对账要读散文，
确定性判据做不到。

## 判据

A350 给出三个候选：① 同行订正并立「编号 → 允许的说法」逐格名册；② 只订正不立尺；
③ 维持现状。L300 先落 ②（三处同行订正，净 0 行）；用户 2026-11-10 选 ① 补上牙齿。

守卫只扫 `## 循环日志` 之后的 bullet（`- ⑮ ...` 那种），只看两种「说法形状」：

1. **编号后紧跟全角括号**：`A344 那一格（...）` / `**A344**（...）`；
2. **编号 + `的` + 名词**：`A344 的可达性缺口`。

括号若是定级 / 轮号元数据（`M，待拍`、`L270 立`），不进内容面；`A344 的 ②` 这种候选
序号引用也不进面。允许集按 A344 表行的权威口径钉：

- `parenthetical`：只许「点名的提交哈希只验解析、没量可达性」这一条；
- `noun`：只许以「可达性缺口 / 可达性 / 提交哈希」起头；
- `forbidden`：把「共享工作树吞提交」「换机器部署」「写盘端点失败」「失败形状」列为
  禁止词，写回活说法面即红。

现量四个活说法格：

| 卷 | 块 | 圈号 | 形状 | 归一后的说法 |
| --- | --- | --- | --- | --- |
| OPTIMIZATION_LOOP_2.md | 266 | ⑮ | parenthetical | 点名的提交哈希只验解析、没量可达性 |
| OPTIMIZATION_LOOP_2.md | 268 | ⑫ | parenthetical | 点名的提交哈希只验解析、没量可达性 |
| OPTIMIZATION_LOOP_2.md | 269 | ② | noun | 可达性缺口 |
| OPTIMIZATION_LOOP_2.md | 269 | ⑫ | noun | 可达性缺口 |

`live_roster()` 用 `(卷, 块, 圈号, 形状, 归一后的说法)` 做精确集合相等，**不用行号**
做键——行号会随插行平移，正是历轮反复重钉的那种陈旧读数。

## 为什么排除 L300 块

L300 块 ⑫ 是「三处订正」的现场，必须引用旧说法才能说明改了什么。按 A348① 的同一口径
「记录不改写」，它不进活说法面：`QUOTE_BLOCKS = {300}`。

排除必须是**承重的**而不是一刀切的宽容：

- `test_the_correction_record_block_is_out_of_the_live_face` 证明现量 0 格来自块 300；
- `test_the_correction_record_block_is_not_a_blanket_tolerance` 在内存里把
  `QUOTE_BLOCKS` 清空 ⇒ 块 300 里那三条旧括号立刻成为缺陷格，且缺陷格**只**来自 300。

## 真实返工：第一遍就咬到第四格

L300 的 ② 只订正了 L269 块 ⑫，块 ② 里同一句「A344 的「失败形状」」还在。

守卫首跑 **9 failed / 2 passed**，其中 `block 269 line 4990` 被 `defects()` 当场点名。
修掉尺子自身的两处形状之后（`的` 后的空格、嵌套 bullet 未进面）仍 **6 failed / 5 passed**，
活缺陷只剩这一格。同行订正 + 补嵌套 bullet 之后 **11 passed**（0.94 s）。

这条读数正是 A350① 要的牙齿：L300 的三处名册漏了第四处，靠人读没发现，靠形状判据发现。
本轮因此在账本与守卫 docstring 里都如实记成「第一遍就咬到第四格」，不美化成一次过。

## 处置

**产品码零改动**。只改守卫与账本：

- 新增 `augmentor/tests/unit/test_backlog_claim_roster_l303.py`（**11 支 / 零 parametrize**）：
  - `TestTheClaimRoster` 四支：现量名册精确相等、无禁止词、权威表行与允许说法互相印证、
    两种形状都在活面；
  - `TestRulerBites` 四支：L266 ⑮ 写错括号、L269 ⑫ 写错名词、L268 ⑫ 写错括号各红一格，
    外加「第五个内容格进面但名册接不住」；
  - `TestTheCorrectionRecordIsNotTolerance` 三支：块 300 不在活面、摘掉排除即红、
    未在册编号同形不扫。
- 把 L269 块 ② 里把 A344 说成「失败形状」的那句旧说法同行订正为「可达性缺口」（净 0 行）；
- A350 行追加「L303 落地①」并收口为 `~~M~~ 已收口（L303）`。

## 守卫与证据

- 守卫：`augmentor/tests/unit/test_backlog_claim_roster_l303.py`，11 支。
- 定向复跑：该守卫 ⇒ **11 passed**（0.89 s / 0.96 s 两次）。
- 受影响守卫批：l242 / l253 / l263 / l264 / l268 / l270 / l303 ⇒ **147 passed**
  （推送前 109.67 s，推送后 103.22 s）；a184 复跑仍是基线 6 红。
- 账本重钉：l242 `MEASURED_MARKED` 149 → 150；l253 `MEASURED_CLOSURE_CELLS` 149 → 150、
  `MEASURED_WIDE_ROWS` 152 → 153、`MEASURED_SAME_VOLUME` 136 → 137、裸 `M` 65 → 64、
  `~~M~~` 70 → 71、`已收口` 75 → 76、combo `("~~M~~","已收口","L",False)` 31 → 32；
  l263 `MEASURED_NO_VERDICT_ROWS` 125 → 124；l268 operative 1002 → 1004；
  l270 `MEASURED_SITES` 751 → 753、`MEASURED_TIERS[REACHABLE]` 18 → 20、
  卷二日志区 175 → 177、`MEASURED_UNIQUE_HEX` 438 → 440、`MEASURED_L264_SITES` 170 → 172；
  l264 站点 170 → 172、`cross_named` 6 → 7、`no_round_subject` 5 → 6、唯一名册 5 → 6。
  逐格按守卫红读数重钉，没有一把尺子被改松。
- 全量定标：本轮不跑全量；L301 的基线仍是 `37 failed / 10272 passed / 6 skipped / 99.62%`，
  37 项红全部落在既有基线族，不声称全绿。
- 提交：`473c43811`（`test,docs(augmentor): L303 A350① A344 说法名册与 L269② 订正`）。

## 余面

- A350 的 ② 已由 L300 落地，③ 被 ① 取代 ⇒ 本行收口。
- 名册现量只覆盖 A344 一个编号。`CLAIMS` 是字典，后续要给别的编号立名册时逐条加，
  不是「全编号自动对账」——没有允许集的编号照旧不进面。
- 叙述性点名（例如 A350 行自己的立账记录「A344 实测三种互斥说法」）不在这一把尺的量面里：
  那是立账记录，不是对 A344 内容的断言。要收它得先定义「什么算断言」，确定性判据还没有。

## 教训

1. **订正记录与活说法面必须分开**：记录要引用旧说法才能说明改了什么，把它留在面里会
   逼着未来的订正去改写历史。排除要写成一张小名单，并配一支「排除是承重的」的牙齿。
2. **名册用内容键，不用行号键**：行号随插行平移，名册会变成陈旧读数；`(卷, 块, 圈号,
   形状)` 随轮次固定。
3. **判据要挑形状，不要读语义**：只认「编号 + 全角括号」与「编号 + 的 + 名词」两种
   可正则化的形状，元数据与候选序号先排除，再谈允许集。
4. **第一遍的红是资产**：9 failed / 6 failed / 11 passed 三段读数都留在账上，它证明
   L300 的三处名册漏了第四处，而不是事后补记一个「一次过」。
5. **改写账本措辞会造出新缺陷格**：本轮 L303 日志块自己一度写出 `A344 那一格（...）`
   与 `A344 的「失败形状」`，被自己的尺子咬到后改成 `<编号>` 占位，而不是扩大排除面。

## 参考

- 账本：`augmentor/OPTIMIZATION_LOOP_2.md` L303 清单行与日志块、A350 行
- 提交：`473c43811`（守卫 + 账本 + 六把常数）
- 守卫：`augmentor/tests/unit/test_backlog_claim_roster_l303.py`
- 复用：`augmentor/tests/unit/test_backlog_mark_vocab_l253.py` 的 `load_volumes()` /
  `log_boundary()` / `table_rows()`
- 同族：A348（`dangling` 点名保留原哈希）、A349②（批次短哈希子集名册）、
  A333（名册随账本变长的代价）
- 立尺：A350（本格）、L300（候选② 三处订正）、L303（候选① 名册）
