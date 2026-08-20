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

## Dependencies



- x_base_core_project
- x_organization_core_entity
- x_organization_core_express
- x_processplatform_core_entity
- x_cms_core_entity
- x_correlation_core_entity
- x_correlation_core_express

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
