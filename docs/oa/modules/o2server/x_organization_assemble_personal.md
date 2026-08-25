# o2server

## Responsibility

organization assemble personal 模块。

## Core Classes and Interfaces

- com.x.organization.assemble.personal.AbstractFactory
- com.x.organization.assemble.personal.ApplicationServletContextListener
- com.x.organization.assemble.personal.Business
- com.x.organization.assemble.personal.ThisApplication
- com.x.organization.assemble.personal.factory.GroupFactory
- com.x.organization.assemble.personal.factory.IdentityFactory
- com.x.organization.assemble.personal.factory.PersonAttributeFactory
- com.x.organization.assemble.personal.factory.PersonFactory
- com.x.organization.assemble.personal.factory.RoleFactory
- com.x.organization.assemble.personal.factory.UnitAttributeFactory

## Key Flows

- 个人设置：`GET .../{id}/setting` 读 `auth_person` 的 mobile/email/icon/theme/lang（可空字段仅在有值时输出）
- 角色列表：`GET .../{id}/role/list` JOIN `x_org_group_member`×`x_org_role` ON m.role_id=r.id WHERE m.person_id=$1，返回 {roles:[{id,name}]}

## Dependencies



- x_base_core_project
- x_organization_core_entity

**Rust（oa4rust/crates/organization_assemble_personal）：**

- 内部 path 依赖：shared
- 关键外部依赖：axum、deadpool-postgres、serde_json、uuid

## REST Endpoints



- `GET /jaxrs/organization/assemble/personal/{id}/role/list`
- `GET /jaxrs/organization/assemble/personal/{id}/setting`
