# o2server

## Responsibility

流程平台 BAM 管控模块，处理流程监控配置和统计报表。

## Core Classes and Interfaces

- com.x.processplatform.assemble.bam.AbstractFactory
- com.x.processplatform.assemble.bam.ApplicationServletContextListener
- com.x.processplatform.assemble.bam.Business
- com.x.processplatform.assemble.bam.ThisApplication
- com.x.processplatform.assemble.bam.factory.TaskCompletedFactory
- com.x.processplatform.assemble.bam.factory.TaskDurationWithPeriodCountObject
- com.x.processplatform.assemble.bam.factory.TaskFactory
- com.x.processplatform.assemble.bam.factory.WorkCompletedFactory
- com.x.processplatform.assemble.bam.factory.WorkDurationWithPeriodCountObject
- com.x.processplatform.assemble.bam.factory.WorkFactory

## Key Flows

- BAM 配置管理：`POST .../create` INSERT `x_bam_config`（uuid 主键、xenabled=true）；`get/{id}` 读 xname/xdefinition/xenabled；`list/{category}` 按 xcategory 过滤 ORDER BY "xcreateTime" DESC LIMIT 100；`delete/{id}` 物理删除
- 运行状态：`GET .../status/{id}` COUNT `x_pp_c_task` WHERE xbamConfig=$1，>0 判 running 否则 idle
- 周期统计：`period/list/completed|expired|task|application/...` 对 `x_task` JOIN `x_work` 按 task_status/work_status（completed/expired/started）与 application/process/activity/person/unit 维度 GROUP BY 计数
- 状态触发：`POST .../state/trigger/{category}`

## Dependencies



- x_base_core_project
- x_organization_core_entity
- x_organization_core_express
- x_query_core_entity
- x_processplatform_core_entity

**Rust（oa4rust/crates/processplatform_assemble_bam）：**

- 内部 path 依赖：shared
- 关键外部依赖：axum、tokio、deadpool-postgres、serde/serde_json、uuid、chrono、tower

## REST Endpoints



- `POST /jaxrs/processplatform/assemble/bam/create`
- `POST /jaxrs/processplatform/assemble/bam/delete/{id}`
- `GET /jaxrs/processplatform/assemble/bam/get/{id}`
- `GET /jaxrs/processplatform/assemble/bam/list/{category}`
- `GET /jaxrs/processplatform/assemble/bam/status/{id}`
