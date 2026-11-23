# R408 PC 门户页面设计器 layoutOp 删除写链误报成功修复

## 根因
PortalPageDesignerApp.vue layoutOp 删除分支（POST/DELETE /api/portal/assemble/surface/delete/layout）在 await 响应后无条件 toast.success(布局已删除) 不读 type/success，catch 仅非 2xx；后端 delete_layout DELETE 0 行 → ActionResult::error(layout not found) HTTP 200 + type:error，仍误报成功。SDK 仅非 2xx 抛错，2xx 失败信封原样 resolve。

## 修复
新增 portalLayoutWriteSucceeded(resp:any):boolean：type:error 或 success:false 失败；type:success 或 type 缺省且 success 非 false 成功；非对象/null/undefined 失败；不强求 bool（后端成功 data.deleted 无权威 bool，严格 bool 致假阴性）。删除分支接住 api.post/api.delete 响应到 lastResp，守卫失败 toast.error 透传 resp.message 并 return，不弹成功；成功才 toast.success。保存分支真后端 save_layout 恒 success 无误报前提本轮不改（仅 L1 记录）。confirm 取消/prompt 空 ID 零写零 toast 保持。

## 验证
- 契约真执行注入桩：真父 c34a8b883 RED 7 failed/11 passed → 修复版 18/18 GREEN。
- 根 vitest 全量：296 files / 2755 tests，0 failed。
- desktop tsc --noEmit：exit 0。

## 真浏览器实测
Playwright Chromium headless 1440×900 对已部署测试服真实页面注入对照测试，非物理真机。覆盖 POST/DELETE 两删除入口 type:error layout not found、success:false、HTTP 403/500、trueSuccess、confirm 取消零写；13 断言全 true，0 pageerror。证据位于被 gitignore 的 test-results/r408-pc-portalpage-layout-write-fix/。测试服地址与凭据统一用 <YOUR_TEST_DOMAIN> / <YOUR_TEST_HOST> 占位，不写真值。

## 构建部署
PC 构建入口 index-BGyhXpmQ.js，整目录 tar + scp + sha256 校验 + 远端原子替换部署，DEPLOY_R408_PC_FULL_OK。