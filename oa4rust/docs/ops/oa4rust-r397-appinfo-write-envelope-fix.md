# R397 PC AppInfoApp writePerms 失败信封误报成功 P1 修复

## 根因
R396 甄别坐实设权限写链在后端 HTTP200+type:error(application not found) 下：writePerms await 不接响应、随后无条件 toast.success「应用权限已写入」，catch 仅覆盖非 2xx，误导用户。

## 修复
新增 appinfoWriteSucceeded(resp:any):boolean，type 为主不强求 bool：type:error 或 success:false 判失败；type:success 或 type 缺省且 success 非 false 判成功；非对象、null、undefined 判失败（后端成功 ActionResult::success 无权威 bool，严格 bool 会假阴性）。仅 writePerms 一个真实写链接响应并守卫：失败 toast.error 透传后端 message、不弹成功、不刷新、不落态；成功才保留原 toast.success。createFile/createApp/deleteApp/只读 load*/showPerms/loadIcon/confirm 守卫/catch 非 2xx/端点/payload/后端/SDK/其它页/migrations 全零改，git diff 确证仅 writePerms 判定改动。

## 验证
契约源码级提取真实函数体并用 new Function/new AsyncFn 真执行注入 api/toast/confirmMsg/prompt/refresh 桩，真父 21c10d7da RED 5 failed/6 passed 到修复版 11/11 GREEN；根 vitest 291 files/2676 tests 0 failed；desktop tsc --noEmit exit 0。PC 构建入口 index-Baozy39C.js 整目录原子部署测试服。1440x900 headless Chromium 非物理真机注入对照 9 断言全 true、0 pageerror：application not found 型 type:error 与 success:false 均红 toast 透传 message 无误报成功、HTTP 403/500 正确错误、trueSuccess 保留绿成功、confirm 取消零写。证据（gitignored）：test-results/r397-pc-appinfo-write-fix/（evidence.json 与 6 张 1440x900 截图）。

## 环境占位
测试域名与主机使用 <YOUR_TEST_DOMAIN>/<YOUR_TEST_HOST> 占位，值从环境变量读取，不入库；本文件仅相对路径，截图来自非物理真机，如实标注。
