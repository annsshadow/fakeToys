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

## Key Flows

- 查询执行：`POST /jaxrs/query/core/express/execute` → `execute_query` 用 sqlparser（PostgreSqlDialect）解析并仅放行单条 SELECT，自动补 LIMIT 500
- 权限注入：execute_query 从 Session 取 person_unique，经 `get_permission_filters` 联查 auth_identity/auth_person_identity/auth_unit 生成 identity/unit 过滤，`inject_where` 将条件并入 WHERE（已有 LIMIT 则插在其前）
- 查询历史：`GET .../history/{limit}` → `get_query_history` 按 create_time 倒序读 x_query_import_record，输出 query/executedAt
- 结果缓存：`POST .../cache/{queryId}` → 校验 x_query 存在后写 x_query_import_record（ttl 默认 3600）；`GET .../cache/status/{queryId}` 以同名记录数返回 cached/hits/misses
- 路由注册：`query_core_express_router(pool)` 挂 execute/history/cache/cache-status 共 4 条路由并以 `.layer(Extension(pool))` 注入连接池；routes.rs 委托回 lib.rs

## Dependencies



- x_base_core_project
- x_query_core_entity
- x_cms_core_entity
- x_processplatform_core_entity
- x_organization_core_express

**Rust（oa4rust/crates/query_core_express）：**

- 内部 path 依赖：shared
- 关键外部依赖：axum、tokio、deadpool-postgres、serde/serde_json、sqlparser、chrono、uuid、bcrypt、base64、md5、urlencoding、anyhow、tower

## REST Endpoints



- `GET /jaxrs/query/core/express/cache/status/{queryId}`
- `POST /jaxrs/query/core/express/cache/{queryId}`
- `POST /jaxrs/query/core/express/execute`
- `GET /jaxrs/query/core/express/history/{limit}`
