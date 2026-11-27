# R622a mind_save 身份盲写 IDOR 真修复审计

## 一、结论与判级
- 结论：P1 身份盲写 IDOR，已在写域内真修复（mind_save 更新分支补属主或 admin 门）。
- 非 P0：需有效会话（非匿名）、需知他人 mind id（UUID 随机非自增）；无匿名跨站利用链。
- L1-0：门在首个 DB 写前，复用既有 person_can_manage_mind（属主或 admin，fail-closed），零新增依赖。
- 修复范围：仅 mind_save 更新分支；body 身份自证不再被采信，person 始终取会话人。

## 二、锚点与根因
- 真父：56476ec5e5069e7201f3334c7c257c6d9bff9dff。
- 文件：oa4rust/backend/crates/mind_assemble_control/src/u2.rs。
- 根因：mind_save 更新分支（原 456-479 行）require_session 只验会话存在，UPDATE x_mind WHERE id=$1 AND deleted_at IS NULL 无 creator 归属谓词、无 person_can_manage_mind 门；sess.person_unique（450 行）仅作版本 INSERT 的 creator，不参与更新授权。
- 兄弟对照：folder_delete/mind_restore/mind_share/mind_share_cancel/mind_destroy 等全部写端点均调 person_can_manage_mind；mind_save 是唯一漏网写端点。
- 危害：任意登录者凭他人 mind id 覆写 name/content、把私有脑图 shared 置 true、并以自己身份写入受害者版本链（fileVersion 递增、creator 为攻击者）。

## 三、RED 到 GREEN 真执行
- oneshot 契约：tests/r622a_mind_save_idor_fix_contract.rs，2 用例，deadpool Pool + Router + Extension(test_session) + tower::ServiceExt 真实执行。
- 夹具隔离两处独立 schema 缺陷（见第五节），否则真父越权会被 schema 错误 500 回滚而掩盖逻辑缺陷。
- RED（真父）：alice 凭 bob 的 mind id POST mind/save 覆写，断言 403 实得 200（left=200 right=403，PIPE_EXIT=101），坐实身份盲写；裸会话 401、兄弟 mind_share 冒名 forbidden 反证均通过。
- GREEN（修复后）：伪造覆写 403 且 DB name/content/shared 未改、版本链 creator 无攻击者；属主本人 200 落库、admin 代改 200 落库；2 passed / 0 failed。

## 四、三 verify
- V1 定点 oneshot：RED EXIT101 到 GREEN EXIT0（2 passed）。
- V2 包级全量：cargo test -p mind_assemble_control --all-targets，31+2+1 passed / 0 failed / 3 ignored（DATABASE_URL 指干净已迁移库）。
- V3 fmt 与 clippy：cargo fmt -p mind_assemble_control -- --check EXIT0；cargo clippy -p mind_assemble_control --all-targets --no-deps -- -D warnings EXIT0。

## 五、行为变化与据实披露
- 行为变化：非属主非 admin 的 mind_save 更新由 200 变 403；body 中自证的身份字段不再被信任。
- schema 张力（独立待修项，非本轮写域）：029 迁移 x_mind_version_info.file_version 为 INTEGER，而 u2.rs 更新分支用 i64 读取（490 行反序列化 panic）；同表 folder_id TEXT NOT NULL 无默认值而版本 INSERT 漏列（500 回滚）。两缺陷并存时当前迁移下更新分支本会 500 回滚、身份盲写被掩盖；契约夹具 ALTER 隔离后方暴露。建议独立任务对齐迁移与源码类型。
- 未跑 GUI；V2 首轮用被夹具 ALTER 的库触发既有 security_gate_tests 的 i32 读取 BIGINT 失败，改用干净已迁移库后全绿，非身份门回归。
- 零新增依赖；GET/路由/Cargo/migrations/前端零改；新增回归契约入库；临时 PG 容器已清理。

## 六、sha256 分列
- u2.rs（修复后）= e825b1b87b3acc91c68afc105cd950f534b29286206e2af21291d2607937a178
- 契约（格式化后）= cdf6829eb6f7647e9fe31c7eaaecce06b34810c6467c7af4ce8dfd96db29df3d
- RED 日志 = 58e477f2968e898b77dc15167f59be97668f581d83474848d0776f35fdcbb76c
- GREEN 日志 = 3d7c833837bd80651ca15e45aceda8fcc47e232ec127157aea7e260ad69393fd
- V2 日志 = 772177d7b6b6ce62d1a34e936eb35bdfdee8d4703b19cca50c66707598dc773d
- runnerSha（meta JSON.stringify 后 sha256，无文件 IO）= 87c0c4c323670f3aee10bf7cf2f37ca9904465a5b87805b6e2ea6b96242d4e30
- 文件 sha256 与 runnerSha 分列，均为哈希值非环境真值；文档自身 sha256 入库前采集。
