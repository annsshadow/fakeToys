# 第四轮 · 有库深度验证 + augmentor 前端 zod + LIMIT 抽样 + 文档 FAQ · 执行计划

- **登记日**：2026-10-26
- **范围**：oa4rust（有库行为验证 / LIMIT 抽样 / FAQ 文档）、augmentor/web（zod 运行时 schema 校验 + FAQ）
- **目标**：10-26 第三轮（P0~P1 批次 + §5.2 有库轮次）已把 FI002 的 41 对迁移缺列 + 7 对遗留表**静态收口**
  （111/112 迁移 + 扫描器补盲 + CI 棘轮全绿），本轮在此之上做三件 10-26 之后仍挂着的真实工作：
  ① 对着**真库**把关键端点的 SQL 行为实跑验证（从"迁移在位"升级到"端点行为可证"）；
  ② 737 处列表查询无 LIMIT 的**抽样治理**（只挑前端真实调用的热点端点，加 LIMIT + OFFSET，不盲改全量）；
  ③ augmentor/web 唯一的 P3 工程项——**zod 运行时 schema 校验**（把编译期 TS 声明升级为前后端契约机器保证）；
  ④ 两份文档 FAQ（augmentor 端点 FAQ + oa4rust FAQ，计划文档明确列出的低价值但已排期的收尾项）。
- **方法**：与 10-24/10-26 两轮同范式——只读勘察先行（真库查询 / 代码 grep / 契约文件核对），
  每项含证据、改法、风险、验收判据；§6 为验证台账（实施中填写，口径：已完成 = 有改动 + 有构建/测试证据）。

## 0. 与既有计划的关系

| 既有计划 / 台账 | 本文档承接 |
|---|---|
| `2026-10-26-fresh-db-500-closure-round.md` | §5.2 已验"迁移在位 + 关键 SQL 形态可通过"；本文档 §1 在其上补**行为级**验证（端点实调 + 响应键断言），不做 10-26 已收口的 FI002/迁移项 |
| `2026-10-24-comprehensive-optimization-round.md` §15 台账 | OA-PERF-LIMIT（737 处无 LIMIT，原判"数据量未到临界"延期）→ 本文档 §2 **抽样**降级执行（只热点端点）；augmentor P3 前端 zod（唯一未启动 P3）→ 本文档 §3 |
| `augmentor/OPTIMIZATION_PLAN.md` §3.1/§3.3 | 前端 zod 运行时 schema 校验（P3 待办）+ 文档 FAQ → 本文档 §3/§4 |
| `docs/tech-debt.md` | TD-1~3 状态不变；本轮不新增技术债（若抽样 LIMIT 发现需产品决策的端点，转 §5 延期台账） |

## 1. 环境基线（本轮实跑，2026-10-26）

- **PostgreSQL 可用**（推翻 10-26 §2.4 "本机无 PG" 的旧口径——那是 Docker 未运行时的快照）：
  - 容器 `lyp-pg`（postgres:17.11，Debian 13），端口 5432 已映射；
  - 库 `oa4rust`（owner `o2server`，384 张业务表）：001→112 全量迁移已应用、**fresh 终态**（10-26 §5.2 rollback→重 apply 的产物）；
  - 库 `laiyipao`：laipyiipao 服务器继续用（本轮不动）；
  - 连接串：`postgres://o2server:password@localhost:5432/oa4rust`（`shared::testing::test_pool()` 默认值，10-26 §5.2 已验可连）。
- **Rust 本机编译仍不可用**（GNU 工具链缺 gcc/dlltool，无 MSVC，与 10-26 §2.4 一致）：
  Rust 改动本机只做 rustfmt（CI 同版本 1.98.1）语法/格式 + 人工复审，编译/测试执行交 CI；
  **但本轮新增一条 10-26 没有的验证通道**——真库 SQL 直跑（`docker exec lyp-pg psql` 实跑 handler 同形 SQL），
  这是行为级证据，强于 10-26 §5.2 的 information_schema 抽样。
