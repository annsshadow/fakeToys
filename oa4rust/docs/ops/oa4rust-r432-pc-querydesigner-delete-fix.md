# R432 PC QueryDesigner deleteQuery 缺 confirmMsg 不可达 + 静默吞错修复

> 证据来源：Playwright Chromium headless 1440x900 对已部署测试服的真实页面实测，**非物理真机**；截图为该 headless 浏览器真实截屏。

## 一、根因与判定

- 目标：`frontend/apps/desktop/src/views/QueryDesigner.vue` 的 `deleteQuery`（:651-660），模板 :33 行内删除 `.qdel` 与 :53 删除按钮均真实可达。
- 缺陷一（功能不可达）：import 段 :278 仅 `import { toast }`，`deleteQuery` :652 调用 `confirmMsg` 却未导入 → 点击删除触发 Vue runtime ReferenceError，确认弹窗不出现、DELETE 0 次（R430 甄别坐实）。
- 缺陷二（同型静默吞错 P1）：补 import 后会暴露 :654 `await api.delete` 不读响应信封，后端 `delete_designer`（lib.rs:665-669）0 行命中返回 HTTP 200 + `type:error(query design not found or already deleted)`，前端仍 :655-656 清 selected + 本地移除行，视觉成功实际未删。故本轮组合修复两者，严禁只补 import。

## 二、修复

- import 段 :278 改为 `import { confirmMsg, toast } from '../utils/toast'`（与其余页面一致，confirmMsg 定义在 utils/toast.ts:182）。
- 复用 R431 已有 `queryDesignerWriteSucceeded(resp:any):boolean`（type 为主不强求 bool）：`deleteQuery` 改 `const lastResp:any = await api.delete(...)` + 守卫，失败 `toast.error(lastResp?.message || '操作未生效，请刷新后重试')` 并 return，不清 selected、不本地移除行；成功才 `selected=null` + `queries.filter` 移除行。
- `confirmMsg` 确认守卫（:652）与 catch（非 2xx）原文案保留。
- 零改：saveQuery（R431 已修）、runQuery/openEdit/loadQueries/saveSqlQuery、端点/payload/method/模板/后端/SDK/其它页。
- 完整 diff：import 1 行 + deleteQuery 守卫 5 行，零外溢。

## 三、契约 RED→GREEN 与验证

- 契约 `frontend/apps/desktop/src/views/QueryDesigner.deleteQuery.contract.test.ts`：@vitest-environment node，14 it；静态检查 import 段含 confirmMsg（否则 ReferenceError 不可达）+ bodyOf 抽取 deleteQuery 函数体 new Function 真执行注入 confirmMsg/api.delete/toast/selected/queries/helper（注入 q 参数与 string 类型标注）。
- 真父 HEAD ffe6f6ff3：RED 4 failed / 10 passed（import 缺 confirmMsg + 200+type:error/success:false/非对象三种失败信封未弹错误 toast，0 桩错误）。
- 修复版：GREEN 14/14；根全量 `pnpm test` 304 files / 2894 tests 0 failed；desktop `pnpm exec tsc --noEmit -p frontend/apps/desktop/tsconfig.json` exit 0。
- 构建入口 `index-CTr_a_4j.js`，整目录 tar + sha256 校验原子部署测试服 `DEPLOY_R432_PC_FULL_OK`。

## 四、真浏览器实测（注入对照，非物理真机）

- 1440x900 headless，路由 `/app/admin/query-designer`，DOM 审计：注入 1 行假查询、行内删除按钮 1、fold=0；confirm overlay 显示「取消/确认」。
- 14 断言全 true / failed=[] / totalWrites=5 / pageErrors=[]：
  - confirm overlay 正常显示（缺 import 时不会出现，本轮修复后可达）；
  - 200+type:error → 红 toast 透传 `query design not found or already deleted`、行保留、写 1 次 DELETE；
  - success:false → 红 toast 透传 `R432 denied`、行保留；
  - type:success → 行移除（空态「暂无查询定义」）；
  - confirm 取消 → 零写、行保留；HTTP500 → HTTP 500 行保留；403 → 没有权限行保留。
- 证据目录（gitignored）：`test-results/r432-pc-querydesigner-delete-fix/`（evidence.json + 7 张 1440x900 截图）。
- 测试服地址按脱敏占位 `<YOUR_TEST_DOMAIN>` / `<YOUR_TEST_HOST>` 表示，本文不含任何真实域名/IP/主机/凭据。

## 五、截图 sha256（headless 1440x900，非物理真机）

- 00-dom-audit.png = `4bf1c0546bcb79459884634f77c20527166b2059d1a43f5a137d75c7b9b1fb34`
- 01-typeerror-row-kept.png = `e6952d77455f8f8744c9d9d71170980e4b44d1e76c11be9280a881c7f44079b0`（红 ✗query design not found or already deleted，行保留）
- 02-success-false-row-kept.png = `0d6091ef054ba0d0ec82df294a8e996e4e6d61dcbcc6d3d240807093f40572f3`（红 ✗R432 denied，行保留）
- 03-true-success-row-removed.png = `42370a436bc72ae8fb1e8e97d2eb3991e8338d2d6228ce1f06ac0b55a18a75eb`（行移除，空态「暂无查询定义」）
- 04-http500.png = `c3b22846bb86f128dd32798636602ae90d6aaec24b067154eefb9d88c2f9cee4` / 05-http403.png = `11ac48aa613eff04a313100f5064198f841d2238b6b02ecd90187da78140ab8d` / 06-confirm-cancel.png = `c394ce2154ccbb95f2f6f62fd13a8d41c250569a49f82ea83336ca8c8283efbb`
