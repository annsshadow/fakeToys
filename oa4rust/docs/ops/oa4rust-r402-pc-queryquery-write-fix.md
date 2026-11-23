# R402 PC 查询定义表面写链误报成功修复

## 根因
QueryQueryApp.vue 的 saveSurface / deleteSurface 在后端 HTTP 200 + type:error（query surface not found）时裸 await 后无条件 toast.success；deleteSurface 还先本地 filter 删行再提示成功，导致失败被误报且本地行丢失。

## 修复
新增 querySurfaceWriteSucceeded(resp:any):boolean：非对象/type:error/success:false/未知 type 返回 false；type:success 或 type 缺省且 success 非 false 返回 true（不强求 bool，后端 save=saved、delete=updated 无统一 bool）。两写链成功提示前统一守卫；失败 toast.error 透传后端 message 并 return，不弹成功、不刷新、不关弹窗；deleteSurface 仅成功后才 filter 删行，失败保留行。

## 验证
- test-first RED→GREEN：真父 981c612439e2a551e14d5ea5c2e02ab7f091fe1a 契约 7 failed/8 passed → 修复版 15/15 GREEN。
- 根 vitest 全量 293 files / 2706 tests 0 failed；desktop tsc --noEmit exit 0。
- PC 构建入口 index-DPZh502O.js，整目录原子部署到测试服。
- 测试服地址：<YOUR_TEST_HOST> / <YOUR_TEST_DOMAIN>（仅占位，真值从部署环境注入，不入库）。
- 真浏览器 Playwright Chromium headless 1440x900（非物理真机，against deployed test server）注入对照：15 断言全 true、0 pageerror；失败 toast 透传且行保留、取消零写。证据在 gitignored test-results/r402-pc-queryquery-write-fix/。

## 写域
QueryQueryApp.vue、QueryQueryApp.writeEnvelope.contract.test.ts、本文档；端点/payload/只读加载/后端/SDK/其它页面零改。