- **augmentor 解释器**：`C:/Python314/python.exe`（3.14.4，依赖齐全，10-24 §7 口径）。

## 2. 批次总览

| 批次 | 主题 | 项数 | 风险 | 提交策略 |
|---|---|---|---|---|
| V | oa4rust 有库行为验证（P0-1~5 端点 + F-3/F-7/F-8 形态） | 6 | 低（只读查询 + 测试，不改生产代码） | 1 提交 |
| L | 737 处无 LIMIT 抽样治理（热点端点加 LIMIT） | 1 批 | 中（改 SQL，需前端契约核对） | 1 提交 |
| Z | augmentor/web zod 运行时 schema 校验 | 3 | 低-中（新增依赖 + 类型层改动） | 1-2 提交 |
| D | 文档 FAQ（augmentor 端点 FAQ + oa4rust FAQ 骨架） | 2 | 低 | 1 提交 |

每批完成后跑该批门禁 → commit → push（沿用前轮 fast-forward 直推 main 模式；推送后在 §6 台账记录 commit）。

---

## 3. V 批次 — oa4rust 有库行为验证（10-26 §5.2 的升级）

> 10-26 §5.2 已验「列/视图在位 + 关键 SQL 形态可通过」；本批次把它升级为「**端点级行为可证**」：
> 对 10-26 P0-1~5 收口的 5 类端点，用真库直跑 handler 同形 SQL（`docker exec lyp-pg psql`），
> 断言：① 语句可执行（fresh 库无 500 形态的列/视图缺失）；② 响应键形态与 handler 代码读取一致；
> ③ 空表返回"诚实的 0"而非报错。不改生产代码，纯验证，产出证据入 §6。

### V-1 `process_bam` 两端点（F-1 收口验证）

- **证据**：`crates/process_bam/src/lib.rs:20-23`（state_summary 三个 FILTER 谓词）、`:66`（state_running）；
  P0-1 已补 `x_process_bam.status` 列（112）。
- **验证**：真库直跑 state_summary 的完整 SELECT（含 FILTER 谓词）→ 断言返回 1 行、
  `total/running/completed/expired` 四键（fresh 空表全 0，诚实）；同法跑 state_running。
- **前端调用**：`ProcessWork.vue:3010-3012`（10-26 §2.2 已核）。

### V-2 `state_category_trigger`（F-2 收口验证）

- **证据**：`crates/processplatform_assemble_bam/src/lib.rs:2207`；P0-2 已补 `x_work.category`。
- **验证**：真库直跑该 handler 的 WHERE 子句形态 → 可执行、空表 0 行；
  前端 `BamApp.vue:535`（10-26 已核）。

### V-3 `PP_E_APPLICATION` 视图查询（F-3 收口验证）

- **证据**：`processplatform_assemble_surface/src/lib.rs:887,975`；P0-3 已在 112 建视图。
- **验证**：真库直跑 handler 的两条带引号大写形态查询 → 可执行；
  **同时做负对照**：临时 `DROP VIEW "PP_E_APPLICATION"` 后同语句必报
  `relation does not exist`（证明验证是响的，不是空跑）→ 还原重建（112 的幂等守卫可重 apply）。

### V-4 `pp_c_keylock` 别名形态（F-7 收口验证）

- **证据**：`processplatform_assemble_surface/src/lib.rs:2916` 等 4 处别名化（P0-8）；
  真库验证 10-26 §5.2 已跑过一次（"F-7 keylock 别名通过"），本轮**扩到 F-6 的 11 处 ref_col 引号化形态**
  （`pp_c_attachment`/`pp_c_snap` 的 WHERE 带引号 `"xworkCompleted"`）：直跑 2 条代表形态。

### V-5 `PP_C_TASK` LEFT 形态（F-8 收口验证）

