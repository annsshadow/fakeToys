# o2server

## Responsibility

关联关系服务处理模块，处理关联数据的创建、保存和删除。

## Core Classes and Interfaces

- com.x.correlation.service.processing.AbstractFactory
- com.x.correlation.service.processing.ApplicationServletContextListener
- com.x.correlation.service.processing.Business
- com.x.correlation.service.processing.ThisApplication
- com.x.correlation.service.processing.jaxrs.ActionApplication
- com.x.correlation.service.processing.jaxrs.CorrelationJaxrsFilter
- com.x.correlation.service.processing.jaxrs.correlation.ActionCreateTypeCms
- com.x.correlation.service.processing.jaxrs.correlation.ActionCreateTypeProcessPlatform
- com.x.correlation.service.processing.jaxrs.correlation.ActionDeleteTypeCms
- com.x.correlation.service.processing.jaxrs.correlation.ActionDeleteTypeProcessPlatform

## Key Flows

- 关联 CRUD：`POST .../create` INSERT `x_correlation`（uuid、creator 固定 'system'）；`save/{id}` 更新 target_id/type；`delete/{id}` 物理删除；`list/{personId}` 按 person_id 倒序；`{id}` 单条读
- link/unlink：`POST .../link` 按 type+person_id+target_id 探测返回 linked；`GET link/{sourceType}/{sourceId}` 读首条；`unlink/{sourceType}/{sourceId}/{targetType}/{targetId}` 同条件 DELETE
- 类型化关联（cms/document、processplatform/job）：`correlation/list/type/...` 按 type+target_id 倒序；`update/type/...` 更新 person_id/type；`delete/type/...` 按 target_id 删除；`readable/type/cms|processplatform` 以 LIKE 'cms/%'/'processplatform/%' 计数判 readable

## Dependencies



- x_base_core_project
- x_organization_core_entity
- x_organization_core_express
- x_processplatform_core_entity
- x_cms_core_entity
- x_correlation_core_entity
- x_correlation_core_express

**Rust（oa4rust/crates/correlation_service_processing）：**

- 内部 path 依赖：shared
- 关键外部依赖：axum、tokio、deadpool-postgres、serde/serde_json、uuid、tower

## REST Endpoints



- `POST /jaxrs/correlation/service/processing/correlation/delete/type/cms/document/{document}`
- `POST /jaxrs/correlation/service/processing/correlation/delete/type/processplatform/job/{job}`
- `GET /jaxrs/correlation/service/processing/correlation/list/type/cms/document/{document}`
- `GET /jaxrs/correlation/service/processing/correlation/list/type/cms/document/{document}/site/{site}`
- `GET /jaxrs/correlation/service/processing/correlation/list/type/processplatform/job/{job}`
- `GET /jaxrs/correlation/service/processing/correlation/list/type/processplatform/job/{job}/site/{site}`
- `GET /jaxrs/correlation/service/processing/correlation/readable/type/cms`
- `GET /jaxrs/correlation/service/processing/correlation/readable/type/processplatform`
- `GET /jaxrs/correlation/service/processing/correlation/type/cms/document/{document}`
- `GET /jaxrs/correlation/service/processing/correlation/type/processplatform/job/{job}`
- `POST /jaxrs/correlation/service/processing/correlation/update/type/cms/document/{document}`
- `POST /jaxrs/correlation/service/processing/correlation/update/type/processplatform/job/{job}`
- `POST /jaxrs/correlation/service/processing/create`
- `POST /jaxrs/correlation/service/processing/delete/{id}`
- `POST /jaxrs/correlation/service/processing/link`
- `GET /jaxrs/correlation/service/processing/link/{sourceType}/{sourceId}`
- `GET /jaxrs/correlation/service/processing/list/{personId}`
- `POST /jaxrs/correlation/service/processing/save/{id}`
- `POST /jaxrs/correlation/service/processing/unlink/{sourceType}/{sourceId}/{targetType}/{targetId}`
- `GET /jaxrs/correlation/service/processing/{id}`
