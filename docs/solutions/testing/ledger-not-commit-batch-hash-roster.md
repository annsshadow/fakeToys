---
title: "账本 not_a_commit 档的批次短哈希子集：254 枚按哈希钉的名册（A349② / L299）"
date: 2026-11-10
last_refreshed: 2026-11-10
category: testing
module: augmentor/tests/unit/test_ledger_hash_reachability_l270.py
problem_type: ledger_guard_blind_spot_partial_roster
component: not_a_commit_batch_hash_roster
severity: medium
symptoms:
  - "L270 的可达性尺子把 not_a_commit 档只数总量、不逐格点名"
  - "一枚写错一位的 9 位 commit 哈希会安静落进这一档，永远不会成为缺陷格"
  - "整档 730 处逐格点名的名册随账本变长，每一轮落账都要重钉"
root_cause: count_only_tier_with_no_roster
resolution_type: partial_exact_roster_by_hash
tags: [ledger, hash-reachability, roster, not-a-commit, batch-hash, blind-spot, teeth]
applies_when:
  - "一把尺子的某一档只数总量、不点名"
  - "这一档里有一族形状明确、按哈希即可咬住的子集"
  - "想补牙齿，又不想让整档名册随账本变长"
related_components:
  - augmentor/tests/unit/test_ledger_hash_reachability_l270.py
  - augmentor/OPTIMIZATION_LOOP_2.md
---

# 账本 `not_a_commit` 档的批次短哈希子集：254 枚按哈希钉的名册

## 现象

L270 把两卷账本里所有反引号包住的短哈希分成五档：`reachable` / `dangling` / `stash` /
`not_a_commit` / `ambiguous`。其中 `not_a_commit` 是「git 里解析不到任何提交对象」的那一档，
现量 730 处，几乎全是 2026-10-22 强推重写后散落的历史对象。

这一档立尺时就只数总量，不逐格点名。L270 把它写成已知盲区 ①，并立 **A349**：
「一枚写错一位的 commit 哈希会安静落进这一档而永远不会成为缺陷格」。
`test_a_fabricated_hash_lands_in_the_count_tier` 把这一格量成案面：编造一枚哈希 ⇒
落 `not_a_commit`、缺陷格仍空。

## 判据

A349 给出三个候选：① 照 l264 把 730 处逐格点名；② 只点名长度落在批次短哈希档且同行写着
「批 / 提交」的子集，其余按区数钉；③ 维持现状。用户 2026-11-10 选 ②，名册粒度选**按哈希钉**。

子集的两条判据都是确定性文本判据，不靠人读：

1. **长度 = 9 位**：本账的批次短哈希一律是 `git rev-parse --short` 的 9 位。sha256 摘要前缀
   现量是 16 / 12 / 10 / 8 位，全文是 40 位；编造读数可能是 11 位。长度先把它们分开。
2. **同一物理行含「批」或「提交」**：账本写批次哈希时，同一行会带「账本批」「代码批」
   「三批」「提交」这类标记。同行不含这两个字的 9 位点名（多为早期 `Lnnn` 行里的裸哈希）
   仍归「其余按区数钉」那一面。

现量：`not_a_commit` 共 730 处，其中子集 **457 处 / 379 格 / 254 枚唯一哈希**；按区分布
卷一清单 86、卷一日志 117、卷二清单 140、卷二日志 114。三个数分开钉住，名册本身是
254 枚 `frozenset`，精确相等。

## 为什么按哈希钉

A349 的缺陷形是「写错一位的 commit 哈希」——错一位必产生**新哈希**。

- **按哈希钉**：任何写错的批次哈希都会落进名册外，当场红；名册只随「第一次被点名的提交」
  增长，历轮重提同一枚不增加名册。
- **按格钉**：名册键是 `(卷, 区段, 轮号, 哈希)`，历轮重提同一枚就是新格，名册随账本变长，
  每一轮都要重钉——正是 A333 点名的「名册型守卫随账本变长」那笔代价，而牙齿并不增加。

站点数 457 与格数 379 另用常数钉住，面规模变化仍会红。这样把「面」与「名册」分开：
面用常数，名册按哈希。

## 处置

**产品码零改动**。只改 `augmentor/tests/unit/test_ledger_hash_reachability_l270.py`：

