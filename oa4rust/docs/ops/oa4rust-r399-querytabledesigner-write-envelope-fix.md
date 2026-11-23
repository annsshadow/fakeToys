# R399 PC QueryTableDesignerApp 三写链失败信封误报成功 P1 修复

## 根因
R398 甄别坐实 saveTable/editTableStructure/executeTable 三真实 UI 写在后端 HTTP200+type:error(table not found) 下不读信封 type，无条件弹绿色成功 toast；executeTable 虽读 response.data 赋 executeResult 但未判 type 同样误报。

## 修复
新增 queryTableWriteSucceeded(resp:any):boolean，type 为主不强求 bool：type:error 或 success:false 判失败；type:success 或 type 缺省且 success 非 false 判成功；非对象、null、undefined 判失败（本批端点无权威 bool，严格 bool 致假阴性）。三写函数 await 后先判响应再提示：失败 toast.error 透传后端 message、不弹成功、不刷新、不落态；saveTable 的 activeFlag=response.data?.tableFlag 赋值与 executeTable 的 executeResult=response.data 赋值均移入守卫成功分支内（失败态不赋值，契约专测）；成功才保留原 toast.success。只读 loadTables/newTable/addColumn/端点路径/payload/method/prompt/后端/SDK/其它页/migrations 全零改，git diff 确证仅三写链+helper 改动。本页三写无 confirm/prompt 守卫，无取消零写可测，据实说明。

## 验证
契约源码级提取真实函数体并用 new Function/new AsyncFn 真执行注入 api/toast/designerPaths/loadTables/activeFlag/executeResult 等桩，真父 2464f6ed9 RED 9 failed/6 passed 到修复版 15/15 GREEN；根 vitest 292 files/2691 tests 0 failed；desktop tsc --noEmit exit 0。PC 构建入口 index-Nz0o4sbf.js 整目录原子部署测试服。1440x900 headless Chromium 非物理真机注入对照 12 断言全 true、0 pageerror：save/edit/execute 三链 type:error table not found 与 success:false 均红 toast 透传 message 无误报成功，HTTP 403/500 正确错误，trueSuccess 保留绿成功且 executeResult 正确赋值。证据（gitignored）：test-results/r399-pc-querytabledesigner-write-fix/（evidence.json 与 9 张 1440x900 截图）。

## 环境占位
测试域名与主机使用 <YOUR_TEST_DOMAIN>/<YOUR_TEST_HOST> 占位，值从环境变量读取，不入库；本文件仅相对路径，截图来自非物理真机，如实标注。
