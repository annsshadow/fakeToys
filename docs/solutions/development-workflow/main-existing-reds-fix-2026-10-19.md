---
title: "main 既有红全量修复：Rust fmt/clippy/测试、Python 8 条、web biome 与 mp-weixin 构建（2026-10-19）"
date: 2026-10-19
last_refreshed: 2026-10-19
category: development-workflow
module: 全仓（oa4rust backend/frontend/mobile + augmentor）+ .github/workflows
problem_type: baseline_gate_erosion
component: ci_baseline_repair
severity: high
symptoms:
  - "main 三个 workflow 长期全红：CI（Rust fmt 漂移 ~120 文件、clippy 60 余条、6 个测试 target 红）、AI platform CI（pytest 8 条红）、OA4Rust web quality（biome 15 条 + build:mp-weixin 崩）"
  - "红积压后任何新 PR 都继承红，无法区分『既有红』与『本次引入』；本轮要求把既有红全部修掉（不只归因）"
root_cause: baseline_gate_erosion_after_authz_campaign
resolution_type: baseline_repair
tags: [baseline-gate, rustfmt, clippy-keep-going, fixture-rot, vite-pin, rolldown, crlf-eol, xlsx-hidden-deps, fail-loud]
applies_when:
  - "CI 红积压到『不知从哪条开始修』，需要逐门禁推进"
  - "要在无 Python/gcc 的本机（Windows）用容器跑与 CI 等价的门禁"
  - "需要把『产品缺陷』与『测试口径失配』分开定性"
---

# main 既有红全量修复（2026-10-19）

承接 [dependabot 20 条 PR 全量分诊](dependabot-pr-triage-2026-10-18.md)：分诊只把"红是谁的"变成判据，
本笔把 `main` 上的既有红**全部修掉**。原则：能改业务代码就改；修不动/有争议就如实停手（本轮回合无此情形，
但把一处定性为**产品缺陷**，见 §2.3）。

## 0. 结果摘要

| 门禁（CI） | 修复前 | 修复后（本地 CI 等价复跑） |
|------------|--------|------------------------------|
| `cargo fmt --all -- --check` | 漂移 120 文件 | EXIT=0（全量重排单独一笔：`129ddf3e9`） |
| `cargo clippy … -- -D warnings` | 60 余条 | EXIT=0，0 error（`f4a46e360`） |
| `cargo test --workspace --lib` | 6 个 target 红 | 0 failed（`87b176706`；产品缺陷修 `fd089b4e0`） |
| `pytest`（AI platform CI） | 8 failed | 0 failed（见 §2.5 与 §3） |
| `biome check --error-on-warnings .` | 15 条 | 0 报项 / 267 文件（`b73185a78`） |
| `build:mp-weixin` | 必崩 | 构建通过（vite 回钉 6.4.3，`b73185a78`） |

## 1. 口径与提交拆分

- **判据只用本地 CI 等价命令**（读 `.github/workflows/*.yml` 的 run 原文，含 working-directory 与工具链版本），
  不靠 CI 页面（gh 未登录、匿名 API 403）。
- 拆分：fmt 独立一笔（纯机械、~120 文件），其余按主题：clippy 收口 / 6 条用例根因 / personal_extend 产品缺陷 /
  Python / web+lock / docs。每笔提交信息写明"改了哪一类、为什么、验证命令与结果"。

### 1.1 fmt 单独一笔的"快照法"

`git diff -w` 分离不出格式化漂移——本轮 156 个改动文件里 155 个有结构变化（换行/参数位置），不是纯空白差异。
因此用快照法：

1. 快照 backend 全部逻辑改动到临时目录（143 文件）；
2. `git checkout -- oa4rust/backend` 还原到 HEAD；
3. `cargo fmt --all`（CI 同版工具链）；
4. 提交纯 fmt（120 文件，+4073/−1731）；
5. 贴回快照，此后所有逻辑提交都基于重排后的代码，diff 里不再混格式噪声。

