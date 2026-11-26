# R302 DR-03b：组织通讯录加载兜底与空态修复

## 背景
基线审计（R300）记录 PC 通讯录 `/app/directory` 存在「加载常驻 Loading / 失败无提示」缺陷：人员名册请求一旦挂起或失败，`busy` 无法复位，页面停留在查询中，用户无明确错误反馈与重试入口。

## 改动范围
- `frontend/apps/desktop/src/views/OrgDirectoryApp.vue`
  - 新增 `withTimeout(promise, ms)` 竞速超时封装，名册请求超时阈值 `PERSON_LOAD_TIMEOUT_MS = 10000`。
  - `loadPersonFilter()` 用 `withTimeout` 包裹 `api.post`；`catch` 写入 `loadError` 持久错误态并 toast；`finally` 必复位 `busy`。
  - 模板新增 `data-testid="org-load-error"` 错误条与 `data-testid="org-load-retry"` 重试按钮；空态文案改为引导「点击加载名册或搜索人员」。
  - `onMounted` 首屏自动加载名册，避免进入页面即空列表。
  - 业务逻辑与接口路径均未改动，仅补加载/错误/重试兜底。
- `frontend/apps/desktop/src/views/OrgDirectoryApp.loadingFallback.contract.test.ts`（新增契约，4 项）

## 契约（test-first RED→GREEN）
1. 请求挂起 20ms 后按超时 reject（不常驻）。
2. 请求快速成功时原样返回结果。
3. `loadPersonFilter` 用 `withTimeout(api.post(...))` 包裹、`catch` 落 `loadError`、`finally` 复位 `busy`。
4. 模板提供明确错误提示、重试按钮与空态加载引导。

- RED：4/4 失败（无 `withTimeout`、无错误态节点）。
- GREEN：`pnpm --filter @oa4rust/desktop exec vitest run src/views/OrgDirectoryApp --reporter=dot` → 14 passed（含本契约 4 项与既有 10 项）。

## 验证
- 契约测试：3 文件 14 passed，exit 0。
- 类型检查：`pnpm --filter @oa4rust/desktop typecheck`（在含 package.json 的工程根执行）exit 0。
- 构建：`pnpm --filter @oa4rust/desktop build` exit 0；产物 `index-CtCVbO26.js` / `index-BOp8VUI6.css`。
- 部署：冻结构建快照后原子替换测试服静态目录（部署脚本 gitignored），远端 index.html 哈希校验一致。

## 真浏览器实测（Playwright Chromium headless）
- 来源：对已部署测试服的真实页面操作，headless 浏览器本地渲染；**非物理真机**。
- 访问域名由本地环境变量/脚本注入，本文以 `<YOUR_TEST_DOMAIN>` 占位，不入库真值。
- 真实 UI 流程：登录 → 进入通讯录 → 等待首屏自动加载 → 点「加载名册」→ 注入网络失败 → 点「重试加载」恢复。
- 结果：
  - 首屏自动加载：`共 21 人 · 第 1 / 2 页`，列表 20 行，按钮未禁用、无错误条。
  - 重新加载：数据回显一致，无常驻 Loading。
  - 注入失败：错误条出现「名册加载失败…请检查网络或稍后重试」，重试按钮可见。
  - 点重试：错误清空、名册恢复 21 人。
- 证据（gitignored，本地）：`test-results/r302-dr03b/`（4 张 PNG + `evidence.json`）。

## 遗留 / 非本轮范围
- H5 端通讯录、后端接口未改（本任务 in-scope 仅 PC 通讯录页）。
- 其他并发成员改动（工作台、业务域等）未触碰。
- 项目为 ≥1000 轮持续迭代，本轮非完成态。
