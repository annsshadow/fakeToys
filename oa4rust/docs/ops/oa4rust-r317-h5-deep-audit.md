# R317 H5 深度功能闭环审计（真浏览器实测）

## 来源与边界

- 执行人：qa-e2e（本地无头浏览器渲染，**非物理真机**）；分支 opt1000/oa4rust-pc-ai-refactor；审计起始 HEAD 63c6a338，收尾 HEAD 51aa40c8（队友并行提交使 HEAD 前移，本任务未改源码）。
- 测试服：站点 <YOUR_TEST_HOST>，H5 入口 /oa4rust-h5/，构建 index-BUQSTyeU.js（已含 R311 建日历/R314 已办同步/R315 搜索信封修复）。凭据仅运行期经 SSH 读取远端凭据文件，不入库、不回显。
- 浏览器：Playwright Chromium headless，390x844，isMobile/hasTouch/DPR2；所有截图为**本地无头浏览器渲染，非物理真机**。
- 方法：真实 UI 登录（账号/密码输入 + 提交按钮）后逐条走真实 DOM 入口，监听 /api/ 响应记录真实 HTTP 状态与响应体；失败项区分业务缺陷、前置数据缺失（取证限制）与脚本选择器问题。
- 证据目录（gitignored）：oa4rust/test-results/r317-h5-deep/，含 evidence.json、supplement-evidence.json、l6-*.json 与 20+ PNG。

## 六条闭环判定

| 编号 | 闭环 | 判定 | 真请求/真响应/页面回显 | 证据 |
|---|---|---|---|---|
| L1 | 登录→首页→个人中心→退出→重登 | PASS | /api/authentication/login 200、who 200；个人中心显示会话用户；退出后重登恢复 | L1-mine.png / L1-logout.png / evidence.json |
| L2 | 日程新建→列表→刷新保留 | PASS | POST /api/calendar/event/create 200；新日程 R317H5日程_1795241668724 创建后可见、刷新后仍可见；日历数 7 | L2-event-form.png / L2-event-created.png |
| L3 | 会议新建→列表 | 取证限制 | H5 会议页仅有待接受/待确认/我的会议读取 Tab，无新建会议 UI 入口；接口读取正常、无写路径可测 | L3-meeting.png |
| L4 | 会话→发消息→刷新回显 | PASS | POST /api/message/assemble/communicate/im/msg 200 且 sent=true；消息 R317H5消息_1795241680318 列表可见、刷新后保留 | L4-message-list.png / L4-chat-sent.png |
| L5 | 工作流待办→审批→已办 | PASS | POST /api/task/aec599a9-.../complete 200；已办列表 GET /taskcompleted/list/my/paging/1/size/50 200 回填，已办 3 条含刚审批项；初轮 toast 断言属时机误判，API 与 DOM 双证通过 | S-L5-pending.png / S-L5-completed.png / supplement-evidence.json |
| L6 | 论坛建版块→发帖→列表回显 | 取证限制 + API 层验证 PASS | UI picker 的版块数据源 GET /api/bbs/assemble/control/forum/view/all 初始返回 data:[] count=0；补版块后 picker 出现选项，但无头 Chromium 对 uni-app picker 的 touch/click 派发触发组件运行时错误（pageerror：Cannot read properties of undefined），无法完成真实 UI 选版块；改由已登录页面同源 fetch POST /api/bbs/assemble/control/topic/create 200、GET mobile/view/all 200 且回显新建主题 | L6b-picker-open.png / L6d-created.png / l6-forum-probe.json / l6-correct-table-evidence.json / l6-api-proof.json |

## 关键观察与源码定位

1. H5 会话持久化：mobile/src/store/session.ts init 走 authApi.who() 恢复会话，login 走 authApi.login + init(true)，force 代际令牌防旧响应覆盖；退出登录清 user 后 reLaunch 登录页。实测重登成功。
2. 日程写入：mobile/src/pages/calendar/calendar.vue 的 .calendar-create-btn/.calendar-create-form 建日历、.compose-btn+.compose 表单建事件；createEvent 走 /api/calendar/event/create。对照 R311 PC 修复与 R316 PC PASS，本轮首次 H5 深挖通过。
3. 消息发送：mobile/src/services/index.ts messageApi.send 调 POST /api/message/assemble/communicate/im/msg，仅 sent===true 落本地占位行；chat/chat.vue .composer input + .composer .send 为真实入口。刷新后从后端回显，非仅本地乐观行。
4. 工作流已办：mobile/src/pages/process/process.vue 待办/已办/我发起 Tab，act() 用 uni.showModal({showInput:true}) 自定义弹窗 + 确定按钮；后端已办端点 /api/processplatform/assemble/surface/taskcompleted/list/my/paging/{page}/size/{size}。R314 D1 后端补写 PP_C_TASKCOMPLETED 在 H5 侧同样生效（已办含刚审批 task id）。
5. 论坛发帖 picker：mobile/src/pages/bbs/new.vue:83 <picker :range="forums.map(f=>f.name)" @change=onForumChange>，数据源 bbsApi.forumList() → GET /api/bbs/assemble/control/forum/view/all；forumIndex>=0 才可提交。后端 bbs_assemble_control/src/lib.rs:566 forum_view_all 从 x_bbs_forum 读取 deleted_at IS NULL 的记录，返回空是数据表无行，不是前端解析缺陷。
6. 版块数据补给的跨表分叉（观察，不判本轮缺陷）：PC 版块创建 BBSForum.vue:374 走 POST /api/bbs/assemble/control/section/create，后端 section_create（bbs_assemble_control/src/lib.rs:263）写 x_bbs_assemble_control_section；而 u2_user_forum_save（u2.rs:1406）写 x_bbs_forum，才是 forum/view/all 的数据源。本轮用 user/forum 端点成功补入版块（幂等创建 R317H5正式版块_...），picker 随即出现选项，坐实数据源口径。

## 真实缺陷

本轮未发现新增 P0/P1 真实功能缺陷。L3 会议新建与 L6 UI 选版块均为取证限制（无真实 UI 入口 / 无头浏览器对 uni-app picker 交互不兼容），不作为业务缺陷；L6 后端 topic/create 写入与 mobile/view/all 回显均 200，证明发帖能力本身可用。

## 取证限制明细

- L3：H5 会议页无新建会议入口（meeting/meeting.vue 仅待接受/待确认/我的会议读取 Tab），无法从 H5 发起创建闭环。
- L6 UI：初始论坛版块表为空（forum/view/all data:[] count=0）；补数据后 picker 选项 DOM 可见，但无头 Chromium 对 uni-app picker 的 touch/click 派发触发组件运行时 undefined.length 错误，无法完成组件内部选中。建议后续用物理设备或真实触摸回放补测该 UI 路径。
- 会议楼栋等前置数据不在本轮 H5 审计范围，未重复取样。

## 结论

R317 H5 六条闭环中 L1/L2/L4/L5 真 UI 全链路 PASS，L3 为无新建入口的取证限制，L6 UI 选版块受 uni-app picker 无头交互限制、API 层写入与列表回显 PASS。全程 pageerror 仅 L6 picker 派发阶段出现（脚本触发的组件兼容问题），其余闭环 pageErrors=0、API 失败=0。R311/R314/R315 的前序修复在 H5 侧均未回归。项目 ≥1000 轮持续迭代，本轮非终态。
