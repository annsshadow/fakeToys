# oa4rust 1000 轮实质优化 · Track-B 台账

> 本文件是 oa4rust 子项目「千轮实质优化」Track-B 线的唯一进度台账。
> Track-A 线（laiyipao + oa4rust 主线）台账在 `docs/plans/optimization-1000-oa4rust-laiyipao.md`（另一会话所有，本线不写）。
>
> 隔离约定：本线在独立 worktree `D:\WORKSPACE\fakeToys-oa4rust-oa4rust`（分支 `opt1000/oa4rust`）工作，
> 只改 `oa4rust/` 下内容；构建产物落 `CARGO_TARGET_DIR=E:\cargo-target\oa4rust-par`（仅命令环境变量，不入库）；
> 临时库一律 `oa_par_` 前缀，用完 DROP。
>
> 轮次编号 `O1、O2…` 与 Track-A 的 `R` 编号区分。
> Track-A 已完成的 oa4rust 轮次（不重做）：R4 优雅停机、R5 全局超时、R6 池超时 503、R7 结构化访问日志、
> R11 panic 收敛、R12 池取连失败 503、R16 /ready 真查库、R17 迁移错误可定位、R24 body 上限、
> R25 DATABASE_URL 配置错误可定位、R26 statement_timeout、R31 /mcp 请求层防护。

## 轮次标准

- 一轮 = 实质性功能或性能优化；编译/lint/纯重构/纯测试/纯文档均不算轮。
- 每轮先探针证明真实后果（改前证据），修复后证明「改后好 + 原功能没坏」（三段证据）。
- 探针打不出真实后果 → 判定「不开轮」，记录结论换题。

### 合并 main 后的强制流程（2026-11-04 三次事故固化）

三次事故同源：**合并结果与生成物/路由表/基线不一致，而流程里没有任何一步会主动发现**。

1. **行注释被当成表达式内容**：`.route(` 首参是多行表达式时，生成器把首参前一行的行注释并入了表达式，2 条路由在生成物里不可达 → CI 暴露、线上 404（O16-0a）。
2. **自动合并静默丢路由**：合并 main 时两侧都改过路由表，文本合并无冲突，但语义已错——105 条 bbs 路由被静默合并掉。
3. **基线被回退**：`coverage-baseline.json` 在合并中被回退到旧值，门禁因此「变绿」而非报警。

规矩（任何 merge main 之后、push 之前强制执行，无例外）：

1. 重跑生成器（`gen_mcp_tools.py` 等），`git diff` 必须为空或仅含本次真实改动；
2. 跑生成物门禁（`check_generated_artifacts_up_to_date.py`、`check_openapi_up_to_date.py`）；
3. 核对三份路由产物条数，与台账基线逐项对齐（`generated_routes.rs` 4874 方法 / 两份 fixture 4170 路径）；条数下降即视为合并丢失，查清再 push；
4. 基线类文件（`coverage-baseline.json`、`idor_handler500_baseline.txt`、limit-scan 基线）逐文件确认未被合并回退（值比对，不看「门禁是否绿」）。

## 逐轮台账

| 轮次 | 主题 | 状态 | 缺口/改法/证据摘要 | commit |
|------|------|------|--------------------|--------|
| O1 | 迁移并发安全：pg advisory lock 串行化多实例迁移 | done | 缺口：多实例并发启动跑迁移无串行化；改前探针实测两个运行者并发跑同一空库，`CREATE TABLE IF NOT EXISTS schema_migrations` 撞 `pg_type_typname_nsp_index`（SQLSTATE 23505），后到实例启动失败。改法：`run_migrations` 前取会话级 advisory lock（固定 key），逐版本重查自然跳过，成功/失败路径都显式 unlock（防连接回池残留锁），断连由 PG 自动释放。证据：改前探针 runner A Err(23505)/B Ok；改后双方 Ok、3 条迁移恰好应用一次、种子恰好一行；回归 shared 363/363（`oa4rust_test` 库）。scratch `oa_par_mig_o1` 已 DROP | 068371db6 |
| O2 | 取连失败语义统一：`shared::db::acquire` 全仓收口（R12 分类覆盖全部请求路径） | done | 缺口：R12 的池取连失败 503 分类只挂在 `ControlPool` trait 上，全仓约 2930 处 handler 仍手写 `pool.get().await.map_err(|_| AppError::Internal)` —— DB 停机/池满/重启窗口一律 500，客户端不敢重试、监控误报代码缺陷（改前证据：ai_assemble_control 等 crate 死池测试断言 500，实测连接被拒返回 `internal server error`）。改法：`shared::db::acquire(&Pool)` 公开入口复用 `classify_acquire_error`（Timeout/Backend/Closed→503，配置类→500），全仓机械替换 2930 处 + 116 文件。证据：死池请求由 500 → 503（`service unavailable: ...`）；`Extension<Session>` 缺失的 500 语义不变。测试同步：~250 处死池断言由 500 翻 503；混合路由循环（同表内既有必需会话扩展的 500 又有池失败的 503）改为「500|503 均证明路由可达」家族断言（保持≠404 的可达性语义）；`crates/shared/crud.rs` fail-loud 契约改锚 Unavailable。验证：全 workspace `cargo test --no-fail-fast` 除 4 个 main 上既有红守卫（quoted_key_guard/quoted_relation_guard/test_module_registration_guard/arity_probe，已在 origin/main 原态复现确认与本改动无关）外全绿；`cargo clippy --workspace --all-targets --keep-going` 零告警 | 16ba8a7e6（rebase 前原始 hash；当前分支上为 f0bcecdd4，后改走 merge 不再 rebase） |
| O3 | quoted_key_guard 误报修复 + 守卫破坏实验自检 | done | 缺口：守卫常红于 main.rs:387 的合法断言 `contains("\"paths\"")`（JSON 内容检查被误判为转义引号键）。改法：断言改 raw string `r#""paths""#`（语义不变）；守卫扫描逻辑抽为 `find_violation_in_line` 并加破坏实验 `guard_still_catches_real_violations`：三类历史真实缺陷形态必须捕获、raw string/quote_ident/普通字符串必须放行。证据：改前 no_escaped_quote_keys 红（报 main.rs:387），改后守卫 2/2 绿、openapi_json 缓存测试绿 | ec279a22c |
| O4 | quoted_relation_guard 自身 char-boundary panic 修复 | done | 缺口（守卫自身缺陷）：迁移含「create + 中文注释」时 parse_created_name 用 `text[i..i+10]` 按字节切 str，撞上「契」的字符边界直接 panic——守卫先崩、真违例报不出。改法：or replace / table|view / if not exists 三处关键字比较改字节级 `bytes[i..i+n]`。破坏实验：新增 `parse_created_name_survives_multibyte_and_still_parses`（多字节不 panic + OR REPLACE/IF NOT EXISTS/大小写/索引/小写五判据 + 全链路混排文件收齐）；既有反向注入自证保持绿。验证：守卫 3/3 绿（改前主用例 panic） | 95b9365c8 |
| O5 | openapi 测试模块挂进模块树（test_module_registration_guard 红） | done | 缺口：crates/openapi/src/tests.rs 存在但 lib.rs 是生成物、无 mod 声明——7 条 OpenAPI 文档结构断言从未被执行（文件在 ≠ 测试在跑）。改法：生成器 gen_openapi_paths.py 固定输出 `#[cfg(test)] mod tests;`，重生成 lib.rs（diff 仅 +2 行、4734 path items 不变）。证据：守卫转绿；`cargo test -p openapi` 7/7 通过（此前从未编译执行）。守卫抓真问题能力 = 本次实测（它报出的就是真实遗漏） | 61bdd53cf |
| O6 | 路由元数缺陷修复 ×3（arity_probe 常红收口） | done | 缺口（arity 探针实测 2 条 + 评审发现 1 条隐形）：① `PUT /api/calendar_assemble_control/event` 死注册（模式无 {id}，handler 要 Path<String>，恒定 500 Wrong number of path arguments）→ 删 PUT 臂（正确入口 /event/update/all/{id} 已在）；② `POST .../upload/folder/callback/callback/{folderId}` 1 参模式挂 2 元组提取器 → 新增单参薄包装委托同实现；③ `attachment_id_update_callback_callback` 只注册在单参路径却提取 2 元组（因必需会话扩展先失败而对探针隐形）→ 收成 Path<String>（callback 未使用）。生成物：gen_mcp_tools.py 重生成注册表（含 ±80 条存量漂移同步，无路由语义改动）。证据：arity_probe 转绿；calendar/file/mcp 测试全绿；clippy 零告警。遗留（台账候选）：file upload scale/scale 两条 legacy 路径元数不符且 reference/scale 无来源，需先核实 o2server 契约 | a7965fa49 |
| O7 | 查询执行期错误分类（SQLSTATE 锚点 + From 自动分类） | done | 缺口：查询/事务/提交失败的 tokio-postgres 错误全仓 237 处 `.map_err(|_| AppError::Internal)` 吞成 500——statement_timeout（57014）、断连（08xxx/被 kill）、停机窗口（57P0x）这类可重试故障客户端无法识别。改法：`shared::db::classify_query_error`（08*/57014/53300/53200/57P01-03/is_closed/io → 503 Unavailable；23505/42P01/语法错/ToSql/FromSql → 500）+ `From<tokio_postgres::Error> for AppError`，`.map_err(|_| Internal)?` → `?` 自动分类。清扫统计：294 候选 → 237 纳入分类、57 回退（sea-orm DbErr 56 + reqwest 1，未实现其分类故保留原状）、另 1 处 await 无 `?`（oidc res.json）与 3898 处非 await（serde/fs 同步调用）均非 DB 错误，明确不动。证据（真触发，非构造错误值）：真 statement_timeout=1s + pg_sleep(3) 实测 57014：改前 500 → 改后 503；真 pg_terminate_backend 杀连接后查询：→ 503；23505/42P01 保持 500；From 实现与分类器同口径。验证：4 条锚点测试 + workspace 全量绿 + clippy 零告警 | 4b08457c4 |
| O8 | sea-orm DbErr 分类（消灭最大语义不一致面：56 处保留点） | done | 锚点实验（源码+真触发双证）：`DbErr::Conn/Exec/Query(RuntimeErr::SqlxError)` 公开携带底层 sqlx::Error，SQLSTATE 可稳定取（`sqlx::Error::Database(db).code()`）；池取连失败被 sea-orm 折叠为类型化的 `DbErr::ConnectionAcquire(ConnAcquireErr::{Timeout, ConnectionClosed})`（非字符串匹配）。改法：`classify_orm_error` 与 O7 共用 `sqlstate_is_retryable` 判据表；`From<DbErr> for AppError`；56 处回退点全部转正为 `?` 自动分类（另回退 2 处误转：ai reqwest json、surface storage String 错误）。证据（全真触发）：sea-orm 事务内 SET LOCAL statement_timeout=1s + pg_sleep(3) 实测 57014 → 锚点断言成立 → 503；未定义表 42P01 → 500；池 max=1 + 事务占连接 → ConnectionAcquire::Timeout → 503；文本列提取 i32 → TryGetErr → 500。回归：organization_core_entity 两处死池断言 500→503（io 断连新语义）；workspace 全量绿、clippy 零告警 | 82a76f4ef |
| O9 | IDOR 穷举矩阵（活注册表 × 7 身份）+ 读面归属闸家族修复 + 接线缺陷族修复 | done | 矩阵（`tests/idor_matrix.rs` 新增）：扫 crates/*/src 非测试代码 `.route(` 字面量（修复多行注册漏扫后 4758 注册/3706 候选，候选数下限断言防注册表静默漂移）；7 身份格子（anon/forged/expired/disabled/non-owner/cross-org/owner 参照列）；判定写死——非豁免非公共面 anon/forged/expired 必 401、disabled 必 401/403、非所有者 403/404/400/422 通过、**canary 泄露与「owner 正常而非所有者 500」= 越权缺陷**、「owner 同 500」入 handler 级棘轮桶（基线 161 只减不增）；写后完整性复核夹具不被改动；探针全程跑专用 scratch 库 `oa_par_idor`（每轮重建+全量迁移+DROP——此前 `{}` 体探针在共享库 25 表写 280+ 垃圾行，把下游 arity_probe 打红，已清并隔离）。破坏实验：摘 `attachment2_id_binary_base64` 归属闸 → 矩阵精确红 12 格（3 注册路径×2 身份+2 委托变体×2）→ 还原转绿。真缺陷修复（改前 canary 实证/500、改后绿）：① FILE_FILE 读面 8 handler + 6 list 端点归属闸（require_owner 本人/admin，与写侧 attachment_id_update 既有约定同口径；list 收窄为本人+admin——引用共享场景需实体级权限模型，当前不存在，fail-loud 方向）；② calendar 族 8 处（calendar_get 两实现/event_get/event_rfc/detail/两个 list filter/manager list：公开日历放行、私有 403，事件读面=公开日历∨事件属主∨日历属主∨管理员）；③ 禁用用户（locked/deleted_at）在途会话两臂失效（person_status_cache 30s TTL，幽灵人放行兼容存量夹具，内存臂命中即逐出）；④ program_center warnlog 族参数个数不匹配（count 传入但 SQL 不引用，恒 500）修复为 LIMIT 参数化夹紧 [1,100]；三族 date 变体（warnlog/prompterrorlog/unexpectederrorlog）String 直传 `$n::date` 序列化必炸 → NaiveDate 解析（非法日期 500→400）。接线缺陷族（生产恒定 500，矩阵定位、非 IDOR 面）：empower router 的 with_state 错当 Extension（全族 14+ 端点）；component router 的 Extension(pool) 层被注释；query_express DatabaseConnection 从未注入（§五 遗留 30）；cms_express/cms_core_entity 双注入；calendar detail 路由注册于 layer 之后（axum Router::layer 不覆盖后注册路由）。decode panic 族：bbs section sort BIGINT→i64、hotpic application NULL 安全、get_calendar_detail 全列 Option 化。parity 生成器补 DatabaseConnection 参数形态。遗留：legacy 上传 scale/scale 两条 arity 基线路由显式豁免（reference/scale 无来源，无法确认 o2server 语义，不开轮）。验证：矩阵 run15 绿（0 越权缺陷）、破坏实验红绿对照、4 守卫绿、13 触及 crate 测试全绿、clippy 全 workspace 零告警 | 4c317dfdf |
| O10 | handler 级 500 棘轮桶清零（161→0）+ 基线棘轮守卫 + 禁用会话 TTL 边界钉 | done | **分族（先分族再动手）**：以 PG 服务端日志（docker logs 抓 SQLSTATE/语句）+ 探针直调实证替代逐个静态读——161 条归 4 族。**族① sea-orm 实体↔列类型不匹配**（bbs forum/section/subject 3 条 + mind×3 raw-SQL 版 + meeting 族）：order_number BIGINT↔i32、create_time TEXT↔Option<DateTime>、file_version BIGINT↔i32、`$n::timestamp`/`INTERVAL*$n` 把参数定死 timestamp/float8 而 String/i64 无法序列化——全部 WrongType **客户端侧**炸（PG 无日志，探针 `Error{kind:ToSql,cause:WrongType}` 实证）。修法=对齐实体/参数类型不改列（bbs 三实体 i64/String+`now_text()`、x_application.deleted_at String、mind i64、meeting 加 `parse_ts_param`/`month_range`/`day_range`（非法日期 400 指认字段）+ INTERVAL 绑 f64）。**族② `{}` 空体无校验直打 DB**：express/subscribe（漏 id 列 23502）+ calendar setting（空 code 撞 uq 23505）+ meeting save（startTime/endTime 空串绑 timestamp）→ 请求校验 400 指认字段（证据测试×3：calendar/express/meeting 各一常驻用例断言 BadRequest 且 message 含字段名）。**族③ 其他子族**（PG 日志 42P01/42703/42P10/42702/42883/42P18 实证）：appdict 占位符 off-by-N（depth 恒 `$3`，曾 `paths.len()+2` 指向路径段参数 → cardinality(int4)>=text）；surface filter/attribute 表名双引号 `"PP_C_TASK"` 大小写敏感查无此表（去引号）+ 列名 camelCase 必须引号（补引号）；归属列名按真实 schema 对齐（x_query_view/statement 只有 `creator`（迁移 029+033/071 口径）；pp_e_mapping/mergeitemplan 只有小写 `creator_person`（迁移 032）——O9 归属闸 helper 引用了不存在列，PERM_VIEW 参数化结构补 `owner_expr` 字段）；designer workcompleted merge JOIN `wc."xworkId"`（驼峰列引号）+ 裸列加 `wc.` 前缀消歧义；document filter 的 `COUNT(*)+ORDER BY`（`list_from_table_filtered_inner` 剥 ORDER BY/LIMIT tail 再 count）；attendance qywx unit 表无 `o2_user` 列（去列+`stat_row_to_value` 改 try_get 容错——dd 同名表有此列，schema 不对称）；meeting config `to_char(text 列)`（直接 SELECT 文本列）；oauth provider 配置缺失 `ok_or(Internal)` → `Unavailable`（503 如实「依赖未配置」）；oauth bind/login 桩 handler Path 参数个数不齐（补齐 3 元组）；org WITH RECURSIVE 三处（DISTINCT+ORDER BY 表达式逐字对齐 select 列表、DISTINCT+JOIN 裸列名加 `p.` 前缀消歧义）；person/personcard/unit cursor 分页 `LIMIT $n` 绑 String（保留 i64 不转串）；card_page 非 withgroup 分支绑定 `$2`（group_type）但 cond 不使用（tokio `Parameters(2,1)`/PG 42P18）→ 按分支绑参+占位符重排；RowNotFound→404（`query_one`+`map_err(Internal)` 批量改 `query_opt`+`ok_or(NotFound)`：service_processing 75 处 + core_express 2 + personal 1；`process_id_complex` 手写 None→Internal 改 NotFound）。**改 schema 的两处（说明+回滚）**：迁移 115 `x_org_definition` 补 `data/creator/create_time/update_time`——029 与 077 重复定义同名表（IF NOT EXISTS），029 先生效缺列，077 的形状才是 personal/definition handler 的语义本体（回滚=DROP COLUMN，见 115_rollback）；迁移 116 `x_general_assemble_office.word_flag` boolean→text——036 定义 VARCHAR 被 101 误以 BOOLEAN DEFAULT false 补列，word_flag 语义是转换任务唯一凭据 uuid（回滚=text→boolean，有 uuid 数据时回滚失败需先清列，见 116_rollback）。（编号 115/116 系避开 main 已占用的 106/107）。**棘轮守卫**：`tests/idor_handler500_baseline.txt` 入库（.gitignore 加 `!` 例外），矩阵断言升级为集合包含「当前 ⊆ 基线」——新增即红；O10 收官基线收缩为**空集**（零容忍），解析跳过 `#` 注释行。**禁用会话 30s TTL 取舍（代码注释+测试+台账三处写明）**：`shared::session::PERSON_STATUS_CACHE_TTL=30s`（模块常量，注释：禁用传播延迟 vs 每请求查库频次的有意取舍，调值须同步改测试+写台账）；纯函数 `person_status_entry_fresh` + 边界测试 `person_status_entry_fresh_boundary`（29s 新鲜/31s 过期，不等真实时间）+ `person_status_cache_ttl_is_pinned`（断言恒 30s）——最坏窗口 30s 被钉死，不得悄悄变长。**证据链**：矩阵 161→0（R8 实测）；每族改前 PG 日志 SQLSTATE/探针 WrongType + 改后矩阵出列；族①另有 bbs 常驻测试 `o10_forum_create_roundtrip_decode_ok`（HTTP 200 + 数据回读正确）。**遗留**：① surface manage list 端点的占位错乱（list 路由按 xid 查单行）类型层已修不再 500，语义层「o2 manage filter 翻页 DSL」属契约对齐工程（未做，FIXME 注释锚在代码里）；② arity_probe 打共享库产生瞬态写读（某写路由 O10 修复后能写入 x_program_invoke → list 读到 timestamp 列 create_time → 潜伏 decode panic 实证后已修 to_char）——arity_probe 探针卫生（打共享库而非 scratch）列入候选池，**O11 已收口**；③ oauth 第三方登录真实链路（钉钉/企微/政务钉钉）依赖外部 provider 配置，测试环境 503 是如实信号。验证（merge origin/main 后复测）：矩阵 R9 绿（0 条 handler 级 500、0 越权缺陷，棘轮基线收缩为空集）；`cargo test --workspace --no-fail-fast` 216 套件 / 7987 用例全绿（含 4 守卫 + idor_matrix 52.8s + arity_probe）；迁移 115/116 在 fresh scratch 库实测——正向全链 001–116（118 文件）零错、回滚复原（word_flag→boolean、4 列 DROP）、再正向幂等，shared migrate 用例 2/2 绿；21 个触及 crate `clippy --all-targets` 零告警（`--workspace --all-targets` 因本机页面文件不足 OOM，非代码问题，已改为分包跑） | 族① 898dc24b8、族② 74d245ba0、族③ 73bb0fcea、族③a 19efc600a、守卫+TTL 88901de54、迁移 24e1bcf27 |
| O11 | 限流器 key 空间上界（唯一 IP 洪泛内存 DoS）+ arity_probe 探针卫生 | done | **改前实测量（一次性探针 + 外部工作集采样）**：限流器是 `Arc<RwLock<HashMap<String, Vec<Instant>>>>`，60s 定时清理只删「1 小时窗口外的 key」——10 万唯一 key 灌进去：表 100_000 条、工作集 +25.2 MB、179 ms 填满；再调一次 cleanup 仍 100_000 条（洪泛 key 全在窗口内，一条删不掉）。50 万唯一 key：表 500_000 条、工作集 +120.6 MB（≈246 B/key 线性），攻击者几分钟即可把进程推到 OOM——key 数完全由请求侧决定，是真实内存 DoS 面而非理论泄漏。**上界依据**：单实例 OA 正常峰值约 5k 个不同来源 IP/小时（办公网 NAT 出口 + 健康检查/监控探针 + 移动端出口 + 少量外网直连）×10 倍安全余量 → `MAX_KEYS=50_000`（约 12.5 MB 常驻）。**处置策略取舍（写进函数注释）**：选 LRU 逐出而非整体重置——LRU 逐出只让最久未用的 key 提前解禁（该 key 最多弱化一个窗口），攻击中/玩家触限中的 key 位于「最新」端不受影响，且攻击者新 key 照样计数照样 429；整体重置会把正被限流的攻击者 key 一并清零，等于每次撑满表送洪泛者一次全额新预算（洪泛帮凶），明确不采用。**实现**：超限时一次逐出到 80% 水位（`EVICT_TARGET=40_000`，一次 10_000 条）——批量逐出把 O(n log n) 排序摊薄到每 10_000 次超限写入一次，避免每请求全表扫描自身变成 CPU DoS；比较器改为先收集 `(末次请求时间, key)` 再排序（原先在比较器里 `table[k]` 反复哈希查表，10 万写入 2255 ms → 同一探针两次采样 343/519 ms）。判定路径零改动（cap 在写锁内追加写入后执行，未满直接 return 0）。**改后实测量（同一探针）**：50 万唯一 key → 表恒 49_955 条、工作集峰值 +22.9 MB 走平（与 10 万 key 时同量级，表规模由 `MAX_KEYS` 而非请求量决定）；cleanup 后仍 49_955 条。**正常流量等价性**：20 并发同 IP、阈值 10 → 恰好 10 通过 + 10 次 429（改前改后一致，常驻用例）；攻击场景 10 万 key 洪泛之后，正常玩家新 key 20 并发仍恰好放行 10 次（不是上界一满就全放行），洪泛后进表的计数准确（5 次=5 条）。**已知代价（钉进注释+测试）**：洪泛前就已静默的旧 key 会被当成 LRU 最旧者逐出、计数归零（最多提前解禁一个窗口）——这是选 LRU 的既定代价，用例 `eviction_drops_least_recently_used_keys_only` 断言「活跃 key 不得被逐出」。**顺带修既有测试竞态**：4 个读 `OA4RUST_AUTH_RATE_LIMIT` 的用例与 1 个写该变量的用例共用 `testing::ENV_LOCK` 串行（此前并发跑会间歇性拿到 200 而非 429）。**遗留②收口（arity_probe 探针卫生）**：改前直连共享库 `oa4rust_test`；改后每轮 DROP/CREATE 专用库 `oa_arity_probe` + 全量迁移、跑完 DROP，并把 `OA4RUST_*_RATE_LIMIT` 抬到 1e6（此前穷举请求在 300 条后全被限流挡掉，探针实际只覆盖前 300 条路由）。实测（共享库 383 表总行数快照）：改前跑一次 3306→3306（本环境写路由被鉴权挡下未落行，但探针仍指向共享库、风险随路由行为变化而复活），改后 3306→3306 且 scratch 库自报写入 0 行、跑完已 DROP；基线 arity-500 0 / unreachable-404 0 未回归。**遗留**：限流器整表写锁在高并发下是单点串行（本次未动——改分片锁需重做窗口语义，收益需先有真实 QPS 基线与分片热点数据支撑，避免无据优化）。验证：`cargo test --workspace --no-fail-fast` **216 套件 / 7992 用例全绿**（较 O10 的 7987 +5 条 O11 用例；含 4 守卫、idor_matrix 59.3s、arity_probe 10.7s） | 90326e48b（上界+LRU）、1cc2e3c38（20 并发守卫）、8f6fd5dce（攻击场景验收）、369008a0b（探针迁 scratch） |
| O13 | 门禁健全性：openapi-paths 空转、fmt 漂移、全项审计 | done | **关键结论：「修之前它到底有没有在工作」——openapi-paths 从未工作过。** 证据三路：① `git log --all -- oa4rust/backend/scripts/check_openapi_up_to_date.py` 全历史为空（不是「曾经存在后被删」）；② 目录现状无此文件；③ 根因是 `.gitignore` 对整个 `oa4rust/backend/scripts/**` 的忽略 + 白名单只列了 `gen_openapi_paths.py`/`extract_routes.py` 等生成器——脚本写好了也 `git add` 不进去，因此**在任何干净克隆/CI 上都只有 exit=2**（本地能跑只是因为工作树里有未入库文件）。过去几轮 warnings-gate 里该项 FAIL 与代码无关，是环境性空转。**修好后立刻抓到真漂移**：O6 删掉 `PUT /api/calendar_assemble_control/event` 死注册后产物从未重生成，`crates/openapi/src/lib.rs` 4734→4733 path items（-13 行）。处置：补 `check_openapi_up_to_date.py`（跑生成器 → 逐字（行尾无关）比对 → 不一致 exit 1；跑完恢复原产物、check 本身不改仓库）+ `.gitignore` 白名单放行（附「为何必须入库」注释）。破坏实验：篡改生成物 → `--only openapi-paths` FAIL(exit=1, 1 告警, 1.2s)，还原 → PASS。**fmt 漂移**：CI 锁 1.98.1、本地 stable 1.99，同一命令相差 1186 hunk/45 文件；根因还包括生成物与 `cargo fmt --check` 天然冲突（`cargo fmt --check` 要求换行、regen diff 门禁要求逐字相等）。处置：`oa4rust/backend/rust-toolchain.toml` 钉 `channel=1.98.1`（+rustfmt/clippy），两个生成器写完产物后自己调 rustfmt；再出一个**纯格式化**提交（`cargo fmt --all`，45 文件，全部 oa4rust 范围，message 明写 fmt-only；逐文件去空白/逗号比对确认无逻辑改动）。破坏实验：人为打乱一行格式 → `fmt --check` exit 1；还原 → exit 0。**全项审计（17 项）**：唯一「引用路径不存在」的只有 openapi-paths；其余引用脚本（`cross_check_r7_readonly.py`/`check-license-headers.mjs`/`limit_scan.py` 等）均存在且已被 git 跟踪。新加元守卫 `script-paths`：扫描各 cmd 里 `scripts/*` 引用，要求文件存在且 `git ls-files` 跟踪——把「同类空转」变成一句可读 FAIL（破坏实验：建一个被 .gitignore 忽略的探针脚本并被某检查项引用 → FAIL「未被 git 跟踪」，删除 → PASS）；custom 守卫加 try/catch，源文件改名/解析失效时按项 FAIL「守卫自身执行异常」而非整条门禁崩溃。审计另抓出两类环境性红：`llm-lab-tests` 在 CI 无 torch，两个测试模块顶层 `import torch` 导致 pytest collection ERROR exit=2（不是代码回归）——按同目录既有约定改 `pytest.importorskip`（模拟无 torch：两模块 SKIPPED、其余用例执行、exit=0；本机有 torch 时 130 用例全绿）；`license` 有 4 个另一条线脚本（`chaos-drill.mjs`/`check-slo.mjs`/`multi-instance-check.mjs`/`trace-pull.mjs`，c3a3abb6d 引入）漏 SPDX，补 header 转绿（破坏实验：删一行 → 「缺失 1」exit=1，还原 → 0）。**顺带修本线遗留**：`limit-scan` 红是 O10 改写 raw SQL 后未回填基线（cms parent_key 9→10 计数表单行读、program_center unbounded 75→76 全量列表、message 9→8 收紧），`--update-baseline` 后基线 693 条绿（破坏实验：临时加一条未登记 SELECT → FAIL，还原 → PASS）。 | a954f9e60（守卫+gitignore 白名单）、f28b4be38（openapi 产物重生成）、a9851279f（toolchain+生成器 fmt 钩子）、ea0b32bef（fmt-only）、e1a67dc93（license）、ebc38a974（limit-scan 基线）、bb6661702（llm-lab torch）、6683b40c2（script-paths+try/catch） |

## 不开轮记录（探针全绿 / 无真实后果）

| 日期 | 探针主题 | 结论 |
|------|----------|------|
| 2026-11-04 | O12 限流器整表写锁成本（`Arc<RwLock<HashMap>>` 单点串行） | **不是瓶颈，不开轮**。方法：release 直调 `check_rate_limit`，3 场景（normal5k / nearcap50k=表逼近 50k 上界 / hot1 单 key）× 并发 1/100/200 × 3 变体（noop=只做 key 格式化、lockonly=只拿写锁即放、full=现行路径），每格 10 万次 × 2 轮，32 线程机。**无竞争**：c=1 时 full p50 0.1–0.3 µs（mean 0.18–0.32 µs），lockonly 0.06 µs——锁本身开销在噪声内。**饱和吞吐（全局锁的真实上界）**：full 2.18–2.29 M ops/s（≈438 ns/次串行），lockonly 3.13–3.40 M ops/s，noop 75–92 M/s。**排队读数**：c=200 时 full p50 80.8–104.2 µs / p99 145–194 µs，但这是闭环压满（200 任务零 think time）的 Little 定律排队（W≈c/λ≈200/2.3 M≈87 µs），锁内工作在 c=1 只占 0.14 µs（full−lockonly），不是额外锁成本。**表规模无额外成本**：nearcap50k 2.28 M/s 与 normal5k 2.25 M/s 同档（50k key 的哈希查找/逐出摊销可忽略）。**判定依据**：2.3 M rps 的串行上限 vs 现实需求——general 阈值 300/min/IP（5 rps/合法 IP），1 万个 IP 全打满也只有 50 k rps（约 2 % 占用率）；HTTP/DB 层远先于限流器成为瓶颈。分片锁只能把上限抬到 N×2.3 M，代价是 LRU 退化成分片 LRU、总上界改为每片 `MAX_KEYS/N`（可观察语义变化），在无真实 QPS 基线下属无据优化 → **不改架构**。顺带修复合并后 main 验证暴露的两处门禁（均为本线遗留）：`50b724b45`（clippy `unnecessary_sort_by`，O11 代码）、`7734da323`（O10 脚本缺 SPDX header）。 |


| 2026-11-04 | O14 生成物漂移治理（33 个含人工内容的 tests_generated.rs） | **不开轮**（纯测试/门禁基础设施，无运行时行为改动）。按用户决策走「方向 1（治本）」：生成器获得 per-crate 入口配置，人工内容迁入人工维护的 `tests_manual.rs`，91 个产物统一重生成，并补两道门禁把「生成物 + 无校验」模式堵死。明细见下方「O14 明细」。 |

## O14 明细（生成物漂移治理）

> 定性沿用 O14 首轮重跑 diff：①纯噪声 7 个（仅少 SPDX + `#![allow]`，生成器不产出、靠仓库级补丁）②模板/源过期 83 个（旧模板 `assert_ne!` 未重生成，59 个受影响）③人工改过 33 个（29 个同时属于②）。③中又分两类：**18 个仅入口形态被人工改**（机械改动，收进配置）、**15 个含真手写测试**（迁入 tests_manual.rs）。`message_assemble_communicate` 的手写探针不在原标记集里，本轮由「非模板形状」扫描补入，故 33 = 18 + 15。

### O14-1 入口配置表（`ROUTER_ENTRY_KINDS`，生成器内显式字典）

| kind | 生成代码 | crate |
|------|----------|-------|
| `pool`（默认，未登记即此形态） | `let pool = shared::testing::test_pool(); crate::router(pool)` | 其余有路由测试的 crate |
| `db` | `let db = shared::testing::mock_db().await; crate::router(db)` | ai_core_entity, attendance_core_entity, bbs_core_entity, calendar_core_entity, correlation_core_entity, file_core_entity, general_core_entity, hotpic_core_entity, jpush_core_entity, message_core_entity, mind_core_entity, organization_core_entity, portal_core_entity, processplatform_core_entity, query_express |
| `db_pool` | `let db = mock_db().await; crate::router(db, mock_pool())` | cms_core_entity, meeting_core_entity |
| `pool_db` | `let pool = test_pool(); crate::router(pool, mock_db().await)` | cms_express |
| `test_router` | `let pool = test_pool(); crate::test_router(pool)` | auth, program_center_core_entity |
| `ws` | `crate::ws_route()` | realtime |

配置注释明确「新增 crate 必须在此登记入口形态」；未登录形态会让 `--all-targets` 编译失败，门禁可见。生成器另自产 SPDX 头与 `#![allow(non_snake_case)]`，并按生成结果计算 import（消除旧版 `use shared::testing::*` / `Method` 的 unused import）。

### O14-2 手写测试迁移清单（15 crate / 84 用例 / 8 处 ignore）

| crate | 数量 | 内容 |
|-------|------|------|
| ai | 1 | F6：`test_index_delete` |
| base | 7 | plan002 U2 批量 5 条 + 纯逻辑路径穿越 2 条 |
| calendar_assemble_control | 7 | plan002 U2 `test_u2_*` |
| cms_core_express | 1 | `#[ignore]`：`test_content_detail` |
| component_core_entity | 1 | `#[ignore]`：`test_component_get` |
| file_assemble_control | 9 | 手写 `let __st/__body` 探针 6 条 + 2 处 `#[ignore]`（list_top）+ `folder2` 特例 |
| message_assemble_communicate | 1 | 手写探针 `test_im_msg_download_id`（原标记集遗漏） |
| organization_assemble_authentication | 2 | 2 处 `#[ignore]`（icon/identity） |
| organization_assemble_personal | 2 | O10 回归（user_setting / role_list） |
| processplatform_assemble_designer | 1 | `#[ignore]`：`test_get_flow` |
| processplatform_core_express | 6 | O10 回归（work/task 系列） |
| processplatform_service_processing | 21 | O10 回归 20 条 + `#[ignore]` gateway_join |
| program_center | 9 | plan002 U2（config 3 + invoke 4 + appstyle 2） |
| program_init | 15 | plan002 U2 12 条 + 纯逻辑 JDBC URL 解析 3 条 |
| query_express | 1 | O9 回归 `test_query_list` |

**与预期差异（如实记录）**：用户口径为「6 处 `#[ignore]`」，实际在 HEAD 上是 **8 处**（上述表内逐条列出）。迁移后 8 条全部保留，运行输出中 8 ignored、76 passed。`file_assemble_control` 的探针为 6 条（`let __st` 5 条 + `folder2` 1 条），加 2 条 `#[ignore]` 共 9 条。迁移只搬不改语义：原块逐行复制（含分节注释与中文说明），仅 rustfmt 归一空白；`tests_manual.rs` 由人工维护，生成器不覆盖。

### O14-3 重生成前后测试数对比

| 指标 | 重生成前 | 重生成后 |
|------|----------|----------|
| tests_generated.rs 文件数 | 90 | 91（+base_core_project 首次产出） |
| tests_generated 测试函数 | 1705（1700 tokio + 5 纯逻辑） | 1314（全 tokio） |
| tests_manual 测试函数 | 0 | 84（79 tokio + 5 纯逻辑，其中 8 ignored） |
| `#[tokio::test]` 合计 | 1700 | 1393（1314 + 79） |

减少的 391 条（1705→1314）不是丢测试，而是旧产物里已不存在的历史集合：过期模板/已删除路由（如 processplatform_assemble_surface 538→414、cms_assemble_control 255→211、ai_assemble_control 31→3）与同名 handler 重复（去重优先保留路由表那份）。反向新增来自内核 crate 首次被路由抽取器识别：organization_assemble_express 4→40、processplatform_assemble_bam 5→48、organization_assemble_authentication 2→24、query_assemble_designer 5→20、base_core_project 0→5。全部变化均可由生成器从源码复现（门禁逐字证明）。

### O14-4 门禁与破坏实验证据

| 门禁 | 判定方式 | 破坏实验 |
|------|----------|----------|
| `generated-tests`（新增） | 生成器跑临时目录 → 与 91 个已提交产物逐字节比对；附首个差异行号；同时校验生成器被 git 跟踪、每个 lib.rs 有 `mod tests_generated;`、规模下限（≥90 文件 / ≥1200 tokio 测试） | 手改 `ai/src/tests_generated.rs` 第 5 行 → FAIL 并精确报出该行；`git checkout` 还原 → PASS。把 `MIN_TESTS` 临时抬到 2000 → 规模断言 FAIL；还原 → PASS |
| `generated-artifacts`（新增） | 快照 → 就地重生成（`gen_mcp_tools.py`）→ 逐字节比对（CRLF 归一）→ 写回快照 | 首次运行即抓到真实漂移：`generated_routes.rs` 1 行（路由源码改为 `crate::get_calendar_detail`，描述未同步）；重生成后 PASS |
| `script-paths`（元守卫，O13） | 各检查项引用的 `scripts/*` 必须存在且被 git 跟踪 | 新判定器两次踩到 `scripts/**` 忽略规则：首次提交只入了 check-warnings，脚本本身被忽略 → 补 `.gitignore` 白名单后 PASS |

### O14-5 全仓生成物清单审计

| 产物 | 生成器 | 门禁 | 结论 |
|------|--------|------|------|
| `crates/*/src/tests_generated.rs`（91） | `generate_handler_tests.py` | **有**：`generated-tests`（本轮新增） | 逐字 + 规模下限 + 生成器跟踪 |
| `crates/openapi/src/{lib.rs,request_types.rs}` | `gen_openapi_paths.py` | **有**：`openapi-paths`（O13 修复） | 逐字比对（行尾无关），跑完恢复原产物 |
| `crates/mcp_server/src/generated_routes.rs` | `gen_mcp_tools.py` | **有**：`generated-artifacts`（本轮新增） | 快照→重生成→比对→还原 |
| `crates/parity/src/generated_tests.rs` | `generate_parity_tests.py` | **无** | 输入 `docs/audits/o2server-parity-report.json` 已在 `c158a0251` 归档清理中删除，干净克隆无法重生成。**登记 O15 债务**：恢复输入并补门禁，或退役该产物 |
| `crates/parity/src/behavior_tests.rs` | 无（头注释误写 AUTO-GENERATED，下一行自称 Hand-written） | **无** | 非脚本产出，实为手工维护；无漂移源。**O15 更正误导性头注释** |
| `frontend/packages/apis/src/api-coverage-generated.test.ts` | `oa4rust/scripts/generate-api-tests.ts` | **无** | 依赖前端 TS 工具链（npm ci/tsx），不在 `check-warnings.mjs` 轻量脚本集合。**登记 O15 债务**（在 web CI 里跑生成+diff） |
| `tests/contracts/backend-route-methods.fixture.ts` | 无（注释称 AUTO-GENERATED from generated_routes.rs，仓库内无脚本） | **无** | 一次性产物，消费方 `route-methods.test.ts`；无法从源头复现。**登记 O15 债务**（补生成器或退役） |
| `backend/tests/behavior_comparison/endpoints.rs` | `extract_endpoints.py` | 无 | 未入库（.gitignore），运行期产物 → 不需要门禁 |
| `scripts/extract_routes.py` 输出（stdout JSON） | `extract_routes.py` | 无 | 不落盘、不入库 → 不需要门禁 |
| `backend/docs/audits/artifacts/fi002_unknown_columns.tsv` | `fi002_column_scan.py --update-baseline` | **有**（CI `oa4rust-ci.yml:341` 跑扫描，新增未知列即 exit 1） | 人工复核后更新的棘轮基线 |
| `limit-scan` 基线 | `limit_scan.py` | **有**：`limit-scan`（O13 修复） | 同上（棘轮基线） |

### O14-6 验证与遗留

- `cargo clippy --all-targets --workspace`：通过（零告警）。
- `cargo test`：除 `shared` 外全部 crate 通过；`tests_manual::` 84 条通过（8 ignored）；`cargo fmt --all --check` 通过。
- `shared` 的 `migrate::tests` 用例：**已用干净库界定**，不是迁移缺陷，也不是本分支引入。
  - 干净库（`oa14_scratch_mig`，空库、跑完 DROP）：`cargo test -p shared --lib migrations_are_ordered_idempotent_and_fail_loud_on_missing_dir` → `1 passed; 0 failed`，118 条迁移从零全部应用。
  - 本地库（默认 `postgres://o2server:<PASSWORD>@localhost:5432/oa4rust`）同命令 → `first migration run: migration 019_create_identity_tables.sql failed: column "person_unique" does not exist (SQLSTATE 42703)`。
  - 根因证据：本地 `schema_migrations` 仅 19 行（最高 `018`），但 schema 已是 043 之后（`auth_person_identity.person_id`，`person_unique` 已不存在），且含 101 之后的 `x_cms_document.xappid` —— schema 领先于台账，019 重跑时索引指向已改名列。干净库全链绿证明迁移 SQL 与顺序无缺陷。
  - 复现：`docker exec lyp-pg psql -U o2server -d oa4rust -c "SELECT count(*) FROM schema_migrations;"` + 上述失败命令。
  - 清理（破坏性，会重建本地开发库）：`docker exec lyp-pg psql -U o2server -d postgres -c "DROP DATABASE IF EXISTS oa4rust WITH (FORCE);" -c "CREATE DATABASE oa4rust OWNER o2server;"`，再由迁移测试/启动全量建库后同命令转绿；或保留旧库，测试时设 `DATABASE_URL=postgres://o2server:<PASSWORD>@localhost:5432/oa4rust_test`。
