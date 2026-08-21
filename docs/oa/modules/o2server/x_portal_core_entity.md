# o2server

## Responsibility

门户核心实体模块，定义门户页面和部件的数据模型。

## Core Classes and Interfaces

- com.x.portal.core.entity.File
- com.x.portal.core.entity.File_
- com.x.portal.core.entity.Page
- com.x.portal.core.entity.PageProperties
- com.x.portal.core.entity.PageVersion
- com.x.portal.core.entity.PageVersion_
- com.x.portal.core.entity.Page_
- com.x.portal.core.entity.PersistenceProperties
- com.x.portal.core.entity.Portal
- com.x.portal.core.entity.PortalProperties

## Key Flows

- 门户/部件列表：`GET /jaxrs/portal/portal/list|widget/list` → SeaORM `portal::Entity::find()` / `widget::Entity::find()` 按 Name 升序，输出 portalCategory、widget 的 category/portal 归属
- 页面列表/详情：`GET /jaxrs/portal/page/list|page/{id}` → `page_list`/`page_get` 过滤 DeletedAt IS NULL（list 按 CreateTime 倒序），详情未命中返回 error("page not found")
- 页面创建：`POST /jaxrs/portal/page/create` → `page_create` 校验 name 必填（缺失报 BadRequest），uuid v4 生成 id，status 默认 active 后 `ActiveModel::insert`
- 页面更新/删除：`POST /jaxrs/portal/page/update|remove` → update 走原生 SQL 更新 name/content/status 且限定 deleted_at IS NULL，rows_affected=0 报 NotFound；remove 置 deleted_at=Utc now 软删
- 脚本列表：`GET /jaxrs/portal/script/list` → `script_list` 按 Name 升序输出 id/name/alias/validated
- 路由注册：`portal_core_entity_router(_pool)` 挂 portal/widget/page/script 共 8 条路由；routes.rs 委托回 lib.rs

## Dependencies



- x_base_core_project
- x_general_core_entity

**Rust（oa4rust/crates/portal_core_entity）：**

- 内部 path 依赖：shared
- 关键外部依赖：axum、tokio、sea-orm、deadpool-postgres、serde/serde_json、chrono、uuid、tower

## REST Endpoints

<!-- Generated from Swagger annotations or action JSON files. Omit this section if the module has no REST endpoints. -->

- [Endpoint list]
