# o2server

## Responsibility

查询服务处理模块，提供查询执行、生成和列表管理能力。

## Core Classes and Interfaces

- com.x.query.service.processing.AbstractFactory
- com.x.query.service.processing.ApplicationServletContextListener
- com.x.query.service.processing.Business
- com.x.query.service.processing.IndexWriteQueue
- com.x.query.service.processing.ThisApplication
- com.x.query.service.processing.factory.AppInfoFactory
- com.x.query.service.processing.factory.ApplicationFactory
- com.x.query.service.processing.factory.CategoryInfoFactory
- com.x.query.service.processing.factory.ProcessFactory
- com.x.query.service.processing.factory.QueryFactory

## Key Flows

- 单查询处理：`POST .../process` 校验 query_type 非空，查 `x_query` WHERE query_type=$1 LIMIT 1，count>0 判 processed
- 批量处理：`POST .../batch` 遍历 queries 逐条执行同上逻辑，query_type 缺失或未命中逐项写入 error，汇总 total/results
- 服务状态：`GET .../status` 统计 `x_query` 总数、pg_stat_activity 活跃连接数、`x_query_processing` 近 1 小时排队数
- 重置：`POST .../reset` DELETE `x_query_processing` 近 1 小时记录并 UPDATE `x_query` SET count='1'

## Dependencies



- x_base_core_project
- x_query_core_entity
- x_organization_core_express
- x_query_core_express
- x_processplatform_core_entity
- x_cms_core_entity
- x_cms_core_express
- x_portal_core_entity

**Rust（oa4rust/crates/query_service_processing）：**

- 内部 path 依赖：shared
- 关键外部依赖：axum、tokio、deadpool-postgres、serde/serde_json、chrono、tower

## REST Endpoints



- `POST /jaxrs/query/service/processing/batch`
- `POST /jaxrs/query/service/processing/process`
- `POST /jaxrs/query/service/processing/reset`
- `GET /jaxrs/query/service/processing/status`
