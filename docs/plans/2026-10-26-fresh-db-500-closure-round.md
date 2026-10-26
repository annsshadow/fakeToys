# 第三轮 · fresh-DB 500 收口与台账存档 · 执行计划

- **登记日**：2026-10-26
- **范围**：oa4rust（迁移 / FI-002 扫描器 / 测试）、docs/plans 台账
- **目标**：把前两轮（B1–B6 + R1–R8）的批次内容存档到 `docs/plans/`；并对本轮只读勘察新发现的
  **同一家族真缺陷**（fresh DB 必 500、既有测试与扫描器结构性漏过）给出执行计划。
- **方法**：只读勘察——FI-002 扫描器探针（增抽 WHERE/FILTER 裸列）+ 迁移 DDL 原文核对 +
  路由/前端调用点核对；随后 `gh` 登录核对 CI（run `40262029237`），PG 容器日志**在线实证**了
  多项缺陷。本文件为**执行计划**：每项含证据、改法、风险、验收判据；§5 为验证台账。

## 0. 与既有计划的关系

| 既有计划 / 台账 | 本文档承接 |
|---|---|
| `2026-10-24-comprehensive-optimization-round.md` | R1–R8 的执行细节（§14/§15 权威）；本文档 §1 只做**台账存档与缺口登记**，不重抄 |
| `2026-10-23-cross-subproject-optimization-backlog.md` | FI002-LEGACY-TABLES 延期项的收尾方案（P0-4） |
| `docs/tech-debt.md` | TD-1 已修、TD-2/3 已接受；本轮不新增技术债 |

---

## 1. 台账存档：R1–R8 状态与两处缺口（2026-10-26 登记）

### 1.1 已落地批次（commit 哈希已回填）

| 轮次 | 交付 | commit |
|---|---|---|
| R1 | 契约路径别名 73 条 + 中间件四表镜像 + 双重 parity 守卫 | `ef48f3c35` |
| R2 | FI-002 静态列扫描固化 + CI 棘轮（首跑挖出 48 对缺口） | `b799858cf` |
| R3 | laiyipao 后台鉴权 3→1 + 钱包合并 + 读端点去写副作用 | `9b2ea89d6` |
| R4 | warnings-gate CI + cool jar 定源 + 两处 CI flaky 修复 | `4a14d6e82` |
| R6 | augmentor/web 类型检查两连修（夹具缺字段 + 缺字段不崩页 + ops 夹具） | `e89a79840`、`d306cd68c`、`62e32db62` |
| R7 | laiyipao 6 处「日界双口径」夹具 + 守卫 | `4e591a7cc` |
| R8 | FI-002 补 111 迁移——41 个 handler 引用列按代码绑定类型幂等补齐（基线 48→7） | `835d55cfb` |

### 1.2 两处台账缺口（fail-loud 登记，不编理由）

| 缺口 | 事实 | 处置 |
|---|---|---|
| **R5 编号缺口** | `git log --grep=R5` 在优化轮序列里**零命中**（R1/R2/R3/R4 → R6/R7/R8）；grep 命中的 "R5" 全是更早安全轮的规则编号，非同族 | 编号被跳过或用于未提交的内部工作，原因不可考。如实记缺口；后续轮次从 R9 继续编号 |
| **R8 未入旧文档** | R8 只存在于 commit message，`2026-10-24-*.md` 无对应条目（该文档 §14 止于 R4、§15 记 R6/R7） | 本文档 §1.1 即为其存档条目；旧文档不改动（surgical） |

### 1.3 CI 核对结果（2026-10-26，gh 已登录为 `annsshadow`）

R8（`835d55cfb`）七条 workflow：**六绿一红**。

| workflow | 结论 | 说明 |
|---|---|---|
| oa4rust CI（`oa4rust-ci.yml`） | **failure** | 仅 `unit-tests` job 红：`correlation_service_processing::u2_delete_rejects_cross_job_ids` E23505（见 FX-1，与 R8 改动无关的夹具并发家族病，R4 修过同族的 `personal` crate） |
| behavior-compare | **success** | **111 迁移在 fresh PG 上应用干净**（R8 的核心风险点解除） |
| laiyipao / auto-checkin / warnings-gate / three-ends contract / supply-chain / CodeQL | success | — |

unit-tests 的 PG 容器日志同时**在线实证**了 F-1（`column "status" does not exist` ×2）、
F-3（`relation "PP_E_APPLICATION" does not exist` ×2），并暴露 F-5~F-8 四个新缺陷——
这些 SQL 错误被现有测试容忍（错误路径/不断言成功），故 job 只折在 FX-1 一个用例上。

---

## 2. 本轮新发现（2026-10-26 只读勘察）

