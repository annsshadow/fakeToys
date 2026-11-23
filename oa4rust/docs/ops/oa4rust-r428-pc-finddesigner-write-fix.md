# R428 PC 查找设计器 FindDesignerApp 保存写链 200+type:error 误报成功 P1 修复

> 证据来源：Playwright Chromium headless 1440x900 对已部署测试服的真实页面实测，**非物理真机**；截图为该 headless 浏览器真实截屏。

## 一、根因与判定

- 目标：`frontend/apps/desktop/src/views/FindDesignerApp.vue` 的 `save()`（:92-108），模板 :9 保存按钮真实可达，路由 `/app/admin/find-designer`，无门控（EndpointFold/DEV 全 0 命中，DOM fold=0）。
- 根因：:106 `.then(() => toast.info("保存成功"))` 完全不读响应信封；后端 `save_designer`（lib.rs:621-631）UPDATE 0 行命中 `ActionResult::error("query design not found")` HTTP 200+type:error，SDK 仅非 2xx 抛错，进入 .then 误报成功且不刷新（R427 甄别坐实，createNew 假 id 与真列表行两路径均复现）。

## 二、修复

- 新增 `findDesignerWriteSucceeded(resp:any):boolean`：`type:error` 或 `success:false` 判失败；`type:success` 或 type 缺省且 success 非 false 判成功；非对象/null 判失败；未知 type 判失败；不强求 named bool（后端成功 `success({id,saved,name,query})`，权威字段 saved 非统一 bool 名，严格 bool 会假阴性）。
- `save()` 的 `.then((r:any) => {...})` 接响应：失败 `toast.error(r?.message || "保存未生效，请刷新后重试")` 并 return，不弹成功不刷新；成功才 `toast.info("保存成功")` + `refetch()` 刷新列表（useQuery 解构补 refetch）。
- createNew 假 id 路径与真列表行路径共用同一 save()，均被守卫覆盖；`.catch` 非 2xx 原文案保持。
- 零改：`preview()`/`createNew()`/`selectItem`/各 load* 只读、列表展示 toast、后端/SDK/端点/payload/method/模板/其它页/migrations。
- 完整 diff：仅 useQuery 解构 +refetch（1 行）、helper（7 行）、`.then` 守卫（8 行），16 insertions / 2 deletions。

## 三、契约 RED→GREEN 与验证

- 契约 `frontend/apps/desktop/src/views/FindDesignerApp.writeEnvelope.contract.test.ts`：@vitest-environment node，readFileSync 源码 + bodyOf 花括号配对 + stripTs + new Function 真执行注入 selected/config/api.put/toast/refetch/helper；17 it（helper 6 + 真列表行 5 + createNew 假 id 2 + 异常与端点 3 + preview 零写 1）。
- 真父 HEAD 8f8fa8ec（无 helper）：RED 8 failed / 9 passed（全为真实断言：helper2 + type:error 真行/假id + success:false + 非对象 + 成功刷新 + PUT端点/refetch，0 桩错误）。
- 修复版：GREEN 17/17；根全量 `pnpm test` 302 files / 2864 tests 0 failed；desktop `pnpm exec tsc --noEmit` exit 0。
- 构建入口 `index-BPo9SSky.js`，整目录 tar + sha256 校验原子部署测试服 `DEPLOY_R428_PC_FULL_OK`。

## 四、真浏览器实测（注入对照，非物理真机）

- 1440x900 headless，路由 `/app/admin/find-designer`，DOM 审计：注入 1 行假配置、保存按钮 1、新建按钮 1、fold=0；createNew 假 id 路径与真列表行路径均覆盖。
- 14 断言全 true / failed=[] / totalWrites=6 / pageErrors=[]：
  - 真列表行 200+type:error → 红 toast 透传 `query design not found`、无成功、不刷新、写 1 次；
  - createNew 假 id 200+type:error → 同上；success:false → 红 toast 透传 `R428 denied`、无成功；
  - type:success → 蓝 toast `保存成功` 且刷新列表；HTTP500 → HTTP 500；403 → 没有权限。
- 证据目录（gitignored）：`test-results/r428-pc-finddesigner-write-fix/`（evidence.json + 7 张 1440x900 截图）。
- 测试服地址按脱敏占位 `<YOUR_TEST_DOMAIN>` / `<YOUR_TEST_HOST>` 表示，本文不含任何真实域名/IP/主机/凭据。

## 五、截图 sha256（headless 1440x900，非物理真机）

- 00-dom-audit.png = `196570c7c6f03b259981a3472122e53431f275b24f5801bfa3cac5d64a438bfe`
- 01-typeerror-row.png = `a7aa82b3bcbf6eb7ec82b41def5419e0ab192ec8c86cbca632c1421469cbac55`（红 ✗query design not found，无成功，选中真列表行）
- 02-typeerror-create.png = `2984aea11db3a51d4af3ad751029acad86cddac6a19895cbe16b602624241366`（createNew 假 id 同一红 toast）
- 03-success-false.png = `a58e40cfcd9f00540661e68ac6209b619a2c9c9b74de55973151db1a8ef9e568`（红 ✗R428 denied）
- 04-true-success.png = `5414539f98d80fd765139d24467220f7973e560c16b6215f5e2d09aec1523a8c`（蓝 ℹ保存成功 + 刷新）
- 05-http500.png = `8c565390870b82d0fd4002162c7d4c85b5e59937ded549e172c12bb8b0997d11` / 06-http403.png = `d343dc9b21decc79ddc670dd178a18ab1444bd413d2896d339ff0f7ce8d7b3f3`（错误 toast 正确）
