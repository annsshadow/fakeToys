---
title: "assess: 远端分支相对 main 的未落地功能盘点（26 条分支 / 453 条真残留）"
type: assess
status: active
date: 2026-10-19
scope: origin 全部业务分支（现存 18 条 ＋ 已归档 8 条）vs main@931af210e；路径级 ∪ 抽样复核
origin: 用户要求「既然有 18 条分支，重新看一下有哪些功能没落地到 main，落文档到 docs/plans 供后续实施」；承接 2026-10-19-001 的 8 条老分支删除决议
---

# 远端分支未落地功能盘点

> **结论（2026-10-19 实测，基线 `main@931af210e`）**
>
> 1. **没有整块未落地功能。** 现存 18 条业务分支（10 条 `feat/*` ＋ 8 条非 `feat/*`）独有提交**全部为 0**，
>    已被 main 完全包含；唯一的分支独有内容来自 8 条已删旧线（归档 tag 保留），其中
>    **1,544 条路径经映射命中 main 的对应物**（布局迁移 / 改名 / 演进），真残留 **453 条**，
>    其中具功能价值的候选 **≤ 10 条**（§四 C1）。
> 2. 8 条旧线的分支独有路径共 **29,758** 条：`oa/` 27,761（旧 o2server 整树，历史归档）＋
>    可映射 1,544（`oa4rust` 946 / `oa4rust-web` 234 / `ai` 364）＋ 真残留 453（§三）。
> 3. 本盘点为**路径级 + 抽样**；`2026-10-19-001` 的 M（同名不同内容）清单为内容级差异，维持「低优先、需人读」原判。
> 4. 可选清理：10 条零独有 `feat/*`（含 3 条指向 main 根提交的空壳）可删除（§四 C3）；8 条非 `feat/*`
>    多为他方审计线，仅记录（§四 C5）。

## 一、范围与基线

- 基线：`main` = `origin/main` = **`931af210e`**（工作树干净；本文件定稿时）。
- 远端业务分支共 **26 条**（不含 main 本体），分三类：
  1. **10 条现存 `feat/*`**：`cms-alignment-improvement`、`oa4rust-completion-phase0`、`oa4rust-completion-phase0-1`、
     `oa4rust-mobile`、`oa4rust-remaining-gap-closure`、`oa4rust-residual-gaps-closure`、`remaining-work-consolidation`、
     `systems-documentation`、`three-ends-f-contract-zero`、`watering-poc`；
  2. **8 条现存非 `feat/*`**：`augmentor-opt100`、`augmentor-vuln-audit-20261016`、`docs/plans-status-audit`、
     `laiyipao-r1-engine-port`、`oa4rust-vuln-audit-r4`、`optimism20261012`、`optimize/oa4rust-hardening`、`restore-main`；
  3. **8 条已删旧线**（2026-07-28 ~ 09-20 快照线；2026-10-19 按 001 决议删除，归档 tag 已推送 origin）：
     `auto-checkin`、`core-modules-gap-closure`、`o2server-rust-rewrite`、`oa-project-documentation`、
     `oa4rust-gap-audit`、`prod-readiness-plan`、`quality-improvement-2026-08-08`、`seaorm-migration`
     → 归档名 `archive/feat-<分支短名>-20261019`。

## 二、判据与复核方法

能安全「整块判定」的分支用**提交级**判据；与 main 无共同祖先的快照线只能用**内容级**判据：

- **提交级**：`git rev-list --count <ref> --not main`（独有提交数）；`git cherry main <ref>`（`−` 有等价提交 / `＋` 无）。
- **内容级**：`git -c core.quotepath=false diff --name-status --no-renames <ref> main`；
  `D` = 分支有、main 无；`M` = 同名不同内容；`A` = main 独有。
- **映射规则（8 条旧线，旧世代 → 新世代的路径搬家）**：

  | 旧路径前缀 | main 对应物 | 证据 |
  |---|---|---|
  | `oa4rust/<X>` | `oa4rust/backend/<X>` | crate 名单 **96/96 一致**（`backend/crates` 与旧线同名同位） |
  | `oa4rust-web/apps/desktop/…` | `oa4rust/frontend/apps/desktop/…` | 同名组件 5/5 命中（EChartsView、ScriptCodeEditor、ScriptWorkbench、SqlEditor、XformRuntime） |
  | `oa4rust-web/apps/mobile/…` | `oa4rust/mobile/…` | main 有独立 `oa4rust/mobile` |
  | `oa4rust-web/packages/…` | `oa4rust/frontend/packages/…` | apis / locales / sdk / ui 四包同名 |
  | `oa4rust-web/<根配置>` | `oa4rust/<根配置>` | .npmrc、biome.json、pnpm-workspace.yaml… |
  | `ai/<X>` | `augmentor/<X>` | 同项目改名：README 首段与 `api/` 结构同源 |