### 2.1 勘察方法与误报排除

- 探针：`Temp/opencode/fi002_where_probe.py`（本机临时目录，**未入库**）——复用
  `oa4rust/backend/scripts/fi002_column_scan.py` 的 universe 与字面量提取，增抽 WHERE/FILTER
  子句裸列，与全量迁移解析（379 表 / 5137 列 / 211 迁移文件）对撞。
- 探针原始读数：表在宇宙内、列不在 **9 条**；表不在宇宙 **3 条**。逐条人工分判后
  **误报三类已全部排除**：
  1. `$N::text IS NULL` 强制转换被当成裸列（`calendar_assemble_control/src/u2.rs:421`、
     `cal_event` ×10、`mind_assemble_control/src/u2.rs:120`）；
  2. `information_schema.columns` 子查询的 WHERE 列被归属到外层表
     （`attendance_assemble_control/src/lib.rs:1114`）；
  3. `pg_stat_activity` 系统视图（`query_service_processing/src/lib.rs:215`）不在扫描器的
     `SYSTEM_SCHEMAS` 名单里。
- 排除后**剩余 3 条真缺陷**（F-1/F-2/F-3），加上既有的 F-4（测试钉 500）。
- **CI 核对（§1.3）后追加**：PG 容器日志在线实证 F-1/F-3，并暴露 F-5~F-8 与 FX-1（main 当前红）。

### 2.2 缺陷清单（均有 file:line 证据；「fresh DB 必 500」为静态判定）

| ID | 缺陷 | 证据 | 前端调用 |
|---|---|---|---|
| F-1 | `process_bam` 两个端点查**不存在的列** `x_process_bam.status` | `crates/process_bam/src/lib.rs:20-23`（state_summary 三个 FILTER）、`:66`（state_running）；DDL `migrations/036_create_additional_tables.sql:353-375` 只有 running/completed/expired/total 计数器列；全量迁移解析确认无 status（deleted_at 有，037 补的） | ✅ `frontend/apps/desktop/src/views/ProcessWork.vue:3010-3012` |
| F-2 | `state_category_trigger` 查**不存在的列** `x_work.category` | `crates/processplatform_assemble_bam/src/lib.rs:2207`；DDL `migrations/020_create_processplatform_tables.sql:4-15`，宇宙 28 列无 category | ✅ `frontend/apps/desktop/src/views/BamApp.vue:535` |
| F-3 | 两个端点查**幽灵大写视图** `"PP_E_APPLICATION"` | `crates/processplatform_assemble_surface/src/lib.rs:887`（application_list_key_key）、`:975`（application_list_terminal_terminal）；**没有任何生效中的迁移创建该视图**——唯一来源是 `migrations/archive/089_fix_500_endpoints.sql.disabled:21-37`，当年因「Creates views over nonexistent uppercase tables」被 archive（`27335205f`），`shared/src/migrate.rs:84` 排除 archive，从未应用；commit 自述「Columns applied manually via SQL instead」⇒ dev/live 库能跑、fresh DB 必 500。视图定义所需的混合大小写列已与 `migrations/032:700-724` 的 DDL 逐列核对一致 | ✅ `ProcessManagerApp.vue:208-209`、`ProcessWork.vue:2447-2449` |
| F-4 | `process_bam` 测试把故障态钉死 | `crates/process_bam/src/tests.rs:39-93`：用**无连接池**（`Config::new()`）断言 500——与 R7 日界夹具同族「绿着但没测到东西」 | — |
| F-5 | `x_program_field` 查询排序**不存在的列** `sort_order` | CI PG 日志实证（`column "sort_order" does not exist`，`SELECT ... FROM x_program_field ORDER BY entity, sort_order`）；宇宙解析确认该表无此列（032:2332 建的表）。ORDER BY 列不被扫描器抽取（B-1 变体） | 待核（program_center 相关页） |
| F-6 | `pp_c_attachment` / `pp_c_snap` 的 WHERE **漏引号** `xworkCompleted` | CI PG 日志实证 ×5（`column "xworkcompleted" does not exist`，HINT 指向混合大小写列 `"xworkCompleted"`）。来源是 `format!` 模板的 `ref_col` 常量裸插值（`processplatform_assemble_surface/src/lib.rs:25335,25511`，契约注释在 `:25663`）；模板其余部分引用都正确带引号 | 待核 |
| F-7 | `pp_c_keylock` 查询**引用了不存在的混合大小写列** `"xcreateTime"/"xupdateTime"` | `processplatform_assemble_surface/src/lib.rs:2916`（handler `control_workorworkcompleted_workOrWorkCompleted`）；032:385-394 的实际列是 `create_time`/`update_time`（小写）；CI PG 日志实证（HINT 同指）。xid/xtitle/xperson/xidentity/xwork 已由 101/075/111 补齐，**只差这两个时间列** | 待核 |
| F-8 | `PP_C_TASK` 的 `TO_CHAR("xcreateTime", ...)` **类型不匹配** | `processplatform_assemble_surface/src/lib.rs:28742-28743`；该列是 TEXT（032 建的表），`to_char(text, unknown)` 函数不存在 → CI PG 日志实证。任何列存在性扫描器都看不见此类（类型层面） | 待核 |
| FX-1 | **main 当前 CI 红的直接原因**：`correlation_service_processing` 夹具 E23505 | `crates/correlation_service_processing/src/tests.rs:112`：`INSERT INTO auth_person ... ON CONFLICT (id) DO NOTHING` 只仲裁 id 索引，仲裁不了 `unique_id` 唯一索引；并发 seed 时 PG 先在 unique_id 上判冲突。**与 R4 修过的 `personal` crate 同族**（`upsert_fixture_person` 重试范式，见 10-24 文档 §14 R4），本 crate 漏修。R4 那次绿是时序运气，非缺陷不存在 | — |

