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

## Dependencies



- x_base_core_project
- x_portal_core_entity
- x_organization_core_express
- x_processplatform_core_entity
- x_cms_core_entity
- x_general_core_entity
- x_program_center_core_entity

## REST Endpoints



- `POST /jaxrs/portal/assemble/surface/create`
- `GET /jaxrs/portal/assemble/surface/get/{id}`
- `GET /jaxrs/portal/assemble/surface/list/{category}`
- `GET /jaxrs/portal/assemble/surface/preview/{id}`
- `POST /jaxrs/portal/assemble/surface/publish/{id}`
- `GET /jaxrs/portal/surface/list`
- `POST /jaxrs/portal/surface/publish`
- `GET /jaxrs/portal/surface/{id}/preview`
