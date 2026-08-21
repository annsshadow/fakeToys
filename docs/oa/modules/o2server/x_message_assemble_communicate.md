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

## Dependencies



- x_base_core_project
- x_organization_core_entity
- x_organization_core_express
- x_message_core_entity
- kafka-clients
- activemq-client

## REST Endpoints



- `GET /jaxrs/message/assemble/communicate/consume/list/{consume}/count/{count}`
- `GET /jaxrs/message/assemble/communicate/consume/list/{consume}/currentperson/count/{count}`
- `GET /jaxrs/message/assemble/communicate/consume/list/{consume}/person/{person}/count/{count}`
- `GET /jaxrs/message/assemble/communicate/consume/type/{type}`
- `POST /jaxrs/message/assemble/communicate/consume/type/{type}/mockputtopost`
- `POST /jaxrs/message/assemble/communicate/consume/{id}/type/{type}`
- `POST /jaxrs/message/assemble/communicate/im/conversation`
- `GET /jaxrs/message/assemble/communicate/im/conversation/business/{businessId}`
- `GET /jaxrs/message/assemble/communicate/im/conversation/list/my`
- `GET /jaxrs/message/assemble/communicate/im/conversation/list/with/person`
- `POST /jaxrs/message/assemble/communicate/im/conversation/mockputtopost`
- `GET /jaxrs/message/assemble/communicate/im/conversation/{id}`
- `GET /jaxrs/message/assemble/communicate/im/conversation/{id}/group`
- `DELETE /jaxrs/message/assemble/communicate/im/conversation/{id}/group/mockdeletetoget`
- `POST /jaxrs/message/assemble/communicate/im/conversation/{id}/group/quit/self`
- `GET /jaxrs/message/assemble/communicate/im/conversation/{id}/icon`
- `POST /jaxrs/message/assemble/communicate/im/conversation/{id}/read`
- `POST /jaxrs/message/assemble/communicate/im/conversation/{id}/read/mockputtopost`
- `GET /jaxrs/message/assemble/communicate/im/conversation/{id}/single`
- `DELETE /jaxrs/message/assemble/communicate/im/conversation/{id}/single/mockdeletetoget`
- `POST /jaxrs/message/assemble/communicate/im/conversation/{id}/top/cancel`
- `POST /jaxrs/message/assemble/communicate/im/conversation/{id}/top/cancel/mockputtopost`
- `POST /jaxrs/message/assemble/communicate/im/conversation/{id}/top/set`
- `POST /jaxrs/message/assemble/communicate/im/conversation/{id}/top/set/mockputtopost`
- `GET /jaxrs/message/assemble/communicate/im/manager/config`
- `POST /jaxrs/message/assemble/communicate/im/msg`
- `POST /jaxrs/message/assemble/communicate/im/msg/clear`
- `POST /jaxrs/message/assemble/communicate/im/msg/collection`
- `GET /jaxrs/message/assemble/communicate/im/msg/collection/list/{page}/size/{size}`
- `POST /jaxrs/message/assemble/communicate/im/msg/collection/remove`
- `GET /jaxrs/message/assemble/communicate/im/msg/download/{id}`
- `GET /jaxrs/message/assemble/communicate/im/msg/download/{id}/image/width/{width}/height/{height}`
- `GET /jaxrs/message/assemble/communicate/im/msg/list/object`
- `GET /jaxrs/message/assemble/communicate/im/msg/list/{page}/size/{size}`
- `POST /jaxrs/message/assemble/communicate/im/msg/revoke/{id}`
- `POST /jaxrs/message/assemble/communicate/im/msg/upload/{conversationId}/type/{type}`
- `GET /jaxrs/message/assemble/communicate/instant/currentperson/consumed`
- `GET /jaxrs/message/assemble/communicate/instant/currentperson/consumed/all`
- `POST /jaxrs/message/assemble/communicate/instant/currentperson/consumed/mockputtopost`
- `GET /jaxrs/message/assemble/communicate/instant/list/currentperson/consumed/count/{count}/asc`
- `GET /jaxrs/message/assemble/communicate/instant/list/currentperson/consumed/count/{count}/desc`
- `GET /jaxrs/message/assemble/communicate/instant/list/currentperson/count/{count}/asc`
- `GET /jaxrs/message/assemble/communicate/instant/list/currentperson/count/{count}/desc`
- `GET /jaxrs/message/assemble/communicate/instant/list/currentperson/noim/count/{count}/desc`
- `GET /jaxrs/message/assemble/communicate/instant/list/currentperson/not/consumed/count/{count}/asc`
- `GET /jaxrs/message/assemble/communicate/instant/list/currentperson/not/consumed/count/{count}/desc`
- `GET /jaxrs/message/assemble/communicate/instant/list/{id}/next/{count}`
- `GET /jaxrs/message/assemble/communicate/instant/list/{id}/prev/{count}`
- `POST /jaxrs/message/assemble/communicate/mark_read/{id}`
- `POST /jaxrs/message/assemble/communicate/mass/enable/type`
- `GET /jaxrs/message/assemble/communicate/mass/list/{id}/next/{count}`
- `GET /jaxrs/message/assemble/communicate/mass/list/{id}/prev/{count}`
- `GET /jaxrs/message/assemble/communicate/mass/{id}`
- `DELETE /jaxrs/message/assemble/communicate/mass/{id}/mockdeletetoget`
- `POST /jaxrs/message/assemble/communicate/message/custom/create`
- `GET /jaxrs/message/assemble/communicate/message/list/paging/{page}/size/{size}`
- `GET /jaxrs/message/assemble/communicate/receive/{consume}`
- `POST /jaxrs/message/assemble/communicate/send`