- **证据**：`processplatform_assemble_surface/src/lib.rs:28742-28743`（P0-8 改 `LEFT(col,10)`）；
  10-26 §5.2 已实证旧形态 `TO_CHAR(text)` 必 500、新形态通过。本轮补一条**灌数据回归**：
  向 `pp_c_task."xcreateTime"` 插 1 条 ISO 文本 → 断言 `LEFT(...,10) = $1` 命中；脏值（非 ISO 前缀）不命中但不 500（纯文本前缀匹配语义，10-26 已判）。

### V-6 遗留 4 表端点形态（P0-4 收口验证）

- **证据**：112 建的 `pp_c_touch_detained/expire/passexpired`（各单列 `xupdatetime`）+ `x_process_transition`；
- **验证**：真库直跑 4 个引用端点的 handler SQL → 可执行、空表 0 行（与 fresh 500 家族病对照：10-26 前这 4 个端点在 fresh DB 必 500）。

**验收判据（本批统一）**：每条验证都留下「语句原文 + 执行结果（行数/键形态）」两行记录进 §6；
V-3 的负对照、V-5 的灌数据回归是**必须项**（防止验证空转——10-26 F-4 的教训：假断言绿着但没测到东西）。
**不改生产代码**；若验证中发现 10-26 某项实际未生效（如 112 某列没在库上），fail-loud 并升级为 P0 修复项（新批次，不在本批内）。

---

## 4. L 批次 — 737 处无 LIMIT 抽样治理（延期项降级执行）

> 10-24 §15 对 OA-PERF-LIMIT 的延期理由是「数据量未到临界 + 改响应形状影响前端契约」。
> 本轮不推翻该判断（不盲改 737 处），只做**抽样**：挑「前端真实调用 + 列表语义明确 +
> 数据随使用增长」的端点加 `LIMIT 100`（或按业务合理值）+ `OFFSET` 可选，
> 响应形状兼容（多返回 `total_count` 或不改，逐端点定）。**每处改动必须 grep 前端调用点核对契约**
> （前端是否已自己做分页/切片；若前端已 `slice(0,N)` 展示，后端加 LIMIT 是纯收益）。

### 选端点的三把尺子（全部满足才进本批）

1. **前端真实调用**：`frontend/apps/desktop|mobile` 里 grep 到调用点（非死路由）；
2. **列表语义 + 无界**：SQL 是 `SELECT ... FROM <表> [WHERE user 级过滤]` 且无 LIMIT，
   表数据随使用自然增长（`auth_person` / `x_bbs_*` / `x_meeting_*` 等业务表，非配置表）；
3. **单页展示**：前端消费该响应是"一页/一屏"语义（表格前 N 行 / 下拉 / 最近 N 条），
   而非"全量遍历"语义（如导出、统计聚合——后两类跳过，加 LIMIT 会改语义）。

### 实施流程（每个候选端点）

1. grep 前端调用点 → 记录「消费方式」（表格 / 下拉 / 列表 / 聚合）；
2. 读 handler SQL → 判「是否已有 user 级过滤」（无过滤的全表列表优先，收益最大）；
3. 加 `LIMIT <N>`（N 按业务定：下拉 20~50、表格 100；`OFFSET` 仅当前端有翻页控件时加，否则不加）；
4. 响应形状：默认**不改**（前端原本就按数组消费）；若端点同时返回 `total` 计数，
   核对 `total` 是否仍全表统计（加 LIMIT 后 `total` 语义是否变——变则拆分：`total` 保留全表、`items` 加 LIMIT，逐处判断）；
5. 本机：rustfmt + 人工复审 SQL；CI：`cargo test`（对应 crate）+ FI-002 棘轮不假红；
6. 真库直跑加 LIMIT 后的语句（fresh 库）确认可执行。

### 规模预期

按 737 处中「前端真实调用」的比例（10-24 勘察提到前端双端已对接大部分路由），
预计候选 40~80 处，其中满足三把尺子的约 **10~20 处**进本批；
不满足的（全量遍历语义 / 无前端调用 / 聚合类）**逐处记入 §5 延期台账并给理由**，不静默跳过（fail-loud）。

