# o2server

## Responsibility

日历核心实体模块，定义日历事件数据模型和基础查询。

## Core Classes and Interfaces

- com.x.calendar.core.entity.Calendar
- com.x.calendar.core.entity.Calendar_
- com.x.calendar.core.entity.Calendar_Event
- com.x.calendar.core.entity.Calendar_EventComment
- com.x.calendar.core.entity.Calendar_EventComment_
- com.x.calendar.core.entity.Calendar_EventRepeatMaster
- com.x.calendar.core.entity.Calendar_EventRepeatMaster_
- com.x.calendar.core.entity.Calendar_Event_
- com.x.calendar.core.entity.Calendar_Setting
- com.x.calendar.core.entity.Calendar_SettingLobValue

## Key Flows

- 公开日历：`GET /jaxrs/calendar/core/entity/calendar/list/public` → `calendar_list_public` 查 `x_cal_calendar` 过滤 IsPublic=true 且 Status="OPEN"，CreateTime 倒序 limit 50
- 我的日历：`GET .../calendar/list/my` → `calendar_list_my` 仅过滤 Status="OPEN"，按 type_（UNIT 不区分大小写）拆分为 myCalendars/unitCalendars，followCalendars 固定空数组
- 日历 CRUD：`calendar_get`（find_by_id + Status="OPEN"，无则 error("calendar not found")）；`calendar_create` 强制 name/type，target 默认 "person"、color 默认 "#1462be"、createor 默认 "anonymous"、status 初始 OPEN；`calendar_update` 缺省字段沿用旧值；`calendar_remove` 软删——status 置 CLOSED
- 事件 CRUD：`event_create` 校验 calendarId/title/startTime/endTime 必填并 parse 为 NaiveDateTime，visibility 默认 PUBLIC；`event_update` 同样过滤 Status="OPEN"；`event_remove` 软删置 CLOSED
- 事件列表：`GET .../event/list/{calendarId}` → `event_list_by_calendar` 过滤 CalendarId+Status="OPEN"，StartTime 升序 limit 100，返回 `{count, calendarId, data}`
- 路由注册：`calendar_core_entity_router(_pool)` 挂 calendar 6 条 + event 4 条路由；routes.rs 委托回 lib.rs

## Dependencies



- x_base_core_project

**Rust（oa4rust/crates/calendar_core_entity）：**

- 内部 path 依赖：shared
- 关键外部依赖：axum、tokio、sea-orm、deadpool-postgres、serde/serde_json、uuid、chrono、tower

## REST Endpoints

<!-- Generated from Swagger annotations or action JSON files. Omit this section if the module has no REST endpoints. -->

- [Endpoint list]
