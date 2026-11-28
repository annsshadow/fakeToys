# R633 meeting add_participant 身份盲邀请真修复审计

## 结论
真修复完成。给 meeting crate 的 add_participant 端点补上"会议存在校验 + 属主/管理员门"，
且置于首个 DB 写之前：跨会议非属主 403、不存在会议 404 且无幽灵邀请、属主本人与管理员 200 真落库、
匿名 401。判级沿用 R631b 甄别(t298)与终审(t300)已定 P2，非新判级。零真值，占位域名。

## 立项与范围
- 真父: 1afe8e31acb3b707830b404fce60a55d12edd43b
- 写域(精确三文件): M oa4rust/backend/crates/meeting/src/lib.rs；A oa4rust/backend/crates/meeting/tests/r633_meeting_participant_owner_fix_contract.rs；A 本文档。
- 范围外(零改): docs/ops/current-state.md、meeting_assemble_control/**、前端、路由、Cargo、migrations。
- 判级依据: R631b(t298)+终审(t300)已定 P2；队长 m02118 明确不采信拔高 P1。

## 机制锚点(真父)
- oa4rust/backend/crates/meeting/src/lib.rs:483 add_participant(pool, session:Option<Extension<Session>>, Path<String>, Json<Value>)。
- :489 require_session(session)? 只验会话存在即丢弃身份；:499 INSERT INTO x_meeting_invite 无归属门、不校验会议存在。
- 兄弟正确范本 oa4rust/backend/crates/meeting_assemble_control/src/lib.rs:2487 u2_meeting_owner_gate 查 creator 后 require_owner，但行不存在放行会留幽灵，故本端点须额外存在校验。
- require_owner 在 shared/src/middleware/rbac.rs:582（admin 或本人 trim/lowercase 相等放行，否则 Forbidden，fail-closed）。

## 修复实现
在首个 DB 写前插入(lib.rs:489-501，12 行):
- let session = require_session(session)?（遮蔽原绑定，保留会话引用）；
- client.query_opt("SELECT creator FROM x_meeting WHERE id = $1", &[&meeting_id]) → Option<Row>；
- .map(|r| r.get::<_,String>("creator"))；creator.ok_or(AppError::NotFound)?（不存在会议 404）；
- shared::middleware::require_owner(&pool.0, &session, &creator).await?（跨人 403，本人/admin 放行）；
- 之后才执行原 INSERT，行为与返回结构不变。

## 决定性实测(独占临时 PG, 环境变量注入连接串, 用后即清)
契约 cargo test -p meeting --test r633_meeting_participant_owner_fix_contract -- --nocapture:
- RED(真父): EXIT=101；跨会议非属主返回 success/added:true(缺陷坐实)；裸401臂 PASS。
- GREEN(修复后): EXIT=0；2 passed/0 failed。
  R633_ARM1_CROSS_FORBIDDEN=403/no_write（alice 对 bob 会议，DB 未改）
  R633_ARM2_GHOST_NOTFOUND=404/no_write（不存在 meeting_id，无幽灵记录）
  R633_ARM3_OWNER=success/row=1；R633_ARM4_ADMIN=success/row=1
  R633_ARM5_NO_SESSION=401；R633_UNAUTH_APP_LAYER=401
  R633_SOURCE_GUARD=existence_and_owner_gate=1
- V2 包级 cargo test -p meeting --all-targets: EXIT=0（lib 16 passed/1 ignored；nullable 1 passed；契约 2 passed）。
- V3 cargo fmt -p meeting -- --check: EXIT=0；cargo clippy -p meeting --all-targets --no-deps -- -D warnings: EXIT=0。

## 三分裁决
- 可达真缺陷：0（本任务为修复，原 P2 身份盲邀请+幽灵写已被前置门消除）。
- 不可达 L1：0。
- 正常：会话门 401、跨人 403、不存在 404、本人/admin 200 真落库、兄弟 owner gate 与源码守卫一致。

## Fail Loud
- 三 verify 全绿，未跳过任何检查；契约测试自建 scratch 库并自管迁移，包级测试指向独立持久迁移库。
- 监控串经环境变量注入，文档只写占位 <TEST_HOST>:<PORT>，无密钥/真域名/IP/个人路径。
- ops_edit_file 保留 mtime 曾致 cargo 链接旧 rlib 假失败，已用 touch + cargo clean -p meeting 强制重编译后复验。

## SHA 清单(runnerSha 与文件字节 sha 分列)
- runnerSha(sha256(JSON.stringify(meta)))=ef10716346dcae6724c9c78fa7a15a6dae91068fe1f4418d2afaf0250a60210b
- lib.rs(字节)=b464054977953b07f95b3d4a60fc566b77cd0019a58c14f0469941612d170543
- 契约测试(源码树与证据副本一致)=f394c173ae56ebb5c23a97680b0327fafbdc4512622236ad950812b48b538699
- red.log=2f3962b724488779b03d6b7c92a5759f2255401415bafec29aa4e1448fc811b0
- green.log=d556552c7ac5338e410ced3047445bee95e39f4c1bb62730f31dbd42d596139e
- v2-all-targets.log=70bddc1717ee1df536b03b51ab2ea7491c59c4f331327d9df611d915eec09738
- v3-clippy.log=7a593d93adf27b70b7c26dfebec020d26165bd11d0b9dc4fd6c54710bd227573

## 修复方向(已落地)
前置存在校验+共享归属门已实现并全绿；无需后续动作。兄弟 meeting_assemble_control 的
u2_meeting_owner_gate 行不存在放行语义仍在，如需消除其幽灵语义另立任务，不在本次写域。
