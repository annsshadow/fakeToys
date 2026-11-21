# R317 静态潜伏缺陷审计：实体 create→list 同源性与响应字段映射

> 本轮为纯静态源码审计，**未改动任何源码/契约/配置**，**未连接测试服、未做真浏览器验证**；文中所有候选均为「静态推理候选」，不构成已确认缺陷。测试环境地址一律以 `<YOUR_TEST_DOMAIN>` / `<YOUR_TEST_HOST>` 占位，全文 0 真值。

## 1. 方法

- 对象：8 类业务实体（BBS 版块/主题/回复、会议、日历、组织单元/群组/人员、文件、流程任务/已办、CMS 笔记、通知/消息）。
- 口径：对每一类实体，从后端 `routes.rs`（或 `lib.rs` 内联路由）确认创建与列表端点，从处理函数提取 `INSERT INTO ...` 写入表/字段与 `SELECT ... FROM ...` 读取表/字段，判断 create 与 list 是否落在**同一张表**；再核对桌面端前端调用端点与响应字段消费是否与之匹配。
- 工具：`ops_grep` 正则 `INSERT INTO\s+\w+|FROM\s+[a-zA-Z_][a-zA-Z0-9_]*` 限定各域生产 `lib.rs`；端点用 `ops_grep` 路由文件；前端用 `ops_grep` 限定 `apps/desktop/src`。
- 已修同类真 bug 作为同源性判断参照（不再重复计为新发现）：BBS subject::create 写 `bbs_subject_info` 而 list 读 `x_bbs_topic`（R303 修）、日历 core_entity create/list 跨表漂移（R311 修）、流程 task_complete 未写 `PP_C_TASKCOMPLETED` 致已办恒空（R314/t83 修）、会议 create 时间绑定缺 cast 与 applied 不可见（R309/t67 修）、文件上传 415 与 folder_id 空串不可见（R314/t82 修）。

## 2. 逐域核对表（8 类实体）

