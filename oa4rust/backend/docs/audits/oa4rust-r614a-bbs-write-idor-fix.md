# R614a bbs_core_entity 写端点身份盲越权修复审计（t215）

- 轮次/任务：R614a / t215（implementation，真修复 RED→GREEN）
- 真父 HEAD：7f79cb7e0764e51e1207f20bc8952ee4305208a9（工作树同，git status 目标目录干净）
- 裁定：P1 — 任意登录人可凭 id 改删他人论坛板块/主题，跨用户破坏性写落库
- 写域：仅 crates/bbs_core_entity/src/lib.rs 的 8 个写端点；新增契约与本文档
- 会话/端口：测试用本地回环 <TEST_HOST>:<PORT>（trust 无密码，真值脱敏不入仓）

## 一、缺陷与根因

- `require_session(session)?` 在 9 个写端点只验会话存在、丢弃 person_unique（lib.rs:27-31）。
- forum/section/subject 的 create/update/delete 共 9 端点此前无身份/角色门禁，
  任意登录人可对任意 id 执行改/删；subject 有 author_id 归属亦被忽略。
- 真父 oneshot 实证：非 admin 凭他人 id POST/DELETE 均 200 且数据库被篡改。
- create_subject 例外：原本已用 resolve_author 做"作者或admin"门（lib.rs:593），零改。

## 二、修复（源码 diff 精确 38+/8-，仅 8 端点）

- create/update/delete forum（:269/:315/:377）与 create/update/delete section
  （:410/:463/:542）：保留会话值 `let sess = require_session(session)?;`，
  入口加 `if !is_admin_db(&db.0, &sess.person_unique).await { return Err(AppError::Forbidden); }`。
- update_subject（:624）/delete_subject（:712）：保留会话值，find 后校验
  `m.author_id == sess.person_unique || is_admin_db(...)`，否则 Forbidden（403）。
- create_subject 未改；成功返回结构/字段、UPDATE/DELETE 实体逻辑、会话门语义全部零改。
- 门禁策略依据（据实）：forum/section 实体无 creator/owner 列，属全局管理对象，
  对齐兄弟 bbs_assemble_control 管理面 delete_forum/reply/subject 的 admin 门语义；
  subject 有 author_id 作者归属，故用"作者本人或admin"门，而非一刀切 admin。
- 守卫零改：require_session/is_admin_db/resolve_author 定义、routes.rs、Cargo.toml、
  migrations、其他 crate、bbs_assemble_control 均未触碰。

## 三、契约与 RED/GREEN

- 契约 crates/bbs_core_entity/tests/r614a_bbs_write_idor_contract.rs：4 个 #[tokio::test]/#[test]，
  不 ignore；oneshot 经 bbs_core_entity_router(db)+layer(test_session(who))，独立 scratch 库避免并发冲突。
- 覆盖：跨用户写 403 且 DB 未篡改（field/count 前后断言）、本人/admin 200 且落库、裸请求 401、
  create_subject 冒名 403、source_guard 源码守卫（admin 门 ≥8、作者门、resolve_author 保留）。
- RED（真父备份覆盖 lib.rs + touch）：EXIT=101，4 failed；越权断言 left:200 right:403，
  source_guard got 0。日志 sha256（文本，脱敏）：dde0cca2179003b00ba4ad8e69ed1bfa6aec518dab21e87e4a995ce1b80a2bf8。
- GREEN（修复版恢复 + touch）：EXIT=0，4 passed。日志 sha256：4052ce2ddb7adecdb4c86f0c936b28891635b277ff876f44ba4a7f4e427b5c5f。
- 真父 lib.rs sha256：e45ba7a71db7bfb102837bb38448f770d76deafe87293d0f678aab4fb97c8da8；修复版 lib.rs sha256：418b9f538a37f6fd250848473e082d4a80dee6d4041fa71a306a13cfc424de42；契约 sha256：3ba0356afd84bf3b6c68bdbcbb0ade504d0b5db1317aa5fcc39e051915c26550。

## 四、三 verify（包级，cd oa4rust/backend）

- V1 `cargo test -p bbs_core_entity --test r614a_bbs_write_idor_contract`：EXIT0，4 passed。
- V2 `cargo test -p bbs_core_entity`：EXIT0，35+4+0 passed。
- V3 `cargo fmt -p bbs_core_entity -- --check` 与 `cargo clippy -p bbs_core_entity -- -D warnings`：均 EXIT0。

## 五、据实披露

- V2 基础库需先按 migrations 迁移；既有测试 o10_forum_create_roundtrip_decode_ok 的
  fixture 会话（o10-rt-user）在修复后因非 admin 收到 403（该测试意图是 O10 类型解码，非权限），
  已在基础库为该 fixture 会话补 admin 角色使其按原意图通过；未改任何源码或测试文件。
- 一次性迁移测试建库后已删除；临时 PG 容器用后清理。
- 未跑浏览器 GUI 真页面，以 oneshot + 源码守卫断言替代。
- 文档不含自身 sha；runnerSha 为元数据紧凑 JSON 的 sha256，与文件 sha256sum 分列如下。

## 六、校验值

- runnerSha（sha256 of compact JSON meta）：b3d6f44ffcd34dee56d3a4a27645af6b5e4d61bca4d4fbf4751818fa63aa6fdc
- 文档文件 sha256（sha256sum）：见提交后 file sha256，不含于本文自身。
