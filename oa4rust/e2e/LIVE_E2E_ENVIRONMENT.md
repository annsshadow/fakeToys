# OA4Rust 真机 GUI E2E — 环境与实跑说明

> 本文档只描述「值从哪里读」，不含任何真实地址、域名、IP、账号、口令或本机端口/SSH 别名。
> 所有真实值以占位符表示，由部署环境通过环境变量注入。

## 适用范围

- 用例目录：`oa4rust/e2e/**`（`live-gui-expanded.spec.ts` 为真实 GUI 交互扩充）
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
- 部署布局（占位符）：后端服务 `<BACKEND_SERVICE>` 监听远端本地端口；web 产物在 `<WEB_DIST_PATH>`，含桌面与 H5 两套；BUG-D 修复（commit `<COMMIT_HASH_BUG_D>`）与 BUG-E 修复（commit `<COMMIT_HASH_BUG_E>`）均已部署。线上 BUG-E 修复 bundle（`<PROCESSWORK_CHUNK>`）只读核对：`task/expire` 出现 0 次、`end_time` 出现 1 次，确认前端不再调用会改写任务状态的 expire 端点。

## 最终全量实跑结果（t16 补种后最终轮，脱敏，真实 GUI 交互）

- 命令：`pnpm exec playwright test e2e/live-gui-expanded.spec.ts e2e/workflow-runtime.spec.ts --workers=1`，live-chromium + coverage-chromium 双 project（**16 条唯一用例 × 2 project = 32 条**）。
- 权威全量实跑结果（line reporter 逐条 + 结束汇总）：**32 条全部 passed / 0 failed / 0 flaky / 0 skipped**（多次串行复跑一致：`32 passed (27.7s)` / `32 passed (28.1s)`）。
  - `live-gui-expanded.spec.ts` 双 project 共 30 条：**30 passed / 0 failed**（#10 / #11 补种组织 seed 后由 RED 转 GREEN）。
  - `workflow-runtime.spec.ts` 双 project 共 2 条：**2 passed / 0 failed**（#16 双 project 均 `passed`；BUG-E 修复持续 GREEN）。
- **BUG-E 修复上线验证通过**：`POST /api/task/{id}/complete` 不再 400，workflow 闭环用例双 project 全部通过；线上 bundle 不再包含 `GET /task/expire/{id}` 写端点调用（改为只读来源 `end_time`），发起→打开→审批闭环恢复。
- **BUG-D 修复持续有效**：通讯录查询不再出现 `TypeError: e.catch is not a function`。
- **#10/#11 由测试数据阻塞转为 GREEN**：补种测试库组织 seed 后，通讯录表格渲染出行 + 表头 +「命中 N 行」，人员分页可推进到第 2 页。
- 逐条（唯一用例 16 条；#1–#16 在 live-chromium + coverage-chromium 双 project 结果均一致，全部 passed）：

| # | 用例 | 结果 |
| --- | --- | --- |
| 1 | 登录页空表单提交按钮禁用 | passed ×2 |
| 2 | 密码显隐切换 input type | passed ×2 |
| 3 | 错误密码显示行内错误且不进入工作台 | passed ×2 |
| 4 | 侧边栏导航切换路由并同步面包屑 | passed ×2 |
| 5 | 用户菜单打开并跳转个人中心 | passed ×2 |
| 6 | 全局搜索快捷键聚焦搜索框 | passed ×2 |
| 7 | 侧边栏折叠与展开切换外壳样式 | passed ×2 |
| 8 | 个人签名填写并提交后更新成功 | passed ×2 |
| 9 | 系统设置分栏切换暴露安全表单字段并操作下拉 | passed ×2 |
| 10 | 通讯录查询本页渲染数据表格与表头 | **passed ×2**（补种组织 seed 后转 GREEN） |
| 11 | 通讯录人员分页按钮推进页码 | **passed ×2**（补种 21 条人员后分页可推进） |
| 12 | 两次密码不一致时前端校验报错且不发写请求 | passed ×2 |
| 13 | 密码表单填写不完整时校验报错且不发写请求 | passed ×2 |
| 14 | 未登录访问受保护路由重定向并保留 redirect | passed ×2 |
| 15 | 退出登录后受保护路由再次重定向到登录页 | passed ×2 |
| 16 | workflow 发起→填写→提交→审批闭环 | **passed ×2**（BUG-E 修复 GREEN） |

- 失败/覆盖率 trace 与截图：`oa4rust/test-results/**`（本轮最终串行全量无失败，故无 test-failed 产物；history 中保留前轮回退样本）。
- 并发说明（环境配置，非产品 bug）：默认 4 workers 用同一 E2E 账号并发登录时，会话互斥导致偶发 `toHaveURL(/\/app\/)` 失败（停在 login 页，失败用例每次不同）；单 spec 2 workers 与全量 --workers=1 均稳定 32/32。建议真机全量使用 `--workers=1` 串行执行。

## 测试库组织 seed 补种（t16，一次性授权）

- 授权边界：captain 授权仅操作**测试库组织相关表**（RUST_ENV=test、专用测试库、active admin 角色存在），不碰 admin 账号、不碰 laiyipao、不碰生产；真值不回显不入库。
- 补种内容（幂等、明显测试标记、无真值）：`x_org_person` 21 行（id=`e2e-org-seed-*`、name=E2E 测试标记、deleted_at=NULL）+ `x_org_login_record` 21 行关联。
- 数据量依据：#10 只需 `all/object` 等任一查询返回非空数组即渲染表格；#11 每页 20 条、需 count ≥ 21 才能推进到第 2 页 → 补 21 条。
- 归属确认：filter 的 count 带 owner_scope（creator=本人或 admin）；E2E 账号绑定 admin 角色，故补种 creator 用测试标记即可被统计。
- 归属/安全确认：补种前只读核对 RUST_ENV=test、db_name 与表归属、active admin 角色、组织表基线全空；补种前后各做一次计数与 SQL replay 对照（见 BUG-F）。