宿主 rustup 恰好装了 `1.98.1-x86_64-pc-windows-gnu`（含 rustfmt 1.9.0-stable，与容器/CI 同版），
`cargo +1.98.1 fmt --all` 即可，不必为了 fmt 重建容器。

## 2. 逐门禁

### 2.1 Rust fmt（`129ddf3e9`）

`cargo fmt --all` 后 `--check` EXIT=0；无 `rustfmt.toml`，纯默认风格。重排后宿主与容器各验一次 `--check`。

### 2.2 Rust clippy（`f4a46e360`）

**坑：必须 `--keep-going`。** cargo 默认在第一个失败的 crate 中止，首轮只能看到"第一批"错误；
本轮修完第一波后又冒出两个当时依赖未编译到的文件（`security_gate_tests_b74/b76`）。

| lint | 数量 | 收口方式 |
|------|------|----------|
| `module_inception` | 41 | `security_gate_tests*.rs` 文件内同名 `mod` 上方加 `#[allow]`（改名会动 lib.rs 声明与测试路径；lib.rs 早有同类先例）。**同形状的 `tests.rs` 实测不触发**，无需处理 |
| `needless_borrow` | 4 | `message_assemble_communicate/src/lib.rs`：参数本身已是 `&str`，去掉多余的 `&` |
| `await_holding_lock` | 2 | `processplatform_assemble_bam/src/tests.rs`：`STATE_DELTA_LOCK` 由 `std::sync::Mutex` 换 `tokio::sync::Mutex::const_new(())`，取锁 `.lock().await`（要罩整段带 await 的用例，异步锁才是语义正解） |
| `result_unit_err` | 4 | `auth/src/password.rs` 文件级 `#![allow]` + 理由注释（解码失败原因刻意不外泄，细分成具名错误类型反而制造试探 oracle） |
| `doc_lazy_continuation` | 4 | 文档注释续行前补空 `//!` |
| `redundant_pattern_matching` | 2 | `matches!(x, Ok(_))` → `x.is_ok()` |
| 其余各 1 | 3 | `useless_vec`（`vec![…]` → 数组）、`type_complexity`（测试内抽 `type ApiResult` 别名）、`manual_pattern_char_comparison`（`rsplit(|c| …)` → `rsplit(['/', '\\'])`） |

### 2.3 Rust 测试：6 个 target（`87b176706`，产品缺陷 `fd089b4e0`）

安全审计加门（会话 + admin/归属校验）后，6 个 target 的用例在**夹具层**失配——逐条定位：

| 用例 | 原红机制 | 修法 |
|------|----------|------|
| bbs `test_section_crud_roundtrip`（403 臂） | 拿"挂着 admin 探针会话"的 app 发 `None` 请求，量到 admin 档（200） | 改 `crate::router(pool)` + 显式非 admin 会话，真正量 403 |
| cms `u3_correlation_update_upsert_real_db` | upsert 已收「会话 + admin」双闸，用例用自造身份 | 取迁移种子 admin（person-admin/role-admin）；无种子则打印原因后跳过（同文件既有口径） |
| message `test_send_message_row_and_conversation_touch_are_atomic` | `send_message` 前置 `is_conversation_member`，自建会话缺成员行 ⇒ 401 | 补 `x_message_conversation_member` 行 + 请求挂发送人会话 |
| pp `u2_record_create_is_content_deduped` | record create 收「会话 + work 发起人」双闸 | 先建 `x_work`（creator）再以发起人提交；补 200 断言 |
| pp `u2_taskcompleted_next_identity_updates_task` | 同上（任务参与人身份） | 改以 `p@P` 身份发；补 200 断言 |
| personal_extend `face_writes_require_owner` | **产品缺陷**：`crud_save` 全列写入、缺键写 `""`；`face_save` 的 spec 含 `person_id` ⇒ 局部保存把归属抹空 ⇒ 属主自己 delete/save 全 403 | 新增 `face_update_spec()`（只含业务列，不含归属列），`face_save` 改走它；测试加"save 后归属仍在"断言，并把种子改权威写法（`ON CONFLICT … DO UPDATE SET person_id = EXCLUDED.person_id`）防残留行假红 |

