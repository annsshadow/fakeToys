# R363 PC 会议管理写操作链失败信封误报成功修复

> 轮次：R363（t175）。范围：PC /oa4rust/app/meeting 会议管理页 MeetingApp.vue 五写端点。
> 依赖审计：R362（oa4rust/docs/ops/oa4rust-r362-pc-meeting-write-audit.md），坐实 D1 P1。
> 环境口径：测试服域名/host/凭据仅运行期从 gitignored 脚本读取，本文档用 <YOUR_TEST_DOMAIN> 占位，0 真值。
> 浏览器口径：Playwright Chromium headless 1440x900 against deployed test server; not a physical device（非物理真机）。

## 一、缺陷与根因

后端 meeting_assemble_control 状态翻转端点在“目标行不存在”场景下仍返回 HTTP 200 + type:success + 权威布尔 false（accept/reject/confirm_allow/manual_completed/modify_starttime 均实测可达；owner_gate 对不存在行放行、UPDATE 不查受影响行数）。R362 审计中 PC 五写函数 await api.post 后无条件弹成功 toast 并刷新，导致 200+success+bool:false 与 200+type:error 两类失败信封均误报成功；markMeetingCompleted 更出现“状态仍为未开始但提示已标记完成”的状态/提示矛盾。SDK 对 2xx 不检查 type:error，为全局口径，本轮不改。

## 二、方案 B 裁决（fail-loud，对齐 R350/R360 先例）

只改前端业务写链判定，不改 SDK api.ts / http.ts 传输层、不改后端/数据库/其它页/migrations、不改端点路径与 payload。

1. 新增本地纯判定 helper \`meetingWriteSucceeded(resp, flagKey)\`：仅当 \`resp.type === 'success'\`（或 type 缺省且 success 非 false）且 \`resp.data[flagKey] ?? resp[flagKey] === true\` 才判成功。
2. 五个写函数（joinMeeting/approveMeeting/leaveMeeting/modifyMeetingTime/markMeetingCompleted）await api.post 接住响应 r 并传各自 flagKey：accepted / confirmed / rejected / modified / completed。
3. 判定不通过时 \`toast.error(r?.message || '操作未生效，请刷新后重试')\`，不刷新、不落态；成功才弹原成功文案并 loadMeetings 刷新。markMeetingCompleted 的 completed:false 不再落态也不弹成功，消除状态/提示矛盾。
4. 非 2xx 仍由 SDK 抛错走各自 catch；confirmMsg 取消守卫、prompt 起止时间、端点路径与 payload 全部不变。
5. 不选传输层统一改法：discardResponse 与 2xx resolve 口径影响十余既有调用点，风险面过大；局部业务判定最小且可测。

## 三、RED→GREEN 台账

- 契约：oa4rust/frontend/apps/desktop/src/views/MeetingApp.meetingWrite.contract.test.ts（源码级 bodyOf 提取 + 剥离 TS 标注 + new Function/new AsyncFn 真执行注入 api/toast/loadMeetings/prompt/confirmMsg 桩）。
- RED：29 tests / 16 failed / 13 passed——helper 成功判定、五函数未调用 helper、bool:false 与 type:error 误报、completed:false 错误落态均被捕获。
- GREEN：29/29 通过；根全量 vitest 277 files / 2239 tests 0 failed；desktop tsc --noEmit exit 0。

## 四、真浏览器实测（非物理真机）

构建 PC 生产包整目录 tar 原子替换测试服 web 目录后，1440x900 headless 登录 → /app/meeting 注入一张会议卡 → 逐项点击注入响应对照：

- bool:false × 3（加入/改时间/标记完成）：均提示“操作未生效，请刷新后重试”，无成功文案。
- type:error：显示后端 message“后端拒绝：业务失败”，无成功文案。
- 403 / 500：各自 catch 显示“没有权限执行该操作”/“HTTP 500”，无成功文案。
- trueSuccess：显示“已加入会议”成功文案。
- 9 项断言全 true，0 pageerror；截图与 evidence 落 gitignored test-results/r363-pc-meeting-write-fix/。

## 五、边界与复核

破坏性真状态翻转/真改时间未授权不执行；失败信封由假 ID 真探针 + route 注入对照取证。实测响应形状与本裁决不符需先报队长，不静默改向。证据截图标注 Playwright Chromium headless against deployed test server; not a physical device，非物理真机。
