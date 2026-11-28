# R625a jpush device_list 身份盲读甄别报告

> 轮次 R625a / 任务 t267 / 甄别型（work 验证型，零源码改）
> 基线父 371650c14ac4d8cb5179cbeb55513f2c8cb8e379；本报告仅甄别，不改任何业务源码与测试。

## 一、结论与判级

据实判 **P1**（身份盲读导致的跨用户凭证与归属信息批量泄露），与 R623a 考勤明细分页身份盲读（批量 PII）同族。
device_list 不接收会话身份，任何通过全局认证层的登录用户可一次读到最近 20 台设备的推送 token 与所属 userId。
判级理由：泄露的是推送凭证 token 与用户归属关联，且为批量跨用户；虽 token 能否直接用于群发取决于服务端 Master Secret（据实未验证、存疑），但凭证与身份关联的批量泄露本身危害已达 P1，不夸大为「可立即群发」。
不预设、不硬凑：本结论由真执行证据支撑，见第三节。

## 二、锚点与根因

- 目标：oa4rust/backend/crates/jpush_core_entity/src/lib.rs:82-113 device_list(db: Extension<DatabaseConnection>)，签名无任何 Session 参数。
- 根因：:85-90 裸 jpush_device::Entity::find() 全表按 create_time 倒序 limit(20)，无按 user_id 的归属过滤；:92-102 逐条回传 id/userId/platform/token。
- token 为推送凭证字段（实体 PushDevice，:68 token: String），与 userId 同信封返回。
- 路由：lib.rs:269 POST 之外的 GET /api/jpush/core/entity/device/list → get(device_list)；backend/src/lib.rs:194 顶层 merge jpush_core_entity::router，真实可达。
- 认证边界：全局 auth_middleware 只验登录态并注入 Session，不校设备归属；device_list 既不取该 Session，也无门。

## 三、真执行证据（oneshot 直调路由器，本地回环临时库，用后即删）

临时契约 oa4rust/backend/crates/jpush_core_entity/tests/r625a_device_list_idor_audit.rs，三用例真父版实跑 3 passed / 0 failed：
- device_list_is_identity_blind_read：alice 会话调 list 得 200，响应体含 bob 的推送 token 与 userId，坐实跨用户盲读；同用例 alice 调兄弟 device_get 读 bob 设备得 403，反证同 crate 读门存在且仅 list 漏网。
- anonymous_list_rejected_by_auth_middleware：自建挂载 auth_middleware 的 Router，无 token 裸调 list 得 401，坐实匿名反臂（全局认证链仍生效）。
- source_guard_device_list_has_no_owner_gate：源码断言 device_list 段无 Session、无 device_read_allowed、含裸 find+limit(20)+token 回传，兄弟 device_get 仍保留 require_session+device_read_allowed。
夹具：three auth_person（alice/bob/admin）+ admin 角色 + 两台设备，均为占位数据；库 DROP/CREATE 由夹具自清理。

## 四、三分据实

- 可达性：需有效登录会话（匿名被 auth_middleware 挡为 401），无需管理员权限；一次请求即取最近 20 台设备，不需遍历设备 id。
- 危害：任意登录用户可批量获得设备推送 token 与其 userId 的映射，跨用户凭证与身份关联泄露；若服务端 Master Secret 可得则可用于定向推送（据实未验证，列为存疑不夸大）。
- 限制：结果上限 20 条且按 create_time 倒序，仅覆盖最近注册设备；token 真正可用依赖 AppKey/Master Secret 是否可得（存疑）；读端点不改写数据，无破坏性；读侧共享接收人合法性与写侧归属门是两套语义，本缺陷仅在写侧缺失门控的对照下凸显。

## 五、兄弟对照与修复方向

- 同 crate device_get（:116-140）与 device_delete（:280-309，R616a 已修）均经 device_read_allowed + DEVICE_READ_OWNER_PREDICATE（:30-37）按 user_id 校本人或 admin，存在但无权 403、不存在 404。
- device_list 是同 crate 唯一未挂归属门的读端点；写侧 device_delete 已有门、读侧 list 无门，呈写有门读无门的不对称。
- 最小修复方向（本报告不实施）：为 device_list 增加 session 与归属过滤——非 admin 仅返回本人 user_id 命中设备，admin 可见全量；沿用 DEVICE_READ_OWNER_PREDICATE 语义，GET/路由/Cargo/migrations 零改；须 RED→GREEN 契约与三 verify。

## 六、sha256 分列（文件 sha 与 runnerSha 口径不同，各自独立）

- src/lib.rs（基线父版）：263445d6b77dba4bea19ecaa2ed73b95b95eeb882bf02279a4c93bd279e3b1f2
- 临时审计契约（执行版）：fb69f8413c696f15fa0c34c7344640ed9d8eaf1d2c26df36c43247cd5adef361
- runnerSha（甄别元数据 JSON 口径，无文件 IO，与文件 sha 分列）：3fbd962cae76c829f079ec327bfe4b793258584476ec47e63c7d9e56e2410eee
- 文档自身 sha256 于提交时经 ops_stat 采集并列于交付报告（避免自引用不写入正文）；本报告 0 真值（无密钥/口令/真实域名或 IP/个人路径，连接串仅本地回环占位且不入库，端口仅运行注入）。