- O15 候选（本轮回调出的无门禁项）：parity `generated_tests.rs`、`api-coverage-generated.test.ts`、`backend-route-methods.fixture.ts`、`behavior_tests.rs` 误导头注释。分类与下一步见下节。
- 已知债务（不在本轮范围，均已在 `main` 上复现）：laiyipao `Server build` 的 `TestPeriodStartHandlesEveryKnownScope` 时区断言（期望 UTC、实际 `+0800 CST`）、auto-checkin `Coverage ratchet` 红；两条与本 PR 改动无关。

## O15 债务登记（生成物清单回收）

> 无门禁的生成物只有两种合法状态：**可恢复**（给出「取回输入 → 重生成 → 接门禁」三步）或**已废弃**（连生成器一起删）。不许长期留一个跑不起来的脚本。

### 已处理

| 产物 | 最终判定 | 证据 |
|------|----------|------|
| `crates/parity/src/generated_tests.rs` | **保留**（已接门禁） | 三分类：① 噪声＝CRLF + 时间戳 + 98 个序号名位移；② 源码/生成器过期＝新增 57 条 personal 别名路由、3 条路径/方法纠正（`message consume` GET→POST、program_center 重复段与 GET→POST）；③ 手工＝1 条 `get_calendar_detail` 语义命名，无独有断言，已原样迁入 `parity/src/tests_manual.rs`。输入 `generated_at=2026-08-12`、文件最后修改 2026-09-17，不是「两年未更新」；4098 条机械用例仅 1 条手工痕迹，不构成「大量人手增强」→ 保留。生成器已去时间戳、固定 LF；parity 已加入 `check_generated_artifacts_up_to_date.py`；破坏实验：插入一行→`首个差异：...:3`、exit 1，还原→exit 0 |
| `frontend/packages/apis/src/api-coverage-generated.test.ts` | **保留**（生成器已升级，接 CI 门禁） | 差异三分类：① 格式/头＝SPDX、import 顺序、prettier 缩进、末尾换行，生成器已复现；③ 手工增强＝每条用例的 `try { await fn?.() } catch`（驱动 V8 覆盖），生成器已按原样固化，未降级产物；② 源码过期＝`index.ts` 新增 28 个函数（attendanceDeepApi 8、processApi 5、documentApi 5、calendarCoreApi 5、attendanceControlApi 2、viewApi/viewCategoryApi/viewFieldConfigApi 各 1），重生成后 367 行纯新增、**0 删除**。门禁：`oa4rust-web.yml` 在 Vitest 前跑 `pnpm gen-api-tests` + `git diff --exit-code`；破坏实验：暂存一条 `// BREAK EXPERIMENT`→红在 `:8718`，还原→exit 0 |
| `tests/contracts/backend-route-methods.fixture.ts` | **保留**（`gen_mcp_tools.py` 单一源头同步输出，已接门禁） | 131/63/2 三分类：**131 只在 fixture** ＝ 102 条 `/api/bbs/assemble/...`（**O16 更正：不是已删残留，而是 `&fmt("rel")` + `API_BASE` 注册的真实路由；O15 的解析只认字面量，误判为残留并从这里删掉**）+ 24 条旧前缀 `/api/query_service_processing/{}`（源码已改为 `/api/query/service/processing/...`）+ 3 条 processplatform 残留（**同为闭包 handler 漏扫的真实路由**）+ `/health`、`/openapi.json`（框架路由，非残留，保留进 fixture）；**63 只在后端** ＝ 61 条 personal 别名路由（fixture 侧漏同步，已补）+ `/ready`（产品路由，已补）+ `/boom`（`#[cfg(test)]` 测试路由，生成器新增 cfg(test) 剥离后不再产出）；**2 条方法集不符** ＝ `/api/calendar_assemble_control/event` fixture 多 PUT（`routes.rs:72-78` 注释明确 O6 删除死 PUT，属 fixture 残留，已随重生成消失）、`/mcp` fixture 有 GET+POST 而工具表仅 POST（GET 真实存在于 `mcp_server/src/main.rs:71-76` 闭包，正则漏扫；GET /mcp 不是工具，用 `EXTRA_FIXTURE_ROUTES` 显式保留在 fixture）。门禁：fixture 加入 `check_generated_artifacts_up_to_date.py` 的 mcp job；破坏实验：插入 `// BREAK EXPERIMENT`→`首个差异：tests/contracts/backend-route-methods.fixture.ts:3`、exit 1，还原→exit 0。**本行后半段（重生成 4108→4041、`route-methods.test.ts` 交 CI）已被 O16-0 推翻**：O16-0 修复解析后重生成 4735→4874（+139），并同步输出 registered fixture；见 O16 节 |

### 可恢复（剩余）
（无。第 3 项已转为单一源头 + 门禁，见上表）

### 非生成物、仅需更正

| 产物 | 问题 | 处理结果 |
|------|------|----------|
| `crates/parity/src/behavior_tests.rs` | 头注释误写 AUTO-GENERATED（下一行自称 Hand-written） | 已改为「Hand-written behavior contract tests — 手工维护，非生成物」，并注明生成的 route_exists 用例在 `generated_tests.rs` |
| `backend/tests/parity_suite.rs` | 头注释写死「785-route」，实际已 4099 路由 / 4098 用例 | 去掉硬编码数字，改为「条数见 generated_tests.rs 文件头」，避免再次腐化 |
| `backend/migrations/032/033/037` | 头注释写「Auto-generated」，仓库内无生成器（一次性产出） | 改为「One-off generated (no reproducible generator)」，如实标注来源 |
| `frontend/packages/apis/src/index.ts:1281` | 段落注释「Auto-generated — do not edit manually」，但该文件长期由人工修改（多个 fix 提交） | 改为「Hand-maintained（早期一次性生成，仓库内无生成器）— 修改时同步后端路由」 |
| `crates/parity/src/generated_tests.rs` 文件头 | 「Tests: 4099」与实际 4098 条不符：1 条路径含空格无法构造 URI，只产出 skipped 注释 | `generate_parity_tests.py` 改为按实际产出计数；重生成后 `Routes: 4099   Tests: 4098`。门禁 `generated-artifacts` 仍绿 |
| `tests/contracts/backend-registered-routes.fixture.ts` | 头注释称 Auto-generated，仓库无生成器；且快照落后：131 条「fixture 有、生成器无」里有 102 条 bbs 与 3 条 processplatform 是**解析漏扫的真实路由**（O16 更正），24 条旧 query 前缀才是真残留；62 条后端新路由缺失 | O16 已改为由 `gen_mcp_tools.py` 同源输出并接入 `check_generated_artifacts_up_to_date.py`；重生成后 4108→4170 条（旧 `{p}` 24 条归零、61 条 personal 别名与 `/ready` 补齐） |

### O15 追加发现（第 4 项全仓扫描）

全仓扫描口径：源码前 25 行匹配 `AUTO-GENERATED` / `auto-generated` / `自动生成` / `DO NOT EDIT` / `请勿手动修改`。结论：除上表已更正的条目，其余声明都有真实生成器且已接门禁（`generated_routes.rs`、`openapi/src/{lib,request_types}.rs`、`generated_tests.rs`、route-methods fixture）或属于生成器脚本自身的输出模板，不再有「声称自动生成但无生成器」的文件头。

| 产物 | 现状证据 | 下一步（可执行） |
|------|----------|------------------|
| `tests/contracts/backend-registered-routes.fixture.ts` | 4108 条 vs 后端生成路由：**131 条只在 fixture**（102 条 `/api/bbs/assemble/...`（**O16 更正：`&fmt` 真实路由，非已删**）+ 24 条旧 `query_service_processing` 前缀（真 C 类）+ 3 条 processplatform（闭包 handler 真实路由）+ `/health`、`/openapi.json` 框架路由）、**62 条只在后端**（61 条 personal 别名 + `/ready`）。按后端重生成后模拟 desktop 守卫：**132 条 desktop `/api` 字面量命中失败，其中 108 条是 bbs 假阳性（完整解析后为 112 条 bbs 调用全部命中），真问题是 24 条旧 query 前缀** | **O16 已闭环**：`gen_mcp_tools.py` 同源输出本 fixture + route-methods；前端 24 条 query 迁移到新前缀。重生成后 desktop 守卫 flagged 从 132 → 24 → 0，见 O16 节 |



## O16 指向已删路由的 `/api` 调用核查（实质轮次）

**结论**：132 条报告是审计口径污染。完整解析后实际 136 个调用点 = **112 条 bbs 假阳性（A=0、B=0）+ 24 条 query C 类待迁移**。没有真断链，也没有死代码。

### 136 条三类分布
| 类 | 条数 | 判定依据 |
|----|------|----------|
| A 真断链 | 0 | 无（112 条 bbs 调用全部命中 `bbs_assemble_control/src/routes.rs` 的真实路由） |
| B 死代码 | 0 | 无（24 条 query 调用均挂在 `QueryViewApp.vue` 的实际按钮/流程上） |
| C 待迁移 | 24 | 后端等价路由为 `query_service_processing/src/lib.rs:312` 的 `p = "/api/query/service/processing"`；旧前缀是审计产物把 `format!("{p}/…")` 的局部变量 `p` 误当字面段造成的 |
| 假阳性（不入三类） | 112 | `BBSForum.vue` 134 个调用点按路径去重 112 个，全部命中 `&fmt("rel")` + `API_BASE` 注册的 bbs 路由；O15 口径 132 = 108（`${encodeURIComponent}` 截断少算 4 条）+ 24 |

### 112 条假阳性根因
`gen_mcp_tools.py` 的旧正则只认 `.route("字面量")`，漏扫三类真实注册：
1. `bbs_assemble_control/src/routes.rs` 的 `.route(&fmt("rel"), …)`，前缀来自 `API_BASE = "/api/bbs/assemble/control"`（106 处 `&fmt` 注册，含链式方法共 112 个 path/method 条目）；
2. `query_service_processing/src/lib.rs` 的 `.route(format!("{p}/…").as_str(), …)`，前缀来自函数内 `let p = "…"`（24 条）；
3. `processplatform_assemble_surface/src/routes.rs:394-402` 的闭包 handler `get(|…| async …)`（3 条）。

同一缺陷还让 `docs/audits/three-ends-2026-09-20/extract_routes.py` 把 `{p}` 当字面路径段写进 `backend_routes.json`，旧 fixture 与前端照抄成了 `/api/query_service_processing/0/…`。

### 129 条守卫削弱事故与修复
- **事故**：O15 依据旧口径把 129 个真实路由条目当成「已删残留」从 route-methods fixture 删除（102 bbs 路径 + 24 query + 3 processplatform）；同时 `route-methods.test.ts` 对未注册路径直接 `continue`，测试仍绿。守卫从「缺路由必红」退化为「fixture 缺什么就少测什么」。
- **修复**：O16-0 让生成器按表达式解析三种注册形态并点名失败不可解析项；重生成 `generated_routes.rs` 4735→4874（**+139**：bbs 112、query 24、closure 3，按 path/method 计；与 129 的差额是 O15 按路径去重且其 bbs 清单只抓到 102 条）。
- **验收**：模拟 desktop 守卫 flagged 从 132 → 24（只剩 query），112 条 bbs 假阳性归零；旧 `{p}` fixture 条目 24 → 0。

### 审计脚本口径修正
`extract_routes.py` 现在按 `.route()` 首参表达式解析：
- 字面量 `"/api/..."` → 真字面段，原样使用；
- `&fmt("rel")` → 用同文件 `API_BASE` 常量 + `normalize_legacy_path`（去空段/去相邻重复）还原；
- `format!("{name}/…")` → 只有能在 `before` 位置解析到 `let/const name = "…"` 的占位符才算局部变量前缀；`{{`/`}}` 还原为字面 `{`/`}`；
- 解析不出的（运行时拼路径）点名计入 `unresolved` 并跳过，**不再把 `{p}` 写进产物**。
历史快照 `backend_routes.json` 未重写（保留审计当时事实），但 fixture 已与它脱钩，改由生成器同源产出。

### 三份产物条数自洽（同一路由表派生）
| 产物 | 计数 | 口径 |
|------|------|------|
| `generated_routes.rs` | **4874** | (path, method) 条目 |
| `backend-route-methods.fixture.ts` | **4170** 路径 / **4874** 方法条目 | 同一扫描按 path 归并方法集 |
| `backend-registered-routes.fixture.ts` | **4170** 路径 | 同一扫描 + `EXTRA_FIXTURE_ROUTES`（路径集合与上者一致） |
差异均为口径而非源头：4874 vs 4170 = 同一路径多方法；4170 vs 4170 = 同一集合。

