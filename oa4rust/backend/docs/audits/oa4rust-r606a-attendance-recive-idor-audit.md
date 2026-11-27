# R606a 考勤按 ID 接收身份盲 IDOR 审计（验证型甄别）

- 结论：P1。attendancedetail/recive 与 reciveSingle 两个写端点只验会话存在、从 body 取 detail id 后直接按 id 更新 received，不校验该明细归属；任意已登录用户凭他人明细 id 即可把他人考勤接收标志改为 true，身份盲越权写坐实。
- 真父 HEAD：2a6b9b5f7a1fae0c7a0af2f6280471ef4de9fcd1（git 核实一致）。
- 零源码改；唯一产出：本文档。测试夹具与日志留 gitignored 证据目录，用后即删。

## 立项与范围
- 三端点同文件 crates/attendance_assemble_control/src/lib.rs：attendancedetail_recive(:1992-2021)、reciveSingle(:2024-2053)、对照 mobile_recive(:1896-1935)。
- 路由 crates/attendance_assemble_control/src/routes.rs:114 POST .../attendancedetail/recive、:115 POST .../reciveSingle、:110 POST .../mobile/recive 真实注册。
- 前端真实消费：apps/desktop/src/views/AttendanceApp.vue:1013-1022 reciveDetailById() 用 prompt('考勤明细 ID:') 输入任意 detail id 直接 POST recive {id}。
- 两端点属同一缺陷（按 id 接收的身份盲写），合并为 1 项，不硬凑独立项。

## 机制锚点（真父源码）
- 缺陷档：:1998 require_session(session)? 只验会话存在、丢弃返回值；:2001-2005 从 payload 取 id；:2009 SQL UPDATE x_attendance_detail SET received = true, update_time = NOW() WHERE id = $1，只绑定 id，无 person_id 归属谓词。:2024-2053 reciveSingle 同款 SQL(:2041)。
- require_session 定义 :89-93 仅 session.map(|Extension(s)| s).ok_or(Unauthorized)，不校验资源归属，故传谁都行。
- 兄弟铁证（正确写法）：mobile_recive :1902 let sess=require_session(session)?;:1905-1913 取 personId，空则 sess.person_unique;:1914 shared::middleware::require_owner(&pool,&sess,&person_id)?;:1923 SQL ... WHERE person_id = $1 AND date = $2 带归属。证明同系统接收操作本应有归属门，唯裸 id 两端点遗漏。
- DDL：migrations/032_create_assemble_control_tables.sql:1308-1321 建 x_attendance_detail(id,person_id,date,status,creator,unit_id,...)；received 列由 migrations/101_add_missing_handler_columns.sql:30 ALTER ... ADD COLUMN IF NOT EXISTS received BOOLEAN DEFAULT false 补入，真迁移后存在可测。

## 方法与决定性实测（临时 PG + 真迁移 + router oneshot）
- 起独立临时 PostgreSQL（主机/端口/账号/口令均占位 <TEST_HOST>:<PORT> / <USER> / <PASS>，真值由环境变量注入、不入库），shared::migrate::run_migrations 建真表。
- seed 两条明细 r606a-alice / r606a-bob（各自 person_id 同名，received=false）；注入 alice/bob Session 经真 router + oneshot 直调端点；另测 mobile_recive 对照与无会话裸 recive。

| 断言 | 实测 | 结论 |
|---|---|---|
| alice 凭 bob 的 id 调 recive | 200 且 bob.received=true、alice 自己 false | 身份盲越权写坐实 |
| alice 凭 bob 的 id 调 reciveSingle | 200 且 bob.received=true | 孪生端点同样越权 |
| alice 冒名 bob 调 mobile/recive | 403 且 bob 未改 | 对照端点归属门有效 |
| 裸 recive 无会话 | 401 | 会话存在档，非匿名 |
| 合计 | 4 passed / 0 failed | 全绿 |

- 决定性日志（gitignored，用后即删）：test-results/r606a-run.log（14 行），字节 sha256=d9a2510b91d25a013f71f834f10715f7444e0e05bde2a3f46a2bc16278ac96c9。
- 夹具（用后即删，120 行）：tests/r606a_attendance_recive_idor_audit.rs，字节 sha256=65af27324782b4a85fc0973d627e754fc29b233e5ff3f7ec165a284eaf81d7c8。

## 三分裁决（据实，不夸大不降级）
1. 核心危害：写端点身份盲，任意已登录用户可篡改他人考勤接收标志，破坏考勤数据完整性与下游依赖，故按队长裁定判 P1。
2. 据实披露攻击前提（不夸大）：需有效会话、需知道目标 detail id；非匿名、非无差别遍历，故不表述为无鉴权漏洞。
3. 非凭据泄露：写操作仅翻转 received 布尔标志，不读取/外泄 token、口令或个人敏感字段，据实披露。
4. 可自愈/影响面有限：received 标志可被再次翻转，非不可逆破坏，故不判 P0。
5. 两端点合并 1 项：recive 与 reciveSingle 同一缺陷的两处实例，不硬凑独立项。

## Fail Loud
- 队长初判 P1，据实分析显示攻击前提（需登录、需知 id、写限标志位）偏保守；按队长裁定判 P1 并完整披露上述限制，如改判需同步本文结论。
- 真父 HEAD 经 git rev-parse 核实一致；git diff 真父..工作树对 lib.rs 零命中，零源码改坐实。
- 临时端口/账号/口令均为占位真值，未写入任何入库文件。

## SHA 清单（runner 紧凑 JSON 哈希与文件字节哈希分列）
- runnerSha=sha256(JSON.stringify(meta))=2be96894d67cd339774dfe4a4fe12b506d2f9b1e572d86ccac6b54b7c5ffc3b8
- 证据日志字节 sha256=d9a2510b91d25a013f71f834f10715f7444e0e05bde2a3f46a2bc16278ac96c9
- 夹具字节 sha256=65af27324782b4a85fc0973d627e754fc29b233e5ff3f7ec165a284eaf81d7c8
- crates/attendance_assemble_control/src/lib.rs=81b243bf5ecde12e9559593d04caa69672e254084a6d2cfba11abd95bc347128
- crates/attendance_assemble_control/src/routes.rs=d829b2f6e01ffac6b900cb59882d0d995104cb799f191cb9623ebd1741e1c0b9
- migrations/101_add_missing_handler_columns.sql=2209f39c775d3d225ee6f844feecdacb7c1957b8135cdbea5536fd70a839e855
- apps/desktop/src/views/AttendanceApp.vue=938c0839b705345ed41039f9794bfbd833b868320b735176e9bb452e5d017d21

## 修复方向
- 两个裸 id 端点注入会话并取回 sess，按兄弟 mobile_recive 模式对目标明细 person_id 调 require_owner（或 SQL 加 AND person_id=$owner），非归属返回 403；补覆盖「他人 id 被拒」的回归测试。
