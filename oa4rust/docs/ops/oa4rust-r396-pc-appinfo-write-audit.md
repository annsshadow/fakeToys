# R396 PC 应用管理写操作链失败信封甄别报告

- 轮次：R396（qa-e2e 只读甄别，不改码）
- 甄别对象：PC 端 `AppInfoApp.vue` 应用管理页设权限/建文件/新建/删除应用写链在后端 HTTP 200 业务失败信封下的 UI 反馈一致性
- 实测环境：Playwright Chromium headless，视口 1440x900，针对已部署测试服；**非物理真机**。测试域名/主机均为占位符 `<YOUR_TEST_DOMAIN>` / `<YOUR_TEST_HOST>`，真值不入库。
- git 基线：`1bfd9c3cd17fe1f97e83022b219dd2a28d9597f4`（HEAD 与上游一致，工作树干净）

## 一、任务与范围

甄别应用管理页写链是否存在与 R366/R368/R373/R381/R386/R390/R392/R394 同型的缺陷：后端返回 HTTP 200 + type:error 业务失败时，前端 await 后无条件弹成功 toast，误导用户。判真缺陷需三全：真 UI 可复现 + 真请求响应 + 源码定位。破坏性真状态翻转写（真建文件、真写权限）未授权不执行，用假 ID + 写响应不拦截走真后端替代。

覆盖甄别：docs/ops 命中 appinfo 相关历史文档仅 `oa4rust-r333-appinfo-slots-fix.md`（插槽视觉修复），非失败信封 UI 反馈专项；同批次 QueryViewApp(R390)/QueryManagerDeep(R392)/JPushApp(R394) 口径可参照但不可套用结论。本轮首次专测该页失败信封 UI 反馈，有效增量。

## 二、源码甄别（真实入口 / 只读探测 / 门控三分）

文件相对路径 `frontend/apps/desktop/src/views/AppInfoApp.vue`；路由 `main.ts:152-153` path=appinfo/name=AppInfoApp，`main.ts:478` 重定向 `/app/admin/appinfo`，页面标题「应用管理」。无 EndpointFold / devToolsEnabled 门控（grep 命中 0，与 R372/R388 门控型不同），写按钮在测试服生产构建真渲染：DOM 审计 18 个按钮，含新建应用=1 / 设权限=1 / 建文件=1 / 删除=1，fold=0。

**裸成功提示且有真实 @click 入口的写路径（await 后无条件 toast.success，不读 type / 权威 bool；catch 仅覆盖非 2xx）**：
- writePerms（:222-235）：模板 :30 `@click.stop="writePerms(item)"`；prompt 取查看者列表 → confirmMsg 守卫 :228 → `await api.post(`/api/appinfo/${id}/permission`, { personIds })` :230 → :231 无条件 toast.success「应用权限已写入」；catch :232-233 仅非 2xx。
- createFile（:212-221）：模板 :31 `@click.stop="createFile(item)"`；prompt 取文件名 → `await api.post("/api/file", { appId, name })` :216 → :217 无条件 toast.success「文件已创建」；catch :218-219 仅非 2xx。后端 file_u2_create 全路径无 200+error 分支（见第四节），误报前提不成立。

**静默刷新型 CRUD（无成功 toast，不构成误报）**：createApp（:180-191，prompt 取 alias/appType → POST /api/appinfo → doSearch 静默刷新）、deleteApp（:192-199，confirmMsg → DELETE → doSearch 静默刷新），模板 :19 / :33 真 @click。

**只读探测（非写，不判缺陷）**：loadIcon（:204-211，POST icon/size 读图标，toast 读结果）、showPerms（:236-249，GET permission/appInfo/{id}/{managers|publishers|viewers}）、loadAppViews/loadAppPublish/loadAppExtra/loadAppExtra2（模板 :20-23）、doSearch（:97-109，GET appinfo/list/all，失败 :108 describeListError 显式报错保留旧列表）。

## 三、后端与 SDK 信封（假 ID 探针实测坐实）

- 写端点注册：`cms_assemble_control/src/routes.rs:576` `.route("/api/appinfo/{id}/permission", post(permission_u2_app_info))`；:548 `.route("/api/file", post(file_u2_create))`。队长静态侦察曾称 POST permission 未注册，实测反驳：已注册且可达。
- permission_u2_app_info（`cms_assemble_control/src/lib.rs:10407-10426`）：u2_require_admin 门禁 → 查 x_cms_appinfo 存在性 → 假 ID/查不到时 :10422-10424 `return Ok(Json(ActionResult::error("application not found")))` = **HTTP 200 + type:error**；存在则 u2_write_permissions（:10349-10403，roleType/personIds 校验失败走 AppError 非 2xx，成功 200 success + granted 计数）。
- file_u2_create（同文件 :9375-9404）：require_session → 生成 uuid **无条件 INSERT** x_cms_file → 200 success + data.id/name；无「查不到 appId」分支、无 200+error 失败路径（app_id 无外键门禁）。
- 共享信封 `shared/response.rs`：ActionResult::error 落 type=error 且 HTTP 200，属 o2server 兼容既有设计。`frontend/packages/sdk/src/api.ts:82-88` parseJsonEnvelope 不查 type，:275-296 仅非 2xx 抛错，2xx+type:error 原样 resolve。
- confirmMsg 是 `#oa4-confirm-overlay` 自定义 DOM 弹窗（utils/toast.ts:182 导出），按钮「取消」/「确认」，非原生 dialog。

