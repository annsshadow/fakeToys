# o2server

## Responsibility

关联关系核心实体模块，定义关联关系数据模型。

## Core Classes and Interfaces

- com.x.correlation.core.entity.PersistenceProperties
- com.x.correlation.core.entity.content.Correlation
- com.x.correlation.core.entity.content.CorrelationProperties
- com.x.correlation.core.entity.content.Correlation_

## Key Flows

- 关联列表：`GET /jaxrs/correlation/core/entity/list` → `list` 查 `x_corr_c_correlation`，CreateTime 倒序 limit 20，输出 id/sourceType/sourceId/targetType/targetId/weight
- 按源查询：`GET .../list/by/{sourceType}/{sourceId}` → `list_by_source` 过滤 SourceType+SourceId，Weight 倒序 limit 20
- 创建关联：`POST .../create` → `create` uuid v4、weight 默认 0、create_time=Utc now、deleted_at 初始 None
- 删除关联：`DELETE .../delete/{id}` → find_by_id 无则 error("correlation not found")；软删置 deleted_at=Utc now
- 路由注册：`correlation_core_entity_router(_pool)` 挂 list/by-source/create/delete 共 4 条路由；routes.rs 委托回 lib.rs

## Dependencies



- x_base_core_project

**Rust（oa4rust/crates/correlation_core_entity）：**

- 内部 path 依赖：shared
- 关键外部依赖：axum、tokio、sea-orm、deadpool-postgres、serde/serde_json、uuid、chrono、tower

## REST Endpoints

<!-- Generated from Swagger annotations or action JSON files. Omit this section if the module has no REST endpoints. -->

- [Endpoint list]