> 同文件约 80 处**不带引号**的 `PP_E_APPLICATION` / `PP_E_APPLICATIONDICT` 会折叠成小写表名
> （`pp_e_application` 等，032 已建），无事；F-3 仅限这两处**带引号**的形态。
> MCP 工具族同样暴露 F-1 的两个 handler（`crates/mcp_server/src/generated_routes.rs:20321-20332`）。

### 2.3 扫描器盲区（F-1/F-2/F-3 能溜进来的根因，P1 收口）

| ID | 盲区 | 证据 |
|---|---|---|
| B-1 | FI-002 不抽 WHERE/FILTER 子句裸列（只抽限定列/INSERT 清单/UPDATE SET/单表 SELECT 裸列，`scripts/fi002_column_scan.py:348-401`） | F-1 的 `status = 'running'`、F-2 的 `category = $1` 完全不可见；`x_process_transition` 的 WHERE 列（gateway_instance/deleted_at）也没进基线（`docs/audits/artifacts/fi002_unknown_columns.tsv:13-16` 只记了 SELECT 清单 4 列）；**ORDER BY 列同类不可见**（F-5 的 sort_order）。**P1-1 已补抽**（含别名豁免，见 §5） |
| B-2 | 标识符大小写折叠（`analyze_sql` 去引号 + lower，`:329`） | `"PP_E_APPLICATION"` 被折成 `pp_e_application`（表存在、列也齐）⇒ 缺**关系**（视图）不可见（F-3）；**列级同病**：DDL 的 `"xworkCompleted"`（混合大小写）与 SQL 里不加引号的 `xworkCompleted` 折合后相同 ⇒ F-6 的引号缺失不可见。关系级已由 P1-2 守卫大小写敏感地把守；列级折叠保留（折叠是判据需要，全建模大小写敏感成本过高） |
| B-3 | `SYSTEM_SCHEMAS` 缺 `pg_stat_activity` | P1-1 补 WHERE 抽取后它会立刻被误报（`query_service_processing/src/lib.rs:215`），已同批入名单 |
| B-4 | SELECT 清单里**带引号的裸列**被静默跳过（裸列分支的 `re.match` 从引号起始即不匹配） | F-7 的 `"xcreateTime"` 因此没进 FI-002 违规集——引号列既不走 QUALIFIED_RE（无 `alias.` 形态）也不走裸列分支。**P1-1 已补认**（SELECT 清单与 UPDATE SET 两分支） |
| B-5 | `format!` 动态模板的 `{}` 插值内容不可见 | F-6 的 `ref_col` 常量（`"xworkCompleted"` 裸串）经模板插值，静态扫描无从判定；**P1-3 已用「helper 家族首字面量必须引号」的 targeted 守卫把守**（通用 format! 语义分析不可行，边界记在守卫头部） |
| B-6 | `SELECT DISTINCT` 的**首列**被 `distinct` 关键字吞掉 | P0-6 反向注入时实证：`SELECT DISTINCT process_key_typo, ...` 的首列永不被抽取（裸列分支 `re.match` 命中 "distinct" 即按关键字跳过）。**已修（2026-10-26 深夜，本批）**：`analyze_sql` 抽取后对 SELECT 列表体剥掉 `DISTINCT`/`ALL` 前缀（`strip_select_modifiers`，正则惰性量词改不动、改在抽取后处理），两向验证：基线态 exit 0；注入 `process_key_typo` exit 1 并点名 `lib.rs:63`（已还原） |
| B-7 | 视图投影列不进宇宙 | P1-1 分判出的 `pp_e_application.xkey/xterminal` 即此类：112 建的 `"PP_E_APPLICATION"` 视图投影列，经大小写折叠归属到表。按基线登记（理由见 tsv），表级检查不受影响；视图存在性由 P1-2 把守。大小写敏感的关系建模（把 B-2/B-7 一起根治）成本过高，留待 |

