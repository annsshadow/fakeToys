---
title: "dependabot 20 条 open PR 全量分诊：逐条判定、两处代码适配与 main 既有红的归因方法"
date: 2026-10-18
last_refreshed: 2026-10-18
category: development-workflow
module: 全仓（augmentor / oa4rust / laiyipao / llm-lab）+ .github/workflows
problem_type: workflow_improvement
component: dependency_management
severity: medium
symptoms:
  - "20 条 dependabot PR 里混着不自洽的单包升级（react-dom 19 的 peer 要求 react ^19.3.1）与和工具链精确 pin 冲突的升级（@vue/runtime-core 3.5.43 vs @dcloudio 锁 vue == 3.4.21）"
  - "main 上已有 3 个 workflow 红（oa4rust CI / AI platform CI / OA4Rust web quality），合并任何 PR 都会继承红，需要先把『红是谁的』变成可验证的判据"
  - "本机无 Python/gcc；gh 未登录（无法给 PR 留言或手动关闭）；CI 日志匿名 API 403 不可读——结论不能靠 CI 页面，只能靠本地/容器复现"
root_cause: multi_package_upgrades_and_undefined_main_baseline
resolution_type: process_and_dependency_update
tags: [dependabot, dependency-triage, react19, antd6, baseline-gate, fail-loud, vulncheck]
applies_when:
  - "一次性清一批 dependabot 依赖升级 PR"
  - "CI 本身就在红，需要区分『既有红』与『本次引入』"
  - "要在无 gh 登录、读不到 CI 日志的环境里给出逐条判定与证据"
---

# dependabot 20 条 open PR 全量分诊（2026-10-18）

## 1. 处理方式与结果

- 合并方式：本地 `git merge` 依赖分支 → 分支 `chore/dependabot-triage-20261018` → 快进推送 `main`（未走 PR 页面，因为 gh 未登录）。
- 判定口径：**每个子项目用与 CI 等价的本地命令复跑**；红必须回答"是 main 既有红还是本次引入"，办法是把同一门禁在 main 基线（`git worktree` 一份全新检出）上再跑一遍，比较失败集合是否逐字相同。
- 结果：**20 条全部处置完毕**——18 条直接合并关闭；#49 以成套改造取代后关闭（PR 本体只改两个包、不自洽）；#72 关闭（拒绝与 dcloudio 锁步冲突的单升）。引入的业务代码改动只有 2 处（见 §3）。

