---
title: "assess: 8 条远端老分支相对 main 的产出比对方案"
type: assess
status: active
date: 2026-10-19
scope: origin/feat/* 8 条（2026-07-28 ~ 08-14）vs main@ff75059d2；824 条提交、约 3 万条分支独有路径
origin: 用户提问「main 是否最新、是否还需要合并其它分支」→ 实测发现 8 条老分支与 main 无共同祖先，需要一套可复算的内容级比对流程来回答「里面还有没有值得抢救的产出」
---

# 8 条远端老分支相对 main 的产出比对方案

## Summary

起因是一个简单的问题：`main` 是最新的吗、还要不要把其它分支合并进来？查完的答案是三句话：

1. **main 就是最新**：`main` = `origin/main` = `ff75059d2`（领先 0 / 落后 0）；本地 5 条分支
   （`augmentor-vuln-audit-20261016` / `chore/dependabot-triage-20261018` / `laiyipao-r1-engine-port` /
   `oa4rust-vuln-audit-r4` / `optimism20261012`）独有提交**全为 0**，全部已在 main 里，无事可做。
2. 但远端有 **8 条 `feat/*` 老分支带着 main 里没有的提交**（合计 824 条，停在 2026-08-14 之前）。
   它们的处理方式**不是"要不要合"**——实测 `git merge-base --all main <分支>` 全部为 **0 条**：
   main 的根提交是 `4f0cf03d1 chore: restore repository state from main branch snapshot`（2026-08-18
   从快照重建），与这 8 条线**没有任何共同祖先**。合并需要 `--allow-unrelated-histories`，等于把两条
   独立线缝在一起，风险远超收益。**唯一可用的判据是内容比对**（路径 / blob），本文件就是这套流程。
3. **与本仓库当前主线的关系**：`laiyipao/**` 与 `augmentor/**` 在 8 条分支上 **D=0、M=0**
   （分支里根本没有这两个域；`A` 表示 main 独有：`laiyipao/` 452 条、`augmentor/` 628 条）；
   关联只集中在 `oa4rust/**`，且是**世代断代**：两代同名路径中内容不同的**只有 1 条**
   （`oa4rust/README.md`），其余要么只在分支（旧世代）、要么只在 main（新世代）。

本方案已在本机以 `main@ff75059d2` **预演跑通**（§五），执行脚本只读、8 条分支合计约 10 秒（§四）。

---

## 一、基线事实（2026-10-19 实测）

- `main` = `origin/main` = **`ff75059d2`**；本地工作树干净。
- `git merge-base --all main <分支>` = **0 条**（8 条全部如此）⇒ 无共同祖先。
- 8 条分支共 824 条提交（其中 merge 33 条），每条分支自己就是一条**快照线**（根提交即整树导入）。

| 分支 | 提交 | merge | cherry −/+ | D（分支独有） | M（同名不同内容） | A（main 独有） | D 顶层分布 |
|------|-----:|------:|-----------:|-------------:|-----------------:|-------------:|------------|
| `origin/feat/auto-checkin` | 1 | 0 | **1**/0 | 1884 | 87 | 2585 | oa4rust=946 ai=409 docs=286 oa4rust-web=242 |
| `origin/feat/core-modules-gap-closure` | 224 | 10 | 0/214 | 28771 | 3989 | 2687 | oa=27761 oa4rust=732 docs=251 |
| `origin/feat/o2server-rust-rewrite` | 84 | 1 | 0/83 | 28398 | 14 | 6746 | oa=27761 oa4rust=442 docs=183 |
| `origin/feat/oa-project-documentation` | 16 | 0 | 0/16 | 27949 | 3 | 6760 | oa=27760 docs=178 ai=10 |
| `origin/feat/oa4rust-gap-audit` | 153 | 7 | 0/146 | 28588 | 43 | 6714 | oa=27761 oa4rust=587 docs=227 |
| `origin/feat/prod-readiness-plan` | 130 | 7 | 0/123 | 28536 | 40 | 6718 | oa=27761 oa4rust=539 docs=223 |
| `origin/feat/quality-improvement-2026-08-08` | 98 | 1 | 0/97 | 28413 | 16 | 6744 | oa=27761 oa4rust=456 docs=184 |
| `origin/feat/seaorm-migration` | 118 | 7 | 0/111 | 28500 | 17 | 6742 | oa=27761 oa4rust=538 docs=188 |

