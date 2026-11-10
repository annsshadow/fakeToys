---
title: "账本游离点名的重钉口径：保留原哈希并同行指向真身（A348① / L302）"
date: 2026-11-10
last_refreshed: 2026-11-10
category: testing
module: augmentor/OPTIMIZATION_LOOP_2.md
problem_type: dangling_citation_repin_policy
component: ledger_dangling_citation
severity: medium
symptoms:
  - "L270 的可达性尺子量到 7 枚不可达对象、26 处点名全部集中在 L265 那一轮"
  - "清单行槽位里的哈希已被并行会话改写，日志块 ⑰⑱ 又明确是历史叙述"
  - "换真身会改写「那一轮当时提交了什么」的证据，维持现状则 gc 之后无处可换"
root_cause: rewritten_commit_citation_without_explicit_true_body_pointer
resolution_type: keep_original_hash_with_true_body_pointer
tags: [ledger, hash-reachability, dangling, rewritten-commit, evidence, repin-policy]
applies_when:
  - "账本点名的提交解析得到、但任何 ref 都到不了"
  - "真身只在本机 git 历史里成立，远端已经取不到"
  - "要保留历史记录，又不想让读者把游离哈希当成当前真身"
related_components:
  - augmentor/OPTIMIZATION_LOOP_2.md
  - augmentor/tests/unit/test_ledger_hash_reachability_l270.py
  - augmentor/tests/unit/test_self_line_claim_l268.py
---

# 账本游离点名的重钉口径：保留原哈希并同行指向真身

## 现象

L270 给两卷账本里所有反引号包住的短哈希立了可达性尺子，按 `git rev-list --branches
--tags --remotes` 的可达面分五档：`reachable` / `dangling` / `stash` /
`not_a_commit` / `ambiguous`。立尺时量到 **7 枚不可达对象、26 处点名**，全部集中在
L265 那一轮（清单行 3 处 + 日志块 23 处）。

成因已经由 L265 块 ⑰⑱ 逐枚写破：共享工作树里并行会话对同一分支做了两次 amend 与一次
reset，主守卫补交枚、账本批等对象被改写；`git gc` 之前它们仍能解析，所以 l264 那把
「解析得到 + 主题说什么」的尺子不红，但「这枚提交在不在分支上」这一维此前全账无人量。

A348 要回答的不是「怎么发现」，而是「发现之后清单行的槽位写成什么」：

- 清单行槽位半新半旧，部分哈希已经指向被改写的对象；
- 日志块 ⑰⑱ 是**历史叙述**，记录的是当时发生了什么，不能追改；
- 真身（`58a1f9987` / `fdc8a5e4a` / `0b32da27f`）只在本机 git 历史里成立，远端取不到。

## 三种候选

A348 原文给出三条互斥口径：

1. **清单行保留原哈希，并同行追加「已被改写出分支，真身见 ⑰⑱」**；
2. 清单行换成真身，原哈希只留在日志块；
3. 维持现状，靠 L270 的名册把 26 处钉死，`git gc` 之后按读数重钉名册。

用户 2026-11-10 选 ①。L302 只落这一条，② 与 ③ 继续待拍。

## 为什么选 ①

1. **原哈希是证据，不是错账**：它记录的是「那一轮当时提交了什么」。换成真身等于用今天
   的本机历史改写当时的记录，而真身只在本机成立，换上去反而制造一个远端读者无法复核的
   指称。
2. **日志块不能追改**：⑰⑱ 已经逐枚点名两次 amend、一次 reset 与六格游离对象；在清单行
   追加一句指向它，读者能同时看到「原哈希」与「为什么它不在分支上」。
3. **gc 之后仍有退路**：`git gc` 跑过，这批对象会从 `dangling` 整批落进
   `not_a_commit`。到那时清单行保留的原哈希仍然指向同一段历史叙述，守卫按红读数重钉
   名册即可——那是纪律，不是重新决策。
4. **不重复哈希**：追加句只写「主守卫补交枚与账本批两枚已被改写出分支，真身见本块
   ⑰⑱」，不把真身再抄一遍。真身口径仍以 ⑰⑱ 为准，避免同一事实在账本里出现两套写法。

## 处置

**产品码零改动**。只改 `augmentor/OPTIMIZATION_LOOP_2.md`：

- L265 清单行保留原哈希与原有叙述，在同一行括号里追加
  「主守卫补交枚与账本批两枚已被改写出分支，真身见本块 ⑰⑱」；
- 不换真身、不重复哈希、不改日志块 ⑰⑱；
- A348 行的定级格收成 `~~M~~ 已收口（L302）`，并在「当前处置」里写明落地的是候选 ①；
- 新增 L302 清单行，哈希槽留「哈希待 L303 回填」，按账本纪律由下一轮回填。

