# R616a jpush_core_entity device_delete 身份盲硬删越权修复审计（t223）

## 一、结论
- 判级：P1（已修复）。身份盲硬删越权：任意登录用户凭他人设备 id 可经 DELETE 端点硬删他人推送设备记录。
- 需有效会话 + 需知目标 id，不构成 P0；非仅 UI/L1 文案问题，非 0 级无害；故 P1。
- 已按同 crate 既有 device_get 范本补 owner 门，RED→GREEN 坐实；未在浏览器 GUI 手工验证（据实披露）。

## 二、根因（真父源码逐行）
- crates/jpush_core_entity/src/lib.rs:24-28 require_session 只验会话存在、丢弃 person_unique。
- :280-296 device_delete 原样 require_session(session)? 取值即丢弃，随后 jpush_device::Entity::delete_by_id(&id) 裸主键硬删，无归属校验。
- 路由 :267 jpush_core_entity_router / :271 delete(device_delete) 对外可达。
- DDL migrations/029_create_missing_core_entity_tables.sql:181 x_jpush_device(id PK, user_id NOT NULL, platform, token, create_time)；037_add_deleted_at.sql:51 补 deleted_at；user_id 即归属列。
- 范本：:30-37 DEVICE_READ_OWNER_PREDICATE（user_id 命中本人或 admin）；:40-60 device_read_allowed；:116-140 device_get 先判 owner 再按 find_by_id 区分 403/404。

## 三、修复（仅 device_delete，成功返回结构零改）
- device_delete 内：let sess=require_session(session)?；let allowed=device_read_allowed(&db.0,&id,&sess.person_unique).await?；
- !allowed 时 find_by_id 判定存在性，存在→Err(Forbidden)、不存在→Err(NotFound)；allowed 才 delete_by_id。
- 守卫零改：device_list/device_get/device_create/template/routes.rs/Cargo.toml/migrations/其他 crate 全部未动；device_create 仍裸 require_session（既有行为，本次不扩域）。
- git diff 相对当前 HEAD：src/lib.rs 15 insertions / 1 deletion，仅 device_delete。

## 四、验证链（真 oneshot + 真父 RED→GREEN）
- 契约 tests/r616a_jpush_device_delete_owner_contract.rs，2 用例不 ignore：
  - device_delete_owner_guard：独立 scratch 库 + 迁移 + seed alice/bob/admin + auth_role admin + 两台设备；经真 router layer(Extension(test_session)) + tower oneshot 直调 DELETE。
  - 断言：跨用户 alice 凭 d-bob → 403 且 DB 记录仍在；裸 → 401；本人 → 200 且已删；admin 凭 d-bob → 200 且已删；不存在 id → 404。
  - source_guard_owner_gate_present：源码函数体断言 device_delete 含 let sess/device_read_allowed/Forbidden/NotFound（全局 count 断言会误命中 device_create，已改为函数体精确断言）。
- RED：真父 lib.rs（sha 330496c6…）覆盖 + touch，定点 cargo test EXIT=101，2 failed，越权左 200 右 403。
- GREEN：恢复修复版 + touch，定点 EXIT=0，2 passed；lib.rs 恢复 sha 与修复版一致。
- V1 定点：EXIT=0，2 passed。
- V2 包级：cargo test -p jpush_core_entity（DATABASE_URL 指向已迁移基础库）EXIT=0，15+2+0 passed；基础库由一次性迁移测试建立，该测试文件用完已删（未入库）。
- V3：cargo fmt --check -p jpush_core_entity EXIT=0；cargo clippy -p jpush_core_entity -- -D warnings EXIT=0。
- 测试连接串与日志真实路径均以占位符记录：<TEST_HOST>:<PORT>、<PROBE_LOG_PATH>。

## 五、据实披露
- 执行期间 HEAD 由派发真父 91eaf3d5e… 前移至 af39071a…（他组 R616b 改 correlation_core_entity），写域不冲突；本修复基于真父 jpush lib.rs，相对当前 HEAD 仍是干净的 15+/1-。
- 未运行浏览器 GUI 手工验证，全部结论来自真 router oneshot 与源码断言。
- P1 排除 P0/L1/0：需会话与已知 id、影响单条设备记录、非提权/绕过认证；非纯 UI 文案。
- 一次性迁移测试仅用于建已迁移基础库使包级既有测试可跑，非交付物，已删除。

## 六、哈希（不含本文档自身 sha）
- 派发真父 HEAD：91eaf3d5e81bd24b8a1870c5ff36be5c50916be0
- 当前 HEAD：af39071ac0e737a8f980fb29f6ea927d8c684751
- 修复版 src/lib.rs sha256：263445d6b77dba4bea19ecaa2ed73b95b95eeb882bf02279a4c93bd279e3b1f2
- 真父 src/lib.rs sha256：330496c6eb1b519a521b49433f84c79fabd8706fff9c8ee752034cd0dea42d1c
- 契约 sha256：5abcebfe7f2ee64240372743ec2a6634b504f2c4117af1c8d3dffb2075651672
- RED 日志 sha256：27d962e3d0f8539aa097bd63305537c498568d690766880051384bf3943b8bc1
- GREEN 日志 sha256：c0d85d0a80c33cc98bc0f50e4b3c00f3b270cf984b9d2fe281e4de0e9fd83856
- V2 全量日志 sha256：39dbf1c3b2ffe41d269ec4f524a3a6218d5b2b7348e21d982cd8c98c4773845e
- runnerSha = sha256(JSON.stringify(meta)) 紧凑：327e5286927322340cfbad50bf82bc35b0f2ec720cb3bca0b65637d245373373