- **踩坑记录（复跑必读）**：
  ① `git ls-tree -r --name-only <tree>:<path>` 返回**相对该树**的路径，映射比较时勿再重复加前缀；
  ② 计数一律 `git rev-list --count`（`git log | Measure-Object` 会因提交体含孤立 CR 而少计）；
  ③ 必须 `--no-renames`（默认重命名检测在本仓库会卡死）与 `-c core.quotepath=false`（否则非 ASCII 路径假缺失）；
  ④ PowerShell 中 `"$b:$p"` 会被当盘符引用，需写 `"${b}:$p"`。
- 复核脚本：§六（只读，约 20 秒）。

## 三、盘点结果

### 3.1 零独有分支（18 条，无任何动作）

**现存 `feat/*` 10 条**（`rev-list --count --not main` = 0）：

| 分支 | tip | tip 日期 | 备注 |
|---|---|---|---|
| `cms-alignment-improvement` | `83f892fb9` | 2026-08-21 | 已并入 |
| `oa4rust-completion-phase0` | `1145f0551` | 2026-08-20 | 已并入 |
| `oa4rust-completion-phase0-1` | `4f0cf03d1` | 2026-08-18 | **= main 根提交（空壳）** |
| `oa4rust-mobile` | `0d086a39a` | 2026-09-19 | 已并入 |
| `oa4rust-remaining-gap-closure` | `314c7a753` | 2026-08-21 | 已并入 |
| `oa4rust-residual-gaps-closure` | `653b49f68` | 2026-08-28 | 已并入 |
| `remaining-work-consolidation` | `fee626514` | 2026-08-25 | 已并入 |
| `systems-documentation` | `4f0cf03d1` | 2026-08-18 | **= main 根提交（空壳）** |
| `three-ends-f-contract-zero` | `4ecfd3f6b` | 2026-09-22 | 已并入 |
| `watering-poc` | `4f0cf03d1` | 2026-08-18 | **= main 根提交（空壳）** |

**现存非 `feat/*` 8 条**（全部 `rev-list --count --not main` = 0）：`augmentor-opt100`（`f5a3efb86`）、
`augmentor-vuln-audit-20261016`（`5920baab6`）、`docs/plans-status-audit`（`7f041f249`）、
`laiyipao-r1-engine-port`（`d79e17e4e`）、`oa4rust-vuln-audit-r4`（`b77873564`）、`optimism20261012`（`f45b31930`）、
`optimize/oa4rust-hardening`（`fad49cd9f`）、`restore-main`（`4f0cf03d1`，= main 根提交的空壳）。

> 说明：`restore-main` 与其余三条空壳一样，只是指向 main 根提交的指针；`optimize/oa4rust-hardening` 是
> oa4rust 三端硬化线的尾端，其全部提交已包含在 main（tip 2026-09-14）。

### 3.2 8 条已删旧线：分支独有路径的域分布与判定

8 条归档 tag（`archive/feat-<名>-20261019`）与 main 的 `D`（分支有、main 无）并集共 **29,758** 条：

| 域 | D 并集 | 映射命中 main | 真残留 | 判定 |
|---|---:|---:|---:|---|
| `oa/` | 27,761 | — | 27,761 | 旧 o2server 整树（PHP/Java 世代）；main 由 `oa4rust/` 取代；**历史归档，不实施** |
| `oa4rust/` | 1,016 | **946** | **70** | main 侧布局为 `oa4rust/backend/**`；crate 名单 96/96 一致 |
| `oa4rust-web/` | 242 | **234** | **8** | 仅存在于 `auto-checkin` 线；main 侧为 `oa4rust/frontend/**` ＋ `oa4rust/mobile/**` |
| `ai/` | 409 | **364** | **45** | main 侧为 `augmentor/**`（同项目改名；README 与 api/ 路由同源） |
| `linux-gcov-coverage/` | 15 | 0 | 15 | main 只保留 `configs/`；其余为基线测量脚本与产物 |
| `docs/` | 312 | 0 | 312 | 旧计划/审计/生成文档（`docs/oa`=221） |
| `(root)` | 3 | 0 | 3 | `tmp_update_manifest.js`、`CLAUDE.md`、`feasibility_review.json` |
| **合计** | **29,758** | **1,544** | **453** | 其余域（`laiyipao/`、`augmentor/`、`auto-checkin/`、`cool/`、`.github/`、`scripts/`、`llm-lab/` 等）D=0，全部已在 main |

