# R436 PC 服务调用设计器执行写链 200+type:error 误报成功 P1 修复

> 证据来源：Playwright Chromium headless 1440x900 对已部署测试服的真实页面实测，**非物理真机**。

## 一、根因与判定

- 目标：`frontend/apps/desktop/src/views/ServiceInvokeDesignerApp.vue` 的 `runM.onSuccess`（:179-181），模板 :29「执行」按钮真实可达，路由 `/app/admin/service-invoke-designer`，无门控。
- 根因：后端 `u3_invoke_execute_inner`（program_center lib.rs:11528+）对假 flag 返回 HTTP 200 + `type:error(invoke not found)`，SDK 仅非 2xx 抛错，onSuccess 无条件 `toast.success("服务调用已执行")` 不读响应信封（R435 甄别坐实）。

## 二、修复

- 新增 `serviceInvokeWriteSucceeded(resp:any):boolean`：`type:error` 或 `success:false` 判失败；`type:success` 或 type 缺省且 success 非 false 判成功；非对象/null 判失败；未知 type 判失败；不强求 named bool（后端成功 `success({id,executed:true})` 权威字段 executed 非统一 bool 名，严格 bool 致假阴性）。
- `runM.onSuccess(lastData)` 接 mutationFn 响应并守卫：失败 `toast.error(lastData?.message || "执行未生效，请刷新后重试")` 并 return，不弹成功；成功才 `toast.success("服务调用已执行")`。
- `onError` 非 2xx 原文案保留；`saveItem/saveM`、`deleteItem/delM`（后端恒 success 业务布尔无 200+error 列 L1）、`loadData`/只读、端点/payload/method/模板/后端/SDK/其它页零改。
- 完整 diff：仅 helper + onSuccess 守卫，14 insertions / 1 deletion。

## 三、契约 RED→GREEN 与验证

- 契约 `frontend/apps/desktop/src/views/ServiceInvokeDesignerApp.runInvoke.contract.test.ts`：@vitest-environment node，readFileSync 源码 + bodyAfter 花括号配对（引号/转义处理）+ Fn 真执行注入 toast/helper/lastData；16 it（helper 6 + onSuccess 守卫 6 + onError/零改 4）。
- 真父 HEAD f7d7245f6：RED 9 failed / 7 passed（helper 6 + onSuccess 失败守卫 3，全部真实断言，0 桩错误）。
- 修复版：GREEN 16/16；根全量 `pnpm test` 306 files / 2937 tests 0 failed；desktop `pnpm exec tsc --noEmit -p frontend/apps/desktop/tsconfig.json` exit 0。
- 构建入口 `index-GDbJ75NK.js`，整目录 tar + sha256 校验原子部署测试服 `DEPLOY_R436_PC_FULL_OK`。

## 四、真浏览器实测（注入对照，非物理真机）

- 1440x900 headless，路由 `/app/admin/service-invoke-designer`，DOM 审计：1 行注入假调用、执行按钮 1、fold=0。
- 11 断言全 true / failed=[] / totalWrites=5 / pageErrors=[]：
  - 假 flag execute 200+type:error → 红 toast 透传 `invoke not found`、无成功、写 1 次；
  - 200+success:false → 红 toast 透传 `R436 denied`、无成功；
  - type:success → 绿 `服务调用已执行` 且无错误；HTTP500 → HTTP 500；403 → 没有权限。
- 证据目录（gitignored）：`test-results/r436-pc-serviceinvoke-write-fix/`（evidence.json + 6 张 1440x900 截图）。
- 测试服地址按脱敏占位 `<YOUR_TEST_DOMAIN>` / `<YOUR_TEST_HOST>` 表示，本文不含任何真实域名/IP/主机/凭据。

## 五、截图 sha256（headless 1440x900，非物理真机）

- 00-dom-audit.png = `dc2b25b4d3368a4aca1a39d369ad53fb7377630177a3e7259a554da3f52676b2`
- 01-typeerror-invoke-not-found.png = `ff899a9f23d20cff2fb75ebcf3e4169889091d1d2db966df92002df3601d5f2b`（红 ✗invoke not found，无成功）
- 02-success-false.png = `a77764d3241e94d4f961d958a869a60f906b17531881ca3137cf86c39a72e126`（红 ✗R436 denied，无成功）
- 03-true-success.png = `90c135535e9f1e8f74b69e2cff22140fdb6765b961d1e7eee7f9a652198bbca6`（绿 ✓服务调用已执行）
- 04-http500.png = `0c47141795892ba2d2ecbd4cdbcd1d38f0c94e6ada846c9ad27d662764c3f8ad` / 05-http403.png = `402a32b312d952cbec6d2416de536d96d329e39795f2bb72f2a939e58a866951`（错误 toast 正确）
