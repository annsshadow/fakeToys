# R632 x_org_custom 四端点身份盲 IDOR 真修复审计

## 一、结论
x_org_custom 的 custom_list/custom_create/custom_update/custom_delete 四端点原缺失身份归属校验：custom_list 匿名可按 identityId 裸读他人字段值（P2），三个写端点任意登录者凭身份 ID/记录 ID 可跨人挂字段、改名、软删（P1，需有效会话与知晓 ID，非匿名非遍历，故不夸 P0）。判级沿用 R631a 甄别（t297）与终审（t299）确认结论，非新判级。本轮已完成真修复。

## 二、锚点与归属依据
- organization_core_entity/src/lib.rs:32-36 require_session 仅验会话存在但原调用方丢弃返回值。
- lib.rs:40-45 CREATOR_OWNER_PREDICATE 定义归属：本人 creator=actor 或 admin（经 auth_person/auth_person_role/auth_role 反查）。
- lib.rs:49-67 ownership_error 区分 403（存在非本人）/404（不存在）。
- x_org_custom 表无 creator 归属列（migrations/029_create_missing_core_entity_tables.sql 中仅 id/identity_id/field_name/field_value/deleted_at），x_org_identity 有 creator 列，故归属经 identityId 反查 x_org_identity 推导，复用同文件 bind_create 的 EXISTS 谓词口径。
- 路由 organization_core_entity_router 含 .layer(Extension(db))，oneshot 直调可行。

## 三、修复内容（四端点同一 lib.rs 单 impl）
- 新增 async fn ensure_identity_owner(db,actor,identity_id)：用 CREATOR_OWNER_PREDICATE 拼 EXISTS(SELECT 1 FROM x_org_identity ...) 校验，非本人非 admin 走 ownership_error。
- custom_list：GET 增 Option<Extension<Session>> + require_session 取 actor，ORM 查询前 ensure_identity_owner。
- custom_create：保留 actor=require_session(session)?.person_unique，INSERT 前 ensure_identity_owner(req.identity_id)。
- custom_update/custom_delete：保留 actor，find_by_id 取 model.identity_id 后、update/软删前 ensure_identity_owner。
- 门均在首个 DB 读/写前；匿名 401、非本人非 admin 403、本人/admin 200、不存在 404 语义保留。

## 四、RED→GREEN 证据
- RED（真父未改源码）：2 用例 0 passed/2 failed（自定义列表匿名臂 left200/right401、源码守卫缺 helper），缺陷坐实，EXIT=101。
- GREEN（修复后）：定点契约 2/2 passed，EXIT=0。独占本地 PG（非回环映射端口避开他组）用后即清。

## 五、三 verify（Fail Loud 披露）
- V1 定点：cargo test -p organization_core_entity --test r632_org_custom_owner_fix_contract → 2 passed，EXIT=0。
- V2 包级：首跑 33 passed/7 failed。6 个 r6w2 夹具因 DATABASE_URL 目标库无迁移表（code 42P01，relation 不存在，写域外环境问题）；1 个 tests::tests::test_custom_list_returns_success@src/tests.rs:93 原断言匿名 500，现因正确加门变 401（正确行为变化，写域外）。不改写域外文件，skip 重跑：cargo test -p organization_core_entity -- --skip r6w2 --skip test_custom_list_returns_success → lib 33 passed/7 filtered + 契约 2 passed + doc 0 passed，EXIT=0。
- V3a fmt-check：首跑仅契约长行 diff，cargo fmt -p organization_core_entity 后 --check EXIT=0。
- V3b clippy：cargo clippy -p organization_core_entity --all-targets --no-deps -- -D warnings → EXIT=0，零告警。

## 六、行为变化与 SHA
- 行为变化：custom_list 匿名从 200/500 变 401；跨人写从 200 变 403；本人与 admin 路径不变。
- 修复对象 organization_core_entity/src/lib.rs sha256=c03c9cf6a0ea359756daea20d72a46cbccaf08113087f695cd06063970d80d2d（42333字节/1197行）。
- 契约 organization_core_entity/tests/r632_org_custom_owner_fix_contract.rs sha256=ebdadd0cfb157eb4cc6a938009e89cd2108a4b4c4879e4bc77d8613e24c3523b（12118字节/382行）。
- runnerSha=cad97c4ec4450b72e18ebb1482b5a595991bb4a7eab1173df85bcc0a1ba46720（对含上述两 SHA 与验证结果的 meta JSON 取 sha256）。三者分列，runnerSha 非文件哈希。