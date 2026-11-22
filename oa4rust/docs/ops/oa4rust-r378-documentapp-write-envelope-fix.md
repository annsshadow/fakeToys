# R378 PC 文档管理写链误报成功 P1 修复

## 问题
DocumentApp.vue 中多个真实 @click 写函数（推荐 / 取消推荐 / 置顶 / 取消置顶 / 发布 / 撤发 / 密文发布 / 通知 / 记录浏览 / 删除 / 新建）在后端返回 HTTP 200 + `type:error` 信封时，仍无条件 `toast.success`，造成误报成功。后端 `ActionResult::error` 仍是 HTTP 200，SDK 传输层只按 `resp.ok` 抛错，不检查 `type`，前端必须自行判定业务结果。

## 修复
- 新增 helper `documentWriteSucceeded(resp:any):boolean`：`type==='error'` 或 `success===false` 判失败；`type==='success'` 或 type 缺省且 success 非 false 判成功；**不做 flagKey 严格 true 校验**（`isTop:false` 取消置顶、`removed:0` 取消推荐都是合法成功）。
- 11 个真实写函数 await 后端响应后先判 helper：失败则 `toast.error(r?.message||'操作未生效，请刷新后重试')` 并 return，不刷新、不落态、不关弹窗；成功才保留原成功提示和副作用。
- 端点路径 / payload / method / confirm 取消守卫 / catch 非 2xx 分支 / 只读加载函数全部保持原样。

## 验证
- 契约 `DocumentApp.writeEnvelope.contract.test.ts` 源码级提取 + stripTs + new Function 真执行注入 api/toast/doSearch/confirmMsg/prompt 桩：父版 RED 14 failed / 17 passed，修复版 31/31 GREEN。
- 根 vitest 283 files / 2492 tests 全绿；desktop tsc --noEmit exit 0。
- PC 构建产物 `index-B9R68tNQ.js` 整目录原子部署测试服。
- 真浏览器对照（Playwright Chromium headless 1440x900，**非物理真机**）：注入 4 组写端点（recommend/top/publish/notify）type:error → 红 toast 透传后端 message、无成功提示；HTTP 500/403 → 正确错误 toast；type:success → 原成功提示仍生效；删除 confirm 取消零写；0 pageerror。证据见 `test-results/r378-pc-documentapp-write-fix/evidence.json` 与 2 张 1440x900 截图。

## 部署与访问
- 部署脚本：`test-results/deploy-pc-r378.sh`（从仓库相对路径执行）。
- 测试服地址从部署环境注入，不在本文件记录真值，统一用 `<YOUR_TEST_DOMAIN>` / `<YOUR_TEST_HOST>` 占位。
