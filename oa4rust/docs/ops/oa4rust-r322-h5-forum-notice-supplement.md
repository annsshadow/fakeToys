# R322 H5 论坛回帖与通知已读补测审计

## 来源与边界

- 执行人：qa-e2e（本地无头浏览器渲染，**非物理真机**）；分支 opt1000/oa4rust-pc-ai-refactor；审计起始 HEAD 56ff8bf0db63046f512ebc706de7135ad57488f5，本轮为只读补测审计，未改源码/契约/配置/后端。
- 测试服：站点 <YOUR_TEST_DOMAIN>，H5 入口 /oa4rust-h5/，构建 index-BUQSTyeU.js（沿用既有产物，本轮未重新 build/部署）。凭据仅运行期经 SSH 读取远端凭据文件，不入库、不回显。
- 浏览器：Playwright Chromium headless，390x844，isMobile/hasTouch/DPR2；所有截图为**本地无头浏览器渲染，非物理真机**。
- 方法：真实 UI 登录（账号/密码输入 + 提交按钮）后逐条走真实 DOM 入口，监听 /oa4rust-api/api/ 响应记录真实 HTTP 状态与响应体；H5 fetch 统一前缀 /oa4rust-api（源码 oa4rust/mobile/src/http.ts:25）。已读状态另以数据库只读查询补证（运行期读后端 .env 的 DATABASE_URL，不回显主机细节）。失败项严格区分真缺陷、取证限制（无 UI 入口/无头 picker 不兼容）、产品语义差异与脚本选择器问题。
- 对照：R317 H5 审计（L1/L2/L4/L5 PASS，L3 无新建会议入口，L6 论坛 picker 无头限制）、R321 H5 第二轮（论坛回帖 0→1 持久 PASS 但为首次建帖路径），本轮聚焦回帖闭环与通知已读流转，避免重复。
- 证据目录（gitignored）：oa4rust/test-results/r322-h5-supplement/，含 evidence.json 与 6 张 PNG。

## 补测闭环判定表

| 编号 | 闭环 | 判定 | 真请求/真响应/页面回显 | 证据 |
|---|---|---|---|---|
| F1-list | 论坛版块主题列表 | PASS | GET /oa4rust-api/api/bbs/assemble/control/mobile/view/all 200，listCount=20，样本主题 R322闭环_176861 | F1-bbs-list-before.png / evidence.json |
| F1-reply | 进主题→回帖→刷新持久 | PASS | GET subject/view/{id} 200；POST reply/create 200；GET reply/list/sub/{topicId} 回帖数 1→2，刷新后仍 2、foundReload=true | F1-topic-before.png / F1-topic-after-send.png / F1-topic-after-reload.png / evidence.json |
| N1-announce | 通知公告已读流转 | 取证限制（非缺陷） | GET ann/list 200 但响应 data 为空（count=0），当前账号无公告，无已读对象可走 | N1-announce-dialog.png / evidence.json |
| N2-conv-list | 会话列表 | PASS | GET /oa4rust-api/api/message/assemble/communicate/im/conversation/list/my 200，convCount=16 | evidence.json |
| N2-markread | 进入会话已读流转 | PASS | 进入会话自动触发 POST .../im/conversation/{id}/read 200（readCalls=1）；DB 只读查该会话 read_status=read、read_time=2026-11-21 22:10:20.770887+08 | N2-chat-entered.png / evidence.json / r322-readstatus-probe.py 输出 |

总计 loops=5，pass=4，limit=1，errors=0，api=27，passAll=true（可判定项全通过）。

## 逐条四件套

### F1 论坛回帖闭环（真 UI）

- 真请求：GET /oa4rust-api/api/bbs/assemble/control/mobile/view/all → 200；点主题 .topic 进详情触发 GET subject/view/{id} 200；在 .composer input（placeholder 写下你的回帖）填 stamp=R322H5回帖_1795270215098，点 .composer-send（@tap=send）触发 POST reply/create 200；刷新后 GET reply/list/sub/8eb22714-4499-41dc-8553-5207c379152e 200。
- 真响应：回帖数 cntBefore=1 → cntAfter=2，found=true；刷新后 cntReload=2、foundReload=true；topicId=8eb22714-4499-41dc-8553-5207c379152e。
- 页面回显：F1-topic-before.png 目视回帖（1）含既有回帖；F1-topic-after-reload.png 目视回帖（2）且新增 stamp 回帖刷新后仍在。
- 源码定位：mobile/src/pages/bbs/bbs.vue:103（.topic tap 打开主题）、topic.vue:112-117（.composer input + .composer-send @tap=send）、mobile/src/services/index.ts:526-541（mobileViewAll/subjectView/replyList=reply/list/sub/{topicId}/replyCreate）；后端 reply/create 真实写表。

