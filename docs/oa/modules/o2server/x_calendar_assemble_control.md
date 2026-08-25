# o2server

## Responsibility

日历管控模块，处理日历配置和日历业务编排。

## Core Classes and Interfaces

- com.x.calendar.assemble.control.AbstractFactory
- com.x.calendar.assemble.control.ApplicationServletContextListener
- com.x.calendar.assemble.control.Business
- com.x.calendar.assemble.control.EnumCalendarSource
- com.x.calendar.assemble.control.EnumCalendarTaskType
- com.x.calendar.assemble.control.EnumRemindPolicy
- com.x.calendar.assemble.control.ExceptionWrapInConvert
- com.x.calendar.assemble.control.MimeTypeDefinition
- com.x.calendar.assemble.control.ThisApplication
- com.x.calendar.assemble.control.factory.CalendarFactory

## Key Flows

- 控制配置读写：`GET .../get/control/config` 读 `cal_control_config`（config_key='global'）解析 JSON 得 enabled/defaultTimeZone/allowSharing，无记录时回退默认值；`update/control/config` 以 ON CONFLICT upsert 写回
- 日历列表：`GET .../list/control/calendars` 查 `CAL_CALENDAR WHERE status = 'OPEN'`（LIMIT 50）返回 id/name/type/enabled
- 日历详情聚合：`GET /jaxrs/calendar/assemble/control/calendar/detail/{id}` 查 `CAL_CALENDAR` 单行并联查 `CAL_EVENT`（status='OPEN'，按 start_time 升序 LIMIT 100）组装 events 与 eventCount

## Dependencies



- x_base_core_project
- x_organization_core_entity
- x_organization_core_express
- x_calendar_core_entity

**Rust（oa4rust/crates/calendar_assemble_control）：**

- 内部 path 依赖：shared
- 关键外部依赖：axum、tokio、deadpool-postgres、serde/serde_json、tower

## REST Endpoints



- `GET /jaxrs/calendar/assemble/control/calendar/detail/{id}`
- `GET /jaxrs/calendar_assemble_control/get/control/config`
- `GET /jaxrs/calendar_assemble_control/list/control/calendars`
- `GET /jaxrs/calendar_assemble_control/update/control/config`