### 2.4 实施期新发现（2026-10-26 深夜，P0/P1 执行中登记）

| 发现 | 详情 | 处置 |
|---|---|---|
| **F-7 家族比 F-7 大** | F-7 只修了 `lib.rs:2916`；P1-1 增强扫描当次抓出同型三处：`8764`（keylock_lock SELECT）、`8797`（mockputtopost UPDATE SET）、`8809`（回读 SELECT）——「修一处同型」靠肉眼必漏，尺子补盲后当场现形 | 同批修（别名化 / SET 改小写列） |
| **112 首版漏写 F-2** | WHERE 盲补前 `x_work.category` 隐身，基线 7→0 的「全绿」是假绿；P1-1 首跑当即点名 | 补进 112（宇宙 5148→5149） |
| **F-6 与 quoted_key_guard 相撞** | F-6 的引号常量写成转义形态会撞 `tests/quoted_key_guard.rs` 的转义引号键禁则（那防 row.get/payload 的 JSON 键缺陷，与 SQL 标识符引用同形不同义） | 改 raw string `r#""x""#`——运行期值相同、无 backslash 字形、守卫天然不误伤；契约注释写明缘由。Python 模拟全仓 608 文件 0 违例 |
| **本机 Rust 编译不可用** | GNU 工具链缺 gcc/dlltool（ring/parking_lot_core 建不了），无 MSVC 工具链——10-11/12 能编是当时有 MinGW，现已不在 | fail-loud：本机只做 rustfmt（语法+格式，CI 同版本 1.98.1）+ Python 端口算法实跑 + 人工类型/borrow 复审；编译与测试执行全部交 CI。**实际发生两轮反馈循环**（见 §5 末）：① 3 处 `&c` 撞 needless_borrow（client 形参本就是 `&Client`，`&c` 即 `&&Client`）——本机审计脚本初版还找错了绑定（找到隔壁函数的 `let c =`），二审改为「找所在函数形参」才定位准；② `Path::file_name()` 返回 `Option<&OsStr>` 被当成 `DirEntry::file_name()` 的 owned 用（E0599）。两轮各约 10 分钟 CI，终态全绿 |
| **沙箱代理间歇性阻断 GitHub 写操作** | 第 5 次推送起 SSH 被 198.18.x.x 关闭、HTTPS schannel/openssl 握手失败、gh POST 全 EOF（GET 正常）；重试后 SSH 恢复 | 重试成功；备选 GitHub Git Database API 当时同样被断（POST EOF），未采用 |

---

## 3. 执行计划

### P0 — migration 112 一批收口（沿用 R8/111 范式：幂等 + TO_REGCLASS 守卫 + 出处行号注释 + rollback）

