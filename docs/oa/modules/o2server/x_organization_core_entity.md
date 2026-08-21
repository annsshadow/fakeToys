# o2server

## Responsibility

组织核心实体模块，定义人员、单位、角色、用户组等组织数据的核心实体和基础查询。

## Core Classes and Interfaces

- com.x.organization.core.entity.Bind
- com.x.organization.core.entity.Bind_
- com.x.organization.core.entity.Custom
- com.x.organization.core.entity.Custom_
- com.x.organization.core.entity.Definition
- com.x.organization.core.entity.Definition_
- com.x.organization.core.entity.Group
- com.x.organization.core.entity.Group_
- com.x.organization.core.entity.Identity
- com.x.organization.core.entity.Identity_

## Key Flows

- 六实体统一 CRUD：definition/group/identity/person/custom/bind 各挂 list+create+`{id}` PUT/DELETE；列表均过滤 DeletedAt IS NULL（definition/group/identity/person Name 升序 limit 20，bind CreateTime 倒序，custom 按 `GET .../custom/list/{identityId}` 过滤 IdentityId）
- 创建校验：definition 要求 name/category/type 非空否则 error("name, category and type are required")；person 要求 name 非空；group/identity/custom/bind 各有对应必填字段；主键均为 uuid v4
- 更新与删除：PUT `{id}` 为部分更新（Some 字段才覆盖），find_by_id 无则 AppError::NotFound；DELETE 为软删 deleted_at=Utc now
- 可选字段输出：person 的 mobile/email、group 的 parentId、bind 的 role 经 `option_to_json` 为 None 时省略键
- 路由注册：`organization_core_entity_router(_pool)` 挂 definition/group/identity/person 各 3 条 + custom 3 条 + bind 3 条共 18 条路由；DatabaseConnection 由外部以 Extension 注入

## Dependencies



- x_base_core_project

**Rust（oa4rust/crates/organization_core_entity）：**

- 内部 path 依赖：shared、orm
- 关键外部依赖：axum、tokio、sea-orm、deadpool-postgres、serde/serde_json、uuid、chrono、tower

## REST Endpoints

<!-- Generated from Swagger annotations or action JSON files. Omit this section if the module has no REST endpoints. -->

- [Endpoint list]
