# R322 PC 论坛回复闭环与通知已读补测审计

## 来源与边界
- 任务：t108 [R322 PC 论坛回复闭环与通知已读补测]，kind=work（只读审计，禁改源码/契约/配置/后端）。
- 审计人/身份：page-core；时间：2026-11-21。分支：opt1000/oa4rust-pc-ai-refactor（禁 main）；起始 HEAD：98afb6e8d。
- 测试入口：PC /oa4rust/app/，Playwright Chromium headless 1440x900；**本地无头浏览器渲染，非物理真机**。
- 登录：真实 UI 登录（占位请输入用户名/请输入密码/按钮登 录），凭据由 ssh 运行期读 extract-remote-creds.py，不回显不入库。
- 域名占位：〈YOUR_TEST_DOMAIN〉、〈YOUR_TEST_HOST〉，0 真值（无域名/IP/口令/会话数据/业务真值入库）。
- 方法：每条判定走真 UI 流程 + 真请求/响应捕获 + 真 DOM 回显 + 源码定位四件套；分真缺陷 / 取证限制 / 产品语义差异，不臆造，不重复报已闭环项。
- 唯一入库物：本文档；证据脚本与截图落 gitignored oa4rust/test-results/r322-forum-final/ 与 r322-net-cap.cjs。

## 一、真缺陷（新增 1 条）

### D1 [P2] 论坛主题详情回复已落库却完全不渲染（双层信封只解一层）
- 复现路径：测试服登录 → 侧栏「沟通 > 论坛」→「我的」tab → 打开任一有回复的主题（本轮用真实 UI 新建主题 R322闭环_176861）→ 详情页「回复 (n)」始终显示「回复 ()」，回复卡片为空白，正文/作者/时间全不显示；刷新后仍为空。
-  corrective 证据（oa4rust/test-results/r322-forum-final/net-cap.json，真浏览器网络捕获）：
  1. 打开详情时 GET /api/bbs/assemble/control/reply/list/sub/{id} 返回 200，data 数组含 2 条真实回复（R322回帖验证_185893、R322H5回帖_1795270215098）。
  2. 通过真实 UI 提交回复 POST /api/bbs/assemble/control/reply/create 返回 200，新回复 id=c9989e5a… 落库。
  3. 随后 PUT /api/bbs/assemble/control/reply/filter/list/page/1/count/50 返回 200，响应体是**双层信封** {data:{data:[3 条],total:3},type:success}。
  4. 同一时刻 DOM 抓取 replyCards = [bbs-reply-undefined, bbs-reply-undefined, bbs-reply-input, bbs-reply-submit]，detailText 含「回复 ()」「暂无回复」——后端 3 条，前端 0 条可见。
- 根因（源码定位，oa4rust/frontend/apps/desktop/src/views/BBSForum.vue）：
  - :555 api.put 拿到的 resp 是完整 ActionResult；:558 `return ((resp as any)?.data ?? []) as Reply[]` 只解一层，得到内层信封对象 {data:[...],total:3} 而非数组；:562-565 watch 把该对象赋给 replies。
  - 模板 :163 `replies.length` = undefined → 显示「回复 ()」；:165 `replies.length === 0` 为 false → 不显示「暂无回复」；:166 `v-for="reply in replies"` 遍历对象键 data/total → reply 为字符串，:166 :data-testid=bbs-reply-undefined、:173 reply.content 为空。
  - 因果链与 net-cap 中恰好 2 个 bbs-reply-undefined 完全吻合（对象 2 个键）。
- 影响：用户发帖/回帖接口成功、数据持久化成功，但详情页永远看不到任何回复，回复闭环在 UI 层断裂；属数据渲染层真缺陷，非写读不同源。
- 与 R318 区分：R318 是 PC 建版块写 x_bbs_assemble_control_section、H5 picker 读 x_bbs_forum 的跨端数据源不同源；本条是同一主题内后端读写同源、纯前端信封解包错误，二者根因不同，不重复。
- 建议修复方向（本审计不改源码）：把 :558 解包到内层数组（如 resp?.data?.data ?? resp?.data ?? []，并归一 total），或让 watch 对信封做兼容归一；补契约断言回复数为真实条数、reply.id 非 undefined。参考 R315 搜索信封同类修复。

