# o2server

## Responsibility

组织认证模块，负责用户登录、登出、会话管理、OAuth 第三方登录及验证码功能。

## Core Classes and Interfaces

- com.x.organization.assemble.authentication.AbstractFactory
- com.x.organization.assemble.authentication.ApplicationServletContextListener
- com.x.organization.assemble.authentication.Business
- com.x.organization.assemble.authentication.ThisApplication
- com.x.organization.assemble.authentication.factory.BindFactory
- com.x.organization.assemble.authentication.factory.IdentityFactory
- com.x.organization.assemble.authentication.factory.PersonFactory
- com.x.organization.assemble.authentication.factory.RoleFactory
- com.x.organization.assemble.authentication.jaxrs.ActionApplication
- com.x.organization.assemble.authentication.jaxrs.AndFxJaxrsFilter

## Dependencies



- x_base_core_project
- x_organization_core_entity
- x_organization_core_express

## REST Endpoints



- `GET /jaxrs/organization/assemble/authentication/identity/{id}`
- `GET /jaxrs/organization/assemble/authentication/person/{id}/icon`
