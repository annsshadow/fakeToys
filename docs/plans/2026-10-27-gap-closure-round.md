# 第六轮 · 六项缺口收口 · 执行计划

- **登记日**：2026-10-27
- **范围**：augmentor（A184 文档引用普查 / Z-FULL 全量契约 / 文档 FAQ）、oa4rust（L-REST 全量无界 SELECT 普查与棘轮 / OA-LIMIT-OFFSET 分页联动）
- **目标**：把 10-26 第四轮结束时明确留白的六项逐一收口，并且**只收口能拿证据的那部分**：
  1. **L-SKIP-LEDGER**：给「14 处跳过名册」一个诚实的结论，而不是补一份编造的名册；
  2. **A184**：把 `docs/augmentor/**/*.md` 接入文档引用普查，修掉旧路径与失效行号；
  3. **Z-FULL**：把 augmentor/web 的全部响应模型接上 zod 运行时校验；
  4. **L-REST**：对 oa4rust 全量 SQL 字面量做无界 SELECT 普查，落基线棘轮，只改高置信列表端点；
  5. **OA-LIMIT-OFFSET**：把已有服务端分页接回前端，并修掉「搜索结果永远第 1 页」「前端只数数组长度、不读服务端 count」；
  6. **BOM-FAQ**：FAQ 只加部署索引，不复制部署内容。
- **方法**：沿用前几轮的「只读勘察 → 判据 → 最小改动 → 红→绿验证 → 台账」范式；每条结论都能指回文件、测试或命令输出。
- **不写什么**：不写 secrets、环境真值、真实服务器地址；不在文档里回显本机绝对路径。

## 0. 与既有计划的关系

| 既有计划 / 台账 | 本文档承接 |
|---|---|
| `docs/plans/2026-10-26-r4-verification-zod-faq-round.md` §7 | 该节把 L-SKIP-LEDGER、L-REST、Z-FULL、OA-LIMIT-OFFSET、BOM-FAQ 列为「本轮未完成 / 转产品决策」；本文档逐项收口，**不修改原表的完成态**，只在新文档里记录后续证据 |
| `docs/plans/2026-10-26-r4-verification-zod-faq-round.md` §8 | L 批次的 8 处命中项与 14 处跳过项是本文档 L-REST 的起点；L-REST 不复用「14 处」这个名册，改用全量普查 |
| `augmentor/OPTIMIZATION_PLAN.md` §3.1 / §3.3 | Z-FULL 与 BOM-FAQ 的上游待办 |
| `oa4rust/backend/docs/audits/found-issues.md` | L-REST 的扫描器与 CI 棘轮沿用 FI-002 的「静态普查 + 基线」方法，但对象从「未知列」换成「无界 SELECT」 |

## 1. 基线（本轮开始时的可复核状态）

### 1.1 仓库与 CI

- 本轮起点提交：`1a18df39a`（`test(augmentor): L264 探针按块号取，修 5 支注入臂假红`）。
- 最近一轮 CI 已在 `1a18df39a` 上全绿；本轮每批提交后仍以 `gh run list` 的实际结论为准，不以「本地看起来过了」代替。
- 本机没有 Rust 编译器：Rust 改动的验证边界是 `cargo fmt --check` + 静态扫描 + CI 编译/测试；这一点在 §10 的每条 Rust 证据里都要写明。

### 1.2 本轮开始时的未提交改动

| 文件 | 状态 | 归属 |
|---|---|---|
| `augmentor/tests/unit/test_comment_refs_a184.py` | 已改，待提交 | A184 |
| `augmentor/web/src/schemas/full-schemas.ts` | 新增，待提交 | Z-FULL |
| `augmentor/web/src/schemas/main-schemas.conformance.test.ts` | 已改，待提交 | Z-FULL |
| `augmentor/web/src/services/api.ts` | 已改，待提交 | Z-FULL |
| `augmentor/web/src/services/api.test.ts` | 已改，待提交 | Z-FULL |
| `augmentor/web/src/services/api.optional-defaults.test.ts` | 已改，待提交 | Z-FULL |
| `augmentor/web/src/test/valid-response.ts` | 新增，待提交 | Z-FULL |
| `augmentor/web/src/test/valid-response.test.ts` | 新增，待提交 | Z-FULL |
| `docs/augmentor/API.md` | 已改，待提交 | A184 |
| `docs/augmentor/ARCHITECTURE.md` | 已改，待提交 | A184 |
| `docs/augmentor/FAQ.md` | 已改，待提交 | BOM-FAQ |

> 这张表是「写计划时」的快照。提交后以 `git log` 为准；不要在计划里把未提交的东西写成已完成。

## 2. 批次总览

| 批次 | 主题 | 风险 | 提交策略 | 完成判据 |
|---|---|---|---|---|
| S | L-SKIP-LEDGER 结论 | 低（文档） | 并入本轮计划文档 | 明确「不可恢复」，不出现编造名册 |
| A | A184 文档引用普查扩展 | 低-中（扫描面变化会翻动读数） | 1 提交 | `docs/augmentor/**/*.md` 全部入场；dead 四档为 0；棘轮重钉 |
| Z | augmentor/web 全量 zod | 中（78 个函数、85 个模型） | 1 提交 | 78/78 接线；415+ 前端测试全绿；conformance 389 字段 |
| L | oa4rust 无界 SELECT 全量普查 | 中（扫描器口径 + 基线） | 1 提交 | `limit_scan.py` 可重复；基线棘轮进 CI；7 处高置信列表加 LIMIT |
| O | 分页联动 | 中（前后端字段口径） | 1 提交 | 后端分页参数被前端真实使用；count 来自服务端；搜索页码不再钉死 1 |
| B | BOM-FAQ 索引 | 低（文档） | 并入 A 或单独提交 | FAQ 指向 `docs/augmentor/DEPLOYMENT.md`，不复制环境值 |

