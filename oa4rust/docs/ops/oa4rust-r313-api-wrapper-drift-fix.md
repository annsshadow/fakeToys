# R313 API 封装路径/方法漂移修复

## 来源与范围

- 任务：t77（R313），工作分支 opt1000/oa4rust-pc-ai-refactor，本轮不修改后端业务源码。
- 部署与实测：测试服 `<YOUR_TEST_HOST>`（`<YOUR_TEST_DOMAIN>`）经 SSH BatchMode 原子部署，Playwright Chromium headless 真实登录与调用，非物理真机；凭据由本地脚本从远端读取，不入库。
- 交付文件：`frontend/packages/apis/src/index.ts`、`frontend/packages/apis/src/apiWrapperDrift.R313.contract.test.ts`、本文档。

## 漂移根因（后端注册对照）

| 封装 | 修复前 | 后端真实注册 | 修复后 |
| --- | --- | --- | --- |
| `documentApi.create` | `POST /api/document/document` | `cms_assemble_control/src/routes.rs:526` 仅 `POST /api/document` | `POST /api/document` |
| `documentApi.update` | `PUT /api/document/document/{id}` | `routes.rs:527` 仅 `POST /api/document/{id}/update`（路径+方法双漂移） | `POST /api/document/{id}/update` |
| `programCenterApi.dictCreate` | `POST /api/program_center/dict/create` | `program_center/src/routes.rs:566` 仅 `POST /api/program_center/dict` | `POST /api/program_center/dict` |

- `/api/document/document` 在后端只有 `.../document/fields` 的 GET 子路由（`routes.rs:540`），并无文档实体创建路由；`/api/program_center/dict/create` 无注册。
- 既有 `api-coverage-generated.test.ts` 只断言函数存在，注释明确 401/404 也视为通过，因此路径漂移长期潜伏，`documentApi.create/update` 与 `programCenterApi.dictCreate` 在真实调用中命中 404 HTML 或空响应。

## `apis.general.dictCreate` 同源核对结论

- 非同源、无需修改：`generalApi.dictCreate`（`index.ts:280`）走 `POST /api/general/dict/create`，后端 `general_core_entity/src/lib.rs:1073` 已真实注册；与 `programCenterApi.dictCreate` 是不同命名空间、不同业务字典端点。
- `apis-integration.test.ts:174-179` 已锁定 `apis.general === generalApi` 与该路径，本轮契约据此断言同源且不混用 `/api/program_center/dict`。

## 契约 RED → GREEN

- 新契约 `apiWrapperDrift.R313.contract.test.ts` 使用 `vi.mock("@oa4rust/sdk")` 真断言三封装调用的方法与 URL（非覆盖率驱动）：4 个 it 覆盖 create、update、dictCreate、general 同源核对。
- 修复前 RED：3 failed / 1 passed，精确暴露三处漂移（仅同源核对通过）。
- 修复后 GREEN：4/4 passed；`pnpm run typecheck` 通过（Scope 6/7）；`packages/apis` 全量 vitest 8 files / 738 tests 全绿，无回归。

## 构建、部署与真浏览器复验

- 桌面构建通过，入口哈希 `index-D1EjQmHJ.js` / `index-BMFkaD73.css`；部署脚本以唯一 STAMP 备份 `web.bak-$STAMP` 后原子替换，远端入口哈希与本地一致。
- 因桌面 bundle 未直接引用 `@oa4rust/apis`，真浏览器验证用 esbuild 将真实源码封装打包为探针，注入已登录的测试服工作台页面执行；`@oa4rust/sdk` 的 `api.setBase` 指向部署网关前缀。
- 真浏览器结果（1440×900）：`create → POST /oa4rust-api/api/document 200 JSON`（返回真实 id 与 draft 状态）；以该 id 执行 `update → POST /oa4rust-api/api/document/{id}/update 200 JSON`（updated:true）；`dictCreate → POST /oa4rust-api/api/program_center/dict 200 JSON`；`apis.general === generalApi` 同源成立；无漂移路径命中、无 nginx 404 HTML、`pageErrors=[]`。
- 证据：`test-results/r313-wrapper-drift/01-wrapper-call.png`（真实登录后工作台页面截图，非静态假）、`test-results/r313-wrapper-drift/evidence.json`（`source` 标注 Playwright Chromium headless against deployed test server; not a physical device）。

## 提交

- 精确 `git add` 三份交付文件为单一提交，提交信息不写真值；部署与浏览器脚本、截图证据均落在 gitignored 的 `test-results/`，不入版本库。