**风险**：中——SQL 改动碰生产查询。缓解：只加 `LIMIT`（不改 WHERE/JOIN/GROUP BY）、
响应形状默认不变、每处 grep 前端 + 真库实跑双证。若某端点核对后判定"加 LIMIT 会改前端语义"
（如前端 `reduce` 全量聚合），跳过并记台账，不硬改。

---

## 5. Z 批次 — augmentor/web zod 运行时 schema 校验（P3 唯一未启动项）

> `OPTIMIZATION_PLAN.md` §3.1 列的 P3 待办：「运行时 schema 校验（如 zod），把『编译期声明』升级为
> 『前后端契约机器保证』」。现状：`src/types/api.ts` 61 个接口由**真实响应探测**生成（§2.6），
> `services/api.ts` 43 个函数有契约测试 + 接线守门（§2.3/§2.6）——但全是**静态**的：
> TS 类型只编译期有效，运行期后端多回/少回/类型漂移的键，前端只能靠页面崩或 undefined 兜底。

### 现状勘察（实施前先读，以下为预判，实施时核实）

- `web/package.json` 依赖清单（有无 zod / 是否需新增；若新增，版本钉 + lock 文件）；
- `services/api.ts` 的 43 个函数返回类型分布（哪些是 61 个接口里的高频消费型）；
- 既有守门测试 `tests/**/api.contract.test.ts` / `api.wiring.test.ts` 的结构（zod 校验层要插在哪：
  预判是 `services/api.ts` 每个函数返回前 `Schema.parse()`，或集中 `validateResponse()` 一个入口）；
- `eslint --max-warnings 0` 与 `tsc --noEmit` 门禁（新增类型层不能破坏 0 告警纪律）。

### 改法（最小面，不重构既有服务层）

1. **新增 `zod` 依赖**（版本钉进 `package.json` + `pnpm-lock.yaml`）；
2. **新增 `src/schemas/` 目录**：不重抄 61 个 TS 接口，只给**本批被校验的接口**写 zod schema
   （与 TS 接口一一对应，字段名/可选性逐键核对——契约测试已把「真实响应键集」钉死，zod 以它为 ground truth）；
3. **`services/api.ts` 加一个集中校验入口** `checkedFetch(schema, ...)`：
   `api.ts` 现有 43 个函数中，**先只接高频消费型**（数据管理 / 分析 / 质量中心三条主链路，
   约 10~15 个函数——实施时按页面调用频率定），其余保持原样（全量接线是后续轮次的活，不在本批）；
4. **校验失败语义**：`ZodError` 包成领域错误（与既有 `to_http_error` 语义对齐的中文文案，
   不裸抛 `issue` 原文——10-24 B5 轮 augmentor 后端同款判据：坏响应不得把库内部措辞透给页面）；
5. **守门测试**：新增 `schemas.conformance.test.ts`——对每个已接 zod 的接口，用**真实后端响应**
   （`TestClient` 或既有契约夹具）跑 `schema.safeParse`，失败即红（zod 与真实响应对不上 = 契约漂移）；
   **双向**（与 10-24 §2.3 既有守门同范式）：zod 声明的键 ⊆ 真实响应键 且 真实响应必填键 ⊆ zod 必填键。

### 不做（本批边界，转 §5.1 台账）

- 不给 43 个函数全量接 zod（只做主链路 10~15 个，全量接线是「契约全量机器化」独立轮次）；
- 不改后端 FastAPI（`response_model` 已是静态契约，zod 是前端的运行期镜像，两侧各自守门，不做强一致工具链）；
- 不引入 openapi-typescript 等代码生成器（61 接口手工 zod 化已够，生成器引入 build 复杂度，收益不成比例——与 10-24 §9 AU-P2 同族判断：能不做工具链就不做）。

