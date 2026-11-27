# R611a mind_core_entity 删除端点身份盲 IDOR 甄别

## 结论
判 **P1**：`delete_mind` / `delete_folder` 两端点仅验会话存在、不校验归属，任意已登录用户凭他人 mind/folder 的 id 可删除他人记录，落库坐实。两端点同族合并 1 项。零源码改动，仅新增本脱敏审计文档。

- 真父：`3adb2a97bd972296ba7649e6ea6a70c174b3e695`（git 核实；git diff 真父 -- crates/mind_core_entity/` 零输出）。
- 本轮唯一产出（写域）：`oa4rust/backend/docs/audits/oa4rust-r611a-mind-delete-idor-audit.md`（本文档）。

## 立项与范围
端到端甄别 mind_core_entity crate 两把删除端点是否存在身份盲越权写，并与同 crate 兄弟 update 端点的归属门作对照。范围仅甄别与取证，不修改任何源码。

## 机制锚点（file:line）
- `crates/mind_core_entity/src/lib.rs:27-31` `require_session` 仅 `session.map(|Extension(s)|s).ok_or(AppError::Unauthorized)`，只提取、不校验归属。
- `crates/mind_core_entity/src/lib.rs:384-408` `delete_mind`：:389 `require_session(session)?` 丢弃返回值只验存在；:390 `mind_mind::Entity::delete_by_id(id.clone()).exec(&db.0)` 按主键盲删，无归属谓词、无管理员门。
- `crates/mind_core_entity/src/lib.rs:543-567` `delete_folder`：:548 同上丢返回值；:549 `mind_folder::Entity::delete_by_id(id.clone()).exec` 同族盲删。
- 路由接线 `crates/mind_core_entity/src/lib.rs:651` `delete(/api/mind/core/entity/mind/{id}, delete_mind)`、:654 `delete(.../folder/{id}, delete_folder)`，:656 `.layer(Extension(db))`；`crates/mind_core_entity/src/routes.rs:4-5` 转发。
- 兄弟正确范本 `crates/mind_core_entity/src/lib.rs:317-380` `update_mind` / :471-539 `update_folder`：:349-352 / :505 SQL `WHERE id = $1 AND ({CREATOR_OWNER_PREDICATE}) RETURNING id`，:368-370 / :524 rows_affected==0 → ownership_error；常量 `CREATOR_OWNER_PREDICATE` :99-104 含 creator 匹配与 admin 放行。删除端点完全没有这层门。

## 方法与决定性实测
独立临时 PG（新端口 15469，trust 认证，与他组实例隔离），全量迁移后 seed 两名主体 A/B 各一 mind + folder + 管理员角色；用 `crate::router(db)` 装配 axum Router，`oneshot` 发真 HTTP 请求（Extension 注入测试 Session）。真测命令：

```
cd oa4rust/backend
DATABASE_URL=<本地回环测试库> cargo test --test r611a_mind_delete_idor_audit -- --test-threads=1
```

| 测试 | 结果 | 证据 |
| --- | --- | --- |
| r611a_delete_mind_cross_user_idor | ok | 无会话 DELETE → 401 且数据未删；A 凭 B 的 mind id DELETE → 200 且 B 行真删；反向 B 删 A → 200 真删 |
| r611a_delete_folder_cross_user_idor | ok | 同构：无会话 401；A 凭 B 的 folder id → 200 真删；反向 B 删 A → 200 真删 |
| r611a_owner_and_admin_delete_succeed | ok | 本人删自己 mind → 200 真删；admin 删 mind / folder → 200 真删 |
| r611a_sibling_update_enforces_owner | ok | A 冒名改 B 的 mind / folder → 403 且 B 行未变（兄弟归属门未误伤） |
| r611a_source_guard_delete_lacks_owner_predicate | ok | 两删除函数体含 delete_by_id + require_session，但不含 require_owner / require_admin / CREATOR_OWNER_PREDICATE |

汇总：`test result: ok. 5 passed; 0 failed; 0 ignored`， EXIT 0。

## 三分裁决（据实，不预设）
- **P1（1 项，两端点合并）**：任意已登录用户可凭 id 删除他人 mind/folder，破坏数据完整性与可用性；删除是不可自愈的破坏性写，风险高于读型 IDOR。真实可达：路由在 `lib.rs:651/654` 注册并经 `router` 暴露。
- **P0 排除**：需有效会话且需知目标 id，非匿名、非无差别批量，不满足 P0。
- **L1 排除**：已落库坐实跨主体删除成功（非仅静态可疑），非低置信。
- **0 排除**：跨用户越权删除为真实缺陷，非误报。
- 据实披露：本次仅后端甄别，未定位到具体前端 Vue 组件消费点；这不影响后端路由真实可达与 oneshot 落库证据，仅说明前端调用链未在本轮范围内追踪。
- 据实披露测试锁定误判：`security_gate_tests_b81.rs:23-83` 仅覆盖无会话 401 与 require_session 单元，只挡匿名不挡已认证；未覆盖跨用户冒名删除，致该缺陷长期漏网。

## Fail Loud
- 临时 PG 使用独立端口，未触碰他组实例；夹具、日志用后即删；DB 不可用则 SKIP 不冒充 pass（本次 DB 可用，实测通过）。
- 全量迁移以 `shared::migrate::run_migrations` 执行；scratch 库用后 DROP。

## SHA 清单（runnerSha 与文件字节 sha256sum 分列）
- `runnerSha` = `6dbb402c8aa6b42dfbd7ff24f4dc3b27312af792eabc2a2fbb6d818774cb1e0d`（= sha256(JSON.stringify(meta))，meta={task:t203,round:R611a,verdict:P1,trueParent:3adb2a97b…,test:5 passed / 0 failed,fixture,log}）
- 文件字节 sha256sum：
  - `crates/mind_core_entity/src/lib.rs` = `b8668ff9c06674ca0cf3165fcfd18edcaf8a7b1440212d4e9f08eca5118855e9`
  - `crates/mind_core_entity/src/routes.rs` = `35b10cca6a092b97c73ed805f4356e0a479a24cc96df1af54be6d3e66e039600`
  - 夹具 `tests/r611a_mind_delete_idor_audit.rs` = `e4d182949394e49d91bdcbf8b9969556a5ded96db4f59f11964d3747d87963cc`（用后删除）
  - 日志 `test-results/r611a-run.log` = `9da143f4c78c7933913b4f9832b72d6cb3e3a15bb664199204957ef99f6653f5`（用后删除）

## 修复方向（供后续实施，本轮不改）
参照兄弟 update：在 `delete_mind` / `delete_folder` 中接收 `require_session` 返回的 session，按记录 id 取 creator，套用 `CREATOR_OWNER_PREDICATE`（或等价归属查询 + `require_owner` / `require_admin`），无权限返回 403，避免按主键盲删。
