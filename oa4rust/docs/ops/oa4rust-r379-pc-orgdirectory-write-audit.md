# R379 PC 通讯录 OrgDirectoryApp.vue 写操作链失败信封甄别

- 轮次：R379，任务 t200
- 执行角色：qa-e2e（缺陷审计与浏览器实测工程师）
- 浏览器：Playwright headless Chromium，视口 1440x900，**非物理真机**
- 性质：只读甄别，未修改任何源码/契约/后端/SDK
- 测试环境：页面地址以 `<YOUR_TEST_DOMAIN>` / `<YOUR_TEST_HOST>` 占位，认证凭据从部署环境注入，本文不含真值

## 一、结论摘要

**判定：1 个 P1 真缺陷。** 通讯录页面唯一真实 UI 可达写函数 `createUnitFromForm` 在调用新建部门接口后不读取响应信封，只要 HTTP 2xx 就无条件弹出「部门「X」已创建」成功提示；而后端在重名等场景返回 **HTTP 200 + type:error**（业务失败），此时 UI 仍误报成功。三全满足：真 UI 可复现、真请求响应、源码定位。

## 二、源码定位

- 组件：`frontend/apps/desktop/src/views/OrgDirectoryApp.vue`（约 649 行）
- 路由：`frontend/apps/desktop/src/main.ts:56-61`，`path=directory`，`name=OrgDirectoryApp`（注意 `path=org` 是 OrgViewer.vue 另一组件，勿混淆）
- 唯一真实写函数 `createUnitFromForm`：
  - `:253` 空 name 校验：`unitCreateError = 请输入部门名称` 并 return（零写请求）
  - `:257` `await withTimeout(api.post('/api/organization/assemble/control/unit', {name, parentId: newDeptParent.value.trim()}), DEPT_TREE_TIMEOUT_MS)`，**未接返回值**
  - `:264-268` 清空表单、关闭弹层、`toast.success('部门「X」已创建')`、`void loadDeptTree()` —— 不检查响应 type/bool
  - `:269-272` catch 只覆盖非 2xx（SDK 抛错路径），对 200+type:error 无感
- 真实 UI 入口（模板 `:72-76`）：`org-create-unit-btn` 展开表单，`org-create-unit-name` 名称，`org-create-unit-parent` 父级，`org-create-unit-submit` 提交（`@submit.prevent=createUnitFromForm`）。无 EndpointFold/devTools 门控。

### 死代码群（非本轮缺陷）

以下函数全文件仅定义处、模板无 `@click`、无 `defineExpose`/window 挂载，用户不可触发，**不判缺陷**（对齐 R364/R365/R372 口径）：
- `manage`(`:504-572`) 六个 put/post 写 + 裸 toast.success
- `orgAuth`(`:579-633`) 登录/登出/改密/双因子等 + confirmMsg + 裸 toast.success
- `runPerson`(`:311`)/`runUnit`(`:371`)/`runIdentity`(`:409`)/`runGroup`(`:447`)/`runRole`(`:471`) 及 `RUNNERS`(`:489-497`) 均为只读探测

## 三、后端与 SDK 契约（真浏览器实测 + 源码）

- `backend/crates/organization_assemble_control/src/u2_org.rs:18-69` `unit_create`：`require_admin`；name 空 → Err(BadRequest) HTTP400；parentId 非空且找不到 → HTTP400；`:40-42` `normalized_name_dup` 命中 → `err('unit already exists')`；`backend/crates/organization_assemble_control/src/u2_helpers.rs:148-150` 的 `err` 返回 `Ok(Json(ActionResult::error))` 即 **HTTP 200 + type:error**
- `u2_helpers.rs:236-251` `normalized_name_dup` SQL：`LOWER(TRIM(name))=LOWER($1) AND parent_id=NULLIF($2,'') AND deleted_at IS NULL`。根部门 parentId='' → NULLIF('','')=NULL，`=NULL` 恒 false，故**根级同名不拦截**；子部门（parentId 非空）同名才命中
- `backend/crates/shared/src/response.rs:188-200` `ActionResult::error`：data=None / type=error / size=-1 / prompt=ExceptionEntityNotExist，HTTP 200
- `frontend/packages/sdk/src/api.ts:82-88` `parseJsonEnvelope` 仅 JSON.parse 不检查 type；`:275-296` 仅非 2xx 抛错