每批完成后跑该批门禁 → commit → push；推送后在 §10 记录 commit 与 CI 结论。

---

## 3. S 批次 — L-SKIP-LEDGER：先承认「找不回来」

### 3.1 事实

10-26 第四轮的 L 批次改了 8 处热点、16 条 LIMIT；当时声称「跳过 14 处」，并把跳过项的分类计数写进 commit message。但那 14 条的逐条名册（端点名 + 理由 + 前端消费方式）**没有落盘**，且 commit message 里的分类相加是 **16 ≠ 14**——类别可能重叠，也可能有 2 条被重复计数。

本轮检查了仓库内所有可能承载该名册的位置：计划文档、`docs/tech-debt.md`、审计工件、提交说明。结论是：

> **原名册不可恢复。** 任何一份「14 条逐条清单」都只能是事后重编，不是当时的记录。

### 3.2 处理

- 不补写、不冒充原记录；在本文档里把这条缺口标为「历史矛盾，已承认，不追补」。
- 用 L-REST 的**全量普查 + 基线**取代「14 条名册」：不试图还原当时为什么跳过哪一条，而是对当前全部无界 SELECT 给出可重复的读数、可复核的分类和可棘轮的下界。
- 这样做的代价写在明处：L-REST 的基线不能证明「当时那 14 条都被正确处理」，只能证明「从本轮起，无界 SELECT 不再增加」。两件事不是一回事，文档不合并表述。

### 3.3 验收

- 本文档出现「不可恢复」与「16 ≠ 14」两句话；
- L-REST 的基线文件、扫描器、CI job 三件套齐备；
- 不在任何提交信息或文档里出现伪造的 14 条逐条名册。

---

## 4. A 批次 — A184 接入 `docs/augmentor/**/*.md`

### 4.1 问题

A184 原先只扫 `augmentor/` 产品树与账本，`docs/augmentor/` 下的 14 份 Markdown（含 `system/` 子目录）是「文档中的文档」，从未被引用普查覆盖。后果是：旧仓库路径、搬走后的行号、指向不存在文件的锚点都不会让门禁变红。

### 4.2 改法

1. `corpus()` 增加 `_repo_docs_walks()`：把 `docs/augmentor/**/*.md` 逐份读入，使用 `docs/augmentor/<相对路径>` 作为仓库坐标。
2. `lines_of()` 同时接受 `augmentor/` 根与仓库根两种坐标，避免 `system/` 子目录被拼错后静默丢文件。
3. 修复 15 条失效引用：14 条是旧 `docs/<FILE>.md` 路径 → `docs/augmentor/<FILE>.md`；1 条是 `logging_setup.py:108` → `logging_setup.py:114`。
4. `scratch` / `scratch_missing` 在作者树与干净克隆之间不可移植，改为只钉**两档之和** `SCRATCH_PAIR`，不钉单格。
5. `dist` 加入 `IDX_SKIP`：前端构建产物是 gitignored 的，跑没跑过 `npm run build` 不应改变普查读数；CI 克隆里也没有它。
6. `FLOOR["anchor_claims"] = 40`：防止「一个锚点都没解析出来」被当成绿。

### 4.3 验收

- `python -m pytest tests/unit/test_comment_refs_a184.py -q -o addopts=""` 全绿（允许 1 条 authoring-tree skip）。
- 当前读数（排除 `dist` 后）：`files_seen 627`、`refs_total 6967`、`live 4985`、`symbol_live 952`、`ambiguous 497`、`scratch_missing 1484`、`anchor_claims 40`、`runtime_ns 1`；`dead_path / dead_line / dead_anchor / symbol_dead` 全 0。
- 注入自证：临时在 `docs/augmentor/FAQ.md` 写入 `docs/augmentor/GHOST_FILE_NOT_REAL.md:12`，普查读数出现 `dead_path 1` 并点名文件/行号；还原后 `git status` 无 FAQ 残留。
- `docs/augmentor` 在场测试：14 份 Markdown 被 `corpus()` 读到；缺少任一份时测试红，而不是静默缩小扫描面。

> 注：`6967` 是当前工作树读数；A184 的文件数、引用数会随文档改动变化，**dead 四档与 `SCRATCH_PAIR` 才是判据**。不要把 `6967` 当成业务不变量写进代码。

---

## 5. Z 批次 — Z-FULL：78 个函数、85 个模型

### 5.1 问题

Z 批次只给主链路 14 个接口接了 zod。剩余 64 个函数的响应仍是「TS 声明说是什么就是什么」，后端改了字段前端不会红。

### 5.2 改法

1. 新增 `augmentor/web/src/schemas/full-schemas.ts`：以 pydantic `response_model` 为唯一 ground truth，生成 70 个 schema；与 `main-schemas.ts` 的 15 个主链路 schema 合计覆盖 85 个模型。
2. `services/api.ts` 新增 `checkedData<T>()` / `checkedResponse<T>()`：校验通过后返回**原始** `response.data`（不返回 zod 的 parse 克隆，避免破坏引用身份与既有测试）。
3. 接线全部 64 个剩余函数，达到 **78/78**；错误消息统一写成「接口 X 的响应不符合契约：字段「Y」（issue）。请检查前后端版本是否一致」。
4. `main-schemas.conformance.test.ts` 扩展为跨语言守卫：
   - 解析 85 个模型（15 主链路 + 70 全量）；
   - 解析 Python 继承并合并基类字段；
   - 解析 `AuditReport` dataclass；
   - 校验 `文件:行号 → 类名` 映射的现量；
   - 校验路由 `response_model` 全覆盖；
   - 字段总数钉为 **389**。