补充事实：

- `feat/auto-checkin` 线的 1 条独有提交（`658e971fc`，多公益站自动签到工具）经 `git cherry` 判定 **有等价提交**
  （`−=1`），且 main 已有 `auto-checkin/`（`src` ＋ `test`）——功能已落地。
- `oa4rust-web` 只出现在 `auto-checkin` 线（1/8），且该线 `oa4rust-web` 的最近提交为 2026-09-18/19 的
  「三端覆盖」战役——与 main 现行 `oa4rust/frontend` 的战役同源。
- main 侧结构（对照用）：`oa4rust/` 顶层 = `.npmrc, README.md, backend, biome.json, e2e, frontend, mobile, package.json,
  playwright.config.ts, pnpm-lock.yaml, pnpm-workspace.yaml, scripts, tests, tsconfig.base.json, unocss.config.ts, vendor,
  vitest.config.ts`；`oa4rust/backend/` 顶层 = `.cargo, .env.example, .env.test.example, .setup_role.sql, Cargo.lock,
  Cargo.toml, README.md, add_routes.js, add_stubs.py, apply_routes.js, check_tables.sql, compile.bat, crates, debug_bbs.js,
  debug_bbs3.js, deny.toml, deploy, docker-compose.yml, docs, examples, migrations, scripts, security, src,
  swagger-ui-v5.17.12.zip, tests`。

### 3.3 真残留 453 条明细（按域，判定见括号）

**`oa4rust/` 70 条**：

- `scripts/` **47**：一次性战役脚本与输出（`bisect_*`、`fix_*`、`gen_*`／`generate_stubs`、`analyze_gap*`、
  `o2server_parity_audit(_v2)`、`verify_*`、`find_*`、`extract_java_paths`、`count_*`、`debug_*`、`repair_router_calls`、
  `scan_recursion`、`realize_one`、`migrate_auth_data`、`endpoints_output.txt`、`gap_output.txt`、
  `scripts/Cargo.{toml,lock}`、`scripts/src/main.rs` 等）→ **不实施**（工具留存于归档 tag，可查）。
- `tests/` **7**：`behavior_compare.rs`、`behavior_compare_endpoints.rs`、`behavior_comparison/{mod,comparator,reporter,allowlist}.rs`、
  `behavior_comparison/allowlist.yaml` → **C1a 人读**（对照 main `crates/parity`）。
- `crates/` **4**：`shared/src/db.rs`（main 已重构为 `src/db/{mod,dialect,rewriter}.rs`）、`shared/src/middleware.rs`、
  `shared/src/middleware/routing.rs`（main 为 `src/middleware/*` ＋ `src/router.rs`）、`personal_extend/src/password.rs`
  （main 为 `auth/src/password.rs`、`personal/src/password.rs`）→ **已被重构覆盖，不实施**。
- `migrations/` **2**：`008_cleanup_duplicates.sql`、`009_correlation_tables.sql`（main 同编号为
  `008_file_tables.sql`、`009_person_group_tables.sql`）→ **C1b 人读**。
- 配置/工件 **10**：`.github/workflows/ci.yml`（main 根已由 `oa4rust-ci.yml` 等 7 条工作流取代）、
  `build_errors.txt`、`cargo_err.txt`、`deploy/rollback-playbook.md`、`docs/brainstorms/oa4rust-complete-implementation-plan.md`、
  `docs/plans/2026-08-08-001-feat-multi-axis-quality-plan.md`、`docs/plans/2026-08-10-002-fix-oa4rust-gap-closure-plan.md`、
  `docs/REALIZE_RUNBOOK.md`、`findings_correctness.json`、`migrations-review-result.json` → **不实施**。

**`oa4rust-web/` 8 条**：

