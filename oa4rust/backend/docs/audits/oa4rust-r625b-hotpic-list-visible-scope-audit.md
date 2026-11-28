# R625b hotpic 通用 list 绕过 visible_scope / base64 跨视图泄露甄别

## 结论
可达真缺陷 1 项（P2）；不可达 L1 0；正常 3 项。纯甄别，零源码/配置/路由/前端改动。

- 真父：`371650c14ac4d8cb5179cbeb55513f2c8cb8e379`（HEAD）。
- 凭据与连接串只以 `<TEST_HOST>:<PORT>` 占位；临时库由环境变量注入。

## 立项范围
hotpic 通用列表端点 `GET /api/hotpic/core/entity/list` 是否绕过 `visible_scope` 可见性过滤、致使认证用户读取其他视图热图（含 base64 图像数据）。兄弟写端点的归属门作为对照，不在本次判定内。

## 机制锚点
- `oa4rust/backend/crates/hotpic_core_entity/src/lib.rs:26-30` `require_session`；`:32-43` `HOTPIC_OWNER_PREDICATE`（全局内容仅管理员写、具体键本人或管理员写）。
- `:60-68` `HotPic { id, application, info_id, title, base64 }`；`:72-109` `list(db: Extension<DatabaseConnection>)` 仅接 db、无 session，裸 `Entity::find().order_by_desc(CreateTime).limit(20).all()`，无 `.filter(visible_scope(...))`；`:82-99` 逐条回传 `id/application/infoId/title/base64`（`m.base64.clone().unwrap_or_default()`）。
- `:116-128` `visible_scope(application, info_id)` = 精确键 OR 全局空键；`:132-172` `list_by_app_and_info` 与 `:176-195` `exists_check` 均带 `.filter(visible_scope(...))`（正面兄弟）。
- `:283-297` 路由 `list/list/by/exists/create/delete`；`src/routes.rs:6-8` 仅 `router()`；`oa4rust/backend/src/lib.rs:193` `.merge(hotpic_core_entity::router(...))` 真实挂载。
- 前端 `oa4rust/frontend/apps/desktop/src/views/HotpicApp.vue:248 doSearch():252` `api.get('/api/hotpic/core/entity/list')` 真实消费该端点。
- 兄弟安全测试 `security_gate_tests_b72.rs:30-57` 写端点无会话 401；`security_gate_tests_idor_b4.rs:125-153` 双向 403/本人 200、`:155-178` 全局仅 admin——写端点有归属门而通用 list 无会话、无 scope。

## 决定性实测（独占临时 PG + oneshot 直调）
独立容器端口以 `<PORT>` 占位；`DATABASE_URL` 环境变量注入。夹具 fresh 库迁移全量后 seed 四行：`r625b-a@A/X`、`r625b-b@B/Y`、`r625b-g@全局空键`、迁移 053 自带 `test-id@test/test-id`。`cargo test -p hotpic_core_entity --test r625b_hotpic_visible_scope_audit -- --nocapture` exit=0，2 passed / 0 failed。

| 臂 | 结果 | 判定 |
|---|---|---|
| 通用 list（handler 直调，仅 db） | `R625B_LIST_IDS=["test-id","r625b-g","r625b-b","r625b-a"]` success，含 bob 视图 `r625b-b` 的完整 base64 | 跨视图泄露坐实 |
| 带键 `list_by_app_and_info(A/X)` | `R625B_SCOPED_IDS=["r625b-g","r625b-a"]`，正确排除 bob | 正常（兄弟 scope 正确） |
| `exists_check(B/Y)` | allExists=true（路径自身语义正确） | 正常 |
| app 层裸会话 `GET list` | `R625B_UNAUTH_RESULT=401` | 正常（认证中间件覆盖） |
| 汇总输出 | `leak=1 scoped_isolate=1 exists=1` | 缺陷与对照同时成立 |

## 三分裁决
- **可达真缺陷 1 项（P2）**：`list` 端点无 session 参数且无 `visible_scope` 过滤，任何已认证用户经 `GET /api/hotpic/core/entity/list` 可读回其他视图热图（含 base64 图像数据），前端 `doSearch` 真实消费。判 P2：须登录、热图非高敏 PII、base64 为图片而非凭据；不夸大 P1。修复方向：`list` 接 session 并按 `visible_scope` 过滤，或让前端仅用带键 `list_by_app_and_info`。
- **不可达 L1 0 项**：无死代码式伪线索。
- **正常 3 项**：`:132-172` 带键列表与 `:176-195` `exists_check` 的 scope 过滤正确；裸 401 由 app 认证中间件提供；迁移 053 自带 `test-id` 行按设计被通用 list 全量回传（非缺陷，已据实计入数量断言）。

## Fail Loud
夹具写完即从源码树删除，`git status` 无本任务残留；证据全落 gitignored `oa4rust/test-results/r625b-hotpic/`（`.gitignore:28`）。门控矩阵：认证由 app middleware 提供（handler 裸签名无 session）；写端点有归属门、通用 list 无 scope。裸 401/双向对照均实测，未跳过任何臂。

## SHA 清单（runnerSha 与文件字节 sha 分列）
- runnerSha（`sha256(JSON.stringify(meta))`）：`e226b3ce83813eef7a1a607cd0649b1485e25dbf66a679f7d34b5387b9b75f07`
- 夹具字节 sha（源码树与证据副本一致）：`e8a70fe5418ac1cf2c1e613b9871af04c8b2ea1fcb22f9a6ab98f3a17057b9ae`
- oneshot.log sha：`700e4a7073b5f696606d6a2a678cb824dc1fbd6943adf02f74103d5f446830fd`
- meta.json sha：`2f7e581eb6151de296d258b3eb901587088eb7718dbac59600c076fabfaa83ae`
- lib.rs 源码指纹（未改，参考）：`6dd62634d169ed055eba9dd72b8f9b02246fa75bab7793f5b6d196835570366b`
- 本文档 sha：写入后采集（见提交记录）。
