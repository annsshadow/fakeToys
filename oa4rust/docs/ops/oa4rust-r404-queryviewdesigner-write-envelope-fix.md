# R404 PC 查询视图设计器 saveView 写链误报成功修复

## 根因
QueryViewDesignerApp.vue saveView 在后端 HTTP 200 + type:error / success:false 失败信封下，await 响应后未判 type/success，无条件写 activeId/viewFlag、toast.success(视图已保存)、loadViews；simulate 因复用 saveView 连带误报成功。SDK 仅非 2xx 抛错，2xx 失败信封原样 resolve。

## 修复
新增 queryViewDesignerWriteSucceeded(resp:any):boolean：type:error 或 success:false 失败；type:success 或 type 缺省且 success 非 false 成功；非对象 null undefined 失败；不强求 bool。saveView 接响应守卫，失败 toast.error 透传 message 并 return，不写 id/viewFlag、不 loadViews、不弹成功；成功路径原样。simulate 复用连带修复；bundle 无成功 toast 不动。

## 验证
- 契约真执行注入桩：真父 2bc612aa6 RED 6 failed/10 passed → 修复版 16/16 GREEN。
- 根 vitest 全量：294 files / 2722 tests，0 failed。
- desktop tsc --noEmit：exit 0。

## 真浏览器实测
Playwright Chromium headless 1440×900 对已部署测试服真实页面注入对照测试，非物理真机。覆盖 type:error view not found / success:false / HTTP 403 / HTTP 500 / trueSuccess / simulate 连带；12 断言全 true，0 pageerror。证据位于被 gitignore 的 test-results/r404-pc-queryviewdesigner-write-fix/。测试服地址与凭据统一用 <YOUR_TEST_DOMAIN> / <YOUR_TEST_HOST> 占位，不写真值。

## 构建部署
PC 构建入口 index-BUqGiCnq.js，整目录 tar + scp + sha256 校验 + 远端原子替换部署，DEPLOY_R404_PC_FULL_OK。