- `apps/desktop/src/views/AIAssistant.vue` → **C1c 人读**（main 同目录无此视图，疑由 `AIChatApp.vue` 承接）。
- `docs/solutions/` 7 份覆盖度报告（`api-coverage-strategy.md`、`coverage-achievement-analysis-2026-09-18.md`、
  `final-achieve-95-coverage-report.md`、`final-coverage-report.md`、`final-coverage-report-2026-09-17.md`、
  `test-coverage-95-target-plan.md`、`test-coverage-summary.md`）→ **不实施**（文档）。

**`ai/` 45 条**：`tests/unit` 29（旧 `*_opt` 系列等）＋ `tests/integration/test_cli_enhanced_commands.py`
＋ `docs/` 6（API.md、API_CORE.md、ARCHITECTURE.md、DEPLOYMENT.md、README.md、USAGE_EXAMPLES.md）
＋ `train_data.json`、`train_data_final.json`、`train_data_final01.json`、`train_data_final02.json`
＋ `bak/` 4（train_data.json、train_data.jsonl、last_line.json、train_data-bd.txt）＋ `enhanceTXT.py`
→ **不实施**（旧测试/数据/备份；`augmentor` 已有演进版套件）。

**`linux-gcov-coverage/` 15 条**：`.gitignore`、`_progress_monitor.sh`、`analyze_gap.py`、`baseline/baseline/coverage.info`、
`baseline/baseline/html/index.html`、`baseline/baseline_report.json`、`baseline/preliminary_report.json`、
`baseline_measurement.py`、`configs/coverage_uml.config`、`Dockerfile.baseline`、`docs/2026-08-12-weekly-report.md`、
`probe_link.sh`、`recapture.py`、`run_baseline_docker.sh`、`run_docker.sh` → **可选归档**（C2）。

**`docs/` 312 条**：`docs/oa` 221（旧系统 API 生成文档）、`docs/plans` 42（2026-07-30 ~ 08-10 旧计划）、
`docs/audits` 34、`docs/reviews` 9、`docs/weekly` 3、`docs/ideation` 1 → **可选归档**（C2）。

**`(root)` 3 条**：`tmp_update_manifest.js`、`CLAUDE.md`（main 已由 `AGENTS.md`／`MEMORY.md` 体系取代）、
`feasibility_review.json` → **不实施**。

## 四、后续实施候选（供拍板；勾选即关闭）

| 编号 | 候选 | 规模 | 建议动作 | 判据 / 命令 |
|---|---|---:|---|---|
| **C1a** | `tests/behavior_comparison` 行为对比自检 | 7 文件 | 人读（≤15 分钟）→ 关闭或搬运 | 对照 `main:oa4rust/backend/crates/parity`；`git show archive/feat-oa4rust-gap-audit-20261019:oa4rust/tests/behavior_comparison/comparator.rs` |
| **C1b** | `008_cleanup_duplicates.sql`、`009_correlation_tables.sql` | 2 文件 | 人读 → 关闭，或按新编号补迁移 | `git show <tag>:oa4rust/migrations/008_cleanup_duplicates.sql` vs `main:oa4rust/backend/migrations/008_file_tables.sql` |
| **C1c** | `AIAssistant.vue` 桌面视图 | 1 文件 | 人读 → 关闭或搬运 | 对照 `main:oa4rust/frontend/apps/desktop/src/views/AIChatApp.vue` |
| **C2** | 历史资料归档（linux-gcov-coverage 15 ＋ docs 312） | 327 | 可选：无视，或 `git restore --source=<tag> -- <path>` 归档 | 不做也不影响 main |
| **C3** | 删除 10 条零独有 `feat/*` | 10 条 | 可选清理（需拍板；流程见 §五） | `git rev-list --count origin/<b> --not main` = 0；3 条空壳指向 `4f0cf03d1` |
| **C4** | `oa/` 27,761 ＋ 旧世代 M 清单 | — | **不实施**（维持 001 判读） | — |
| **C5** | 8 条非 `feat/*` 分支 | 8 条 | 仅记录；清理需各方确认 | 多为审计线，内容已全部在 main |

执行顺序建议：先做 **C1**（三件人读，合计 ≤45 分钟）；若 C1a–C1c 全部关闭，本计划即可收口
（`status: active → completed`），C2/C3 由用户拍板后按 §五 流程另行执行。

## 五、行动边界与删除流程