| # | 实体 | 创建端点 → 写入表/字段（源码定位） | 列表端点 → 读取表/字段（源码定位） | 同源? | 前端消费匹配? |
|---|---|---|---|---|---|
| 1 | BBS 版块 section | `POST /api/bbs/assemble/control/section/create` → `shared::crud_create` spec `table:"x_bbs_assemble_control_section"`（`bbs_assemble_control/src/lib.rs:247-298`） | `GET /api/bbs/assemble/control/section/list` → `SELECT id,name,enabled FROM x_bbs_assemble_control_section`（同文件 `:216`） | 是（同表） | 是 |
| 2 | BBS 主题 topic | `POST /api/bbs/assemble/control/topic/create` → `INSERT INTO x_bbs_topic (id,forum_id,title,content,creator,author_id,section_id,create_time)`（`:471-474`） | `GET /api/bbs/assemble/control/topic/list/forum/{forumId}` → `SELECT ... FROM x_bbs_topic WHERE forum_id=$1`（`:496`）；另有全局/置顶/加精多 list 均读 `x_bbs_topic` | 是（R303 已修） | 是 |
| 3 | BBS 回复 reply | `POST /api/bbs/assemble/control/reply/create` → `INSERT INTO x_bbs_reply (id,subject_id,author_id,topic_id,content,creator,create_time)`，`topic_id=subject_id` 双写（`:542-545`） | `GET /api/bbs/assemble/control/reply/list/sub/{id}` → `SELECT id,topic_id,content,creator,create_time FROM x_bbs_reply WHERE topic_id=$1`（`:718`）；全局回复 list 同表（`:689`） | 是 | 是（create 双写 topic_id，list 按 topic_id 过滤） |
| 4 | 会议 meeting | `POST /api/meeting/assemble/control/meeting/create` → `INSERT INTO x_meeting (...,applied) VALUES (...,$5::text::timestamp,$6::text::timestamp,...,TRUE)`（`meeting_assemble_control/src/lib.rs:2503`，R309/t67 修） | `GET /api/meeting/assemble/control/meeting/list/apply/{page}/size/{size}` → `FROM x_meeting WHERE applied=true ...`（`:1070`）；分类 list 同表 | 是 | 是（t67 真浏览器已证列表回显+刷新持久） |
| 5 | 日历 calendar | `POST /api/calendar/core/entity/calendar/create` → `cal_calendar::ActiveModel{...}.insert()`（`calendar_core_entity/src/lib.rs:374-391`，实体映射 `x_cal_calendar`，R311 修） | `GET /api/calendar/core/entity/calendar/list/my` 等 → `FROM x_cal_calendar ...`（同文件 `:278`） | 是（core 族；R311 已修） | 是 |
| 6 | 组织单元/群组/人员 | `x_org_unit`：`INSERT INTO x_org_unit (...)`（`organization_assemble_control/src/u2_org.rs:53`）；`x_org_group`：同文件 `:661`；`x_org_person`：`u2_person.rs:128` | `x_org_unit`：`SELECT ... FROM x_org_unit WHERE parent_id IS NULL/=$1`（`lib.rs:169,177`）；`x_org_group`：`FROM x_org_group`（`:682,729`）；`x_org_person`：`FROM x_org_person`（`:2270,2323`） | 是（三子域均同表） | 是 |
| 7 | 文件 file | `POST /api/file/assemble/control/file/create` → `INSERT INTO x_file (id,name,path,size,creator,create_time,folder_id)`（`file_assemble_control/src/lib.rs:437,493,582`）；`upload` → `INSERT INTO x_file ... folder_id='root'`（R314/t82 修） | `GET /api/file/assemble/control/file/list/{folderId}` → `FROM x_file WHERE folder_id=$1 AND deleted_at IS NULL`（`:293`）；`FILE_FILE` 族另走附件/网盘模型（`:836` 等），与 `x_file` 分属不同业务语义 | 是（各自族内同源；t82 已修 folder_id 口径） | 是（t82 真浏览器已证上传后列表可见+刷新持久） |
| 8a | 流程任务/已办 | `POST .../service/processing/create` → `INSERT INTO x_work (...)`（`processplatform_service_processing/src/lib.rs:166`）；任务 `INSERT INTO x_task`（`:245,1311,1543`）；完成 `INSERT INTO x_workcompleted`（`:2002`，R314/t83 修） | `x_work`：`FROM x_work`（`:187`）；任务：`FROM x_task`（`:1165` 等）；已办：`FROM x_workcompleted`（`:572`） | 是（三族各自同源；R314 已修） | 是 |
| 8b | CMS 笔记 note | `POST /api/cms/core/entity/note/create` → `cms_entity_create("x_cms_note",...)`（`cms_core_entity/src/lib.rs:801-807`，INSERT 模板 `:524`） | `GET /api/cms/core/entity/note/list` → `list_cms_entity_table("x_cms_note")`（`:465-468`，`FROM x_cms_note` `:410`） | 是 | 是 |
| 8c | 通知/消息 message | ① `POST /api/message/custom/create` → `INSERT INTO x_msg_message (id,title,body,type,consumer,is_read,xid,xtitle,xbody,xtype,xconsumed)`（`message/src/lib.rs:151-177`）；② `POST /api/message/assemble/communicate/message/custom/create` → `INSERT INTO x_message (...,sender,type='custom')`（`message_assemble_communicate/src/lib.rs:2812-2855`） | ① `GET /api/message/consume/list/{consume}/count/{count}` → `SELECT ... FROM x_msg_message WHERE xconsumer=$1 AND xconsumed=false`（`message/src/lib.rs:50-62`）；② `GET /api/message/assemble/communicate/message/list/paging/{page}/size/{size}` → `FROM x_message WHERE (sender=$3 OR admin)`（`message_assemble_communicate/src/lib.rs:2869-2888`） | 是（两族各自同表） | **存疑，见 §3 候选 M1/M2** |

## 3. 静态推理候选（有源码定位证据；非已确认缺陷）

### M1（中）消息 custom_create 的 sender 默认值与列表归属谓词口径可能不一致

- 写：`message_assemble_communicate/src/lib.rs:2833-2841`，`sender` 取 `req.sender`，缺省 `"system"`；`conversation_id` 缺省空串，且要求 `is_conversation_member` 通过。
- 读：同文件 `:2861-2885`，列表归属谓词 `MESSAGE_SENDER_OWNER_PREDICATE = (sender = $3 OR admin)`，`$3` 为会话人；桌面端分页消费 `imMessagePaging.ts:49-74` 按 `content/sender/createTime` 渲染。
- 前端：`IMChat.vue:821` 以 `op === 'customCreate'` 调用 `api.post('/api/message/assemble/communicate/message/custom/create', {})`（空请求体）→ 后端落 `sender="system"`、`conversation_id=""`。
- 推理：缺省写入的 `"system"` 不等于任何自然人会话人，该记录在本人视角 `message/list/paging` 中不可见；但 `is_conversation_member("", actor)` 是否放行、以及该端点业务语义（系统播报 vs 本人消息）未经真浏览器验证，**不能据此断言为 bug**。
- 建议修复方向（若确认为缺陷）：创建时以会话人为 `sender` 事实源（仿 BBS create_topic 的 `creator=sess.person_unique`），或前端在调用时显式传入会话人；不动列表读谓词。
- 验证方式：真浏览器登录后调用该创建入口，再打开消息分页面板，检查新消息是否出现；若为系统播报语义则应在文档中明确其不可见性属预期。

