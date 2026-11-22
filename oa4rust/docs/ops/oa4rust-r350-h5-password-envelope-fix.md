# R350 H5 我的页改密误报成功修复

> 环境：测试服 <YOUR_TEST_DOMAIN> / <YOUR_TEST_HOST> 均为占位符，0 真值入库。真浏览器证据为 Playwright Chromium headless 390x844 isMobile/hasTouch/DPR2 对已部署测试服的真实渲染，非物理真机。

## 1. 范围与根因

- 页面：oa4rust/mobile/src/pages/mine/mine.vue:62-85 changePassword()；服务：oa4rust/mobile/src/services/index.ts:39-40 changePassword。
- R349 审计与真浏览器双重确证：错误旧密码 PUT /api/person/password → HTTP 200 + 业务错误信封 {type:error, message:old password mismatch}；页面却弹「密码修改成功」并清空表单，用户被误导为已改密。
- 三重链路：mine.vue:78 无条件成功 toast；index.ts:40 discardResponse:true 吞掉响应体；http.ts:186-192 HTTP 2xx 一律 resolve 不检查业务信封。

## 2. 方案 B 裁决（fail-loud）

- 采用最小化前端判定：index.ts changePassword 去掉 discardResponse:true，让 http.ts 的 parseEnvelope 返回完整响应体；mine.vue 收到响应后先判 resp?.type === error 或 resp?.success === false，显示后端 message（缺省「修改失败」）并 return，不清表单不弹成功；仅明确成功才弹成功 toast + 清空表单。
- 不改传输层 http.ts：login/logout/refresh 等十余处 discardResponse:true 调用点仍保留，本轮只消除 H5 mine 改密一处误报。其它调用点若存在同类 2xx+error 信封误判，需在各自页面层单独判定或另立传输层统一治理任务——本轮未覆盖，fail-loud 记录，不静默声称全站修复。
- 硬边界遵守：未改 http.ts/后端/SDK/desktop/其它页/migrations；端点路径与 payload 未变；validateNewPassword 预校验与两次密码一致性逻辑未动；破坏性真改密未执行（错误旧密码取证）。

## 3. 修复内容

- mobile/src/pages/mine/mine.vue:74-81：const resp: any = await authApi.changePassword(...) 后 if (resp?.type === error || resp?.success === false) 显示后端 message 并 return；成功后才 showToast 密码修改成功 + 清表单。
- mobile/src/services/index.ts:39-40：mapi.put(/api/person/password, data) 去 discardResponse:true。
- 契约 mobile/src/pages/mine/mine.password.envelope.contract.test.ts（新建）：源码级 bodyOf 花括号配对提取 changePassword 函数体，replace 剥离 TS 标注（const resp: any / catch (e: unknown) / : boolean），new AsyncFn 真执行注入 uni/authApi/validate 桩；5 it 覆盖 index 无 discardResponse、type:error 与 success:false 显示后端 message 且表单保留、明确成功弹成功清表单、可达链保护。自定义函数参数均带冒号 string 标注防 TS7006。

## 4. RED → GREEN 台账

- RED：pnpm exec vitest run src/pages/mine/mine.password.envelope.contract.test.ts exit=1，3 failed / 2 passed（index 仍 discardResponse、错误信封弹成功且清表单）；首轮路径错误 ENOENT 与 Unexpected token : / Missing initializer in const declaration 均由契约路径与 TS 标注剥离修正，非业务通过。
- GREEN：同命令 exit=0，5/5 passed。
- 全量：cd oa4rust && pnpm exec vitest run exit=0，72 files / 463 tests 全 passed（含既有 shape.test.ts / password.test.ts 无回归）。

## 5. 真浏览器复验（错误旧密码分支）

- 脚本 test-results/r350-h5-password-envelope-check.cjs（gitignored）：登录 → H5 我的页 → 三格填错旧密码 → 提交 → 捕获 PUT 响应 body → 25 次乘 100ms 高频轮询 toast DOM 文本 → 断言表单保留。
- 8 项全 true：pageLoaded / passwordHttp200 / businessErrorEnvelope / noFalseSuccess / errorMessageShown / formPreserved / noPageErrors / noApiFailures；failed=[]。
- 实测响应：HTTP 200，body {type:error, message:old password mismatch, prompt:com.x.base.core.project.exception.ExceptionEntityNotExist}；toast DOM 采样 old password mismatch；表单 old/new 均保留错误旧密码与新密码。
- 截图 test-results/r350-h5-password-envelope/01-wrong-password-error-toast.png（390x844，headless 非物理真机，取证见同目录 evidence.json）；evidence.json source 明标 NOT physical device。

## 6. 取证限制

- 未执行破坏性真改密；仅错误旧密码分支。明确成功分支由契约真执行覆盖（GREEN 5/5），未在真浏览器用真实口令验证。
- uni.showToast 约 2s 自动消失，首轮采样漏掉文案；脚本已改高频轮询并采得错误 toast 文本，硬断言成立。
