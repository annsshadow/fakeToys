# o2server

## Responsibility

流程平台设计器管控模块，处理流程应用的预览和发布管理。

## Core Classes and Interfaces

- com.x.processplatform.assemble.designer.AbstractFactory
- com.x.processplatform.assemble.designer.ApplicationServletContextListener
- com.x.processplatform.assemble.designer.Business
- com.x.processplatform.assemble.designer.CompareApplication
- com.x.processplatform.assemble.designer.Control
- com.x.processplatform.assemble.designer.ExceptionDynamicClassNotExist
- com.x.processplatform.assemble.designer.FormVersionQueue
- com.x.processplatform.assemble.designer.MappingExecuteQueue
- com.x.processplatform.assemble.designer.MessageFactory
- com.x.processplatform.assemble.designer.ProcessVersionQueue

## Key Flows

- 流程定义 CRUD：`POST .../create` 校验 name 非空后 INSERT `x_process_definition`（uuid、version=1、creator=session.person_unique）；`get/{id}` 含 process_definition jsonb；`list/{category}` 分页（page≥1、size clamp 1..100，all 全量）；`save/{id}` 更新 process_definition + update_time；`delete/{id}` 物理删除
- 流程预览：`GET .../preview/{id}` 解析 process_definition jsonb 的 nodes/edges 并生成 preview_url
- 表单及版本：`form/{id}` 读 `PP_E_FORM`；`formversion/list/form/{formId}` 读 `PP_E_FORMVERSION` ORDER BY xversion DESC
- 流程启停：`process/enable/{id}` UPDATE `PP_E_PROCESS` SET xstatus='enabled'；`process/enabled/{id}` 仅 xstatus='enabled' 时返回 enabled=true

## Dependencies



- x_base_core_project
- x_organization_core_entity
- x_organization_core_express
- x_query_core_entity
- x_processplatform_core_entity
- x_general_core_entity

**Rust（oa4rust/crates/processplatform_assemble_designer）：**

- 内部 path 依赖：shared
- 关键外部依赖：axum、tokio、deadpool-postgres、serde/serde_json、uuid、chrono、tower

## REST Endpoints



- `POST /jaxrs/processplatform/assemble/designer/create`
- `POST /jaxrs/processplatform/assemble/designer/delete/{id}`
- `GET /jaxrs/processplatform/assemble/designer/get/{id}`
- `GET /jaxrs/processplatform/assemble/designer/list/{category}`
- `GET /jaxrs/processplatform/assemble/designer/preview/{id}`
- `POST /jaxrs/processplatform/assemble/designer/save/{id}`
