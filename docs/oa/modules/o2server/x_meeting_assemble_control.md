# o2server

## Responsibility

会议管控模块，处理会议业务编排、日程关联和会议室调度。

## Core Classes and Interfaces

- com.x.meeting.assemble.control.AbstractFactory
- com.x.meeting.assemble.control.ApplicationServletContextListener
- com.x.meeting.assemble.control.Business
- com.x.meeting.assemble.control.MessageFactory
- com.x.meeting.assemble.control.ThisApplication
- com.x.meeting.assemble.control.WrapTools
- com.x.meeting.assemble.control.factory.AttachmentFactory
- com.x.meeting.assemble.control.factory.BuildingFactory
- com.x.meeting.assemble.control.factory.MeetingFactory
- com.x.meeting.assemble.control.factory.RoomFactory

## Key Flows

- 创建/修改会议：`POST /jaxrs/meeting/assemble/control/meeting/create` → `create_meeting`（校验 title/startTime/endTime，uuid v4 生成 id）→ INSERT INTO `x_meeting`（title/content/start_time/end_time/creator）→ 返回新会议 id；`POST .../meeting/save/{id}` → `save_meeting` UPDATE 同表
- 邀请与应答：`POST .../meeting/{id}/add/invite` → INSERT INTO `x_meeting_invite`（status='wait'）；`POST .../meeting/{id}/accept` → `meeting_id_accept` UPDATE `x_meeting` SET status='accepted'
- 楼宇与会议室查询：`GET .../building/list` → `building_list` 查询 `x_meeting_building`（count+data JSON，另有 like/pinyin/pinyininitial 变体）；`GET .../room/list` → `room_list` 查询 `x_meeting_room`

## Dependencies



- x_base_core_project
- x_organization_core_entity
- x_organization_core_express
- x_meeting_core_entity
- x_general_core_entity

**Rust（oa4rust/crates/meeting_assemble_control）：**

- 内部 path 依赖：shared
- 关键外部依赖：axum、tokio、deadpool-postgres、serde/serde_json、uuid、tower

## REST Endpoints



- `GET /jaxrs/meeting/assemble/control/building/list`
- `GET /jaxrs/meeting/assemble/control/building/list/like/pinyin/{key}`
- `GET /jaxrs/meeting/assemble/control/building/list/like/{key}`
- `GET /jaxrs/meeting/assemble/control/building/list/pinyininitial/{key}`
- `GET /jaxrs/meeting/assemble/control/building/{id}`
- `GET /jaxrs/meeting/assemble/control/config/system/config`
- `POST /jaxrs/meeting/assemble/control/config/system/config/manage`
- `POST /jaxrs/meeting/assemble/control/create`
- `DELETE /jaxrs/meeting/assemble/control/delete/{id}`
- `GET /jaxrs/meeting/assemble/control/list/{meetingId}`
- `POST /jaxrs/meeting/assemble/control/meeting/create`
- `POST /jaxrs/meeting/assemble/control/meeting/delete/{id}`
- `GET /jaxrs/meeting/assemble/control/meeting/list/applied/completed`
- `GET /jaxrs/meeting/assemble/control/meeting/list/applied/processing`
- `GET /jaxrs/meeting/assemble/control/meeting/list/applied/wait`
- `GET /jaxrs/meeting/assemble/control/meeting/list/apply/{page}/size/{size}`
- `GET /jaxrs/meeting/assemble/control/meeting/list/coming/day/{count}`
- `GET /jaxrs/meeting/assemble/control/meeting/list/invited/completed`
- `GET /jaxrs/meeting/assemble/control/meeting/list/invited/processing`
- `GET /jaxrs/meeting/assemble/control/meeting/list/invited/rejected`
- `GET /jaxrs/meeting/assemble/control/meeting/list/invited/wait`
- `GET /jaxrs/meeting/assemble/control/meeting/list/wait/accept`
- `GET /jaxrs/meeting/assemble/control/meeting/list/wait/confirm`
- `GET /jaxrs/meeting/assemble/control/meeting/list/year/{year}/month/{month}`
- `GET /jaxrs/meeting/assemble/control/meeting/list/year/{year}/month/{month}/all`
- `GET /jaxrs/meeting/assemble/control/meeting/list/year/{year}/month/{month}/day/{day}`
- `GET /jaxrs/meeting/assemble/control/meeting/list/year/{year}/month/{month}/day/{day}/all`
- `POST /jaxrs/meeting/assemble/control/meeting/save/{id}`
- `GET /jaxrs/meeting/assemble/control/meeting/{id}`
- `POST /jaxrs/meeting/assemble/control/meeting/{id}/accept`
- `POST /jaxrs/meeting/assemble/control/meeting/{id}/add/invite`
- `POST /jaxrs/meeting/assemble/control/meeting/{id}/checkin`
- `POST /jaxrs/meeting/assemble/control/meeting/{id}/confirm/allow`
- `POST /jaxrs/meeting/assemble/control/meeting/{id}/confirm/deny`
- `POST /jaxrs/meeting/assemble/control/meeting/{id}/delete/invite`
- `POST /jaxrs/meeting/assemble/control/meeting/{id}/manual/completed`
- `POST /jaxrs/meeting/assemble/control/meeting/{id}/modify/completedtime`
- `POST /jaxrs/meeting/assemble/control/meeting/{id}/modify/starttime`
- `POST /jaxrs/meeting/assemble/control/meeting/{id}/reject`
- `GET /jaxrs/meeting/assemble/control/openmeeting/list/room`
- `GET /jaxrs/meeting/assemble/control/room/list`
- `GET /jaxrs/meeting/assemble/control/room/{id}`
