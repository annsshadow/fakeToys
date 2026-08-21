# o2server

## Responsibility

组件管控模块，处理应用中心、市场配置和部署管理。

## Core Classes and Interfaces

- com.x.component.assemble.control.AbstractFactory
- com.x.component.assemble.control.ApplicationServletContextListener
- com.x.component.assemble.control.Business
- com.x.component.assemble.control.ThisApplication
- com.x.component.assemble.control.factory.ComponentFactory
- com.x.component.assemble.control.jaxrs.ActionApplication
- com.x.component.assemble.control.jaxrs.ComponentJaxrsFilter
- com.x.component.assemble.control.jaxrs.StatusJaxrsFilter
- com.x.component.assemble.control.jaxrs.component.ActionCreate
- com.x.component.assemble.control.jaxrs.component.ActionDelete

## Key Flows

- 组件 CRUD：`create|save|delete` 操作 `x_component`（name/type，软删 deleted_at）；`delete/all` 批量软删；`status/list` 统计 total/active/deleted
- 分类统计：`list/control/categories` 对 `CPT_COMPONENT` DISTINCT type 并逐类 COUNT 判定 enabled，type='system' 映射为 System Components
- 控制配置：`get/update control/config` 对 `x_component_assemble_control_config`（id='default'）upsert enabled/maxComponentCount/allowCustomComponents

## Dependencies



- x_base_core_project
- x_organization_core_entity
- x_organization_core_express
- x_component_core_entity

**Rust（oa4rust/crates/component_assemble_control）：**

- 内部 path 依赖：shared
- 关键外部依赖：axum、tokio、deadpool-postgres、serde/serde_json、uuid、tower

## REST Endpoints



- `POST /jaxrs/component/assemble/control/component/delete/all`
- `GET /jaxrs/component/assemble/control/status/list`
- `GET /jaxrs/component_assemble_control/create/component`
- `GET /jaxrs/component_assemble_control/delete/component`
- `GET /jaxrs/component_assemble_control/get/component`
- `GET /jaxrs/component_assemble_control/get/control/config`
- `GET /jaxrs/component_assemble_control/list/components`
- `GET /jaxrs/component_assemble_control/list/control/categories`
- `GET /jaxrs/component_assemble_control/save/component`
- `GET /jaxrs/component_assemble_control/update/control/config`
