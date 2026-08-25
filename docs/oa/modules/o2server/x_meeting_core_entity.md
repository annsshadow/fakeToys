# o2server

## Responsibility

会议核心实体模块，定义会议室和会议数据模型。

## Core Classes and Interfaces

- com.x.meeting.core.entity.Attachment
- com.x.meeting.core.entity.Attachment_
- com.x.meeting.core.entity.Building
- com.x.meeting.core.entity.Building_
- com.x.meeting.core.entity.ConfirmStatus
- com.x.meeting.core.entity.Meeting
- com.x.meeting.core.entity.MeetingConfig
- com.x.meeting.core.entity.MeetingConfigProperties
- com.x.meeting.core.entity.MeetingConfig_
- com.x.meeting.core.entity.MeetingModeEnum

## Key Flows

- 会议室列表：`GET /jaxrs/meeting/core/entity/room/list` → `room_list` 查 `x_meeting_room`，Name 升序 limit 20，可选字段经 `option_to_json` 为 None 时省略键
- 会议室创建：`POST .../room/create` → name 必填否则 BadRequest("name is required")，equipment 序列化为 JSON 字符串存储，uuid v4、create_time=Utc now
- 会议室查询/更新/删除：`GET .../room/{id}` 无则 error("room not found")；`POST .../room/save/{id}` 与 `POST .../room/delete/{id}` 经 deadpool 执行原生 UPDATE/DELETE SQL，rows_affected==0 时 error("room not found")
- 会议列表：`GET .../meeting/list` StartTime 倒序 limit 20，organizerId 取 creator 字段；startTime/endTime 输出键名为带引号的 `"\"startTime\""`/`"\"endTime\""`
- 按会议室查会议：`GET .../meeting/list/by/{roomId}` 过滤 RoomId，StartTime 倒序 limit 20
- 创建会议：`POST .../meeting/create` → title/roomId/startTime/endTime 必填否则 BadRequest；时间串 parse NaiveDateTime 失败报 invalid "startTime"；organizerId 默认 "system"
- 会议更新/删除：`POST .../meeting/save/{id}`、`POST .../meeting/delete/{id}` 原生 SQL 操作 x_meeting，rows_affected==0 时 error("meeting not found")
- 路由注册：`meeting_core_entity_router(pool)` 挂 room 5 条 + meeting 6 条共 11 条路由，`.with_state(pool)` 注入连接池

## Dependencies



- x_base_core_project

**Rust（oa4rust/crates/meeting_core_entity）：**

- 内部 path 依赖：shared
- 关键外部依赖：axum、tokio、sea-orm、deadpool-postgres、serde/serde_json、uuid、chrono、tower

## REST Endpoints

<!-- Generated from Swagger annotations or action JSON files. Omit this section if the module has no REST endpoints. -->

- [Endpoint list]
