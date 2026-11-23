# R418 PC 视图管理删除写链 200+type:error 误报成功 P1 修复报告

> 证据来源：Playwright Chromium headless 1440x900 对已部署测试服的真实页面实测，**非物理真机**；截图均为该 headless 浏览器真实截屏。

## 一、根因与判定

- 目标：`frontend/apps/desktop/src/views/ViewApp.vue` 的 `removeView`，模板真实入口在删除按钮，无门控可达。
- 根因：:128 `await api.delete(/api/view/{id})` 后无条件 `toast.success("已删除") + loadViews()`，不读响应信封；后端 `view_u2_delete` 对假 ID 命中 HTTP 200 + `type:error(view not found)`，SDK 仅非 2xx 抛错，误报成功成立（R417 甄别坐实）。 R418 修复此 P1。

## 二、修复

- 新增 `viewDeleteWriteSucceeded(resp:any):boolean`：`type:error` 或 `success:false` 判失败；`type:success` 或 type 缺省且 success 非 false 判成功；非对象/null/undefined 判失败；未知 type 判失败；不强求 named bool（后端成功 `deleted:true` 非统一 bool 名，严格断言会假阴性）。
- `removeView` 用 `const lastResp:any = await api.delete(...)` 承接响应；守卫失败 `toast.error` 透传 `message`（缺省"操作未生效，请刷新后重试"）并 return——不弹成功、不 loadViews；成功才原 `toast.success("已删除") + loadViews()`。
- `confirmMsg` 守卫保留；只读函数 loadViews/loadViewCategories/viewDetail/viewData 及后端/SDK/端点/method/payload/模板/其它页面/migrations 零改。
- 完整 diff：仅 helper + removeView 守卫，13 insertions / 1 deletion。

## 三、契约 RED→GREEN 与验证

- 契约 `frontend/apps/desktop/src/views/ViewApp.writeEnvelope.contract.test.ts`：@vitest-environment node，readFileSync 源码 + bodyOf 花括号配对 + stripTs + new Function 真执行注入 api.delete/toast/confirmMsg/loadViews/helper；16 it（helper 7 + removeView 9）；参数 `v:any` 注入避免 TS7006。
- 真父 HEAD 74edaa1d3（无 helper）：RED 6 failed / 10 passed（全部真实断言，0 桩错误）。
- 修复版：GREEN 16/16；根全量 `pnpm test` 300 files / 2834 tests 0 failed；desktop `pnpm typecheck` exit 0。
- 构建入口 `index-BRh8AFlo.js`，整目录 tar + sha256 校验原子部署测试服。

## 四、真浏览器实测（注入对照，非物理真机）

- 1440x900 headless，路由 `/app/admin/view`，DOM 审计：1 行注入假视图、删除按钮 1、fold=0。
- 16 断言全 true / failed=[] / totalWrites=6 / pageErrors=[]：
  - type:error → 红 toast 透传 `view not found`、无成功、不刷新（reloadDelta=1 仅 goto）、行保留；
  - success:false → 红 toast 透传 `R418 denied`、无成功、行保留；
  - `deleted:0` 仍判成功（不强求 bool）→ 绿 toast 已删除、行消失；
  - type:success → 绿 toast 已删除、loadViews 刷新（reloadDelta=2）、行消失；
  - HTTP500 / 403 → 红 toast 正确；
  - confirm 取消 → 零写、行保留；0 pageerror。
- 证据目录（gitignored）：`test-results/r418-pc-viewapp-delete-write-fix/`（evidence.json + 8 张 1440x900 截图）。
- 测试服地址按脱敏占位 `<YOUR_TEST_DOMAIN>` / `<YOUR_TEST_HOST>` 表示，本文不含任何真实域名/IP/主机/凭据。

## 五、截图 sha256（headless 1440x900，非物理真机）

- 00-dom-audit.png = `de9c0a45c65415b0a9be28a5bc812942cec8ccbdf89b67e8a67e9c4adabe127c`（行与删除按钮可见）
- 01-typeerror-view-not-found.png = `e258eb751d70b1b87242fc27bd84c69f6455e66451716360e484094d178d1a7e`（红 ✗view not found + 行保留）
- 02-success-false.png = `79d17d64fe84284789c9ca2cb8f11340484d9c8644f8e2e5743da825e8f567d5`（红 ✗R418 denied + 行保留）
- 03-deleted-zero.png = `9ab9f82e7128867d81d95595469508fd718ea33ad57e387b5a738f469a4c77c3`（绿 ✓已删除，deleted:0 不强求 bool）
- 04-true-success.png = `9ab9f82e7128867d81d95595469508fd718ea33ad57e387b5a738f469a4c77c3`（绿 ✓已删除 + 行消失）
- 05-http500.png = `8d1bb9c3f119f1e52bffb9d3f48ca7b37affe810eea94b7510751ceb6df1229f` / 06-http403.png = `4372c0fe74ff45f5d9c384a9e8aeb5408cd54ac788435f46ee6c37a8679188ca` / 07-delete-cancel.png = `d164f18698de1cf9d1103c41f2529ef60580f21f3a1f5e01258192dc689e5afa`（错误 toast 与取消零写）