## 守卫与证据

这是**只动账本**的一轮，但账本行本身在六把尺子的量面里，因此改完必须按红读数重钉：

- 改前红读数（逐字）：
  - l242 `assert 149 == 148`；
  - l253 宽面 `152 == 151`、`M 65 != 66`、`~~M~~ 70 != 69`、`已收口 75 != 74`、
    组合 `31 != 30`、闭合格 `149 != 148`、同卷 `136 != 135`；
  - l263 `((152 + 5) + 126) != 282`；
  - l268 `LIVE 0 / STALE 6` 与 `assert 1000 == 1002`；
  - l270 站点 `751 == 750`、`reachable 18 != 17`、卷二清单区 `187 != 186`、
    唯一 hex `438 == 437`；
  - l264 不受影响，定向复跑 **24 passed**。
- 重钉后的常数：l242 `MEASURED_MARKED` 148 → **149**；l253
  `MEASURED_CLOSURE_CELLS` 148 → **149**、`MEASURED_WIDE_ROWS` 151 → **152**、
  `MEASURED_SAME_VOLUME` 135 → **136**、`MEASURED_LETTERS` 的裸 `M` 66 → **65** 与
  `~~M~~` 69 → **70**、`MEASURED_WORDS` 的 `已收口` 74 → **75**、
  `MEASURED_COMBOS` 的 `("~~M~~", "已收口", "L", False)` 30 → **31**；l263
  `MEASURED_NO_VERDICT_ROWS` 126 → **125**；l268 operative 自指 1000 → **1002**；
  l270 `MEASURED_SITES` 750 → **751**、`MEASURED_TIERS[REACHABLE]` 17 → **18**、
  卷二清单区 186 → **187**、`MEASURED_UNIQUE_HEX` 437 → **438**。
- 定向复跑（六文件，含未改的 l264）：
  `python -m pytest tests/unit/test_backlog_closure_l242.py
  tests/unit/test_backlog_mark_vocab_l253.py
  tests/unit/test_round_first_verdict_l263.py
  tests/unit/test_log_block_bullet_ownership_l264.py
  tests/unit/test_self_line_claim_l268.py
  tests/unit/test_ledger_hash_reachability_l270.py -q -p no:cacheprovider --no-cov`
  ⇒ **136 passed**（114.46 s）。
- 提交：`da6426a7f`（`test,docs(augmentor): L302 A348① 清单行保留原哈希并追加真身说明`）。
- 全量定标**本轮未跑**：L301 记账批的基线仍是 `37 failed / 10272 passed / 6 skipped /
  99.62%`，37 项红全部落在既有基线族。本批没有把它说成全绿。

## 余面

- 候选 ②（清单行换成真身）与 ③（维持现状）仍在案，未拍。
- `git gc` 之后，这批对象会从 `dangling` 掉进 `not_a_commit`。L270 的名册届时会红，
  重钉口径仍是「按守卫红读数续钉」，不是把尺子改松。
- 清单行保留原哈希意味着**远端读者拿不到真身**；真身只在 ⑰⑱ 的本地历史叙述里成立。
  这是选 ① 的已知代价，已写进 A348 的定级说明。

## 教训

1. **记录与错账要分开**：游离对象不是「账本写错了」，而是「当时确实提交过这个对象，
   后来被改写」。处置目标是让读者看懂这件事，而不是把历史改成今天的样子。
2. **证据优先于可点击性**：换真身能让哈希在今天的仓库里解析得到，却抹掉了「当时提交
   的是什么」。只在本机成立的真身，不能拿来替换账本里的原哈希。
3. **指向要显式**：保留原哈希还不够，必须在同一行写清「已被改写出分支，真身见 ⑰⑱」，
   否则读者只会看到一枚解析得到、但任何 ref 都到不了的哈希。
4. **只动账本也要重钉尺子**：账本行在六把守卫的量面里，新增一行、改一个定级格都会推动
   常数；改完必须逐把按红读数重钉并留证据。
5. **不把「定向绿」说成「全绿」**：本轮只跑了六把受影响守卫，全量基线仍有 37 项既有红，
   如实记录。

## 参考

- 账本：`augmentor/OPTIMIZATION_LOOP_2.md` L302 清单行、A348 行、L265 块 ⑰⑱
- 提交：`da6426a7f`（L302 账本批）
- 守卫：`augmentor/tests/unit/test_ledger_hash_reachability_l270.py`（可达性五档与游离名册）
- 同族：A344（可达性这一维的立尺）、A349（`not_a_commit` 的批次短哈希子集）、
  A333（名册随账本变长的代价）、L264（逐格点名名册）