5. 测试 fixture 不手抄 payload：新增 `src/test/valid-response.ts`，用 `z.toJSONSchema()` 从 schema 现算最小合法响应，按「方法 + 路径」查表；`valid-response.test.ts` 自证每个样本都能被自己的 schema 吃下。

### 5.3 验收

- `npx vitest run`：**24 files / 415 tests** 全绿；
- `npm run typecheck`：0 错；`npm run lint`：0 错；`npm run build`：成功；
- conformance：**93 tests** 全绿，字段总数 389；
- `api.test.ts` 79 tests、`api.optional-defaults.test.ts` 14 tests 全绿；
- 反向自证：`valid-response.ts` 对未登记端点**抛错**，不返回 `{}`；`getProgress` 的 `{}` 用例仍然红（故意保留）。

---

## 6. L 批次 — L-REST：全量无界 SELECT 普查与棘轮

### 6.1 口径

扫描器 `oa4rust/backend/scripts/limit_scan.py` 复用 `fi002_column_scan.py` 的 `SRC_ROOTS` 与 `rust_literals()`，只扫产品码；`#[cfg(test)]` 模块、`tests/`、`*_tests.rs` 不入账（测试夹具不是产品 SQL）。每条 SQL 字面量按**顶层形状**分桶：

| 桶 | 判据 | 是否治理对象 |
|---|---|---|
| `bounded` | 有 `LIMIT <n|$n|?|{…}>` 或 `FETCH FIRST n ROWS` | 否 |
| `aggregate` | 顶层投影只有 `COUNT/EXISTS/SUM/AVG/MIN/MAX`（含 `COALESCE(MAX(...))`） | 否 |
| `single_row` | WHERE 对身份键 `id` / `xid` / `unique_id` 做等值绑定，或同时绑定 ≥2 个 `*_id`（复合身份） | 否 |
| `parent_key` | WHERE 只对**一个**非身份 `*_id`（外键/父键）做等值绑定 | **是**（逐端点决策） |
| `key_set` | `WHERE id IN (...)` / `= ANY(...)` | 否 |
| `scalar` | 无 `FROM` 的标量查询 | 否 |
| `subquery` | `INSERT/UPDATE/DELETE ... SELECT` | 否 |
| `unbounded` | 其余 | **是** |

**已知边界（写进脚本 docstring，不藏着）**：

- `single_row` 只认**身份键约定**：`id` / `xid` / `unique_id`，或同时绑定 ≥2 个 `*_id`（复合身份，如 `doc_id + person_id`）。`WHERE work_id = $1` / `person_id = $1` 这类「按父键取子列表」归 `parent_key`，与 `unbounded` 一样进基线棘轮。为什么不按数据库主键/唯一键判：历史表大量**没有主键/唯一约束**（例如 `x_attendance_config` 只有普通索引），schema 无法替静态扫描背书；身份键是应用层的稳定约定，复合身份用「同时绑定 ≥2 个 `*_id`」的形状近似。两者都不是数据库唯一性证明，真正的唯一性仍需数据治理（见 OA-P1-3 备注）。
- `parent_key` 不等于「已批准无界」：它只表示「形状是父键取子列表，需要逐端点决策」。新增一条 `parent_key` 同样会让 CI 红，必须在基线里显式登记理由。
- `format!` 里的动态表名/列名只解析静态部分；无法判定的整条 SQL 会记入 `skipped` 并在输出里明示，不静默丢弃。
- `LIMIT {}` / `LIMIT {n}` 是 Rust 格式化占位符，运行期必然有界，统一归 `bounded`。
- `LIMIT $1` 也归 `bounded`：参数由调用方给，脚本不猜它会不会是 10^9；这是「SQL 形状有上界」与「上界取值是否合理」两件事，后者不在静态扫描能力内。
- 带 `GROUP BY` 的投影（如 `SELECT status, COUNT(*) ... GROUP BY status`）目前不算 `aggregate`，因为投影不只有聚合函数；它若同时带父键等值会被归 `parent_key`，否则归 `unbounded`。这是保守方向：宁可多登记，不静默放行。

### 6.2 基线

- 工件：`oa4rust/backend/docs/audits/artifacts/limit_unbounded.tsv`。
- 键：`file<TAB>bucket<TAB>count`，登记 `unbounded` 与 `parent_key` 两个桶的文件级计数；附首条证据与复核理由。
- 棘轮：
  - 新文件出现 `unbounded` / `parent_key`，或既有文件计数**增加** → 退出 1；
  - 计数**减少** → 打印收敛提示，退出 0（复核后 `--update-baseline` 收紧）；
  - 扫描器自身失效（总字面量 < 2600、文件数 < 120、分桶低于地板）→ 退出 1，防止「什么都没扫到」被当成绿。
- 初始普查（10-27，加 LIMIT 之前、但已把 `LIMIT {…}` 与 `SELECT 1` 探针改判之后）：
  **2745 条 SELECT 字面量 / 138 个文件**，分桶 `aggregate=267 bounded=552 key_set=93 scalar=23
  single_row=1148 subquery=31 unbounded=631`。
- 加 7 处 LIMIT 后：`bounded 552→559`、`unbounded 631→624`（7 条从 unbounded 迁到 bounded），
  基线 **624 条 / 76 个文件**（文件数 = 基线里出现 unbounded 的文件数，不是扫描面 138 个文件）。
