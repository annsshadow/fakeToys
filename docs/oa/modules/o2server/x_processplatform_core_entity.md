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

## Key Flows

- 工作列表/详情：`GET /jaxrs/process/work/list|work/{id}` → SeaORM 查 pp_work，DeletedAt IS NULL，list 按 CreateTime 倒序 limit 20，formData 字符串反序列化为 JSON；详情未命中报 NotFound
- 任务查询：`GET /jaxrs/process/task/list|task/{id}` → pp_task 同规则过滤软删（list CreateTime 倒序 limit 20），输出 workId/title/assigneeId/status/createTime
- 工票查询：`GET /jaxrs/process/ticket/list` → `ticket_list` 过滤 DeletedAt IS NULL 按 CreateTime 倒序 limit 20，description 可空
- 已办列表：`GET /jaxrs/process/workcompleted/list` → `workcompleted_list` 查 pp_work_completed 按 CompleteTime 倒序 limit 20，输出 workId/result/completeTime
- 路由注册：`processplatform_core_entity_router(_pool)` 挂 work/task/ticket/workcompleted 共 6 条只读路由；routes.rs 委托回 lib.rs

## Dependencies



- x_base_core_project
- x_query_core_entity

**Rust（oa4rust/crates/processplatform_core_entity）：**

- 内部 path 依赖：shared
- 关键外部依赖：axum、tokio、sea-orm、deadpool-postgres、serde/serde_json、uuid、tower

## REST Endpoints



- `GET /jaxrs/process/task/list`
- `GET /jaxrs/process/task/{id}`
- `GET /jaxrs/process/ticket/list`
- `GET /jaxrs/process/work/list`
- `GET /jaxrs/process/work/{id}`
- `GET /jaxrs/process/workcompleted/list`
