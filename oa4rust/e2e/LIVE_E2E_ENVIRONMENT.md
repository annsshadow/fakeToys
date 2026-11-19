# OA4Rust 真机 GUI E2E — 环境与实跑说明

> 本文档只描述「值从哪里读」，不含任何真实地址、域名、IP、账号、口令或本机端口/SSH 别名。
> 所有真实值以占位符表示，由部署环境通过环境变量注入。

## 适用范围

- 用例目录：`oa4rust/e2e/**`（`live-gui-expanded.spec.ts` 为本轮新增的真实 GUI 交互扩充）
- 全局预检：`oa4rust/e2e/global-setup.ts`
- 运行配置：`oa4rust/playwright.config.ts`
- 会话/响应审计辅助：`oa4rust/e2e/support/live.ts`

## 必需环境变量（值由部署环境注入，禁止入库/回显）

| 变量名 | 用途 | 约定 |
| --- | --- | --- |
| `BASE_URL` | 被测站点根 URL | 必须是绝对 URL；占位示例 `<YOUR_TEST_HOST>`，例如 `<YOUR_TEST_HOST>/oa4rust/` |
| `E2E_USERNAME` | 登录用户名 | 由测试环境提供，禁止硬编码 |
| `E2E_PASSWORD` | 登录口令 | 由测试环境提供，禁止硬编码 |
| `E2E_PROCESS_NAME` | 可选：工作流流程名 | 未设置时用例回落到仓库内置种子名 |
| `COLLECT_COVERAGE` | 可选：是否采集覆盖率 | `true` 时启用覆盖率报告项目 |

## global-setup 预检逻辑

1. 从 `config.projects[0].use.baseURL` 读取 `BASE_URL`，为空或非字符串则直接抛错（不静默跳过）。
2. 校验 `E2E_USERNAME` 与 `E2E_PASSWORD` 都存在，任一缺失即抛错。
3. 以 `new URL('/login', baseURL)` 构造目标，`fetch(..., { redirect: 'manual', signal: AbortSignal.timeout(10000) })`。
4. 网络不可达/超时 → 抛 `Live GUI E2E cannot reach ...`；HTTP >= 400 → 抛预检失败。
5. 预检不通过 → 整个 playwright 运行直接失败，不会产生虚假通过。

## 本轮实跑环境（真值均以占位符表示）

- 环境类型：测试服务器经 **SSH 隧道 + 本地 TLS 终止/路径重写代理** 访问。开发机无法直连公网域名，且后端静态资源使用 nginx 前缀 `/oa4rust/`，故用本地代理补齐 TLS Host/SNI、路径前缀与 CSRF Origin 语义。
- 链路（全部占位符）：
  - SSH 隧道：`<SSH_ALIAS>` 免密，把远端监听端口转发到本地回环 `<TUNNEL_PORT>`，带 ServerAliveInterval 自动重连。
  - 本地代理：监听本地回环 `<LOCAL_PROXY_PORT>`，负责 (a) TLS 终止并注入正确 Host/SNI；(b) 路径重写：`/oa4rust-api/*` → 后端 `/api/*`、`/oa4rust/assets/*` → 静态资源、`/login` 等裸路径 → `/oa4rust/*`；(c) 把浏览器 Origin 规范化为后端可信源 `<TRUSTED_ORIGIN>`，恢复隧道破坏的同源语义。
  - `BASE_URL` = `http://127.0.0.1:<LOCAL_PROXY_PORT>`（回环地址 + 占位端口）。
- 凭证：经 captain 授权，在**测试库**中新建专用 E2E 账号（随机口令 + bcrypt hash，绑定 admin 角色），口令只在运行时从本地注入的环境变量读取，全程未回显、未写入仓库/本文档；测试库原有 admin 账号未被修改。
- 部署布局（占位符）：后端服务 `<BACKEND_SERVICE>` 监听远端本地端口；web 产物在 `<WEB_DIST_PATH>`，含桌面与 H5 两套；BUG-D 修复（commit `<COMMIT_HASH>`）已部署到测试服务器，线上 bundle 确认防御逻辑生效。

## 最终全量实跑结果（t12 复验轮，脱敏，真实 GUI 交互）

- 命令：`pnpm exec playwright test` 覆盖 `live-gui-expanded.spec.ts`（15 条）+ `workflow-runtime.spec.ts`（1 条），live-chromium + coverage-chromium 双 project（**16 条唯一用例 × 2 project = 32 条**）。
- 权威全量实跑结果：**32 条中 26 passed / 6 failed**（3 个唯一用例各在 2 个 project 失败）。
  - `live-gui-expanded.spec.ts` 双 project 共 30 条：**26 passed / 4 failed**（#10 / #11 各在 live+coverage 双 project 失败）。
  - `workflow-runtime.spec.ts` 双 project 共 2 条：**0 passed / 2 failed**（#16 在 live+coverage 双 project 失败，BUG-E RED 证据）。