三个主线域的分布（D = 分支有 main 没有；A = main 有分支没有）：

| 域 | D（8 条分支） | M | A（main 独有） |
|----|--------------|---|---------------|
| `laiyipao/` | **0** | **0** | 452 |
| `augmentor/` | **0** | **0** | 628 |
| `oa4rust/` | 442 ~ 946 | **1**（`oa4rust/README.md`） | 1333 ~ 1334 |

复算命令（单条，秒级）：

```powershell
git merge-base --all main 'origin/feat/seaorm-migration'          # 预期：无输出，退出码 1
git rev-list --count 'origin/feat/seaorm-migration' --not main    # 预期：118
git -c core.quotepath=false diff --name-status --no-renames 'origin/feat/seaorm-migration' main | Measure-Object -Line
```

---

## 二、为什么只能比内容（以及三条仪器规则）

### 2.1 `git cherry` 的语义与边界

`git cherry main <branch>` 逐条列出分支独有提交并给判定：

- `-`：main 中**存在补丁等价**的提交（patch-id 相同）⇒ 高置信「已落地」。
  本批实测样例：`origin/feat/auto-checkin` 的 1 条提交判 `-`，且**独立证实**其工具确在 main
  （`auto-checkin/` 目录在 main 有 12 笔提交、该提交 25 个路径全部存在于 main、其中 6 个逐字节一致）。
- `+`：**只表示"没找到逐字节等价的补丁"，不等于没落地**。重写、重命名、合批提交、快照重建、
  上下文移位都会让同一次改动变成 `+`。所以 `+` 必须由内容级比对兜底。
- `git cherry` **跳过 merge 提交**：824 − 791 = 33 = merge 数。需要时单独用
  `git rev-list --count --merges <分支> --not main` 列。

### 2.2 三条仪器规则（都是实测踩出来的，照做即可）

1. **计数一律 `git rev-list --count`**。本机实测 `git log --oneline main | Measure-Object` = 1990，
   而 `git rev-list --count main` = 3092；`git log --format='%H'` 又是 3092。
   本仓历史里存在含孤立 CR 的提交信息，PowerShell 按行收集时会把行并起来（具体到哪几条未逐条追证，
   但结论确定：**别用 `git log | Measure-Object` 做计数**）。
2. **必须 `--no-renames`**。`git show --name-status` 默认做重命名检测，面对这些老树是 O(文件数²)：
   实测跑到 renameLimit 27628 的警告后卡死 40 分钟。加上 `--no-renames` 后整树 diff 只用 0.3~1.5 秒。
3. **必须 `-c core.quotepath=false`**，否则非 ASCII 路径会以 `"\346\226\207.md"` 转义形式返回，
   后续 `main:$path` 查找全部落空，制造大量假的"main 里没有"。

另有两坑：① **不要逐路径 spawn git**（曾用"每文件两次 git 调用"，面对 2.7 万路径的提交等于 5 万+ 次
进程启动 ≈ 27 分钟；改为整树一次 diff 即可）；② PowerShell 里写 `"$b:$p"` 会被当成**盘符引用**解析报错，
必须写 `"${b}:$p"`。

### 2.3 一个试过但无判别力的思路（勿重复）

`分支独有提交改过的路径 ∩ D/M` 曾是设想的主力判据，**实测无判别力**：这些分支自身就是整树快照线
（一条 224 提交的分支"改过" 33644 个路径 ≈ 整棵树），交集结果与不做交集的 D/M 完全相同，已弃用。

---

## 三、判据与结论口径

对比的输入只有两类，输出只允许三种结论。

**A. `D`（分支有、main 没有）** —— 分两类看待：

- 属**旧世代 / 归档前路径**（`oa/`、`ai/`、`docs/` 旧路径、`CLAUDE.md`、`cool/`、`.workbuddy/`、
  旧世代的 `oa4rust/` 等）⇒ 默认视为**已被取代**，不动。量级证据：`oa/` 一项就是 27761 条，
  且只有 1 条提交的 `auto-checkin` 分支也带 1884 条 D —— D 的主体是"main 自己后来删掉 / 改名 / 归档过的路径"，
  不是各分支的产出。
- 属**当前主线域且人工判定确有价值**（`oa4rust/` 旧世代文件、`laiyipao/`、`augmentor/`）⇒ 记为
  **候选抢救**，逐文件搬运（`git show <分支>:<路径>` 落盘 → 评审 → 手动入 main），**不做 merge**。