## 四、真实 E2E 证据（1440x900 headless 非物理真机，0 pageerror）

### 4.1 假 ID / 边界探针（probe1）
- DOM 33 按钮，`org-create-unit-btn` 存在；空 name POST → 400 type:error "name is required"；假 parentId → 400 type:error "parent unit not found"；PUT 假 flag → 200 type:error "unit not found"（无 UI 入口）

### 4.2 失败信封构造（probe2/probe3）
- 后端重名检测的父级作用域特性导致 probe2 探测时意外创建了 2 条测试部门行（详见取证限制）；利用其产生的子部门关系，`POST unit {name: 已有子部门名, parentId: 该子部门父ID}` → **HTTP 200 + type:error "unit already exists"**，不新增行（dup 命中提前 return）

### 4.3 决定性对照（不拦截写响应，走真后端）
- 真 UI 点击「新建部门」→ 填入上述 (name,parentId) 组合 → 点「提交创建」
- 真 POST `/api/organization/assemble/control/unit` → HTTP 200
- UI toast：**「✓部门「…」已创建」**（误报成功）
- 截图：`test-results/r379-pc-orgdirectory-write/decisive-toast.png`（gitignored）

### 4.4 非 2xx / 守卫 / 失败空态（v2）
- 空 name 提交：写请求数 0，错误文案「请输入部门名称」
- HTTP 500 注入：toast「✗新建部门失败：HTTP 500: Internal Server Error…」正确
- HTTP 403 注入：toast「✗新建部门失败：没有权限执行该操作…」正确
- 部门树接口 500：`dept-tree-error` 显示「部门树加载失败：HTTP 500…重试加载」+ 重试按钮 1
- 部门树空数据：`dept-tree-empty`「暂无部门树数据，请点击「加载部门树」」——失败态与空态可区分
- 全程 0 pageerror

## 五、三分判定

### 真缺陷（1 个，P1）
`createUnitFromForm` 在后端 200+type:error 下无条件 toast.success 误报成功，三全满足。**最小修复方向（对齐 R363 方案 B）**：`api.post` 接住响应，新增写响应成功判定 `writeSucceeded(resp)`——`type==='success'`（或 type 缺省且 success!==false）且存在 `data.id` 才算成功，否则 `toast.error(resp?.message || '部门未创建')` 并保留表单不关闭，不刷新列表。

### 取证限制
- 破坏性真建部门未授权执行；失败信封通过真重名路径（200+error）取得，未用真建部门翻转状态。
- probe2 探测意外产生 2 条测试部门行（根级同名副本），已在测试服留存，属本轮未授权副作用，如实记录；未在任何源码/文档中写入真实部门 ID 之外的值。
- 决策性对照使用真实存在的子部门关系构造重名，但子部门本身来自上述副作用，未做新增写。

### 产品语义差异
- `ActionResult::error` 用 HTTP 200 + type:error 是 o2server 兼容既有设计，非本轮改动。
- 根级同名不拦截（NULLIF('','')=NULL 恒 false）是既有后端行为，属另一潜在议题，本轮不改码、仅记录。
- 空 name 走 400、假 parentId 走 400，与重名走 200 是不同校验层级，据实区分。

## 六、覆盖甄别与通过项

- 通过：空 name 零写、403/500 正确 toast.error、失败态/空态可区分、0 pageerror、死代码群不判缺陷。
- 覆盖甄别：docs/ops 既往记录无 OrgDirectoryApp 写链失败信封专项，本轮首次专测。
