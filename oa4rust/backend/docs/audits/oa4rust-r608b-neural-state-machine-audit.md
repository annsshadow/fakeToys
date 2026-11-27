# R608b designer 神经模型状态机甄别（只读）

## 结论 / 真父 / 零源码改
- 结论：判 P2（可达真缺陷 1 项）——designer 神经模型 generate/learn/reset 三端点缺当前状态前置，合法属主可对任意状态做非法跃迁；有归属门、非匿名、非跨用户遍历，不构成越权或泄露。
- 真父：7ebab5dd5477ad3384ac27adebddaedea6aed332（台账 §508 立项）；当前 HEAD 已前移一个无关文档提交 2fc089c69（R608c t191），对下列锚点源码零 diff，不影响结论。
- 本次零源码/配置改动；临时 oneshot 夹具用后即从源码树删除，唯一入库物为本报告。

## 立项与范围
- 目标：甄别合法属主能否绕过神经模型状态机，对 learning 调 generate、对 idle 调 learn、对 processing 调 reset；并以 stop_generating/stop_learn 兄弟门作语义分叉铁证。
- 不计入：R4362 会议 status 产品口径族、R603 urge/press 已甄别族；台账零命中。

## 机制锚点（query_assemble_designer/src/lib.rs）
- :1476/:1515/:1705 三处理函数签名；:1481/:1520/:1735+:1752 各先过 require_row_owner（定义 :30，查 creator_person/creator 归一，空归属或行不存在仅 admin 可过）。
- :1493 `UPDATE ... SET status='generating', update_time=to_char(NOW(),'YYYY-MM-DD HH24:MI:SS') WHERE flag=$1` 无 `AND status`；:1532 SET learning 无前置；:1713 SET idle 无前置。
- 兄弟正面：:1786 stop_generating 带 `AND status='generating'`，:1827 stop_learn 带 `AND status='learning'`，n==0 → "model not found or not generating"/"not learning"。
- 路由真实消费：:851 generate / :852 learn / :854 reset(POST) / :948 reset/status(GET) / :855 stop_generating / :856 stop_learn；前端 QueryManagerDeep.vue:773/:774/:789/:790/:791 全在册。

## 方法与决定性实测（独立 PG + 真迁移 + oneshot 直调）
- 起独立临时 PostgreSQL（主机/端口/账号/口令均为占位 <TEST_HOST>:<PORT> / <USER> / <PASS>，真值由环境变量 DATABASE_URL 注入、不入库），shared::migrate::run_migrations 建真表；两个 scratch 库分别 seed learning/idle/processing 模型（creator_person=虚构属主）。
- 注入合法属主 Session 直调真 handler，注入 outsider/无会话 Session 验归属门；日志见 gitignored 证据目录。

| 断言 | 实测 | 结论 |
|---|---|---|
| learning 调 generate | Ok；status=generating | 非法跃迁被接受；原合法 stop_learn 因状态被改写而拒绝(not learning) |
| idle 调 learn | Ok；status=learning | 跳过 generate 直接改写；stop_generating 拒绝(not generating) |
| processing 调 reset | Ok；status=idle | 非法跃迁被接受 |
| 兄弟 generating→stop_generating | Ok；status=idle | 正面门未变 |
| 兄弟 learning→stop_learn | Ok；status=idle | 正面门未变 |
| 非属主 generate | Err(forbidden) | 归属门未变 |
| 无会话 generate | Err(unauthorized) | 认证门未变 |
| 源码守卫 | 3 无状态前置 + 2 STOP 带 AND status | 与静态一致 |
| 合计 | 2 passed / 0 failed | 非法跃迁全部坐实 |

## 三分裁决（据实，不夸大不降级）
1. 可达真缺陷 1 项（P2）：需有效会话 + 按 flag 命中 + 通过归属门才能触发，非匿名、非无差别遍历，故 P2 而非 P1；危害是状态机完整性被破坏（processing/idle 模型可被改状态、合法 STOP 失效），非凭据泄露。
2. 不可达 L1：0 项——端点真实可达且已实测坐实，不适用。
3. 正常项：stop_generating/stop_learn 兄弟门带前置且正面有效；require_row_owner 归属门与认证门未变；三前端入口真实消费。
4. 预判 `WHERE` 无状态前置已由真库 oneshot 证实，非静态猜测。

## Fail Loud
- 夹具迭代中出现编译错误（断言字符串多余引号、Option<Extension<Session>> 未包 Some、两处 unused import 触发 -D warnings）与兄弟正面测试初态错误（seed 默认 learning/idle 与 STOP 所需 generating/learning 不符）——均为夹具问题，已逐一修正后全绿，不掩盖。
- 当前 HEAD 比真父多一个无关文档提交，已核对该提交对上列源码零 diff；若后续有人改动上述源码需重跑。
- 非回环端口/账号/口令为临时真值，仅经环境变量注入，未写入任何入库文件。

## SHA 清单（runner 紧凑 JSON 哈希与文件字节哈希分列）
- runnerSha=sha256(JSON.stringify(meta))=0bf6a54eb00fe2f5c8f01e20ada53fcb28f23215f732cae210259e31c3fa4ebd
- oa4rust/backend/crates/query_assemble_designer/src/lib.rs=8525f1742c14f105bee21b89ff2cfaa9954ef619a228694240293faced1f11f7
- oa4rust/backend/crates/query_assemble_designer/src/routes.rs=043a66c2b01a3e8f185a76c224d4f24c828e6fa94b6964a6823a3e3d86e4fa20
- oa4rust/backend/crates/shared/src/middleware/rbac.rs=5d6f6b64efc59739e87dc8fd37ba3848eff651b838b3d3679d73f60145716d6a
- oa4rust/backend/migrations/032_create_assemble_control_tables.sql=722c2899500c201245ffc3b612ae00fd8c05c21aa8512bab2453b57266428d9e
- oa4rust/frontend/apps/desktop/src/views/QueryManagerDeep.vue=f356b2c85a187b4be7b6c2f9fd7caacfa493a1f1991c77d85639cba3b8aa0b1e
- 夹具字节 sha256=872ed23dd26539a3c2cd343898f6c62b2cf717533babc29751b5125d293dd710（gitignored，用后即删）
- 决定性日志字节 sha256=30f553910ad35a72525a5d7625b56785d8dca10ddeee61e29a24f0dc1782563d（gitignored）

## 修复方向
- 给 generate/learn/reset 三端点加当前状态前置门（参照兄弟 stop_* 的 `AND status`），仅允许合法源态，非法态返回明确错误；复位端点复用同一核心函数时一并纳入前置校验。