### 24 条 C 类映射（方法不变，去掉前缀后 `0` 标志段）
| # | 旧路径（前缀 `/api/query_service_processing/0`） | 新路径（前缀 `/api/query/service/processing`） |
|---|------|------|
| 1 | `/design/search` | `/design/search` |
| 2 | `/index/directory/document/count` | `/index/directory/document/count` |
| 3 | `/index/update/extra/document` | `/index/update/extra/document` |
| 4 | `/neural/generate/model/0` | `/neural/generate/model/0` |
| 5 | `/neural/learn/model/0` | `/neural/learn/model/0` |
| 6 | `/neural/list/calculate/model/0/work/0` | `/neural/list/calculate/model/0/work/0` |
| 7 | `/neural/stop/generating/model/0` | `/neural/stop/generating/model/0` |
| 8 | `/neural/stop/learning/model/0` | `/neural/stop/learning/model/0` |
| 9 | `/table/reload/dynamic` | `/table/reload/dynamic` |
| 10 | `/table/0/insert` | `/table/0/insert` |
| 11 | `/table/0/update/0` | `/table/0/update/0` |
| 12 | `/touch/high/freq/document/node/0/reset` | 同右 |
| 13 | `/touch/high/freq/document/node/0/touch` | 同右 |
| 14 | `/touch/high/freq/work/node/0/reset` | 同右 |
| 15 | `/touch/high/freq/work/node/0/touch` | 同右 |
| 16 | `/touch/high/freq/workcompleted/node/0/reset` | 同右 |
| 17 | `/touch/high/freq/workcompleted/node/0/touch` | 同右 |
| 18 | `/touch/low/freq/document/node/0/reset` | 同右 |
| 19 | `/touch/low/freq/document/node/0/touch` | 同右 |
| 20 | `/touch/low/freq/work/node/0/reset` | 同右 |
| 21 | `/touch/low/freq/work/node/0/touch` | 同右 |
| 22 | `/touch/low/freq/workcompleted/node/0/reset` | 同右 |
| 23 | `/touch/low/freq/workcompleted/node/0/touch` | 同右 |
| 24 | `/touch/optimize/index/0/touch` | 同右 |
（表中「同右」＝旧路径后缀完全保留，仅替换两段前缀；实际提交见 `QueryViewApp.vue`。）

### O16-1 未注册路径不再静默跳过
- `route-methods.test.ts` 未注册调用显式计数，基线 **864 → 860**（迁移后 4 条无动态段路由命中；另 20 条带实参 `0`，精确匹配下仍计未注册）；基线只允许下降。
- 另设 fixture 路径数下限 **4170**：删任何真实路由独立触发红。
- 破坏实验：删除 `/api/ai/app/list` → 红在 `route-methods.test.ts:94`（未注册 865 > 864）；重生成还原 → 绿。

### O16-2 真实运行验证
后端 `--http` + 已迁移测试库 `oa4rust_test`（避开本地 `oa4rust` 库 019 迁移失败的环境问题）：
```
GET /api/query_service_processing/0/table/reload/dynamic -> 404
GET /api/query/service/processing/table/reload/dynamic   -> 401
```
401 说明新路由存在、请求已进入鉴权中间件（不是 404）。本地 `vitest` 的 `route-methods` 与 `desktop-endpoints` 两套契约、`pnpm typecheck` 均绿；其余前端套餐测试由 CI 覆盖（本地未跑全量 unit/component）。

### O16-0b 生成器格式复发（追加硬要求）
- **成因**：生成器固定输出单行 `'path': ['GET'],`，超过 Biome `lineWidth=120` 的行会被要求换行；每次重跑都会把手工 `format --write` 的产物打回原形，frontend-qa 必红。
- **修法**：在生成器内实现 Biome 的实测换行规则（单行 ≤120 / `'key': [` 可容纳时数组逐行 / 否则 key 独占一行），**不调用 biome 二进制**——generated-artifacts 门禁在纯 Python 环境运行，引入 Node 会让「可离线复现」变成「取决于前端工具链」。
- **为什么修生成器而不是产物**：产物是派生物，手改只能撑到下一次重生成；这正是本轮要消灭的坏习惯。
- **验收**：重跑生成器两次，三份产物 `git diff --exit-code` 均为空；`pnpm lint` 零 error；两次之间 `biome check` 均绿。破坏实验：去掉格式化步骤 → 红在 `tests/contracts/backend-route-methods.fixture.ts` 的 `format` 规则；恢复 → 绿。
- 顺带修 `scripts/generate-api-tests.ts` 的 `lint/style/useTemplate`（此前被 fixture 红掩盖的存量问题）；本机 8 个 `.cjs`/`sw.js` format 报错是 `core.autocrlf` 本地 CRLF 噪声，已就地转 LF，仓库内容无变化。

### O15 更正（102 条 bbs 判定）
O15 第 3 项把 102 条 `/api/bbs/assemble/...` 判为「已删残留」是**错误**的：它们全部是 `&fmt("rel")` + `API_BASE` 注册的真实路由。误判原因是 O15 只认字面量 `.route("…")`，把「fixture 有、字面量扫描无」直接等同于「后端已删」，没有验证第二种注册形态。同一误判导致 129 条守卫条目被删。正确判定见上方 112 条假阳性根因与 129 条事故记录。

### O16 剩余
无 A/B 类处置项；24 条 C 类已迁移。未给后端加旧前缀 alias：仓库内无外部客户端依赖旧路径的证据，不为假设性兼容加层。

## O17 分页入口统一钳制（实质轮次：单请求读放大 / 内存 DoS 面）

### 为什么是它

「既有后续清单」第 6 项（输入上界缺失）里，分页 `size/page` 是唯一同时满足「真实生产后果 + 可实测 + 没做过」的开放项：任意已登录用户把 `size` 改成 `1e9`，一个本该 20 行的列表请求就变成全表读取，响应体/DB 传输/进程驻留都随数据量线性放大；`(page-1)*size` 在极端值下还会溢出（debug panic / release 负 offset → PG 500）。这不是「更快一点」，是把一个单请求即可触发的读放大面关掉。

### 改前（探针：5 万行 `x_org_person`，同一台机/同一库/同一并发）

| 场景 | p50 | p95 | 吞吐 | 错误率 | 响应/传输字节 |
|------|-----|-----|------|--------|----------------|
| 正常 `page/1/size/20`（20 次） | 23 ms | 26 ms | — | 0 | — |
| `size=1e9` 单请求 ×5 | 688 ms | 862 ms | — | 0/5 | 8,316,806 B（8.3 MB） |
| `size=1e9` 并发 8×5 | 1121 ms | 1166 ms | **7.08 req/s** | 0/40 | 332,672,240 B（317 MB） |

### 改后（同一探针、同一数据、同一并发）

| 场景 | p50 | p95 | 吞吐 | 错误率 | 响应/传输字节 |
|------|-----|-----|------|--------|----------------|
| 正常 `page/1/size/20`（20 次） | 23 ms | 26 ms | — | 0 | — |
| `size=1e9` 单请求 ×5 | 27 ms | 28 ms | — | 0/5 | 31,998 B（≈32 KB） |
| `size=1e9` 并发 8×5 | 33 ms | 41 ms | **223.46 req/s** | 0/40 | 1,279,920 B（1.2 MB） |

同一场景改前→改后：响应字节 260×、并发吞吐 31.6×、p95 28×；正常分页 p50/p95 无回归（23/26 ms 不变）。

### 缺口的量化口径

- 静态盘点：`Path` 形态分页 handler 153 个，其中 **133 个没有任何 size 上界**（只有 `size.max(1)`、或直接把 `size` 绑进 `LIMIT`）；`page_window` 此前只被 20 处采用。
- 另有 `page` 被当作 OFFSET 传入的 23 个 processplatform 旧式端点（`LIMIT size OFFSET page`），以及 body/query 派生分页（bbs subject filter、processplatform_service_processing）。

### 改法

- `shared::response::page_size(size) = size.clamp(1, 200)`；`page_window` 复用它。上界 200 与项目既有 `page_window` 约定一致（前端实测最大 `pageSize=100`）。
- 静态盘点出的 133 个无上界 handler 中，实际改写 80 个计算点（含 23 个旧式端点核心、7 处 SQL 侧 offset）；其余是薄包装，经其核心函数间接覆盖。先脚本预演、逐文件 diff 复核；已钳制的 100/500/1000 点不动。
- `page` 当 OFFSET 的 23 个旧式端点只钳 `size`，不改既有 offset 语义。
- SQL 侧 `LIMIT $n OFFSET ($m-1)*$n` 的 7 处（program_center deploy/dict/script、portal script、query table/import/view）改为 Rust 先算 offset 再绑 `LIMIT $1 OFFSET $2`，避免 PG 侧 bigint 溢出。
- 溢出点统一 `saturating_mul`（designer 4 处、personal log、program_center u3_snap、bbs subject 列表、service_processing）。
- 顺带覆盖 body/query 派生分页（bbs subject filter 的 `count`、service_processing 的 `size`）。

### 验证

- `cargo clippy --all-targets -- -D warnings` 零告警；`cargo fmt --all --check` 绿。
- 全新库 `oa4rust_o17_check`：先跑 migrate 用例建全表，再 `cargo test --workspace --lib` 全绿（0 FAILED）。
  - 环境说明：本机开发库 `oa4rust` 有历史 schema 漂移（`auth_person_identity` 是旧版 `person_id` 列），迁移 019 在该库失败；这是存量环境问题、与本次改动无关，全新库上 migrate 用例 2/2 绿。
- 探针是临时文件，跑完即删（不引入需要 DB + 5 万行数据的常驻测试）。

### O17 收口（B1：可失败的分页语义 + OFFSET 上界，仍属同一轮）

#### 为什么 B1 而不是 B2

原 O17 的做法是「静默钳位」：`size` 超 200 就钳到 200，照常返回 200。复核把风险拆成三个可验证的问题，结论是**钳位本身比读放大更难查**：

- 77 个钳制点 / 77 个 handler 里，53 个的 offset 由服务端推导（52 个 `page_window` + 1 个 bbs 手工），而 **24 落点 / 25 条路由把客户端 `page`/`from` 直接当 OFFSET 绑定**——钳 `size` 管不到这些落点。
- **0/77 回显实际生效的 size**；另有 **11 个端点回显客户端原始请求 size**（organization ×4、meeting ×4、message ×2、general ×1）。信封因此撒谎：客户端看到 `size=1000`，实际只拿到 200 行，再按「下一页从第 1001 行开始」算就会重复/漏行，全程零错误码。
- 前端（桌面 + 移动端）扫描 **0 处 >200**：最大 200（移动端聊天），桌面 BBS 20、IM 50、AI 20/50、考勤 50；`QueryManagerDeep` 的 500 只用于本地 `slice`；BBS 的 `rows.length >= pageSize` 判据在 20 上不受影响。改成 400 对既有前端零破坏。

三个候选里选 **B1（单一语义、可失败）** 而不是 B2（保留钳位另加 `checked_page_*` 并存）：并存语义等于让 88 个调用点继续各自挑选，钳位路径永久保留，每新增一个端点都要再判断一次；改成 `Result` 后编译器强制每个调用点适配（88 处全部 `?`），不存在「漏检」的调用点。B1 的适配成本是一次性的机械改动，并存是长期的判断成本。

- `page_size(size)` / `page_offset(offset)` / `page_window(page,size)` 返回 `Result<_, AppError>`，越界 `AppError::BadRequest`（复用既有 400 分类，不新增错误体系）。
- 88 个调用点全部补 `?`；`auth::person::list` 原返回 `Json<ActionResult<Value>>` 无法 `?`，改为 `Result<Json<...>, AppError>`。
- 11 个「钳位 + 回显原始 size」端点一并变 400；信封撒谎问题随拒绝一起消失，**没有**给 `ActionResult` 加「生效页大小」字段（不为已消失的问题扩一轮）。

- B1 适配时静态盘点又发现 **10 处内联 `size.clamp(1, 200)`**（不经过 shared helper，编译器抓不到）：`ai` ×4、`cms`、`file`、`personal`、`organization_assemble_express`、`message`、`processplatform_assemble_surface` 的 `u2_adjust_size`（后被 2 处调用）。这些同样是「静默钳位」，一并改为 `page_window` / `page_size`；对应单测从「钳位断言」改成「越界 400 断言」（`file` 的 `attachment2_limit_offset`、`processplatform` 的 `u2_adjust_size`）。改完后全仓不再有 200 上限的内联 clamp。

#### OFFSET 洞是 O17 原本没收的

原 O17 明文写的是「`page` 当 OFFSET 的 23 个旧式端点**只钳 `size`，不改既有 offset 语义**」——也就是当时只封了 `size` 侧读放大，**offset 上界是这一轮才补的**。台账不能把它记成「一开始就设计好了」。

- 范围：23 个 `processplatform_assemble_surface` 的 `*_paging_page_size_size_core`（path `page` 直接绑 OFFSET）+ `program_center::u3_bar_mass_page`（path `from` 直接绑 OFFSET，bar/foo 两条路由）= **24 落点 / 25 条路由**。
- 上界：`MAX_PAGE_OFFSET = (MAX_PAGE_NUMBER - 1) * MAX_PAGE_SIZE = 1,999,800` 行（不是早期 commit message 里误写的 1,998,000）。依据：`MAX_PAGE_NUMBER=10,000` 页 × 每页最大 200 行 ≈ 200 万行；O17 实测数据集 5 万行、余量约 40×；正常前端每页 20/50/200 对应 10 万/5 万/1 万页，5 万行 × 每页 20 只需 2,500 页、offset 49,980，离上界两个数量级。上界与 `page_window` 共用同一组常量，避免两套阈值漂移。
- 校验位置：`let page = shared::response::page_offset(page)?;` / `let from = ...` 插在取 DB 连接之前，越界请求进 DB 前即被拒。
- keyset（seek）化列为后续项，不混进本轮：它改变分页语义与前端「下一页」契约，需要单独一轮带回归。

#### 契约测试（`tests/integration_tests/scenarios/pagination_contract.rs`）

覆盖三类落点，逐条记录实际响应体（`[pagination-contract]` 日志）：

| 落点 | 请求 | 期望 | 实测 |
|------|------|------|------|
| personcard（`page_window`） | `size=1000000000` | 400 | 400，`prompt=ExceptionBadRequest` |
| personcard | `size=201` | 400 | 400 |
| personcard | `size=0` / `size=-1` | 400 | 400 |
| personcard | `size=abc` | 400 | 400（axum 路径解析失败，纯文本体） |
| personcard | `size=200` | 200 且行数 ≤200 | 200，行数 ≤200 |
| personcard | `page=1 size=20` | 200 | 200 |
| personcard | `page=1e18` | 400（offset 上界） | 400 |
| draft（`page` 即 OFFSET） | `page=1e18` / `1999801` | 400 | 400 |
| draft | `page=1999800` / `page=0` | 200 | 200 |
| draft | `size=201` | 400 | 400 |
| bar（`from` 即 OFFSET） | `from=1e18` / `1999801` | 400 | 400 |
| bar | `from=0 count=51` | 200 | 200 |
| bar | `count=201` | 400 | 400 |
| person_filter（内联 clamp 补收） | `size=201` / `size=0` | 400 | 400 |
| person_filter | `size=20` | 200 | 200 |

#### 破坏实验

| 实验 | 改动 | 红的具体断言 | 恢复后 |
|------|------|--------------|--------|
| 1. 拿掉 size 的 400 | `page_size` 改回 `Ok(size.clamp(1,200))` | `size=1e9: expected 400, left:200 right:400`，响应回显 `"size":1000000000` | 全绿 |
| 2a. 全局拿掉 offset 上界 | `page_offset` 直接 `Ok(offset)` | `page_window page=1e18: left:200 right:400`，响应 `size:200,count:0` | 全绿 |
| 2b. 只拿掉 24 个旧式落点的校验 | 删除 23+1 处 `page_offset`/`from` 校验，保留 `page_window` | `draft page=1e18: left:200 right:400`，响应 `size:0,count:0` | 全绿 |

#### 改前 / 改后数字（同一探针、同一 50k `x_org_person` 数据集、20 次请求）

| 场景 | 基线 `origin/main`（静默钳位） | B1 改后 | 变化 |
|------|-------------------------------|---------|------|
| 正常 `page=1 size=20` | p50 17.65 ms / p95 18.83 ms / 2,615 B / 0 错误 | p50 17.81 ms / p95 20.05 ms / 2,615 B / 0 错误 | 零回归（p50 +0.9%、p95 +6.5%，同 harness 噪声级） |
| `size=1e9` | p50 19.29 ms / p95 20.56 ms / 25,506 B / 20×200 | p50 4.26 ms / p95 5.00 ms / 225 B / 20×400 | 400 拦截；字节 113×↓、p50 约 4.5×↓ |
| personcard `page=1e18` | p50 27.95 ms / p95 31.08 ms / 114 B / 20×200 | p50 4.29 ms / p95 4.97 ms / 240 B / 20×400 | 400 拦截；p50 6.5×↓ |
| draft `page=1e18` | p50 6.10 ms / p95 6.52 ms / 112 B / 20×200 | p50 4.54 ms / p95 4.91 ms / 240 B / 20×400 | 400 拦截 |

口径说明：本节 harness 为预热后 20 样本，基线/改后均约 17.7 ms；上方 O17 原表的 23 ms 是 5 次冷样本的不同口径。零回归以**同一 harness 的基线 vs 改后**对比为准，不宣称复现 23 ms。

#### 「请求在进 DB 前被拒」的侧证据（pg_stat，100 次 `page=1e18`）

| | `pp_c_draft`（draft 表扫描计数） | `x_org_person`（personcard 表扫描计数） |
|---|---|---|
| 基线 | 8 → 118（delta **110**） | 67 → 169（delta **102**） |
| B1 改后 | 2 → 2（delta **0**） | 23 → 23（delta **0**） |
| 控制组 20 次正常请求 | — | delta **5**（证明计数器本身有效，0 不是「统计没刷新」） |

#### 验证

- 全新库 `oa4rust_o17_b1_check`：先 `cargo test -p shared --lib migrate::tests -- --test-threads=1` 建全表（2/2 绿），再 `cargo test --workspace --lib`（97 个测试目标全绿，0 FAILED）。
- `cargo test --test integration_runner -- --ignored --test-threads=1`：**8 passed / 16 failed**；同样的 16 条在基线 `origin/main 31238a2cb` 上**逐条同名失败**（7 passed / 16 failed），即存量问题、与本次改动无关（`bpmn work/create 405`、`cms` 非 JSON 体、`console`/`organization` 列反序列化等，属 O10 候选与存量路由/实体漂移）。新增 `pagination_contract` 是唯一增量，通过。
  - 这 16 条 DB 场景均带 `#[ignore]`，CI 的 `cargo test --test integration_runner` 只跑非 ignore 用例，因此本来就不在 CI 门禁内；本轮未改 CI 范围。
- `cargo clippy --all-targets -- -D warnings` 零告警；`cargo fmt --all --check` 绿。

### 遗留

- 每页 200 / offset 1,999,800 已成为对外行为的一部分且**失败即 400**（不再静默钳位）；确有 >200 的导出场景应开显式 export 端点，不放开通用分页。
- `size`/`page` 校验在 HTTP 层完成，但 keyset（seek）化未做（后续单独一轮）。
- **登记债务**：`IMChat.vue` 的 `page++` 从未进入请求（翻页是假的，前端既有行为），本轮不修。
- 字段类上界（超长字段/深嵌套/畸形 JSON）仍未做，保留在清单第 6 项。

## O18 集成测试长期 8/16 收口（可观测失败 + 双库分裂 + 列类型错配）

### 第一步定性（16 红，A 3 / B 12 / C 1 / D 0）

| 用例 | 失败原文首行（截断） | 第一步归类 | 依据 |
|------|----------------------|-----------|------|
| console_jpush_express_flow | `error retrieving column xvalue: error deserializing column 0` | A | `x_console_status.xvalue` 是 VARCHAR，代码读 i64 |
| org_assemble_control_flow | `error retrieving column level: deserializing column 3` | A | `x_org_unit.level/sort` 是 INTEGER，代码读 String |
| bbs_endpoints_flow | repository 层 500，无 ERROR 日志 | A→C | 加捕获式 subscriber 后无 ERROR；行落开发库 `oa4rust`，实证双库分裂 |
| query_endpoints_flow | `seeded x_query_import_record not returned` | C | 未显式种子前置，依赖残留数据 |
| 其余 12 条 | 路径/形状/token 断言过期 | B | 逐条对照实装路由与 handler |

`A>3` 未触发（A=3），但第二步复核把 5 条 B 的深层断言重新判成 A，见下。

### 第二步基础设施与断依赖验证

- `tests/integration_tests/error_capture.rs`：捕获式 `tracing_subscriber::fmt()`（ERROR 入内存 200 行），panic hook 在 panic 旁打印 `captured ERROR logs`；`init()` 幂等，`db.rs`/`db_isolated.rs`/`helpers.rs` 首行调用。没有真实 SQL 错误前不允许按猜测改。
- 双库分裂：`create_app` 会按 `DATABASE_URL` 另建 SeaORM 池，deadpool 池却指向一次性测试库；`*_core_entity` 写入落开发库。新增 `create_app_with_sea_orm(pool, sea_orm_pool, ...)`，测试用 `SELECT current_database()` 取池真实库名后注入同库 SeaORM 连接。bbs 由 A 改判 C 并转绿。
- **断依赖验证**：`DATABASE_URL=postgres://o2server:password@localhost:5432/oa4rust_no_such_db` 跑整套 `integration_runner --ignored`，结果与正常配置**完全一致**（当时 24 条基线：9 passed / 15 failed，同一批用例）。说明测试只依赖自己创建的一次性库 `oa4rust_test_<pid>`，没有偷偷依赖开发库残留数据；`db.rs` 只从 URL 解析 host/port/user/pass，CREATE/DROP 固定连 `postgres`，所以库名不存在不影响建库。
- `DATABASE_URL` 造连接的全部调用点：
  - `src/lib.rs:74` —— `create_app` 生产路径；测试已改走 `create_app_with_sea_orm` 注入，不再可达。
  - `crates/mcp_server/src/tool_bridge.rs:1384` —— `build_core_router`，仅 `main.rs` 路径使用，测试不可达。
  - `tests/integration_tests/db.rs`、`tests/integration_tests/helpers.rs`、`tests/idor_matrix.rs` —— 仅测试路径，且都指向一次性库或只取连接元信息。
  结论：生产路径保留 `DATABASE_URL`；测试路径全部显式注入，不再有第二条隐式连接。

### A 类修复证据（修前 500 / 修后 200）

| 端点 | 修前 | 修后响应体 |
|------|------|-----------|
| `GET /api/console/metric/cpu` | 500 `error retrieving column xvalue` | `{"type":"success","data":{"name":"cpu","value":42,"unit":"%"}}` |
| `GET /api/organization/unit/{id}` | 500 `error retrieving column level` | `{"type":"success","data":{"id":"...","name":"IT","level":"0","sort":"0"}}` |

### 系统性扫描（资产：`backend/scripts/scan_row_get_column_mismatch.py`，`git add -f`）

迁移按 CREATE + `ALTER ADD COLUMN` + `ALTER COLUMN TYPE` 累积（跳过 `*_rollback.sql`），取值点必须关联最近 SELECT；`try_get` 失败归 SILENT。结果 **FAIL=38 / SILENT=4 / LOOSE=10**，退出码 1。

| crate | FAIL | 主要列（声明类型→代码类型） |
|-------|------|------------------------------|
| general_assemble_control | 8 | `create_time` TIMESTAMPTZ→String ×4；`maintenance_mode/allow_registration` TEXT→bool ×4 |
| processplatform_service_processing | 6 | `x_task.start_time/end_time` TIMESTAMP→String ×4；`x_timer_job.fire_at/created_at` ×2 |
| cms_assemble_control | 3 | `path_levels` TEXT[]→String ×3 |
| message_assemble_communicate | 3 | `create_time` TIMESTAMPTZ→String ×3 |
| query_assemble_designer | 3 | `update_time` TIMESTAMP→String ×3 |
| component_core_entity | 2 | `order_number` TEXT→i32 ×2 |
| correlation | 2 | `order_number` TEXT→i32 ×2 |
| organization_assemble_control | 2 | `level/sort` INTEGER→String ×2（已修） |
| personal | 2 | `created_at` TIMESTAMP→String ×2 |
| query_assemble_surface | 2 | `x_query_view.create_time/update_time` ×2 |
| auth / console / portal_assemble_surface / processplatform_assemble_bam / program_center | 各 1 | `change_password_time`、`xuptime`、`published`、`xenabled`、`create_time` |

病因分布：时间戳 22、布尔 6、整数↔字符串 7、数组 3。已抽查 8 处均为真实错配。SILENT=4 全在 `crates/component/src/lib.rs:57/61/129/133`（`x_component.order_number TEXT` 读 `Option<i32/i64>`，`try_get` 吞错→字段恒空）。LOOSE=10 仅登记，不在本轮修。

### B 类测试修复（只改测试；每条新语义与出处）

| 用例 | 新语义 | 出处 |
|------|--------|------|
| `new_real_endpoints::portal` | `/api/portalcategory/list` 读 `x_portal.category`，返回 `{id, name, count}` | `portal/src/lib.rs:169-199` |
| `new_real_endpoints::org` | 人员列表 `data` 直接是数组（`ActionResult::success`） | `organization_assemble_control` handler |
| `new_real_endpoints::console` | `jpush hello` 的 `data` 是字符串 `"hello"` | `jpush/src/lib.rs:23-27` |
| `file_portal` | 顶层 `data` 数组 | `legacy_success` |
| `program_center` | `applications/modules_all` 顶层 `data` 数组；配置读取走 `/config/list`（无 `/config/{key}`） | `program_center/src/lib.rs:47-87`、`routes.rs:409` |
| `query_assemble` | designer/surface 列表顶层 `data` 数组；preview 契约是 `id + preview_url + html`（该端点不读 DB） | `query_assemble_surface/src/lib.rs:310-326` |
| `program_center_core_entity` | 只断言 `id + name`，无 `category=="Office"` 约束 | O14 重生成测试 `9e59942ac` |
| `cms_document` | 改用 canonical `/api/document`：create→publish→get→update→delete→list；创建固定 `status=draft`，id 服务端生成 | `cms_assemble_control/src/routes.rs` plan002 U2 段，`b63d277b8` |
| `ldap_fallback` | 登录成功以 `Set-Cookie` 为准，响应体不含 token；种子口令必须是 `{bcrypt}` 前缀的真实哈希 | `auth/src/lib.rs:53/384`；此前伪造哈希导致 login 永远失败 |
| `new_real_endpoints::ai` | `config/list/mcp` 走 `legacy_success`，顶层 `data` 即数组 | `ai_assemble_control/src/lib.rs:1028` |

`/work/create` 判定：全仓仅测试与台账出现；实装路由是 `POST /api/processplatform/service/processing/work`（`u2::work_create`，`2fe0b8212`），前端/文档 0 处引用 → **测试路径过期，不是产品漏注册**。但 bpmn 测试后续的 `agree/disagree/return/countersign` 在 backend 0 命中，属产品语义缺口，见下。

### 当前红（18 passed / 6 failed）与书面归类