## 四、真浏览器实测（1440x900 headless，非物理真机）

证据目录（gitignored）：`test-results/r396-pc-appinfo-write/`，含 `app-dom-audit.png` / `app-probe-done.png` / `app-decisive-setperm.png` / `app-stub-setperm.png` / `app-cancel-guard.png` / `app-http500.png` / `app-http403.png` / `app-states-fail.png` / `app-states-empty.png` / `evidence.json` / `states-evidence.json`。脚本 `test-results/r396-pc-appinfo-write.cjs` 与 `r396-pc-appinfo-states.cjs` 同目录。

### 4.1 真后端假 ID 探针（fetch 直打，写响应不注入）

- permFakeId（假 ID + personIds=[R396-VIEWER]）：**200 + type:error「application not found」**。
- permEmptyIds（假 ID + 空 personIds）：**200 + type:error「application not found」**（存在性校验在前，先于 roleType/personIds 校验）。
- fileFakeAppId（假 appId + 文件名）：**200 + type:success + data.id**（真 INSERT 1 条空名文件，见取证限制）。

### 4.2 决定性对照（真 UI 点击 + 写响应完全不拦截走真后端，排除注入伪造质疑）

只注入应用列表 1 行（R396-NOSUCH-APP / R396测试应用），设权限写 route 不拦截：
- 真 UI 点「设权限」→ prompt 填 R396-VIEWER → confirm 确认 → 真 POST `/api/appinfo/R396-NOSUCH-APP/permission` = **200 + type:error「application not found」**（newWrites 捕获真响应体）→ 仍弹绿 toast「✓应用权限已写入」。writesDelta=1。截图 `app-decisive-setperm.png` 目视确认绿 toast 与 200+error 并存。**三全满足**。
- 人造 200+type:error stub 对照：同样弹「✓应用权限已写入」（`app-stub-setperm.png`），但该形状由自己注入，仅用于证明前端不读 type，不单独判缺陷；判缺陷依据为真后端响应。
- confirm 取消守卫：点「设权限」→ prompt 填值 → confirm 点取消 → toasts=[]、writesDelta=0、newWrites=[]，零写（`app-cancel-guard.png`）。

### 4.3 非 2xx / 失败态 / 空态 / 运行时

- 注入 500 → 红 toast「✗写入权限失败: HTTP 500: Internal Server Error」（`app-http500.png` 目视确认）；注入 403 → 红 toast「✗写入权限失败: 没有权限执行该操作」（`app-http403.png`）。catch 分支正确。
- 列表 500 → 「⚠️应用列表加载失败: HTTP 500: Internal Server Error」+「重试」按钮 1（`app-states-fail.png` 目视确认）；列表 200 空 data → 仅表头无错误提示（`app-states-empty.png`），失败与空态可区分。
- 全程 0 pageerror；totalWrites=7（含探针与对照）。

## 五、三分判定（不改码）

**判 1 个 P1 真缺陷（三全满足）**：writePerms 在后端 200 + type:error「application not found」下无条件弹「✓应用权限已写入」。
- 真 UI 可复现：应用管理页列表行「设权限」按钮真点击（DOM 真渲染，无门控）。
- 真请求响应：真 POST `/api/appinfo/{假ID}/permission` = 200 + type:error，写响应完全不拦截走真后端。
- 源码定位：`AppInfoApp.vue:230` await 不接响应、:231 无条件 toast.success、:232 catch 仅非 2xx；后端 `lib.rs:10422-10424` 明确 200+error。
- 最小修复方向（不在本轮实施）：对齐 R363/R395 方案 B——新增 `appinfoWriteSucceeded(resp: any)` 以 type 为主判定（不强求权威 bool），writePerms 接响应后再判，失败 toast.error 透传后端 message，不弹成功。

**不判缺陷项**：
- createFile：后端 POST /api/file 无 200+error 失败分支可达（假 appId 仍 success），误报前提不成立；队长静态侦察「后端全非 2xx」判断对本端点不成立（实测 200 success），但同样不构成误报。
- createApp / deleteApp：无成功 toast，走 doSearch 静默刷新，属「静默 CRUD」口径。
- loadIcon / showPerms / loadAppViews 等：只读探测或读结果提示，非写。

**取证限制（据实记录，fail-loud）**：
- 破坏性真写（对真实应用真写权限、真建文件）未授权不执行，用假 ID + 不拦截对照替代；假 ID 权限写在存在性校验处短路，零实际权限变更。
- fileFakeAppId 探针因后端无存在性门禁，真情 INSERT 了 1 条空名测试文件（id 见 evidence.json，测试服数据，非生产），据实记录未隐瞒。
- 空态文案为「仅表头、0 总计」，未出现「暂无应用」字样，据实记录不虚报文案。
- 未覆盖全部只读 load* 探测端点，按代表性覆盖。

**产品语义差异（非缺陷）**：ActionResult::error 落 HTTP 200 是 o2server 兼容既有设计；permission 假 ID 返回 200 error application not found；file 假 appId 仍 success 因 app_id 无外键门禁。

## 六、结论

本轮实测坐实 1 个 P1 用户可达真缺陷（设权限失败信封误报成功），三全满足；createFile/CRUD/只读项经甄别不构成同类误报。证据与截图均为 1440x900 headless 已部署测试服真浏览器实测，非物理真机；所有真值（域名/主机）以占位符代替，未入库。