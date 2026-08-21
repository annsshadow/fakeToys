# o2server

## Responsibility

基础核心项目模块，提供平台公共基础设施、工具类和通用配置。

## Core Classes and Interfaces

- com.x.base.core.container.EntityManagerContainer
- com.x.base.core.container.EntityManagerContainerBasic
- com.x.base.core.container.FactorDistributionPolicy
- com.x.base.core.container.FieldType
- com.x.base.core.container.PersistChecker
- com.x.base.core.container.RemoveChecker
- com.x.base.core.container.checker.AbstractChecker
- com.x.base.core.container.checker.BooleanValueListPersistChecker
- com.x.base.core.container.checker.BooleanValuePersistChecker
- com.x.base.core.container.checker.ByteValueArrayPersistChecker

## Key Flows

- 健康检查：`GET /jaxrs/base/echo/get` → `echo_get` 无需数据库，固定返回 `{"type":"echo","message":"pong"}`（带 utoipa OpenAPI 注解）
- 缓存监控：`GET /jaxrs/base/cache/detail` → `cache_detail` 从 deadpool 池取连接，执行 `SELECT count(*) FROM pg_class WHERE relname LIKE 'cache_%'` 统计 `cache_` 前缀表数量，返回 `{status:"running", cacheCount}`
- OpenAPI 入口：`GET /jaxrs/base/openapi/info` → `openapi_info` 返回 307 Redirect 到 `/openapi.json`
- 路由注册：`base_router(pool)` 委托 `routes::build_router`，挂上述三条 GET 路由并以 `axum::Extension(pool)` 注入连接池

## Dependencies



- None listed

**Rust（oa4rust/crates/base）：**

- 内部 path 依赖：shared
- 关键外部依赖：axum、tokio、deadpool-postgres、serde/serde_json、utoipa、uuid、tower

## REST Endpoints



- *No endpoints registered.*
