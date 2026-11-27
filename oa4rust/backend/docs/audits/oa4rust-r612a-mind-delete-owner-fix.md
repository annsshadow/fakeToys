# oa4rust R612a mind 删除端点身份盲 IDOR 修复审计

- 结论：P1 真修复。delete_mind / delete_folder 两端点由裸 ORM 删除改为 owner-scoped SQL 删除；跨用户凭 id 删除被拒，合法本人/管理员保持可用。
- 任务：t207（implementation）；派发父 HEAD `fe330ade932f1235edecd496d3f39abb5995dad5`，工作树 HEAD 同值，目标文件字节与真父备份一致。
- 写域：`crates/mind_core_entity/src/lib.rs` 仅两端点；新增契约 `crates/mind_core_entity/tests/r612a_mind_delete_owner_contract.rs`；本文档。其余端点/守卫/路由/依赖/迁移零改。

## 缺陷与修复
1. 真父 `src/lib.rs:389` / `:548`：`require_session(session)?` 仅验会话存在、丢弃返回值；`:390` / `:549` 直接 `mind_mind::Entity::delete_by_id(id)` / `mind_folder::Entity::delete_by_id(id)`，无 creator 归属判断，任意登录用户可凭 id 删除他人 mind/folder（破坏性写落库）。
2. 修复：两端点保留会话值 `let session = require_session(session)?; let actor = session.person_unique;`；删除改为 `DELETE FROM x_mind_base_info WHERE id = $1 AND ({CREATOR_OWNER_PREDICATE})` / `DELETE FROM x_mind_folder_info ...`，绑定 `id`、`actor`；复用既有 `CREATOR_OWNER_PREDICATE`（本人或 admin）原常量，未改动。
3. `rows_affected() == 0` 时调用既有 `ownership_error(&db.0, table, &id)`：记录存在但非本人→403，不存在→404；成功仍返回 id + rowsAffected。SQL 事件查询/其余写端点 `create_mind`/`update_mind`/`create_folder`/`update_folder`/`create_version` 零改。

## 契约与验证
- 契约 3 用例不 ignore：delete_mind_owner_guard / delete_folder_owner_guard / source_guard_owner_predicate_and_no_orm_delete；经 `mind_core_entity_router(db)` + `Extension(test_session)` + tower oneshot 真 DELETE，独立 scratch 库。
- 五路径断言：alice 凭 bob id→403 且 bob 记录仍存在；无会话→401；本人→200 且记录消失；admin 代删→200 且记录消失；不存在 id→404。源码守卫确认谓词复用、4 处 actor、4 处 ownership_error、ORM 裸删 0 残留。
- 真父 RED：覆盖真父 lib.rs 后定点测试 EXIT=101，3 failed（两越权用例 left:200/right:403，source_guard actor 计数 2/4）；日志 SHA-256 `5916f2e8...960a1d`。
- 修复版 GREEN：恢复修复版后 EXIT=0，3 passed；日志 SHA-256 `c82681db...78dfe0c1`。
- 三 verify 全 EXIT=0：V1 定点 3 passed（日志 sha `d8ed0640...02093c801`）；V2 包级全量 lib 31 passed + 契约 3 passed + doc 0（日志 sha `90dc892f...138a53`）；V3 `cargo fmt -p mind_core_entity -- --check` 空输出、`cargo clippy -p mind_core_entity -- -D warnings` 通过（日志 sha `24a0f323...b9e69203bd`）。基础库 `oa_r612a_base` 由一次性迁移测试建立，该测试验证后已删除。
- workspace 级 `cargo fmt --check` 存在真父既有的其他 crate 格式差异，非本任务引入；本任务采用包级 fmt/clippy 并据实披露。

## 真值与脱敏
- 测试库连接串仅以占位符 `<TEST_HOST>:<PORT>` 表示，仓库零真值主机/端口/口令/密钥。
- 源码哈希：真父 lib.rs `b8668ff9...8855e9`；修复版（格式化前）`35f47995...eca8eb622`；格式化后 `0b716630...ed776e7da`；契约（格式化后）`70a587c8...e8483e9a`。
- runnerSha（紧凑 JSON 元数据 SHA-256）：`4b49f0d622a543f15034b042919f63829e795ad21020de215262d2b8bbd163bf`，与本文件字节 SHA-256 分列；本文件不含自身 SHA 以避免自引用。
- 未跑浏览器 GUI 真页面；以 oneshot 真 handler + 落库断言 + 源码守卫替代。
