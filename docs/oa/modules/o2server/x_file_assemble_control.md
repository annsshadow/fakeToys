# o2server

## Responsibility

文件管控模块，处理文件配置、存储池、分类管理等文件业务编排。

## Core Classes and Interfaces

- com.x.file.assemble.control.AbstractFactory
- com.x.file.assemble.control.ApplicationServletContextListener
- com.x.file.assemble.control.Business
- com.x.file.assemble.control.FileUtil
- com.x.file.assemble.control.ThisApplication
- com.x.file.assemble.control.factory.Attachment2Factory
- com.x.file.assemble.control.factory.AttachmentFactory
- com.x.file.assemble.control.factory.FileFactory
- com.x.file.assemble.control.factory.Folder2Factory
- com.x.file.assemble.control.factory.FolderFactory

## Dependencies



- x_base_core_project
- x_organization_core_entity
- x_organization_core_express
- x_file_core_entity
- x_general_core_entity
- x_cms_core_entity

## REST Endpoints



- `GET /jaxrs/anonymous/file/{id}/download/stream`
- `GET /jaxrs/attachment/download/{attid}/stream`
- `GET /jaxrs/file/assemble/control/attachment2/{id}/office/preview/type/{type}`
- `POST /jaxrs/file/assemble/control/file/create`
- `POST /jaxrs/file/assemble/control/file/delete/{id}`
- `GET /jaxrs/file/assemble/control/file/list/{folderId}`
- `POST /jaxrs/file/assemble/control/file/upload`
- `GET /jaxrs/file/assemble/control/file/{id}`
- `POST /jaxrs/file/core/entity/file/create`
- `POST /jaxrs/file/core/entity/file/delete/{id}`
- `POST /jaxrs/file/core/entity/file/update/{id}`
- `GET /jaxrs/file/{id}/download/stream`
