# o2server

## Responsibility

程序中心核心实体模块，定义应用程序、脚本、调用、代理和结构数据模型。

## Core Classes and Interfaces

- com.x.program.center.core.entity.Agent
- com.x.program.center.core.entity.Agent_
- com.x.program.center.core.entity.Application
- com.x.program.center.core.entity.Application_
- com.x.program.center.core.entity.AppPackApkFile
- com.x.program.center.core.entity.AppPackApkFile_
- com.x.program.center.core.entity.Attachment
- com.x.program.center.core.entity.Attachment_
- com.x.program.center.core.entity.Captcha
- com.x.program.center.core.entity.Captcha_

## Key Flows

- 异步路由组装：`program_center_core_entity_router(pool)` 为 async fn（routes.rs 同样 async 委托），先经 `shared::db::create_sea_orm_pool()` 取 SeaORM 连接，再 merge 5 个子路由
- 五域子路由：application/script/invoke/agent/structure 各自 `_router(pool, db)` 注册 list(GET)/创建(POST)/`{id}` 更新(PUT)+删除(DELETE)，路径前缀 `/jaxrs/program_center/<domain>`
- 字段长度约束：lib.rs 定义 MAX_NAME_LEN=200、MAX_TEXT_LEN=500、MAX_LONG_TEXT_LEN=2000 供实体校验复用
- 测试替身：`#[cfg(test)] program_center_mock_router` 以固定 "ok" 响应镜像 5 个 list 端点供单测使用

## Dependencies



- x_base_core_project
- x_general_core_entity

**Rust（oa4rust/crates/program_center_core_entity）：**

- 内部 path 依赖：shared、file_assemble_control
- 关键外部依赖：axum、tokio、sea-orm、deadpool-postgres、serde/serde_json、chrono、uuid、tower

## REST Endpoints

<!-- Generated from Swagger annotations or action JSON files. Omit this section if the module has no REST endpoints. -->

- [Endpoint list]