另修 `quoted_key_guard`（r419 门）：`ROWS` 夹具改裸列名、`seed_rows` 经 `dialect().quote_ident(col)` 引用——
`xcreatorPerson` 这类 camelCase 列在 PostgreSQL 下不带引号会被折成小写而找不到列。

### 2.4 另一处仓库级教训（clippy 之外）

`security_gate_tests.rs` 的既有失败与审计加门是同一批次的产物：**加门时应连带跑全量 lib 测试**，
否则"门 + 夹具"的失配会在下次全量跑时成批爆出（本轮 6 个 target 同源）。

### 2.5 Python：8 条红（AI platform CI）

| 用例 | 定性 | 修法 |
|------|------|------|
| `test_rmw_serialization_l273` 注入臂 | **判据靠调度**的 flaky（本机 5 跑 3 红 2 绿；全量跑会撞上绿的那几跑） | 读边加一格 50 ms 窗：读回真内容后再返回，快照与"写回整份"的窗口撑到毫秒级 ⇒ 20 支快照必然重叠，判据从"会不会坏"改成"必然坏"（两边判据仍不许为零，不削弱牙齿） |
| `test_log_block_bullet_ownership_l264` | 取全谱窗口后 40 枚里全是 dependabot 的 merge/chore，轮号 0 枚 ⇒ 常红 | 改用本仓对主线历史的既有口径 `--first-parent`（窗口仍 40），量的是主线自己走过的轮数 |
| `test_name_path_containment_l259` faiss 臂 | 漏建 `storage_dir`（`db/` 不存在时 `stat` 直接 ENOENT），与闸门无关 | 补建 `(tmp_path / "db")` |
| `test_xlsx_zipbomb_l288` ×5 | 产品侧缺 pandas/openpyxl 时降级成 `DataLoadError`，三组判据都到不了 ⇒ 恒红 | `requirements-dev.txt` 增 `pandas` / `openpyxl`（量的是真实读边，依赖属于测试前置，不是产品缺陷） |

### 2.6 oa4rust-web：biome 与 mp-weixin（`b73185a78`）

- biome 15 条 = 6 条真红 + 8 条 **CRLF 假红**（`git ls-files --eol` 证 index=lf、worktree=crlf；写回 LF 后无内容差异，
  此类"假红"别改代码去迎合）。真红：`BBSForum.vue` 的 URL 归一正则 `[\u0000-\u0020]` 触发 `noControlCharactersInRegex`，
  改写 `/[\p{Cc}\u0020]/gu`（覆盖同一批控制码点；`java\tscript:` 混淆仍被拦）+ 5 个文件的 organizeImports/格式自动修。
- `build:mp-weixin` 崩溃根因：`@dcloudio/vite-plugin-uni` 的 peer 是精确 pin `vite: "5.2.8"`，工作区安全 override
  把 `vite@<=6.4.2` 顶到 `6.4.3` ⇒ **真实可用版是 6.4.3**；dependabot 把 mobile 的 vite 抬到 8.3.4（vite 8 = Rolldown）
  后与 dcloudio 插件不兼容（`Cannot read properties of undefined (reading 'context')`）。修法：回钉 `"vite": "6.4.3"`
  + 同步 `pnpm-lock.yaml` + `mobile/README.md` 记口径。桌面端不受该 pin 约束（仍走 vite 8）。

## 3. 验证矩阵（本地 CI 等价）

