# R615b：correlation_core_entity delete_by_id 身份盲越权甄别（t220）

## 结论

据实裁决 **P1**：`correlation_core_entity::delete_by_id` 只验会话存在、丢弃会话身份，
按主键 `find_by_id` 命中记录后直接把 `deleted_at` 置为当前时间并 `update` 落库；
任意登录用户凭他人的关联记录 id 即可软删该记录。属身份盲越权破坏性写。
需会话且需知 id、软删可逆、无敏感信息回显，排除 P0；仅该端点局部失效，排除 L1/0。

## 逐行证据（源码）

- `crates/correlation_core_entity/src/lib.rs:26-30` `require_session` 仅验会话存在，丢弃 `person_unique`。
- `crates/correlation_core_entity/src/lib.rs:199-228` `delete_by_id`：`:204` `require_session(session)?` 丢弃返回值；
  `:205` `correlation::Entity::find_by_id(&id).one(&db.0)` 仅主键无归属；`:212-222` 置 `deleted_at` 并 `active.update` 软删；
  不存在 id 时返回 `Ok(ActionResult::error)`（HTTP 200 error envelope，语义保留）。
- `crates/correlation_core_entity/src/entities/correlation.rs:6-18` 表 `x_corr_c_correlation` 列仅
  id/source_type/source_id/target_type/target_id/weight/create_time/deleted_at，无 creator/owner/person_id（NO-OWNER-COL）。
- `migrations/029_create_missing_core_entity_tables.sql:109-120` DDL 确认无归属列。
- `crates/correlation_core_entity/src/lib.rs:237-249` 路由 `delete/{id} → delete_by_id` 注册；
  `backend/src/lib.rs:191` `.merge(correlation_core_entity::router(...))` 可达。
- 兄弟正确范式 `crates/hotpic_core_entity/src/lib.rs:32-58/250-274`：
  `HOTPIC_OWNER_PREDICATE`（creator=$2 或 admin 角色）+ `ownership_error` 区分 403/404，
  `delete_by_id` 保留会话取 actor 并带谓词 UPDATE、rows_affected 为 0 报归属错误。

## 甄别测试（真执行）

临时 PostgreSQL（占位 `<TEST_HOST>:<PORT>`，trust 无密码）apply 全量迁移后，
经真 router oneshot 直调 `delete_by_id`（日志占位 `<PROBE_LOG_PATH>`）：

1. 越权：alice 凭 bob 的 `c-bob` id DELETE → HTTP 200，且查询 `deleted_at IS NOT NULL` 为真（越权软删坐实）。
2. 裸请求（无会话）→ HTTP 401；这是当前唯一存活的门。
3. 本人删自己的 `c-alice` → HTTP 200 且软删落库。
4. admin 删 `c-third` → HTTP 200 且软删落库（无 owner 列，admin 同为普通会话）。
5. 不存在的 id → HTTP 200 error envelope，原语义保留。
6. 源码守卫：丢弃会话值、`find_by_id` 仅主键、无 `require_owner`/`OWNER_PREDICATE`/`ownership_error`、
   `deleted_at: Set(...)` 软删、实体无归属列 —— 全部命中。

命令：`cd oa4rust/backend && DATABASE_URL=<TEST_HOST>:<PORT> cargo test -p correlation_core_entity --test <PROBE> -- --nocapture --test-threads=1`
结果：EXIT 0，2 passed / 0 failed / 0 ignored；输出文本 sha256 = `2bb2a01ea1c48a834cb8c1b8801e14cfef42626f11fa8f4deda6b9bbfa62f5d0`。
临时探针与 scratch 库用后即删；零源码改已复核（目标文件 sha256 见下）。

## 判级讨论（据实）

- **NO-OWNER-COL 反向考量**：该表确无归属列，不能像 hotpic 那样按 creator 过滤。
  但这不消灭"任意登录人可软删关联记录"的越权事实；正确修法是按 ownerless 系统表加 admin 门
  （参照兄弟热图谓词与本仓库 R4 对 `x_correlation` 的 admin 门裁决），而非维持现状。
- **范围区分**：台账中的 `correlation_service_processing` 属另一 crate、`x_correlation` 属另一张表，
  R4 已单独处理，本问题不是其重复计数；`src/tests.rs:100-119` 只测无会话 401，不覆盖越权。
- **前端消费点**：`ProcessWork.vue:2651-2653/2771` 仅冒烟探针调用 list/create/delete/by，无实际 UI 删除入口；
  可感知面有限，故不判 P0；但后端端点已落库破坏，仍为 P1。
- 未跑浏览器 GUI 真页面；以 oneshot 真 router + 源码真执行替代，据实披露。

## 溯源码哈希（sha256）

- crates/correlation_core_entity/src/lib.rs = `033a6c74f74a745e63245b77143d5f96462e4f9c804b30966d2e8981d8f24fa0`
- crates/correlation_core_entity/src/entities/correlation.rs = `08f0f75a57c769eab120daab83a945d6ab77b49a7f827a891100634aa5052c1f`
- migrations/029_create_missing_core_entity_tables.sql = `4fcc3848a770ff91e467bed7adbd4c138024b23d65937d998d5dbe97b4c5f6c2`
- crates/hotpic_core_entity/src/lib.rs = `6dd62634d169ed055eba9dd72b8f9b02246fa75bab7793f5b6d196835570366b`
- crates/correlation_core_entity/src/tests.rs = `ce2a52b7292eb3b41549e4d91e652e6a101660ccd4e1fadcc79754f591694dc9`
- probe output sha256 = `2bb2a01ea1c48a834cb8c1b8801e14cfef42626f11fa8f4deda6b9bbfa62f5d0`
- runnerSha（紧凑 JSON meta） = `070bf08aaa2c3618f112468534ada9c0f47d2899da26321793e8e2eebca31171`

（文档 sha256 由提交前重算写入，不含自身哈希。）