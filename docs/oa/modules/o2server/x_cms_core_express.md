# o2server

## Responsibility

CMS 核心表达式模块，提供 CMS 内容列表和查询能力。

## Core Classes and Interfaces

- com.x.cms.core.express.permission.CmsPermissionService
- com.x.cms.core.express.tools.CriteriaBuilderTools
- com.x.cms.core.express.tools.DateOperation
- com.x.cms.core.express.tools.LogUtil
- com.x.cms.core.express.tools.filter.QueryFilter
- com.x.cms.core.express.tools.filter.term.BetweenTerm
- com.x.cms.core.express.tools.filter.term.DateBetweenTerm
- com.x.cms.core.express.tools.filter.term.EqualsTerm
- com.x.cms.core.express.tools.filter.term.InTerm
- com.x.cms.core.express.tools.filter.term.IsFalseTerm

## Key Flows

- 内容列表：`GET /jaxrs/cms/core/express/content/list` → `content_list` 从 deadpool 池取连接，原生 SQL `SELECT id, title, category_id, status FROM x_cms_content ORDER BY create_time DESC LIMIT 20`，返回 `{count, data}`
- 内容详情：`GET .../content/detail/{id}` → `content_detail` 参数化查询 `WHERE id = $1`，content 列为 Option（NULL 时输出空串）；查询失败映射为 AppError::NotFound
- 路由注册：`cms_core_express_router(pool)` 挂上述两条 GET 路由并以 `Extension(pool)` 注入连接池；routes.rs 委托回 lib.rs

## Dependencies



- x_base_core_project
- x_cms_core_entity

**Rust（oa4rust/crates/cms_core_express）：**

- 内部 path 依赖：shared
- 关键外部依赖：axum、tokio、deadpool-postgres、serde/serde_json、uuid、tower

## REST Endpoints

<!-- Generated from Swagger annotations or action JSON files. Omit this section if the module has no REST endpoints. -->

- [Endpoint list]