| 项 | 改法（草案 SQL，实施时按 111 头部范式写全证据注释） | 风险 | 验收 |
|---|---|---|---|
| P0-1 | `ALTER TABLE "x_process_bam" ADD COLUMN IF NOT EXISTS "status" TEXT`（代码绑定类型：与 `'running'/'completed'/'expired'` 字面量比较 → 文本系，同 R8 判据） | 低。live 库若已有 Java 时代 status 列，IF NOT EXISTS 是 no-op；fresh 库建成空列 ⇒ 两个端点从 500 变 200（计数为 0，诚实） | `cargo check -p oa4rust` clean；CI behavior-compare（fresh PG）冒烟不回归 |
| P0-2 | `ALTER TABLE "x_work" ADD COLUMN IF NOT EXISTS "category" TEXT` | 低，同上 | 同上 |
| P0-3 | `"PP_E_APPLICATION"` 视图**不存在才建**（`pg_class` relname + relkind='v' 守卫；勿用 CREATE OR REPLACE——live 库可能已手工建过、定义未知，replace 有覆盖风险）。定义照 archive/089：投影混合大小写列 + `xid AS xkey, xid AS xterminal`，源表 `"pp_e_application"` | 低-中（触碰关系拓扑；已核 032 DDL 列名与视图逐列一致） | 同 P0-1；另加路由级判别：挂载后 fresh DB 返回 200/空列表而非 500 |
| P0-4 | 4 张遗留表幂等建表（收尾 FI002-LEGACY-TABLES）：3× `pp_c_touch_*` 单列 `xupdatetime TIMESTAMP`（**不照抄 archive/089 的 `id + xupdateTime` 旧驼峰形状**——现 handler 已改小写）；`x_process_transition`（id PK / work_id / to_activity / condition / gateway_instance / deleted_at + gateway_instance 索引） | 中（建表形状决策；仅按 handler 实际引用列最小化，索引必要性附注） | 4 个端点 fresh DB 从 500 变 200；`fi002_column_scan` 基线 7→0 |
| P0-5 | 修 F-4 假 500 测试：改为 `is_db_available()` 门 + 行为断言（200 + total/running/completed/expired 四键）；保留一条无库也跑的 router 构建测试 | 低 | `cargo test -p process_bam` 本机过（无库臂）；CI 有库臂过 |
| P0-6 | FI-002 基线收敛：P0-1~4 修完后 `--update-baseline` 重写（7→0），并跑两方向验证（基线态 exit 0；临时注入一处假引用 exit 1） | 低 | 两方向实测记录进 §5 |
| P0-7 | **修 FX-1（main 当前红）**：`correlation_service_processing::seed_admin_session` 照 R4 范式修（重试 + 每次 DELETE 清残留，3 次仍失败 fail-loud）；同批全仓扫同型——所有 `INSERT INTO auth_person ... ON CONFLICT (id)` 夹具逐个登记（修 / 照 R4 修 / 说明为何安全） | 低-中（并发时序类，修完需 CI 连绿佐证；不排除偶发复红，复红再查） | 本机 `cargo test -p correlation_service_processing`（无库臂不回归）；CI `unit-tests` 转绿 |
| P0-8 | F-5~F-8 一批（均为 CI PG 日志实证的 fresh-DB 500）：F-5 `x_program_field ADD COLUMN sort_order`（代码绑定：仅用于 ORDER BY，取 INTEGER——按 Java `sort_order` 命名惯例，实施时若 handler 也读该值再核）；F-6 把 `ref_col` 常量改带引号（`"\"xworkCompleted\""`，插值点集中一处改）；F-7 SQL 层别名化（`create_time AS "xcreateTime", update_time AS "xupdateTime"`，handler 零改动）；F-8 `TO_CHAR` 改文本安全等价形式（候选：`LEFT("xcreateTime", 10) = $1`——依赖存量文本为 ISO 前缀，**实施时先用 live 数据样本确认格式再定**，不可则 `"xcreateTime"::timestamp` 铸造并接受不可解析值 500 的现状登记） | 中（F-8 涉语义判断；F-6 触 format! 模板） | CI PG 日志中这 5 类 ERROR 消失；相关端点 fresh DB 200 |

### P1 — 扫描器补盲（让 P0 这类不再溜进来）

| 项 | 改法 | 风险 | 验收 |
|---|---|---|---|
| P1-1 | `analyze_sql` 增抽 WHERE/FILTER/ORDER BY 裸列（单表才归属；三类豁免：`$N::type` cast、系统 schema 子查询、函数调用），SELECT 清单补认**带引号裸列**（B-4），`SYSTEM_SCHEMAS` 补 `pg_stat_activity`（B-3）。**先全仓跑一遍、逐条人工分判新增对再并基线**（防把误报写成基线） | 中（提取逻辑是 FI-002 棘轮的根基，宁严勿松） | 全仓新增对逐条有分判记录；基线只增不减且每条有理由；CI `fi002-column-scan` 不假红 |
| P1-2 | 新增静态检查：代码里**带引号的大写标识符**（FROM/JOIN/UPDATE/INTO 后）必须在 migrations 里有对应 CREATE（视图/表都覆盖） | 低-中（新尺子；按既有守卫风格入库 + 反向注入自证） | 反向注入（删掉 112 的视图创建）→ 红；还原 → 绿 |
| P1-3 | `format!` SQL 模板的插值常量守卫：凡插进 SQL 的列名/表名常量必须带 SQL 引号或来自白名单（B-5） | 低-中 | 反向注入（把白名单常量改成不带引号形态）→ 红 |

### P2 — 台账与工程卫生（便宜，保持账目完整）

| 项 | 内容 |
|---|---|
| P2-1 | 本文档即执行与验证台账；`migrations/archive/` 加一行「089 因 views over nonexistent tables 被禁用，勿重启用；PP_E_APPLICATION 视图的修正版在 112」说明（防下一个人重启用这颗陷阱） |
| P2-2 | 用户项（见 §4） |

---

## 4. 明确不做 / 边界 / 需用户拍板

