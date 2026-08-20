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

## Dependencies



- x_base_core_project
- x_organization_core_entity
- x_organization_core_express
- x_component_core_entity

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