**验收**：`npm test` 全绿（含新 conformance 测试）+ `tsc --noEmit` 0 错 + `eslint --max-warnings 0`；
`check-warnings --only augmentor-web` 通过。

---

## 6. D 批次 — 文档 FAQ（两份，排期已存在的收尾项）

### D-1 `docs/augmentor/FAQ.md`（OPTIMIZATION_PLAN §3.3 待办）

> **落点订正**：原计划写 `augmentor/docs/FAQ.md`。执行时发现 `augmentor/README.md` 的
> §文档 已声明「详细文档汇总到仓库根 `docs/augmentor/`（2026-10-14 迁出）」，
> 即 `augmentor/docs/` 是被废弃的旧布局。按仓库更近、更权威的约定落到
> `docs/augmentor/FAQ.md`，并在 `docs/augmentor/README.md` 与 `augmentor/README.md`
> 两处文档索引补链接——不新造第二个 docs 目录。

- 内容面（从 `OPTIMIZATION_PLAN.md` / `README.md` / 端点契约中抽取，不新造事实）：
  ① 多模型后端怎么选/切（ERNIE/OpenAI/Ollama/Claude/Gemini 的**选型与切换**——勘察确认
  **不存在跨厂商自动降级链**，FAQ 按实现写，订正见 §8；以及 `DiskCache` 什么条件下命中）；
  ② 数据格式 13 种（`ExportFormat` 枚举为准）互转边界（L16/L20 收口的「容器 vs 行内 schema」）；
  ③ 数据安全三件套（脱敏 / 泄漏检测 / 就绪审计）的触发时机；
  ④ 路径白名单（`web.data_roots` 出厂 `["data"]`，F-07/F-08 的读/写不对称语义）；
  ⑤ 性能热点现状（dedup 20MB / 流式 1.66MB / 倒排搜索 7.96ms——dedup 与流式引 §2.2 实测值，
  倒排 7.96ms 出自 `OPTIMIZATION_LOOP.md` 的 L13 条目，**不在 §2.2**，不写"约"）。
- **纪律**：所有数字/行为必须能指回 `OPTIMIZATION_PLAN.md` 或代码锚点（file:line），
  指不回的删掉不写（与 10-24 F-06「文档漂移」同类风险，FAQ 是新文档，出生即带锚点）。

### D-2 `oa4rust/docs/FAQ.md` 骨架（10-26 台账未排期，本轮补骨架不补全）

- 为什么只补骨架：oa4rust 是 96 crate 的大系统，全量 FAQ 是独立轮次；
  本批只落**高频问题 3 条**（有 10-26 台账可指回的）：
  ① fresh DB 的 500 家族病是怎么防的（FI-002 扫描器 + 111/112 迁移 + `mount_coverage_guard`，指 10-26 §2/§3）；
  ② 契约路径别名（73 条）为什么存在、中间件四表镜像的不变式（指 10-24 R1）；
  ③ 本机无编译器时 Rust 改动的验证口径（指 10-26 §2.4：rustfmt + Python 端口 + 人工复审 + CI 真库）。
- 不写"如何写新 crate"之类无台账可指回的内容（宁少勿错）。

**验收**：两份 FAQ 每个事实句都有可指回的锚点（文档 §号 或 file:line）；`docs/` 链接完整性（本仓有 link-format 审计先例，提交前跑一遍）。

---

## 7. 明确不做 / 边界（§5.1 延期台账）