| PR | 依赖/变更 | 判定 | 关键证据 |
|----|-----------|------|----------|
| #42 | laiyipao/server `golang.org/x/crypto` → 0.58.0 | 合入 | 建库+迁移+播种后 `go build`/`go vet`/`go test ./...` 12 包 ok；`govulncheck` 代码可达漏洞 0 条 |
| #44 | augmentor `nlpcda>=2.5.8` | 合入 | 纯下限收紧；`pip download --no-deps nlpcda>=2.5.8` 有 cp311 轮子 |
| #45 | augmentor `Pillow>=12.3.0` | 合入 | 下限收紧；实测装到 **13.0.0**（升级前 `>=10.0.0` 解析到的也是 13.0.0，解析结果不变） |
| #46 | 根 `@playwright/test` 1.64.0 | 合入 | 与 #64 同一依赖的两处配置；见 #64 |
| #47 | augmentor(dev) `pytest>=9.1.1` | 合入 | 合并后树 `pip install --dry-run -r requirements-dev.txt` 解析到 pytest 9.1.1 |
| #48 | augmentor `pyyaml>=6.0.3,<7.0.0` | 合入 | 同上解析到 PyYAML 6.0.3 |
| #49 | augmentor/web `react-dom` → 19.3 | **改造后合入** | PR 本体只改 react-dom/@types/react-dom，与 peer 要求不自洽，未直接合；改拆成套 React 19（`react`/`react-dom` `^19.3.1` + `@types/react(-dom)` `^19.3.0`）随分诊分支落 main，PR 以 `Closes #49` 关闭，见 §3 |
| #50 | augmentor `sentence-transformers>=6.1.0,<7.0.0` | 合入 | 下限收紧（原 `>=6.0.0`）；轮子可取 |
| #52 | augmentor/web `@ant-design/icons` 6.3.4 | 合入 | 与 #54 必须配套（antd 6 自带 `@ant-design/icons ^6.3.4` 且 peer react>=18） |
| #53 | oa4rust `uuid` 1.28.0 | 合入 | Rust 门禁失败集合与 main 逐字相同 |
| #54 | augmentor/web `antd` 6.6.5 | 合入（需代码适配） | 见 §3；适配后 tsc/eslint/build/vitest(311) 全绿 |
| #56 | oa4rust `tower-http` 0.6.11 | 合入 | 同 #53 |
| #57 | oa4rust `sysinfo` 0.39.6 | 合入 | 同 #53 |
| #59 | oa4rust `lopdf` 0.45.0 | 合入 | 同 #53（供应链例外见 §6） |
| #61 | oa4rust `sea-query` 0.32.7 | 合入 | 同 #53 |
| #62 | augmentor/web `typescript-eslint` 8.71.1 | 合入 | `npm run lint --max-warnings 0` 绿 |
| #63 | llm-lab `pytest>=9.1.1` | 合入 | 容器内 `pip install -r requirements.txt` + pytest：32 passed / 1 skipped |
| #64 | oa4rust `@playwright/test` 1.64.0 | 合入 | `pnpm install --frozen-lockfile` ✓；`playwright test --list` 82 tests ✓ |
| #72 | laiyipao/miniapp `@vue/runtime-core` 3.4.21 → 3.5.43 | **关闭** | 代码零引用；与 dcloudio 锁的 `vue == 3.4.21`（`@vue/compiler-sfc` 精确版）锁步，单升 runtime-core 是拧的工具链。闭合方式：`Closes #72` 随提交进 main（gh 未登录无法手动关） |
| #73 | laiyipao/miniapp `vue-tsc` 3.3.13 | 合入 | 四件套全绿：type-check / vitest 1060 / build:h5 / build:mp-weixin；类型探针（注入错误必须红）实测退出码 2 |

## 2. 归因方法（本仓可直接复用）

1. **先建 main 基线**：`git worktree add ..\fakeToys-main-wt origin/main --detach`，在全新检出的 main 上把红的门禁重跑一遍。
2. **比较失败集合**：本次在分支与 main 上各跑一遍 oa4rust 全量 Rust 门禁（全新数据库），失败集合**逐字相同** ⇒ 5 个 cargo bump 未引入任何新失败。
3. **对"零 `.py` 改动"的 Python 红同理**：分支不碰 Python 源码，15 条 pytest 失败必然是 main 既有；容器实测确认失败均为守卫/元测试与 xlsx 类。
4. **审计类红需要看门禁判据而不是退出码**：`govulncheck` 的 CI 判据是 grep `Your code is affected by [1-9]`，实测"0 vulnerabilities" ⇒ 过闸（即使 Module Results 里有不可达条目）。

## 3. 本次引入的业务代码改动（仅两处）

1. `22f0706d7` **antd 6 选择器适配**：antd 6 把 Select 的 DOM 由 `.ant-select-selector` 调整了结构，6 个页面测试的选择器改为 `.ant-select`，并在 `src/test/setup.ts` 记录第 4 条坑位。45 条用例由红转绿。
2. **成套 React 19**（`augmentor/web/package.json` + lock）：`react-dom 19.3.1` 的 peer 要求 `react ^19.3.1`、`@types/react-dom 19.3.0` 要求 `@types/react ^19.3.0`，dependabot 单改 react-dom 的 PR 不自洽，故四件套同时升。本地门禁：`npm ci` / `npm run lint` / `npm run test:coverage`（22 文件 311 例，stmts 87.32%）/ `npx tsc --noEmit` / `npm run build` 全绿。

## 4. main 既有红清单（push 后归因用，均非本次依赖升级引入）

### 4.1 `CI`（oa4rust，main #81 红）

在 CI 同款工具链（rustc 1.98.1）容器里、以全新数据库复跑：

