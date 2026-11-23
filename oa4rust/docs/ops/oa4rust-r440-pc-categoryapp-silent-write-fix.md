# R440 PC 分类管理（CategoryApp）新建/删除静默吞错修复

- 轮次：R440（t286 / attempt e02f5bea），依赖 R439 甄别结论。
- 协议：PC 1440x900 headless Chromium 真浏览器实测，**非物理真机**。
- 真父/RED 基线：55190c587806b2d00ae845ed6ac9e18288ecda3f（甄别/台账提交只改文档，源码同版）。

## 缺陷

CategoryApp.vue 的 createCategory（原 :95-106）与 deleteCategory（原 :107-115）在 await api.post/api.delete 后不接响应、不读 type，直接 load()；catch 仅覆盖非 2xx。真后端对假 appId/假 categoryId 返回 HTTP 200 + type:error（application not found / category not found），前端零反馈静默吞错——用户看不到任何失败提示，误以为已生效（零反馈静默吞错变体，2 个用户可达 P1）。

## 修复

- 新增 categoryWriteSucceeded(resp:any):boolean，以 type 为主不强求权威 bool：type:error 或 success:false 失败；type:success 或 type 缺省且 success 非 false 成功；非对象/null/undefined 失败；未知 type 失败。后端 create 成功返回 {id}，delete 成功返回 {deleted:true}，权威字段名不统一，严格 bool 判据会假阴性。
- createCategory：const lastResp 承接 api.post 响应，守卫失败 toast.error 透传 lastResp.message 并 return 不 load，保留用户上下文；成功才 load()。
- deleteCategory：const lastResp 承接 api.delete 响应，守卫失败 toast.error 透传并 return 不 load；成功才 load()。
- 本页成功分支原本无成功 toast（靠 load 重拉），修复只加失败 toast.error、不加成功 toast，保持产品语义。
- 零改：prompt 输入、confirmMsg 守卫、catch 非 2xx 原文案、load 只读、端点/payload/method、模板、后端、SDK、其它页面。

## 验证

- RED→GREEN：真父 55190c587 跑新契约 11 failed / 9 passed（全真实断言、0 桩错误）；修复版 20/20 全通过。
- 三 verify（oa4rust 目录）：pnpm exec vitest run frontend/apps/desktop/src/views/CategoryApp.writeEnvelope.contract.test.ts → 20/20 exit0；pnpm test → 308 files / 2974 tests 0 failed exit0；pnpm exec tsc -p frontend/apps/desktop/tsconfig.json --noEmit → exit0。
- 构建入口：index-D0jsC-Ui.js；整目录 tar + sha256 比对 + 远端 web.incoming-r440 原子替换部署测试服（DEPLOY_R440_PC_FULL_OK）。
- 真浏览器 E2E（1440x900 headless 非物理真机）：19 断言全 true、failed=[]、pageErrors=[]、totalWrites=10。列表注入假行替代破坏性真写（据实披露）：假 appId 假 categoryId 为后端 U2Gate NotFound 语义，POST/DELETE 走真路由注入 200 type:error 响应体，page.on(response) 记录真实请求 body 坐实。
- 决定性对照：新建 200+type:error(application not found) 红 toast 且不刷新；success:false 红 toast 且不刷新；type:success 才刷新；500/403 走 catch 红 toast 且不刷新；prompt 取消零写。删除 200+type:error(category not found)/success:false 红 toast、行保留且不刷新；type:success 行消失并刷新；500/403 红 toast行保留；confirm 取消零写行保留。

## 证据（gitignored test-results/r440-pc-categoryapp-write-fix/，非物理真机）

- evidence.json：source=Playwright Chromium headless 1440x900 against deployed test server; NOT a physical device。
- 关键截图 sha256（均 1440x900）：
  - 01-create-typeerror.png = 8de8663fc860bb2f0b8f31c0a123676d547e049cf6f651967806bdd8a629df43（红✗ application not found，行保留）
  - 03-create-success.png = 302a38c2084d044244f91491ea45184696fcd84d86ed0c4f206d0ef0e8d28107（成功态无成功 toast，靠 load 重拉）
  - 07-del-typeerror.png = c7a8be394ea1917ec290122bc8b392515b85b0f68c6fdff46f79be961d813f2d（红✗ category not found，行保留）
- 共 13 张截图 + evidence.json，真域名仅留 gitignored 证据目录，不回显、不入库。

## 环境说明

- 测试服地址、凭据均从部署环境变量 / .local.keys 注入，本文只写来源不写真值。
- 域名占位：<YOUR_TEST_DOMAIN> / <YOUR_TEST_HOST> / <HOST>，真值 0 命中。