**B. `M`（同名、内容不同）** —— **需人读**，本批量级可控（3 ~ 43 条/分支，仅 `core-modules` 为 3989）。
读之前先做两道复核（都已验证过，照着做）：

```powershell
# 复核 1：是否只是行尾 / 空白差异（防呆用：本批抽样 40 条 docs 差异，实测 0 条被消解、全部为实质差异）
git diff --ignore-cr-at-eol --numstat "${branch}:$path" "main:$path"
# 复核 2：差异量级（决定值不值得读内容）
git diff --numstat "${branch}:$path" "main:$path"
```

**C. `cherry -`** ⇒ 该分支的独有提交已被 main 覆盖，可归入「冗余」。

**结论只允许三种**：`冗余（可删）` / `候选抢救(k 条)` / `需人读(j 条)`。
每条结论必须附证据：命令 + 计数 + 样例（sha 或路径）。

---

## 四、执行脚本（只读，ASCII 安全）

已在本机以 `main@ff75059d2` 跑通（8 条分支合计约 10 秒）。存成 `stale-branch-triage.ps1`
直接执行即可，脚本不建分支、不切工作树、不删任何东西。

```powershell
# stale-branch-triage.ps1 -- read-only comparison of the 8 old remote lines vs main
# Run:  powershell -ExecutionPolicy Bypass -File stale-branch-triage.ps1
# Rules (see the plan doc, section 2): counts via git rev-list; --no-renames;
# -c core.quotepath=false; use ${branch}: in interpolated strings.
$ErrorActionPreference = 'Continue'
try { [Console]::OutputEncoding = [System.Text.Encoding]::UTF8 } catch { }
$OutputEncoding = New-Object System.Text.UTF8Encoding $false
$domains  = @('oa4rust/', 'laiyipao/', 'augmentor/')
$branches = @(
  'origin/feat/auto-checkin',
  'origin/feat/core-modules-gap-closure',
  'origin/feat/o2server-rust-rewrite',
  'origin/feat/oa-project-documentation',
  'origin/feat/oa4rust-gap-audit',
  'origin/feat/prod-readiness-plan',
  'origin/feat/quality-improvement-2026-08-08',
  'origin/feat/seaorm-migration'
)
"main = {0} @ {1}" -f (git rev-parse --short HEAD), (git rev-parse --abbrev-ref HEAD)
foreach ($b in $branches) {
  $uniq   = [int](git rev-list --count $b --not main)
  $merges = [int](git rev-list --count --merges $b --not main)
  $cherry = @(git --no-pager cherry main $b)
  $minus  = @($cherry | Where-Object { $_ -like '- *' }).Count
  $plus   = @($cherry | Where-Object { $_ -like '+ *' }).Count
  $rows   = @(git -c core.quotepath=false diff --name-status --no-renames $b main)
  $D = New-Object System.Collections.ArrayList
  $M = New-Object System.Collections.ArrayList
  $a = 0
  foreach ($r in $rows) {
    $bits = $r -split "\t"
    $st = $bits[0].Substring(0,1)
    $p  = $bits[-1]
    if ($st -eq 'D') { [void]$D.Add($p) } elseif ($st -eq 'M') { [void]$M.Add($p) } elseif ($st -eq 'A') { $a++ }
  }
  $dTop = @{}; $mTop = @{}
  foreach ($p in $D) { $k = ($p -split '/')[0] + '/'; if ($p -notmatch '/') { $k = '(root)' }; $dTop[$k] = 1 + ($dTop[$k] -as [int]) }
  foreach ($p in $M) { $k = ($p -split '/')[0] + '/'; if ($p -notmatch '/') { $k = '(root)' }; $mTop[$k] = 1 + ($mTop[$k] -as [int]) }
  "==== {0}" -f $b
  "  commits: unique={0} (merges={1})   cherry: minus={2} plus={3}" -f $uniq, $merges, $minus, $plus
  "  vs main: D={0}  M={1}  A={2}" -f $D.Count, $M.Count, $a
  "  D top dirs: " + ((($dTop.GetEnumerator() | Sort-Object Value -Descending | Select-Object -First 6) | ForEach-Object { "{0}={1}" -f $_.Key, $_.Value }) -join '  ')
  "  M top dirs: " + ((($mTop.GetEnumerator() | Sort-Object Value -Descending | Select-Object -First 6) | ForEach-Object { "{0}={1}" -f $_.Key, $_.Value }) -join '  ')
  foreach ($dm in $domains) {
    $dIn = @($D | Where-Object { $_ -like "$dm*" })
    $mIn = @($M | Where-Object { $_ -like "$dm*" })
    "  {0}  D={1} M={2}" -f $dm, $dIn.Count, $mIn.Count
    if ($mIn.Count -gt 0) { "    M: " + (($mIn | Select-Object -First 20) -join ' | ') }
    if ($dm -eq 'oa4rust/' -and $dIn.Count -gt 0) { "    D sample: " + (($dIn | Select-Object -First 8) -join ' | ') }
  }
}
```

