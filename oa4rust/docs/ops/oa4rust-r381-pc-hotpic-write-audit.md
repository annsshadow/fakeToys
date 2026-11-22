# R381 PC HotpicApp.vue 写操作链失败信封甄别报告

- 轮次：R381；任务：t203；角色：qa-e2e（缺陷审计与浏览器实测工程师）。
- 甄别对象：`frontend/apps/desktop/src/views/HotpicApp.vue`（约 357 行）写操作链在后端 HTTP 200 + 业务失败信封下是否存在无条件误报成功。
- 测试方式：Playwright Chromium headless，视口 1440x900，**非物理真机**（自动化无头浏览器实测，非人工真机操作）。
- 环境：测试服地址按占位符 `<YOUR_TEST_DOMAIN>` / `<YOUR_TEST_HOST>` 记录，本文件不含真实域名、IP、端口、账号或凭据真值。
- 证据目录（已被 .gitignore 忽略）：`test-results/r381-pc-hotpic-write/`。
- 范围约束：只读甄别，未修改任何产品源码 / 契约 / 后端 / SDK；破坏性真写（真建热图、真清密文、真删部门级数据）未授权，不执行。

## 一、结论（三分判定）

**判 1 个 P1 真缺陷。** `hotpicWrite` / `hotpicMore` / `onDelete` 三个写函数在后端返回 HTTP 200 + 业务失败信封时，不读取响应体即无条件弹出成功提示（或就地改动列表），构成与 R366 / R368 / R370 / R373 / R375 / R377 / R379 同型的静默失败误报成功。

- 真缺陷三全满足：① 真 UI 可复现（headless 浏览器真实点击按钮）；② 真请求响应（不拦截写路由，真后端返回 200 + type:error 或 200 + success + deleted:false）；③ 源码定位（`HotpicApp.vue:189`、`:215` 无条件 `toast.success`，`:255-256` 失败仍删本地行）。
- 通过项：非 2xx（403 / 500）能正确 `toast.error`；`confirmMsg` 取消守卫零写；列表失败态 / 空态可区分；全程 0 pageerror；只读探测函数不误报。

### 取证限制

- 破坏性真写（真建热图 / 真删用户热图 / 真清密文）未授权不执行；采用假 ID 探针 + 只注入触发条件、写请求仍走真后端不拦截的决定性对照替代，避免污染真实数据。
- `core/entity/create`（真 INSERT）与 `core/entity/delete` 假 ID 分支未做真库副作用验证：假 ID 的 `core/entity/delete` 实测返回 404（`ownership_error`），属非 2xx 正确路径，前端 catch 能覆盖；此处仅作代表性覆盖，未逐一 UI 触发全部 15+ 按钮。
- 空态注入（列表 200 + 空 data）实测页面为 0 总计、仅表头，未出现文案『暂无热帖数据』（模板 `:50` 存在但该注入路径下 body text 未捕获到），据实记录为空态可区分但不虚报文案命中。

### 产品语义差异（非本轮缺陷）

- 后端 `ActionResult::error` 采用 HTTP 200 + `type:error`（`shared/response.rs:188-200`）是 o2server 兼容既有设计，不是本轮改动对象。
- `user_hotpic_id` 的 DELETE 分支对不存在记录返回 200 + `type:success` + `deleted:false`（`hotpic_assemble_control/src/lib.rs:872-881`）属幂等删除口径；`core/entity/delete` 假 ID 走 `ownership_error` 返回非 2xx（404），与其它 hotpic 端点 200 口径不同，属既有实现差异。
- `user/hotpic/exists/check` 为 COUNT 只读 EXISTS 语义（R358 已甄别），非写端点；`hotpicMore` 的 `existsCheck` / `byApp` / `byId` / `cipherList` / `userList` 多为 GET / PUT 只读探测，仅 `userDelete2` 为真实写。

## 二、源码甄别

模板 `:24-38` 共 15 个真实 `@click` 按钮（含 9 个 `hotpicWrite` 分支与 6 个 `hotpicMore` 分支），`:44` / `:59` 行内删除按钮 `@click.stop=onDelete`，全部真实渲染（DOM 审计 45 按钮命中：建热图 / 改标题 / 存配置 / 建用户热图 / 删用户热图 / 清 BBS 密文 / 清 CMS 密文 / 建实体热图 / 删实体热图 / 删用户热图2 / 存在校验 / 删除 均命中），无 EndpointFold / devTools 门控，非 R372 死代码页。

- `hotpicWrite(op)` `:156-193`：9 分支分别 `await api.post/delete(...)` 后 `:189` 无条件 `toast.success('热图操作已提交')`；`:190-192` catch 仅覆盖非 2xx 抛错。删除类分支（userDelete / cipherBbs / cipherCms / coreDelete）有 `prompt` + `confirmMsg` 守卫 `:171/175/179/186`。
- `hotpicMore(op)` `:195-219`：`userDelete2` 为真实写 `:210-213`（`prompt` 双 ID + `confirmMsg` 守卫后 `DELETE user/hotpic/{id}/{id2}`），`:215` 无条件成功提示；其余为只读探测。
- `onDelete(item)` `:252-260`：`confirmMsg` 守卫后 `:255 DELETE core/entity/delete/{item.id}`，`:256` 无条件就地 `filter` 移除行；业务失败信封（若该端点返回 200 + error）会导致行被误删且无错误提示，状态与提示矛盾。
- 只读探测非写：`doSearch` `:235-250`（失败 `:246 describeListError('热帖加载失败')` 保留旧数据）、`loadHotpicMeta` / `loadHotpicMeta2` / `loadHotpicDeep` / `loadHotpicTwin` 均 `toast.error` 非写。

