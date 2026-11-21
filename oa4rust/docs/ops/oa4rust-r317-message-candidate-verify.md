# R317 消息候选 M1/M2 真浏览器复验报告

- 任务：t92 [R317][A3]（静态候选坐实/排除，只读）
- 基线：t91 oa4rust-r317-static-latent-audit.md（commit ac8f1d733）
- 仓库分支：opt1000/oa4rust-pc-ai-refactor（未改任何源码/契约/配置，未重建未部署）
- 测试入口：测试服现有产物；测试域名用 <YOUR_TEST_DOMAIN> 占位，本报告 0 真值

## 1. 方法与取证口径

- 真浏览器：Playwright Chromium headless，对已部署测试服做真登录（管理员账号），逐页截图 1440x900。
- 截图来源标注：Playwright Chromium headless against deployed test server; not a physical device（非物理真机）。
- 凭据：运行期从测试服凭据文件读取，不回显、不入库；base URL 运行期从既有 harness 读取，不写入本报告。
- 端点触发方式：IMChat.vue 的 imMsgAction / imMore / openMsgPaging 三个函数在模板中无任何 @click 绑定（源码中仅命中函数定义行），属前端死代码，真实用户无法从 UI 点击触发。因此改用真登录页面内 fetch（credentials: include，与 SDK api.ts 的 credentials: include 一致）直接请求真实后端端点取证，并在 /app/im 真登录页截图留证。
- 数据库回读：远端只读查询（psycopg2）按本次 marker 前缀回读 x_message / x_msg_message，作为写库字段的旁证。
- harness 与证据均落 gitignored 目录 oa4rust/test-results/r317-message-verify/，不入库。
- marker：R317-VFY-1795242264219（本次唯一测试标记，非真值）。

## 2. M1：IM 自定义消息 custom_create 的 sender 缺省

### 2.1 源码定位

- 写端点：POST /api/message/assemble/communicate/message/custom/create（message_assemble_communicate/src/routes.rs:138）。处理函数 message_custom_create（message_assemble_communicate/src/lib.rs 约 :2810-2855）：写表 x_message，字段 (id, conversation_id, content, sender, type, create_time)；sender = req.sender 缺省 system，conversation_id = req.conversationId 缺省空串；门禁 is_conversation_member(&client, conversation_id, &sess.person_unique)（定义同文件 :2957，SQL SELECT 1 FROM x_message_conversation_member WHERE conversation_id=$1 AND person_id=$2）。
- 读端点：POST /api/message/assemble/communicate/message/list/paging/{page}/size/{size}（routes.rs:139-140）。查询 SELECT ... FROM x_message WHERE {predicate} ORDER BY create_time DESC LIMIT $1 OFFSET $2；MESSAGE_SENDER_OWNER_PREDICATE 为 (sender = $3 OR EXISTS(... 角色 admin ...))，$3 = 会话人 sess.person_unique。
- 前端消费：IMChat.vue:785 openMsgPaging → imMessagePaging.ts buildImPagingPath(page,20)；normalizePagingRows 按 content/sender/createTime 渲染，imMore(op=customCreate) 只发空请求体 {}（:821）。

### 2.2 真浏览器请求 / 响应

~~~text
1) POST .../message/custom/create  body={}                  → HTTP 200, body.type=error, message=not a conversation member
2) POST .../im/conversation 建会话                          → HTTP 200, convId=true
3) POST .../message/custom/create  {conversationId, content}（sender 缺省） → HTTP 200, 落库 sender=system
4) POST .../message/custom/create  {conversationId, content, sender=marker-USER-A} → HTTP 200, 落库 sender=marker-USER-A
5) POST .../message/list/paging/1/size/100                  → HTTP 200, 共 12 行；marker 匹配 2 行：
     - DEFAULT-SENDER 行 sender=system（管理员可见）
     - EXPLICIT-SENDER 行 sender=marker-USER-A（管理员可见）
DB 只读回读 x_message：两行 sender 值与分页一致（system / marker-USER-A）。
~~~

### 2.3 判定：部分坐实 / 取证限制