### M2（中）系统消息 custom_create 的 consumer 缺省与 consume/list 过滤口径可能不一致

- 写：`message/src/lib.rs:160-175`，`consumer = req.consumer.unwrap_or_default()`（空串），规范列与遗留列同值写入（`xconsumer=$5`）。
- 读：同文件 `:50-62`，`consume_list` 按 `xconsumer = $1`（路径参数 consume）且 `xconsumed=false` 过滤。
- 前端：`IMChat.vue:747-751` 以 `kind === 'custom'` 调用 `api.post('/api/message/custom/create', { title, body })`，未传 `consumer`。
- 推理：空串 `consumer` 只有在调用方用空串作为 consume 路径参数时才可被列出；若前端随后用具体消费者 ID 查询，则新建消息不在结果中。该端点有 admin 门禁（`:157`），可能为管理端按 consumer 定向发布，**缺省为空是否属预期需真浏览器确认**。
- 建议修复方向（若确认为缺陷）：把 `consumer` 设为必填并在前端表单补输入，或创建时回填默认的当前消费者标识；不动列表 SQL。
- 验证方式：真浏览器以 admin 发一条带具体 consumer 的系统消息，再以该 consumer 调 `consume/list/{consume}/count/{count}`，检查是否回显。

### T1（低）日历/组织/文件三域存在「x_* 新表 vs 大写遗留表」双轨读口径

- 日历：`calendar_assemble_control/src/lib.rs:73` 的 `list_control_calendars` 读 `CAL_CALENDAR`；`calendar_core_entity` 的 create/list 读 `x_cal_calendar`（`:278,374-391`）。
- 组织：`organization_assemble_express/src/lib.rs:77` 的 `list_organization_units` 读 `ORG_UNIT`（列 `superior/level`）；`organization_assemble_control`（u2_org.rs:53 / lib.rs:169）读写 `x_org_unit`（列 `parent_id/level`）。
- 文件：`file_assemble_control/src/lib.rs` 中 `x_file`（`:293,437`）与 `FILE_FILE`（`:836,1181`）并存，前者为按 folder 的上传文件，后者为附件/网盘模型（语义不同，非简单双轨）。
- 推理：这些是不同 API base / 不同业务语义的表族，跨族消费会导致「在一处建、在另一处看不到」，但不能仅凭表名不同判定为缺陷；**需先确认前端实际消费路径**。
- 验证方式：真浏览器分别走 assemble/control 与 core/entity 端点，核对同一实体在两族列表中的可见性；若属不同数据源，在文档标注「非同源、属不同业务语义」即可。

## 4. 价值排序（下一步候选）

1. **M1 / M2（中）**：静态证据链最完整（写端点缺省值 vs 读端点过滤谓词 vs 前端调用参数三方对齐），若真浏览器复现则可直接 test-first 修复，优先级最高。
2. **T1（低）**：需先做前端消费路径甄别，避免把不同业务语义的表误判为同源漂移。
3. 其余 6 类实体：主写端点与列表端点同源，且历史同类缺陷已修并有真浏览器证据，本轮无需重复处置。

## 5. 取证边界

- 本文所有结论来自本地静态源码阅读与既有契约核对，**未做真浏览器验证、未连接 `<YOUR_TEST_DOMAIN>` 测试服、未执行任何写操作**。
- §3 列出的候选只是「按静态证据推理出的可疑点」，不构成已确认缺陷；是否成 bug 必须由真浏览器真实流程 + 后端日志/DB 回读坐实后才可进入 test-first RED→GREEN 修复。
- 本轮**未发现可复现的「写 A 表读 B 表」式同源漂移**（历史 4 处同类问题均已修复并有独立证据链）。
- 审计对象限于 8 类实体的主写/主列表端点；孪生端点探测（如 GET 写变体、server 页 twin 探测）不在本文结论范围。

测试环境占位：`<YOUR_TEST_DOMAIN>` / `<YOUR_TEST_HOST>`（本文未写入任何真实域名、IP、口令或个人路径）。