### N2 通知已读流转（真 UI + DB 补证）

- 真请求：GET im/conversation/list/my 200；进入会话 chat/chat.vue loadHistory finally 自动 POST im/conversation/0c5a460c-ec28-40a3-96fe-1749c3199153/read 200（以 apiBaseline 基线只统计新增 read 请求，readCalls=1）。
- 真响应：read 端点 200；会话列表响应体只含 createTime/id/lastMessage/name/type，不含 read_status 字段，故前端响应不足以判定持久化。
- 页面回显：N2-chat-entered.png 目视会话页（标题 R322已读验证）+ 发消息框，markRead 已触发。
- DB 补证：只读查 x_message_conversation where id=0c5a460c-... 返回 [id, read, 2026-11-21 22:10:20.770887+08]——read_status=read 且 read_time 与实测时点一致，坐实持久化（非仅 HTTP 200）。
- 源码定位：mobile/src/services/index.ts:191-192（markRead POST .../im/conversation/{id}/read）、chat/chat.vue:176-177（loadHistory finally 自动 markConversationRead）、后端 message_assemble_communicate/src/lib.rs:1107（校验会话成员后 UPDATE x_message_conversation SET read_status=read, read_time=NOW()）、routes.rs:79-80（POST+PUT 注册）。

### N1 通知公告（取证限制）

- 真请求：GET /oa4rust-api/api/ai/assemble/control/ann/list 200。
- 真响应：data 为空、count=0；noticeDomCount=0、dialogShown=false。
- 判定：当前账号无公告数据，无已读对象可走，属取证限制非业务缺陷。公告 annApi 仅本地弹窗展示（index/index.vue:119 @tap=viewNotice=n、:178-185 .notice-mask），本身无已读持久化语义；真正的已读持久化入口是上述 IM markRead。

## 取证限制（不计业务缺陷）

1. N1 通知公告：当前账号 ann/list 为空（count=0），无公告可读，无法走公告已读流转；公告本身亦无已读持久化语义，已读流转由 IM markRead 覆盖并已坐实。
2. 论坛发帖 picker：R317 已记录 uni-app picker 在无头 Chromium 无法可靠派发 touch 选项选择（组件内部 undefined.length），本轮不重复攻坚发帖，仅就既有主题走回帖闭环（发帖写表已在 R317 由 API 层验证）。

## 真实缺陷

- 本轮补测未发现新增真实 P0/P1 缺陷。论坛回帖闭环经真 UI + 真响应 + 刷新回显完全坐实；通知已读经真请求 200 + DB 只读状态补证完全坐实。N1 为空数据取证限制，非缺陷。

## 证据索引

- 截图（本地无头浏览器渲染，非物理真机）：F1-bbs-list-before.png、F1-topic-before.png、F1-topic-after-send.png、F1-topic-after-reload.png、N1-announce-dialog.png、N2-chat-entered.png。
- 数据：evidence.json（F1/N1/N2 全量真请求真响应）；分析脚本 r322-analyze.cjs、r322-conv-check.cjs。
- DB 补证脚本：r322-readstatus-probe.py（运行期读 .env，输出仅 read_status 三元组，无主机/口令回显）。
- 全部证据在 gitignored 目录 oa4rust/test-results/r322-h5-supplement/ 与 oa4rust/test-results/ 下，不入库。

## 结论

R322 补测：H5 论坛回帖闭环（进版块→主题→回帖→刷新持久）与通知已读流转（进入会话自动 markRead + DB read_status=read 持久化）均以真 UI + 真请求/真响应/页面回显验证 PASS；通知公告因当前账号无数据为取证限制，非缺陷。未发现新增真实 P0/P1。项目持续迭代非终态。