| ID | 事项 | 不做理由 |
|---|---|---|
| L-REST | 737 处无 LIMIT 的**全量**治理 | 抽样已覆盖热点（L 批次）；全量需逐端点灌数据压测 + 前端分页契约逐端点对表（10-24 原判延期理由未失效，本轮只是把"最痛的那几处"摘出来先做） |
| L-SKIP-LEDGER | 本轮跳过的 **14 处**候选的逐条名册（端点名 + 理由 + 前端消费方式） | 本轮只把 8 处命中项的改动写进 §8；跳过项的分类计数进了 commit message，逐条名册未落盘，且分类相加为 16 ≠ 14（类别可能重叠）。数字以「14 处」为权威，名册在下一轮 LIMIT 抽样时补齐——**不当作已完成** |
| Z-FULL | 43 个服务函数**全量**接 zod | 主链路 10~15 个先接（Z 批次）；全量接线是「契约全量机器化」独立轮次，且每接一个要写一份 conformance 测试，一次做完回归面太大 |
| OA-LIMIT-OFFSET | 后端加 OFFSET/翻页控件联动 | 前端无翻页控件的端点只加 LIMIT（截断到前 N），不加 OFFSET；做完整分页是前后端联动需求，转产品决策 |
| BOM-FAQ | augmentor FAQ 里写"如何部署 Docker 生产" | `docker/` 目录 + README 已有；FAQ 不重复运维文档（防漂移），只写"高频使用问题" |

## 8. 验证与提交台账（实施中填写）

> 口径同前两轮：**已完成 = 有改动 + 有构建/测试证据**；跳过/待外部条件逐条写明。

| 项 | 状态 | 证据 | commit |
|---|---|---|---|
| V-1~V-6 有库行为验证 | **已完成** | 真库（lyp-pg / oa4rust 库 / o2server）直跑记录：V-1 state_summary FILTER 四键 + state_running 0 行；V-2 state_category_trigger 0 行；V-3 视图查询 0 行 + **负对照**（DROP VIEW 后 `relation does not exist`，还原 112 后在位）；V-4 keylock 别名 0 行；V-5 **灌数据回归**（ISO 文本 1 命中、脏值不命中不 500）+ **负对照**（`TO_CHAR(text)` 实证 `function to_char(text, unknown) does not exist`——F-8 原缺陷形态真库复现，LEFT 改法正确）；V-6 遗留 4 表 SELECT/UPDATE 全部可执行（UPDATE 加 `WHERE FALSE` 保护，0 行副作用）。库已恢复 fresh 终态 | — |
| L 批次 LIMIT 抽样 | **已完成** | 8 处热点 → **16 条 LIMIT**（9 个文件）：org_express `unit_list_all`、org_control `person_list_like` + `u2_person` 两分支、portal `dict_list`、program_center `config_list` 同族 5 处、auth `role_list`（50）、empower 3 处、mind `list_folders`、query `query_list`。跳过 14 处（逐条名册缺口见 §7 L-SKIP-LEDGER）。验证：真库逐条直跑通过（fresh 空表 0 行 / 非空表截断正确）、`cargo fmt --check` 8 个改动 crate 通过、`fi002_column_scan.py` 基线态 exit 0；`cargo test` / clippy 交 CI（本机无编译器） | `12c59474e` |
| Z 批次 zod | **已完成** | `zod@4.6.7` + `@types/node`（仅 devDependency）；schema 以 pydantic `response_model` 为唯一 ground truth（15 模型 / 53 字段），`checkedResponse<T>` 接 14 个主链路函数、校验通过返回**原始** `response.data`（不是 zod 的裁剪克隆）。`main-schemas.conformance.test.ts` 静态解析 Python 类做双向键集 + 必填性比对（反向注入自证：改 `version_id` 后守卫变红、还原后绿）；`api.test.ts` 16 处与后端契约不符的旧 fixture 改为契约形状，另补 5 条运行时行为测试。门禁：vitest **336 passed** / `tsc --noEmit` 0 错 / `eslint --max-warnings 0` 0 / `vite build` 通过 / `check-warnings --only augmentor-web` PASS | `373548cf2` |
| Z-CI-REPIN | **已完成** | `AI platform CI` 在 `373548cf2` / `5169ea920` 的红灯共两处，均非产品缺陷：① `test_api_endpoint_consumption_l224.py` 把前端产品语料与「被排除的测试替身」精确钉成各 22；Z 批次新增 `web/src/schemas/main-schemas.ts`（产品）与 `main-schemas.conformance.test.ts`（替身）⇒ 两侧各 23。断言与 docstring 同批重钉为 23 并写明归因，判据方向未动（schema 不发 `api.<verb>` 调用、替身仍被 `.test.` 排除）。② 重钉提交的主题提到 `L224`，而 `test_log_block_bullet_ownership_l264.py` 的 `probe()` 取「ref 面上最新一枚带轮号的提交」⇒ 探针从 L294 漂到 L224，5 支注入臂假红（`cross_unnamed: 1`）。改为按块号取探针（`probe(round_no)`；跨轮臂 `with_probe_block(..., probe_round=294)` 显式要 L294），不再按「谁最新」取。验证：两支用例本机分别 24/24、1/1 通过；同文件其余用例本机全部失败于 `ModuleNotFoundError: fastapi`（环境缺口，非断言），交 CI 复核；A184 棘轮档逐格不变（refs_total 6167 → 6169，两格增量全是 `live`） | `cd18b6182` + 本提交 |
| D-1/D-2 FAQ | **已完成** | 新增 `docs/augmentor/FAQ.md`（5 组问答：模型选型 / 13 种格式 / 安全三件套 / 路径白名单 / 性能热点，落点按 README 的文档迁移约定订正，见 §6-D1）与 `oa4rust/docs/FAQ.md`（3 条骨架：fresh-DB 500 防线 / 73 条契约别名与四表镜像 / 本机无编译器的验证口径），两处文档索引已补链接。锚点逐条核对：FAQ 的 31 条代码引用用 A184 的同一把尺子逐条判定为 `live`，文档引用用 §号。A184 普查改动前后**逐格相同**（files 628 / refs 6167 / 四档死链 0 / ambiguous 462 / runtime_ns 1 / scratch_missing 1365） | `5169ea920` |