| 项 | 理由 |
|---|---|
| 不改 handler 的 `status` 过滤语义（如改查 running/completed/expired 计数器列） | 无 live DB 证据，猜 Java 时代语义的风险 > 收益；补列是保守解 |
| 不动约 80 处不带引号 `PP_E_*` 引用 | 折叠正确、指向已建表，无事 |
| 不重启用 archive/089 | 它同时含「views over nonexistent uppercase tables」的致命伤；只取其 PP_E_APPLICATION 视图定义的**修正版**进 112 |
| **需用户：追加 6 项 required context 进 `laiyipaoCIRuleset`（WF-REQUIRED）** | **已落地（2026-10-26，用户操作）**：ruleset `24845982` 的 `required_status_checks` 由 3→9，逐字命中 §479 权威清单（quality / fi002-column-scan / openapi-guard / contract-gate / test / Dependency vulnerability gate）；明确排除项（integration-tests/unit-tests/behavior-compare）未误加。fail-loud 两点：① `bypass_mode: "always"`——admin 可无限制 bypass，required 只对 PR 流程与协作成员生效，不拦你本人直推；② 本轮 P1-2/P1-3 两把新守卫随 `quality` 一起 block 合并，其偶发假阳性（B-6/B-7 类）也会卡合并——守卫是"宁可红也不放"设计，可接受 |
| **需用户：WF-REQUIRED** | GitHub ruleset 追加 6 个 required context（精确清单见 10-24 文档 §14 R4），需页面操作 |
| 111 的 41 列类型为「代码绑定」推定 | **已复核（2026-10-26，用户启动 Docker 后）**：`lyp-pg`（postgres:17）复用为 oa4rust 测试库（建 `o2server` 角色 + `oa4rust` 库），001→112 全应用。information_schema 抽样：112 的 6 列全在位（status TEXT / sort_order INTEGER / category TEXT / gateway_instance TEXT / xupdatetime TIMESTAMP + x_process_transition 建表），111 抽样 5 列（person_id / xidentity / checkin_time / vip / xcontent）类型符代码绑定。`PP_E_APPLICATION` 视图已建。rollback 验证：112 rollback 干净回退（视图 + 4 表全删），重 apply 幂等。**分叉实证**：fresh 库 `auth_person_identity.person_id` 为 TEXT（111 补），live 若 VARCHAR 则形状不同——但 VARCHAR/TEXT 对 String 绑定等价，无行为影响。库已恢复全量 fresh 终态 |

---

## 5. 验证台账（实施中填写）

> 口径不变：**已完成 = 有改动 + 有构建/测试证据**；跳过/待外部条件逐条写明（AGENTS #11）。

| 项 | 状态 | 证据 | commit |
|---|---|---|---|
| P0-1 x_process_bam.status | **已完成** | 112 幂等 ALTER（代码绑定 TEXT）；fi002 宇宙 5137→5148 列；行为待 CI 真库 | `e09753e3b` |
| P0-2 x_work.category | **已完成（经波折）** | 112 首版**漏写**此 ALTER——WHERE 盲补前它隐身，P1-1 增强扫描首跑当即抓出，同批补进 112（5148→5149 列）。「尺子抓出尺子建造者的遗漏」的实证 | `2148b7f2b`（补列）；`e09753e3b`（112 主体） |
| P0-3 PP_E_APPLICATION 视图 | **已完成** | pg_class relname+relkind 守卫、不存在才建（刻意不用 OR REPLACE）；定义照 archive/089 且与 032 DDL 逐列核对 | `e09753e3b` |
| P0-4 遗留表 4 张 | **已完成** | 3× pp_c_touch_*（xupdatetime，不照抄 089 旧驼峰）+ x_process_transition（6 列 + gateway_instance 索引，列全部来自 handler）；FI002-LEGACY-TABLES 收尾 | `e09753e3b` |
| P0-5 process_bam 假 500 测试 | **已完成** | 三用例改 is_db_available 门 + 行为断言（200 + 响应键）；保留 router 构建与 ActionResult 纯测试 | `88883a262` |
| P0-6 FI-002 基线收敛 | **已完成** | 7→0（`e09753e3b`）；P1-1 后余 2 对视图投影列登记（`2148b7f2b`）。两方向验证：基线态 exit 0；反向注入 exit 1 并点名（SELECT 列打错 → lib.rs:63；WHERE 列打错 → lib.rs:17，均已还原） | `e09753e3b`、`2148b7f2b` |
| P0-7 FX-1 夹具 E23505（main 当前红） | **已完成** | shared::testing::upsert_fixture_person（DELETE 残留 + DO UPDATE + 23505 重试×3 fail-loud，判据与 R4 personal 版逐字一致）；18 处四列形态迁移 + bbs 两处七列形态内联同范式助手；CI unit-tests 转绿待确认 | `650833674` |
| P0-8 F-5~F-8（CI 实证四缺陷） | **已完成** | F-5 补 sort_order INTEGER（`e09753e3b`）；F-6 十一处 ref_col/work_col 常量引号化（先转义形态，后改 raw string `r#""x""#` 绕开 quoted_key_guard 误伤——见 §2.4）；F-7 别名化 ×4（2916 + **8764/8797/8809 三处同型**，后者是 P1-1 扫描新抓出的）；F-8 TO_CHAR(text)→LEFT(col,10)（ISO 前缀假设有全族 ORDER BY 先例，注释明示） | `e09753e3b`、`88883a262`、`2148b7f2b`、`7014a465b` |
| P1-1 WHERE/ORDER BY/引号列抽取 | **已完成** | analyze_sql 增抽 WHERE/FILTER 裸列（剥子查询、排 cast/函数/关键字）+ ORDER BY/GROUP BY 裸列 + SELECT/SET 引号列（B-4）+ pg_stat_activity 入 SYSTEM_SCHEMAS + SELECT 别名豁免。首跑 9 对逐条分判：4 假阳性（别名）/2 真缺陷/2 视图投影列/1 自身遗漏 | `2148b7f2b` |
| P1-2 引号大写标识符检查 | **已完成** | tests/quoted_relation_guard.rs：大小写敏感核对引号大写关系 ↔ 生效迁移 CREATE；反向注入 5 形态 + 负对照 4 项；算法经 Python 端口对 612 真实文件实跑 0 违例 | `7014a465b` |
| P1-3 format! 插值常量守卫 | **已完成** | tests/sql_ident_constant_guard.rs：五个 SQL 插值 helper 的首字面量（列参数）必须小写或已引号；反向注入 5 形态 + 负对照；真树 612 文件 0 违例 | `7014a465b` |
| P2-1 archive/089 警示 | **已完成** | migrations/archive/README.md：089 冷藏原因 + 「勿重启用」+ 修正版在 112 + 守卫把守 | 本文档提交 |