- 新增 `batch_citation_rows(rows, volumes)`，把两条判据做成唯一产地；
- 新增 `MEASURED_NOTCOMMIT_BATCH_SITES = 457` / `MEASURED_NOTCOMMIT_BATCH_CELLS = 379` /
  `MEASURED_NOTCOMMIT_BATCH_HASHES = 254` 与 `NOTCOMMIT_BATCH_HASHES`（254 枚）；
- 面两格：三维现量、哈希名册精确相等；
- 牙齿三格：
  - 9 位编造哈希写在带「批」的行上 ⇒ 进子集而名册接不住；
  - 9 位编造哈希落在**不含**「批 / 提交」的行上 ⇒ 站点 +1、落 `not_a_commit`，子集不动；
  - 16 位 sha256 前缀写在带「批」的行上 ⇒ 不进子集（长度先把摘要与批次短哈希分开）；
- `test_a_fabricated_hash_lands_in_the_count_tier` 的样本改成 11 位编造批哈希，证明
  「非批次档写错一位仍只数总量」——这一格继续作为 A349 ① 的案面。

零 `parametrize`，l97 / l98 两把全仓普查尺子一格未动。

## 守卫与证据

- 守卫：`augmentor/tests/unit/test_ledger_hash_reachability_l270.py`，新增 5 支（面 2 / 牙齿 3），
  改写 1 支样本。
- 定向复跑：`python -m pytest tests/unit/test_ledger_hash_reachability_l270.py -q --no-cov`
  ⇒ **25 passed**（79.27 s）。
- 全量定标：`python -m pytest tests -q -p no:cacheprovider` ⇒ `37 failed, 10272 passed,
  6 skipped, 6 warnings, 617.98s`，覆盖率 `99.62%`。37 项红全部落在既有基线族：l210 端点文档面 2、
  l224 端点消费 8、l233 密钥门禁 4、l252 备份契约 1、a184 注释引用 6、l251 分页消费 1、
  l200 文档数 5、l265 异常透传 2、memory monitor 1、l237 TS 契约 7。本轮未引入新红。
- 账本重钉：l264 `MEASURED_SITES` 165 → 167；l270 `MEASURED_SITES` 744 → 747、
  `MEASURED_TIERS` reachable 11 → 14、`MEASURED_UNIQUE_HEX` 432 → 434、
  `MEASURED_L264_SITES` 165 → 167；l268 operative 自指 994 → 996。
- 受影响守卫批：l264 / l270 / l268 / l177 / l242 / l253 / l263 / l203 ⇒ **153 passed**
  （115.52 s）；a184 复跑仍是基线 6 红。

## 余面

其余 **273 处** `not_a_commit` 仍只按四个区的总量钉，写错一位的非批次哈希（如 11 位编造
读数）仍会安静落进这一档。A349 的 ①（整档逐格点名）与 ③（维持现状）继续待拍；
② 只覆盖批次短哈希这一族。

## 教训

1. **部分名册是合法的中间档**：不是「全量点名」或「只数总量」二选一。把缺陷形最明确、
   按哈希即可咬住的那一族收成精确名册，其余继续用面常数钉住。
2. **名册钉缺陷形，不钉表面**：这一格要抓的是「错一位 ⇒ 新哈希」，所以按哈希钉；
   按格钉会连「哪一轮重提」一起钉死，名册随账本变长而牙齿不增。
3. **面与名册分开钉**：站点数 / 格数用常数（面规模变化会红），哈希用名册（精确相等）。
   两个维度各自现量，不是算术推的。
4. **收窄之后要更新案面**：`test_a_fabricated_hash_lands_in_the_count_tier` 的样本从
   「11 位、同行不含批」改成「11 位、同行含批」，让「非批次档仍只数总量」继续有案面，
   而不是被新子集顺手掩盖。
5. **「补尺」不等于「收口」**：A349 只落候选②，① / ③ 仍待拍；账本与守卫 docstring 都要
   显式写明余面，避免「② 已落地」被读成整条已清。

## 参考

- 账本：`augmentor/OPTIMIZATION_LOOP_2.md` L299 清单行与日志块，A349 行
- 提交：`07bc9b0c2`（`test,docs(augmentor): L299 A349② 批次短哈希子集 254 枚名册`）
- 守卫：`augmentor/tests/unit/test_ledger_hash_reachability_l270.py`
- 同族：A333（名册随账本变长的代价）、A348（`dangling` 那一族的重钉）、l264（逐格点名名册）
- 立尺：L270（五档可达性尺子）、A349（本格）
