# o2server

## Responsibility

查询核心表达式模块，提供查询执行和历史记录能力。

## Core Classes and Interfaces

- com.x.query.core.express.assemble.surface.jaxrs.index.ActionExportWi
- com.x.query.core.express.assemble.surface.jaxrs.index.ActionPostWi
- com.x.query.core.express.assemble.surface.jaxrs.index.ActionPostWo
- com.x.query.core.express.assemble.surface.jaxrs.morelikethis.ActionPostWi
- com.x.query.core.express.assemble.surface.jaxrs.morelikethis.ActionPostWo
- com.x.query.core.express.assemble.surface.jaxrs.search.ActionPostWi
- com.x.query.core.express.assemble.surface.jaxrs.search.ActionPostWo
- com.x.query.core.express.index.Directory
- com.x.query.core.express.index.Facets
- com.x.query.core.express.index.Filter

## Dependencies



- x_base_core_project
- x_query_core_entity
- x_cms_core_entity
- x_processplatform_core_entity
- x_organization_core_express

## REST Endpoints



- `GET /jaxrs/query/core/express/cache/status/{queryId}`
- `POST /jaxrs/query/core/express/cache/{queryId}`
- `POST /jaxrs/query/core/express/execute`
- `GET /jaxrs/query/core/express/history/{limit}`
