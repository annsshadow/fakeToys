# o2server

## Responsibility

程序中心模块，提供应用程序管理、脚本集合和配置功能。

## Core Classes and Interfaces

- com.x.program.center.AbstractFactory
- com.x.program.center.AgentEvalResources
- com.x.program.center.ApplicationServletContextListener
- com.x.program.center.Business
- com.x.program.center.CenterQueue
- com.x.program.center.CenterQueueBody
- com.x.program.center.CenterQueueRefreshBody
- com.x.program.center.CenterQueueRegistApplicationsBody
- com.x.program.center.CompareServiceModule
- com.x.program.center.Context

## Key Flows

- 应用与风格：`GET /jaxrs/program/applications`、`appstyle/current/style` 查 `x_applications`（name/app_id/disable），current_style 取前 3 条派生 portalList；`center/regist_applications` 注册应用
- Agent 与脚本：`agent/*` 系列查 `x_program_agent`（deleted_at IS NULL）取 flag 并支持 enable/disable/execute；`script/list|paging|flag|id` 查 `x_program_script`
- 数据结构与定时任务：`datastructure/modules/all` 联查 `x_program_module` LEFT JOIN `x_program_field` 统计字段数并按 entity 映射 className；`schedule/fire/{id}` 向 `x_program_schedule_log` INSERT 'fired' 记录，schedule_report/list 汇总日志

## Dependencies



- x_base_core_project
- x_program_center_core_entity
- x_organization_core_entity
- x_cms_core_entity
- x_portal_core_entity
- x_processplatform_core_entity
- x_general_core_entity
- x_query_core_entity
- x_organization_core_express
- x_message_core_entity
- mysql-connector-j

**Rust（oa4rust/crates/program_center）：**

- 内部 path 依赖：shared
- 关键外部依赖：axum、tokio、deadpool-postgres、bcrypt、base64、uuid、chrono、md5、urlencoding

## REST Endpoints



- `GET /jaxrs/program/applications`
- `GET /jaxrs/program/appstyle/current/style`
- `GET /jaxrs/program/datastructure/modules/all`
- `POST /jaxrs/program_center/agent/create`
- `POST /jaxrs/program_center/agent/save/{id}`
- `POST /jaxrs/program_center/application/create`
- `POST /jaxrs/program_center/application/save/{id}`
