# R367 PC 程序中心写操作链失败信封误报成功修复

> 轮次：R367（t180）。范围：PC /oa4rust/app/program 程序中心页 ProgramCenterApp.vue 写操作链。
> 依赖审计：R366（oa4rust/docs/ops/oa4rust-r366-pc-programcenter-write-audit.md），坐实 P1（真后端假 ID 探针 9 端点 200+type:error）。
> 环境口径：测试服域名/host/凭据仅运行期从 gitignored 脚本读取，本文档用 <YOUR_TEST_DOMAIN> / <YOUR_TEST_HOST> 占位，0 真值。
> 浏览器口径：Playwright Chromium headless 1440x900 against deployed test server; not a physical device（非物理真机）。

## 一、缺陷与根因

后端 program_center 写端点在“目标不存在/无权限”等失败场景返回 HTTP 200 + type:error + message（ActionResult::error，HTTP200 兼容 o2server 既有设计），SDK 对 2xx 不检查业务 type（全局口径，本轮不改）。ProgramCenterApp.vue 大量有真实 @click 入口的裸写函数 await api.get/post 后无条件 toast.success，导致操作实际未生效（如 agent not found）却提示成功，掩盖失败、误导运维。R366 审计真后端假 ID 探针坐实 9 端点 status200+type:error；真浏览器注入列表行后点禁用/执行，真响应 200+type:error 仍弹「已禁用」「已触发执行」。

## 二、方案裁决（type 判定为主，fail-loud）

只改前端业务写链判定，不改后端/SDK api.ts 传输层/其它页/migrations，不改端点路径与 payload、confirm 取消守卫与 catch 分支。

1. 新增本地纯判定 helper `writeSucceeded(resp)`：`resp.type === 'error'` 或 `resp.success === false` 判失败；`resp.type === 'success'`（含 type 缺省且 success 非 false）判成功。program_center 有约 288 处 `success(json!({...}))` 无专门权威布尔，故不照搬 R363 的严格 bool===true 口径，避免假阴性。
2. 新增 `guardedWrite(promise)`：await 业务请求后先过 writeSucceeded，失败则 `throw new Error(r?.message || '操作未生效，请刷新后重试')` 走既有 catch 的 toast.error；成功才透传响应给原 toast.success 流程。
3. 19 个真实入口写函数（toggleAgent/executeAgent/saveAgentEdit/saveDictData/saveScript/pcStyleErase/pcCollectDelete/pcInvokeSaveById/pcInvokeDeleteById/pcConfigSave/pcTokenThreshold/pcU3/U4Write/U5Write/U6/U7/U19、installMarket/uninstallMarket）共 63 处 await api 调用统一套 guardedWrite；5 处 catch 改为透传后端 message。
4. 非 2xx（403/500）仍由 SDK 抛错走 catch 正确 toast.error；端点路径与 payload 不变。
5. 不选传输层统一改法：SDK parseJsonEnvelope 对 2xx 原样 resolve，全局改风险面过大；局部业务判定最小且可测。

## 三、RED→GREEN 台账

- 契约：oa4rust/frontend/apps/desktop/src/views/ProgramCenterApp.writeEnvelope.contract.test.ts（源码级 bodyOf 提取 + 剥离 TS 标注 + new Function/new AsyncFn 真执行注入 api/toast/load* 桩，非文本断言）。
- RED：65 tests / 22 failed / 43 passed——helper 缺失、19 写函数未接守卫、type:error 误报成功均被捕获。
- GREEN：65/65 通过；根全量 vitest 278 files / 2304 tests 0 failed；desktop tsc --noEmit exit 0。

## 四、真浏览器实测（非物理真机）

构建 PC 生产包（入口 index-BVoxcqJy.js）整目录 tar 原子替换测试服 web 目录后，1440x900 headless 登录 → /app/program 注入一张 Agent 行 → 6 组写响应对照：

- type:error（禁用/执行）：均显示后端 message「后端拒绝：agent not found」，无成功文案。
- 403 / 500：各自 catch 显示「没有权限执行该操作」/「HTTP 500」，无成功文案。
- error 无 message：回退「操作未生效，请刷新后重试」，无成功文案。
- trueSuccess：显示「✓已禁用」成功文案。
- 9 项断言全 true，0 pageerror；截图与 evidence 落 gitignored test-results/r367-pc-programcenter-write-fix/。

## 五、边界与复核

破坏性真状态翻转/真删未授权不执行，用假 ID 真探针 + route 注入对照取证；changePassword/adminLogout 等敏感写仅验取消守卫未真执行。实测响应形状与本裁决不符需先报队长，不静默改向。证据截图标注 Playwright Chromium headless against deployed test server; not a physical device，非物理真机。

## 六、t182 复审续修（F1/F2/F3 全部纳入守卫，无豁免）

t181 复审指出三类遗漏，本轮按裁决全部纳入修复、无独立后续、无豁免：

1. F1（high）：createAgentM/deleteAgentM/deleteAppM/deleteScriptM/deleteDictM/createDictM/compareM/runScriptM/eraseImageM 九个 useMutation 在 onSuccess 入口新增 writeSucceeded 守卫：type:error 走 toast.error 透传后端 message 并 return，不落态、不刷新、不弹成功；type:success 保持原成功提示与 load* 刷新。mutationFn/端点/payload/onError 不变。
2. F2（medium）：pcOpenPlatform 的 tokenPut/thresholdPut/invokeTokenExec/moduleComparePost/moduleComparePut/avatarPut/avatarEraseGet 七分支统一 awaited guardedWrite 包裹，失败经既有 catch 的 toast.error 透传 message；端点路径/payload/confirm 取消守卫不变。同函数 changePassword/invokeExecGet/invokeTokenPost/adminLogout 原有 type 判定保留。
3. F3（low）：createInvoke/deleteInvoke 与 pcU10/pcU13/pcU15/pcU16/pcU17 的 await api 写调用统一套 guardedWrite；s() 的 catch 吞错 GET 探针（pcU11/pcU12/pcU14）属只读探测且刻意容错过网请求，不构成写成功误报，非有意豁免写链守卫。
4. 契约扩充到 111 it：F1 九例两态、F2 七例两态、F3 七例两态真执行；用 HEAD(t180) 版源码反向验证 26 failed / 85 passed（F1×9 + F2×7 + F3×7 全部 RED），修复版 111/111 GREEN。根全量 vitest 278 files / 2350 tests 0 failed；desktop tsc --noEmit exit 0。
