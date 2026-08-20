# o2server

## Responsibility

流程平台核心实体模块，定义工作、任务、工单等流程数据模型。

## Core Classes and Interfaces

- com.x.processplatform.core.entity.PersistenceProperties
- com.x.processplatform.core.entity.content.Attachment
- com.x.processplatform.core.entity.content.AttachmentProperties
- com.x.processplatform.core.entity.content.Attachment_
- com.x.processplatform.core.entity.content.Data
- com.x.processplatform.core.entity.content.DataRecord
- com.x.processplatform.core.entity.content.DataRecordItem
- com.x.processplatform.core.entity.content.DataRecordProperties
- com.x.processplatform.core.entity.content.DataRecord_
- com.x.processplatform.core.entity.content.DocSign

## Dependencies



- x_base_core_project
- x_query_core_entity

## REST Endpoints



- `GET /jaxrs/process/task/list`
- `GET /jaxrs/process/task/{id}`
- `GET /jaxrs/process/ticket/list`
- `GET /jaxrs/process/work/list`
- `GET /jaxrs/process/work/{id}`
- `GET /jaxrs/process/workcompleted/list`