- 与 10-27 最早期那版外部普查（unbounded 641）差的 17 条，账要拆开说：**10 条 `LIMIT {…}` 改判
  bounded** + **7 处新增 LIMIT**；`SELECT 1` 探针是 4 条从 unbounded 移出、同时 4 条从 single_row
  移入 scalar，对 unbounded 是净 0（所以 `single_row` 从 1167 落到 1148）。最终一切以
  `limit_unbounded.tsv` 的 624 为准，不以任何口头数字为准。

- **10-28 修订**（本轮）：
  1. 扫描面排除 `*_tests.rs`（此前把 11 个 `security_gate_tests.rs` 的 83 条测试 SQL 计入产品面）：
     `aggregate 267→222`、`scalar 23→22`、`single_row 1148→991`、`unbounded 624→617`、
     `parent_key 0→113`；总字面量 `2745→2662`、文件 `138→127`。
  2. `single_row` 收紧为身份键约定（`id` / `xid` / `unique_id`，或 ≥2 个 `*_id` 复合身份）：
     非身份单列父键（`work_id` / `person_id` / `doc_id` …）改判 `parent_key`，与 `unbounded` 一起
     进基线棘轮；`single_row 991`、`parent_key 113`、`unbounded 617`。
  3. BBS 两个别名端点与同族 15 条列表加 `LIMIT 100`（§6.3 第二轮）：
     `bounded 559→573`、`parent_key 127→113`、`unbounded 617`（不变）。
  4. 基线重生成：`unbounded 617 条 / 82 个文件` + `parent_key 113 条 / 43 个文件`，合计 730 条，
     `python scripts/limit_scan.py` 基线态退出 0；反向自证（临时探针）在 `unbounded` 与 `parent_key`
     两个桶上都会红。

### 6.3 两轮改动

**第一轮（10-27）**：只改 7 处高置信列表端点（历史台账保留，不重写）：
`list_to_enable` / `list_forums` / `list_control_sections` / `list_hotpics` /
`list_jpushs` / `list_schedule` / `list_components`，均加 `LIMIT 100`。

**第二轮（10-28，本轮）**：给 BBS 别名端点与 15 条同族 `parent_key` 列表加 `LIMIT 100`：

| # | 位置 | 查询 | 改法 | 理由 |
|---|---|---|---|---|
| 1 | `bbs/src/section.rs::view_forum` | 版块下子版块列表 | `LIMIT 100` | 列表语义、单页展示 |
| 2 | `bbs/src/subject.rs::top` | 版块置顶主题列表 | `LIMIT 100` | 置顶数量增长型，前端单页 |
| 3 | `bbs_assemble_control::list_topics_by_forum` | 版块主题列表 | `LIMIT 100` | 列表语义 |
| 4 | `bbs_assemble_control::forum_view_all` | 全量版块（别名端点，与 `list_forums` 同形） | `LIMIT 100` | 与第一轮 #2 同族；`x_bbs_forum` 有主键，100 内截断 |
| 5 | `bbs_assemble_control::user_forum_list` | 全量版块（别名端点） | `LIMIT 100` | 同 #4 |
| 6 | `bbs_assemble_control::reply_list_sub_id` | 主题回复列表 | `LIMIT 100` | 回复列表语义 |
| 7 | `bbs_assemble_control::subject_top_section` | 版块置顶主题（别名端点，与 #2 同族） | `LIMIT 100` | 同 #2 |
| 8 | `bbs_assemble_control::section_viewforum_forumId` | 版块下子版块（别名端点，与 #1 同族） | `LIMIT 100` | 同 #1 |
| 9 | `bbs_assemble_control::subjectattach_list` | 主题附件列表 | `LIMIT 100` | 附件列表语义 |
| 10-11 | `bbs_assemble_control/u2.rs` 两个附件列表别名 | 主题附件 | `LIMIT 100` | 与 #9 同族 |
| 12 | `message_assemble_communicate::im_conversation_id_group` | 会话成员列表 | `LIMIT 100` | 成员列表语义 |
| 13-14 | `processplatform_service_processing` 两条 review 列表 | 工作项 review 记录 | `LIMIT 100` | 记录列表语义 |
| 15-16 | `processplatform_service_processing` 两条 snap 列表 | 工作项快照列表 | `LIMIT 100` | 快照列表语义 |

**第三轮（10-28 下午，P4 批量收口）**：三组并行勘察（覆盖 parent_key 全部 60 条 P4 明细 + P2 的 11 条
attendance 统计）后，按「列表语义 + ORDER BY 稳定截断」给 **38 处加 LIMIT**（37 处 `LIMIT 100` +
1 处单行 get-one `LIMIT 1`），`parent_key 113 → 75`、`bounded 573 → 611`：

| 文件 | 处数 | 端点 |
|---|---|---|
| `attendance_assemble_control/lib.rs` | 10 | 排班配置 2179、单位日/月统计 3815/3861/3911/4055/4106、本人状态聚合 6333、单人单日打卡 7634、打卡预检 8202（`LIMIT 100`）；排班配置 get-one 7981（`query_opt`，`LIMIT 1`） |
| `file_assemble_control/lib.rs` | 1 | 引用附件列表 2892 |
| `meeting_assemble_control/lib.rs` | 2 | 会议控制项 44、会议室当日会议 1738 |
| `mind_assemble_control/u2.rs` | 1 | 脑图共享记录 241 |
| `program_center/lib.rs` | 1 | 个人收藏 186 |
| `personal_extend/personal.rs` | 1 | 本人人脸条目 260 |
| `organization_assemble_control/lib.rs` | 5 | 群组 819、单位/身份职务 3081/3231、单位属性 3428、人员属性 4099 |
| `organization_assemble_control/u2_org.rs` | 2 | 组织直接下级 260/276 |
| `processplatform_service_processing/lib.rs` | 2 | 工作/已完成工作附件 2948/2970 |
| `cms_assemble_control/lib.rs` | 6 | 文档附件列表 4040/4112/4269/4341（同 SQL 四克隆）、表单列表 5409、output 列表 5773 |
| `portal_assemble_designer/lib.rs` | 4 | 字典 941、页面 1797、脚本 2547、组件 2932 |
| `portal_assemble_surface/lib.rs` | 3 | 页面 1370、脚本 2098、组件 2315 |

