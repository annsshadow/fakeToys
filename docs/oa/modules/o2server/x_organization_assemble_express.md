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

## Key Flows

- 表达式配置：`GET /jaxrs/organization/assemble/express/config/get` → `get_express_config` 统计 `SELECT COUNT(*) FROM ORG_UNIT WHERE deleted_at IS NULL`，输出 enabled=count>0、syncInterval=300、maxRecords=count
- 组织单元列表：`GET .../units/list` → `list_organization_units` 按 level,name 排序查 ORG_UNIT，level/superior 可选映射为 type/parent
- 数据同步：`GET .../data/sync` → `sync_organization_data` 统计 ORG_UNIT 记录数，输出 synced=count>0、syncedRecords
- 服务状态：`GET .../status/get` → `get_express_status` 无库访问，固定返回 status="running"、errors=0、warnings=0
- 路由注册：lib.rs `organization_assemble_express_router(pool)` 委托 routes.rs，挂 config/units/sync/status 共 4 条 GET 路由并以 `.layer(Extension(pool))` 注入连接池

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
