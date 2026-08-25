# o2server

## Responsibility

消息核心实体模块，定义消息数据模型和基础查询能力。

## Core Classes and Interfaces

- com.x.message.core.entity.IMConversation
- com.x.message.core.entity.IMConversationExt
- com.x.message.core.entity.IMConversationExt_
- com.x.message.core.entity.IMConversation_
- com.x.message.core.entity.IMMsg
- com.x.message.core.entity.IMMsgCollection
- com.x.message.core.entity.IMMsgCollection_
- com.x.message.core.entity.IMMsgFile
- com.x.message.core.entity.IMMsgFile_
- com.x.message.core.entity.IMMsg_

## Key Flows

- 消息列表：`GET /jaxrs/message/core/entity/list` → `list` 查 message 实体，CreateTime 倒序 limit 20，输出 id/title/type/consumer/isRead，body 为 Some 时才附加
- 按消费者查询：`GET .../list/by/{consume}` → 过滤 Consumer 等于路径参数，CreateTime 倒序 limit 20
- 未读计数：`GET .../unread/count/{consume}` → `unread_count` 过滤 Consumer + IsRead=false 后 PaginatorTrait count，输出 count 与 consumer
- 路由注册：`message_core_entity_router(_pool)` 挂 list/by-consume/unread-count 共 3 条只读路由（无写操作）

## Dependencies



- x_base_core_project

**Rust（oa4rust/crates/message_core_entity）：**

- 内部 path 依赖：shared
- 关键外部依赖：axum、tokio、sea-orm、deadpool-postgres、serde/serde_json、tower

## REST Endpoints

<!-- Generated from Swagger annotations or action JSON files. Omit this section if the module has no REST endpoints. -->

- [Endpoint list]