**D 批次的订正、环境事实与已知缺口（fail-loud，不写「已完成」了事）**

1. **「多模型降级顺序」是计划里的错误措辞**。实现勘察（`augmentor/models/factory.py` 的
   `MODEL_BACKENDS` 表 + `augmentor/models/base.py` 的同后端重试）确认：**没有跨厂商自动降级链**，
   一次请求只打 `models.default` 选中的那一个后端。FAQ 按事实写「单后端 + 同后端内重试」，
   §6-D1 的要求文字已同步改写，不留两份互相矛盾的说明。
2. **本机没有 augmentor 的 Python 运行时**（无 fastapi / pydantic；为跑 A184 文档引用守卫，
   另在系统 Python 3.12 装了 pytest 9.1.1，仅本机工具、不入仓）。因此后端 pytest 与
   `TestClient` 真实响应夹具**本机跑不了**：Z 批次的跨语言一致性用「静态解析 `response_model`」
   替代，运行时证据由 CI 的 pytest 套件提供——**不声称「本机已验证后端」**。
3. **L 批次跳过项的逐条名册缺失**：commit message 只留分类计数，且分类相加 16 ≠ 14。
   已在 §7 立 `L-SKIP-LEDGER`，把补齐责任留给下一轮，不把缺口写成完成。
4. **FAQ 落在 `docs/augmentor/` 后不在 A184 的扫描面内**：那把尺子扫的是 augmentor 项目内的
   Markdown 与代码注释，仓库根 `docs/` 不在其中。所以本轮 31 条代码引用虽已逐条判定 `live`，
   日后搬行**不会自动变红**——属已知缺口，登记在此；要补自动化须把 `docs/` 纳入扫描面，
   那是独立改动（会一并收进 `docs/plans/` 的存量引用），不在本批夹带。

**待用户操作**：CI 全绿确认（每批推送后，gh 登录态沿用 10-26 的 keyring）；
若 V 批次发现 10-26 某项实际未生效（库上缺列/缺视图），fail-loud 升级 P0，不在本批内私下修。