- **Rust**（`rustc 1.98.1` 容器 + 本机 PostgreSQL）：`cargo fmt --all -- --check` EXIT=0；
  `cargo clippy --workspace --all-targets --keep-going -- -D warnings` EXIT=0（0 error）；
  `cargo test --workspace --lib --no-fail-fast` 0 failed；6 个定向用例（bbs/cms/message/pp_record/pp_task/personal_extend）全绿；
  守卫门（quoted_key_guard / designer_route_match / migrate-only / u2_probe / integration_runner）全绿。
- **Python**（`python:3.11` 容器 = CI 口径：`pip install -r requirements-dev.txt`，**不含 faiss**；
  `working-directory=augmentor`）：`python -m compileall -q augmentor api cli.py` EXIT=0；
  `pytest`（含 `--cov-fail-under=80`）→ **0 failed / 10124 passed / 14 skipped，覆盖率 100%**（EXIT=0）。
  同口径修复前基线：8 failed / 10025 passed / 86 skipped（含 xlsx 5 支与 3 支守卫红）。
- **web**（本机 pnpm 11.25.0 / node 22）：`test` 1028 passed；`test:mobile` 121 passed；`typecheck`；
  `biome check --error-on-warnings .` 0 报项（267 文件）；`build`(desktop)；`build:h5`；`build:mp-weixin`；`pnpm install --frozen-lockfile` EXIT=0。
- **本机未复跑的 CI job 及其理由（如实标注）**：
  - `behavior-compare` / `behavior-gate`（要拉 Java o2server 容器 ~10 min + 播种）：本轮后端**唯一的**产品改动是
    `personal_extend::face_save`，而 `tests/behavior_comparison` 覆盖的模块只有 bbs / cms / meeting / org / processplatform
    （`seeds/seed_fixtures.sql` 里的 `x_*` 表清单）⇒ 该 job 的行为面不受影响；且该 job 自带 FAIL 基线回归闸，
    只对"新增失败"报警。
  - `openapi-guard`（`python scripts/check_openapi_security.py`）：只检查路由/OpenAPI 安全方案声明，本轮未改路由与安全方案。

## 4. 可复用教训

1. **clippy 必须 `--keep-going`**：否则只看到第一个失败 crate，"修完还有第二波"。
2. **加鉴权门之后要立刻跑 workspace 全量测试**：门生效会把既有夹具成批打成红（本轮 6 个 target 同源），
   拖到下一轮就变成"既有红"。
3. **依赖驱动的元测试有"隐藏依赖"**：xlsx 守卫量真实读边，需要 pandas/openpyxl，而 CI 只装 requirements-dev.txt
   ⇒ 依赖不进清单这些用例恒红（装齐后 5 条直接转绿）。
4. **判据靠调度的注入臂要收紧成必现**：flaky 的守卫在快机上"多数绿"，会周期性红 CI；用"读边窗口"把竞态钉成必现，
   比放宽判据更干净。
5. **CRLF 假红别改代码**：`git ls-files --eol` 一锤定音（index=lf/worktree=crlf ⇒ 无内容差异）。
6. **工具链 pin 的"真实可用版"要看 override 后的解析**：dcloudio 的 peer 写 5.2.8，工作区 override 顶到 6.4.3，
   dependabot 抬到 8.x 就崩；锁步工具链的包升级要先看 peer+override 的组合而不是目标版本号。
7. **`crud_save` 全列写入是系统性陷阱**：spec 含归属列就会在局部保存时抹归属（本轮 personal_extend 实测）。
   新增 `*_save` 时用窄白名单（只列业务列），归属只由 create 定。
8. **容器构建产物的磁盘账要记**：本轮 Rust target 缓存卷涨到 85.7 GB、把宿主 C: 顶满，连累 Docker 引擎
   （vhdx 无法扩容 ⇒ guest I/O error ⇒ dockerd SIGBUS）。删掉这些卷后 guest 的数据盘用量从 ~107 GB 回落到 ~8.5 GB，
   但**宿主上的 vhdx 文件仍是 129 GB**——要压缩（`Optimize-VHD -Mode Full` 或 diskpart `compact vdisk`，需管理员）
   才会把空间真正还给 C:。大构建建议放独立磁盘或跑完即 `docker volume rm`。