- **BUG-D 修复上线验证通过**：部署 bundle 中 `e.catch is not a function` 错误已消失，`Promise.resolve(...).catch(...)` 防御生效；通讯录「查询本页」不再抛 TypeError。但两用例仍失败且**形态变化**为「表格 0 行 / 分页超时」，根因为测试库组织数据全空（详见 BUG-D 更新与 BUG-F），非 BUG-D 代码回归。
- 逐条（唯一用例 16 条；#1–#15 在 live-chromium + coverage-chromium 双 project 结果一致，#16 双 project 均为 failed）：

| # | 用例 | 结果 |
| --- | --- | --- |
| 1 | 登录页空表单提交按钮禁用 | passed |
| 2 | 密码显隐切换 input type | passed |
| 3 | 错误密码显示行内错误且不进入工作台 | passed |
| 4 | 侧边栏导航切换路由并同步面包屑 | passed |
| 5 | 用户菜单打开并跳转个人中心 | passed |
| 6 | 全局搜索快捷键聚焦搜索框 | passed |
| 7 | 侧边栏折叠与展开切换外壳样式 | passed |
| 8 | 个人签名填写并提交后更新成功 | passed |
| 9 | 系统设置分栏切换暴露安全表单字段并操作下拉 | passed |
| 10 | 通讯录查询本页渲染数据表格与表头 | **failed**（新形态：组织库空数据，表格 0 行；BUG-D TypeError 已消失） |
| 11 | 通讯录人员分页按钮推进页码 | **failed**（新形态：组织库空数据，分页信息「共 0 人」、推进超时） |
| 12 | 两次密码不一致时前端校验报错且不发写请求 | passed |
| 13 | 密码表单填写不完整时校验报错且不发写请求 | passed |
| 14 | 未登录访问受保护路由重定向并保留 redirect | passed |
| 15 | 退出登录后受保护路由再次重定向到登录页 | passed |
| 16 | workflow 发起→填写→提交→审批闭环 | **failed ×2 project**（BUG-E，POST /api/task/{id}/complete 400，产品 bug RED） |

- 失败 trace/截图：`oa4rust/test-results/**/test-failed-1.png` 与同目录 `trace.zip`、`error-context.md`（仓库相对路径，不含真值）。
- BUG-E 关键 trace：`oa4rust/test-results/workflow-runtime-starts-re-aa680-hrough-visible-web-controls-live-chromium/trace.zip`，解压后 `1-trace.network` L71–L76：GET `task/{id}` → 200（task_status=`active`）、GET `task/v2/{id}` → 200、GET `task/expire/{id}` → 200 后同一任务变 `expired`、POST `task/{id}/complete` → 400。

## 关键环境阻塞与修复过程（如实记录）

1. 旧前端产物（BUG-A）：新版部署后消失，登录页语义 id/aria-label 已恢复。
2. 种子账号口令不可用：测试库种子 admin 的 hash 为占位值、其他已知口令的账号未 seed，导致认证失败；经授权新建专用 E2E 账号后解决。
3. CSRF Origin 门禁：后端 cookie 写接口要求 Origin 与 `APP_PUBLIC_ORIGIN` 精确一致；隧道代理改变了浏览器 Origin 语义，导致签名保存/登出等写接口 403 `ExceptionAccessDenied`。已在测试代理层规范化 Origin 后消除（非产品 bug）。
4. 两处测试选择器漂移：密码显隐按钮改为 `.eye-btn`、流程弹窗改为 `#start-process`/`#start-title`（均已修进 spec）。
5. 本轮新发现：测试库组织数据全空（`x_org_person` / `x_org_unit` / `x_org_group` / `x_org_role` / `x_org_login_record` 均 0 行，仅 `x_org_identity` 1 行），导致通讯录「查询本页」所有只读查询返回 0 行、表格不渲染。补种组织 seed 数据不在本轮授权边界（授权仅限 BUG-E 审批任务，且 captain 已裁决 BUG-E 不补种），交 captain 裁决。

## 可复现 bug 列表（交 captain 分流）

