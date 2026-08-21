# o2server

## Responsibility

CMS 核心实体模块，定义栏目、应用、配置等 CMS 数据模型。

## Core Classes and Interfaces

- com.x.cms.core.entity.AppInfo
- com.x.cms.core.entity.AppInfoConfig
- com.x.cms.core.entity.AppInfoConfig_
- com.x.cms.core.entity.AppInfo_
- com.x.cms.core.entity.CategoryExt
- com.x.cms.core.entity.CategoryExt_
- com.x.cms.core.entity.CategoryInfo
- com.x.cms.core.entity.CategoryInfo_
- com.x.cms.core.entity.CmsBatchOperation
- com.x.cms.core.entity.CmsBatchOperation_

## Key Flows

- 栏目列表：`GET /jaxrs/cms/category/list` → `category_list` 查 `x_cms_category` 过滤 DeletedAt IS NULL，SortOrder 升序 limit 20，输出 id/name/parentId/sortOrder/status/createTime
- 栏目查询与创建：`GET .../category/{id}`（find_by_id + DeletedAt IS NULL，无则 error("category not found")）；`POST .../category/create` → `category_create` uuid v4、parentId 键名为带引号的 `"\"parentId\""`、status 默认 "active"、create_time=Utc now
- 文章列表：`GET .../article/list` → `article_list` 查 `x_cms_article` 过滤 DeletedAt IS NULL，CreateTime 倒序 limit 20，输出含 categoryId/title/content/authorId/status/publishTime
- 文章查询与创建：`GET .../article/{id}`（无则 error("article not found")）；`POST .../article/create` → `article_create` status 默认 "draft"、publish_time 初始 NULL
- 路由注册：`cms_core_entity_router(_pool)` 挂 category/article 各 3 条共 6 条路由；routes.rs 委托回 lib.rs

## Dependencies



- x_base_core_project
- x_query_core_entity

**Rust（oa4rust/crates/cms_core_entity）：**

- 内部 path 依赖：shared
- 关键外部依赖：axum、tokio、sea-orm、deadpool-postgres、serde/serde_json、uuid、chrono、tower

## REST Endpoints

<!-- Generated from Swagger annotations or action JSON files. Omit this section if the module has no REST endpoints. -->

- [Endpoint list]
