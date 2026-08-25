# o2server

## Responsibility

门户设计器模块，处理门户页面的设计和配置管理。

## Core Classes and Interfaces

- com.x.portal.assemble.designer.AbstractFactory
- com.x.portal.assemble.designer.ApplicationServletContextListener
- com.x.portal.assemble.designer.Business
- com.x.portal.assemble.designer.ComparePortal
- com.x.portal.assemble.designer.ThisApplication
- com.x.portal.assemble.designer.factory.ApplicationDictFactory
- com.x.portal.assemble.designer.factory.ApplicationDictItemFactory
- com.x.portal.assemble.designer.factory.FileFactory
- com.x.portal.assemble.designer.factory.PageFactory
- com.x.portal.assemble.designer.factory.PortalFactory

## Key Flows

- 门户设计创建与保存：`POST /jaxrs/portal/design/save`（及 `/jaxrs/portal/assemble/designer/create`）→ `create_design`（uuid v4，creator="system"）→ INSERT INTO `x_portal_design`；`POST .../designer/save/{id}` → `save_design` UPDATE `x_portal_design` SET content/update_time
- 页面 CRUD：`POST .../designer/page/create` → `create_page` INSERT INTO `x_portal_page`（name/category/content JSON 序列化存储）；`POST .../page/save/{id}` → UPDATE content；`GET .../page/{id}` → `get_page` 按 id 查询并反序列化 content
- 设计列表：`GET /jaxrs/portal/design/list` → `list_designs` 查询 `x_portal_design` WHERE deleted_at IS NULL ORDER BY update_time DESC；页面按分类浏览走 `GET .../page/list/{category}` → 查询 `x_portal_page WHERE category=$1`

## Dependencies



- x_base_core_project
- x_portal_core_entity
- x_organization_core_express
- x_general_core_entity

**Rust（oa4rust/crates/portal_assemble_designer）：**

- 内部 path 依赖：shared
- 关键外部依赖：axum、tokio、deadpool-postgres、serde/serde_json、uuid、tower、bcrypt、base64、anyhow、chrono、md5、urlencoding

## REST Endpoints



- `POST /jaxrs/portal/assemble/designer/create`
- `GET /jaxrs/portal/assemble/designer/get/{id}`
- `GET /jaxrs/portal/assemble/designer/list`
- `POST /jaxrs/portal/assemble/designer/page/create`
- `POST /jaxrs/portal/assemble/designer/page/delete/{id}`
- `GET /jaxrs/portal/assemble/designer/page/list/{category}`
- `POST /jaxrs/portal/assemble/designer/page/save/{id}`
- `GET /jaxrs/portal/assemble/designer/page/{id}`
- `POST /jaxrs/portal/assemble/designer/save/{id}`
- `GET /jaxrs/portal/design/list`
- `POST /jaxrs/portal/design/save`
- `GET /jaxrs/portal/design/{id}`
