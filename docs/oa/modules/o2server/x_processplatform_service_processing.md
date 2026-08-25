# o2server

## Responsibility

流程平台服务处理模块，处理流程实例的创建、执行和取消等操作。

## Core Classes and Interfaces

- com.x.processplatform.service.processing.AbstractFactory
- com.x.processplatform.service.processing.ApplicationServletContextListener
- com.x.processplatform.service.processing.BaseProcessing
- com.x.processplatform.service.processing.Business
- com.x.processplatform.service.processing.ExceptionRecordProcessing
- com.x.processplatform.service.processing.MessageFactory
- com.x.processplatform.service.processing.Processing
- com.x.processplatform.service.processing.ProcessingToProcessingSignalStack
- com.x.processplatform.service.processing.ProcessPlatformKeyClassifyExecutorFactory
- com.x.processplatform.service.processing.SerialBuilder

## Key Flows

- 流程启动/完成：`POST /jaxrs/work/{id}/start|complete` → 事务内 `FOR UPDATE` 状态机校验（start 要求 pending、complete 要求 processing），更新 x_work 并插入 "Start Event" 任务或批量完成 x_task 后写入 x_workcompleted
- 流程创建/执行：`POST .../create|execute/{id}` → `create_process` uuid v4 插入 x_work（status=pending）；`execute_process` 事务内置 processing 并创建 start 活动 x_task
- 任务生命周期：`POST /jaxrs/task/{id}/claim|complete|reject|transfer/{person}` → FOR UPDATE 校验后更新 task_status/person，写 x_record 审计；complete 自动激活同 activity_token 的下一个 pending 任务，reject 回退前序 completed 任务
- 网关聚合/分叉：`POST /jaxrs/gateway/{work_id}/{activity_token}/join|.../gateway/fork/{gateway_instance_id}` → join 校验同 token 全部 completed 才写 gateway_join 记录；fork 按 x_process_transition 为每条转移创建 pending 任务
- 定时器：`POST .../timer/start|timer/{job_id}/cancel` → TimerRegistry 内存 HashMap 与 x_timer_job 表双写，后台协程每 30s tick 触发到期 expire 作业并标 fired_at
- 快照与回滚：`GET .../snap/restore/{id}|v2/rollback/{work}/{id}` → 读 x_snap 最新 snap_data，事务内恢复 work_status 并写 restore/rollback 记录
- 路由注册：routes.rs `router(pool)` 注册约 80 条 get/post/put 路由，构建时启动 `TimerRegistry::start_background()` 并以两层 Extension 注入 pool 与 timer

## Dependencies



- x_base_core_project
- x_organization_core_entity
- x_organization_core_express
- x_query_core_entity
- x_processplatform_core_entity
- x_processplatform_core_express
- x_portal_core_entity
- x_program_center_core_entity
- x_cms_core_entity

**Rust（oa4rust/crates/processplatform_service_processing）：**

- 内部 path 依赖：shared
- 关键外部依赖：axum、tokio、deadpool-postgres、serde/serde_json、chrono、uuid、anyhow、bcrypt、base64、md5、urlencoding、tower

## REST Endpoints



- `POST /jaxrs/processplatform/service/processing/cancel/{executionId}`
- `POST /jaxrs/processplatform/service/processing/create`
- `POST /jaxrs/processplatform/service/processing/execute/{id}`
- `GET /jaxrs/processplatform/service/processing/get/{id}`
- `GET /jaxrs/processplatform/service/processing/instance/{executionId}`
- `GET /jaxrs/processplatform/service/processing/list/{category}`
- `GET /jaxrs/processplatform/service/processing/process/{id}/complex`
- `GET /jaxrs/processplatform/service/processing/work/list`
- `PUT /jaxrs/work/{id}/processing`
- `POST /jaxrs/work/{id}/retract`
- `POST /jaxrs/work/{id}/terminate`