- `cargo fmt --all -- --check` 大面积漂移（仓库无 `rustfmt.toml`），代表位置：`crates/ai_assemble_control/src/lib.rs`、`crates/ai_assemble_control/tests.rs`。
- `cargo clippy`：`crates/preview/src/lib.rs:262` `manual_pattern_char_comparison`。
- quoted_key_guard：`crates/processplatform_assemble_surface/src/security_gate_tests.rs:1355-1356`，夹具 `("PP_C_WORK", "\"xcreatorPerson\"", "r419-work")` 不在守卫 SKIP_MARKERS（quote_ident/IDENT）内。
- `cargo test --workspace --lib` 6 个 target 失败（与 main 基线逐字相同）：
  - bbs：`test_section_crud_roundtrip`
  - cms：`u3_correlation_update_upsert_real_db`
  - correlation_service_processing：`u2_update_replaces_site_scope`、`u2_create_reports_invalid_targets`
  - message_assemble_communicate：`test_send_message_row_and_conversation_touch_are_atomic`
  - personal_extend：`face_writes_require_owner`
  - processplatform_service_processing：`u2_record_create_is_content_deduped`、`u2_taskcompleted_next_identity_updates_task`

### 4.2 `AI platform CI`（main #96 红）

容器（python:3.11，与 CI 同版本）内 `pip install -r requirements-dev.txt` + `pytest`：

- `15 failed / 10019 passed / 85 skipped`（20:13），失败清单：`test_rmw_serialization_l273`、`test_logging_apply_concurrency_l289`、`test_measurement_rulers_l94`×2、`test_name_path_containment_l259`、`test_parametrize_blind_face_l98`×4、`test_repo_residue_l94`、`test_xlsx_zipbomb_l288`×5。
- frontend-build 侧（`npm ci → lint → test:coverage → tsc → build`）本地全绿，不是红的来源。
- ⚠️ 口径注记：CI 的 pytest 只装 `requirements-dev.txt`（经 `-r requirements-core.txt`，**不含 Pillow**）。人为装上 Pillow 13.0.0 后，`test_untested_guard_branches_l265.py::TestTheStubDoesNotLeak::test_pillow_is_still_absent_after_the_stub_tests` 会失败——那是"Pillow 缺席"前提被破坏的产物，不是回归；同轮出现的 `test_dependency_registry_lock_l290.py` 3 个 ERROR 单独跑该文件为 17 passed（退出码 1 来自单文件跑不满 `--cov-fail-under=80` 的覆盖率门禁），推测为混合运行时的夹具/桩污染，未深挖（本次零 `.py` 改动）。

### 4.3 `OA4Rust web quality`（main #97 红）

在 **main 基线 worktree（6bacf19ed）** 上逐字复现：

- `pnpm lint`（`biome check --error-on-warnings .`，267 文件）→ `Found 15 errors`：3 条具名规则（`frontend/apps/desktop/src/contracts/process-definition.test.ts:5:1` 与 `frontend/apps/desktop/src/views/AttendanceApp.vue:232:1` 的 assist/source/organizeImports、`frontend/apps/desktop/src/views/BBSForum.vue:747:25` lint/suspicious/noControlCharactersInRegex）+ 12 处 formatter 漂移。
- `pnpm --filter @oa4rust/mobile build:mp-weixin` → `TypeError: Cannot read properties of undefined (reading 'context')`，栈：rolldown@1.2.13 resolve ← `@dcloudio/uni-cli-shared` `mp/usingComponents.js:412` parseVueComponentImports ← `@dcloudio/uni-mp-vite` `plugins/mainJs.js:36`。
  **根因（2026-10-19 定案）**：`@dcloudio/vite-plugin-uni` 的 peer 是精确 pin `vite: "5.2.8"`，工作区安全 override 把 `vite@<=6.4.2` 顶到 `6.4.3` ⇒ 真实可用版是 6.4.3；dependabot 把 mobile 的 vite 抬到 8.3.4（vite 8 = Rolldown）后与 dcloudio 插件不兼容，插件上下文里取不到 `context`。修法：`"vite": "6.4.3"` 回钉 + 同步 lock，见 §7。
- 本次两个 playwright bump 只改 `oa4rust/package.json` 一行 + lockfile 里 playwright/playwright-core/fsevents 三组条目，biome 与 vite/@dcloudio 解析不变 ⇒ 上面两处红与本次无关（main 上逐字复现）。

## 5. 环境与命令口径（无 Python/gcc 的 Windows 机器）