9. **pytest 的等价口径包含 working-directory**：CI 在 `augmentor/` 下跑 `pytest`（`testpaths=tests`）。
   若从仓库根跑，会多收 `llm-lab/` 等目录的用例（本轮多收 130 支），并让"以 `tests/` 为基准算相对键"
   的元测试（l98 盲面守卫）在越界项上假红——复刻 CI 时要把工作目录一起复刻。

## 5. 遗留

1. ~~**faiss 存在时的 4 条 latent 红**（CI 不装 faiss，故不是 CI 红）~~ **已修（追加轮 §6.1）**：
   产品侧在 `top_k<=0` 早返回、`load`/`delete` 重建索引前判维度；测试侧用 `sys.modules["faiss"]=None`
   哨兵把"没装"变成可控输入。原判据（"别把语义交给 faiss 的 assert"）成立。
2. ~~`shared::crud_save` 的同类审计~~ **已修（追加轮 §6.2）**：助手层改"只写载荷里出现的列"，
   5 处含归属列的 save 补窄 `*_update_spec()`；21 个调用点的逐点判定见
   [crud_save PATCH 语义](../best-practices/crud-save-patch-semantics.md)。
3. `main` 的 CI 归因文档（分诊文档 §4）已随之更新为"已修"状态。

## 6. 追加轮（同日）：faiss latent 红与 crud_save 系统性收口

§5 的两条遗留当日收口。口径不变：判据只用本地 CI 等价命令；能改产品代码就改（本轮两处都定性为**产品缺陷**）。

### 6.1 faiss 的 4 条 latent 红（`python:3.11` 容器 + `faiss-cpu`）

| 用例 | 定性 | 修法 |
|---|---|---|
| `test_zero_top_k_returns_no_rows`、`test_top_k_zero_is_the_echo_only_mode_the_ui_allows` | 产品缺陷（两条后端行为不一致） | `FAISSDB.search`：`k <= 0` 早返回 `[]`，不再把语义交给 faiss 的 `assert k > 0` |
| `test_load_with_wrong_dimension` | 产品缺陷 | `load`/`delete` 重建索引前判 `vectors.shape[1] == dimension`；不匹配则跳过入索引 + 告警（`load()` 仍返回 True，数据留在 `_vectors`） |
| `test_faiss_not_installed_raises` | 测试口径（隐含"环境没装 faiss"） | `monkeypatch.setitem(sys.modules, "faiss", None)` 把"缺失"变成可控输入，两种环境跑同一条分支 |

口径与可复用教训见 [可选依赖导致的 latent 红](../testing/optional-dependency-latent-reds.md)。

### 6.2 crud_save 系统性收口

- **助手层（根因）**：`shared::crud::crud_save` 改为**只写载荷里出现的列**——缺键保持原值、显式 `""`/null 才清空、
  一个可写列都没有时不发空 `SET`（改查 `SELECT 1 … WHERE id=$1`，保 not-found 语义与 `*_owned` 门禁链）。
  一处改动覆盖全部 21 个 `crud_save` 调用点。
- **归属层（防显式改写）**：5 处 spec 含归属列者新增 `*_update_spec()` 并让 save 改走它——
  `program_center::collect_save`（`person_id`）、`portal::dict_save`/`widget_save` 与
  `query::importer_save`/`stat_save`（`creator`）；`collect_save` 原"剥离 personId"的补丁随之删除。
- **测试**：shared 三条实库用例（部分载荷保留未提交列 / 空载荷只答存在性 / 显式空值清空）+
  program_center 的 collect save 往返（部分保存后归属仍在、客户端硬塞 `personId` 被忽略、非属主 `saved=false`）+
  portal/query 的窄 spec 形状测试（钉住"save spec 不含 creator"）。
