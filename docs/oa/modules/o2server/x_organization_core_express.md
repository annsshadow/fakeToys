# o2server

## Responsibility

组织核心表达式模块，提供组织数据的表达式引擎和动态查询能力。

## Core Classes and Interfaces

- com.x.organization.core.express.Organization
- com.x.organization.core.express.assemble.authentication.jaxrs.authentication.ActionCaptchaLoginWi
- com.x.organization.core.express.assemble.authentication.jaxrs.authentication.ActionCaptchaRSAPublicKeyWo
- com.x.organization.core.express.assemble.authentication.jaxrs.authentication.ActionCaptchaWo
- com.x.organization.core.express.assemble.authentication.jaxrs.authentication.ActionCodeLoginWi
- com.x.organization.core.express.assemble.authentication.jaxrs.authentication.ActionLoginWi
- com.x.organization.core.express.assemble.authentication.jaxrs.authentication.ActionModeWo
- com.x.organization.core.express.assemble.authentication.jaxrs.authentication.ActionOauthGetWo
- com.x.organization.core.express.assemble.authentication.jaxrs.authentication.ActionOauthListWo
- com.x.organization.core.express.assemble.authentication.jaxrs.authentication.ActionSwitchUserWi

## Key Flows

- 服务状态：`GET /jaxrs/organization/core/express/status` → `get_status` 经 deadpool 连接池统计 `SELECT COUNT(*) FROM x_org_person / x_org_group`，并读 `x_org_config` 的 sync_enabled，输出 status="running"、personCount、groupCount、enabled
- 组织同步：`GET /jaxrs/organization/core/express/sync` → `sync_organization` 统计 x_org_person 记录数，输出 synced=count>0、syncedRecords、lastSyncTime、message="同步完成"
- 服务配置：`GET /jaxrs/organization/core/express/config` → `get_config` 读 `x_org_config WHERE config_key='sync_enabled'`，输出 enabled 与固定值 syncInterval=300、maxRecords=10000
- 路由注册：`organization_core_express_router(pool)` 挂 status/sync/config 共 3 条 GET 路由并以 `.layer(Extension(pool))` 注入连接池；routes.rs 委托回 lib.rs

## Dependencies



- x_base_core_project

**Rust（oa4rust/crates/organization_core_express）：**

- 内部 path 依赖：shared
- 关键外部依赖：axum、tokio、deadpool-postgres、serde/serde_json、thiserror、tracing、uuid、tower

## REST Endpoints



- `GET /jaxrs/organization/core/express/config`
- `GET /jaxrs/organization/core/express/status`
- `GET /jaxrs/organization/core/express/sync`
