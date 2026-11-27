# R608a query_service 神经模型身份盲 IDOR 写甄别（验证型甄别）

- 结论：P1。query_service 的 neural/generate/{modelFlag} 与 processing/execute 两端点只验证会话存在、不校验模型归属，任意已登录用户凭已知 model flag 即可把他人神经模型 status 改写为 generating / processing；同功能的 designer 兄弟端点有归属门并拒绝（403），构成身份盲越权写。
- 真父 HEAD：7ebab5dd5477ad3384ac27adebddaedea6aed332（当前 HEAD 已前移一个他组提交 2fc089c6971f2f0c68e2feb46f6fa779b9254b68；真父..HEAD 对本文全部锚点源码 git diff 零命中，静态锚点仍为真父快照）。
- 零源码改；唯一产出：本文档。测试夹具与日志留 gitignored 证据目录，用后即删。

## 立项与范围
- 历史审计 2026-10-14-security-audit-round4.md:1125 仅记 query_service 只收会话存在档、只挡匿名，未专项甄别身份盲 IDOR 写；台账 docs/ops/current-state.md 无 R608a / query_service 神经模型 IDOR 相关命中。
- 两端点同文件 crates/query_service/src/lib.rs：neural_generate_model(:26-52)、processing_execute(:135-219)；对照兄弟 crates/query_assemble_designer/src/lib.rs:1475-1509 neural_generate_model_modelFlag。
- 路由 crates/query_service/src/routes.rs:11-14 POST .../neural/generate/{model_flag}、:19-22 POST .../processing/execute，:23 layer(Extension(pool)) 真实可达。前端 apps/desktop/src/views/QueryViewApp.vue 经 apis postmodelflag 消费同一 flag，用户可触达。
- 同族两端点合并为 1 项（同一身份盲模型状态写缺陷），不硬凑独立项。

## 机制锚点（真父源码）
- crates/query_service/src/lib.rs:20-24 require_session 仅 session.map(|Extension(s)|s).ok_or(AppError::Unauthorized)，只验会话存在、不校验归属。
- neural_generate_model :31 require_session(session)? 丢弃返回值；:34-40 SQL UPDATE x_query_neural_model SET status = 'generating', update_time = NOW() WHERE flag = $1，无 creator 归属谓词、无 admin 门；:42-44 result==0 才报 model not found。
- processing_execute :140 let sess=require_session(session)?；:195-201 同表 SQL SET status = 'processing', update_time = NOW() WHERE flag = $1，模型 flag 非空时无归属门；creator 落入 x_query_processing 不弥补模型归属。
- 兄弟铁证 crates/query_assemble_designer/src/lib.rs:30-55 require_row_owner：无会话 401(:37)，SELECT coalesce(nullif(trim(creator_person),''),creator,'') AS owner FROM {table} WHERE {key_col}=$1(:42-43)，Some(o)→require_owner(:52)，None→require_admin_opt 失败关闭(:53)；:1481-1488 以 x_query_neural_model/flag 调用后再 UPDATE。
- require_owner(crates/shared/src/middleware/rbac.rs:582-590) 归一 trim().to_lowercase() 后比对，admin 放行；is_admin(rbac.rs:76-99)。DDL migrations/032_create_assemble_control_tables.sql:2599-2611 x_query_neural_model(id,name,flag,status,creator,create_time,update_time,sequence,order_number,creator_person,update_person)，037 加 deleted_at。

## 方法与决定性实测（临时 PG + 真迁移 + router oneshot）
- 起独立临时 PostgreSQL（主机/端口/账号/口令均占位 <TEST_HOST>:<PORT> / <USER> / <PASS>，真值由环境变量注入、不入库），shared::migrate::run_migrations 建真表。
- seed 两条 bob 模型(r608a-bob-model / r608a-bob-model2, status=idle, creator=bob)；注入 alice/bob Session 经真 router + oneshot 直调 query_service 与 designer 兄弟端点，另测无会话拒访与源码守卫。