**分类冲突的显式裁决**：attendance 3815/3861/3911/4055/4106/6333/7634 在 §6.5 初版被粗分为 P2
「汇总留基线」；本轮端点级复核证实它们是**有界小集合**（月内日统计 ≤31 行、状态聚合 = 状态种类数、
单人单日打卡个位数），`LIMIT 100` 在真实数据形态下不截断任何行，按「证据优先」改判加 LIMIT。
P2 其余（3630/3676/3957 行数=单位人数可超 100、7691 导出链路）维持原分类留基线。

**不改什么（第三轮新增留基线理由，P4 剩余 29 条 + P2 剩余 12 条）**：

- 索引/同步 feed（截断=静默丢数据）：`ai/index.rs:42`、`ai_assemble_control:1759`；
- 单行期望（`query_opt`，多行语义错误应显式失败）：`ai_assemble_control:1802`、`pp_designer:2449`；
- 权限/判重检查集合（非展示列表，截断破坏正确性）：`ai_assemble_control:2022,2159`（会话属主检查）、`pp u2:878`（附件复制判重）；
- 全量组装：`pp_designer:2705`（流程画布元素）、`cms:8514`（EAV 行折叠为单文档对象）；
- 死 handler（无路由注册）：`cms:5508`；
- 版本历史累积型（截断属产品决策）：`mind:196`、`mind u2:272`、`portal_designer:1898,2664`；
- 大数据量列表（需真分页而非裸 100）：`attendance:1501,1663,8025`、`calendar:605`、`file:276`、`cms:4184`（batch_download）；
- 中置信展示列表（大概率小、未验证真实数据量，留待有库轮次）：`correlation:119`、`meeting:508`、
  `query_surface:1420`、`endpoints_person2:362`、`endpoints_unit2:482`、`pp:1688,2475,2694,2716,2738`。

别名端点与「同族」的判断依据：同一张表、同一谓词形状、同一列表语义（§6.5 附录逐条核对过）；
不是「看起来像」就顺手改。`forum_view_all` / `user_forum_list` 此前 §6.3 明确说「本轮不改、留基线」，
本轮按端点语义复核后决定：它们就是版块列表（`x_bbs_forum` 有主键、`ORDER BY sort` 稳定序），
100 截断不改变正常规模下的语义，故补上。

**不改什么（必须留在基线里）**：

- 导出 / 备份 / 迁移 / 全量遍历 / 聚合统计：加 LIMIT 会改变语义，不是「治理」而是「改需求」。
- 死路由、配置静态表、helper、单行查询：不是列表，或已被上游收窄。
- `parent_key` 里剩下的 113 条（§6.5 决策表）：本轮只改 BBS 别名（9 处）+ 消息会话成员（1 处）+ 流程 review/snap（4 处）共 14 条，
  其余按决策表留在基线，下一轮按「分页联动」批量收口。

### 6.4 验收

- `python scripts/limit_scan.py` 在基线态退出 0；`--update-baseline` 生成的基线可复核；
- 反向自证：临时新增一条无 LIMIT 列表 SELECT，扫描器退出 1 并点名文件/行号；删除后恢复 0；
  对 `parent_key` 桶同样成立（探针用 `WHERE person_id = $1` 形状）；
- CI 的 `limit-scan` job 接在 `fi002-column-scan` 同层；job 失败必须红，不允许 `continue-on-error`；
- `cargo fmt --check` 覆盖本轮改动的 6 个 crate；`cargo check` 覆盖 `bbs` / `bbs_assemble_control` /
  `message_assemble_communicate` / `processplatform_service_processing` 与 `parent` 包；
- 改动后的 SQL 在真库可执行（本机 PostgreSQL 容器内 `psql` 直跑；空表 0 行，非空表截断正确）。

### 6.5 `parent_key` 决策表（10-28 初版 113 条；第三轮后 75 条）

按端点语义分四类。第三轮（§6.3）迁出 38 条后现量 `parent_key=75`（P1 25 / P2 12 / P3 9 / P4 29）；
初版分类数字保留作历史，现量以 `limit_unbounded.tsv` 为准：

| 类 | 语义 | 决策 | 初版条数 | 现量 |
|---|---|---|---|---|
| P1 权限/身份全量 | 角色名/身份名/权限矩阵/组织 scope，调用方要**全量**做鉴权，分页会改变「有没有权限」的语义 | **留基线，不加 LIMIT**（评估按 scope 收窄而非截断） | 25 | 25 |
| P2 报表/统计/导入/BAM | 月度统计、GROUP BY 汇总、导入结果、BAM 状态看板，语义是「汇总」 | **留基线，不加 LIMIT**（截断即改需求）；第三轮对其中 7 条经端点级复核改判「有界小集合」加 LIMIT（见 §6.3 裁决） | 19 | 12 |
| P3 单行唯一性期望 | 谓词是复合业务键（`doc_id + field_name`、`portal_id + name` 等），正常数据下期望 ≤1 行；调用方多走 `query_opt` | **留基线**；若改为 `query` 多行消费需补 LIMIT | 9 | 9 |
| P4 待分页列表 | 父键下的业务列表（附件、版本、日程、收藏等） | 第三轮迁出 31 条（§6.3 表）；**剩余 29 条留基线**，分类理由见 §6.3「不改什么」 | 60 | 29 |

