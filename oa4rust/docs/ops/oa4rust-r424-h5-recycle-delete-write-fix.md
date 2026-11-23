# R424 H5 回收站彻底删除吞信封误报成功修复

## 根因

- `mobile/src/services/index.ts` 的 `recycleApi.remove` 带 `discardResponse: true`，SDK `http.ts` 对 2xx 直接 resolve `{success:true,data:undefined}`，吞掉后端信封。
- 后端 `recycle_id_delete` 查无行/DELETE 0 行返回 HTTP 200 + `{type:"error", message:"file not in recycle bin"}`，被吞后 `recycle.vue` 的 `remove()` 无条件 `uni.showToast({title:"已删除"})`，误报成功且行仍在。

## 修复

- `recycleApi.remove` 去掉 `discardResponse: true`，返回类型调整为真实信封 `Record<string, unknown>`；端点 `/api/recycle/delete/{id}` 与 DELETE 方法不变。
- 新增 `recycleDeleteSucceeded(resp: any): boolean`：`type:error` 或 `success:false` 为失败；`type:success` 或 type 缺省且 success 非 false 为成功；非对象/null 为失败；不强求权威 bool（后端成功字段 `id/deleted:true` 无统一 bool 名，严格 bool 会假阴性）。
- `remove()` 用 `lastResp` 承接响应并守卫：失败 `toast.error` 透传后端 message（缺省「删除失败，请刷新后重试」）并 return，不弹已删除、不 load 刷新、保留行；成功才 `uni.showToast({title:"已删除"})` + `await load(rows.value.filter(...))` 乐观排除已删行。
- `showModal` 确认守卫、busy、catch、finally 完全不动。

## 零改范围

- SDK `http.ts` 全局 discardResponse 语义与 `http.test.ts` 钉住的测试（`discardResponse` 仍生效于其它调用点）。
- `recycleApi.empty/emptyAll`、`bbs/new.vue`、其它 `discardResponse` 调用点（login/logout/refresh/process data/program_center collect）、后端、端点/payload/method、其它页面、migrations。
- `recycle_empty` 恒 success 列为 L1 观察项，本轮不处理。

## 验证

- RED（真父 77abd74c0，新契约）：15 failed / 1 passed，失败全为真实断言（去 discardResponse 1 + helper 缺失 7 + remove 失败透传/成功守卫 7），0 桩错误。
- GREEN 四条（均在 `mobile/` 目录执行，exit 0）：
  - `pnpm exec vitest run src/pages/recycle/recycle.writeEnvelope.contract.test.ts`：16/16。
  - `pnpm exec vitest run src/services/http.test.ts`：41/41（discardResponse 全局语义未破坏）。
  - `pnpm test`：76 files / 502 tests，0 failed。
  - `pnpm typecheck`：exit 0。
- H5 构建 `pnpm run build:h5` 成功，入口 `index-BIqurAns.js`；整目录 tar + sha256 校验原子部署到 <YOUR_TEST_HOST> 测试服（`DEPLOY_R424_H5_FULL_OK`）。

## 真浏览器实测（非物理真机）

- Playwright Chromium headless，390x844 viewport，isMobile/hasTouch，DPR2，对部署后的 `<YOUR_TEST_DOMAIN>` H5 测试服实测；截图为 fullPage（DPR2 放大），**非物理真机**。
- 15/15 断言全 true，failed=[]，pageErrors=[]。决定性对照：
  - 假 ID DELETE 200 + type:error → 红 toast「file not in recycle bin」，无「已删除」，行保留，写 1 次 DELETE。
  - 200 + success:false → 红 toast「R424 denied」，无「已删除」，行保留。
  - 200 + type:success → 绿 toast「已删除」，行消失（空态），仅 1 次写。
  - HTTP 500 → 「HTTP 500」且无成功；HTTP 403 → 「没有权限」且无成功；取消 → 0 写、行保留。
- 证据（gitignored）：`oa4rust/test-results/r424-h5-recycle-write-fix/`（evidence.json + 7 张 fullPage 截图）。关键截图 sha256：
  - 01-typeerror-file-not-in-recycle-bin.png = 75ac0838626bb6a7f851d25f21f10f8f5c165cd5d46063d1fbb9937d85728539
  - 02-success-false.png = 4ee0d7771b67276e45062db8a5912cd0be68470daaef3a1b703b007c79eabdbf
  - 03-true-success.png = 626bc9a6a585f51d47ff428c0c2f82fdd84790340f30156c39b8decc06417205
- 肉眼确认：01/02 失败态仍显示「R424测试回收项」行，03 成功态显示「回收站为空」空态。

## 提交

- 精确 4 文件：`mobile/src/services/index.ts`、`mobile/src/pages/recycle/recycle.vue`、`mobile/src/pages/recycle/recycle.writeEnvelope.contract.test.ts`（新建）、本文档。