- **不 merge / 不 rebase / 不用 `--allow-unrelated-histories`**（8 条旧线与 main 无共同祖先，合并语义不成立）。
- **搬运一律单向**：`git show <tag>:<路径> > <目的路径>` 或 `git restore --source=<tag> -- <路径>`，经评审后入 main。
- **删除远端分支流程**（8 条旧线的先例，本轮已完整走完）：
  ① 打 annotated tag：`git tag -a archive/<分支短名>-<日期> <tip-sha> -m "<说明>"` 并 `git push origin`；
  ② 知会相关方；③ `git push origin --delete <refs>`；④ 本地 `git fetch --prune`。
  恢复方法：`git branch <名> archive/<分支短名>-<日期>`。
- 本盘点除 8 条旧线的删除（前一轮决议）外，未改动任何远端状态。

## 六、复核脚本（只读，可复跑）

```powershell
# 前提：8 条归档 tag 在本仓库（git tag -l 'archive/*' 应输出 8 条）
[Console]::OutputEncoding = [System.Text.Encoding]::UTF8
$tags   = @(git tag -l 'archive/*')
$mainB  = @{}; @(git ls-tree -r --name-only main:oa4rust/backend) | ForEach-Object { $mainB[$_]  = $true }
$mainOa = @{}; @(git ls-tree -r --name-only main:oa4rust)         | ForEach-Object { $mainOa[$_] = $true }
$aug    = @{}; @(git ls-tree -r --name-only main:augmentor)       | ForEach-Object { $aug[$_]    = $true }
$U = @{}
foreach ($t in $tags) {
  foreach ($r in @(git -c core.quotepath=false diff --name-status --no-renames $t main)) {
    if ($r -like 'D*') { $U[($r -split "`t")[-1]] = $true }
  }
}
$r1 = @($U.Keys | Where-Object { $_ -like 'oa4rust/*' -and -not $mainB.ContainsKey($_.Substring(8)) })
$r2 = @($U.Keys | Where-Object { $_ -like 'oa4rust-web/*' -and -not (
        $mainOa.ContainsKey('frontend/' + $_.Substring(12)) -or
        $mainOa.ContainsKey($_.Substring(12)) -or
        ($_.Substring(12) -like 'apps/mobile/*' -and $mainOa.ContainsKey('mobile/' + $_.Substring(12).Substring(12)))) })
$r3 = @($U.Keys | Where-Object { $_ -like 'ai/*' -and -not $aug.ContainsKey($_.Substring(3)) })
"oa4rust 残留={0}  oa4rust-web 残留={1}  ai 残留={2}" -f $r1.Count, $r2.Count, $r3.Count   # 期望 70 / 8 / 45
$r1 | Sort-Object; $r2 | Sort-Object; $r3 | Sort-Object
# 其余域计数（期望值）：
@($U.Keys | Where-Object { $_ -like 'oa/*' }).Count                   # 27761
@($U.Keys | Where-Object { $_ -like 'docs/*' }).Count                 # 312
@($U.Keys | Where-Object { $_ -like 'linux-gcov-coverage/*' }).Count  # 15
@($U.Keys).Count                                                      # 29758
```

## 附录 A：001 决议与本次删除执行记录

- 依据：`docs/archived-plans/2026-10-19-001-assess-stale-remote-branches-vs-main-plan.md` §五 / §六。
- 执行（2026-10-19，第二程）：
  1. `git tag -a archive/feat-<名>-20261019 <tip>` ×8（annotated，含 tip sha 与 001 文档引用）；
  2. `git push origin`（8 个 tag 已在远端；`git ls-remote --tags origin 'refs/tags/archive/*'` 可见）；
  3. `git push origin --delete` ×8；`git fetch --prune`（远端 `feat/*` 现余 10 条）。
- 恢复：`git branch <名> archive/feat-<名>-20261019`，或 `git restore --source=archive/… -- <路径>`。

## 附录 B：判定要点速查

- 本盘点回答「未落地功能」= **无整块缺失**；453 条真残留里只有 C1 三件（≤10 文件）值得人读，其余为
  旧世代脚本 / 旧测试 / 数据备份 / 历史文档 / 工件，**默认不实施**。
- 所有「已落地但换位」的内容以**路径映射 + 同名同构抽样**为证（§三）；内容级差异（M）仍以 001 §五 为准。
- 复核数字一律以重跑 §六 为准；main 继续演进后，期望值需重算（残留只会更少）。
