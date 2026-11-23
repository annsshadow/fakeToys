# R421 PC 表单设计器 saveForm 更新分支 200+type:error 误报成功 P1 修复报告

> 证据来源：Playwright Chromium headless 1440x900 对已部署测试服的真实页面实测，**非物理真机**；截图均为该 headless 浏览器真实截屏。

## 一、根因与判定

- 目标：`frontend/apps/desktop/src/views/FormDesigner.vue` 的 `saveForm` 更新分支，模板入口 :15 保存按钮（`:disabled=!currentForm.name`），点击列表项加载表单后可达，无门控。
- 根因：:1111 `await api.put(/api/form/{id}, payload)` 不接响应信封，:1119 裸 `toast.info("保存成功")`；后端 `form_u2_update` 对假 ID 命中 HTTP 200 + `type:error(form not found)`，SDK 仅非 2xx 抛错，误报成功成立（R420 甄别坐实）。R421 修复此 P1。

## 二、修复

- 新增 `formDesignerWriteSucceeded(resp:any):boolean`：`type:error` 或 `success:false` 判失败；`type:success` 或 type 缺省且 success 非 false 判成功；非对象/null/undefined 判失败；未知 type 判失败；不强求 named bool（后端成功返回 id/saved 非统一 bool 名，严格断言会假阴性）。
- `saveForm` 更新分支改 `const lastResp:any = await api.put(...)`；守卫失败 `toast.error` 透传 `message`（缺省"操作未生效，请刷新后重试"）并 return——不写回 definition/appId、不 loadForms、不弹成功；成功才原 `toast.info("保存成功") + 写回 + loadForms()`。
- 创建分支 `created?.data?.id` 校验、submitPreview（死代码，submitBtnCount=0）与其它 5 处 toast.info、只读 load*/surfaceOps 及后端/SDK/端点/payload/method/模板/其它页面/migrations 零改。
- 完整 diff：仅 helper + saveForm 更新分支守卫，14 insertions / 2 deletions。

## 三、契约 RED→GREEN 与验证

- 契约 `frontend/apps/desktop/src/views/FormDesigner.writeEnvelope.contract.test.ts`：@vitest-environment node，readFileSync 源码 + bodyOf 花括号配对（处理单双反引号与转义）+ stripTs + new Function 真执行注入 currentForm/routeAppId/layoutConfig/mobileLayout/validationRules/validationGroups/serializeFormDefinition/formSavePayload/api.put+post/loadForms/toast.info+error；13 it（helper 6 + 更新分支 6 + 创建分支 1）；helper 缺失回退 false 保证 RED。
- 真父 HEAD b79fd97b3（无 helper）：RED 4 failed / 9 passed（全部真实断言，0 桩错误；helper 相关 2 + type:error/success:false 透传各 1）。
- 修复版：GREEN 13/13；根全量 `pnpm test` 301 files / 2847 tests 0 failed；desktop `pnpm typecheck` exit 0。
- 构建入口 `index-CQ-XdJGn.js`（chunk `FormDesigner-CyyHx9jV.js`），整目录 tar + sha256 校验原子部署测试服 `DEPLOY_R421_PC_FULL_OK`。

## 四、真浏览器实测（注入对照，非物理真机）

- 1440x900 headless，路由 `/app/admin/form-designer`，DOM 审计：1 行注入假表单、保存按钮 1、点击列表项后按钮启用、fold=0。
- 13 断言全 true / failed=[] / totalWrites=5 / pageErrors=[]：
  - type:error → 红 toast 透传 `form not found`、无成功、不刷新、写 1 次；
  - success:false → 红 toast 透传 `R421 denied`、无成功、不刷新；
  - type:success（不带权威 bool）→ 绿 toast `保存成功`、loadForms 刷新；
  - HTTP500 / 403 → 红 toast 正确；0 pageerror。
- 证据目录（gitignored）：`test-results/r421-pc-formdesigner-write-fix/`（evidence.json + 6 张 1440x900 截图）。
- 测试服地址按脱敏占位 `<YOUR_TEST_DOMAIN>` / `<YOUR_TEST_HOST>` 表示，本文不含任何真实域名/IP/主机/凭据。

## 五、截图 sha256（headless 1440x900，非物理真机）

- 00-dom-audit.png = `7ae07875541bea089e1455d4b25c819fc93df445f35c9c1ee5449882978bc86b`（假表单行 + 保存按钮）
- 01-typeerror-form-not-found.png = `e7704b9f949a276facd56e1c118578459b57fd4c827bd274725d998057497f22`（红 ✗form not found，无成功）
- 02-success-false.png = `7ce09d5c7e8c88244a3bbbd73c61a684f6baae5e6464e77291efd3e88a386da3`（红 ✗R421 denied，无成功）
- 03-true-success.png = `932ee9631905b8599aba7ed6ddaf665f9bc930316caeaab163dc415f653e120e`（绿 ✓保存成功）
- 04-http500.png = `3a632f05a4f60fcb07b469c50991b3f73aa9a87f65a42e456c1de098e23fa93e` / 05-http403.png = `92bface21984fc6dd894e432fc383ebaa1e7651e4ca9b25c98621ed6ca487daf`（错误 toast 正确）
