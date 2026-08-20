# o2server

## Responsibility

查询设计器管控模块，提供查询视图的设计、创建和管理。

## Core Classes and Interfaces

- com.x.query.assemble.designer.AbstractFactory
- com.x.query.assemble.designer.ApplicationServletContextListener
- com.x.query.assemble.designer.Business
- com.x.query.assemble.designer.CompareQuery
- com.x.query.assemble.designer.ThisApplication
- com.x.query.assemble.designer.factory.ImportModelFactory
- com.x.query.assemble.designer.factory.NeuralFactory
- com.x.query.assemble.designer.factory.ProcessFactory
- com.x.query.assemble.designer.factory.QueryFactory
- com.x.query.assemble.designer.factory.StatementFactory

## Dependencies



- x_base_core_project
- x_organization_core_entity
- x_organization_core_express
- x_query_core_entity
- x_query_core_express
- x_processplatform_core_entity
- x_cms_core_entity
- x_cms_core_express
- x_general_core_entity

## REST Endpoints



- `POST /jaxrs/query/assemble/designer/create`
- `POST /jaxrs/query/assemble/designer/delete/{id}`
- `GET /jaxrs/query/assemble/designer/get/{id}`
- `GET /jaxrs/query/assemble/designer/list/{category}`
- `POST /jaxrs/query/assemble/designer/save/{id}`
