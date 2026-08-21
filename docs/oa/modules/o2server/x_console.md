# o2server

## Responsibility

控制台模块，提供命令行、日志查看和系统监控功能。

## Core Classes and Interfaces

- com.x.server.console.AiAgent
- com.x.server.console.CleanLogTask
- com.x.server.console.CommandFactory
- com.x.server.console.CommandThreads
- com.x.server.console.ConfigFactory
- com.x.server.console.DruidStatLogger
- com.x.server.console.DumpDataTask
- com.x.server.console.InstrumentationAgent
- com.x.server.console.Main
- com.x.server.console.NodeAgent

## Key Flows

- 系统状态：`GET .../status` 读 `x_console_status`（xid='system'）返回 status/version/uptime，无记录时回退 running/1.0.0/0
- 日志与指标：`GET .../logs/{type}` 按 xtype 查 `x_console_log` ORDER BY xtimestamp DESC LIMIT 100；`metric/{name}` 读 `x_console_metric`，缺省回退 42/count
- 消息与缓存：`POST .../send/message` INSERT `x_console_message`（uuid 主键）；`cache/clear/{type}` DELETE `x_console_cache` WHERE xtype=$1
- 命令执行：`POST .../command/execute` 先 is_admin RBAC 校验，再白名单（uname/df/free/ps/uptime）+ shell 元字符黑名单双重过滤后 sh -c 执行，返回 stdout/stderr/exitCode
- 系统信息：`GET .../system/info` 经 sysinfo 输出 os/arch/cpuCores/memory

## Dependencies



- x_base_core_project
- x_bbs_core_entity
- x_program_center_core_entity
- x_organization_core_entity
- x_cms_core_entity
- x_processplatform_core_entity
- x_query_core_entity
- x_message_core_entity

**Rust（oa4rust/crates/console）：**

- 内部 path 依赖：shared
- 关键外部依赖：axum、tokio、deadpool-postgres、serde/serde_json、uuid、chrono、sysinfo、tower

## REST Endpoints



- `POST /jaxrs/console/cache/clear/{type}`
- `POST /jaxrs/console/command/execute`
- `GET /jaxrs/console/logs/{type}`
- `GET /jaxrs/console/metric/{name}`
- `POST /jaxrs/console/send/message`
- `GET /jaxrs/console/status`
- `GET /jaxrs/console/system/info`