## 三、后端契约与 SDK

- 路由：`hotpic_assemble_control/src/routes.rs` 注册 `POST /api/hotpic/create/hotpic`、`POST .../user/hotpic/changeTitle`、`DELETE .../user/hotpic/{id}`、`DELETE .../cipher/hotpic/bbs/{id}`、`DELETE .../cipher/hotpic/cms/{id}` 等；`hotpic_core_entity/src/lib.rs:294-295` 注册 `POST /api/hotpic/core/entity/create` 与 `DELETE /api/hotpic/core/entity/delete/{id}`。
- 失败信封：`hotpic_assemble_control/src/lib.rs` 的 `user_hotpic_changeTitle` `:682-710`（id 空 / 属主查不到 / UPDATE 0 行均 200 + type:error）、`delete_hotpic` `:460-484`（0 行 200 + error）、`save_hotpic` `:424-456`（0 行 200 + error）、`user_hotpic_id` DELETE `:872-881`（假 ID 200 + success + deleted:false）；`hotpic_core_entity/src/lib.rs:250-274` 的 `delete_by_id` 0 行走 `ownership_error` 非 2xx。
- SDK：`frontend/packages/sdk/src/api.ts:82-88` 的 `parseJsonEnvelope` 只做 `JSON.parse` 不检查 `type`；`:275-296` 仅在非 2xx 抛错（403 → PermissionError，其它 → httpStatusError），2xx 的 type:error 原样 resolve，前端 await 后即走到无条件成功提示。
- 确认弹窗：`confirmMsg` 为自定义 DOM 遮罩 `#oa4-confirm-overlay`（`frontend/apps/desktop/src/utils/toast.ts:182-271`），按钮文本『取消』/『确认』，非原生 `dialog`。

## 四、真浏览器实测（1440x900 headless 非物理真机）

### 首轮：DOM 审计 + 真后端假 ID 探针（0 pageerror）

- DOM：45 个按钮，全部写按钮真实渲染（见上），`hasCreateBtn` 类探针命中。
- 假 ID 探针（真后端 fetch，不拦截）：`changeTitle` 空 id → 200 + type:error『id is required』；`changeTitle` 假 id → 200 + type:error『hotpic not found』；`DELETE user/hotpic/R381-NOSUCH` → 200 + type:success + deleted:false；`DELETE cipher/hotpic/bbs/...` 与 `.../cms/...` 假 id → 200 + type:error『hotpic not found』；`core/entity/delete` 假 id → 404 type:error『not found』；`exists/check` → 200 + success + data:[]。

### 第二轮：决定性对照（真 UI 点击 + 真后端写响应，不拦截，0 pageerror）

- 只做真实 UI 操作，写请求完全走真后端：① 点『改标题』→ `POST .../changeTitle {title}` 真响应 200 + type:error『id is required』→ UI toast『✓热图操作已提交』；② 点『清 BBS 密文』输入假 ID + 确认 → 真 `DELETE .../bbs/R381-NOSUCH-BBS` = 200 + type:error『hotpic not found』→ toast『✓热图操作已提交』；③ 点『删用户热图』输入假 ID + 确认 → 真 `DELETE .../user/hotpic/R381-NOSUCH-USER` = 200 + success + deleted:false → toast『✓热图操作已提交』。
- 截图：`decisive-changeTitle.png`、`decisive-cipherBbs.png`、`decisive-userDelete.png`（肉眼确认成功 toast）。
- 取消守卫：点『清 CMS 密文』输入假 ID 后在 `#oa4-confirm-overlay` 点『取消』，`writesBefore=3 / writesAfter=3 / newWrites=[]`，零写请求，0 pageerror。

### 第三轮 v2：非 2xx 与失败 / 空态（0 pageerror）

- 403（changeTitle 注入）：toast『✗热图操作失败: 没有权限执行该操作』，正确；500（changeTitle 注入）：toast『✗热图操作失败: HTTP 500: Internal Server Error』，正确。
- 列表 500：页面出现『热帖加载失败』与『重试』，失败态可见；列表 200 + 空 data：0 总计仅表头（空态文案见取证限制），与失败态可区分。
- 截图：`v2-403.png`、`v2-500.png`、`v2-fail-state.png`、`v2-empty-state.png`。

## 五、最小修复方向（对齐 R363 方案 B，本轮不改码）

新增统一写响应成功判定（建议命名 `hotpicWriteSucceeded(resp, flagKey?)`）：**以 type 判定为主**——`type === 'success'`（或 type 缺省且 `success !== false`）为通过前提；若响应带权威布尔（`deleted` / `saved` / `success`）或 `id`，再附加校验 `bool === true` 或 `id` 非空（不对所有端点一刀切强制 bool）；否则判失败 `toast.error(resp?.message || '操作未生效')`。

落地：① `hotpicWrite` 各分支 `const resp = await api.post/delete(...)`，`:189` 改为先判成功再 `toast.success`，失败走 `toast.error`；② `hotpicMore` 的 `userDelete2` `:213` 同理，`:215` 仅成功后提示；③ `onDelete` `:255` 接响应，仅判成功后再 `:256` 就地移除行，失败保留行并 `toast.error`。参考 `frontend/apps/desktop/src/views/MeetingApp.vue:634-640` 的 `meetingWriteSucceeded` 范式。

## 六、覆盖甄别

`docs/ops` 中 hotpic 相关文档（r333-hotpic-slots-fix、r308、r332、r335）均为视觉 / 插槽修复，未涉及写操作链失败信封的 UI 反馈，本轮为首次专项甄别，属有效增量。