| 用例 | 现在红在第几步 | 现在归哪类 | 为什么 | 谁还没定 | 下一轮处理 |
|------|----------------|-----------|--------|----------|-----------|
| `new_real_endpoints::ai_endpoints_flow` | `mcp data array` 为空 | A（接线缺陷） | 路由挂到桩 `crates/ai/src/config.rs:291-305`；真实现 `ai_assemble_control/src/lib.rs:971` 未被使用 | 已定（用户）：直接修接线 | 单独提交：改挂真实现 + 红→绿测试，并列出所有挂该桩的路由 |
| `ldap_fallback_flow` | `GET /api/ldap/config` 非 JSON | D（待证据） | 全 backend 无该路由；登录+cookie 部分已绿；parity 是否承诺未查 | 待 O20 消费者证据 | 查 `frontend/`、`mobile/`、`docs/`、OpenAPI/MCP 路由表；有承诺→A，无→C 删断言 |
| `cms_extended_crud_flow` | `POST /api/cms_assemble_control/appinfo/create` 405 | B+A（混合） | `/api/cms_assemble_control/*` 全不存在；canonical `/api/appinfo` 等存在，但 app/category/comment 无 update，`sort_order/ext_content` 不在模型 | 待 O20 消费者证据 | 逐端点查消费者：无消费者且 canonical 有→改测试；无消费者且无实现→删该断言并记录能力缺口 |
| `bpmn_process_flow` | `POST /work/create` 405 | B+A（混合） | 路径过期（B）；后续 `agree` 在 backend 0 命中（A） | 待 O20 消费者证据 | 查前端流程页实际调 canonical 还是 agree 系；canonical→改测试 |
| `bpmn_task_actions_flow` | 同 | B+A（混合） | `disagree/return` 不存在；canonical 是 `claim/complete/reject/transfer` | 待 O20 消费者证据 | 同上 |
| `bpmn_countersign_flow` | 同 | D（待证据） | 会签语义 backend 0 命中 | 待 O20 消费者证据 | 查前端/文档/parity；无消费者→C 删测试，有→A 写 O20 |

**本 PR 不修这 6 条，另开轮（O20「消费者证据判定」）。** 依据：O18 已验证的独立成果（捕获式日志、双库分裂、console/org 两个 500、10 条测试修正、断依赖验证）不应被待判定的产品问题扣住。按「A>3 先停」，本轮**没有擅自实现产品端点、也没有删测试或放宽断言**。

### 验证

- `integration_runner --ignored --test-threads=1`：**18 passed / 6 failed**（起点 8/16，16 红中 10 条已转绿；剩余 6 条全部有本文归类）。
- `cargo test --workspace --lib`：在全新库 `oa4rust_o18_lib_check`（先跑 migrate 用例建表）全绿，最后一次 **FAILED=0**。直接指向开发库 `oa4rust` 时 `shared::migrate` 因 019 历史 schema 漂移（`auth_person_identity` 旧 `person_id`）失败——存量环境问题，与本次改动无关，与 O17 记录一致。
- `cargo clippy --all-targets -- -D warnings`：零告警；`cargo fmt --all --check`：绿（已执行 `cargo fmt --all`）。

## O19 分批计划（已确认；O18 合并后开工）

前置：O18 合并后开工；每批必须回到 SQL 现场逐条确认（扫描器有「最近 SELECT 误归属」残余风险，**不许按脚本输出直接改**），每批提交「SQL 现场逐条确认记录」→ 先写一条能红的回归测试 → 再改类型 → 报「改前 500 / 改后 200 及响应体」。批次顺序按前端引用热度，唯一调整：SILENT 从最后挪到第 3 批。

排序依据：`frontend/src + mobile/src` 中对各 API 前缀的引用次数（authentication 21、processplatform 18、message 10、organization 8、query 7、portal 4、program_center 4、cms 3、general 2；component/correlation/bam/console 直接引用 0，按管理端低频处理）。

| 批次 | crate | 预计处数 | 理由 |
|------|-------|----------|------|
| 1 | auth + organization_assemble_control | 3 | 登录/组织是每条请求的前置路径，引用热度最高；org 详情 handler 已在 O18 修，**列表 handler 仍有 2 处**（见下节现场确认） |
| 2 | processplatform_service_processing + processplatform_assemble_bam | 7 | 工作流/任务页引用 18 次；x_task 起止时间 + timer job。**已收口**（见下节，另修连带 3 类：x_work.create_time、instance 5 处裸读、bam/create 的 INSERT） |
| 3 | SILENT（component `order_number`） | 4 | **静默错数据比 500 更隐蔽**：`try_get` 吞错导致字段恒空，不报错就没人发现；按用户要求挪到第 2 批之后，先于其余 FAIL 批次 |
| 4 | message_assemble_communicate + personal | 5 | IM 10 次、个人签名；时间戳同型，可批量验证同一改法 |
| 5 | query_assemble_designer + query_assemble_surface | 5 | 查询设计/展示页 7 次；`update_time/create_time` |
| 6 | portal_assemble_surface + program_center + console | 3 | 门户首页/应用中心/系统监控，热度中等但单点少 |
| 7 | general_assemble_control | 8 | 系统配置页引用仅 2 次，但含 4 个 bool 读 TEXT + 4 个时间戳，改动面最大，放中后段 |
| 8 | cms_assemble_control + component_core_entity + correlation | 7 | CMS/设计器/关联，前端直接引用 3/0/0，属长尾；含 TEXT[] 数组 3 处 |

门禁策略：扫描器**先不接 CI**（接上去就是 38 条永久红）。全部 FAIL=0 后再接门禁，并做破坏实验：手工把一个正确的 `row.get` 改成错类型 → 门禁必须红。

### O19 批次 1 收口（auth + organization_assemble_control，3 处）

**SQL 现场逐条确认**（连接 `oa4rust` 实库查 `information_schema.columns`，不采信脚本结论）：

| # | 位置 | SELECT 原文（关键列） | 列真实类型 | 代码取值类型 | 现场结论 |
|---|------|----------------------|-----------|-------------|---------|
| 1 | `crates/auth/src/switch_user.rs:49-68` | `SELECT id, unique_id, name, mobile, email, icon, job, department, unit, position, change_password_time, password_expired_time FROM auth_person WHERE unique_id = $1 AND locked = false AND deleted_at IS NULL` | `change_password_time` / `password_expired_time` 均为 `timestamp without time zone`（nullable=YES） | `Option<String>`（两列） | 错配成立；同一条 SELECT 两列同型，一并修 |
| 2 | `crates/organization_assemble_control/src/lib.rs:139`（flag=0 分支） | `SELECT id, name, parent_id, level, sort, creator, create_time::text FROM x_org_unit WHERE parent_id IS NULL ORDER BY sort ASC, create_time DESC LIMIT $1` | `level` = `integer` | `Option<String>`（第 164 行） | 错配成立 |
| 3 | 同上；另一分支 `:147`（`WHERE parent_id = $1`） | `SELECT id, name, parent_id, level, sort, creator, create_time::text FROM x_org_unit WHERE parent_id = $1 ORDER BY sort ASC, create_time DESC LIMIT $2` | `sort` = `integer` | `Option<String>`（第 168 行） | 错配成立 |

「最近 SELECT 误归属」残余风险核对：扫描器指到的不是 O18 已修的 `organization_assemble_control_unit_flag`，而是**同表同列、另一个 handler** `..._unit_list_flag_next_count`。回代码逐行确认后排除了误归属——O18 只修了详情 handler，列表 handler 确实漏改。

**固定流程证据**（真实数据路径：seed 真行 → 起真实服务 → HTTP 调用；测试见 `tests/integration_tests/scenarios/new_real_endpoints.rs` 的两个 `*_regression` 用例）：

| # | 改前（红原文 + 状态） | 改后（状态 + 响应关键字段） | 破坏实验（改回错类型） |
|---|---------------------|--------------------------|----------------------|
| 1 | `switch_user.rs:69` panic `error retrieving column change_password_time: error deserializing column 10` → panic_guard 转 **500** `{"type":"error","message":"internal server error",...}` | **200** `{"type":"success","data":{"passwordExpired":false,"person":{"unique":"switch-unique-...","name":"Switch Target",...},"roleList":[]},...}` | 回退为 `Option<String>` → 同一用例红在 column 10 |
| 2 | `lib.rs:160` panic `error retrieving column level: error deserializing column 3` → **500** 同上 | **200** 列表命中行 `{"id":"unit-list-...","level":"1","sort":"7","creator":"","createTime":"..."}` | 回退 level 为 `Option<String>` → 同一用例红在 column 3 |
| 3 | level 修好后暴露：`lib.rs:161` panic `error retrieving column sort: error deserializing column 4` → **500** 同上 | 同上响应行 `"sort":"7"` | 回退 sort 为 `Option<String>` → 同一用例红在 column 4 |

- 修复方式：按列真实类型读（`Option<chrono::NaiveDateTime>` / `Option<i32>`），再按既有响应形状回写成字符串（与 O18 已修的 `unit_flag` 一致），不改对外契约。
- 全量回归：`integration_runner --ignored --test-threads=1` → **24 passed / 1 failed**（唯一失败仍是 ldap A 类，见 O20 表 #2）。
- 扫描器复核：`FAIL=38 → 35`（本批 3 处全消），`SILENT=4` / `LOOSE=10` 未变，确认没有把别处读成新 FAIL。
- 门禁：`cargo clippy --workspace --all-targets -- -D warnings` 零告警；`cargo fmt --all --check` 绿。仅改 handler 取值类型，未动路由，生成物（openapi/mcp/parity/handler-tests）无需重生成。

### 门禁补齐：集成场景 job（2026-11-04）

**问题**：`oa4rust-ci.yml` 的 `integration-tests` job 跑的是 `cargo test --test integration_runner`，**没有 `--ignored`**；而 `integration_runner` 里全部场景用例都标了 `#[ignore]`。结论：这些用例在 CI 中**一次都没执行过**，16 条长期红灯没人发现的根因是门禁没跑到它们，不是“没人看 CI”。

**修法**（不整体跳过）：
- 新增 job `integration-scenarios`：同样的临时 Postgres service + migrations，执行 `python3 scripts/run_integration_scenarios.py`；脚本内部跑 `cargo test --test integration_runner -- --ignored --test-threads=1`。
- 唯一豁免走可读清单 `backend/tests/integration_skip_list.txt`（不是命令行散参）。
- 元守卫在 `scripts/run_integration_scenarios.py`：
  1. 条目数必须恰好 **1**（多塞一条必须改脚本常量，留下 diff）；
  2. 每条必须精确存在于 `cargo test ... -- --ignored --list` 的测试集合，**点错名字＝没排除 → 退出 1**。
- 负例实测：把条目改成 `..._typo` → 守卫报“不存在”退出 1；再加一条 `pagination_contract` → 守卫报“必须恰好 1 条”退出 1（均已还原）。
- 正向实测：清单只豁免 ldap 时 → `24 passed; 0 failed`（`--skip` 生效）。
- 真红验证：临时移出 ldap 豁免后，同一命令得到 `24 passed; 1 failed`（ldap）退出 101；已还原。PR 上的运行记录见本轮提交说明。

**ldap 为什么是唯一豁免**：A 类真缺（`GET /api/ldap/config` 无实现）+ parity 书面承诺（见 O20 表 #2）。**豁免到期条件**同该条转绿条件：① 实现端点并过 parity；② 正式撤销 parity 承诺并删除本条。到期未处理 = 守卫清单与实现状态不一致，按台账违约处理。

**计数**：本项属 O19 一轮内的“门禁补齐”，不计实质轮次；集成测试通过数仍按 `24 passed / 1 failed（ldap 本地红）` 记账，CI 口径为 `24 passed / 0 failed`（ldap 豁免）。

### O19 批次 2 收口（processplatform_service_processing + bam）

**SQL 现场逐条确认**（实库 `information_schema.columns`；`x_work` 与 `x_task` 的时间列都是 `timestamp without time zone`）：

| # | 位置 | SELECT 原文（关键列） | 列真实类型 | 代码取值类型 | 现场结论 |
|---|------|----------------------|-----------|-------------|---------|
| 1 | `processplatform_service_processing/src/lib.rs:91`（get） | `SELECT id, title, activity, activity_token, person, start_time, end_time FROM x_task WHERE work = $1` | `x_task.start_time` TIMESTAMP | `Option<String>` | 错配 |
| 2 | 同上 `:95` | 同上 | `x_task.end_time` TIMESTAMP | `Option<String>` | 错配 |
| 3 | 同上 `:136`（get 的 work 段） | `SELECT id, title, process, application, work_status, creator, create_time, start_time, end_time FROM x_work WHERE id = $1` | `x_work.start_time` TIMESTAMP | `Option<String>` | 错配 |
| 4 | 同上 `:140` | 同上 | `x_work.end_time` TIMESTAMP | `Option<String>` | 错配 |
| 5 | 同上 `:4188`（timer restore） | `SELECT id, work_id, task_id, fire_at, cron, created_at, kind, ... FROM x_timer_job` | `fire_at` / `created_at` TIMESTAMP NOT NULL | `String` + `parse_from_str` | 错配（`get` 阶段就失败，parse 永远到不了） |
| 6 | 同上 `:4189` | 同上 | 同上 | 同上 | 错配 |
| 7 | `processplatform_assemble_bam/src/lib.rs:56` | `SELECT xid, xname, xdefinition, xenabled FROM x_bam_config WHERE xid = $1` | `xenabled` = `character varying` | `Option<bool>` | 错配 |

**成串暴露（本轮额外发现并已修，同一端点内）**：
- `x_work.create_time` 在 get/instance 两个 handler 里也按 String 读（scanner 未报）→ 一并修为 `Option<NaiveDateTime>`。
- `get_process_instance` 的 5 处时间读取全部是裸 `row.get("...")`（scanner 无法推断类型，直接漏报）→ 一并修。
- `bam/create` 的 INSERT 写 `xenabled, true`（bool 字面量进 varchar 列，Postgres 不隐式转换）且漏了 NOT NULL 的 `id` 列 → 修成 `(id, xid, xname, xdefinition, xenabled) VALUES ($1,$1,$2,$3,'true')`。

**破坏实验（逐列回退 → 同一用例必红，均已还原）**：

| 回退的列 | 红原文 | 用例 |
|---|---|---|
| `x_task.start_time` | `error retrieving column start_time: error deserializing column 5` | `process_timestamp_columns_regression` |
| `x_task.end_time` | `... end_time: error deserializing column 6` | 同上 |
| `x_work.start_time` | `... start_time: error deserializing column 7` | 同上 |
| `x_work.end_time` | `... end_time: error deserializing column 8` | 同上 |
| `x_work.create_time` | `... create_time: error deserializing column 6` | 同上 |
| `x_timer_job.fire_at` | `... fire_at: error deserializing column 3` | `timer_restore_timestamp_columns_regression` |
| `x_timer_job.created_at` | `... created_at: error deserializing column 5`（实际 SELECT 列序 5，扫描器行号 4186 只是线索） | 同上 |
| `x_bam_config.xenabled` | `... xenabled: error deserializing column 2` | `bam_config_enabled_column_regression` |
| `bam/create` 旧 INSERT | 500 `{"type":"error","message":"internal server error","prompt":"com.x.base.core.project.exception.ExceptionInternal"}` | 同上 |

**改后响应体（关键字段）**：
- `processing/get`：`{"createTime":"2024-01-01 00:00:00","startTime":"2024-01-02 03:04:05","endTime":"2024-01-03 06:07:08","tasks":[{"startTime":"2024-01-02 03:04:05","endTime":"2024-01-03 06:07:08",...}]}`（instance 同形）
- `bam/get`：`{"enabled":true,...}`；`bam/create`：`{"created":true,"id":"<uuid>"}`，回读 `enabled=true`
- timer restore 直接读回：`fire_at=2024-01-02 03:04:05 created_at=2024-01-01 00:00:00`（seed 真值）

**扫描器盲区（必须记账，不许当成已清零）**：同一文件 `process_id_complex` handler 的 `:2407/:2409` 仍是裸 `row.get("start_time")/("end_time")`，对应 `x_task` TIMESTAMP，**scanner 不报**（无 turbofish 时推断不出类型）。本轮未修（不在批次 2 端点、补测试才能改），列入批次 3 之后的新增项。`x_status` 类 `to_char` 出来的 TEXT 列（如 `:2341`）是真 TEXT，不是盲区。

**其他观察（未在本轮改动）**：`TimerRegistry::restore` 目前仓库内无调用方（`routes.rs` 只调 `start_background`）。即 timer 任务的错配过去不会在生产触发，但重启后不恢复 timer 本身是另一个接线缺口，**归入 O22（批次 3 完成后）**，不在批次 3 里混做；判定与转绿条件见 O22 节。

**结果**：`run_integration_scenarios.py` → **27 passed / 0 failed**（ldap 豁免）；扫描器 `FAIL=35 → 28`，SILENT=4 / LOOSE=10 未变；fmt / clippy / 三个生成物检查全绿。

### O19 行为级守卫：种子化全路由扫描（门禁补齐，不计实质轮）

**为什么必须换判据**：批次 1/2 已实证静态扫描器认不出裸 `row.get("x")`（`get_process_instance` 5 处、`process_id_complex` 2 处整段漏报），`FAIL=28` 不是真数。根因是**这些 500 只在表里有行时才触发**：O9 的全路由扫描跑在空库上，反序列化永远不会发生。

**做法**（`backend/tests/idor_matrix.rs::seeded_route_scan_handler500_ratchet`）：
1. 独立 scratch 库 `oa_par_seeded`（每轮 DROP/CREATE + 全量迁移，与 IDOR 矩阵的 `oa_par_idor` 隔离，两测试可并行）；
2. 从 `pg_attribute` 枚举 `public` 全部 383 张表，每表 ≥1 行；按 `format_type` 生成值：时间列真实 TIMESTAMP、布尔列 `true`、数组真数组、uuid 固定值、enum 取首个 label；文本列按列名启发式给「能触发转换失败」的值（布尔名 → `'true'`、时间名 → `'2026-01-01 10:00:00'`）；主键统一给 `1`，使 `{id}` 型路由能命中种子行；
3. `session_replication_role=replica` 绕过 FK/触发器，用完恢复 `DEFAULT`（该连接回池后 app 复用，不能留 replica 语义）；
4. 复用 O9 的 `scan_routes` / `fill_params` / `fire`：全量注册路由 × 有效 Owner 身份（`GET` 先、写路由后，避免写路由销毁种子行）；`mock` 路由与 arity 家族（`KNOWN_ARTIFACT_ROUTES` 4 条）显式排除并说明；
5. **棘轮基线**（`tests/idor_seeded_handler500_baseline.txt`，2026-11-05 起）：当前 500 集合必须与基线**完全相等**——基线外的新 500 红（新类型错配当天被拦）；基线里复现不出来的条目红（已修但没删行）。条目数必须等于 `BASELINE_COUNT`（当前 10，批次 3 由 46 下调为 11，批次 4 再降为 10），修复时同 PR 删行并下调，基线只许变小。失败时按路由列出，并附本次请求 `error_capture` 捕获的真实 ERROR 串（`drain()` 做逐请求归因）。

**种子器特例（实库确认，均非豁免而是给合法值）**：
- `public.pg_class` 是业务表（不是系统目录），`to_regclass('pg_class')` 会因 pg_catalog 隐式优先解析到系统表 → 全部改为 `public."<table>"` 限定；
- `init_server_command.command` CHECK ∈ {`execute`,`stop`}；`x_query_neural_job.action` CHECK ∈ {`learn`,`generate`} → 按约束给合法值。

**与「列类型扫描器」的区别（两者保留，门禁以行为级为准）**：

| | 静态扫描器 `scan_row_get_column_mismatch.py` | 本守卫 |
|---|---|---|
| 判据 | 静态推断 SQL 列类型 vs Rust 泛型 | **行为级**：真的写行、真的发请求、真的读行 |
| 裸 `row.get` | 认不出（必须报 UNKNOWN，当前漏报） | 会命中 |
| 空库/无种子 | 仍可跑 | 必须有种子数据才有意义 |
| 结论强度 | 线索 | 门禁 |

**首次运行结果（2026-11-05，46 条入基线）**：种子 **383 表成功 / 0 失败**；全量路由扫描得 **46 条 handler500**：**35 条带 `error retrieving column ... / error deserializing column N`**（23×`create_time`、3×`xversion`、2×`file_version`、2×`asDefault`、1×`start_time`、1×`total`、1×`max_forum_count`、1×`order_number`、1×`deleted_at`），**11 条 handler 未输出任何 ERROR 日志**（O21 待定性，其中「出错却无记录」本身是观测缺陷）。**这直接证明「28 不是真数」**：静态扫描器没报的 `asDefault` / `max_forum_count` / `total` / `file_version` / `order_number` / `deleted_at` 都被抓到了。

**CI job 已接入并完成三步验收（`seeded-route-scan`，与 `integration-scenarios` 并列）**：不等清零，改棘轮基线，今天就有保护。**PR #162 三步记录（2026-11-05）**：
1. 绿：run `41878588203`（commit `be2ab0560`），`seeded-route-scan` 与 `integration-scenarios` 均 success，扫描 **10/10**；
2. 人为塞一条新 500（`x_meeting.start_time` 去掉 `::text`，commit `d63b575a9`）：run `41881217384` 的 `seeded-route-scan` **failure**，原文 `新增 / 未登记 handler500：1 条` / `column:start_time`；`GET /api/meeting/{id}` → 500，`errs=... panic=error retrieving column start_time: error deserializing column 4`；
3. 还原（commit `879913be8`）：run `41883003626` 全绿，扫描 **10/10**。
三次记录齐，状态改为**已接入 / 门禁验收通过**。

**规矩（每批引用）**：**同一条 SQL 字符串在不同 handler 里可能有不同读法，批量替换前必须逐 handler 核对读取类型，并以该 handler 的行为级回归验证。** 依据批次 3 实测：`get_process_instance` 与 `process_id_complex` 的 `x_task` SELECT 文本完全相同，但前者按 `NaiveDateTime` 读、后者按 `String` 读；批量把整段 SQL 改成 `::text` 后，前者在 `lib.rs:91` 直接红（`error deserializing column 5`）。

### O19 批次 3 收口（30 个列类型站点 + SILENT 4 处 + process_id_complex）

**范围**：种子化守卫首跑的 35 条列错配（对应 30 个唯一 panic 站点），加静态扫描器 SILENT 4 处（`component/src/lib.rs` 的两个 handler），加批次 2 记账的两处盲区（`process_id_complex` 的 `x_task.start_time/end_time`）。

**SQL 现场确认**（`oa4rust` 实库 `information_schema.columns` + 行值抽样；完整 SELECT 见各 crate 编辑）：

| 站点族 | SELECT 关键列 | 列真实类型 | 原代码取值 | 现场结论 |
|---|---|---|---|---|
| `ai` chat list completion / paging | `x_ai_clue.create_time`、`x_ai_completion.create_time` | `timestamp with time zone` | `Value::String(row.get(...))` | 错配 |
| `ai` conversation list | `x_ai_conversation.create_time` | `timestamp` | 同上 | 错配 |
| `ai` config get/list model | `x_ai_model.xasdefault`（另 `xenable` 是 boolean） | `text`，真实行存 `'true'`/`'false'` | `Option<bool>` | 错配 |
| `portal` / `portal_assemble_designer` / `portal_assemble_surface` 共 13 条路由 | `x_portal_page.create_time`、`update_time` | `timestamp` | `Option<String>` | 错配 |
| `query_assemble_surface` / `query_assemble_designer` 共 4 条路由 | `x_query_view.create_time`、`update_time` | `timestamp` | `Option<String>` | 错配 |
| `meeting` 详情 / 参会人列表 | `x_meeting.start_time/end_time/create_time`、`x_meeting_invite.create_time` | `timestamp` | `Option<String>` | 错配 |
| `message_assemble_communicate` 会话图标 / IM 配置 | `x_message_conversation_icon.create_time`、`x_message_config.create_time` | `timestamp with time zone` | `Option<String>` | 错配 |
| `attachment2` 用户容量 | `COALESCE(SUM(FILE_FILE.length),0)` | `numeric`（SUM(bigint) 返回 numeric） | `i64` | 错配 |
| `file_assemble_control` 回收站列表/详情 | `FILE_FILE.deleted_at` | `timestamp` | `Option<String>` | 错配 |
| `mind` 列表/树/版本 | legacy `mind_base_info.file_version`、`mind_folder_info.order_number`、`mind_version_info.file_version` | `bigint` | `Option<i32>` | 错配 |
| `processplatform_assemble_designer` 三个 version 端点 | `pp_e_formversion/xprocessversion/xscriptversion.xversion` | `bigint` | `Option<i32>` | 错配 |
| `bbs_assemble_control` 配置 | `x_bbs_assemble_control_config.max_forum_count` | `integer` | `i64` | 错配 |
| `component` 两个 handler（SILENT） | `x_component.order_number` | `text`（真实行存 `'1'` 数字串） | `try_get::<Option<i32>>` 后 `try_get::<Option<i64>>`，两者都静默失败 → **字段直接不存在** | SILENT |
| `process_id_complex` | `x_task.start_time/end_time` | `timestamp` | 裸 `row.get::<_, String>` | 错配（静态扫描器漏报） |

**根因归纳**：① 时间列统一改为 SQL 侧 `::text AS <列名>`，保持既有 JSON 字符串契约（与同 crate 已用 `to_char`/`dialect.cast_text` 的约定一致）；② 整数列按实库位宽修正（bigint→i64、integer→i32）；③ `SUM()` 显式 `::bigint`；④ `xasdefault` 是 TEXT 布尔字面量，SQL 侧 `(xasdefault = 'true')` 转 bool；⑤ SILENT 改为「TEXT 解析 → i64/i32 回退 → 一律输出 orderNumber（不可解析给 0）」——**任何情况下字段都在，不再静默消失**。

**先红证据（同一守卫/用例）**：
- 种子化守卫首跑：46 条 500，其中 35 条带 `error retrieving column <col>: error deserializing column N`（逐条见修订前的 `idor_seeded_handler500_baseline.txt`）。
- SILENT 用例破坏实验（把 TEXT 解析回退为 `None`）：`assertion left == right failed: order_number 为 TEXT '7'，orderNumber 必须是 7（缺失/null=静默丢字段）`。
- `process_id_complex` 破坏实验（回退 `::text`）：`error retrieving column start_time: error deserializing column 5`，路由 `/api/processplatform/service/processing/process/{id}/complex` 返回 500。

**改后响应体（关键字段有具体值）**：
- `processing/process/{id}/complex`：`"tasks":[{"activity":"act-ts","endTime":"2024-01-03 06:07:08","startTime":"2024-01-02 03:04:05","title":"Task TS"}]`
- `component/list/all`：`{"id":"component-...","name":"Seeded Component","orderNumber":7,...}`（`order_number` 存 TEXT `'7'`，输出数字 7）

**回归用例**：新增 `new_real_endpoints::component_order_number_text_column_regression`；`process_timestamp_columns_regression` 追加 complex 路由断言（防止「修 get/instance 不修 complex」再次发生）。

**破坏实验回合（均先红后还原绿）**：
| 回退点 | 红原文 | 验证对象 |
|---|---|---|
| `x_meeting.start_time` 去掉 `::text` | `error deserializing column 4`，`GET /api/meeting/{id}` 新增 500 | 棘轮守卫 |
| component TEXT 解析改为 `None` | `orderNumber` 断言失败（实际 0/缺失） | `component_order_number_text_column_regression` |
| complex `start_time/end_time` 去掉 `::text` | `error deserializing column 5`，complex 返回 500 | `process_timestamp_columns_regression` |

**结果**：棘轮守卫 **46 → 11**（`BASELINE_COUNT` 已下调为 11，基线文件仅余 11 条 O21 无日志条目）；`integration_runner --ignored --test-threads=1 --skip ldap_fallback` → **28 passed / 0 failed**；fmt 绿。

**成串暴露记录**：批次 3 第一次整套集成测试时，`process_timestamp_columns_regression` 在 `lib.rs:91` 再次红（`error deserializing column 5`）——原因是 `get_process_instance` 与 `process_id_complex` 的 SELECT 文本完全相同，批量替换误把该处也改成 `::text`，而它仍按 `NaiveDateTime` 读。已按「读法决定 SQL」逐处区分：该处保留裸时间列，complex 保留 `::text`。**这条证明批量替换必须配行为级回归，SQL 字符串相同不等于读法相同。**

