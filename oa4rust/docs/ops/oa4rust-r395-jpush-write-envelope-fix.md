# R395 PC JPushApp 写链 200+type:error 误报成功 P1 修复

## 根因
R394 甄别坐实存推送、删推送、新版解绑、旧版解绑四个真实 UI 入口在写路径上裸 await 后无条件 toast.success：后端 HTTP200+type:error 业务失败时 SDK 不抛错，前端误报成功。

## 修复
新增 jpushWriteSucceeded(resp:any):boolean，type 为主不强求 bool：type:error 或 success:false 判失败；type:success 或 type 缺省且 success 非 false 判成功；非对象、null、undefined 判失败。jpushWrite 十分支以 lastResp 承接响应并在共享成功提示前统一守卫；jpushUnbindNew（GET 写语义）、jpushCtrlWrite、sendWxTest（在 accepted/queued 分支前守卫）、delDevice（失败不删本地行，成功才 filter）逐一守卫。失败 toast.error 透传后端 message 并 return，不弹成功、不刷新、不落态。端点路径、payload、method、prompt 取参、confirm 守卫、catch 非 2xx、只读加载零改；后端、SDK、其它页、migrations 零改。

## 验证
契约源码级提取真实函数体并用 new Function/new AsyncFn 真执行注入 api/toast/confirmMsg/prompt 桩，真父 a401f0fdc RED 18 failed/11 passed 到修复版 29/29 GREEN；根 vitest 290 files/2665 tests 0 failed；desktop tsc --noEmit exit 0。PC 构建入口 index-DDmeVjsJ.js 整目录原子部署测试服。1440x900 headless Chromium 非物理真机注入对照 12 断言全 true、0 pageerror：type:error/success:false 多组红 toast 透传后端 message 无误报成功、HTTP 403/500 正确错误、trueSuccess 成功提示保留、confirm 取消零写。证据（gitignored）：test-results/r395-pc-jpush-write-fix/（evidence.json 与 9 张截图）。

## 环境占位
测试域名与主机使用尖括号占位符 YOUR_TEST_DOMAIN 与 YOUR_TEST_HOST（文档正文写作 <YOUR_TEST_DOMAIN>/<YOUR_TEST_HOST>），值从环境变量读取，不入库；本文件仅相对路径，截图来自非物理真机，如实标注。
