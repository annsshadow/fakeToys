# o2server

## Responsibility

流程平台展现管控模块，处理流程表面的预览和发布功能。

## Core Classes and Interfaces

- com.x.processplatform.assemble.surface.AbstractFactory
- com.x.processplatform.assemble.surface.ApplicationServletContextListener
- com.x.processplatform.assemble.surface.Business
- com.x.processplatform.assemble.surface.Control
- com.x.processplatform.assemble.surface.ExceptionRecordProcessing
- com.x.processplatform.assemble.surface.JobControlBuilder
- com.x.processplatform.assemble.surface.ThisApplication
- com.x.processplatform.assemble.surface.WorkCompletedControl
- com.x.processplatform.assemble.surface.WorkCompletedControlBuilder
- com.x.processplatform.assemble.surface.WorkControl

## Key Flows

- 表面管理：`POST /jaxrs/processplatform/assemble/surface/create` → 校验 name 后 INSERT INTO `x_process_surface`（content 存 jsonb）→ 返回 id/name/category/version；`save/{id}`、`publish/{id}`、`delete/{id}` 对同表执行 UPDATE 并回查结果
- 草稿启动与任务办理：`POST .../draft/start/{id}` → UPDATE `PP_C_DRAFT` SET xstatus 后回查草稿元数据；`POST .../task/{id}/processing` → 查询 `PP_C_TASK` 返回任务详情 JSON
- Work 全生命周期与待办列表：`POST .../work/v2/{id}/terminate|reroute|rollback|retract` 按 id 查询 `PP_C_WORK` 返回实例元数据；task/read/workcompleted 分页列表族按条件分页查询 `PP_C_TASK`/`PP_C_READ`/`PP_C_WORKCOMPLETED`/`PP_C_JOB` 后返回 count+data

## Dependencies



- x_base_core_project
- x_organization_core_entity
- x_organization_core_express
- x_query_core_entity
- x_processplatform_core_entity
- x_processplatform_core_express
- x_cms_core_entity
- x_portal_core_entity
- x_general_core_entity
- x_program_center_core_entity
- x_correlation_core_express

**Rust（oa4rust/crates/processplatform_assemble_surface）：**

- 内部 path 依赖：shared
- 关键外部依赖：axum、deadpool-postgres、tokio

## REST Endpoints



- `POST /jaxrs/processplatform/assemble/surface/create`
- `POST /jaxrs/processplatform/assemble/surface/delete/{id}`
- `GET /jaxrs/processplatform/assemble/surface/get/{id}`
- `GET /jaxrs/processplatform/assemble/surface/list/{category}`
- `GET /jaxrs/processplatform/assemble/surface/preview/{id}`
- `POST /jaxrs/processplatform/assemble/surface/publish/{id}`
- `POST /jaxrs/processplatform/assemble/surface/save/{id}`
