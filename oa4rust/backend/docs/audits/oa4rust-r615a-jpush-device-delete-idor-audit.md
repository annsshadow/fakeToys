# R615a jpush_core_entity device_delete 身份盲 IDOR 甄别

## 结论
判级 **P1**：`device_delete` 对任意已认证用户开放「凭 id 硬删他人推送设备」，无 user_id 归属门；跨用户删除在真库落库生效（行物理消失），破坏数据完整性。零源码改，本报告为唯一交付。

## 根因（oa4rust/backend/crates/jpush_core_entity/src/lib.rs）
- :26-28 `require_session` 仅 `session.map(|Extension(s)|s).ok_or(Unauthorized)`，只验会话存在，不校 person_unique。
- :280-296 `device_delete`：:285 `require_session(session)?;` 丢弃返回的 Session（连 person_unique 都不取）；:286 `jpush_device::Entity::delete_by_id(&id).exec(...)` 按主键裸硬删，无归属谓词、无存在性/所有权区分。
- 同 crate 读面正确：:30-37 `DEVICE_READ_OWNER_PREDICATE` 按 user_id 或 admin；:40-60 `device_read_allowed`；:121-123 `device_get` 取 `sess.person_unique` 传入读门，无权存在→403、不存在→404。写面完全绕过该门。
- 次候选（据实单列、不硬凑入主结论）：:156-161 `device_create` 同样丢弃会话返回值，:163-167 直接采信 body 的 `userId`，理论可冒名注册设备；本次未做真测试坐实，仅作后续线索。

## 真实可达性
- 路由 crates/jpush_core_entity/src/lib.rs:265-276 `jpush_core_entity_router`：:271 `delete("/api/jpush/core/entity/device/{id}", device_delete)`，:273 `device/create`，均 `.layer(Extension(db))`。
- backend/src/lib.rs:194 `.merge(jpush_core_entity::router(sea_orm_pool.clone()))` 真实挂载到主路由。
- 兄弟正确对照 crates/hotpic_core_entity/src/lib.rs:250-274 `delete_by_id`：`HOTPIC_OWNER_PREDICATE` + `ownership_error`（无权 403/404），证明同类删除本应收归属门。

## 真测试（临时 PG scratch 库 + oneshot 真 router）
命令：`DATABASE_URL=<占位> cargo test --test r615a_jpush_device_delete_idor_audit -- --nocapture`
结果：**3 passed / 0 failed**（source_guard + sibling_get + device_delete_idor 三测试全绿）。
- 裸 DELETE 无会话 → 401，且 bob 行未删。
- alice 凭 bob 的 device id DELETE → 200、`deleted=true`，回查 bob 行物理消失（硬删坐实）。
- 反向 bob 删 carol → 200 且行消失；本人删自己 → 200；admin 删他人 → 200。
- 不存在 id → 200、`deleted=false`（无 404 语义，佐证盲删）。
- 兄弟 `device_get`：alice 读 bob → 403、读自己 → 200、admin 读 bob → 200、不存在 → 404（读门正确，反证写门缺失）。
- 源码守卫断言：delete 丢弃会话且裸 `delete_by_id`、get 走 `device_read_allowed`，通过。

## 三分判定
- **P1（成立）**：管理语义的写端点对任意登录人开放，可跨用户真实硬删他人设备，破坏数据完整性；写比读破坏性强。
- 非 P0：需有效 Session（非匿名），且需知道目标 device id，故不夸大为未认证任意写。
- L1 排除：缺陷在认证之后可达，认证前层不构成此缺陷路径。
- L0 排除：非匿名任意写；`require_session` 确实拒绝无会话（401 已实测）。
- 两端点合并为 1 项：device_create 未真测，不并入 P1 计数。

## Fail Loud 与披露
- **测试锁定**：security_gate_tests_b83.rs:23-55 仅测无会话 401；security_gate_tests_idor_b7.rs 只覆盖读门 device_get；台账对 `device_delete` 零专项命中——故身份盲写长期漏网。
- **前端**：本次仅后端甄别，未定位 .vue 消费组件；不影响后端端点真实可达与上述真测试结论。
- **零源码改**：`git diff <trueParent> -- crates/jpush_core_entity/` 零输出；仅新增本报告文档。
- **脱敏**：文档与日志不含真值；临时库连接串/端口一律占位符，测试身份为假名。
- **临时资源**：scratch 库与夹具用后即删，临时 PG 容器用后即移除；未触碰他组容器。

## SHA 清单（文件 sha256sum 与 runnerSha 分列）
- runnerSha（sha256(JSON.stringify(meta))）：`9b718bafd9c20690440ce31a52c05fa9f84ff194623d4ff0a6279879461ef631`
  meta={task:t219, round:R615a, verdict:P1, trueParent:a9b7b1000c00bc7ad207c5edf2a7a599b148b412, test:3 passed / 0 failed, fixture:r615a_jpush_device_delete_idor_audit.rs}
- lib.rs 文件 sha256：`330496c6eb1b519a521b49433f84c79fabd8706fff9c8ee752034cd0dea42d1c`
- routes.rs 文件 sha256：`397a625ebaf326ec4f4f82d55c9f9b2d1d4c96844dca9c622352df4b72e030f7`
- 真实父 HEAD：`a9b7b1000c00bc7ad207c5edf2a7a599b148b412`（git 核实一致）
