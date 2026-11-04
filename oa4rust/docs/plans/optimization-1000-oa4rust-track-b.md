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

### 可恢复

| 产物 | 现状证据 | 下一步（可执行） |
|------|----------|------------------|
| `crates/parity/src/generated_tests.rs` | 生成器可运行，输入被 `c158a0251` 删除。临时恢复输入后生成器跑通，但输出与已提交产物差 **+1635 / -381 行**（产物已落后于当前路由） | ① `git show c158a0251^:docs/audits/o2server-parity-report.json > docs/audits/o2server-parity-report.json`；② `python oa4rust/backend/scripts/generate_parity_tests.py`；③ `cargo test -p parity`（依赖根包，需要可构建 openapi 的环境）；④ 把 parity 加进 `check_generated_artifacts_up_to_date.py` 的 JOBS；⑤ 若确认不愿接受这 2016 行变更，改判「已废弃」：删生成器 + 产物，并移除 `parity/src/lib.rs` 的 `mod generated_tests` |
| `frontend/packages/apis/src/api-coverage-generated.test.ts` | `npx --prefix oa4rust tsx oa4rust/scripts/generate-api-tests.ts` 可跑（78 模块 / 666 函数），但产物与生成器差 **+4680 / -8221 行**：已提交产物含人工增强（SPDX 头、实际 `await fn?.()` 调用以驱动 V8 覆盖、单行格式） | 修：先把生成器升级到当前产物形态（SPDX + 实际调用 + 格式），重生成后把「生成 + `git diff --exit-code`」放进 `oa4rust-web.yml` 的 Vitest 步骤之前 |
| `tests/contracts/backend-route-methods.fixture.ts` | 头注释称 AUTO-GENERATED，仓库内无生成器。临时比对：fixture 4108 条 vs `generated_routes.rs` 归一后 4040 条；**131 条只在 fixture、63 条只在后端、2 条方法集不符**（`/api/calendar_assemble_control/event`、`/mcp`） | 修：让 `gen_mcp_tools.py` 在写 `generated_routes.rs` 的同时输出该 fixture（单一源头），或新增 `gen_route_methods_fixture.py`；接进 `generated-artifacts` 门禁；重生成后跑 `vitest run tests/contracts/route-methods.test.ts` |

### 非生成物、仅需更正

| 产物 | 问题 | 下一步 |
|------|------|--------|
| `crates/parity/src/behavior_tests.rs` | 头注释误写 AUTO-GENERATED（下一行自称 Hand-written） | 改注释即可，不涉及门禁 |


1. ~~迁移并发安全~~（O1 done）。
2. ~~sea-orm/sqlx 错误分类~~（O7/O8 done）。
3. ~~限流器 key 空间洪泛~~（O11 done：50k 上界 + LRU 逐出）。
4. ~~IDOR / 越权~~（O9 done：矩阵 + 读面归属闸家族）。
5. 管理端路由鉴权穷举扫描。
6. 输入健壮性：超长字段/深嵌套/畸形 JSON 的上界缺失。
7. 性能基线：真实数据量下关键路径 P95/P99。
8. 数据完整性：业务写与审计写同事务、孤儿行、级联语义。
9. **O10 候选（O9 棘轮桶，161 条 handler 级 500）**：sea-orm 实体类型与列类型系统性不匹配（实体 `Option<DateTime>` ↔ 列 TEXT、`i32` ↔ BIGINT——insert 的 RETURNING 全列解码必炸，bbs_forum_info 已实证根因）+ `{}` 体无校验直打 DB 族。逐族对齐实体定义与 migration 实态，修复一族下调棘轮基线。
10. decode panic 族残部：`row.get::<String>` 裸读可空列的存量点（O9 修了 bbs sort/hotpic application/calendar detail 三处实证点，全仓仍有同类潜伏—— arity_probe/矩阵对有数据的表会逐个暴露）。
11. ~~限流器整表写锁分片化~~（O12 判定不开轮：饱和上限 2.3 M rps，现实需求低约 2 个数量级，数据见「不开轮记录」；若未来单实例 QPS 显著上升再带基线重开）。