### 5.1 CI 反馈循环与终态（2026-10-26，gh 实跑）

| 提交 | oa4rust CI | 说明 |
|---|---|---|
| `650833674`（P0-7） | 绿 | FX-1 修复后 unit-tests 转绿（R8 时代那次红的直接修复） |
| `e09753e3b`（112） | 绿 | 112 fresh PG 应用 + FI-002 棘轮 job 绿 |
| `88883a262`（P0-5/P0-8） | 绿 | process_bam 行为测试 + F-6/F-7/F-8 SQL 修 |
| `2148b7f2b`（P1-1） | **红** | quality：message:206 needless_borrow（P0-7 迁移的 `&c` 家族） |
| `7014a465b`（P1-2/P1-3） | **红** | 同一错误（修复未含在该提交） |
| `4e90ed1f8`（台账） | **红** | quality：quoted_relation_guard 的 `Path::file_name()` Option（E0599） |
| `69be59402`（clippy 反馈修） | — | 3 处 needless borrow + drop 移除 |
| `84d068905`（file_name 修） | **全绿** | 8/8 job（quality/unit-tests/integration/behavior-compare/fi002/openapi/behavior-gate）；两把新守卫首次真正编译执行并通过 |

**main  transient red 时长约 50 分钟**（`2148b7f2b`→`84d068905` 之间三个提交的 oa4rust CI 红），
根因全是「本机无编译器 ⇒ CI 首编」的反馈类错误，非逻辑回归。其余 workflow
（laiyipao/three-ends/auto-checkin/supply-chain/warnings-gate）全程绿。

**外部服务遗留（2026-10-26，非代码缺陷）**：

- **PR #102 合并**（`273595c`）：Dependabot 自动提的依赖安全修复 PR
  （body-parser 1.20.8→1.20.9 修 CVE-2026-111181；undici 8.10.2→8.12.3 修 4 高危 CVE + 1 中危）。
  PR 自身 CI 在旧 base 上红（`augmentor/web` 的 `model_generate_seconds` 类型缺失），
  但 main 已在 base 之后独立修好该类型（`api.ts:553` + `System.test.tsx:111/128`），
  合并后 `AI platform CI` 3/3 全绿（含之前红的 `Docker smoke test` / `Frontend build`）——
  **那个"遗留"已自然消失，非真缺陷**。
- **Dependabot Updates workflow 红**（run `40295641134` @ `273595c`）：
  Dependabot 更新器在 08:56 处理 `vite` 时报 `unknown_error`，整个 updater 容器 exit 1。
  **这是 GitHub 托管的 `dynamic/dependabot/dependabot-updates` workflow，本地无法 rerun
  （`gh run rerun` 403 拒绝）**。属服务侧行为：① 大概率瞬态，下轮 scheduled 自愈；
  ② 若持续，需读 GitHub 端 Dependabot 日志（write access）+ 改 vite 版本约束
  （依赖策略决策，非单方可决）。**登记观察，不改代码**。