### O19 批次 4（CI 门禁自证暴露的 4 条写路由 500）

**发现路径**：PR #162 的 `seeded-route-scan`（run `41872460818`）报 **14 条** = 基线 10 条复现 + **4 条新增**；同一守卫在本地干净迁移库上只报 10 条。这是门禁第一次靠 CI 与本地差异抓出本地看不见的缺陷。

**为什么本地看不见**：扫描在本地是 `GET` 全过、写路由大面积 `401`（非 admin 身份被 handler 级权限判定拒绝），写路由实际未执行；CI 的 `read_dir` 顺序不同，这 4 条在权限状态失效前被真正执行。用「扫描前先直打这 4 条」在本地复现出同样的 500，证明它们是被 401 掩盖的真实缺陷，不是 CI 环境漂移。

| 路由 | 根因（真实错误原文） | 修复 |
|---|---|---|
| `POST /api/ai_assemble_control/chat/completion` | `LIMIT $2` 被 PG 推断为 `bigint`，代码绑 `i32` → `error serializing parameter 1` | `context_window` 改 `i64`（`chat_completion` 与 `process_chat_request` 两处） |
| `POST /api/ai_assemble_control/chat/completion/stream` | `call_llm_stream` 无 `AI_API_KEY` 直接 `AppError::Internal`，与 `chat_completion` 的 canned 门控不一致 → 可选外部连接器变成硬依赖 | 无 key 时返回 canned 单 token 流（返回类型改为 boxed stream，两分支同型） |
| `PUT /api/attendance/assemble/control/dingding/attendance/list/{id}/next/{count}` | SQL 只用 `$2/$3/$4` 却绑 4 个参数 → `could not determine data type of parameter $1`（`$1` 从未被引用） | 参数编号改 `$1/$2/$3`，绑定 `[filter_user, id, count]` |
| `PUT /api/attendance/assemble/control/qywx/attendance/list/{id}/next/{count}` | 同上（同函数 `ddqy_attendance_list_next`） | 同上 |

**修复后响应体（本地种子库直打）**：completion `200` → `{"data":{"conversationId":"619686d1-...","messages":[{"content":"","role":"user"},{"content":"你好..."}]}}`；stream `200` → `event: token`；两条 list `200` → `{"data":[],"type":"success","message":"","date":"..."}`。

**基线**：`GET /api/cms/view/list/all` 在迁移后的库上不再 500（复现不出来；实体库 `oa4rust` 为空），按规则删行，`BASELINE_COUNT` 11 → **10**，未加任何新行。

**观察（进 O21 观察项，不在本批修）**：棘轮对写路由的覆盖受身份授权与 `read_dir` 顺序影响——本地与 CI 的 500 集合可能不同。后续需要：固定路由顺序（按 path 排序）、给扫描身份最小可执行权限，或对写路由单独用特权身份重扫；在此之前，「本地绿」不等于「写路由已覆盖」。

## O20 消费者证据判定（O18 六条红灯收口）

判定规则只有一条：**有没有真实消费者**。证据源三处：前端/移动端调用、文档/契约（OpenAPI / MCP 路由表 / parity corpus）、O2OA parity 书面要求。下表每条都有可复核的引用，不做「应该有」式推断。

| # | 用例 | 原红点 | 证据 | 归类 | 处置 |
|---|------|--------|------|------|------|
| 1 | `new_real_endpoints::ai_endpoints_flow` | `/api/ai/config/list` 与 `/api/ai/config/get/mcp` 恒空 / `mcp not found` | 前端 `frontend/apps/desktop/src/views/AIChatApp.vue:211`（`/api/ai/config/get/mcp/${mcpFlag}`）、`:596`（`/api/ai/config/list/mcp/paging/1/size/50`）；契约 fixture / openapi / mcp generated_routes 均列旧词路径；真实现 `crates/ai_assemble_control/src/lib.rs:971`（list）、`:794`（get）早已存在 | **接线缺陷**（非缺功能） | **已修**：旧词路由从 `ai` crate 空桩改挂 `ai_assemble_control::router`；单独提交 `fix(o20)`。同一空桩只被 2 条路由使用（list paging、get mcp），随路由一并删除，全仓无其他挂载点 |
| 2 | `new_real_endpoints::ldap_fallback_flow` | `GET /api/ldap/config` 返回 `not found`（非 JSON） | `backend/tests/parity/corpus/oa4rust-ldap_config.json`（`path=/api/ldap/config`、`method=GET`、`status=200`）+ `backend/tests/parity_runner.rs:157` 的 `("ldap_config","GET","/api/ldap/config")` | **A**（真缺） | **本 PR 不修，另开轮**。backend 全仓无此路由；红灯保留并改成可读的 A 类失败信息。**转绿条件（二者之一）**：① 有人实现该端点并过 parity corpus 断言；② parity 承诺被正式撤销——同时删 `oa4rust-ldap_config.json` 与 `parity_runner.rs:157` 条目，并把本条改为 C。**是否挡 CI**：不挡。`oa4rust-ci.yml:218` 的 integration job 跑的是 `cargo test --test integration_runner`（**无 `--ignored`**），而全部场景用例都标了 `#[ignore]`，该红灯只在本机执行 `-- --ignored` 时出现。**定期检查人**：目前无自动化任务兜底，由当轮 O 系列收口责任人本地跑一次全量 `--ignored --test-threads=1` 并在台账记结果；连续两轮未跑视为台账违约（此条即“墙纸”判定线） |
| 3 | `cms_extended::cms_extended_crud_flow`（旧前缀部分） | 6 实体全打 `/api/cms_assemble_control/...` | 前端 `frontend/packages/apis/src/index.ts` 只出现 `/api/appinfo`、`/api/categoryinfo`、`/api/comment`、`/api/categoryinfo/extContent`；全仓 `.vue` 无 `cms_assemble_control` 前缀调用；旧前缀仅存于测试与台账 | **B**（旧词 → canonical） | **已改测试**：`/api/cms_assemble_control/{appinfo,categoryinfo,comment,file,form,view}` → `/api/{appinfo,categoryinfo,comment,file,form,view}` |
| 4 | 同上（update 部分） | appinfo/categoryinfo/comment 的 update 断言 | canonical 只注册 create/get/delete(+list)，无 PUT/POST update 路由；前端无三者的 update 调用 | **C**（能力缺口，已确认无人使用） | **已删断言**，台账记缺口；file/form/view 的 update 真实存在，保留并改打 canonical |
| 5 | 同上（category 字段部分） | `sortOrder` / `extContent` 写入后读回 | canonical `categoryinfo_u2_create` 只收 `name` / `parentCategoryId` / `appId`（`cms_assemble_control/src/lib.rs:9802`），无这两个字段的写入面 | **C** | **已删断言** |
| 6 | `bpmn_process::bpmn_process_flow` / `bpmn_task_actions_flow` | `/work/create`、`/execute/{id}`、`/task/{id}/agree`（及 disagree/return） | 前端 `frontend/apps/desktop/src/views/ProcessWork.vue` 只用 `/api/work/{id}/start`、`/processing`、`/complete` 与 `/api/task/{id}/claim`、`/complete`、`/reject`、`/transfer`；契约 fixture 同样列 canonical；`/work/create` 与 agree/disagree/return 仅出现在测试里 | **B** | **已改测试**：建单改 `POST /api/processplatform/service/processing/work`，其余改 canonical；断言全部回 SQL 现场（work_status / task_status / x_record content） |
| 7 | `bpmn_process::bpmn_countersign_flow` | `agree/disagree/return/countersign` 全无 | 前端无任何会签调用；parity corpus 无 agree/countersign 条目；全仓无实现 | **C**（能力缺口，已确认无人使用） | **已删除该用例**（不留悬空红灯） |

旧词 → 新词映射（B 类落地依据）：

| 旧词（测试曾用） | canonical（前端实际调用） |
|---|---|
| `POST /api/ai/config/list` 与 `/get/mcp`（挂错 crate） | 同路径，但语义挂 `ai_assemble_control` 真实现 |
| `/api/cms_assemble_control/appinfo/*` | `/api/appinfo`、`/api/appinfo/{id}`、`/api/appinfo/list/all` |
| `/api/cms_assemble_control/categoryinfo/*` | `/api/categoryinfo`、`/api/categoryinfo/{id}` |
| `/api/cms_assemble_control/comment/*` | `/api/comment`、`/api/comment/{id}` |
| `/api/cms_assemble_control/file/*` | `/api/file`、`/api/file/{id}`、`/api/file/{id}/mockputtopost` |
| `/api/cms_assemble_control/form/*` | `/api/form`、`/api/form/{id}`（PUT 更新） |
| `/api/cms_assemble_control/view/*` | `/api/view`、`/api/view/list/all`、`/api/view/{id}`（PUT 更新） |
| `POST /work/create`、`/execute/{id}` | `POST /api/processplatform/service/processing/work`、`PUT /api/work/{id}/processing` |
| `/api/task/{id}/agree`（disagree/return 同族） | `/api/task/{id}/claim`、`/complete`、`/reject`、`/transfer/{person}` |

验证结果：`integration_runner --ignored --test-threads=1` → **24 passed / 1 failed**，唯一失败是 #2 ldap（A 类，按决定本轮不修）。O19 批次 1 新增 2 条回归用例后由 22 passed 增至 24 passed。

### O21 待定性清单：11 条「handler 500 但无任何 ERROR 日志」（只列清单，未动代码）

种子化守卫抓到 46 条 500，其中 11 条在 `error_capture` 里**一个 ERROR 都没有**。**这 11 条全部存在可观测性缺陷：错误被 `?` / `map_err(|_| AppError::Internal)` 吞掉，生产上只知道 500，不知道原因。**「出错却无任何记录」比「出错有记录」严重，故单列。以下为证据与初步归类，**待用户确认后再动**（本轮不改代码）。

