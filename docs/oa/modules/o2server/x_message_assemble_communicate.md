# o2server

## Responsibility

消息通信模块，处理消息的发送、接收、已读未读统计等通信逻辑。

## Core Classes and Interfaces

- com.x.message.assemble.communicate.AbstractFactory
- com.x.message.assemble.communicate.ActivemqConsumeQueue
- com.x.message.assemble.communicate.AndFxConsumeQueue
- com.x.message.assemble.communicate.ApiConsumeQueue
- com.x.message.assemble.communicate.ApplicationServletContextListener
- com.x.message.assemble.communicate.Business
- com.x.message.assemble.communicate.CalendarConsumeQueue
- com.x.message.assemble.communicate.DingdingConsumeQueue
- com.x.message.assemble.communicate.ExceptionAndFxMessage
- com.x.message.assemble.communicate.ExceptionDingdingMessage

## Key Flows

- 发送消息：`POST /api/message/assemble/communicate/send` → `send_message`（uuid v4 生成 id，type 缺省 "text"）→ INSERT INTO `x_message`（conversation_id/content/sender/type）并 UPDATE `x_message_conversation` SET last_message_time=NOW() → 返回 sent 状态
- 消息接收与已读：`GET .../receive/{consume}` → `receive_list` 查询 `x_message_consume` WHERE consumed=false ORDER BY create_time ASC；`POST .../mark_read/{id}` → UPDATE `x_message_consume` SET consumed=true；另有按 consume/count/type 维度的 `consume/list/*` 分页查询族
- IM 会话管理：`POST .../im/conversation` → `im_conversation` INSERT INTO `x_message_conversation`（type 缺省 "single"）；`GET .../im/conversation/business/{businessId}` 按 business_id 定位会话，`GET .../im/conversation/list/my` ORDER BY update_time DESC 返回会话列表

## Dependencies



- x_base_core_project
- x_organization_core_entity
- x_organization_core_express
- x_message_core_entity
- kafka-clients
- activemq-client

**Rust（oa4rust/crates/message_assemble_communicate）：**

- 内部 path 依赖：shared
- 关键外部依赖：axum、tokio、deadpool-postgres、serde/serde_json、uuid、tower、bcrypt、base64、anyhow、chrono、md5、urlencoding

## REST Endpoints



- `GET /api/message/assemble/communicate/consume/list/{consume}/count/{count}`
- `GET /api/message/assemble/communicate/consume/list/{consume}/currentperson/count/{count}`
- `GET /api/message/assemble/communicate/consume/list/{consume}/person/{person}/count/{count}`
- `GET /api/message/assemble/communicate/consume/type/{type}`
- `POST /api/message/assemble/communicate/consume/type/{type}/mockputtopost`
- `POST /api/message/assemble/communicate/consume/{id}/type/{type}`
- `POST /api/message/assemble/communicate/im/conversation`
- `GET /api/message/assemble/communicate/im/conversation/business/{businessId}`
- `GET /api/message/assemble/communicate/im/conversation/list/my`
- `GET /api/message/assemble/communicate/im/conversation/list/with/person`
- `POST /api/message/assemble/communicate/im/conversation/mockputtopost`
- `GET /api/message/assemble/communicate/im/conversation/{id}`
- `GET /api/message/assemble/communicate/im/conversation/{id}/group`
- `DELETE /api/message/assemble/communicate/im/conversation/{id}/group/mockdeletetoget`
- `POST /api/message/assemble/communicate/im/conversation/{id}/group/quit/self`
- `GET /api/message/assemble/communicate/im/conversation/{id}/icon`
- `POST /api/message/assemble/communicate/im/conversation/{id}/read`
- `POST /api/message/assemble/communicate/im/conversation/{id}/read/mockputtopost`
- `GET /api/message/assemble/communicate/im/conversation/{id}/single`
- `DELETE /api/message/assemble/communicate/im/conversation/{id}/single/mockdeletetoget`
- `POST /api/message/assemble/communicate/im/conversation/{id}/top/cancel`
- `POST /api/message/assemble/communicate/im/conversation/{id}/top/cancel/mockputtopost`
- `POST /api/message/assemble/communicate/im/conversation/{id}/top/set`
- `POST /api/message/assemble/communicate/im/conversation/{id}/top/set/mockputtopost`
- `GET /api/message/assemble/communicate/im/manager/config`
- `POST /api/message/assemble/communicate/im/msg`
- `POST /api/message/assemble/communicate/im/msg/clear`
- `POST /api/message/assemble/communicate/im/msg/collection`
- `GET /api/message/assemble/communicate/im/msg/collection/list/{page}/size/{size}`
- `POST /api/message/assemble/communicate/im/msg/collection/remove`
- `GET /api/message/assemble/communicate/im/msg/download/{id}`
- `GET /api/message/assemble/communicate/im/msg/download/{id}/image/width/{width}/height/{height}`
- `GET /api/message/assemble/communicate/im/msg/list/object`
- `GET /api/message/assemble/communicate/im/msg/list/{page}/size/{size}`
- `POST /api/message/assemble/communicate/im/msg/revoke/{id}`
- `POST /api/message/assemble/communicate/im/msg/upload/{conversationId}/type/{type}`
- `GET /api/message/assemble/communicate/instant/currentperson/consumed`
- `GET /api/message/assemble/communicate/instant/currentperson/consumed/all`
- `POST /api/message/assemble/communicate/instant/currentperson/consumed/mockputtopost`
- `GET /api/message/assemble/communicate/instant/list/currentperson/consumed/count/{count}/asc`
- `GET /api/message/assemble/communicate/instant/list/currentperson/consumed/count/{count}/desc`
- `GET /api/message/assemble/communicate/instant/list/currentperson/count/{count}/asc`
- `GET /api/message/assemble/communicate/instant/list/currentperson/count/{count}/desc`
- `GET /api/message/assemble/communicate/instant/list/currentperson/noim/count/{count}/desc`
- `GET /api/message/assemble/communicate/instant/list/currentperson/not/consumed/count/{count}/asc`
- `GET /api/message/assemble/communicate/instant/list/currentperson/not/consumed/count/{count}/desc`
- `GET /api/message/assemble/communicate/instant/list/{id}/next/{count}`
- `GET /api/message/assemble/communicate/instant/list/{id}/prev/{count}`
- `POST /api/message/assemble/communicate/mark_read/{id}`
- `POST /api/message/assemble/communicate/mass/enable/type`
- `GET /api/message/assemble/communicate/mass/list/{id}/next/{count}`
- `GET /api/message/assemble/communicate/mass/list/{id}/prev/{count}`
- `GET /api/message/assemble/communicate/mass/{id}`
- `DELETE /api/message/assemble/communicate/mass/{id}/mockdeletetoget`
- `POST /api/message/assemble/communicate/message/custom/create`
- `GET /api/message/assemble/communicate/message/list/paging/{page}/size/{size}`
- `GET /api/message/assemble/communicate/receive/{consume}`
- `POST /api/message/assemble/communicate/send`
