# o2server

## Responsibility

热点图片管控模块，处理轮播配置、面板和应用管理。

## Core Classes and Interfaces

- com.x.hotpic.assemble.common.date.DateOperation
- com.x.hotpic.assemble.control.AbstractFactory
- com.x.hotpic.assemble.control.ApplicationServletContextListener
- com.x.hotpic.assemble.control.Business
- com.x.hotpic.assemble.control.ThisApplication
- com.x.hotpic.assemble.control.factory.HotPictureInfoFactory
- com.x.hotpic.assemble.control.jaxrs.ActionApplication
- com.x.hotpic.assemble.control.jaxrs.JaxrsCipherFilter
- com.x.hotpic.assemble.control.jaxrs.JaxrsManagerUserFilter
- com.x.hotpic.assemble.control.jaxrs.hotpic.ActionChangeTitle

## Key Flows

- 轮播图 CRUD：`create|save|delete|get|list` 操作 `x_hotpic`（title/image_url，软删 deleted_at）；`user/hotpic/changeTitle` 仅更新 title
- cipher/user 双通道读取：`cipher/hotpic/{id}`、`cipher/hotpic/bbs|cms/{id}` 与 `user/hotpic/filter/list/page/{page}/count/{count}` 均查 `x_hotpic`（LIMIT/OFFSET 分页）；`exists/check` 按 creator COUNT 判断是否已有图
- 面板与应用派生：`list/control/panels`、`list/control/applications` 对 `x_hotpic` DISTINCT creator 生成面板/应用列表；`config` 由记录数推导 enabled

## Dependencies



- x_base_core_project
- x_organization_core_entity
- x_organization_core_express
- x_hotpic_core_entity
- x_bbs_core_entity
- x_cms_core_entity
- x_general_core_entity

**Rust（oa4rust/crates/hotpic_assemble_control）：**

- 内部 path 依赖：shared
- 关键外部依赖：axum、tokio、deadpool-postgres、serde/serde_json、uuid、tower

## REST Endpoints



- `GET /jaxrs/hotpic/assemble/control/cipher/hotpic/bbs/{id}`
- `GET /jaxrs/hotpic/assemble/control/cipher/hotpic/cms/{id}`
- `GET /jaxrs/hotpic/assemble/control/cipher/hotpic/filter/list/page/{page}/count/{count}`
- `GET /jaxrs/hotpic/assemble/control/cipher/hotpic/{id}`
- `GET /jaxrs/hotpic/assemble/control/config`
- `GET /jaxrs/hotpic/assemble/control/list/control/applications`
- `GET /jaxrs/hotpic/assemble/control/list/control/panels`
- `POST /jaxrs/hotpic/assemble/control/update/control/config`
- `GET /jaxrs/hotpic/assemble/control/user/hotpic/application/{infoId}`
- `POST /jaxrs/hotpic/assemble/control/user/hotpic/changeTitle`
- `GET /jaxrs/hotpic/assemble/control/user/hotpic/exists/check`
- `GET /jaxrs/hotpic/assemble/control/user/hotpic/filter/list/page/{page}/count/{count}`
- `GET /jaxrs/hotpic/assemble/control/user/hotpic/{id}`
- `POST /jaxrs/hotpic/create/hotpic`
- `POST /jaxrs/hotpic/delete/hotpic`
- `GET /jaxrs/hotpic/get/hotpic/{id}`
- `GET /jaxrs/hotpic/list/hotpics`
- `POST /jaxrs/hotpic/save/hotpic`
- `GET /jaxrs/hotpic_assemble_control/cipher/hotpic/bbs/id`
- `GET /jaxrs/hotpic_assemble_control/cipher/hotpic/cms/id`
- `GET /jaxrs/hotpic_assemble_control/cipher/hotpic/filter/list/page/page/count/count`
- `GET /jaxrs/hotpic_assemble_control/cipher/hotpic/id`
- `GET /jaxrs/hotpic_assemble_control/create/hotpic`
- `GET /jaxrs/hotpic_assemble_control/delete/hotpic`
- `GET /jaxrs/hotpic_assemble_control/get/control/config`
- `GET /jaxrs/hotpic_assemble_control/get/hotpic`
- `GET /jaxrs/hotpic_assemble_control/list/control/applications`
- `GET /jaxrs/hotpic_assemble_control/list/control/panels`
- `GET /jaxrs/hotpic_assemble_control/list/hotpics`
- `GET /jaxrs/hotpic_assemble_control/save/hotpic`
- `GET /jaxrs/hotpic_assemble_control/update/control/config`
- `GET /jaxrs/hotpic_assemble_control/user/hotpic/application/infoId`
- `GET /jaxrs/hotpic_assemble_control/user/hotpic/changeTitle`
- `GET /jaxrs/hotpic_assemble_control/user/hotpic/exists/check`
- `GET /jaxrs/hotpic_assemble_control/user/hotpic/filter/list/page/page/count/count`
- `GET /jaxrs/hotpic_assemble_control/user/hotpic/id`