P1 明细（25）：`auth`（lib.rs:331,348）、`auth/switch_user.rs:77`、`auth/welink.rs:108`、
`bbs_assemble_control:1298,1866`、`bbs_assemble_control/u2.rs:1973`（角色绑定）、
`cms:1489,2034`（权限矩阵）、`organization:3622`（组角色）、
`organization:4277,4543,4592,4800,4852,4907`（身份/组 scope 6 条）、
`express/endpoints.rs:233,241,250` + `endpoints_org.rs:287` + `endpoints_person2.rs:270,285,323`
（人员权限投影 6 条）、`organization_assemble_personal:79`、`query_core_express:121`。

P2 明细（19）：`ai/index.rs:42`（AI 索引作业）、`attendance:3630,3676,3815,3861,3911,3957,4055,4106`
（月度统计 8 条）、`attendance:6333,7691`（GROUP BY 状态汇总 2 条）、`attendance:7634`（记录检索）、
`organization:4196`（导入结果）、`processplatform_assemble_bam:384,484,1702,1802,1910,2011`
（BAM 状态看板 6 条）。

P3 明细（9）：`cms:2511,3120`（doc+field 取值，`query_opt` 期望单行）、`cms:6327,6386`
（app+unique_name 脚本定位）、`message:3272`（会话扩展定位）、`portal_designer:3398`
（widget name+portal 定位）、`portal_surface:2826`（script name+portal 定位）、
`org u2_org:1172`（duty name 定位）、`hotpic:105`（application+info_id 图片定位）。

P4 明细（60）：`ai:1759,1802,2022,2159`、`attendance:1501,1663,2179,7981,8025,8202`、
`calendar:605`、`cms:4040,4112,4184,4269,4341,5409,5508,5773,8514`、`correlation:119`、
`file:276,2892`、`meeting:508`、`meeting_control:44,1738`、`mind:196`、`mind u2:241,272`、
`org:819,3081,3231,3428,4099`、`org u2_org:260,276`、
`endpoints_person2:362`、`endpoints_unit2:482`、`personal_extend:260`、
`portal_designer:941,1797,1898,2547,2664,2932`、`portal_surface:1370,2098,2315`、
`pp_designer:2449,2705`、`pp_processing:1688,2475,2694,2716,2738,2948,2970`、
`pp u2:878`、`program_center:186`、`query_surface:1420`——逐条 `file:line`
见 `oa4rust/backend/docs/audits/artifacts/limit_unbounded.tsv` 的 `parent_key` 行
（本轮 113 条全在基线登记，证据列含行号与首条 SQL，随基线文件提交，不在此重复抄录）。

> 判定口径：**只有 P4 且语义是「列表」的才允许直接加 `LIMIT 100`**；P1/P2 是「全量/汇总」语义，
> 加 LIMIT 等于改需求，必须走「按 scope 收窄」或「接分页」，不能截断；P3 的单行期望靠
> `query_opt` 兜底（多行会报错，是显式失败），留基线不静默放行。本轮 P4 实际改了
> BBS 别名 9 条、消息会话成员 1 条、流程 review/snap 各 1 条（共 12 条，见 §6.3 第二轮），
> P4 剩下的 60 条进基线待下一轮批量收口。

---

## 7. O 批次 — OA-LIMIT-OFFSET：把分页真正接回去

### 7.1 后端事实

`crates/organization_assemble_control/src/u2_person.rs::person_list_filter_paging` 已经做了三件正确的事：

- `page = page.max(1)`、`size = size.clamp(1, MAX_PAGE_SIZE)`（`MAX_PAGE_SIZE = 100`）；
- 先 `COUNT(*)` 得 `count`，再 `LIMIT $6 OFFSET $7` 取当前页；
- 返回 `{count, page, size, data}`。

问题在前端：`Personal.vue` 把 `person/list/filter/1/size/20` 的页码写死为 `1`，且只把返回数组当调试输出；`OrgDirectoryApp.vue` 根本没有接这个分页端点；BBS 搜索把 `page/1` 写死；授权委托页只显示数组长度，不读服务端 `count`。

### 7.2 改法

1. **OrgDirectoryApp.vue**：人员页增加服务端分页控件。
   - 状态：`personPage`、`personPageSize = 20`、`personCount`；
   - 请求：`POST /api/organization/assemble/control/person/list/filter/{page}/size/{size}`，body 为 `{}`；
   - 展示：当前页 / 总页数 / 总人数；上一页、下一页按钮按 `count` 与 `size` 计算禁用态；
   - 空结果把 `rows` 置空并让页码栏显示「共 0 人」，不保留上一次的旧表；`lastAction`
     同时记录「第 x 页（0/0 行）」，避免「看起来没反应」。
2. **Personal.vue**：授权委托页签显示服务端 `count`。
   - `/api/person/empower/list/currentperson` 与 `/list/to` 的响应是 `ActionResult`：`data` 是数组，`count` 是全表计数；
   - 页签文案从「数组长度」改为 `count ?? data.length`，截断时不再把 100 说成全部。
3. **BBSForum.vue**：搜索请求的页码从 `page/1` 改为 `page/${page.value}`；`handleSearch()` 已把 `page` 重置为 1，保持不变。
4. **`packages/apis/src/index.ts`**：`personList` 的返回类型从 `PagedResponse<T>`（`total`）改为后端真实形状（`count`），避免下一个调用方继续读错字段。若改动会牵动生成器，先只修类型注释与字段名，不扩大范围。

