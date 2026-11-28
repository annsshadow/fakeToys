# R626a jpush device_list 身份盲读 IDOR 真修复审计

## 一、结论
判级 P1。真父版 device_list 无会话门、裸 ORM 全表读取最近 20 台设备并回传 token/userId；任意登录者可越权读取他人推送凭证。本轮已修复并通过 RED→GREEN 坐实。

## 二、锚点与根因
- oa4rust/backend/crates/jpush_core_entity/src/lib.rs 原 device_list 仅接收 db，无 session 提取，裸 Entity::find().order_by_desc(CreateTime).limit(20) 全表扫描。
- 兄弟 device_get / device_delete 已有会话归属门（R616a 修复写侧），读侧列表遗漏，形成写有门读无门的不对称。
- 路由 GET device/list 真挂载；全局 auth_middleware 仅验登录不校资源归属，故登录态任意人可越权。

## 三、RED→GREEN 证据矩阵（真父 b01120b3..，临时 oneshot 契约）
| 证据臂 | RED（真父） | GREEN（修复后） |
| --- | --- | --- |
| alice 会话只见本人设备、不见 bob token/userId | FAIL 读到 bob | PASS 隔离 |
| 匿名裸调列表 | FAIL 200 | PASS 401 |
| admin 跨查可见全体 | PASS | PASS |
| 源码守卫（session 门 + 归属 SQL + 无裸 find） | FAIL | PASS |
RED：1 passed / 3 failed（EXIT 101）；GREEN：4 passed / 0 failed（EXIT 0），touch 强制重编译后格式化复跑仍 4/4。

## 四、修复要点与行为变化
- 新增 DEVICE_LIST_OWNER_PREDICATE（单参数 $1 = 会话 person_unique），口径与 DEVICE_READ_OWNER_PREDICATE 一致。
- device_list 加 Option<Extension<Session>>，require_session 置于首次 DB 访问前；原生 SQL 带归属谓词替代裸 ORM。
- 非 admin：从「可见全员设备+token」变为「仅本人」；admin 保持可见全体；匿名从 200 变 401。路由/前端/配置零改。

## 五、验证结果
- V1 定点：4 passed / 0 failed（EXIT 0）。
- V2 包级（已迁移库）：跳过两处写域外过期断言后，lib 14/14、r616a 契约 1/1、本契约 4/4 全绿（EXIT 0）。
- V3 cargo fmt --check EXIT 0；cargo clippy --all-targets --no-deps -- -D warnings EXIT 0（0 警告）。
- Fail Loud：写域外既有断言 src/tests.rs:29（裸调期望 500，修复后正确返回 401）与 r616a 源码守卫（硬编码 require_session 出现 2 次，修复后为 3 次）在完整包级运行中失败，属安全行为正确变化导致的过期断言，写域外不修，已如实披露。

## 六、SHA 分列（无真值）
- 甄别元数据 runnerSha（对审计元数据 JSON 取 sha256）：66cd997617e901fa36ee6e30b7268f4a628f6d4d058272f277f01fa0fbd1afa3
- 源码 oa4rust/backend/crates/jpush_core_entity/src/lib.rs sha256：5e168d53cdfb7bda38be1f2a8494d12c8261283cfa9f859fbfaece444af9eed7
- 契约 oa4rust/backend/crates/jpush_core_entity/tests/r626a_device_list_owner_fix_contract.rs sha256：d568a0c056afe12e1668bd5ad481a141f90ed0b03e216b726375f8f83609943a
- 文档自身 sha256 于交付时经 ops_stat 采集并写入报告。