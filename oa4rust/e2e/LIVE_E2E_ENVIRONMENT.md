# OA4Rust 真机 GUI E2E — 环境与实跑说明

> 本文档只描述「值从哪里读」，不含任何真实地址、域名、IP、账号或口令。
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

## 当前环境阻塞（如实记录，未伪造通过）

- 本会话环境：`BASE_URL`、`E2E_USERNAME`、`E2E_PASSWORD`、`E2E_PROCESS_NAME`、`COLLECT_COVERAGE` 均未设置（`UNSET`）。
- 仓库内不含真实地址/凭证：无 `.env`、无 `.local.keys`、无 `remote-config.env`，且按保密红线不从 git 历史或其他文件翻找/猜测真值。
- 本机常见本地端口（3000 / 5173 / 8080 / 8000 / 3001）无监听服务。
- 因此 `global-setup` 预检必然在「缺环境变量」阶段失败，**本轮未对测试服务器实跑，未产出通过/失败逐条结果，也没有 trace/截图**。
- 实跑证据（如实记录）：执行 `pnpm exec playwright test --project=live-chromium e2e/live-gui-expanded.spec.ts`，`global-setup.ts:11` 抛出 `Live GUI E2E requires E2E_USERNAME and E2E_PASSWORD; credentials are never defaulted or silently skipped`，用例未进入浏览器阶段。
- 状态：**待 captain 注入环境后实跑**。

## 环境注入后可直接执行的实跑命令模板（占位符形式）

```bash
cd <REPO_ROOT>/oa4rust
export BASE_URL='<YOUR_TEST_HOST>/oa4rust/'
export E2E_USERNAME='<YOUR_E2E_USER>'
export E2E_PASSWORD='<YOUR_E2E_PASSWORD>'
# 可选：export E2E_PROCESS_NAME='<YOUR_SEED_PROCESS_NAME>'
pnpm exec playwright test --project=live-chromium e2e/live-gui-expanded.spec.ts
# 全量真机回归（含登录/导航/表单/工作流/表格/错误/权限）：
pnpm exec playwright test --project=live-chromium
```

失败时 `playwright.config.ts` 已配置：`trace: 'retain-on-failure'`、`screenshot: 'only-on-failure'`、`video: 'only-on-failure'`，
产物落在 `oa4rust/test-results/`；逐条结果与汇总 JSON 落在 `playwright-coverage/results.json`（COLLECT_COVERAGE=true）或 `test-results/results.json`。

## 本轮已扩充的真实 GUI 用例（共 15 条，均为可见 DOM 交互）

1. 登录页空表单提交按钮禁用（登录/表单初始态）
2. 密码显隐切换 input type（登录/按钮点击）
3. 错误密码行内报错且不进入工作台（登录/错误处理）
4. 侧边栏导航切换路由并同步面包屑（导航）
5. 用户菜单打开并跳转个人中心（导航/菜单）
6. 全局搜索快捷键聚焦（键盘交互）
7. 侧边栏折叠与展开（外壳交互）
8. 个人签名填写并提交成功（表单填写/提交）
9. 系统设置分栏切换并操作安全字段（表单/下拉）
10. 通讯录「查询本页」渲染表格（表格操作）
11. 通讯录人员分页前进/后退（表格分页）
12. 两次密码不一致前端校验、不发写请求（错误处理）
13. 表单不完整校验、不发写请求（错误处理）
14. 未登录访问受保护路由重定向并保留 redirect（权限路径）
15. 退出登录后受保护路由再次重定向（权限路径）

## 可复现 bug 列表

- 本轮无服务器实跑，**未发现可复现 bug**；发现将在环境注入后按「步骤/预期/实际/trace路径」登记并交 captain 分流。
