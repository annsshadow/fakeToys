# o2server

## Responsibility

流程平台服务处理模块，处理流程实例的创建、执行和取消等操作。

## Core Classes and Interfaces

- com.x.processplatform.service.processing.AbstractFactory
- com.x.processplatform.service.processing.ApplicationServletContextListener
- com.x.processplatform.service.processing.BaseProcessing
- com.x.processplatform.service.processing.Business
- com.x.processplatform.service.processing.ExceptionRecordProcessing
- com.x.processplatform.service.processing.MessageFactory
- com.x.processplatform.service.processing.Processing
- com.x.processplatform.service.processing.ProcessingToProcessingSignalStack
- com.x.processplatform.service.processing.ProcessPlatformKeyClassifyExecutorFactory
- com.x.processplatform.service.processing.SerialBuilder

## Dependencies



- x_base_core_project
- x_organization_core_entity
- x_organization_core_express
- x_query_core_entity
- x_processplatform_core_entity
- x_processplatform_core_express
- x_portal_core_entity
- x_program_center_core_entity
- x_cms_core_entity

## REST Endpoints



- `POST /jaxrs/processplatform/service/processing/cancel/{executionId}`
- `POST /jaxrs/processplatform/service/processing/create`
- `POST /jaxrs/processplatform/service/processing/execute/{id}`
- `GET /jaxrs/processplatform/service/processing/get/{id}`
- `GET /jaxrs/processplatform/service/processing/instance/{executionId}`
- `GET /jaxrs/processplatform/service/processing/list/{category}`
- `GET /jaxrs/processplatform/service/processing/process/{id}/complex`
- `GET /jaxrs/processplatform/service/processing/work/list`
- `PUT /jaxrs/work/{id}/processing`
- `POST /jaxrs/work/{id}/retract`
- `POST /jaxrs/work/{id}/terminate`
