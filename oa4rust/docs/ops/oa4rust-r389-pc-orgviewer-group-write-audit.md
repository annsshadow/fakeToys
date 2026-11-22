# R389 PC OrgViewer 组织组创建写链失败信封甄别

- 轮次：R389；执行：qa-e2e；性质：只读甄别（不改码、不注入源码、不改后端）。
- 目标：甄别 PC `OrgViewer.vue` 组织组创建写链 `createGroupFromForm` 在后端失败信封（HTTP 200 + `type:error` 或 200 + success+权威 bool:false）下是否误报成功。
- 真机口径：**Playwright Chromium headless，视口 1440x900，针对已部署测试服；非物理真机**。测试服地址、凭据均以 `<YOUR_TEST_DOMAIN>` / `<YOUR_TEST_HOST>` 占位，真值不入库。

## 一、结论：0 个用户可达真缺陷

真后端 group 创建端点在本页前端调用路径下（仅提交 `{name}`），**不会产生 HTTP 200 + `type:error` 的失败信封**：空名/无效 unitId 均返回 400 非 2xx（SDK 抛错→前端 catch→`toast.error` 正确）；重名顶层组反而返回 200 + `type:success` 并真实创建（属后端重名检测缺陷，见「非阻断观察」，与前端 toast 一致性无关）。误报成功的**前提（后端失败信封）在本链路不成立**，故判 0 真缺陷（对齐 R385 Calendar、R388 CmsIndex 口径）。

注入的 200 + `type:error` 人造信封确会误弹成功（L1 潜在隐患，仅记录，不判本轮缺陷）——因为真后端前端路径不产生该形状。

## 二、源码甄别（真实入口，非门控）

- 文件：`frontend/apps/desktop/src/views/OrgViewer.vue`（约 1600 行）；路由 `frontend/apps/desktop/src/main.ts:50-55` `path=org` / `name=OrgViewer`。
- 真实入口：模板 `:127-134` `<div class=org-create-form data-testid=org-create-group-form>`——`:128` 按钮 `org-create-group-btn`（切换 `showGroupCreate`）、`:129` `<form @submit.prevent=createGroupFromForm>`、`:130` 输入 `org-create-group-name`（v-model `newGroupName`）、`:131` 提交 `org-create-group-submit`、`:133` 错误位 `org-create-group-error`。该表单位于 EndpointFold（模板 `:8-118`）**之外**，非门控隐藏。E2E DOM 审计：`createBtn=1 / form=1 / fold=0`，入口真实可达。
- 写函数 `createGroupFromForm`（`:1453-1476`）：`:1454` trim 取 name，`:1455` 空名校验（写 `groupCreateError` 并 return，零写）；`:1459-1465` `await withTimeout(api.post('/api/organization/assemble/control/group', { name }), ...)` **未接响应、未判 type/bool**；`:1466-1469` 清表单/关弹窗/`toast.success(组织组「…」已创建)`/`void loadOrgTree()`；`:1470-1472` catch 仅非 2xx → `groupCreateError` + `toast.error`。
- 对照：`loadOrgTree`（`:1430-1447`，`api.put(group/list/like, {})` 只读）失败写 `treeError`（`组织架构加载失败：…`）+ 重试按钮（模板 `:136-139` `org-tree-retry`）。
- EndpointFold 内 `orgCreate('group')` / `orgUnitWrite('groupCreate')` / `orgEntity('groupCreate')` 等高级写按钮（模板 `:57-109`）属门控区，生产构建 `devToolsEnabled=import.meta.env.DEV=false` 隐藏（同 R372/R388），本轮不作为可达入口判定。

## 三、后端契约（真浏览器实测，不臆造）

- 路由：`backend/crates/organization_assemble_control/src/u2_router.rs:115` `POST /api/organization/assemble/control/group → u2_org::group_create`。
- Handler：`backend/crates/organization_assemble_control/src/u2_org.rs:631-674`：
  - `:636` `require_admin`；`:639-641` 空名 → `AppError::BadRequest("name is required")`（**HTTP 400 非 2xx**）；
  - `:642-649` 无效 unitId → `AppError::BadRequest("unit not found")`（**400**）；
  - `:650-652` 重名 → `err("group already exists")`（`u2_helpers.rs:148-150` `ActionResult::error` = HTTP 200 + `type:error`）——**但因 `normalized_name_dup`（`u2_helpers.rs:236-251`）中 `scope_col = NULLIF($2,'')` 对 `unit_id` 为 NULL 的顶层组比较失效，此分支对顶层组不可达**（见非阻断观察）；
  - `:658-673` 成功 → `ok({id, name})` = HTTP 200 + `type:success` + data.id。
- 共享信封：`backend/crates/shared/src/response.rs:188-200` `ActionResult::error` 设 `type=error/size=-1/prompt=ExceptionEntityNotExist` 但仍是 HTTP 200。
- SDK：`frontend/packages/sdk/src/api.ts:82-88` `parseJsonEnvelope` 仅 `JSON.parse` 不查 type；`:275-296` 仅 `!resp.ok` 抛错（400/403/404 → ApiError/PermissionError 进 catch），200 + `type:error` 原样 resolve。

### 真后端探针（E2E `probe`/`dupProbe`，1440x900 headless）