- 空请求体返回业务 error 是门禁对空 conversationId 的正确拒绝，不是 sender 缺省缺陷的直接证据。
- 合法会话下缺省 sender 确实落库为 system（已坐实）；本次以管理员登录，读谓词含 admin 放行分支，两条 marker 消息均可见。
- 对普通（非 admin）会话人，sender = $3 谓词会使 sender=system 的消息不可见——构成潜在可见性缺口，但本轮用管理员账号未能直接复现普通用户不可见，故判为取证限制，需另派非 admin 账号复验后才能定性真缺陷。
- 修复方向（若另派）：message_custom_create 以会话人 sess.person_unique 作为 sender 事实源（而非 req.sender 缺省 system），或新增 creator 列并让列表谓词改用 creator。

## 3. M2：系统消息 custom_create 的 consumer 写读列不同源

### 3.1 源码定位

- 写端点：POST /api/message/custom/create（message/src/routes.rs；message/src/lib.rs 约 :151-177 custom_create，require_admin 门禁）。INSERT：INSERT INTO x_msg_message (id,title,body,type,consumer,is_read,xid,xtitle,xbody,xtype,xconsumed) VALUES ($1,$2,$3,$4,$5,false,$6,$2,$3,$4,false)，$5 = req.consumer.unwrap_or_default()（缺省空串）。INSERT 列清单不含 xconsumer。
- 读端点：GET /api/message/consume/list/{consume}/count/{count}（message/src/lib.rs 约 :50-62 consume_list）：SELECT ... FROM x_msg_message WHERE xconsumer=$1 AND xconsumed=false ORDER BY xcreateTime ASC LIMIT $2。另 unread_count（约 :219-230）：SELECT COUNT(*) ... WHERE xconsumer = $1 AND xconsumed = false。
- 漂移：写列 consumer（规范列），读列 xconsumer（遗留列）——写读不同源。源码注释（O24-3 批次4）曾称规范列与遗留列同值写入，但实际 INSERT 未含 xconsumer，属实现与注释漂移。
- 前端消费：IMChat.vue:747-751 imMsgAction(kind=custom) 只发 {title, body}（不含 consumer）。

### 3.2 真浏览器请求 / 响应

~~~text
1) POST /api/message/custom/create {title,body,consumer=marker-CONSUMER} → HTTP 200, created=true
2) GET  /api/message/consume/list/marker-CONSUMER/count/50             → HTTP 200, data=[], visible=false
3) POST /api/message/custom/create {title,body}（consumer 缺省空串）     → HTTP 200, created=true
4) GET  /api/message/consume/list/<其它 consumer>/count/50             → HTTP 200, data=[], defaultInOther=false
DB 只读回读 x_msg_message 两行：
   - 规范列 consumer = marker-CONSUMER / 空串空值；
   - 遗留列 xconsumer = NULL（两行均 NULL）、xtype = 空串。
~~~

### 3.3 判定：确凿真缺陷（写读列不同源）

- 即使传具体 consumer，consume/list 也查不到新消息：DB 回读 xconsumer=NULL 坐实写列与读列不同源。
- 这比 t91 静态候选 consumer 缺省空串更强：缺省空串只是取值为空，而此处是根本没写读列，故任何 consumer、包括缺省值都无法列出。
- 现象：新建系统消息对任何 consume/list / unread_count 消费者均不可见（消费者列表与未读计数恒为空）。
- 复现：管理员登录 → 按 3.2 顺序请求 → 观察到 create 均 200 而 consume/list 恒为空 → DB 回读 xconsumer=NULL。
- 修复方向（若另派）：custom_create 的 INSERT 补写 xconsumer（$5，与规范列 consumer 同值），使 consume_list / unread_count 读列与写列同源；或统一改为读规范列 consumer 并回填历史数据。注意 xid 为 text、id 为 varchar，主键需绑两次以规避 E42P08；xconsumer 同为文本列可直接复用 consumer 值绑定。

## 4. 判定汇总

~~~text
M1  缺省 sender=system 落库        已坐实；管理员可见；普通用户可见性 = 取证限制（需非 admin 账号复验）
M2  custom_create 写 consumer / 读 xconsumer  确凿真缺陷（写读列不同源，xconsumer=NULL），任何 consumer 不可见
~~~

## 5. 取证边界

- 本轮为只读复验：未改源码/契约/后端/配置，未重建未部署测试服；新建的少量 marker 消息为必要测试数据，均带 R317 前缀可识别。
- IMChat.vue 三个入口为前端死代码，故端点用真登录页面内 fetch 触发；截图仅能证明真登录会话存在与页面渲染状态，端点请求/响应以 evidence.json 与 DB 回读为准。
- M1 普通用户可见性、M2 修复均需另派任务（本任务不修复）。