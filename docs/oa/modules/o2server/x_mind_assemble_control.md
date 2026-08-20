# o2server

## Responsibility

思维导图管控模块，处理导图配置和文件夹管理。

## Core Classes and Interfaces

- com.x.mind.assemble.common.date.DateOperation
- com.x.mind.assemble.control.AbstractFactory
- com.x.mind.assemble.control.ApplicationServletContextListener
- com.x.mind.assemble.control.Business
- com.x.mind.assemble.control.MessageFactory
- com.x.mind.assemble.control.ThisApplication
- com.x.mind.assemble.control.factory.MindBaseInfoFactory
- com.x.mind.assemble.control.factory.MindContentInfoFactory
- com.x.mind.assemble.control.factory.MindFolderInfoFactory
- com.x.mind.assemble.control.factory.MindIconInfoFactory

## Dependencies



- x_base_core_project
- x_organization_core_express
- x_mind_core_entity
- x_general_core_entity

## REST Endpoints



- `GET /jaxrs/mind/assemble/control/config`
- `POST /jaxrs/mind/assemble/control/config/update`
- `POST /jaxrs/mind/assemble/control/folder/move/{folderId}`
- `POST /jaxrs/mind/assemble/control/folder/save`
- `GET /jaxrs/mind/assemble/control/folder/tree/my`
- `GET /jaxrs/mind/assemble/control/folder/{id}`
- `POST /jaxrs/mind/assemble/control/folder/{id}/force`
- `POST /jaxrs/mind/assemble/control/folder/{id}/update`