- 逐点审计表、判定标准（归属列 vs 关联列）与遗留见
  [crud_save PATCH 语义](../best-practices/crud-save-patch-semantics.md)。

### 6.3 本轮验证

判据同 §3：本地 CI 等价命令，逐条给退出码/计数。

**Rust**（`rust:1.98.1-bookworm` 容器 + PostgreSQL 容器，`_verify.sh` 九步）

| 步骤 | 命令 | 结果 |
|---|---|---|
| 1 fmt | `cargo fmt --all -- --check` | ✅ 0（首跑报 5 处差异，**全在本轮新加的测试代码里**，已格式化后复查 0） |
| 2 clippy | `cargo clippy --workspace --all-targets --keep-going -- -D warnings` | ✅ 0 error / 0 warning |
| 3 迁移 | `cargo run --bin oa4rust -- --migrate-only` | ✅ 0 |
| 4 单测 | `cargo test --workspace --lib --no-fail-fast` | ✅ 全绿（含 shared 3 条新实库用例） |
| 5 契约 | `cargo test -p parity --lib` | ✅ 4143 passed / 0 failed |
| 6 探针 | `cargo test --test u2_probe` | ✅ 2 passed |
| 7 集成 | `cargo test --test integration_runner` | ✅ 1 passed / 6 ignored（需真库，既有口径） |
| 8 路由对账 | `cargo test --test designer_route_match` | ✅ 1 passed |
| 9 引号键守卫 | `cargo test --test quoted_key_guard` | ✅ 1 passed |

- **两条环境坑（不是代码问题，但会让人误判"全绿"，如实记录）**：
  1. 容器里的 rustup 工具链**没装 rustfmt/clippy**：首跑 `FMT_EXIT=1`/`CLIPPY_EXIT=1` 的真身是
     `error: 'cargo-fmt' is not installed for the toolchain` ——**不是"检查通过"**。
     装上组件后这两步才真正执行，并立刻抓到上面那 5 处格式差异。
  2. `utoipa-swagger-ui v7.1.0` 的构建脚本要从 `github.com` 下载 swagger-ui（本网络
     `curl: (35) SSL_ERROR_SYSCALL in connection to github.com:443`）。构建脚本自带
     `SWAGGER_UI_DOWNLOAD_URL` 口子：指向 `target/` 里先前成功下载过的同一枚
     `v5.17.12.zip`（`file://` 协议）即可 —— **只影响资源获取，不影响编译与检查结果**。
- 顺手收口一条**既有 flake**：`query_service_processing::tests::u2_contract::u2_reset_clears_recent_processing_in_one_txn`
  走 HTTP 且 reset 是 admin 档，但它自己不调 `ensure_schema()`，只靠"同包别的用例先跑过"播种角色链。
  实测：删掉 `auth_role`/`auth_person`/`auth_person_role` 后**单跑必 403**（`ExceptionAccessDenied`），
  全量跑则靠线程调度侥幸通过。按同文件 9 个兄弟用例的写法补 `ensure_schema().await`；
  补后同样的删除条件下单跑通过。

**Python**（`python:3.11` + `requirements-dev.txt`，`augmentor/` 工作目录 = AI platform CI 的 backend-test job）

| 口径 | 命令 | 结果 |
|---|---|---|
| 装了 `faiss-cpu 1.15.1` | `python -m compileall -q augmentor api cli.py` + `pytest -q` | ✅ **10124 passed / 14 skipped**，覆盖 99.96%（4 条 latent 红归零） |
| 卸掉 faiss（CI 口径） | 同上 | ✅ **10124 passed / 14 skipped**，覆盖 99.99%（与装 faiss 同数、同跳过集） |

两轮都在**干净克隆**上跑（不是作者工作树），跳过的 14 条全是既有的"需要作者工作树/数据集/前端产物"档，
与本次改动无关。
