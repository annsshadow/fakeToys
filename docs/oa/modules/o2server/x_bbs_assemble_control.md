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

## Dependencies



- x_base_core_project
- x_bbs_core_entity
- x_organization_core_express
- x_general_core_entity

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