### 7.3 验收

- OrgDirectoryApp：`page=1` 与 `page=2` 发出不同的 URL；总页数由 `count/size` 算出；`count=0` 时下一页禁用；
- Personal.vue：当 `count > data.length` 时页签显示 `count`，不显示 `data.length`；`count` 缺失时回退数组长度；
- BBSForum：搜索第 2 页时请求 URL 含 `page/2`，不再固定 `page/1`；
- `pnpm --filter @oa4rust/desktop typecheck` 通过；桌面端构建通过（若本机依赖齐全）；无法本机验证的交互在 §10 明确写「未实跑」。
- 不改后端 `person_list_filter_paging` 的语义；它已经是本轮的正确参照。

---

## 8. B 批次 — BOM-FAQ：只加索引，不复制部署内容

### 8.1 改法

- `docs/augmentor/FAQ.md` 顶部把「部署见 README 与 `docker/`」改为指向 `docs/augmentor/DEPLOYMENT.md`（§1 本地、§2 Docker、§3 生产、§5 排障）。
- 文末新增「附：部署问题去哪查」索引表：按问题类型指向 DEPLOYMENT 的小节，**不复制**端口、卷、域名、密钥值。
- 「本文件不写什么」同步改为指向 `DEPLOYMENT.md`，避免两处口径漂移。

### 8.2 验收

- FAQ 内不出现环境真值（域名 / IP / 端口映射 / 密钥 / 个人目录）；
- 索引表每一行都能在 `DEPLOYMENT.md` 找到对应小节；
- A184 的 dead_path 仍为 0（新增的 `docs/augmentor/DEPLOYMENT.md` 引用必须活）。

---

## 9. 风险、回滚与边界

| 风险 | 影响 | 缓解 |
|---|---|---|
| A184 扫描面扩大导致读数翻动 | 棘轮误红 | 先跑普查、定位新增 ambiguous / dead，再重钉；只钉可移植的 `SCRATCH_PAIR` 与 dead 四档 |
| Z-FULL 生成 schema 与后端漂移 | 前端误报或漏报 | conformance 以 pydantic 源码为 ground truth；字段总数 389 与 85 模型双钉 |
| L-REST 分类口径误伤 | 把该治理的算成 `single_row`，或反之 | 分类规则写进脚本 docstring；基线按文件+桶，新增即红；7 处改动逐条给理由 |
| 加 LIMIT 改变导出/聚合语义 | 数据被截断 | 只改 7 处高置信列表；导出/备份/迁移/全遍历一律留基线，不顺手改 |
| 前端分页字段读错（`total` vs `count`） | 页码/计数显示错 | 以 `u2_person.rs` 的返回结构为准；`count` 缺失时回退数组长度 |
| 本机无 Rust 编译器 | 编译错误只能在 CI 发现 | `cargo fmt --check` + 静态扫描 + CI；§10 明确写「本机未编译」 |

**回滚**：每批一个提交，按批次 `git revert <sha>` 即可；L-REST 的基线文件与扫描器同批提交，回滚时一起回滚，不留下「有 job 没基线」的半态。

## 10. 验证台账（实施中填写）

> 口径：只有「有改动 + 有命令/测试证据」才算完成；「计划了」「看起来对」不算。Rust 项必须注明本机是否编译。

