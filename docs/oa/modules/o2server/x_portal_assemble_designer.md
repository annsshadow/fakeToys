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

## Dependencies



- x_base_core_project
- x_portal_core_entity
- x_organization_core_express
- x_general_core_entity

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
