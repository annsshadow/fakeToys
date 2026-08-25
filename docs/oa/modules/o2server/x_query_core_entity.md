# o2server

## Responsibility

查询核心实体模块，定义查询项、视图和导入等查询数据模型。

## Core Classes and Interfaces

- com.x.query.core.entity.ImportModel
- com.x.query.core.entity.ImportModel_
- com.x.query.core.entity.ImportRecord
- com.x.query.core.entity.ImportRecordItem
- com.x.query.core.entity.ImportRecordItem_
- com.x.query.core.entity.ImportRecord_
- com.x.query.core.entity.Item
- com.x.query.core.entity.ItemAccess
- com.x.query.core.entity.ItemAccessActivity
- com.x.query.core.entity.ItemAccessProperties

## Key Flows

- 视图列表/详情：`GET /jaxrs/query/view/list|view/{id}` → deadpool 查 QUERY_VIEW 且 deleted_at IS NULL（list 按 create_time 倒序 limit 20），description/querySql 可选字段按存在输出；详情未命中返回 error("view not found")
- 视图创建：`POST /jaxrs/query/view/create` → `view_create` uuid v4 生成 id，status 缺省 active，INSERT 后回显 id/name/creatorId
- 查询项列表：`GET /jaxrs/query/item/list` → `item_list` 查 QUERY_ITEM 过滤软删 limit 20，输出 viewId/fieldName/dataType
- 导入记录列表：`GET /jaxrs/query/import/list` → `import_list` 查 QUERY_IMPORT，importTime 可空
- 数据模型：lib.rs 定义 QueryView/QueryItem/QueryImport 结构体，camelCase 序列化（querySql/creatorId/viewId 等）
- 路由注册：`query_core_entity_router(pool)` 挂 item/view/import 共 5 条路由并以 `.layer(Extension(pool))` 注入连接池；routes.rs 委托回 lib.rs

## Dependencies



- x_base_core_project

**Rust（oa4rust/crates/query_core_entity）：**

- 内部 path 依赖：shared
- 关键外部依赖：axum、tokio、deadpool-postgres、serde/serde_json、uuid、tower

## REST Endpoints

<!-- Generated from Swagger annotations or action JSON files. Omit this section if the module has no REST endpoints. -->

- [Endpoint list]