| 断言 | 实测 | 结论 |
|---|---|---|
| 无会话 POST neural/generate/{bob flag} | 401 | 仅会话存在门 |
| alice 凭 bob flag → query_service neural/generate | 200 success, bob status=generating | 身份盲改写坐实 |
| alice 凭 bob flag → query_service processing/execute | 200, bob status=processing | 同族越权改写坐实 |
| alice 凭 bob flag → designer 兄弟端点 | 403, bob status 未变 | 兄弟归属门有效对照 |
| 源码守卫（两 SQL 无 owner 谓词、require_session 丢返回值、兄弟 require_row_owner 存在） | 全部命中 | 机制静态坐实 |
| 合计 | 4 passed / 0 failed | 越权写全链坐实 |

- 决定性命令：DATABASE_URL=<占位> cargo test --test r608a_query_service_neural_idor_audit -- --test-threads=1 → EXIT0，finished in 18.05s。
- 决定性日志（gitignored，用后即删）：test-results/r608a-run.log（12 行 / 735 字节），字节 sha256=c57c39dd175c9838eb93d439860f372c543211d86118e836e666b9860c210f1f。
- 夹具（gitignored，用后即删，117 行）：tests/r608a_query_service_neural_idor_audit.rs，字节 sha256=26d899805e5c47a401b1d83aedc7c057bc2fcd1f30fe747ed6f121f4f3c62aa9。

## 三分裁决（据实，不夸大不降级）
1. 核心危害：任意已登录用户可凭已知 flag 改写他人神经模型 status，造成他人模型被意外重新生成/置处理中，破坏模型生成状态完整性与下游调度一致性；同功能 designer 端点有归属门证明可避免，故判 P1。
2. 据实披露攻击前提（不夸大）：需有效会话（非匿名）、需知道目标 model flag（非批量遍历）、写操作只翻转 status 标志位（非凭据泄露、非任意 SQL 写）；无会话 401、designer 403 均已坐实，故不表述为无差别越权。
3. 非 L1：两端点真实可达、真测 200 坐实，非不可达缺陷。
4. 两端点合并 1 项：同属 query_service 模型状态写身份盲，不硬凑独立项。
5. 与 R606a 区分：R606a 为考勤 received 标志位按 ID 写（P1），本项为神经模型生成/处理状态写，同属身份盲写族但影响模型生成链，独立成项。

## Fail Loud
- 真父之后多一个他组提交 2fc089c6971f2f0c68e2feb46f6fa779b9254b68，已核实对全部锚点源码零 diff；若有人在此期间改动上述源码需重跑真测。
- 首版夹具 c.execute 误传多段 SQL 字面量致编译失败 E0061，已改 concat! 单字面量后重跑通过；非产品逻辑缺陷。
- 非回环端口/账号/口令均为临时真值，未写入任何入库文件。

## SHA 清单（runner 紧凑 JSON 哈希与文件字节哈希分列）
- runnerSha=sha256(JSON.stringify(meta))=e39a4a10cbcbefba249aedba634f207791b7c9622584fed9ea7e5f751d1e286b
- 证据日志字节 sha256=c57c39dd175c9838eb93d439860f372c543211d86118e836e666b9860c210f1f
- 夹具字节 sha256=26d899805e5c47a401b1d83aedc7c057bc2fcd1f30fe747ed6f121f4f3c62aa9
- crates/query_service/src/lib.rs=1e8ee95fd995a55675ac1ddf20d04c8624b1c6ba215a52f9ebdeb45e1d4e5e54
- crates/query_service/src/routes.rs=925a8bf0e6e5a73af04e35386fae822a94dae16c6ebecc5cb36c42db829e601d
- crates/query_assemble_designer/src/lib.rs=8525f1742c14f105bee21b89ff2cfaa9954ef619a228694240293faced1f11f7
- migrations/032_create_assemble_control_tables.sql=722c2899500c201245ffc3b612ae00fd8c05c21aa8512bab2453b57266428d9e
- ../frontend/apps/desktop/src/views/QueryViewApp.vue=9b8a6e048a4f60331fe7dc3828ca67faa1a96a82bedd7b97f06fcca01117084e

## 修复方向
- query_service 两端点改用兄弟 require_row_owner(pool,session,'x_query_neural_model','flag',model_flag) 或等价归属门（creator_person/creator + admin 失败关闭），再执行 status UPDATE。
- 补归属测试：A 对 B 模型 flag 调 neural/generate 与 processing/execute 均返回 403/401，本人/admin 正常；与设计者端点保持一致。
