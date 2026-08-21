# o2server

## Responsibility

组织表达式模块，提供组织相关的脚本表达式和动态处理能力。

## Core Classes and Interfaces

- com.x.organization.assemble.express.AbstractFactory
- com.x.organization.assemble.express.ApplicationServletContextListener
- com.x.organization.assemble.express.Business
- com.x.organization.assemble.express.ThisApplication
- com.x.organization.assemble.express.factory.GroupFactory
- com.x.organization.assemble.express.factory.IdentityFactory
- com.x.organization.assemble.express.factory.PersonAttributeFactory
- com.x.organization.assemble.express.factory.PersonFactory
- com.x.organization.assemble.express.factory.RoleFactory
- com.x.organization.assemble.express.factory.TrustFactory

## Dependencies



- x_base_core_project
- x_organization_core_entity

**Rust（oa4rust/crates/organization_assemble_express）：**

- 内部 path 依赖：shared
- 关键外部依赖：axum、tokio、deadpool-postgres、serde/serde_json、uuid、tower

## REST Endpoints



- `GET /jaxrs/organization/assemble/express/config/get`
- `GET /jaxrs/organization/assemble/express/data/sync`
- `GET /jaxrs/organization/assemble/express/status/get`
- `GET /jaxrs/organization/assemble/express/units/list`