**环境限制（fail-loud，三重）**：① 本机无 PostgreSQL（原生仅客户端、Docker 未运行），DB 面全部标注
「待 CI 真库」；② 本机 **Rust 编译不可用**——GNU 工具链缺 gcc/dlltool、无 MSVC 工具链
（ring/parking_lot_core 原生构建必败），故全部 Rust 改动的验证口径是「rustfmt（CI 同版本 1.98.1）
语法+格式 + Python 端口算法实跑 + 人工类型/borrow 复审」，编译与测试执行交 CI；③ 沙箱代理
间歇性阻断 GitHub 写操作（SSH 被关 / HTTPS 握手败 / gh POST EOF，GET 正常），推送重试后恢复。
`gh` 已于本轮登录（keyring）；FI-002 扫描器为纯 stdlib，本机用 Python 3.12 实跑
（前序文档记的 `C:/Python314/python.exe` 本机不存在，口径差异在此登记）。

### 5.2 有库轮次（2026-10-26 深夜，用户启动 Docker 后实跑）

**容器复用**：用户启动的 `lyp-pg`（postgres:17，原为 laiyipao 库）复用为 oa4rust 测试库——
建 `o2server` 角色（LOGIN SUPERUSER）+ `oa4rust` 库，001→112 全应用（114 个迁移，
`037`/`051` 需先手动建 `schema_migrations` 追踪表——它是迁移运行器 DDL 建的，不在目录里）。
`shared::testing::test_pool()` 默认连接串 `postgres://o2server:password@localhost:5432/oa4rust`
对容器验证可连。**一个容器两副身份**（laiyipao 继续用它的 `laiyipao` 库，oa4rust 测试用新 `oa4rust` 库）。

**验证结果（真库直跑，非 CI 推定）**：

| 项 | 方式 | 结果 |
|---|---|---|
| 112 的 6 列 | `information_schema.columns` | 全在位、类型符代码绑定（status TEXT / sort_order INTEGER / category TEXT / gateway_instance TEXT / xupdatetime TIMESTAMP） |
| 111 抽样 5 列 | `information_schema.columns` | 类型符绑定；`person_id` 在 fresh 库是 TEXT（111 补），live 若 VARCHAR 则形状分叉——VARCHAR/TEXT 对 String 绑定等价，**无行为影响** |
| `PP_E_APPLICATION` 视图 | `pg_views` | 已建 |
| 112 rollback | 应用→rollback→重 apply | 干净回退（视图 + 4 表全删）、重 apply 幂等 |
| F-1/F-2/F-5 SQL | 真库直跑 | 通过（fresh 空表 count=0，诚实） |
| F-3 视图查询 | 真库直跑 | 通过（混合大小写列需引号——F-6 类实证） |
| F-7 keylock 别名 | 真库直跑 | 通过 |
| **F-8 旧形态** | `TO_CHAR("xcreateTime",'YYYY-MM-DD')` 于 TEXT 列 | **实证 `to_char(text, unknown) does not exist`——坐实原 handler 的 500 是真实缺陷，非推定** |
| F-8 新形态 | `LEFT("xcreateTime",10)` | 通过 |

**本轮最大增量**：F-8 从「待 live 数据样本确认 `pp_c_task."xcreateTime"` 文本格式」变成
**已实证**——旧形态 `TO_CHAR(text)` 在真库上确报 `function to_char(text, unknown) does not exist`，
新形态 `LEFT(col,10)` 通过，修法正确且无运行时风险（脏值不触发 500，因是纯文本前缀匹配）。

库已恢复全量 fresh 终态（112 重 apply 后），`test_pool()` 可直连，后续第四轮可对着真库跑行为测试。

---

## 6. 取证方法与复现口径

| 取证 | 命令 / 位置 |
|---|---|
| FI-002 棘轮现值 | `cd oa4rust/backend && python scripts/fi002_column_scan.py`（基线态 exit 0，7 对均在基线内） |
| WHERE 盲区探针 | `python Temp/opencode/fi002_where_probe.py`（临时目录，未入库；复现需先建同款 universe 导入） |
| CI 核对 | `gh run list -c 835d55cfbdc97606cff5d910ca62ea16fbc87c03 --json ...`；失败 run `40262029237`，`gh run view 40262029237 --log-failed` |
| 视图从未被应用 | `git show 27335205f --stat` + `shared/src/migrate.rs:84`（排除 archive/） |
| 前端调用点 | `grep` `frontend/apps/desktop/src/views/{ProcessWork,ProcessManagerApp,BamApp}.vue` 对应行号 |