- **BUG-A**（部署漂移 / fe-desktop，**已修复**）：部署版登录页密码框无 `id="login-password"`、显隐按钮无 `aria-label="显示或隐藏密码"`；新版部署后复验通过。
- **BUG-B**（后端错误语义 / t9）：登录失败时提示 "Session expired, please login again"（会话过期），非用户名/密码错误提示，属误导性错误语义。
- **BUG-C**（认证凭证 / 环境问题，**已解决**）：种子 admin 的 hash 为占位值不可登录；经授权新建专用 E2E 账号解决，非产品代码 bug。
- **BUG-D**（真实前端 bug / fe-desktop，**已修复上线并验证**）：`OrgDirectoryApp.vue` 的 `collect(label, await api...)` await 位置错误 → `s(p)=p.catch` 对已解析值抛 `TypeError: e.catch is not a function`。t10 修复（53 处移除 await + `s` 放宽为 `Promise.resolve(p).catch()`）已随 commit 部署；本轮实测 TypeError 消失、bundle 确认防御生效。**但本轮通讯录两用例仍失败且形态变化**：测试库组织数据全空，所有只读查询 0 行 → `rows.length=0` → 表格/「命中」不渲染（#10）、分页停留「共 0 人」且推进超时（#11）。此为【测试数据】缺失问题，非 BUG-D 代码回归；交 captain 裁决是否补种组织 seed。
- **BUG-E**（真实产品 bug / e2e-runner-2 实证，非夹具，**本轮保持 RED**）：workflow 发起新流程→打开刚发起任务→审批通过，`POST /api/task/{id}/complete` 返回 **400**。根因：前端 `ProcessWork.vue:1603 loadTaskInfo` 把 GET `/api/processplatform/service/processing/task/expire/{id}` 当只读『超时信息』调用，但该路由 `routes.rs:100` 绑定到写端点 `task_id_expire`（`lib.rs:1363 UPDATE x_task SET task_status='expired'`）；于是打开任务详情就把刚创建的 active 任务置 expired，随后 `task_complete`（`lib.rs:3795`）只接受 active/processing → 400。trace 实证见上方「最终全量实跑结果」。影响面：所有『发起→打开→审批』闭环。判定：产品 bug，**未补种掩盖**；修复方向优先纯前端（移除对写端点的误调，超时信息改只读来源），以免受服务器无 Rust 工具链限制。待 captain 派发修复任务。
- **BUG-F**（产品代码 bug 候选 / 后端，本轮新发现，交 captain 分流）：`POST /api/person/list/login/after/object` 对合法 `date`（如 `2026-01-01`）返回 **500**。后端 `organization_assemble_express/src/endpoints_person2.rs:39 login_after` 把 DB 错误统一吞为 `Err(AppError::Internal)`，掩盖真实原因；psql 直接 replay 同 SQL/参数成功返回空集，故非 SQL 语法/参数问题。前端 `s()` 已能 catch 该 500（不再触发 BUG-D），不影响主流程，但属错误语义问题，建议后端成员复核。

## 实跑命令模板（占位符形式）

```bash
cd <REPO_ROOT>/oa4rust
# 1) 建立到 <SSH_ALIAS> 的隧道 + 启动本地 TLS/重写/Origin 规范化代理，
#    使 BASE_URL 指向 http://127.0.0.1:<LOCAL_PROXY_PORT>
# 2) 运行时注入凭证（真值不入库/不回显）：
export E2E_USERNAME='<YOUR_E2E_USER>'
export E2E_PASSWORD='<YOUR_E2E_PASSWORD>'
export BASE_URL='http://127.0.0.1:<LOCAL_PROXY_PORT>'
# 可选：export E2E_PROCESS_NAME='<YOUR_SEED_PROCESS_NAME>'
pnpm exec playwright test --project=live-chromium e2e/live-gui-expanded.spec.ts
# 全量真机回归：
pnpm exec playwright test e2e/live-gui-expanded.spec.ts e2e/workflow-runtime.spec.ts
```

失败时 `playwright.config.ts` 已配置：`trace: 'retain-on-failure'`、`screenshot: 'only-on-failure'`、`video: 'only-on-failure'`，
产物落在 `oa4rust/test-results/`。

## 本轮已扩充的真实 GUI 用例（共 15 条，均为可见 DOM 交互）

1. 登录页空表单提交按钮禁用 2. 密码显隐切换 3. 错误密码行内报错 4. 侧边栏导航+面包屑 5. 用户菜单跳个人中心 6. 全局搜索快捷键 7. 侧边栏折叠展开 8. 个人签名填写提交 9. 系统设置分栏+下拉 10. 通讯录查询本页表格 11. 通讯录分页 12. 两次密码不一致校验 13. 表单不完整校验 14. 未登录重定向 15. 退出登录后重定向
（workflow-runtime 另 1 条：发起→填写→提交→审批闭环）
