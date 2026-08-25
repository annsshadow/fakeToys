# o2server

## Responsibility

论坛管控模块，处理论坛配置、版块管理和主题回复。

## Core Classes and Interfaces

- com.x.bbs.assemble.common.date.DateOperation
- com.x.bbs.assemble.control.AbstractFactory
- com.x.bbs.assemble.control.ApplicationServletContextListener
- com.x.bbs.assemble.control.Business
- com.x.bbs.assemble.control.MessageFactory
- com.x.bbs.assemble.control.ThisApplication
- com.x.bbs.assemble.control.factory.BBSConfigSettingFactory
- com.x.bbs.assemble.control.factory.BBSForumInfoFactory
- com.x.bbs.assemble.control.factory.BBSOperationRecordFactory
- com.x.bbs.assemble.control.factory.BBSPermissionInfoFactory

## Key Flows

- 管控配置读写：`GET /jaxrs/bbs/assemble/control/config` → `get_control_config::query_one` 读取 `x_bbs_assemble_control_config` → 返回 enabled/maxForumCount/allowAnonymous；`POST .../update/control/config` → `update_control_config` UPDATE 同表
- 发表主题：`POST /jaxrs/bbs/assemble/control/topic/create` → `create_topic`（uuid v4 生成 id，creator 缺省 "system"）→ INSERT INTO `x_bbs_topic`（forum_id/title/content/creator/create_time）→ 返回新主题 id；`GET .../topic/list/forum/{forumId}` 按 create_time DESC 查询同表
- 发表回复与版块浏览：`POST /jaxrs/bbs/assemble/control/reply/create` → `create_reply` INSERT INTO `x_bbs_reply`；`GET /jaxrs/bbs/assemble/control/forum/list` → `list_forums` 查询 `x_bbs_forum` ORDER BY sort ASC 返回论坛列表

## Dependencies



- x_base_core_project
- x_bbs_core_entity
- x_organization_core_express
- x_general_core_entity

**Rust（oa4rust/crates/bbs_assemble_control）：**

- 内部 path 依赖：shared
- 关键外部依赖：axum、tokio、deadpool-postgres、serde/serde_json、uuid、tower、bcrypt、md5、chrono

## REST Endpoints



- `GET /jaxrs/bbs/assemble/control/config`
- `GET /jaxrs/bbs/assemble/control/forum/list`
- `GET /jaxrs/bbs/assemble/control/forum/view/all`
- `GET /jaxrs/bbs/assemble/control/forum/{id}`
- `GET /jaxrs/bbs/assemble/control/permission/section/{sectionId}`
- `GET /jaxrs/bbs/assemble/control/permission/subject/{subjectId}`
- `POST /jaxrs/bbs/assemble/control/reply/create`
- `GET /jaxrs/bbs/assemble/control/reply/list/sub/{id}`
- `GET /jaxrs/bbs/assemble/control/section/list`
- `GET /jaxrs/bbs/assemble/control/section/viewforum/{forumId}`
- `POST /jaxrs/bbs/assemble/control/shutup/create`
- `GET /jaxrs/bbs/assemble/control/subject/top/{sectionId}`
- `GET /jaxrs/bbs/assemble/control/subject/view/{id}`
- `POST /jaxrs/bbs/assemble/control/topic/create`
- `GET /jaxrs/bbs/assemble/control/topic/list/forum/{forumId}`
- `POST /jaxrs/bbs/assemble/control/update/control/config`
- `GET /jaxrs/bbs/assemble/control/uuid`
