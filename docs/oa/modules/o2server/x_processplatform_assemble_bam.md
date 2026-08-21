# o2server

## Responsibility

流程平台 BAM 管控模块，处理流程监控配置和统计报表。

## Core Classes and Interfaces

- com.x.processplatform.assemble.bam.AbstractFactory
- com.x.processplatform.assemble.bam.ApplicationServletContextListener
- com.x.processplatform.assemble.bam.Business
- com.x.processplatform.assemble.bam.ThisApplication
- com.x.processplatform.assemble.bam.factory.TaskCompletedFactory
- com.x.processplatform.assemble.bam.factory.TaskDurationWithPeriodCountObject
- com.x.processplatform.assemble.bam.factory.TaskFactory
- com.x.processplatform.assemble.bam.factory.WorkCompletedFactory
- com.x.processplatform.assemble.bam.factory.WorkDurationWithPeriodCountObject
- com.x.processplatform.assemble.bam.factory.WorkFactory

## Dependencies



- x_base_core_project
- x_organization_core_entity
- x_organization_core_express
- x_query_core_entity
- x_processplatform_core_entity

## REST Endpoints



- `POST /jaxrs/processplatform/assemble/bam/create`
- `POST /jaxrs/processplatform/assemble/bam/delete/{id}`
- `GET /jaxrs/processplatform/assemble/bam/get/{id}`
- `GET /jaxrs/processplatform/assemble/bam/list/{category}`
- `GET /jaxrs/processplatform/assemble/bam/status/{id}`