预期输出形态（以预演为准）：每条分支一段，`minus` 只有 `auto-checkin` 为 1、其余为 0；
`laiyipao/` 与 `augmentor/` 恒为 `D=0 M=0`；`oa4rust/` 每条的 `M` 只列 `oa4rust/README.md`。

---

## 五、预演结果（2026-10-19，`main@ff75059d2`）

| 分支 | cherry | 域内 D/M | 判读 |
|------|--------|----------|------|
| `auto-checkin` | **−=1** | laiyipao 0/0、augmentor 0/0、oa4rust 946/1 | **冗余**：其工具已在 main（可删候选） |
| `core-modules-gap-closure` | +=214 | laiyipao 0/0、augmentor 0/0、oa4rust 732/1 | D 主体 `oa/`=27761（旧世代）；M=3989 需人读（`docs/`=3981，抽样 40 条全为实质差异，非行尾） |
| `o2server-rust-rewrite` | +=83 | laiyipao 0/0、augmentor 0/0、oa4rust 442/1 | 旧世代整树；M=14（`docs/`=10）需人读 |
| `oa-project-documentation` | +=16 | laiyipao 0/0、augmentor 0/0、oa4rust 0/0 | 纯文档线；M=3（`(root)`=2、`cool/`=1）需人读 |
| `oa4rust-gap-audit` | +=146 | laiyipao 0/0、augmentor 0/0、oa4rust 587/1 | 旧世代 oa4rust 树；M=43（`docs/`=39）需人读 |
| `prod-readiness-plan` | +=123 | laiyipao 0/0、augmentor 0/0、oa4rust 539/1 | 同上；M=40（`docs/`=36）需人读 |
| `quality-improvement-2026-08-08` | +=97 | laiyipao 0/0、augmentor 0/0、oa4rust 456/1 | 同上；M=16（`docs/`=12）需人读 |
| `seaorm-migration` | +=111 | laiyipao 0/0、augmentor 0/0、oa4rust 538/1 | 同上；M=17（`docs/`=13）需人读 |

预演判读：

- **没有任何一条分支带 `laiyipao` / `augmentor` 的内容**（两个域在分支上完全不存在）。
- `oa4rust/` 的"候选"是**旧世代整树**（442 ~ 946 条路径/分支）与 1 条同名差异
  （`oa4rust/README.md`，差异量 19 增/29 删，非行尾）。是否抢救属产品/各方判断。
- M 类每条分支只有个位数到几十条（`core-modules` 例外，3989 条），且集中在
  `docs/`、`(root)`（`README.md` / `.gitignore` 等）、`cool/`、`.workbuddy/` —— 都是旧线自身的文件。
- 结论口径建议：8 条分支**均不需要合并**；如需收口，按 §三 判据产出逐分支结论后再动远端。

---

## 六、执行后的动作与边界

- **不 merge、不 rebase、不用 `--allow-unrelated-histories`**（无共同祖先，合并语义不成立）。
- 若最终判定为「冗余」要收口远端：先 `git tag archive/<分支短名>-20261019 <sha>`（本地打标，
  是否推送另定）→ 再删远端分支；这些分支属各方产出，**删除前先知会**。
- 「候选抢救」只做文件搬运：`git show <分支>:<路径> > <目的路径>`，经评审后手动入 main。
- 比对完成后：把逐分支结论回填本文件 §五，`status` 收口为 `completed`，并按
  `docs/solutions/development-workflow/plan-status-lifecycle.md` 的惯例处理归档。

**本次范围声明**：本文件只新增比对方案（`docs/plans/` 按需新建），未删除任何分支、未改动任何远端状态。
