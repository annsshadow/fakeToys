# R626b hotpic list 绕过 visible_scope 身份盲读修复审计

## 结论
- 判级 P2：`GET /api/hotpic/core/entity/list` 真父仅带 `db: Extension<DatabaseConnection>`、无会话门，裸 `hotpic::Entity::find()...limit(20)` 全表直读并回传他人 base64；需登录会话才可达，属越权读（可见范围失败），非数据破坏、非匿名、非写库，排除 P0/P1/L1/0。
- 修复：list 增 `session: Option<Extension<Session>>` + `require_session` 匿名 401 门置于 DB 访问前；可见范围 = 本人 `creator` 命中 OR admin OR 全局行（`application` 与 `info_id` 皆空），复用现成 `HOTPIC_OWNER_PREDICATE` 语义、参数绑定无注入面；成功返回结构（count+data）零改。

## 源码锚点（零改基线，sha256）
- `oa4rust/backend/crates/hotpic_core_entity/src/lib.rs` 311 行 sha256=`6dd62634d169ed055eba9dd72b8f9b02246fa75bab7793f5b6d196835570366b`：require_session(:26-30) 只验存在；HOTPIC_OWNER_PREDICATE(:36-43)；list(:72-110) 裸全表查询；路由(:283-297) GET list 可达。
- 修复后同文件 sha256=`bef384ddde1b63f07acf8d2f68eac15e9a2bd467460ebd27fa39a45eaa964249`（12632 字节），Out of scope `list_by_app_and_info`/`exists_check`/路由/Cargo/migrations/前端 均零改。
- `Session.person_unique` 现成字段（lib.rs:256）作为 actor；DDL 029/033 提供 `creator` 列。

## 验证链（真 RED→GREEN + 三 verify）
- 契约 `oa4rust/backend/crates/hotpic_core_entity/tests/r626b_hotpic_list_scope_contract.rs`（sha256=`aba5cefc5d26e40392d1fc8f3a63f82ba77e7227e1e2919c5b56a78aa1acd511`，3 用例不 ignore）：建独立 scratch 库跑真实迁移；seed alice/bob/admin+admin 角色+3 行热图（他人行含标记）；oneshot 经 router+Extension(test_session)。断言非 admin 仅见本人+全局且不得见他人 base64、裸调 401、admin 见全体、源守卫含 session 门/admin 谓词/全局行条件。
- 真父覆盖 RED：切回真父 lib.rs 后 `cargo test -p hotpic_core_entity --test r626b_hotpic_list_scope_contract` EXIT=101（3 failed：越权读到他人行/裸调 200/源守卫缺门），日志 sha256=`8b7a4a854a6a7f9a8ab2541ab431ffc56fde1b109bfba4365c34b5c1d91dc1c6`。恢复修复版 GREEN：EXIT=0（3 passed），日志 sha256=`5c9a268ca943077a3df32e030cc97de8cbcac49105686275ad3ed27ed390b316`。
- V1 定点 GREEN 同上 EXIT=0。
- V2 包级：迁移基础库上 `cargo test -p hotpic_core_entity -- --skip tests::tests::test_list_returns_success` EXIT=0（lib 17+契约 3+b72 1+doc 0 全绿），日志 sha256=`76e4d010e1415fe2ba184455148e5f8c7f3c8c253acec5c612d47ce1b918d9a1`。未迁移基础库时另有 4 个 environmental failed（relation 不存在），建迁移库后消除。
- V3 `cargo fmt -p hotpic_core_entity --check` EXIT=0（空日志 sha256=`e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855`）；`cargo clippy -p hotpic_core_entity -- -D warnings` EXIT=0（日志 sha256=`a2681a2842605ecaa7db85a519b2e16a3d986c0b1c1c0303104d159038a808f4`）。cargo 在 `oa4rust/backend` 执行。

## 据实披露（Fail Loud）
- **写域外旧断言预期行为变化**：`src/tests.rs` 的 `tests::tests::test_list_returns_success` 用 `mock_db()`（固定不可达 :1 地址、与 DATABASE_URL 无关）无会话 GET list 并断言 500。真父（无会话门→查库失败）pass 500；修复（匿名 401 门）fail（left 401/right 500 at src/tests.rs:29）。真父日志 sha256=`7dc2dfafaa2a73c191b2b3739c9a00f189b9df6047f22c3a43f00f0c3cdf6bf8`、修复日志 sha256=`faf3b34d914cbb6a8ad5d9cac03a90c95ec602d0a7743412a2877417398ab38a`。该文件在写域外不可改，故 V2 以 `--skip` 排除并如实披露：它是旧不安全行为的锁定断言，安全修复必然改变之。
- 测试数据库使用临时容器、端口以 `<TEST_HOST>:<PORT>` 占位；测试夹具中的口令形状为占位符，均非真值。
- 未跑浏览器 GUI；行为由 Rust oneshot 契约坐实。
- runnerSha=sha256(JSON.stringify(meta))（紧凑 JSON、不含自身 sha）=`81c5c635021f086f5a04f5ecbcff564ec18f5f75f943008bd1fe9871f36a1f07`；本文档文件 sha256 另列于提交记录。
