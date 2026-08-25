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

## Key Flows

- 上传建档：`POST /jaxrs/file/assemble/control/file/upload|create` → 从 Session 取 person_unique 为 creator → INSERT INTO `x_file`（name/path/size/folder_id）→ 返回文件元数据
- 下载分发：`GET /jaxrs/file/{id}/download/stream`、`/jaxrs/attachment/download/{attid}/stream` 及 anonymous 变体 → 查 `FILE_FILE` 内容并 base64 解码 → 按原 MIME 附件流式响应
- 预览与存储配置：`GET .../attachment2/{id}/office/preview/type/{type}` → `require_owner` 归属校验后 .docx 转 HTML 返回（其余格式降级）；控制配置端点查询 `x_file_assemble_control_storage_pool` 等配置表

## Dependencies



- x_base_core_project
- x_organization_core_entity
- x_organization_core_express
- x_file_core_entity
- x_general_core_entity
- x_cms_core_entity

**Rust（oa4rust/crates/file_assemble_control）：**

- 内部 path 依赖：shared
- 关键外部依赖：axum（multipart）、deadpool-postgres、zip

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