| 批次 | 状态 | 证据 | 提交 |
|---|---|---|---|
| S L-SKIP-LEDGER | 结论已定，随计划提交 | 本文档 §3：明确「原名册不可恢复」、保留「16 ≠ 14」矛盾、不伪造名册 | 见 §12 |
| A A184 | 本机全绿，已提交 | `pytest tests/unit/test_comment_refs_a184.py -q -o addopts=""`：**26 passed / 1 skipped**；普查读数 `files_seen 627 / refs_total 6967 / live 4985 / symbol_live 952 / ambiguous 497 / scratch_missing 1484 / anchor_claims 40`；dead 四档 0；注入 `docs/augmentor/GHOST_FILE_NOT_REAL.md:12` 后 `dead_path 1` 并点名，还原后无残留 | 见 §12 |
| Z Z-FULL | 本机全绿，已提交 | `vitest run`：24 files / 415 tests；conformance 93 tests、字段总数 389；`tsc --noEmit` / `biome check` / `vite build` 全 0 错 | 见 §12 |
| L L-REST | 已实施并自证 | 10-27 轮：`python scripts/limit_scan.py` 基线态退出 0：2745 条 / 138 文件 / `bounded=559 unbounded=624`；基线 `limit_unbounded.tsv` 624 条 / 76 文件；反向自证：临时 `crates/ai/src/__limit_scan_probe_tmp.rs` → 退出 1 并点名 `unbounded 1 条 > 基线 0 条`，删除后恢复 0；7 条改动 SQL 在本机 PostgreSQL 容器内逐条执行成功（空表 0 行、非空表 1 行）；`rustfmt --check --edition 2021` 对 7 个改动文件无输出（本机无 rustc，编译/测试交 CI）。**10-28 修订**：扫描面排除 11 个 `security_gate_tests.rs`（83 条测试 SQL 不再入产品面），`single_row` 收紧为身份键约定、拆出 `parent_key` 桶进基线；BBS 别名 9 处 + 消息 1 处 + 流程 4 处 = **14 条列表**加 `LIMIT 100`；`python scripts/limit_scan.py` 基线态退出 0：2662 条 / 127 文件 / `bounded=573 unbounded=617 parent_key=113`；基线 `limit_unbounded.tsv` 合计 730 条 / 82 文件（`unbounded 617 + parent_key 113`）；反向自证在 `unbounded` 与 `parent_key` 两桶均成立（`parent_key` 探针用 `WHERE person_id = $1` 形状，临时 `crates/ai/src/__limit_scan_parent_key_probe.rs` → 退出 1 并点名 `parent_key 1 条 > 基线 0 条`，删除后恢复 0）；`cargo check` 覆盖 6 个改动 crate 全绿；`cargo fmt --check` 覆盖 4 个改动的 `.rs` 文件全绿（`bbs/section.rs`、`bbs/subject.rs`、`bbs_assemble_control/lib.rs + u2.rs`、`message_assemble_communicate/lib.rs`）；`cargo test` 4 个 crate 全绿（7 + 57 + 107 + 71 共 242 通过）；**既有 fmt 基线未动**：`processplatform_service_processing/u2.rs:346` 的 `dict_data_fns!` 宏块在 HEAD 已有 fmt diff，不在本次改动范围 | 见 §12 |
| O 分页联动 | 已实施并本机全绿 | `pnpm --filter @oa4rust/desktop exec tsc --noEmit` 退出 0（无输出）；`pnpm --filter @oa4rust/desktop build` 退出 0（built in 3.01s）；`biome check` 4 个改动文件 0 错；URL 由 `personPage` 插值、总页数由 `count/size` 计算；BBS 搜索页码改 `page/${page.value}`（**交互未实跑**，无浏览器会话） | 见 §12 |
| B BOM-FAQ | 已改并本机全绿 | FAQ 顶部 / 「本文件不写什么」/ 文末索引三处指向 `docs/augmentor/DEPLOYMENT.md`；A184 dead_path 0（新增引用是活的） | 见 §12 |
| G gap-closure（10-28） | 已实施并本机全绿 | 同 L-REST 10-28 修订行；决策表见 §6.5（`parent_key` 113 条按端点语义分 4 类：P1 权限全量 25 / P2 报表导入 19 / P3 单行期望 9 / P4 待分页 60）；CI `limit-scan` job 注释与 `.github/workflows/oa4rust-ci.yml` 同步更新 | 见 §12 |
| H P4 批量收口（10-28 第三轮） | 已实施并本机全绿 | 三组勘察（60 条 P4 + P2 attendance 11 条逐端点核对 SQL/路由/消费方式）后 38 处加 LIMIT（37×`LIMIT 100` + 1×`LIMIT 1`）：`parent_key 113→75`、`bounded 573→611`；`python scripts/limit_scan.py --update-baseline` 重生成基线 692 条，基线态退出 0；地板 `parent_key 100→65`（比例惯例 ≈87%）；反向自证：`WHERE person_id = $1` 探针 → 退出 1 点名文件，还原退出 0；`cargo check` 8 crate 全绿（attendance/meeting_control/mind_control/program_center/personal_extend/org_control/cms/portal×2 等）；`cargo test` 9 crate 全绿（48+62+32+152+19+267+337+27+26）；`cargo fmt --check` 改动 crate 干净（`pp u2.rs:346` 为既有本机版本差异）；`file_assemble_control` 因本机无 C 工具链（gnu host 无 gcc）check/test 交 CI，此前同 crate 按 zip 更大 features 集编译绿 | 见 §12 |

## 11. 本轮明确不做

- 不补写 L 批次 14 条跳过名册（不可恢复，见 §3）。
- 不对 641 条无界 SELECT 逐条加 LIMIT；只治理 7 处高置信列表 + 14 处 10-28 列表（§6.3 第二轮），其余进基线。
- 不把 `single_row` 的 `*_id = $1` 重新分类为 unbounded；**10-28 修订后口径已变**：非身份单列父键（`work_id` / `person_id` / `doc_id` …）归 `parent_key`，与 `unbounded` 一样进基线棘轮；`parent_key` 的边界与「按 scope 收窄 / 接分页」的下一轮路径写在 §6.5 决策表。
- 不改后端 `person_list_filter_paging` 的已有分页语义；本轮只把前端接回去。
- 不在 FAQ 里复制部署细节、端口、卷、域名或密钥注入方式。
- 不批量格式化既有文件；不修与六项缺口无关的 lint / 格式基线。

## 12. 提交记录

> 批次提交完成后回填。每个 hash 都能用 `git show --stat <hash>` 复核；CI 结论以 `gh run list --commit <hash>` 为准。

| 批次 | 提交 |
|---|---|
| Z-FULL | `1e4ba2280` feat(augmentor/web): Z-FULL——78/78 端点接入 zod 响应契约（70 个全量 schema） |
| A184 + BOM-FAQ | `499da9a8e` test(augmentor): A184 接入 docs/augmentor 14 份 Markdown；FAQ 补部署索引 |
| L-REST | `de023d18c` feat(oa4rust): L-REST——无界 SELECT 全量普查 + 基线棘轮，7 处列表加 LIMIT |
| L-REST 10-28 修订 | `d1cf2a73e` feat(oa4rust): L-REST 修订——扫描面排除测试夹具、single_row 收紧为身份键、拆出 parent_key 桶进基线；BBS 别名 + 消息/流程 16 条列表加 LIMIT 100（16 处以 diff 计数为准，§10 的「14 条」与原 §12 草稿的「12 条」是笔误） |
| OA-LIMIT-OFFSET | `3c9b39122` fix(oa4rust/web): OA-LIMIT-OFFSET——人员/授权列表接服务端分页，BBS 搜索跟随页码 |
| 台账回填 | 本章节自身提交 |

