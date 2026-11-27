# R603a 催办/加压写操作状态机非法跃迁审计（验证型甄别）

- 结论：P2。task_id_urge / task_id_press 两个写端点无当前状态前置校验，把 completed/done/working 等任务无条件改写为 urged/pressed；兄弟写端点全部按状态拒绝，且全库无 urged/pressed 恢复逻辑，任务陷入不可逆孤立态。
- 真父 HEAD：160be792361850f529a8c1fba09664ca8cc1d5bd（当前 HEAD 已前移一个无关文档提交 c9f3472c5；真父..HEAD 对本文所有锚点源码 git diff 零命中，静态锚点仍为真父快照）。
- 零源码改；唯一产出：本文档。测试夹具与日志留 gitignored 证据目录，用后即删。

## 立项与范围
- 台账 docs/ops/current-state.md 检索 task_id_urge/task_id_press/催办/urged/非法跃迁 全零命中，本缺陷从未专项甄别。
- 实例1：crates/processplatform_service_processing/src/lib.rs:1249-1281 task_id_urge；路由 src/routes.rs:105 GET /api/processplatform/service/processing/task/urge/{id}；前端 ProcessTaskCenterApp.vue:746 真实消费。
- 实例2：同文件 :1326-1350 task_id_press；路由 src/routes.rs:102 POST /api/processplatform/service/processing/task/press/{id}；前端 ProcessWork.vue:1331-1332 真实消费。u2 轨 :185/:186 同样可达。
- 两实例同一缺陷的两种写操作，合并为 1 项，不硬凑独立项。

## 机制锚点（真父源码）
- task_id_urge（:1249-1281）：u4_session(session)? + u4_require_actors(Task,id)；事务内 UPDATE x_task SET task_status = urged WHERE id=$2，无任何当前状态前置条件；INSERT x_record(record_type=urge) 留痕；commit。
- task_id_press（:1326-1340）：同款 u4_session + u4_require_actors；直接 UPDATE x_task SET task_status = pressed WHERE id=$2，无状态前置条件；无留痕。
- 兄弟门对照（同 crate）：task_claim(:3733 要求 pending)、task_complete(:3821 要求 active|processing)、task_reject(:4003 要求 active|processing) 全部对不合法状态返回 BadRequest。唯 urge/press 独缺状态门。
- 归属门 u4_require_actors(:589-619)+U4ActorKey::Task SQL(:565-569) 查 x_task.person 与 x_work.creator，任一等于会话 person_unique 或 is_admin 放行——只管人不管状态。
- 后果锚点：BAM 统计 crates/processplatform_assemble_bam/src/lib.rs:333-335 / :391-393 等端点按 task_status = completed / expired 过滤；urged/pressed 不在其列，任务从统计视图消失。全 backend 源码检索 urged|pressed 仅命中 lib.rs（写入点）、security_gate_tests.rs（断言）、u2.rs（兄弟写）三文件，无任何恢复逻辑。

## 兄弟铁证（语义分叉）
- u2.rs:2038-2065 touch_urge：require_admin_opt + WHERE task_status = active + start_time < NOW()-24h 前置，且只 INSERT x_record 留痕、不改 task_status。与 task_id_urge 直接改状态、无状态门形成鲜明分叉，证明「催办」应有状态约束的设计意图在本系统内已存在。
- security_gate_tests.rs:2438-2468 把 press→pressed、urge→urged 副作用固化为预期断言，从未校验前置状态；测试头注释只记第十六批补归属门，未提状态机。
- 台账 2026-10-12-security-audit.md:58 记 delay/urge/merge 原任意登录可触发、已补仅会话门禁——只补认证，未补状态机。

## 方法与决定性实测（临时 PG + 真迁移 + oneshot 直调）
- 起独立临时 PostgreSQL（主机/端口/账号/口令均为占位 <TEST_HOST>:<PORT> / <USER> / <PASS>，真值由环境变量注入、不入库），用 shared::migrate::run_migrations 建真表。
- 三个 scratch 库分别 seed completed/done/working 任务，注入合法负责人 Session 直调真 handler；对照轨同期调兄弟门 claim/complete 确认其为拒绝。

| 断言 | 实测 | 结论 |
|---|---|---|
| completed 任务调 urge | 兄弟 claim 拒绝；urge Ok、task_status=urged | 终态被无条件改写 |
| done 任务调 press | 兄弟 complete 拒绝；press Ok、task_status=pressed | 终态被无条件改写 |
| working 任务调 urge | urge Ok、task_status=urged | 在办态被无条件改写 |
| 合计 | 3 passed / 0 failed | 非法跃迁全部坐实 |

- 决定性日志（gitignored，11 行）：test-results/r603a-oneshot.log，字节 sha256=83dc1304f08b12c659d8e2e01a0ba340b8cc405cad376df8c835b0be42d06a63。
- 夹具（gitignored，用后即删，107 行）：tests/r603a_task_urge_state_machine_audit.rs，字节 sha256=dcec312832a770635ac4b64abb4c7dd53f5fa26c80295384e7c794e3fdd12676。

## 三分裁决（据实，不夸大不降级）
1. 需有效会话 + 需按 ID 命中 task + 需通过 u4_require_actors 归属门才能触发，非匿名、非无差别遍历，故判 P2 而非 P1。
2. 非 L1：端点真实可达但利用前提明确（合法负责人或管理员对已知 task ID 操作），不构成不可达缺陷。
3. 影响面据实：读端点不涉及凭据泄露；危害是任务状态机被破坏、任务从 BAM 统计消失且无自愈，属完整性与可用性问题。
4. urge/press 合并 1 项；u2 touch_urge 是正确实现，不计入缺陷项。

## Fail Loud
- 夹具首版编译失败：把 handler 返回值当 Json<Value>（实为 Json<ActionResult<Value>>）且漏传 task_complete 第 4 参 Option<Json<Value>>——均为夹具问题，非被测逻辑失败；修正后全绿。
- 当前 HEAD 比指定真父多一个无关文档提交，已核实对全部锚点源码零 diff，不影响结论；如有人在此期间改动上述源码需重跑。
- 非回环端口/账号/口令均为临时真值，未写入任何入库文件。

## SHA 清单（runner 紧凑 JSON 哈希与文件字节哈希分列）
- runnerSha=sha256(JSON.stringify(meta))=7dd06f84f7bb9af6307af965e88a2aff6218dfff4779cc97ee2a7570fa0b83e2
- crates/processplatform_service_processing/src/lib.rs=2b1d6062caed855bb29ed7ae408af188d4033d2cd0f015ce55b0e8e4e15a4289
- crates/processplatform_service_processing/src/u2.rs=af5758380778f04db98f63f5c2977dea127bd13291a5451973b6525a0af8d279
- crates/processplatform_service_processing/src/security_gate_tests.rs=bb8a0fe0d3af56bdab56954a1338463273b9e619fa5e69ee5891482c3deb541b
- crates/processplatform_assemble_bam/src/lib.rs=cf7ca4614a8bcc6b02a79ffdb48581657960ccb1f064f640c205e80bca47f6f5
- crates/processplatform_service_processing/src/routes.rs=37939c44a58e038e8fb5369ba1b7aa908546bed70fd0feb259ccca84738467e4

## 修复方向
- 给 task_id_urge / task_id_press 加当前状态前置门（参照 task_claim/task_complete/task_reject），仅允许 active/processing 等合法态；终态拒绝并返回明确错误。
- 同步更新 security_gate_tests.rs:2438-2468 的预期断言，使其覆盖非法态被拒；并为存量 urged/pressed 孤立任务提供恢复路径。
