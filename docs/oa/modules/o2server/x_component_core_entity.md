# o2server

## Responsibility

组件核心实体模块，定义组件数据模型和 CRUD 能力。

## Core Classes and Interfaces

- com.x.component.core.entity.Component
- com.x.component.core.entity.Component_
- com.x.component.core.entity.PersistenceProperties

## Key Flows

- 组件全量列表：`GET /jaxrs/component/core/entity/list/all` → `component_list_all` 原生 SQL 查 `CPT_COMPONENT` 过滤 `deleted_at IS NULL`，order_number 升序；order_number 为 NULL 时输出中省略 orderNumber 键
- 组件查询：`GET .../{flag}` → `component_get` 以 `WHERE (id = $1 OR name = $1) AND deleted_at IS NULL` 支持 id 或 name 双匹配（query_opt），未命中返回 AppError::NotFound
- 组件计数：`GET .../count` → `component_count` 执行 `SELECT COUNT(*) FROM CPT_COMPONENT WHERE deleted_at IS NULL` 返回 `{count}`
- 路由注册：`component_core_entity_router(pool)` 挂上述三条 GET 路由并以 `Extension(pool)` 注入连接池；routes.rs 委托回 lib.rs

## Dependencies



- x_base_core_project

**Rust（oa4rust/crates/component_core_entity）：**

- 内部 path 依赖：shared
- 关键外部依赖：axum、tokio、deadpool-postgres、serde/serde_json、uuid、tower

## REST Endpoints

<!-- Generated from Swagger annotations or action JSON files. Omit this section if the module has no REST endpoints. -->

- [Endpoint list]
