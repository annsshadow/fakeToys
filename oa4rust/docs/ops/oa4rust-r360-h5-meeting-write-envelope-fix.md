# R360 H5 会议五写端点静默失败误报成功修复

> 环境：测试服 <YOUR_TEST_DOMAIN> / <YOUR_TEST_HOST> 均为占位符，0 真值入库。真浏览器证据为 Playwright Chromium headless 390x844 isMobile/hasTouch/DPR2 对已部署测试服的真实渲染，非物理真机。

## 1. 范围与根因

- 页面：oa4rust/mobile/src/pages/meeting/meeting.vue:102-126 act() 与五个包装调用；服务：oa4rust/mobile/src/services/index.ts:403-413 五个写端点。
- R359 审计（commit 8dd2575b9，文档 oa4rust/docs/ops/oa4rust-r359-h5-meeting-write-audit.md）与真浏览器双重确证：假 ID 调用 accept/reject/confirm/deny/checkin 得到 HTTP 200 + {type:success, data:{accepted/rejected/confirmed/denied/checkedIn:false, id}}，页面仍无条件弹「已接受/已拒绝/已确认出席/已确认缺席/签到成功」；200 + type:error 同样误报。
- 三重链路：meeting.vue:107 原 await fn() 后无条件成功 toast；index.ts 五端点 discardResponse:true 吞掉响应体；http.ts:186-191 HTTP 2xx 一律 resolve 且 parseEnvelope(:67-77) 不查 type/success。后端 UPDATE 0 行仍 200 成功信封、owner_gate 对不存在行放行（R359 判定为既有设计，本轮不改）。

## 2. 方案 B 裁决（fail-loud，与 R350 改密同型先例）

- 采用最小化前端判定：index.ts 五写端点去掉 discardResponse:true，让 http.ts 的 parseEnvelope 返回完整业务信封；meeting.vue act() 增加第 4 参 flagKey，await fn() 后读取信封，仅当 (type==='success' 或 type 缺省且 success 非 false) 且 data[flagKey] 或根[flagKey] 严格等于 true 时弹 okText 成功 toast + load() 刷新；否则弹错误 toast，优先显示后端 resp.message，缺省「操作未生效，请刷新后重试」；catch 分支维持非 2xx 错误提示，busyId 并发守卫与列表回显不变。
- 不改传输层 http.ts：login/logout/refresh 等十余处 discardResponse:true 调用点仍保留，本轮只消除 H5 meeting 五写操作误报；其它调用点若存在同类 2xx+error 信封误判，需在各自页面层单独判定或另立传输层统一治理任务——本轮未覆盖，fail-loud 记录，不静默声称全站修复。
- 硬边界遵守：未改 http.ts/后端/SDK/desktop/其它页/migrations；五端点路径与 payload 未变；不执行破坏性真状态翻转/签到 INSERT，真浏览器仅假 ID 路由注入取证；实测响应形状与裁决一致（bool 在 resp.data[flagKey]，403/500 走 catch），未触发上报队长改向。

## 3. 修复内容

- mobile/src/pages/meeting/meeting.vue:102-126：act 签名改为 act(id: string, fn: () => Promise<unknown>, okText: string, flagKey: string)；新增 typeOk/flag 判定，成功才 toast+load，否则错误 toast；五个调用点分别传 'accepted'/'rejected'/'confirmed'/'denied'/'checkedIn'。
- mobile/src/services/index.ts:403-413：accept/reject/confirmAllow/confirmDeny/checkin 五端点去掉 discardResponse:true，均保留原路径与 null payload。
- 契约 mobile/src/pages/meeting/meeting.write.envelope.contract.test.ts（新建）：源码级 bodyOf 花括号配对提取 act 函数体，replace 剥离 TS 标注，new AsyncFn 真执行注入 uni/busyId/load/fn 桩；10 it 覆盖 index 无 discardResponse、五调用点 flagKey、bool:true 成功+刷新、bool:false 缺省错误文案、type:error 显示 message、success:false、type 缺省、平铺 bool、非 2xx catch、busyId 拦截；自定义函数参数均带冒号 string 标注防 TS7006。

## 4. RED → GREEN 台账

- RED：mobile vitest 单文件 exit=1，5 failed / 4 passed（index 仍 discardResponse、五调用点缺 flagKey、bool:false/type:error/success:false 仍误报成功）。
- GREEN：同命令 exit=0，10/10 passed。
- 全量：cd oa4rust/mobile && pnpm exec vitest run exit=0，75 files / 486 tests 全 passed；cd oa4rust && pnpm exec vitest run exit=0，276 files / 2210 tests 全 passed；cd oa4rust/mobile && pnpm exec tsc --noEmit exit=0。

## 5. 真浏览器复验（注入对照，非物理真机）

- H5 pnpm exec uni build exit=0，入口 index-BfKhwmJj.js；脚本 test-results/deploy-h5-r360.sh 整目录 tar 原子替换测试服 h5 目录，DEPLOY_R360_H5_OK。
- 脚本 test-results/r360-h5-meeting-write-check.cjs（gitignored）：390x844 headless 登录 → 会议页注入 1 条待办会议 → 依次触发 falseSuccess×5/typeError/403/500/trueSuccess 九次写操作，25 次乘 100ms 高频轮询 toast DOM，记录每个 POST 的真实 HTTP 状态。
- 9 项断言全 true：loginOk / cardsInjected / allClicked / falseSuccessNoFalseReport / typeErrorShowsBackendMsg / http403NoFalseReport / http500NoFalseReport / trueSuccessShowsSuccess / noPageErrors；failed=[]。实测九 POST 全部真实发出且状态分别为 200×7 + 403 + 500；bool:false 五次均显示「操作未生效，请刷新后重试」无成功 toast，type:error 显示后端 message，trueSuccess 显示「已接受」，403/500 显示既有 catch 错误文案，errors=[]。
- 截图 test-results/r360-h5-meeting-write-fix/meeting-write-fixed.png 与 evidence.json（source 明标 NOT physical device，gitignored）；首轮脚本轮询内删除 toast DOM 破坏 uni-app toast 单例致后续采样为空，已改为只采集不删除后复验全绿，取证脚本问题已如实记录。

## 6. 取证限制

- 未执行破坏性真状态翻转/签到 INSERT（未授权）；假 ID 路由注入仅验证信封判定与提示，不涉及真实业务数据变更。
- 后端 owner_gate 对不存在行放行与 UPDATE 0 行仍返回成功信封属既有设计，R359 已判定产品语义差异，本轮前端判定只保证不误报，不替代后端幂等/状态码治理。
