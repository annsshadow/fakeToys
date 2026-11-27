# R623a 考勤明细分页身份盲读甄别报告（v2_detail_list_page_size）

> 甄别性质：work 验证型，零源码 / 零测试 / 零其他文档修改；仅以一次性 oneshot 契约实测留证，契约用后即删。

## 一、结论与判级

- 判级：P1（身份盲读逻辑缺陷确凿；属读面越权，非写面篡改，故不升 P0）。
- 可达性：真实 HTTP 路由 POST /api/attendance/assemble/control/v2/detail/list/{page}/size/{size} 已注册；任意有效会话（无需管理员）即可分页读取全员考勤明细。
- 危害：一次空 body 请求即可拿到全员 person_id、日期、状态、fileId 等考勤 PII；亦可通过 body.personId 定点读取任意他人明细，冲突于同模块请假端点的非管理员本人钳制口径。
- 限制：需有效会话，匿名请求被外层认证中间件 401 拦截（反臂实测）；本轮只甄别不改行为，缺陷仍待独立修复轮次处置。

## 二、锚点与根因

- 端点 v2_detail_list_page_size（src/lib.rs 约 6589 行）签名仅含 pool、Path(page,size)、可选 body，无 Session 参数。
- personId 与 status 均取自 body 自证（约 6598–6615 行），不来自会话身份。
- COUNT 与数据 SQL 谓词均为 ($1 = '' OR person_id = $1) AND ($2 = '' OR status = $2)（约 6619–6635 行）：过滤值为空串即整表放行；body 缺省时 personId 取空串 → 全员可读。
- 结果映射 id/personId/date/status/fileId/createTime，legacy_success 信封携带总计数（约 6639–6681 行）。
- 根因：读面过滤条件完全由请求体提供，服务端未将会话 person_unique 作为授权边界；与写面 R620a/R621a/R622a 已修的身份绑定方向一致，属同一身份盲区族的读面漏网。

## 三、真执行证据（oneshot 实测）

- 夹具：独占临时 PostgreSQL 实例（本地回环占位连接串，仅运行期注入，不入库），迁移后种 alice/bob/admin 三个占位人员与 admin 角色、三条分属不同人的考勤明细、两条请假记录。
- 双用例一次运行 2 passed / 0 failed：
  1) alice 空 body 读明细 → 200 且返回含 bob、carol 的他人记录；alice 指定 personId=bob → 200 且仅返回 bob 记录；admin 空 body → 200 可读全员；
  2) 匿名（无 token）经挂载认证中间件的 Router 请求同一路径 → 401。
- 全链路经真实 Router 与 axum oneshot 发起，非纯静态推断；身份盲读行为坐实。

## 四、兄弟端点对照

- 同模块 v2_leave_list_page_size（约 6101 行）接收 Extension<Session>，先以共享 is_admin 判定，非管理员把 filter_person 钳制为 session.person_unique。
- 实测 alice 请求时带 person=bob，仍只返回 alice 自己的请假记录，未泄露 bob；证明读面钳制范式在本模块已有正确实现，目标明细端点属孤立漏网。

## 五、可达性限制与修复建议（据实，不扩大）

- 不夸大项：攻击者仍需一个有效登录会话（401 反臂成立）；需知道该 v2 路由；person_id 为 UUID 形态不可枚举，但空 body 路径无需猜测即全员可读，故可枚举性不构成有效缓解。
- 读侧共享接收人对其他端点是合法业务，但本端点无任何共享/属主语义，属纯身份盲读，不应据此开脱。
- 修复方向（留待独立实现轮）：仿照请假端点引入 Session，非管理员强制 person_id = session.person_unique，管理员方可按 body.personId 过滤；前端空 body 调用应同步改为依赖服务端会话钳制。

## 六、真实性与 sha 分列

- 派发真父提交：1c87a418e50ffd913c887133fa825a30e6793e93。
- 实测执行时 HEAD 已由队友文档提交前移至 62ee5047b8ee378ed7bd348e9e8166817813ecd5；git merge-base 确认真父为 HEAD 祖先，且该区间无任何针对目标端点源码的改动，甄别结论对真父同样成立。
- 一次性甄别契约 sha256：初次写入 aea85d4bf228a5dec01d733aeb3af266212331146c4578b920ae7a89adb2cbbd；经 cargo fmt 重排后最终执行版本 b52c968042bf6eec175cf9632747b2b066957e7a6e1ea3c16323f9cd51ad3715（2 passed 即在此版本上跑出，用后已删，不入库）。
- 目标源码 crates/attendance_assemble_control/src/lib.rs sha256：2fa0d41af986e80322debdb30ec15219696910a78132aa5ebb21831132ccf047。
- runnerSha（甄别元数据 JSON 的 SHA-256，无文件 IO，与上述文件 sha 分列）：9dba42778418c01a6e584762cf605943b4f5cf87928b02c69997773e6af12425。
- 保密自检：文档 0 真值——不含口令、令牌、带凭证连接串、真实域名/IP/主机、真实端口映射或个人目录；测试仅用本地回环占位与占位口令，全部运行期注入。

## 七、验证工具链与既有告警（Fail Loud 据实）

- V1 定点 oneshot：cargo test -p attendance_assemble_control --test r623a_attendance_detail_idor_audit -- --nocapture 得 2 passed / 0 failed；格式化后 touch 强制重编译复跑仍 2 passed。
- V2 包级全量：以已迁移库（克隆模板库）跑 cargo test -p attendance_assemble_control --all-targets 得全绿、0 failed（lib 57 passed / 15 ignored，各集成契约全过）；默认根库未迁移时 5 个既有用例因缺表失败，属环境未迁移非回归。
- V3 格式与静态：cargo fmt -p attendance_assemble_control -- --check 得 EXIT 0（仅新契约长行，cargo fmt -p 后无越界改动）；cargo clippy -p attendance_assemble_control --all-targets --no-deps -- -D warnings 得 EXIT 101。
- clippy 失败项据实披露：仅来自既有写域外文件 tests/r607a_attendance_recive_owner_fix.rs:184 与 :189 的 clippy::bool_assert_comparison（布尔字面量比较），与本轮零源码改无关，属既有告警，按写域约束不修；故 V3 静态检查未全绿，Fail Loud 明示。