| 探针 | 真实响应 |
|---|---|
| groupCreateEmpty（name=""） | HTTP 400，`type:error`，`bad request: name is required` |
| groupCreateFakeUnit（name+不存在 unitId） | HTTP 400，`type:error`，`bad request: unit not found` |
| 重名顶层组（已存在组名，仅传 name） | HTTP 200，`type:success`，`data.id` 为新 UUID（**真实创建了重复组，未走 error 分支**） |

## 四、决定性对照（写响应完全走真后端，不拦截）

E2E 首轮：DOM 审计后，真 UI 点 `org-create-group-btn`→填已存在组名→点 `org-create-group-submit`（**不拦截 group 写路由**）。
- 真请求 `POST /api/organization/assemble/control/group` = **HTTP 200 + `type:success` + 新 data.id**（真后端真实创建重复组）；随后 `PUT group/list/like` 刷新列表出现两个同名组。
- UI：`toasts=["✓组织组「…」已创建"]`，`formClosed=true`，`errors=[]`——toast 与真响应（success）**一致**，无失败信封误报。
- 三全判据：真 UI 点击 ✅ + 真请求响应 ✅ + 源码定位 ✅，但真响应是 **success 而非失败信封**，「失败信封误报成功」不成立 → **0 真缺陷**。
- 截图：`org-decisive-toast.png`（1440x900，绿 toast + 列表两个同名组，肉眼确认）。

### 注入对照（L1 潜在隐患，非本轮缺陷）

E2E v2：注入 `POST group → 200 + {type:error, message:"group already exists"}`（人为形状），真 UI 提交后 `toasts=["✓组织组「R389-注入TypeErr」已创建"]`、`formClosed=true`、`errors=[]`——裸 `await` 不读 type 确会误报。但真后端前端路径不产生该形状，故仅记为 L1 潜在隐患，不判本轮缺陷。截图 `org-v2-injected-typeerror.png`。最小修复方向（若未来后端改 200+error 口径）：新增 `groupWriteSucceeded(resp)` 判 `type==='success'`（或 type 缺省且 success!==false）且 `resp.data?.id` 存在才成功，否则 `toast.error(r?.message || '操作未生效')` 并保留表单（参考 `frontend/apps/desktop/src/views/MeetingApp.vue:634-640 meetingWriteSucceeded`、`OrgDirectoryApp.vue` R380 `writeSucceeded`）。

## 五、通过项与取证限制

通过项（均 0 pageerror）：
- 真实入口可达、非门控（`createBtn=1/form=1/fold=0`）。
- 空 name 零写：`writesDelta=0`，错误 `请输入组织组名称`，无成功 toast。
- 非 2xx 正确：注入 500 → `✗新建组织组失败：HTTP 500: Internal Server Error…`；注入 403 → `✗新建组织组失败：没有权限执行该操作…`（均写 `org-create-group-error`）。
- 失败态/空态可区分：列表 500 → `组织架构加载失败：HTTP 500…` + 重试按钮 1；列表空 data → `暂无组织架构数据，请点击「新建组织组」创建第一组`。

取证限制（fail-loud）：
- 破坏性真状态翻转/真删未授权不执行：用假 ID + 不拦截对照 + 注入信封替代；决定性对照中真后端真实创建了一条重复组（测试数据，未清理后端）。
- 空名/假 unitId 由 HTTP 探针坐实（前端表单仅能提交 name，假 unitId 无 UI 输入路径）；重名由真 UI 提交坐实。
- 真后端前端路径未覆盖所有失败源（如 DB 异常 500 由注入对照替代）。

## 六、三分判定

- **真缺陷：0 个**。失败信封误报成功前提（后端 200+error）在本链路前端路径不成立；空名/假 unitId 为非 2xx 正确错误；重名为 200 success 真创建（toast 与响应一致）。三全不满足。
- **取证限制**：破坏性真写未执行；重名真创建一条重复测试数据；注入 200+error 仅作 L1 潜在隐患对照。
- **产品语义差异 / 非阻断观察**：后端 `ActionResult::error` 用 HTTP 200 是 o2server 兼容既有设计（同批次一致）；`group_create` 空名/无效 unitId 用 400（AppError）而重名用 200+error 是既有 handler 口径差异。**非阻断观察：`normalized_name_dup`（`u2_helpers.rs:244`）中 `scope_col = NULLIF($2, '')` 对 `unit_id` 为 NULL 的顶层组织组，`unit_id = NULL` 恒为 false，导致顶层组重名检测失效、可重复创建同名组——这是后端数据完整性缺陷，不属本任务「写链失败信封误报成功」范围，如实记录供队长评估，本轮不立项、不改码。**

## 七、覆盖甄别

`docs/ops` 命中 8 处（`oa4rust-r301-u1-org-tree.md` 首屏 PUT 修复、`oa4rust-r305-p0a-visual.md` 视觉、`oa4rust-r307-org-create-entry.md` 创建入口新增、`oa4rust-r379-pc-orgdirectory-write-audit.md` 明确 path=org 属 OrgViewer 另一组件、`oa4rust-c301-business-upgrade-r301.md` 等），均为首屏加载/视觉/入口新增/组织通讯录写链，**从未专项甄别 OrgViewer 组织组创建的失败信封 UI 反馈**，本轮首次专测为有效增量。

后备候选（本轮不立项仅记录）：`Personal.vue saveSignature`（`:227-232` 裸 `.then` toast 签名已保存），后端信封未实测。