| # | 路由 | handler（证据） | 行为依据 | 初步归类 |
|---|---|---|---|---|
| 1 | GET `/api/ai_assemble_control/chat/list/completion/{clueId}/paging/{page}/size/{size}` | [routes.rs:85](file:///d:/WORKSPACE/fakeToys-oa4rust/oa4rust/backend/crates/ai_assemble_control/src/routes.rs) → `lib.rs` 同名 handler | 查询列表，DB 错误 `map_err(|_| AppError::Internal)` | **C 疑似实体/列类型错配被吞**（空表不触发，种子行才有） |
| 2 | GET `/api/cms/view/list/all` | [lib.rs:83-91](file:///d:/WORKSPACE/fakeToys-oa4rust/oa4rust/backend/crates/cms_express/src/lib.rs) `cms_view::Entity::find().all()` | SeaORM 全列解码，错误被 `map_err` 吞 | **C 疑似实体/列类型错配被吞**（与 O10 同族） |
| 3 | GET `/api/mpweixin/bind/code/{code}` | [mpweixin.rs:283](file:///d:/WORKSPACE/fakeToys-oa4rust/oa4rust/backend/crates/auth/src/mpweixin.rs) | 校验 session 后 `UPDATE auth_person ... WHERE unique_id`，错误被吞 | **C 疑似 DB 错配/缺行被吞**（非外部调用，需现场确认） |
| 4 | GET `/api/mpweixin/login/code/{code}` | [mpweixin.rs:265-280](file:///d:/WORKSPACE/fakeToys-oa4rust/oa4rust/backend/crates/auth/src/mpweixin.rs) | `mpweixin_openid(&code)` 调外部微信接口，`code="1"` 必失败 | **A 外部连接器/配置缺失** |
| 5 | GET `/api/qiyeweixin/code/{code}` | [qiyeweixin.rs:132-150](file:///d:/WORKSPACE/fakeToys-oa4rust/oa4rust/backend/crates/auth/src/qiyeweixin.rs) | `exchange_code_for_userid` 外部调用 + 查 `auth_person` | **A 外部连接器** |
| 6 | GET `/api/qiyeweixin/update/person/detail/{code}` | [qiyeweixin.rs:189-241](file:///d:/WORKSPACE/fakeToys-oa4rust/oa4rust/backend/crates/auth/src/qiyeweixin.rs) | `get_user_detail` 外部调用 + 写 `auth_person` | **A 外部连接器** |
| 7 | GET `/api/welink/code/{code}` | [welink.rs:121-140](file:///d:/WORKSPACE/fakeToys-oa4rust/oa4rust/backend/crates/auth/src/welink.rs) | 外部 WeLink 登录 + 读 `auth_person` | **A 外部连接器** |
| 8 | GET `/api/zhengwudingding/code/{code}` | [zhengwudingding.rs:33-93](file:///d:/WORKSPACE/fakeToys-oa4rust/oa4rust/backend/crates/auth/src/zhengwudingding.rs) | 外部政务钉钉 API + 查 `auth_person` | **A 外部连接器** |
| 9 | GET `/api/andfx/moa/sso/token/{token}/enter/{enterId}` | [andfx.rs:41-68](file:///d:/WORKSPACE/fakeToys-oa4rust/oa4rust/backend/crates/auth/src/andfx.rs) | 解析/校验 SSO token，失败直接返回 `AppError` | **A 外部连接器** |
| 10 | GET `/api/organization/core/express/config` | [lib.rs:83-92](file:///d:/WORKSPACE/fakeToys-oa4rust/oa4rust/backend/crates/organization_core_express/src/lib.rs) `query_one(config_key='sync_enabled')` | 种子行 `config_key='seed'` → **RowNotFound** → `Internal`；若有该行还会 `get::<bool>("config_value")`（TEXT→bool 二次错配） | **B 缺行未降级（+ 潜在列错配）** |
| 11 | GET `/api/organization/core/express/status` | 同上 crate `get_status` 同款 `query_one` | 同 #10 | **B 缺行未降级（+ 潜在列错配）** |

**分轮结论**：A 类 6 条（外部连接器）需先确认“无配置时应该 200 空态还是 500”，不属批处理；B 类 2 条按「缺行返回默认/404 + 加 error 日志」修；C 类 3 条按 O19 的六步流程现场确认（很可能是被吞掉的列错配，修完记得同时删基线条目）。**上述三类都不在本轮动手。**

## 既有后续清单

1. ~~迁移并发安全~~（O1 done）。
2. ~~sea-orm/sqlx 错误分类~~（O7/O8 done）。
3. ~~限流器 key 空间洪泛~~（O11 done：50k 上界 + LRU 逐出）。
4. ~~IDOR / 越权~~（O9 done：矩阵 + 读面归属闸家族）。
5. 管理端路由鉴权穷举扫描。
6. 输入健壮性：超长字段/深嵌套/畸形 JSON 的上界缺失（O17 已收口分页 `size` / `page` / `offset` 上界，超限 400；字段长度与嵌套深度仍未做；keyset 分页化另列后续）。
7. 性能基线：真实数据量下关键路径 P95/P99。
8. 数据完整性：业务写与审计写同事务、孤儿行、级联语义。
9. **O10 候选（O9 棘轮桶，161 条 handler 级 500）**：sea-orm 实体类型与列类型系统性不匹配（实体 `Option<DateTime>` ↔ 列 TEXT、`i32` ↔ BIGINT——insert 的 RETURNING 全列解码必炸，bbs_forum_info 已实证根因）+ `{}` 体无校验直打 DB 族。逐族对齐实体定义与 migration 实态，修复一族下调棘轮基线。
10. decode panic 族残部：`row.get::<String>` 裸读可空列的存量点（O9 修了 bbs sort/hotpic application/calendar detail 三处实证点，全仓仍有同类潜伏—— arity_probe/矩阵对有数据的表会逐个暴露）。
11. ~~限流器整表写锁分片化~~（O12 判定不开轮：饱和上限 2.3 M rps，现实需求低约 2 个数量级，数据见「不开轮记录」；若未来单实例 QPS 显著上升再带基线重开）。

## O23 可诊断性（第一步：产品代码）

### 根因：为什么过去 500 一条 ERROR 都没有

1. `impl IntoResponse for AppError` **完全没有日志**：错误只在最外层 `trace_middleware` 留一条
   `warn!("server error", method, uri, status, request_id)`，**没有原始错误链**（`AppError` 的
   `Display` 大多是固定文案，`Database`/`InternalAnyhow` 的真实 cause 也没进日志）。
2. handler 里 **`.map_err(|_| AppError::Internal)` 把 source 当场丢弃**：全仓 **3895 处 / 137 个 .rs 文件**。
   错误到 `IntoResponse` 时已经只剩一个不带任何信息的 `Internal` 变体。
3. 外部连接器「没配」和「代码炸了」共用 `AppError::Internal`（500）：缺 `ANDFX_KEY` 这类部署问题
   被当成代码缺陷报出去，既误导排查，又无法自动区分可重试/不可重试。

**影响面**：种子化扫描 46 条 500 中 11 条完全无 ERROR（现基线 10 条），生产上只知道「500」，
无法回答 method/path/trace id/根因；新写的类型错配也难以当天定位。

### 修法

1. **请求上下文注入**：`trace_middleware` 把 `method/path/trace_id(x-request-id)` 放进
   `tokio::task_local! { REQUEST_CONTEXT }`，`IntoResponse` 在 5xx 时读回。
2. **500 日志**：`IntoResponse` 对 500 打一条 `ERROR`（503 打 `WARN`），字段固定为
   `method/path/trace_id/status/error/source`；响应体不变，`InternalWithSource` 对外仍是
   `"internal server error"`，不泄内部信息。
3. **不丢 source**：新增 `AppError::InternalWithSource { context, cause }` 与
   `AppError::internal_with_source(ctx, source)`（`cause` 保存 `Debug` 形态，anyhow 会带完整
   `Caused by` 链）；`context` 用 `concat!(file!(), ":", line!())` 标出出错点。
4. **缺配置 → 503**：新增 `AppError::MissingConfig { item }`，响应 `message` 写明缺哪一项。
   - 连接器：`ANDFX_KEY`/`ANDFX_ENTER_ID`、`ZWDINGDING_API_BASE`/`..._CORP_ACCESS_TOKEN`/
     `..._APP_ACCESS_TOKEN`、`QYWX_CORP_ID`/`QYWX_APP_SECRET`、`WELINK_APP_KEY`/`WELINK_APP_SECRET`、
     `MPWEXIN_APP_ID`/`MPWEXIN_APP_SECRET`；`oauth::provider_config` 同步改为 503。
   - `organization/core/express/config|status`：`config_value` 是 **TEXT**，改为 `query_opt` +
     `String` 读取 + `parse::<bool>()`。**缺行 → 503（`x_org_config.sync_enabled`）**；
     **有行但值不是 true/false → 500 并带原始值/parse 错误**（真数据缺陷，不再伪装成「缺配置」）。

### 命中数与例外（完整逐处清单见 `docs/plans/o23-internal-source-inventory.md`）

| 项 | 数量 | 处理 |
|---|---|---|
| `.map_err(|_| AppError::Internal)` | 3895 处 / 137 文件 | 机械替换为带 `file:line` + source 的构造 |
| 其中缺环境变量 | 7 处（andfx 2、zhengwudingding 3、mpweixin 2） | 改 `missing_config(<项名>)`（503） |
| 其余统一替换 | 3888 处 | `internal_with_source(concat!(file!(), ":", line!()), e)` |
| 手工补上下文 | 18 处（连接器 upstream `errcode`/缺字段、express 配置读取） | 同上，source 为说明串 |
| 保留直接 `AppError::Internal` | 125 处 / 36 文件 | 例外：测试断言；无底层错误对象的 `ok_or` 不变量；生成器脚本非运行时代码 |

### 验收与破坏实验（本地）

- 新增 `shared::error::o23_tests` 4 条：source 保留、500 不泄露、缺配置 503 且带项名、
  500 日志含 method/path/trace_id/source。
- **破坏实验 A（丢 source）**：把 `source_for_log` 对 `InternalWithSource` 返回 `"-"` →
  `five_hundred_logs_context_and_source` **FAILED**，原文
  `日志缺原始 source 链: ... ERROR ... method=GET path=/api/demo trace_id=trace-123 status=500 error=internal server error source=-`；已还原。
- **破坏实验 B（缺配置改回 500）**：把 `MissingConfig` 映射改回 500 →
  `missing_config_maps_503_and_names_item` **FAILED**，`left: 500 right: 503`；已还原。
- 本地：`cargo test -p shared o23_tests` 4/4 通过；`-p auth --lib` 100 passed；
  `-p organization_core_express` 6 passed；`cargo fmt --all --check` 绿；
  `cargo clippy --workspace --all-targets -- -D warnings` 零告警。
- 已知无关失败：`cargo test -p shared --lib` 的 `shared::migrate` 用例在本地开发库因 019
  历史 schema 漂移失败（与 O17/O18 记录同因，CI 用全新库，非本次改动引入）。
- **基线随修法下调 10 → 1**（同 PR 删行 + `BASELINE_COUNT=1`）：6 组连接器 + express
  config/status 由 500 改 503 后扫描复现不出来，按棘轮规则必须删；剩余 1 条为 ai chat list
  的 `E42P18 could not determine data type of parameter $2`（真实 SQL 参数缺陷，转绿条件见第三步）。
  本地 `seeded_route_scan_handler500_ratchet` 复跑：`种子化 383 表 / 失败 0`，
  `handler500：1 条`，`棘轮基线：1 条（当前 1 条），集合相等` → ok。
  该条 500 的捕获日志实证第一步生效：`ERROR ... method=GET path=... trace_id=<uuid> status=500
  source=DbError { code: E42P18, message: "could not determine data type of parameter $2" }`。

## O23 门禁可信度（第二步：不依赖枚举顺序）

### 计数口径（O24 收口，引用本台账前必读）

**本台账所有「handler 500 → N 条」的数字，统计口径都是「真正被执行到的路由」，不是
「枚举到的全部路由」。** 这不是同义词，差别曾让结论偏乐观：

- O9/O21 时期写路由从未真正执行（非 admin 身份 → 大面积 401，加上 logout/删会话把扫描
  token 自我注销），枚举到的写路由有 1980 条，但执行到的写路由 ≈ 0；
- 因此那段时间写的「handler 500 → 0」实际含义是「**在被执行到的路由上 → 0**」；
- O23 第二步补齐 admin 身份 + 每条写路由前恢复会话后，写路由首次真正执行，存量 500
  从 1 暴露到 40（GET 9 / 非 GET 31）。**O24 修掉探针库隔离缺陷后复测为 42**
  （GET 11 / 非 GET 31）：ORM 路由此前读的是开发库而非种子库，漏了 2 条
  `column:equipment`。引用 40 这个数字时，必须注明它是「O23 第二步当时的实测」。

**引用规则**：以后任何「500 已清零 / 只剩 N 条」的结论，必须同时给出该轮
`执行 读 x / 写 y（共 z）` 的覆盖行；只引用条数、不引用覆盖面的，一律视为口径不完整。
基线文件头与本节都保留这条说明，防止后人拿旧数字当结论。

### 改法（`backend/tests/idor_matrix.rs`）

1. **稳定排序**：`scan_routes` 结果先按 `path`、再按 `method` 排序，读段/写段都保持该顺序；
   扫描结果不再取决于 `read_dir` 枚举顺序，本地与 CI 的执行序列一致。
2. **读写分别计数 + 下限**：`read_total` / `write_total` 分别断言 ≥ `MIN_SCANNED_*`；
   本次实测 **读 2393 / 写 1980**，下限设 1500 / 1850（约 5% 余量）。
   只数总数会让「写路由从未执行」静默变绿，这是本条的修复点。
3. **扫描身份真正能写**：新增 `UID_ADMIN/TOK_ADMIN`，绑定 `role-admin`（`role-admin` 在
   migration seed 中 `name='admin' and disable=false`，满足 `is_admin` 查询）；扫描改用该身份。
   写阶段每条路由执行前 `ensure_scan_admin` 恢复「auth_person + auth_person_role + 会话」，
   因为写路由本身含 logout/删会话/改人——否则 token 自杀后余下写路由全 401。
4. **断言执行数而非枚举数**：统计 `executed_read` / `executed_write` / `executed` 并分别下限
   （1450 / 1800 / 3300）。实测 **执行 读 2393 / 写 1979（共 4372）**。
5. **401/403 显式打印 + 异常上报**：每轮打印 `401 读 x / 写 y；403 读 x / 写 y`。
   两条按设计的 401 进 `WRITE_401_ALLOWED`（逐条写理由）：
   - `POST /api/authentication/refresh`：只接受 Cookie 凭据，Bearer 按契约 401；
   - `POST /api/meeting/assemble/control/config/system/config/manage`：刻意注册在匿名可达
     router（模块前缀契约），session 不注入，handler 内 401。
   其余任何写路由 401 一律红；总 401 比例上限 20%。实测 **401 读 2 / 写 2，比例 0.1%**。
6. **类别细粒度**：`classify_error` 增补 SQLSTATE / `wrong-type` / `param-count` / `no-source`，
   同类根因可归并成族、修一族删多行；不再一律写 `no-error-log`。

### 破坏实验（写路由权限收紧 → 401）

在 `POST/PUT /api/bbs/assemble/control/update/control/config` 入口临时加入 401 返回
（`O23_FORCE_401` 环境开关，仅实验期），复跑守卫：

```text
== 扫描覆盖：枚举 读 2393 / 写 1980；执行 读 2393 / 写 1979（共 4372）；
   401 读 2 / 写 4；403 读 1 / 写 3；401 比例 0.1% ==
thread 'idor_matrix::seeded_route_scan_handler500_ratchet' panicked at tests\idor_matrix.rs:1378:9:
写路由出现未登记的 401 共 2 条（admin 扫描身份未生效，或权限被收紧）：
POST /api/bbs/assemble/control/update/control/config
PUT /api/bbs/assemble/control/update/control/config
test result: FAILED. 0 passed; 1 failed
```

预期红 → 已还原；复跑 `== 棘轮基线：40 条（当前 40 条），集合相等 ==`、
`test result: ok. 1 passed`（种子化 383 表 / 失败 0）。

### CI 验收

- 第一步：run **41890728872**（sha 87c6d44c7）全绿（unit/integration/seeded-route-scan 等）。
- 第二步：run **41894547475**（sha 555855821）全绿，其中 **`seeded-route-scan` = success**
  （写路由首次真正执行、基线 40 条集合相等、401 白名单断言通过）。
  关键 job：`unit-tests / integration-tests / seeded-route-scan / integration-scenarios / quality` 均 success。

### 基线 1 → 40（写路由第一次真正执行暴露的存量债）

写路由从未执行是「本地 10 条 / CI 14 条」的另一半根因。补 admin 后，存量 500 从 1 条
暴露到 **40 条（GET 9 / 非 GET 31）**；O24 修复探针库隔离后复测 **42 条
（GET 11 / 非 GET 31）**（下表已含新增的 `column:equipment` 2 条）。全部按族登记条件如下
（完整 key 在 `backend/tests/idor_seeded_handler500_baseline.txt`，行数==`BASELINE_COUNT`）：

| 族（reason） | 路由（方法 + 路径） | 根因 | 转绿条件 | 责任人 | 期限 |
|---|---|---|---|---|---|
| `category:wrong-type`（1，原 9） | `GET /api/organization/assemble/control/identity/{flag}/order/before/{followFlag}` | Rust 参数与列类型不一致（Int8↔i32）（bbs Int4↔i64、general Text↔bool 已在批次 7 清除；Jsonb↔String 子族已在批次 2 清除） | 按列真实类型改写绑定参数；对应路由 200 后删行并下调 `BASELINE_COUNT` | O24 数据层对齐（后端） | 2026-11-26 |
| `category:E42703`（4） | `DELETE/PUT /api/organization/assemble/control/personattribute/{flag}`；`DELETE/PUT /api/organization/assemble/control/unitattribute/{flag}` | SQL 引用不存在的列 `name` | 按 migration 实态改列名（或补列）；200 后删 4 行 | O24 数据层对齐（后端） | 2026-11-26 |
| `category:E42804`（3） | `DELETE /api/portal/assemble/designer/script/{id}`；`DELETE /api/processplatform/assemble/designer/script/{id}`；`GET /api/processplatform/service/processing/touch/mergeitem` | `deleted_at` TEXT 对 timestamp；`COALESCE` 两侧类型不一致 | 统一 timestamp 绑定/强转；200 后删 3 行 | O24 数据层对齐（后端） | 2026-11-26 |
| `category:E23502`（0，原 10） | 已全部清除（缺 id 3 条 → 批次 4；实体引用 7 条 → 批次 6） | INSERT 未生成必填 `id` / `application` / `portal_id` | — | O24 数据层对齐（后端） | 已收口 |
| `category:E42P10`（0，原 3） | 已清除（批次 8） | `ON CONFLICT` 无匹配唯一索引 | 已收口 | O24 数据层对齐（后端） | 已收口 |
| `category:param-count`（0，原 6） | 已全部清除（permission 4 条 → 批次 3；draft/start 2 条 → 批次 5） | SQL 占位符数量与绑定参数不一致 | — | O24 数据层对齐（后端） | 已收口 |
| `category:E42P18`（1） | `GET /api/ai_assemble_control/chat/list/completion/{clueId}/paging/{page}/size/{size}` | `$2` 类型无法推断 | 给 `$2` 显式 `::text`/CAST；200 后删行 | O24 数据层对齐（后端） | 2026-11-12 |
| `column:status`（1，原 2） | `GET /api/program_center/market/{flag}/installed/version` | 裸 `row.get` 把非 TEXT 列按 TEXT 解码（panic → 500） | 改 `try_get` 并统一实体/列类型；200 后删行 | O24 数据层对齐（后端） | 2026-11-12 |
| `column:equipment`（2，O24 新增） | `GET /api/meeting/core/entity/room/list`；`GET /api/meeting/core/entity/room/{id}` | SeaORM 实体 `Option<String>`/TEXT 对 JSONB 列解码不匹配 | 改 `try_get`/`Option` 类型对齐实体；200 后删 2 行 | O24 数据层对齐（后端） | 2026-11-26 |
| `category:E23503`（1） | `DELETE /api/processplatform/service/processing/work/{id}` | `x_timer_job` 外键阻断删除 | 先清子表或改 `ON DELETE CASCADE`；200 后删行 | O24 数据层对齐（后端） | 2026-12-10 |
| `category:E42601`（1） | `POST /api/document/filter/list/{page}/size/{size}/manager` | 动态 SQL 拼接 `ORDER` 语法错 | 改参数化/正确拼 ORDER BY；200 后删行 | O24 数据层对齐（后端） | 2026-12-10 |

## O23 第三步：O21 原 10 条的逐条条件

| # | 路由 | 处置/当前状态 | 转绿条件 | 责任人 | 期限 |
|---|---|---|---|---|---|
| 1 | GET `/api/ai_assemble_control/chat/list/completion/...` | 仍在基线（`category:E42P18`） | `$2` 显式类型后 200，删行并下调 `BASELINE_COUNT` | O24 数据层对齐（后端） | 2026-11-12 |
| 2-8 | andfx / mpweixin(bind,login) / qiyeweixin(code,update) / welink / zhengwudingding | **已按 A 类收口**：缺配置 → 503 `MissingConfig(项名)`；扫描不再落 500（已删行） | 部署注入对应环境变量后 200 空态/正常登录；若引入真实连接器再补一次 e2e | 部署配置责任人 + O23 收口人 | 2026-11-12 |
| 9-10 | GET `/api/organization/core/express/config`、`/status` | **已按 B 类收口**：缺 `x_org_config.sync_enabled` → 503；有行按 TEXT→`parse::<bool>`（坏值 500 带原始值） | 种子/迁移补 `sync_enabled` 行后 200；坏值分支由 500 日志断言守护 | O24 数据层对齐（后端） | 2026-11-12 |

**纪律**：基线是「已登记的技术债」，不是「有基线所以能绿」。新增任何基线路必须同时
在这两张表登记 转绿条件/责任人/期限；没有到期条件的行不许加。

## O24 防退化（语义自证 + 期限机检）

### O24-1 3895 处机械替换语义未变（证据）

完整证据（方法、逐类逐字对照、日志原文、14 处 `missing_config` 逐条）：
[`docs/plans/o24-mechanical-replacement-evidence.md`](./o24-mechanical-replacement-evidence.md)。

同一 harness、同一 scratch 库、同一份种子（383/383），改前 `e2e9beec5` / 改后工作树：

| 断言 | 结果 |
|---|---|
| 唯一路由键 | 4355 / 4355，集合完全相同（`only_before=0`、`only_after=0`） |
| 状态码分布 | 除 500 51→42、503 7→16 外，200/307/400/401/403/404/405/422 **逐项相等** |
| 4xx→5xx / 5xx→4xx | **0 条 / 0 条** |
| 状态变化 | 9 条，全部 500→503（全部缺配置；`prompt` 不变，仅 `message` 变为 `service not configured: missing <项>`） |
| 同状态 body 差异 | 642 条，**全部 200**；641 条 JSON 的 key 集合 / `type` / `message` / `prompt` / `count`/`size`/`position`/`spent` 逐字相同，1 条 SSE 仅 `conversationId` 不同 |
| source 保全 | 500 日志含 `file:line` + `ColumnDecode` 完整链；503 日志含 `MissingConfig` 的 `file:line`（两行原文见证据文档第 5 节） |
| 409 | 本仓无 409 生产者，**不可达**，不伪造样本 |

**发现（先报告，未自行裁定）**：`organization/core/express` 的 `/config`、`/status` 不只是
换措辞——改前 `config_value`（TEXT）被直接按 `bool` 解码，**有合法行时也会 500**；改后
按 `String` 读再 `parse::<bool>()`，有合法行时是 200。这是 O23 台账已写明的设计内修复，
但确属成功路径上的行为变化；本地种子库没有 `sync_enabled` 行，扫描只观测到 503 分支。
**缺口**：目前没有「有 `sync_enabled` 行」的集成用例钉住 200 分支，列入后续。

`missing_config` 实际落点 **14 处**（用户口径 12 处是 O23 早期统计；后续把
`ANDFX_ENTER_ID` 与 `ZWDINGDING_APP_ACCESS_TOKEN` 也纳入）。逐处改前/改后见证据文档第 7 节。

### O24-2 期限由机器执行（`backend/tests/idor_matrix.rs`）

基线文件改为 **5 字段 TAB**：`<METHOD> <route>\t<reason>\towner:<责任人>\tdue:YYYY-MM-DD\tcond:<转绿条件>`。

门禁顺序（任何一项失败即 FAIL，不继续）：

1. 解析 → **下限断言**：解析行数 ≥ 当前实际 500 条数（防解析器静默失效后「0 行 / 0 条」假绿）；
2. 元数据：每行必须有 `METHOD`+路径、`column:`/`category:` reason、`owner`、`cond`、
   `due` 合法 `YYYY-MM-DD`；
3. **到期**：以 UTC+8 业务日判断，`today > due` 且该行仍在基线 → 报出
   `行 | due | 超期 N 天 | 责任人 | 原定转绿条件`；
4. 去重；行数 == `BASELINE_COUNT`；棘轮集合相等。

破坏实验（全部实际执行）：

| 实验 | 预期 | 实际 |
|---|---|---|
| `E42P18` 行 due 改 `2026-11-04` | 红 | `基线中有 1 行已超过期限仍在基线（今天 2026-11-05，UTC+8）：GET /api/ai_assemble_control/chat/... | due 2026-11-04 | 超期 1 天 | 责任人 O24数据层对齐(后端) | 原定转绿条件：给 $2 显式 ::text/CAST；200 后删行` |
| 删 `GET /api/meeting/core/entity/room/list` 的 `owner:` | 红 | `基线条目格式/元数据错误：缺少 owner（责任人）: GET /api/meeting/core/entity/room/list` |
| 复原 | 绿 | `== 棘轮基线：42 条（当前 42 条），集合相等 ==`，`test result: ok` |

### 计数口径校准：基线 40 → 42

O24 修掉探针的**库隔离缺陷**（`create_app` 用进程级 `DATABASE_URL` 建 SeaORM 连接，
ORM 路由读的是本机开发库而非 scratch 种子库，导致基线随开发库垃圾行漂移：本地 41 / CI 40）。
改为 `create_app_with_sea_orm` + `create_sea_orm_pool_with_url(scratch)` 后，
`meeting_core_entity` 两条 ORM 路由首次读到种子行，暴露 `column:equipment` 500：

```text
GET /api/meeting/core/entity/room/list   column:equipment
GET /api/meeting/core/entity/room/{id}   column:equipment
```

基线因此 **40 → 42（GET 11 / 非 GET 31）**，`BASELINE_COUNT=42`，集合相等。CI 的旧 40
不是「更干净」，而是**扫描器当时没读到种子库**——这也是本次把口径写进台账的原因。

### O24-3 待修 0 条（批次 1-16 已清 42）

**口径**：非 GET 0 条 + GET 0 条 = 0（原 42；批次 1 清 1、批次 2 清 4、批次 3 清 4、批次 4 清 3、
批次 5 清 2、批次 6 清 7、批次 7 清 4、批次 8 清 3、批次 9 清 4、批次 10 清 3、批次 11 清 1、
批次 12 清 1、批次 13 清 1、批次 14 清 2、批次 15 清 1、批次 16 清 1）。

**注意**：基线归零 ≠ 这个族「完成」——仍有 7 条端点因为缺实体引用参数被改成 **400（缺参数）**，
功能对用户依旧不可用，见 [`o24-3-feature-still-unavailable.md`](./o24-3-feature-still-unavailable.md)。
2026-11-05 查证：hotpic 5 条无真实 `default`/`0` 键，登记为「未完成功能（无后端支撑的空壳）」；
portal page 2 条因 `x_portal` 零行、前端写死 `portal/default` 本身错误而停止修复。

**已执行的排序（按真实用户后果，不按条数）**：

1. 写路径里「会写库但报错」的族（用户以为失败、实际可能已落库）优先；
2. 再按条数多的可批量验证族；
3. GET 留最后（批次 13-16 即这一段）。

每批固定流程：`information_schema` 现场确认真实列类型 → **先红**（复现当前 500）→ 改 →
行为级响应体断言 → 破坏实验 → **单独提交** → **删对应基线行并下调 `BASELINE_COUNT`**。
禁止「修好了但基线行留着」，也禁止一次性全清。

**筛序规矩（批次 1 判据，每批先扫一遍）**：写路径里先找「**写语句已提交、错误发生在提交之后**」
的族——响应是 5xx，但数据已经落库；用户重试会造成重复写（唯一性冲突、双份记录、状态错乱）。
判据：handler 里能指出「某条写 SQL 成功返回后，仍执行了会失败的语句（解码 panic / 二次校验 /
后续写）」。这类永远排在最前，优先级高于任何按条数聚合的族。批次 1 的 portal surface publish
就是基线里唯一一条（UPDATE 已提交 → SELECT 解码 panic）。

### O24-3 批次 1：portal surface publish（唯一「先落库、后 500」的基线项）

**族判定**：逐条读非 GET 31 条的 handler，只有 `POST /api/portal/assemble/surface/publish/{id}`
满足「写已提交、随后才报错」。其余写路由的 UPDATE/INSERT 本身即失败（ToSql/参数/约束错误，
事务未落库），不属于本族。判定依据：

```text
UPDATE x_portal_surface SET published = true, published_at = NOW(), update_time = NOW() WHERE id = $1
  → 成功（published 已落库）
SELECT id, published, published_at ...            → 把 BOOLEAN published 按 Option<String> 解码
  → ColumnDecode panic → 500（source=-，panic_guard 兜底）
```

用户视角：响应 500「发布失败」，但数据实际已发布——重试会重复发布，前端的状态刷新与实际不一致。

**SQL 现场确认**（`information_schema.columns`，测试库现场）：

```text
id | text | YES |    published | boolean | YES |    published_at | text | YES |    update_time | text | YES |
```

**先红**：新增 `tests/integration_tests/scenarios/portal_surface_publish.rs`（真实 HTTP +
真实 PostgreSQL，`integration_runner` 独立 `#[ignore]` 用例 `portal_surface_publish_contract`）：

1. 预置 `x_portal_surface.published=false` → POST publish → **200**，断言
   `type=success`、`data.id`、`data.published == bool true`、`data.publishedAt` 非空，
   并回库断言 `published=true`、`published_at` 非空（同时钉住「不能 500 后已落库」）；
2. id 不存在 → 200 `type=error` / `message=surface not found`，回库断言无幽灵行。

未改代码时第 1 条实测红：`500 Internal Server Error`，日志
`panic_guard: handler panicked; ... error retrieving column published: error deserializing column 1`。

**改**：`published` 按真实类型 `Option<bool>` 解码，响应值改为 `Value::Bool`
（与 `surface_list` 的 `Option<bool>` 口径一致）。`published_at` 真实类型是 TEXT，保持原样。

**破坏实验**：把 `Option<bool>` 改回 `Option<String>` → 第 1 条红（500 / panic
`error retrieving column published`）；复原后绿。

**收口**：删基线路 `POST /api/portal/assemble/surface/publish/{id}  column:published`，
`BASELINE_COUNT 42 → 41`；门禁实测 `== 棘轮基线：41 条（当前 41 条），集合相等 ==`
（覆盖行 `读 2393 / 写 1980；执行 读 2393 / 写 1979（共 4372）`）。

### O24-3 批次 2：JSONB 参数绑定族（4 条，3 个 handler）

**族判定**：按错误类别 `wrong-type` 的 9 条里，按「列/参数真实类型」再分族。本批是
Jsonb↔String 子族：SQL 里 `$n::jsonb` 或 `jsonb_set(..., $2::jsonb, ...)` 把参数类型
**推断为 JSONB**，而 Rust 侧绑定 `String`/`Option<String>` → `ToSql` 失败
（`WrongType { postgres: Jsonb, rust: "..." }`）。共 4 条基线、3 个 handler
（queryview 两条路由共用 `table_tableFlag_row_id_part_update`）。

**SQL 现场确认**（`information_schema.columns`，测试库现场）：

```text
x_meeting_room.equipment        | jsonb
x_process_surface.content       | text      ← 并不是 jsonb，旧 SQL 的 ::jsonb 只是加在参数上
x_query_table_data.data         | text      ← 同上
```

**先红**：新增 `tests/integration_tests/scenarios/jsonb_param_binding.rs`
（`integration_runner` 独立 `#[ignore]` 用例 `jsonb_param_binding_contract`），逐条真实 HTTP：
会议室保存 `equipment` 数组 → 200 且回库 `equipment::text` 等于 JSON；过程表面保存
payload → 200 且 `data.content` 原样回显、回库内容等于 payload；queryview 两条路由各一次
局部更新 → 200 `saved=1` 且回库 `data::jsonb->'/part'` 等于 payload。
未改代码时实测红（第一条即 500，日志
`source=crates\meeting_core_entity\src\lib.rs:320: ... ToSql(4) ... WrongType { postgres: Jsonb, rust: "Option<String>" }`）。

**改**：让参数侧保持文本，写入仍产生合法 JSON：

| handler | 旧 | 新 |
|---|---|---|
| `meeting_core_entity::update_room` | `equipment = $5` | `equipment = $5::text::jsonb` |
| `processplatform_assemble_surface::save_surface` | `content = $1::jsonb` | `content = $1::text`（列本身是 TEXT） |
| `query_assemble_surface::table_tableFlag_row_id_part_update` | `ARRAY[$1], $2::jsonb` | `ARRAY[$1], $2::text::jsonb` |

**破坏实验**（逐个复原，证明 3 个 handler 各自被钉住）：
① 三处全复原 → 第 1 条红（meeting 500）；② 只复原后两处 → 第 2 条红（processplatform
`ToSql(0) Jsonb vs String`）；③ 只复原 queryview → 第 3 条红（`ToSql(1)`）；④ 全部复原修复 → 全绿。

**收口**：删 4 条基线路，`BASELINE_COUNT 41 → 37`；门禁实测
`== 棘轮基线：37 条（当前 37 条），集合相等 ==`。

### O24-3 批次 3：查询设计器 permission 族（4 条，同一 handler 工厂）

**族判定**：`POST /designer/{query,stat,table,view}/.../permission` 四条路由都走
`set_permission_generic`，共用同一条编译期 `predicate`（如 `(flag = $1 OR id = $1)`）：

```text
守卫 SELECT: SELECT owner FROM {table} WHERE (flag = $1 OR id = $1)      ← 只绑 flag ✓
UPDATE     : UPDATE {table} SET permission = $1, update_time = ... WHERE (flag = $1 OR id = $1)
             ↑ 绑定 [permission, flag]，$1 已被 permission 占用 → 2 绑定对 1 占位符
```

**先红**：新增 `tests/integration_tests/scenarios/query_permission_binding.rs`
（`integration_runner` 独立 `#[ignore]` 用例 `query_permission_binding_contract`），
四条路由逐条真实 HTTP + 真实库：200 `type=success`、`data.id` 等于路径键、
`permissionSet=true`，回库 `permission` 等于 `permissionList` 的序列化文本且
`update_time::text` 非空。未改代码时第 1 条实测红：
`source=crates\query_assemble_designer\src\u2_closures.rs:2036: Error { kind: Parameters(2, 1) }`。

**改**：仅 UPDATE 侧拼接谓词时确定性替换 `$1` → `$2`
（谓词是编译期常量，无注入面；守卫 SELECT 保持原样）。

**破坏实验**：把 `resource.predicate.replace("$1", "$2")` 还原为 `resource.predicate`
→ 第 1 条红（500 / `Parameters(2, 1)`）；复原后四条全绿。

**收口**：删 4 条基线路，`BASELINE_COUNT 37 → 33`；门禁实测
`== 棘轮基线：33 条（当前 33 条），集合相等 ==`。

### O24-3 批次 4：INSERT 未写必填主键/规范列的「遗留给定」族（3 条 E23502）

**族判定**：三条 handler 都按遗留 `x*` 列写 INSERT，但表的 NOT NULL 列是规范列。全是
「写语句本身失败」，不属于「先落库后 500」族。

**SQL 现场确认**（`information_schema.columns`，测试库现场）：

```text
x_console_message : id varchar NOT NULL；另有遗留 xid
x_cms_viewrecord  : id varchar NOT NULL, doc_id varchar NOT NULL
x_msg_message     : id/title/type/consumer/is_read NOT NULL；另有遗留 xid/xtitle/...
```

**先红**：新增 `tests/integration_tests/scenarios/insert_required_id.rs`
（`integration_runner` 独立 `#[ignore]` 用例 `insert_required_id_contract`）：
console 发消息 → 200 `sent=true` 且回库 `id` 非空；message 自定义创建 → 200 `created=true`、
`data.id` 回库一致且 title/type/consumer/is_read 落库；document 浏览记录 → 200 `docId` 回显
且回库 `id` 非空。未改代码时首条实测红：
`source=crates\console\src\lib.rs:189: E23502 null value in column "id" of relation "x_console_message"`。

**改**（按列实态补写，遗留列同值保留）：

| handler | 旧 INSERT | 新 INSERT |
|---|---|---|
| `console::send_message` | `(xid, xtoken, xmessage)` | `(id, xid, xtoken, xmessage)`，id=xid=uuid |
| `cms::document_cipher_id_persist_view_record` | `(doc_id, view_id, record_data, person_id)` | 增加 `id = uuid` |
| `message::custom_create` | `(xid, xtitle, xbody, xtype, xconsumed)` | 增加 `(id, title, type, consumer, is_read)`；id/xid 因 varchar/text 类型不同绑两个占位符（否则 E42P08） |

**破坏实验**（逐个复原，证明三条各自被钉住）：仅还原 message → 第 2 条红（E23502 `x_msg_message.id`）；
仅还原 cms → 第 3 条红（E23502 `x_cms_viewrecord.id`）；全复原（先红阶段）→ 第 1 条红。

**收口**：删 3 条基线路，`BASELINE_COUNT 33 → 30`；门禁实测
`== 棘轮基线：30 条（当前 30 条），集合相等 ==`。

### O24-3 批次 5：`draft/start` 写 `xstatus='started'`（裁定执行，2 条）

**裁定**：不删路由、不降级（不改写成只 touch `xupdateTime`），写 `'started'`。

**xstatus 取证（裁定依据，三项证据）**：

| 证据 | 结论 | 来源 |
|---|---|---|
| 无 CHECK | 该表无任何检查约束 | `pg_constraint`（`information_schema` 现场） |
| 无 DEFAULT | `column_default` 为空、列可空 | `information_schema.columns` → `pp_c_draft.xstatus text` |
| 无读方 | 全仓无读取 `pp_c_draft.xstatus` 做状态判断 | 后端 grep：`xstatus` 读取只属 `x_console_status`/`x_express_info`/`PP_E_PROCESS`/`PP_C_JOB`；前端 `apps/desktop/src/views/ProcessManagerApp.vue:584` 仅 POST + toast「草稿已发起」，不读状态；parity 只断言路由存在 |

**契约项（必须遵守）**：`pp_c_draft.xstatus` 当前只有写方、无读方，值集暂为 `{started}`。
将来若引入状态词表，**必须迁移已存在的 `started` 行**，不许新旧值混用。字面量收进唯一写方
常量 `DRAFT_STARTED_STATUS`（`crates/processplatform_assemble_surface/src/lib.rs`），注释内嵌上述三项证据。

**根因**：`UPDATE PP_C_DRAFT SET xstatus = $1, "xupdateTime" = NOW() WHERE xid = $2`
只绑了 `&[&id]` 一个参数 → `Parameters(1, 2)`，500。

**先红**：`tests/integration_tests/scenarios/draft_start_status.rs`
（`integration_runner` 独立 `#[ignore]` 用例 `draft_start_status_contract`）；未改代码时实测红：
`source=crates\processplatform_assemble_surface\src\lib.rs:7723: Error { kind: Parameters(1, 2) }`，
断言文案「draft/start 应为 200（旧实现 SET xstatus = $1 只绑 id → Parameters 失配），实际 500」。

**行为断言**（POST + GET 共用 handler，两路都跑）：

- POST 返回 **200**、`type=success`、`data.id == seed id`、`data.updateTime` 非空（前端 toast 语义不变）；
- GET 同路由返回 200 且回显非空 `createTime/updateTime`；
- 回库校验 `xstatus = 'started'` 且 `xupdateTime` 已更新（种子行 `xupdateTime=''` → 非空）。

**破坏实验**：把绑定退回 `&[&id]` → 测试红（`Parameters(1, 2)`，日志 500）；复原 → 绿。

**收口**：删 2 条基线路，`BASELINE_COUNT 30 → 28`；门禁实测
`== 棘轮基线：28 条（当前 28 条），集合相等 ==`。

### O24-3 批次 6：E23502 实体引用族（hotpic + portal page，7 条）

**裁定口径**：`hotpic.application` / `info_id`、`portal page.portal_id` 是**实体引用**，不是
状态值。先查请求上下文（路径 / body / 父资源）是否携带该实体标识；能推导就绑定并断言落库值
指向正确父实体；推导不出来一律 **400（缺必要参数）**，不许编造合法 id。

**信息来源证据（逐条）**：

| 路由 | 实体引用 | 来源 | 结论 |
|---|---|---|---|
| `POST /api/hotpic/create/hotpic` | `application` + `info_id` | body 字段 `application` / `infoId`（`HotpicRequest`） | 前端 `HotpicApp.vue` 只发 `{title}` → 无来源，缺参 400；若 body 提供则绑定 |
| `POST /api/hotpic/assemble/control/user/hotpic` | 同上 | 同上（共用 `create_hotpic`） | 同上 |
| `GET /api/hotpic_assemble_control/create/hotpic` | 同上 | 同上（GET 变体共用 handler，扫描器发 `{}`） | 400 |
| `POST /api/hotpic/assemble/control/update/control/config` | `application` + `info_id` | config body 的 `application` / `infoId` | 前端 `HotpicApp.vue` 发 `{}` → 400 |
| `GET /api/hotpic_assemble_control/update/control/config` | 同上 | 同上（共用 `update_control_config`） | 400 |
| `POST /api/portal/assemble/designer/page` | `portal_id` | body 字段 `portalId`（与同 crate `create_widget` 取参口径一致） | 前端 `PortalPageDesignerApp.vue` 只发 `{name,category,content}` → 400 |
| `POST /api/portal/assemble/designer/page/create` | 同上 | 同上（共用 `create_page`） | 400 |

**schema 现场证据**：`x_hotpic.application` / `info_id` 与 `x_portal_page.portal_id` 均
**NOT NULL、无 DEFAULT、无 CHECK、无外键**（`information_schema.columns` + `pg_constraint`）；
表内已有 `application=''` 的历史脏数据 42 行，但不能作为「父实体引用」的来源。
`x_portal_page.status` 亦为 NOT NULL 且无 DEFAULT，沿用同表既有写方
`portal_core_entity::page_update` 的默认值 `"active"`（非新造词表）。

**实现**：

- `HotpicRequest` 增 `application` / `infoId` 两个可选字段；`create_hotpic` 与
  `update_control_config` 缺任一项 → `AppError::BadRequest("missing required parameter: application/infoId")`；
  INSERT 补 `application` / `info_id` 列（有来源时绑定真实值）。
- `CreatePageRequest` 增 `portalId`；`create_page` 缺失 → `BadRequest("...portalId")`；
  INSERT 补 `portal_id` / `status` 列。

**先红**：`entity_reference_required_contract`（`tests/integration_tests/scenarios/entity_reference_required.rs`）
未改代码时实测红：`== 500 Internal Server Error ==`，日志
`E23502 null value in column "application" of relation "x_hotpic"` /
`E23502 null value in column "portal_id" of relation "x_portal_page"`。

**行为断言**：7 条全覆盖 —— 缺参路径 400 + `type=error` + 文案点名缺失参数；有参路径 200
+ 回库 `application`/`info_id`、`portal_id` 等于 body 提供的父实体 id（`status='active'`）；
GET 变体同样 400。

**破坏实验**（逐个还原，证明三处各自被钉住）：

- 只还原 `create_hotpic` → 第 1 条红（500，E23502 `application`）；
- 只还原 `update_control_config` → 第 3 条红（500，E23502 `application`）；
- 只还原 `create_page` → 第 6 条红（500，E23502 `portal_id`）。

**收口**：删 7 条基线路，`BASELINE_COUNT 28 → 21`；门禁实测
`== 棘轮基线：21 条（当前 21 条），集合相等 ==`；`E23502` 家族清零。

### O24-3 后续批次的实体引用口径（沿用批次 6）

凡 `E23502` 命中实体引用列：先找请求上下文来源（路径 / body / 父资源）；有来源就绑定并断言
落库指向正确父实体；没有来源一律 400 + 台账写「信息来源」证据，不编造合法 id。

### O24-3 批次 7：`wrong-type` 配置写路径（bbs + general，4 条）

**schema 现场证据**（`information_schema.columns`，`oa4rust_test`）：

| 列 | 实际类型 | DEFAULT / CHECK | 旧绑定 | 结论 |
|---|---|---|---|---|
| `x_bbs_assemble_control_config.max_forum_count` | `integer` | `1000` / 无 | `i64` | 按列类型收窄 `i32` |
| `x_general_assemble_control_config.maintenance_mode` | `text` | 无 / 无 | `bool` | 写入 `$1::bool::text`，读取 `::bool` |
| `x_general_assemble_control_config.allow_registration` | `text` | 无 / 无 | `bool` | 同上 |

`x_general_assemble_control_config` 当时 **0 行**；写入契约统一为 Postgres bool 字面量
`true`/`false`，读路径 `::bool` 强转。**契约项**：该表若将来出现非 `true`/`false` 的历史值，
`::bool` 会报错，必须先迁移数据，不许新旧混用。

**根因**：`ToSql(1) WrongType { postgres: Int4, rust: "i64" }`（bbs）；
`ToSql(0) WrongType { postgres: Text, rust: "bool" }`（general）。

**先红**：`wrong_type_config_contract`（`tests/integration_tests/scenarios/wrong_type_config.rs`）
未改代码时实测红：bbs POST 500（WrongType 500）；断言文案「bbs POST 应为 200…实际 500」。

**行为断言**：bbs POST/PUT 各一次 → 200 + 回库 `enabled`/`max_forum_count`/`allow_anonymous`；
general POST/PUT 各一次 → 200 + 回库 `maintenance_mode`/`allow_registration` 为 `true`/`false`
文本；写入后 GET `/api/general/assemble/control/status` 必须 200 并解出相同布尔（读 `::bool` 契约）。

**破坏实验**（逐个还原，三处各红）：还原 bbs → i64 红；还原 general UPDATE 去掉 `::bool::text` → 红；
还原 general SELECT 去掉 `::bool` → 读路径红。

**收口**：删 4 条基线路，`BASELINE_COUNT 21 → 17`；门禁实测
`== 棘轮基线：17 条（当前 17 条），集合相等 ==`。

### O24-3 批次 8：`E42P10` upsert 路径（attendance + calendar，3 条）

**schema 现场证据**：

| 表 | 唯一约束 | 旧 SQL 冲突目标 | 结论 |
|---|---|---|---|
| `x_attendance_statistic` | 仅 `id` 主键 | `(person_id, year, month)` | E42P10 + 缺 `id` → 确定性 id + `ON CONFLICT (id)` |
| `cal_control_config` | 仅 `id` 主键 | `(config_key)` | E42P10 + 缺 `id` → UPDATE-then-INSERT 单行幂等 |

两表 `create_time`/`update_time` 均为 **text**，`NOW()` 不能隐式赋值 → 统一 `NOW()::text`
（否则即使修好冲突目标也会转成 E42804）。

- attendance id = `person_id-year-month`（同键同值，可重复 upsert）；无读取方解析 id。
- calendar 用 `WHERE config_key='global'` 的 UPDATE；影响 0 行才 INSERT（uuid id）。

**先红**：`upsert_paths_contract`（`tests/.../upsert_paths.rs`）未改代码时实测红：
attendance POST 500（E42P10），断言文案「应为 200…实际 500」。

**行为断言**：attendance POST 两次 → 200、同键**仍 1 行**、`update_time` 非空（写后必读）；
GET 变体 200；calendar 写两次 → 200、`config_key='global'` **仍 1 行**、读回 `config_value`
与写入同值（写后必读）。

**破坏实验**：还原 attendance 冲突目标 → 红；还原 calendar 为 `ON CONFLICT (config_key)` → 红。

**收口**：删 3 条基线路，`BASELINE_COUNT 17 → 14`；门禁实测
`== 棘轮基线：14 条（当前 14 条），集合相等 ==`。

### O24-3 批次 9：`E42703` 属性表路径（person/unit attribute，4 条）

**schema 现场证据**（`information_schema.columns`）：

```text
x_org_person_attribute | id, person_id, attribute_key, attribute_value, creator, create_time, deleted_at
x_org_unit_attribute   | id, unit_id,  attribute_key, attribute_value, creator, create_time, deleted_at
```

两张表**没有 `name` 列**，旧实现用 `resolve_generic_id`（`id = $1 OR name = $1`）
→ `E42703 column "name" does not exist`。这两张表的路径参数本来就是**记录 id**
（GET 变体同表同口径也按 id 查），不是 name。修法：

- 新增 `resolve_id_exact` / `soft_delete_exact`（`u2_helpers.rs`），只按 `id` 精确解析；
- `u2_misc.rs` 四处切换：`person_attribute_edit/delete`、`unit_attribute_edit/delete`。
- 同文件其余 `resolve_generic_id`/`soft_delete_generic` 仍用于 `x_org_unit` /
  `x_org_permission_setting` / `x_org_personcard`（这些表有 `name`），不受影响。

**先红**：`attribute_paths_contract`（`tests/.../attribute_paths.rs`）未改代码时实测
`500` + `SqlState(E42703) column "name" does not exist`（`u2_helpers.rs:150`）。

**行为断言**（每条写后必读）：person PUT → 200 + 回库 `attribute_value='new'`；
person DELETE → 200 + `deleted_at` 非空，且已软删记录再 PUT → 200 + `type=error`
（该 handler 的「未命中」走 o2server 旧语义，不是 5xx）；unit PUT/DELETE 同构。

**破坏实验**：单独还原 person edit 的解析 → 红（E42703）；单独还原 unit delete 的软删 → 红
（E42703）；各自复原后绿。

**收口**：删 4 条基线路，`BASELINE_COUNT 14 → 10`；门禁实测
`== 棘轮基线：10 条（当前 10 条），集合相等 ==`。

### O24-3 批次 10：`E42804` 脚本软删 + mergeitem（3 条）

**schema 现场证据**（`information_schema.columns`）：

| 表.列 | 实列类型 | 旧写法 | 结论 |
|---|---|---|---|
| `x_portal_script.deleted_at` | timestamp | `to_char(NOW(), ...)`（text） | 改 `NOW()` |
| `PP_E_SCRIPT.deleted_at` | timestamp | `to_char(NOW(), ...)`（text） | 改 `NOW()` |
| `x_attachment.create_time` | timestamp | `COALESCE(create_time, id)`（timestamp vs varchar） | 改 `COALESCE(..., TIMESTAMP 'epoch')` |

mergeitem 的 NULL 语义：`create_time` 为 NULL 时按「最旧」参与去重（保留最早创建的行），
与既有「保留最早」行为一致。

**先红**：`script_delete_paths_contract`（`tests/.../script_delete_paths.rs`）未改代码时
实测 500 + `E42804 column "deleted_at" is of type timestamp ... but expression is of type text`。

**行为断言**（每条写后必读）：portal/pp 脚本 DELETE → 200 + `deleted=true`，回库
`deleted_at` 非空；mergeitem → 200 且 `data.merged == 1`，回库同键只剩 1 行、保留最早行。

**破坏实验**：分别还原 psp `to_char`、mergeitem 的 `COALESCE(..., id)` → 各自红
（E42804）；复原后绿。

**收口**：删 3 条基线路，`BASELINE_COUNT 10 → 7`；门禁实测
`== 棘轮基线：7 条（当前 7 条），集合相等 ==`。

### O24-3 批次 11：`E42601` document filter 纯 tail（1 条）

`document_filter_paging_manager_u3` 传给 `list_from_table_filtered_legacy` 的
where_clause 是**纯 tail**（`ORDER BY create_time DESC LIMIT 200`）。旧拆分只认
`" ORDER BY "`（带前导空格），漏匹配后数据查询拼成
`SELECT * FROM x_cms_data_document WHERE ORDER BY ...` → `E42601 syntax error at or near "ORDER"`。

修法（`cms_assemble_control/src/lib.rs` 的 `list_from_table_filtered_inner`）：

- 关键字匹配改为不带前导空格的 `ORDER BY` / `LIMIT`，取最早出现位置切分；
- `filter_only` 进 `WHERE`，`tail` 单独以空格追加；纯 tail 时无 `WHERE`。

**先红**：`document_filter_paths_contract` 未改代码时实测 500 + `E42601`（position 48）。

**行为断言**：`POST .../manager`（纯 tail）200 + 返回种子文档；`PUT .../size/{size}`
（filter + tail 的常规形态）同样 200 + 返回种子文档（回归保护）。

**破坏实验**：把关键字列表改回带前导空格 → 红（E42601）；复原后绿。

**收口**：删 1 条基线路，`BASELINE_COUNT 7 → 6`；门禁实测
`== 棘轮基线：6 条（当前 6 条），集合相等 ==`。

### O24-3 批次 12：`E23503` work 删除顺序（1 条）

**schema 现场证据**（FK 查询）：

```text
x_timer_job.work_id → x_work.id，delete_rule = NO ACTION
```

旧实现先 `DELETE FROM x_work` 再清 `x_task` / `x_draft`，只要该 work 有定时任务就
`E23503 foreign_key_violation`。改为**先子后父**：`x_timer_job` → `x_task` → `x_draft` → `x_work`
（同一事务内，行锁与归属校验不变）。

**先红**：`work_delete_paths_contract` 未改代码时实测 500 +
`E23503 ... violates foreign key constraint "x_timer_job_work_id_fkey"`。

**行为断言**（写后必读）：DELETE → 200 + `data.value=true`，回库四张表（work/task/draft/timer_job）
都不再有该 work 的行；另断言不存在的 work 仍是 `200 + type=error` 旧语义（未把 not-found 改成 5xx）。

**破坏实验**：临时跳过 `x_timer_job` 删除 → 红（E23503）；复原后绿。

**收口**：删 1 条基线路，`BASELINE_COUNT 6 → 5`；门禁实测
`== 棘轮基线：5 条（当前 5 条），集合相等 ==`。**剩余 5 条全部是 GET。**

### O24-3 批次 13：`E42P18` ai chat 分页占位符编号错位（1 条）

绑定顺序是 `$1=clue_id, $2=offset, $3=size`，旧 SQL 写成 `LIMIT $4 OFFSET $3`：
`$2` 从未被引用 → PG 无法推断其类型（`E42P18 could not determine data type of parameter $2`），
`$4` 也不存在。改为与同文件 `x_ai_conversation` 分页一致的 `LIMIT $3 OFFSET $2`。

**先红**：`ai_chat_paging_contract` 未改代码时实测 500 + `E42P18`（`lib.rs:2493`）。

**行为断言**：page=1 size=2 返回两条且按 `create_time` 倒序；page=2 size=1 返回较旧一条
（确认 size/offset 语义没被调换）。

**破坏实验**：把占位符改回 `$4/$3` → 红（E42P18）；复原后绿。

**收口**：删 1 条基线路，`BASELINE_COUNT 5 → 4`；门禁实测
`== 棘轮基线：4 条（当前 4 条），集合相等 ==`。

### O24-3 批次 14：`column:equipment` meeting room jsonb（2 条）

**schema 现场证据**：`x_meeting_room.equipment` 是 `jsonb`（nullable），而 SeaORM 实体
`meeting_room::Model` 声明 `Option<String>` → `Entity::find` 解码失败
（`error deserializing column 3`），`GET room/list` 与 `GET room/{id}` 均 500。

修法（`meeting_core_entity`）：

- 实体字段改 `Option<Json>`（`sea_orm` 的 `Json = serde_json::Value`）；
- `create_room` 的 equipment 由 `serde_json::to_string` 改为直接透传 `Value`；
- `get_room` 响应的 equipment 不再 `from_str`，直接输出。

**先红**：`meeting_room_jsonb_contract` 未改代码时实测 500（`error deserializing column`）。

**行为断言**（读 + 写后必读）：list 含种子行；`room/{id}` 返回 `equipment` 原值
`[{"name":"projector"}]`；`POST room/create` 200 后回库 `equipment::text` 与写入值一致。

**破坏实验**：把 ActiveModel 写入值改成固定错误值 → 写后必读断言红；复原后绿。

**收口**：删 2 条基线路，`BASELINE_COUNT 4 → 2`；门禁实测
`== 棘轮基线：2 条（当前 2 条），集合相等 ==`。

### O24-3 批次 15：`wrong-type` identity order（1 条）

**schema 现场证据**：`x_org_identity.order_number` 是 **bigint**，代码用 `(index + 1) as i32`
绑定 `UPDATE ... SET order_number = $1` → tokio-postgres `ToSql(0) WrongType
{ postgres: Int8, rust: "i32" }` → 500。改绑 `i64`。

**先红**：`identity_order_contract` 未改代码时实测 500 +
`ToSql(0) ... WrongType { postgres: Int8, rust: "i32" }`。

**行为断言**（写后必读）：`GET identity/{B}/order/before/(0)` → 200 + `data.success=true`，
回库逐行 `order_number` 为 A=1、B=2。

**破坏实验**：改回 `i32` → 红（同一 WrongType）；复原后绿。

**收口**：删 1 条基线路，`BASELINE_COUNT 2 → 1`；门禁实测
`== 棘轮基线：1 条（当前 1 条），集合相等 ==`。

### O24-3 批次 16：`column:status` program deploy（1 条）

**schema 现场证据**：`x_program_deploy.status` 是 `text` 且 **nullable**；
`market_flag_installed_version` 与 `deploy_id` 都用非 Option 的 `row.get("status")`，
NULL 时 tokio-postgres 解包 panic（`error deserializing column status`）被守护成 500。
两处同根因一并改为 `Option<String>` + 空串兜底（`deploy_id` 不在基线里，但同一张表同一种读法）。

**先红**：`program_deploy_status_contract` 未改代码时实测 500（panic
`error deserializing column 3`）。

**行为断言**：`installed/version` 与 `deploy/{id}` 均 200，status 为 NULL 的行输出
`""`（不是 500）。这两条是读路径，不适用「写后必读」。

**破坏实验**：把 `deploy_id` 那处改回非 Option 读 → 第二条断言红；复原后绿。

**收口**：删最后 1 条基线路，`BASELINE_COUNT 1 → 0`；门禁实测
`== 棘轮基线：0 条（当前 0 条），集合相等 ==`。

### O24-3 42 → 0 分批明细（每批根因族与数字）

| 批次 | 根因族 | 清除 | 基线 |
|---|---|---|---|
| 1 | 写已提交、随后解码 panic（`column:published`） | 1 | 42→41 |
| 2 | Jsonb ↔ String 参数绑定（`wrong-type`） | 4 | 41→37 |
| 3 | 查询设计器 permission 参数个数（`Parameters(2,1)`） | 4 | 37→33 |
| 4 | INSERT 缺必填 id / 规范列（`E23502`） | 3 | 33→30 |
| 5 | `draft/start` 状态字面量（无词表） | 2 | 30→28 |
| 6 | 实体引用列 NULL（`E23502`，降级为 400） | 7 | 28→21 |
| 7 | 配置写路径 Rust 绑定类型不符（`wrong-type`） | 4 | 21→17 |
| 8 | `ON CONFLICT` 无唯一约束（`E42P10`） | 3 | 17→14 |
| 9 | 属性表按 `flag` 匹配不存在的列（`E42703`） | 4 | 14→10 |
| 10 | 时间列与 `timestamptz` 比较（`E42804`） | 3 | 10→7 |
| 11 | filter 子句把 `ORDER BY/LIMIT` 拼进 WHERE（`E42601`） | 1 | 7→6 |
| 12 | 删除顺序违反外键（`E23503`） | 1 | 6→5 |
| 13 | 分页占位符与绑定顺序错位（`E42P18`） | 1 | 5→4 |
| 14 | `jsonb` 列按 `String` 解码（`column:equipment`） | 2 | 4→2 |
| 15 | `bigint` 列按 `i32` 绑定（`WrongType`） | 1 | 2→1 |
| 16 | nullable `text` 按非 Option 解码（`column:status`） | 1 | 1→0 |

最后 8 批（9-16）清 14 条；同一条指令下另含批次 8（E42P10）3 条，合计 **17 条，基线 17 → 0**。

### 降级清单（两张，均不等于「已修好」）

**① 可用性降级：7 条 400（缺参数），功能对用户仍不可用**
hotpic 5 条（`application`/`infoId`）+ portal page 2 条（`portalId`），逐条前端入口与字段来源见
[`o24-3-feature-still-unavailable.md`](./o24-3-feature-still-unavailable.md)。2026-11-05 查证结论：
`x_hotpic` 无 `default`/`0` 真实键 → 登记为「未完成功能（无后端支撑的空壳）」；
`x_portal` 零行 → 前端写死 `portal/default` 本身是错的，停止修复。

**①-a 2026-11-05 后续裁定：hotpic 改判「全局热点图」 → 5 条已转可用**
`x_hotpic` 生产库 51 行 `application=''`/`info_id=''` 说明「无所属应用 = 全局」这条路径事实上已存在，
只是三条读路径都按精确键匹配、把全局行丢掉（**写成功、读不到**）。裁定采用全局模型并落地：

- **约定显式化（此前只存在于数据里、没写在任何文档）**：`(application, info_id)` 两列皆空串 =
  **部署级全局热点图**，在任何 application 视图下可见。代码里以 `GLOBAL_APPLICATION` / `GLOBAL_INFO_ID`
  常量 + `HotpicScope::global()` 具名表达，注释写明语义；半截参数（只给一列）→ 400，不把半截当全局。
- **读路径**统一为精确匹配 `(application, info_id)` **或**全局（两列皆 `''`），只用 AND/OR 精确比较，
  不做 `LIKE` / `IS NULL` 混用 / `COALESCE` 兜底：
  `hotpic_assemble_control::user_hotpic_application_infoId`、`hotpic::{exists_check,list_by_application_and_info_id}`、
  `hotpic_core_entity::{list_by_app_and_info,exists_check}`（SeaORM `visible_scope` Condition）。
- **可见性三段断言**（`hotpic_global_scope_contract`）：全局行在任意视图可见；特定 `(application, infoId)`
  行只在该视图可见；写错键的行（如 `('default','0')`）仍然读不到——写侧错误不被读侧全局兜底掩盖。
- **写后必读**：5 条写端点（`POST /api/hotpic/create/hotpic`、`POST /api/hotpic/assemble/control/user/hotpic`、
  `POST /api/hotpic/assemble/control/update/control/config`、`GET /api/hotpic_assemble_control/create/hotpic`、
  `GET /api/hotpic_assemble_control/update/control/config`）各自缺省创建后回库断言 `application=''`/`info_id=''`
  且 title/image_url 同值。
- **51 行存活证据（核心）**：对开发库 `oa4rust` 发真实 HTTP 请求
  `GET /api/hotpic/user/hotpic/it-probe-view/it-probe-info`，实测 `type=success`、`count=51`、
  `data` 长度 **51**——此前这 51 行「写成功、读不到」，现在在任意（无匹配行的）视图下全部可见。
  隔离库内同规模用例（造 51 行空键）由 `hotpic_global_scope_contract` 覆盖。

**①-b portal page 2 条（保持 400，本族唯一残留）**
依赖关系：**先有门户创建流程与至少一条真实门户行，才谈页面创建**。`x_portal` 零行、迁移无
`INSERT INTO x_portal` = 门户功能没有数据底座。前端 [`PortalApp.vue:437`](../../frontend/apps/desktop/src/views/PortalApp.vue#L437)
写死的 `portal/default` 单独登记为**错误常量**（不是待办默认值），不得据此补数据或补默认门户 id。

**② 语义降级：状态值以具名常量落库**
`pp_c_draft.xstatus` 无 CHECK / 无 DEFAULT / 无读方三项证据成立，`draft/start` 写 `'started'`
并收进唯一常量。它是一条**契约待办**：将来引入状态词表时，必须迁移已存在的 `started` 行，不许新旧混用。

### 「500 → 400 ≠ 功能可用」口径

- **可用**的唯一判据：真实 HTTP **200** + 业务级响应体断言；写路径再加「写后必读」同值断言。
- 只把 500 改成 400（哪怕文案更明确）**不算修好**，只能算「错误可控、不再污染日志」，必须进降级清单。
- 42 条里：**35 条**回到 200 且有行为断言；**7 条**只到 400（见上表 ①）。
  - 该族（批次 6 的实体引用降级）当时结论：**未完成，35 可用 / 7 不可用**，基线清零不等于全部修好。
  - 2026-11-05 后续：hotpic 5 条按「全局 = 空键」模型转可用（①-a），当前 **40 可用 / 2 不可用**；
    残留的 2 条是 portal page，保持 400（①-b）。
- 基线归零只说明「没有已知的 500 了」，不说明这个族的产品功能完整。

### O24-3 基线新起点（PR #162 合并后）

- `BASELINE_COUNT = 0`；CI `seeded-route-scan` 实测：
  `== 扫描覆盖：枚举 读 2393 / 写 1980；执行 读 2393 / 写 1979（共 4372）；401 读 2 / 写 2；403 读 1 / 写 0；401 比例 0.1% ==`，
  `== 棘轮基线：0 条（当前 0 条），集合相等 ==`。
- 新规矩：**后续任何新增 500 直接红，禁止往基线加行**。
- 临时豁免：必须单列理由（owner、根因、影响面、提出日期），且 `due:` 到期由门禁机器执行；
  不允许「先加行再解释」。

### 写路径修复规矩：「写后必读」（批次 7 起，强制执行）

任何写路径修复都必须补一条**写入后立即读回并断言同一值**的用例（SQL 直读同库），
防止「只修罗盘、不修读写」。判据：

1. 写返回 200 后，用同库连接按业务键读回，断言值等于请求值；
2. upsert 类再断言**行数不变**（幂等），并断言 `update_time` 已更新；
3. 若读路径自身依赖类型强转（如 `::bool`、`::text`），读回断言必须走该读路径（HTTP GET）。




### O24-1 收口：express 成功路径（裁定执行）

`organization/core/express` 的 500→200 不再只靠推断：

- 新增 `tests/integration_tests/scenarios/organization_core_express.rs`，三条真实 HTTP +
  真实 PostgreSQL 用例，并在 `integration_runner` 以独立 `#[ignore]` 用例注册
  （CI 的 `run_integration_scenarios.py` 会跑它）：
  1. `sync_enabled=true` → `/config`、`/status` **200**，断言 `enabled/syncInterval/
     maxRecords/status/personCount/groupCount` 具体值；
  2. `sync_enabled=maybe` → **500**，body `internal server error`，日志含原始值
     `x_org_config.sync_enabled=\"maybe\"` 与 `source=...lib.rs:27`；
  3. 无行 → **503**，文案 `service not configured: missing x_org_config.sync_enabled`。
- `seed_all_tables` 补合法 `sync_enabled=true` 行；种子化为扫描实测 `/config`、`/status`
  均 **200**（此前只到 503，200 分支从未执行）。
- 破坏实验：临时改回 `row.get::<bool>()` → 第 1 条红（500 / panic
  `error retrieving column config_value`）；复原后三条全绿。

### O24 计数看板（每轮固定报）

区分「更多缺陷变可见」与「修不动在堆债」，不靠「全绿」：

| 轮次 | 新暴露 | 已清除 | 基线净变化 | 基线总量 |
|---|---|---|---|---|
| O24-1/2（自证 + 门禁 + 库隔离校准） | **2**（`column:equipment`，此前读错库未暴露） | 0 | **+2**（40→42） | 42 |
| O24-1 收口（express 用例 + 种子行） | 0 | 0 | 0 | 42 |
| O24-3 批次 1（portal surface publish） | 0 | **1** | **-1**（42→41） | 41 |
| O24-3 批次 2（JSONB 参数绑定族） | 0 | **4** | **-4**（41→37） | 37 |
| O24-3 批次 3（查询设计器 permission 族） | 0 | **4** | **-4**（37→33） | 33 |
| O24-3 批次 4（INSERT 缺必填 id/规范列族） | 0 | **3** | **-3**（33→30） | 30 |
| O24-3 批次 5（draft/start 写 started） | 0 | **2** | **-2**（30→28） | 28 |
| O24-3 批次 6（E23502 实体引用族） | 0 | **7** | **-7**（28→21） | 21 |
| O24-3 批次 7（wrong-type 配置写路径） | 0 | **4** | **-4**（21→17） | 17 |
| O24-3 批次 8（E42P10 upsert 路径） | 0 | **3** | **-3**（17→14） | 14 |
| O24-3 批次 9（E42703 属性表路径） | 0 | **4** | **-4**（14→10） | 10 |
| O24-3 批次 10（E42804 脚本软删 + mergeitem） | 0 | **3** | **-3**（10→7） | 7 |
| O24-3 批次 11（E42601 document filter 纯 tail） | 0 | **1** | **-1**（7→6） | 6 |
| O24-3 批次 12（E23503 work 删除顺序） | 0 | **1** | **-1**（6→5） | 5 |
| O24-3 批次 13（E42P18 ai chat 分页） | 0 | **1** | **-1**（5→4） | 4 |
| O24-3 批次 14（meeting room jsonb equipment） | 0 | **2** | **-2**（4→2） | 2 |
| O24-3 批次 15（identity order_number bigint） | 0 | **1** | **-1**（2→1） | 1 |
| O24-3 批次 16（program deploy nullable status） | 0 | **1** | **-1**（1→0） | **0** |

**清除清单**：

| 批次 | 族 | 删除的基线行 key | 测试与破坏实验 |
|---|---|---|---|
| O24-3 批次 1 | `column:published`（写库后 500） | `POST /api/portal/assemble/surface/publish/{id}` | `portal_surface_publish_contract`：先红 500（panic 解码）→ 改后 200 且回库 `published=true`；破坏实验改回 `Option<String>` → 红；门禁 41/41 集合相等 |
| O24-3 批次 2 | `wrong-type` 的 Jsonb↔String 子族 | `POST /api/meeting/core/entity/room/save/{id}`；`POST /api/processplatform/assemble/surface/save/{id}`；`POST /api/queryview/table/row/part/update/{tableFlag}/{id}`；`POST /api/queryview/table/{flag}/row/{rid}/part/update` | `jsonb_param_binding_contract`：先红 500（ToSql Jsonb vs String）→ 改后 200 且回库校验 JSON；破坏实验逐个复原三处 → 各自红；门禁 37/37 集合相等 |
| O24-3 批次 3 | `param-count` 的 permission 子族 | `POST /api/query/assemble/designer/query/{flag}/permission`；`POST .../stat/{id}/permission`；`POST .../table/{flag}/permission`；`POST .../view/{id}/permission` | `query_permission_binding_contract`：先红 500（`Parameters(2, 1)`）→ 改后 200 且回库 permission/update_time；破坏实验还原谓词 → 红；门禁 33/33 集合相等 |
| O24-3 批次 4 | `E23502` 的「INSERT 缺必填 id/规范列」子族 | `POST /api/console/send/message`；`POST /api/document/cipher/{id}/persist/view/record`；`POST /api/message/custom/create` | `insert_required_id_contract`：先红 500（E23502 `x_console_message.id`）→ 改后 200 且回库 id/title/type/consumer/is_read；破坏实验逐个复原 message/cms → 各自红；门禁 30/30 集合相等 |
| O24-3 批次 5 | `param-count` 的 draft/start 子族（唯一写方） | `POST /api/processplatform/assemble/surface/draft/start/{id}`；`GET /api/processplatform/assemble/surface/draft/{id}/start` | `draft_start_status_contract`：先红 500（`Parameters(1, 2)`）→ 改后 POST/GET 均 200 且回库 `xstatus='started'`、`xupdateTime` 非空；破坏实验退回 `&[&id]` → 红；门禁 28/28 集合相等 |
| O24-3 批次 6 | `E23502` 的实体引用子族 | `POST/GET /api/hotpic*`（create/hotpic ×2、update/control/config ×2、user/hotpic ×1）；`POST /api/portal/assemble/designer/page`；`POST /api/portal/assemble/designer/page/create` | `entity_reference_required_contract`：先红 500（E23502 `application`/`portal_id`）→ 改后缺参 400、有参 200 且回库指向父实体；破坏实验逐个还原 3 处 handler → 各自红；门禁 21/21 集合相等 |
| O24-3 批次 7 | `wrong-type` 的配置写路径 | `POST/PUT /api/bbs/assemble/control/update/control/config`；`POST/PUT /api/general/assemble/control/status/update` | `wrong_type_config_contract`：先红 500（WrongType）→ 改后 POST/PUT 均 200 且回库正确；破坏实验逐个还原 bbs/general 写/读 → 各自红；门禁 17/17 集合相等 |
| O24-3 批次 8 | `E42P10` 的 upsert 路径 | `GET/POST /api/attendance/assemble/control/statistic/do`；`GET /api/calendar_assemble_control/update/control/config` | `upsert_paths_contract`：先红 500（E42P10）→ 改后 200、同键仍 1 行且 update_time 更新、config 写后必读同值；破坏实验逐个还原 → 各自红；门禁 14/14 集合相等 |
| O24-3 批次 9 | `E42703` 的属性表路径（无 `name` 列） | `PUT/DELETE /api/organization/assemble/control/personattribute/{flag}`；`PUT/DELETE /api/organization/assemble/control/unitattribute/{flag}` | `attribute_paths_contract`：先红 500（E42703 `column "name"`）→ 改后 200 且回库 `attribute_value`/`deleted_at` 命中（写后必读）、软删后再 PUT `type=error`；破坏实验分别还原 person 解析 / unit 软删 → 各自红；门禁 10/10 集合相等 |
| O24-3 批次 10 | `E42804` 脚本软删 text→timestamp；mergeitem `COALESCE` 类型 | `DELETE /api/portal/assemble/designer/script/{id}`；`DELETE /api/processplatform/assemble/designer/script/{id}`；`GET /api/processplatform/service/processing/touch/mergeitem` | `script_delete_paths_contract`：先红 500（E42804）→ 改后 200 且回库 `deleted_at` 非空、mergeitem 只剩最早 1 行（写后必读）；破坏实验分别还原 psp `to_char` / `COALESCE(..., id)` → 各自红；门禁 7/7 集合相等 |
| O24-3 批次 11 | `E42601` document filter 纯 tail 拼进 WHERE | `POST /api/document/filter/list/{page}/size/{size}/manager` | `document_filter_paths_contract`：先红 500（E42601，position 48）→ 改后 manager 与常规分页均 200 且返回种子文档；破坏实验把关键字改回带前导空格 → 红；门禁 6/6 集合相等 |
| O24-3 批次 12 | `E23503` work 删除先删父表 | `DELETE /api/processplatform/service/processing/work/{id}` | `work_delete_paths_contract`：先红 500（E23503 `x_timer_job_work_id_fkey`）→ 改后 200 且四张表均无残留（写后必读）、not-found 仍 `type=error`；破坏实验跳过 timer_job 删除 → 红；门禁 5/5 集合相等 |
| O24-3 批次 13 | `E42P18` ai chat 分页占位符编号错位 | `GET /api/ai_assemble_control/chat/list/completion/{clueId}/paging/{page}/size/{size}` | `ai_chat_paging_contract`：先红 500（E42P18 `could not determine data type of parameter $2`）→ 改后 200、倒序与分页窗口均断言；破坏实验改回 `$4/$3` → 红；门禁 4/4 集合相等 |
| O24-3 批次 14 | `column:equipment` meeting room jsonb | `GET /api/meeting/core/entity/room/list`；`GET /api/meeting/core/entity/room/{id}` | `meeting_room_jsonb_contract`：先红 500（`error deserializing column equipment`）→ 改后 200、equipment 原值返回、create 后回读 `equipment::text` 同值（写后必读）；破坏实验写入固定错误值 → 红；门禁 2/2 集合相等 |
| O24-3 批次 15 | `wrong-type` identity order_number bigint | `GET /api/organization/assemble/control/identity/{flag}/order/before/{followFlag}` | `identity_order_contract`：先红 500（`ToSql(0) WrongType { postgres: Int8, rust: "i32" }`）→ 改后 200 且回库 order_number A=1/B=2（写后必读）；破坏实验改回 i32 → 红；门禁 1/1 集合相等 |
| O24-3 批次 16 | `column:status` program deploy nullable status | `GET /api/program_center/market/{flag}/installed/version` | `program_deploy_status_contract`：先红 500（panic `error deserializing column 3`）→ 改后 installed/version 与 deploy/{id} 均 200、NULL status 输出 `""`；破坏实验改回非 Option 读 → 红；门禁 0/0 集合相等 |


## O25：`x_timer_job` 一次性任务（2026-11-05）

### 三数

| 指标 | 数量 | 说明 |
|---|---|---|
| 新增可用端点 | **1** | `POST /api/processplatform/service/processing/timer/start`（once）。真实 POST → 200 + 回库行存在、`fire_at/created_at` 类型与值正确。 |
| 修复真缺陷 | **3** | ① 写入口 TIMESTAMP 按 String 绑定 → 真实 500；② ticker 每 30s 重复 fire 且覆盖 `fired_at`；③ `restore` 从未接线，重启后未到期/已到期 job 全部失联。 |
| 仍不可用项 | **1（本模块）** | 前端 [`ProcessManagerApp.vue:682`](../../frontend/apps/desktop/src/views/ProcessManagerApp.vue#L682)「定时器」按钮发空 body `{}` → `workId/fireAt` 缺失 → 400，已登记到 `o24-3-feature-still-unavailable.md`。 |

### 结论口径

- **once 任务真实可用**：`POST timer/start` → 200；到期后恰好执行 **1 次**（连续 ≥3 个 tick 周期 `fired_at` 只写一次）；真实进程重启后未到期 job 重新排期、已到期未执行执行一次、已执行**绝不重放**。
- **cron 实际状态 = 拒收（不是实现）**：parity corpus、文档、前端都没有 cron 的书面承诺；`timer/start` 入口对 `cron` 直接 **400**（`cron 定时任务暂不支持，仅支持一次性 fireAt`），且**不落** `x_timer_job` 行。回归 `timer_cron_is_rejected` 断言 400 + 文案 + 零行。
- **expire 分支已删**：全仓没有任何路径产出 `kind='expire'` 的 job；`x_task.task_status='expired'` 仅由手动端点 `task/{id}/expire` 写入，`fire` 内该分支无消费者，删除。

### 提交

| commit | 内容 |
|---|---|
| `1a32bcb9c` | 写入口 `fire_at/created_at` 按 `NaiveDateTime` 绑定 |
| `52107921e` | once job 原子认领 + 从待执行集合移除 |
| `7d621f796` | `router()` 启动时接线 `TimerRegistry::restore` |
| `4fbb057d0` | 真进程重启回归（P1 创建 → 杀进程 → P2 到期触发 + 已执行不重放） |
| `db9ca402a` | cron 400 拒收、删除 expire 死分支 |
| `5a4013797` | startup-smoke 进程存活断言（见下节） |

## graceful：启动即倒计时（2026-11-05）

### 性质

**与关闭信号无关**：修前 `timeout(grace, server)` 包住整个服务生命周期。进程只要启动，`DEFAULT_GRACE_PERIOD=10s` 一到就被强制退出，日志固定打印 `graceful shutdown timed out; forcing exit grace_ms=10000`——即使从未收到 SIGINT/SIGTERM。

### 引入 commit

- `2b0f888db`（2026-11-02 21:20:58 +0800，`fix(oa4rust): 接入优雅停机，部署重启不再腰斩在途请求（R4）`）
- 后续 `ea0b32bef`（2026-11-04 02:24:00 +0800）是 fmt-only，无逻辑改动。

### 调用链与影响面

```
backend/src/main.rs:189  TcpListener::bind(OA4RUST_BIND_ADDR, 默认 0.0.0.0:3000)
backend/src/main.rs:193  shared::graceful::serve_router_with_shutdown(
                            listener, app, DEFAULT_GRACE_PERIOD, shutdown_signal())
```

- 唯一生产二进制是 `backend/src/main.rs`（`oa4rust`）；`--migrate-only` 分支在 bind 之前退出，不起服务。
- 影响面 = **所有走 `serve_router_with_shutdown` 的二进制**：当前只有 `oa4rust` 主服务，即默认部署形态下 100% 受影响——服务起来 10s 后必死。

### 真实部署路径复现（不发任何关闭信号）

旧实现（`timeout(grace, server)` 整体包裹）：

| 时间点 | 进程 | `/health` | 日志 |
|---|---|---|---|
| listening `2026-11-05T01:22:42.675273Z` | 起 | — | `listening on 127.0.0.1:39997` |
| t+15s | 已退出 | 不可访问 | — |
| t+30s | 已退出 | 不可访问 | — |
| t+60s | 已退出 | 不可访问 | — |
| `2026-11-05T01:22:52.754662Z`（t+10.08s） | 退出 | 不可访问 | `WARN shared::graceful: graceful shutdown timed out; forcing exit grace_ms=10000` |

修复版（只在排空阶段计时，相同命令、同一隔离库）：

| 时间点 | 进程 | `/health` |
|---|---|---|
| t+15s | alive | 200 |
| t+30s | alive | 200 |
| t+60s | alive | 200 |

### 为什么长期未被发现

`startup-smoke`（`.github/workflows/oa4rust-ci.yml`，修前）启动二进制后循环 `/health`，**首次成功即 `exit 0`**。实测健康检查在第 **1 秒**成功，最晚一条检查在进程启动后第 **1–2 秒**执行；**没有任何检查超过 10s**，而 grace 自退出发生在 +10s——冒烟窗口 < grace，所以永远绿。

### 修复与验证

- 实现：`oneshot` 记录「关闭信号已到达」，`select!` 监听服务与信号；只有信号到达后才对排空阶段套 `timeout(grace, ...)`。
- 单测 `shared::graceful::tests::server_keeps_running_before_shutdown_signal`：宽限 100ms，等 300ms 后请求 `/ok` 必须 200。破坏实验（还原整体 timeout）红原文：

  ```
  panicked at crates\shared\src\graceful.rs:98:69:
  called `Result::unwrap()` on an `Err` value: Os { code: 10061, kind: ConnectionRefused, ... }
  test result: FAILED. 0 passed; 1 failed
  ```

  还原修复版后 `3 passed; 0 failed`。
- CI：`startup-smoke` 首次健康后继续在 **15s / 30s** 断言「进程存活 + `/health` 200」（30s = 3 倍 grace）。破坏实验（旧实现跑同一步）红原文：

  ```
  ::error::server died before 15s without any shutdown signal (grace period must only apply after a signal)
  ```

  修复版同一步：`::notice::alive and /health 200 at 15s` / `at 30s`。

### 教训（可复用判据）

**1. 生命周期超时陷阱：「等待某事件的超时」≠「包裹整个 future 的超时」**

- 旧实现 `timeout(grace, server)` 把宽限期套在整个服务生命周期上。它连「收到信号后能干净退出」的测试都能通过——因为它退得又快又干净，测试只验证了「信号后退出」这个方向。
- 因此凡涉及 shutdown / grace / 生命周期超时的改动，**必须同时断言反方向**：
  - 未收到信号时**不得**退出；
  - 存活时间必须覆盖 **≥3 倍 grace**（本例 10s grace → 至少 30s）。
- 单测形态：宽限 100ms + 等 3 倍（300ms）后请求必须 200，而不是只测「发信号后能退出」。

**2. 冒烟窗口必须覆盖被断言的时间域**

- 第 2 秒 `exit 0` 的冒烟无法检验第 10 秒的事件，等于没有门禁。
- 规则：断言涉及超时 / 重试 / 周期 / 宽限期时，检查点必须落在**时间域之外**；本例在首次健康后补 15s、30s 两个检查点（15s 越过 10s grace，30s = 3 倍）。
- 只加「进程活着 + `/health` 200」的检查点即可：它同时覆盖「进程没死」与「路由还挂着」。

**3. 避免误读历史**

- 引入 commit `2b0f888db`（2026-11-02 21:21）的**意图是正确的**（部署重启不腰斩在途请求），但**实现致命**（无信号也 10s 自杀）。
- 后续只看到「已接入优雅停机 / 已修复」的人，容易误以为该能力一直正常；台账与提交信息必须写清「意图正确、实现有致命缺陷、由哪条断言兜住」。

## O26-A：删除 `shared::scheduler` 死代码（2026-11-05）

### 判定口径（先写清，避免再次误读）

- 「有代码」≠「在跑」。后台能力的判定必须同时给出：① 启动调用点（`start` / `spawn` / `interval` 的实际调用链）；② 运行证据（日志、表写入或测试观测）。
- 只有类型定义、trait、测试替身而无生产实例化 → 判定为死代码，不计入「已有调度」。

### 零调用方证据（删除前）

- `git grep -n "scheduler" -- oa4rust/backend` = 87 行。除以下 4 处外，全部在 `scheduler.rs` 自身及其两个测试模块（`scheduler_tests` / `tokio_scheduler_tests`）内：
  - `crates/shared/src/lib.rs:31` `pub mod scheduler;`（无 `pub use` 再导出）；
  - `crates/shared/Cargo.toml:44` `tokio-cron-scheduler = "0.10"`；
  - `Cargo.lock`；
  - `scheduler.rs` 自身。
- `TokioCronScheduler` / `MockScheduler` / `Scheduler` trait：全仓无生产实例化；`start()` 只出现在文件自带测试里（测试注释明确「任务注册但不 `start()` 就不会真的执行」）。
- 无 feature flag、无条件编译分支；`tokio-cron-scheduler` 只有 `shared` 一个 crate 依赖，删除后 `cargo check --workspace` 通过。
- 结论：**零生产路径**。

### 保留原语评估

- `schedule_once` / `schedule_cron` 全部委托给第三方 `tokio-cron-scheduler`；`systemtime_to_cron` 只是把 `SystemTime` 转成 cron 字段，不属于本仓库的调度原语。
- 现有 `TimerRegistry` 已自带 `fire_at` 一次性能力，one-shot 语义已覆盖；周期 cron 本仓库明确不实现（见 O26-D）。
- 结论：**无可保留原语，整删**。不把 cron 解析迁进 `TimerRegistry`——那等于引入一个同样没有调用方的周期调度能力。

### 删除前后数字

- 删除前：`scheduler.rs` **764 行 / 28,734 字节**；`oa4rust/backend` 内 `scheduler` 引用 **87 行**；`shared` lib 单测 **386 条**。
- 删除后：`oa4rust/backend` 内引用 **0 行**（`git grep -n "scheduler" -- oa4rust/backend` 无匹配）；`shared` lib 单测 **366 条**（-20 条，全部为 scheduler 自带测试）。
- 依赖清理：`Cargo.lock` 移除 3 个包（`tokio-cron-scheduler`、`cron`、`num-derive`），`shared` 依赖少 1 条。
- 验证：`cargo check --workspace` 通过；`cargo fmt --check` 通过；`cargo test -p shared --lib` 365 passed / 1 failed。失败项 `migrate::tests::migrations_are_ordered_idempotent_and_fail_loud_on_missing_dir` 在 `origin/main`（未改代码）上同样失败：本机脏库报 `column "person_unique" does not exist`（SQLSTATE 42703），与本次删除无关。

### 风险：死代码被误读为「已有调度」

- `docs/archived-plans/2026-08-19-001-fix-close-all-blocking-gaps-plan.md` 与 `docs/plans/2026-10-27-comprehensive-test-coverage.md` 都把该文件描述为「调度器 / TokioCronScheduler 支持 schedule_once / schedule_cron」。
- 若只删代码不改口径，后来者会以为「周期调度已实现、只是没启用」；事实是从未有任何生产调用点，能力本身也只是第三方包装。
- 正确表述：**本仓库没有周期调度器；一次性定时只有 `TimerRegistry.fire_at`。**

## O26：后台任务接线审计（2026-11-05）

### 判定口径（三类定义，后续审计一律沿用）

- **有代码 ≠ 在跑。** 判定「运行中」必须同时给出：① 启动调用点（`spawn` / `interval` / `start_background` 的实际调用链，且该调用链在进程启动路径上）；② 运行证据（测试观测、日志字段或表写入）。
- **未接线**：实现完整、有路由或函数入口，但没有任何启动调用点/调度调用点。
- **无此能力**：全仓检索不到实现（给出负向检索命令与范围）。
- 只有类型定义、trait 或测试替身，且无生产实例化 → 计为死代码，不计入「已有能力」。

### 审计表（三类 + 证据列）

| # | 能力 | 启动调用点 | 运行证据 | 判定 |
|---|------|------------|----------|------|
| 1 | `x_timer_job` 一次性任务 | `processplatform_service_processing/src/routes.rs:28-37`：router 构造时 spawn `restore` 后 `start_background()`；tick 定义 `lib.rs:4084`，默认 30s（`TIMER_TICK_SECONDS` 仅测试压缩） | `tick` 只取 `kind=="once" && cron.is_none()`（`lib.rs:4103-4111`）；集成场景 `tests/integration_tests/scenarios/timer_jobs.rs:174` 直接驱动 tick | **运行中** |
| 2 | LDAP 用户同步 | `src/main.rs:98` `ldap::sync::init_from_env` → `crates/ldap/src/sync.rs:356` `spawn_sync_worker`；默认 3600s，可由 `LDAP_SYNC_INTERVAL_SECS` 调整 | 循环内成功/失败分别 `info!`/`warn!`（`sync.rs:331-335`） | **运行中**（未配置 LDAP 时 `init_from_env` 返回 false，不启动） |
| 3 | CMS 浏览量 flush | `cms_assemble_control/src/lib.rs:7917` handler 首调 `view_count_buffer` → `:7962-7972` 惰性 spawn，60s flush | 计数先进内存再批量 upsert；失败放回 pending 下轮重试（`:7984` 注释） | **运行中**（首个浏览量请求后启动） |
| 4 | 会话过期清理 | `SessionManager::with_pool`（`src/lib.rs:277`、`auth/src/lib.rs:1090`）→ `shared/src/session.rs:124-134`，300s | 清内存过期条目，防内存泄漏（`:121-123` 注释） | **运行中** |
| 5 | token 阈值扫描 | 同上，`session.rs:136-147`，60s | 扫描 `auth_token_threshold` 表（`:142-146`） | **运行中** |
| 6 | 速率限制内存清理 | `src/main.rs:89` `RateLimiter::new()` → `shared/src/rate_limit.rs:95-101`，60s（`CLEANUP_INTERVAL_SECONDS`） | 清理滑动窗口条目（`:105-107`） | **运行中** |
| 7 | WebSocket 心跳 | `realtime/src/lib.rs:324`，每个连接 30s tick、90s 无 pong 断开 | `:333-336` 超时 `warn!` 并 break | **运行中** |
| 8 | 微信模板消息异步队列 | `auth/src/mpweixin.rs:451-455`：`router()` 在运行时上下文内 `spawn_template_queue_worker()`；`auth::router()` 由 `src/lib.rs:100` 合并进主路由 | worker 串行消费（`:57-67`）；队列满/未启动时调用方降级同步发送（`:397-408`） | **运行中** |
| 9 | 任务到期推进（`x_task` 到期置态） | **无**：只有手动路由 `GET /task/{id}/pass/expired`（`routes.rs:180`，实现 `lib.rs:1291`）；`TimerRegistry::tick` 不含任何 task 逻辑 | — | **未接线** |
| 10 | 维护/超时推进族（8 条 `touch/*`） | **无**：仅注册 HTTP 路由（`processplatform_service_processing/src/routes.rs:194-201`），全仓无调度器调用这些函数 | — | **未接线** |
| 11 | 程序中心周期调度（`x_program_schedule`） | **无写入方、无执行器**：全仓无 `INSERT/UPDATE/DELETE ... x_program_schedule`；只有 SELECT（`program_center/src/lib.rs:8159/8166/8226/12031/12039` 等） | `fire` 仅 `INSERT x_program_schedule_log`（`:8421`、`:12053`），不执行任务；前端仅调用一次 fire（`frontend/apps/desktop/src/views/ProgramCenterApp.vue:1940`） | **未接线**（表只读、无 cron 执行器） |
| 12 | 微信模板投递结果 | 发送链路见 #8 | 异步路径**不解析 `errcode`**（`mpweixin.rs:71-93` 只返回原始 JSON）；仅传输错误 `warn!`（`:60-64`）；无 `msgid`、无落库；同步路径校验 `errcode` 但只回调用方（`:429-442`） | **无法观测**（计入未接线） |
| 13 | `shared::scheduler`（TokioCronScheduler） | **无**：零生产调用点 | 仅自身测试 `start()` | **死代码（O26-A 已删除）** |
| 14 | 周期 cron 执行器 | 无 | 全仓无 cron 解析/触发；`x_program_schedule.cron_expression` 只被 SELECT 展示（`lib.rs:4929/8166/8226`） | **无此能力** |
| 15 | 消息重试 / 死信 | 无 | `mpweixin.rs:27` 注释：「v1 简化：失败仅记日志，不做重试/持久化」 | **无此能力** |
| 16 | 分布式锁 / 单 leader 选举 | 无 | `x_program_schedule.server_node` 仅存储与展示（`lib.rs:8226`），无认领/加锁 SQL | **无此能力** |
| 17 | 文件 / 临时目录清理 | 无 | `tokio::spawn` + `interval` 全仓枚举无文件清理循环 | **无此能力** |

**小结**：运行中 8 项、未接线 4 项（含 1 项「无法观测」）、无此能力 4 项、死代码 1 项（已删）。

### O26-B 维护族取证（dry-run，**不接线**）

**取证方法（可复现）**：在容器 PG 上建空库 → `cargo run --bin oa4rust -- --migrate-only` → 按每条能力的 WHERE 条件构造可判定数据集 → 用 `BEGIN; <真实语句>; ROLLBACK;` 执行，取 psql 打印的命中行数。**下表数字为构造数据集上的命中行数，不代表线上规模**；上线前须在生产只读副本按同条件 `SELECT count(*)` 复核。

数据集规模：`x_record=8`、`x_draft=4`、`x_work=10`、`x_job=4`、`x_task=8`、`x_workcompleted=6`、`x_attachment=8`。

| 能力 | 影响表与条件 | 需删行数(dry-run) | 保留期 | 可恢复 | 上限 | 幂等 | 建议周期 |
|------|--------------|------------------|--------|--------|------|------|----------|
| `touch/cleanevent` | `x_record`：`DELETE WHERE record_type='event' AND create_time < NOW()-24h` | **2** | 24h | **否**（物理删；`x_record` 无 `deleted_at`） | 否（无 LIMIT） | 是（重复执行 0 行） | 每小时；**接线前必须先有软删/归档**，否则误删不可逆 |
| `touch/deletedraft` | `x_draft`：`UPDATE deleted_at=NOW() WHERE deleted_at IS NULL AND work_id IN (已结束 work)` | **2** | 无（所有已结束 work 的草稿） | **是**（软删，置回 NULL 可还原） | 否（无 LIMIT） | 是 | 每天一次；需业务确认「work 结束即删草稿」 |
| `touch/handoverjob` | `x_job`：`UPDATE person = x_work.creator WHERE person='' AND creator<>''` | **2** | 无 | **否**（覆盖字段，无历史表；只能按 `work.creator` 人工反推） | 否（无 LIMIT） | 是（回填后不再命中） | 每小时 |
| `touch/loglongdetained` | `x_record`：`INSERT ... SELECT` 未结束且 `start_time < NOW()-24h`，同日同 work `NOT EXISTS` | **2** | 24h 阈值；按 **DB 自然日** 去重 | 插入行可单独删，但属物理删 | 否（无 LIMIT） | 是（当日重复 0 行；跨日每天新增 1 条） | 每小时或每天；**注意时区**（见下） |
| `touch/touchdelay` | `x_record`：`INSERT ... SELECT` `task_status='expired'` 的 work（DISTINCT），当日去重 | **2** | 无任务时间阈值；按 **DB 自然日** 去重 | 插入行可单独删 | 否（无 LIMIT） | 是（当日重复 0 行） | 每小时；**注意时区** |
| `touch/urge` | `x_record`：`INSERT ... SELECT` `task_status='active' AND start_time < NOW()-24h`，按 work+类型+内容**永久**去重 | **2** | 24h 阈值；去重**永久** | 插入行可单独删 | 否（无 LIMIT） | 是（但同一 work 只催办一次，永不重复） | 每小时也无实际效果；需业务先确认「只催一次」是否符合预期 |
| `touch/merge` | `x_workcompleted`：`DELETE` 同 work 重复，保留 `(completed_time,id)` 最早 | **3** | 无 | **否**（物理删） | 否（无 LIMIT） | 是 | 每小时/每天；`completed_time IS NULL` 的行比较不成立 → 会被静默漏删 |
| `touch/mergeitem` | `x_attachment`：`DELETE` 同名 + 同 `work_id` + 同 `workcompleted_id` 重复，保留最早 | **4** | 无 | **否**（物理删；表虽有 `deleted_at` 但语句未使用） | 否（无 LIMIT） | 是 | 每天；**唯一要求管理员权限**（`require_admin_opt`） |

**接线前必须先解决的三处共性问题**：

1. **8 条全是 `GET` 且带写副作用**：浏览器预取、爬虫、健康检查都可能触发。接线时不能保留「GET 即执行」形态，至少改为内部调度直调函数 + 手工入口加管理员门禁。
2. **7 条权限过宽**：除 `mergeitem` 外均 `require_session_opt`（任意登录用户即可触发批量删改）。其中 `cleanevent` / `merge` / `mergeitem` 是物理删除。
3. **日界时区**：`loglongdetained` / `touchdelay` 用 `date_trunc('day', NOW())`（DB 时区，容器通常 UTC），与业务 UTC+8 日界不一致；北京时间 00:00–08:00 会被算作前一天。测试侧已有 `business_today()`（`tests/idor_matrix.rs:1343`）修正口径，生产侧尚未。

### O26-C 任务到期推进

**不动**。等业务规则（到期判定依据、置态目标、是否写记录、是否通知）。

### O26-D 程序中心周期调度（不实现 cron）

**写入方证据**：

- 全仓 `git grep "INSERT INTO x_program_schedule"` / `UPDATE x_program_schedule` / `DELETE FROM x_program_schedule`：**0 条**。
- 只有 `x_program_schedule_log` 的 INSERT：`program_center/src/lib.rs:8421`、`:10365`、`:12053`。
- `x_program_schedule` 的全部访问都是 SELECT（`:1392/1445/1498/1551/1604/4929/8166/8226/10647/10700/12031/12039`）。
- 路由只有 4 条：`list/schedule`、`list/schedulelocal`、`report`（GET+POST，只读日志）、`schedule/fire`（GET+POST）。**没有任何创建/修改/删除计划的路由**。
- 前端唯一调用是 `ProgramCenterApp.vue:1940` 的 `schedule/schedule/fire`，不创建计划。

**裁定**：不实现 cron；不新增「创建计划」路由，也不改现有路由（不存在可拒收的创建路径）。台账口径固定为：**`x_program_schedule` 是只读表，无写入方、无执行器；`cron_expression` 仅展示。**

**遗留债务（需排期）**：`schedule/fire` 返回 `fired: true`，但只写一条 `x_program_schedule_log`，不执行任何业务动作（`:12047-12061`）。这是「接受但永不执行」的误导性成功返回；后续要么返回明确的「不支持执行」，要么接入真正的执行器。**本轮不改**。

### O26-E 微信模板投递可观测性（登记债务，不实现）

**现状**：

- 异步队列 worker（`auth/src/mpweixin.rs:57-67`）：只捕获传输/解码错误并 `warn!`，**不解析 `errcode`**；业务失败（如 access_token 失效、用户未关注）被当作成功，无任何记录。
- 同步降级路径（`:429-442`）：校验 `errcode`，但只把错误文本回给调用方，不落库。
- 两条路径都没有 `msgid`，没有「成功/失败」落库，也没有重试（`:27`）。

**最小方案（不实现，等排期）**：

1. 统一发送函数返回结构：`(errcode: i64, errmsg: String, msgid: Option<String>)`，以 `errcode == 0` 判定成功。
2. 结构化日志（最低成本）：`tracing::info!/warn!` 固定字段 `touser`、`template_id`、`errcode`、`errmsg`、`msgid`、`success`。
3. 需要可查询/可对账时再落表：新增 `x_mpweixin_template_delivery(id, touser, template_id, msgid, success, errcode, errmsg, create_time)`，异步队列与同步路径复用同一写入点。
4. 不在本轮加入重试/死信（属 O26 #15 的独立能力）。