## 二、取证限制（补强，不计新增缺陷）

### L1 通知已读：当前账号无未读会话，且后端会话列表接口本就不返回 unread 字段
- 探针（oa4rust/test-results/r322-forum-final/notice-unread-probe.json）真结果：GET /api/message/assemble/communicate/im/conversation/list/my 返回 200、16 个会话，但每条 sampleKeys 仅 [createTime,id,lastMessage,name,type]，unreadList=[]；DOM 未读栏/全部已读按钮/未读徽标均为 0；pageErrors=[]、apiFailures=[]。
- 源码坐实：后端 oa4rust/backend/crates/message_assemble_communicate/src/lib.rs:644 SQL 仅 SELECT id, name, type, last_message, create_time FROM x_message_conversation；:648-680 data 映射只产出 id/name/type/lastMessage/createTime，**从无 unread 字段**。前端 IMChat.vue 全文件 .unread= 仅 :535 乐观清零，无从后端填充 unread 的路径；:325 totalUnread 依赖 c.unread 恒为 undefined。
- 与 t89 关系：oa4rust/docs/ops/oa4rust-r316-deep-functional-audit.md:20/39/54 已就本域给出「取证限制（非缺陷）：当前账号无未读会话，已读流转无法触发」结论。本轮补强的是「后端不产出 unread 字段」这一结构性原因，**不重复计为新真缺陷**。若后续产品要求未读流转可用，应另开实现任务补后端 unread 统计与前端填充。

## 三、产品语义 / 待查观察（证据不足，不计缺陷）
- 论坛「全部」tab 走 PUT /api/bbs/subject/index/list/page/1/count/20 返回 total=0，而「我的」tab POST /api/bbs/subject/filter/listsubjectinfo/page/1/count/20 返回 29 条；两者过滤口径与权限语义不同。本轮未取得足以判定为缺陷的完整请求/响应链，列为待查，不臆造结论。
- 本域无其他新观察；通知公告 instant/list 列表可见 1 条真实消息，与 t89 一致。

## 四、证据索引（均 gitignored，本地无头浏览器非物理真机）
- 目录：oa4rust/test-results/r322-forum-final/
- 决定性网络捕获：net-cap.json（GET reply/list/sub 2 条、POST reply/create 200、PUT reply/filter/list 双层信封 3 条、DOM bbs-reply-undefined）。
- 回复闭环截图：01-compose.png / 02-my-topics.png / 03-detail-before.png（回复 0、暂无回复）/ 04-replied.png（回复已发送 toast，仍为骨架）/ 05-persisted.png（刷新后与 03 同哈希，回复仍空）。
- 补充探针：reply-probe.json（双层信封实证）、evidence.json（7 断言：topicCreatedViaUI/detailOpened/replyInputPresent/noPageErrors/noApiFailures=true；replyCountFromZero/replyPersistAfterReload=false）、notice-unread-probe.json、01-notice-probe.png。
- 脚本：r322-forum-final.cjs / r322-net-cap.cjs / r322-notice-probe.cjs。

## 结论
R322 补测坐实 1 条新增真缺陷：论坛回复双层信封只解一层导致已落库回复完全不渲染（D1，P2，根因 BBSForum.vue:558 + 模板 163/165/166），给出精确复现与修复方向；通知已读经探针与源码坐实为后端不返回 unread 字段的结构性限制，t89 已报不重复计缺陷。0 pageerror、0 API 失败。按铁律本轮为 ≥1000 轮迭代中的一环，非终态；本任务只读审计，待队长派单再交付 D1 修复。