- Rust：镜像 `oa4rust-verify:latest`（rustc/cargo 1.98.1，与 CI 同版本）+ `--network host` 复用宿主机 PostgreSQL 容器；容器内 `rustup component add rustfmt clippy`；`CARGO_TARGET_DIR=/tmp/target` 避免污染 Windows 产物。测试硬编码库名 `oa4rust`，并行跑前必须串行用库。
- Python：`python:3.11` 容器（与 ai-platform-ci 一致）；`pip download --no-deps` 验证被 bump 的下限规格在 cp311 有轮子；`pip install --dry-run -r requirements-dev.txt` 在合并后树上复核可解析。
- 前端：宿主 pnpm 11.25.0（与 CI pin 一致；宿主 node 24 与 CI 的 22 有版本差，未构成差异）。
- 关 PR：gh 未登录 / API 403 ⇒ 不能留言与手动关；改用"提交信息含 `Closes #N` 随 push 进 main"让 GitHub 自动关闭。

## 6. 遗留与后续

1. **PR 收尾实况（API 复核）**：18 条在 push 后由 GitHub 判定为 merged（含 #42/#44/#45/#46/#47/#48/#50/#52/#53/#54/#56/#57/#59/#61/#62/#63/#64/#73）；#72 与 #49 以 `Closes #N` 关闭（均为 closed-未合并，符合"不接受该 PR 的改动/被等价改造取代"的意图）。
2. **oa4rust-web 两处既有红（已修，2026-10-19）**：biome 的 15 条里 6 条真红（自动修 + `BBSForum.vue` 手修为 `/[\p{Cc}\u0020]/gu`），另 8 条是**本机 CRLF 造成的假红**（`git ls-files --eol` 证 index=lf）；mp-weixin 按 §4.3 根因把 mobile 的 vite 回钉 6.4.3。全过程见 `main-existing-reds-fix-2026-10-19.md`。
3. **供应链例外到期**：`lopdf` 的 `RUSTSEC-2026-0187` 例外 2026-10-28 到期（见 `docs/solutions/security-issues/rustsec-pinned-audit-double-track-gate.md`），本次 #59 升级到 0.45.0 后需复扫。
4. **push 后 CI 归因**：main 上 `CI`/`AI platform CI`/`OA4Rust web quality` 的既有红已全部修掉（见 §7），
   此后新一轮 CI 若仍红，可直接用"与 main 基线逐字比对"的口径判"本次引入"。

## 7. 跟进：main 既有红全量清零（2026-10-19）

§4 的三份既有红已在分支 `fix/main-existing-reds-20261018` 上逐条修掉（本地 CI 等价验证后进 main）；
完整方法、教训与遗留见 [main-existing-reds-fix-2026-10-19.md](main-existing-reds-fix-2026-10-19.md)。

| 门禁 | 修复要点 | 提交 |
|------|----------|------|
| `cargo fmt --all -- --check` | rustc/rustfmt 1.98.1 全量重排 120 文件（纯机械，单独一笔；用"快照法"从逻辑改动里分离） | `129ddf3e9` |
| `cargo clippy … -D warnings` | 60 余条按仓库先例收口（`module_inception` 41 处 allow、bam 异步锁、auth 文件级 allow 等）。**加 `--keep-going` 才能拿全清单** | `f4a46e360` |
| `cargo test --workspace --lib` | §4.1 的 6 个 target 全部转绿：5 个是夹具/断言失配；`personal_extend` 那条是**产品缺陷**（`face_save` 全列 spec 把 `person_id` 抹成空串） | `87b176706`、`fd089b4e0` |
| `quoted_key_guard` | `ROWS` 夹具改裸列名 + `dialect().quote_ident(col)` 引用（PG 下 camelCase 不带引号会被折成小写） | `87b176706` |
| `pytest`（AI platform CI） | 8 条红清零：l273 注入臂加"读边 50 ms 窗"（flaky → 必现）、l264 改 `--first-parent`、l259 faiss 臂补建目录、xlsx 5 条补 pandas/openpyxl 依赖 | 见新文档 §2.5 |
| `biome` + `build:mp-weixin` | biome 6 条真红 + 8 条 CRLF 假红；mp-weixin 按 §4.3 回钉 vite 6.4.3 | `b73185a78` |