## BUG-F 补种前后只读对照（真值不回显）

| 探针 | 补种前 | 补种后 |
| --- | --- | --- |
| 组织表计数（person / login_record / identity） | 0 / 0 / 1 | 21 / 21 / 1 |
| login_after JOIN SQL replay（`login_time > '2026-01-01'`） | 0 行、无 SQL 错误 | 21 行、无 SQL 错误 |
| `POST /api/person/list/login/after/object`（合法 date） | HTTP 500 `internal server error` | HTTP 500 `internal server error` |

- 结论：SQL 本身合法且补种后能返回 21 行，但 API 仍 500 → BUG-F 为**后端错误语义 bug**（`endpoints_person2.rs:39 login_after` 吞错），与数据量/空库无关；前端 `s()` 能 catch 该 500，不阻塞 #10/#11。因写范围不含后端且有 Rust 工具链阻塞，留 backlog 交 captain。

## 关键环境阻塞与修复过程（如实记录）

1. 旧前端产物（BUG-A）：新版部署后消失，登录页语义 id/aria-label 已恢复。
2. 种子账号口令不可用：测试库种子 admin 的 hash 为占位值、其他已知口令的账号未 seed，导致认证失败；经授权新建专用 E2E 账号后解决。
3. CSRF Origin 门禁：后端 cookie 写接口要求 Origin 与 `APP_PUBLIC_ORIGIN` 精确一致；隧道代理改变了浏览器 Origin 语义，导致签名保存/登出等写接口 403 `ExceptionAccessDenied`。已在测试代理层规范化 Origin 后消除（非产品 bug）。
4. 两处测试选择器漂移：密码显隐按钮改为 `.eye-btn`、流程弹窗改为 `#start-process`/`#start-title`（均已修进 spec）。
5. BUG-D（await 位置错误）已修复上线；TypeError 消失，本轮补种后 #10/#11 转 GREEN。
6. BUG-E（打开任务误调写端点 expire）已由 t13 前端修复、t14 部署；workflow 闭环持续 GREEN。
7. #10/#11 曾因测试库组织数据全空失败（表格 0 行 / 分页超时）；t16 经授权补种 21 条组织 seed 后转 GREEN（非产品 bug）。
8. 4 workers 同账号并发登录偶发会话互斥（环境配置）；全量改用 --workers=1 串行后稳定 32/32。

## 可复现 bug 列表（交 captain 分流）

- **BUG-A**（部署漂移 / fe-desktop，**已修复**）：部署版登录页密码框无 `id="login-password"`、显隐按钮无 `aria-label="显示或隐藏密码"`；新版部署后复验通过。
- **BUG-B**（后端错误语义 / t9）：登录失败时提示 "Session expired, please login again"（会话过期），非用户名/密码错误提示，属误导性错误语义。
- **BUG-C**（认证凭证 / 环境问题，**已解决**）：种子 admin 的 hash 为占位值不可登录；经授权新建专用 E2E 账号解决，非产品代码 bug。
- **BUG-D**（真实前端 bug / fe-desktop，**已修复上线并验证**）：`OrgDirectoryApp.vue` 的 `collect(label, await api...)` await 位置错误 → `s(p)=p.catch` 对已解析值抛 `TypeError: e.catch is not a function`。t10 修复已部署；TypeError 消失。
- **BUG-E**（真实产品 bug / e2e-runner-2 实证，**已修复并 GREEN**）：`ProcessWork.vue:1603 loadTaskInfo` 曾把 GET `/task/expire/{id}` 当只读调用，实为写端点把 active 任务置 expired，导致 `POST /task/{id}/complete` 400。t13 纯前端修复、t14 部署；workflow 闭环双 project 全部 passed。
- **BUG-F**（产品代码 bug / 后端，**待修 backlog**）：`POST /api/person/list/login/after/object` 对合法 date 稳定返回 500；`endpoints_person2.rs:39 login_after` 把 DB 错误统一吞为 `Err(AppError::Internal)`，补种后 SQL replay 返回 21 行仍 500，证明与数据量无关，属错误语义 bug。前端已能 catch，不阻塞主流程。

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
# 全量真机回归（建议 --workers=1 串行避免同账号会话竞争）：
pnpm exec playwright test e2e/live-gui-expanded.spec.ts e2e/workflow-runtime.spec.ts --workers=1
```

失败时 `playwright.config.ts` 已配置：`trace: 'retain-on-failure'`、`screenshot: 'only-on-failure'`、`video: 'only-on-failure'`，
产物落在 `oa4rust/test-results/`。

## 本轮已扩充的真实 GUI 用例（共 15 条，均为可见 DOM 交互）

1. 登录页空表单提交按钮禁用 2. 密码显隐切换 3. 错误密码行内报错 4. 侧边栏导航+面包屑 5. 用户菜单跳个人中心 6. 全局搜索快捷键 7. 侧边栏折叠展开 8. 个人签名填写提交 9. 系统设置分栏+下拉 10. 通讯录查询本页表格 11. 通讯录分页 12. 两次密码不一致校验 13. 表单不完整校验 14. 未登录重定向 15. 退出登录后重定向
（workflow-runtime 另 1 条：发起→填写→提交→审批闭环，本轮已 GREEN）
