# o2server

## Responsibility

关联关系核心表达式模块，提供关联状态和同步能力。

## Core Classes and Interfaces

- com.x.correlation.core.express.service.processing.jaxrs.correlation.ActionCreateTypeCmsWi
- com.x.correlation.core.express.service.processing.jaxrs.correlation.ActionCreateTypeCmsWo
- com.x.correlation.core.express.service.processing.jaxrs.correlation.ActionCreateTypeProcessPlatformWi
- com.x.correlation.core.express.service.processing.jaxrs.correlation.ActionCreateTypeProcessPlatformWo
- com.x.correlation.core.express.service.processing.jaxrs.correlation.ActionDeleteTypeCmsWi
- com.x.correlation.core.express.service.processing.jaxrs.correlation.ActionDeleteTypeCmsWo
- com.x.correlation.core.express.service.processing.jaxrs.correlation.ActionDeleteTypeProcessPlatformWi
- com.x.correlation.core.express.service.processing.jaxrs.correlation.ActionDeleteTypeProcessPlatformWo
- com.x.correlation.core.express.service.processing.jaxrs.correlation.ActionListTypeCmsWithSiteWo
- com.x.correlation.core.express.service.processing.jaxrs.correlation.ActionListTypeCmsWo

## Key Flows

- 服务状态：`GET /jaxrs/correlation/core/express/status` → `get_status` 经 deadpool 连接池 `SELECT COUNT(*) FROM x_correlation`，输出 status="running"、totalRecords、enabled=count>0
- 关联同步：`GET /jaxrs/correlation/core/express/sync` → `sync_correlation` 统计 x_correlation 记录数，输出 synced=count>0、syncedRecords、message="同步完成"
- 路由注册：`correlation_core_express_router(pool)` 挂 status/sync 共 2 条路由并以 `.layer(Extension(pool))` 注入连接池；routes.rs 委托回 lib.rs

## Dependencies



- x_base_core_project
- x_query_core_entity
- x_cms_core_entity
- x_processplatform_core_entity
- x_correlation_core_entity

**Rust（oa4rust/crates/correlation_core_express）：**

- 内部 path 依赖：shared
- 关键外部依赖：axum、tokio、deadpool-postgres、serde/serde_json、uuid、tower

## REST Endpoints



- `GET /jaxrs/correlation/core/express/status`
- `GET /jaxrs/correlation/core/express/sync`
