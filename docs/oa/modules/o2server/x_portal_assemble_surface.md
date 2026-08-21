# o2server

## Responsibility

门户展现模块，处理门户页面的预览、发布和渲染。

## Core Classes and Interfaces

- com.x.portal.assemble.surface.AbstractFactory
- com.x.portal.assemble.surface.ApplicationServletContextListener
- com.x.portal.assemble.surface.Business
- com.x.portal.assemble.surface.Control
- com.x.portal.assemble.surface.ThisApplication
- com.x.portal.assemble.surface.factory.ApplicationDictFactory
- com.x.portal.assemble.surface.factory.ApplicationDictItemFactory
- com.x.portal.assemble.surface.factory.ElementFactory
- com.x.portal.assemble.surface.factory.FileFactory
- com.x.portal.assemble.surface.factory.PageFactory

## Key Flows

- 门户表面创建与浏览：`POST /jaxrs/portal/assemble/surface/create` → `create_surface`（category 缺省 "default"，html 初始 "<div></div>"，uuid v4）→ INSERT INTO `x_portal_surface`；`GET .../surface/list/{category}` → `list_surfaces` 按分类查询同表 ORDER BY update_time DESC
- 预览渲染：`GET /jaxrs/portal/surface/{id}/preview` → `preview_surface` 读取 `x_portal_surface` 的 html 字段 → 返回 preview_url=`/preview/{id}` 与 html 内容
- 发布：`POST /jaxrs/portal/surface/publish`（及 `/assemble/surface/publish/{id}`）→ `publish_surface` UPDATE `x_portal_surface` SET published=true/published_at=NOW() → 回读 published 状态返回；全量列表走 `GET /jaxrs/portal/surface/list`（WHERE deleted_at IS NULL）

## Dependencies



- x_base_core_project
- x_portal_core_entity
- x_organization_core_express
- x_processplatform_core_entity
- x_cms_core_entity
- x_general_core_entity
- x_program_center_core_entity

**Rust（oa4rust/crates/portal_assemble_surface）：**

- 内部 path 依赖：shared
- 关键外部依赖：axum、tokio、deadpool-postgres、serde/serde_json、uuid、tower、bcrypt、base64、anyhow、chrono、md5、urlencoding

## REST Endpoints



- `POST /jaxrs/portal/assemble/surface/create`
- `GET /jaxrs/portal/assemble/surface/get/{id}`
- `GET /jaxrs/portal/assemble/surface/list/{category}`
- `GET /jaxrs/portal/assemble/surface/preview/{id}`
- `POST /jaxrs/portal/assemble/surface/publish/{id}`
- `GET /jaxrs/portal/surface/list`
- `POST /jaxrs/portal/surface/publish`
- `GET /jaxrs/portal/surface/{id}/preview`
