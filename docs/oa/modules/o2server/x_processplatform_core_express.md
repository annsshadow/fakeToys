# o2server

## Responsibility

流程平台核心表达式模块，提供流程任务的终止、撤回等操作能力。

## Core Classes and Interfaces

- com.x.processplatform.core.express.ExceptionDataConvert
- com.x.processplatform.core.express.ExceptionRecordProcessing
- com.x.processplatform.core.express.ExceptionWorkDataWillBeEmpty
- com.x.processplatform.core.express.ProcessingAttributes
- com.x.processplatform.core.express.WorkDataHelper
- com.x.processplatform.core.express.assemble.surface.jaxrs.anonymous.ActionReadCountWithPersonWo
- com.x.processplatform.core.express.assemble.surface.jaxrs.anonymous.ActionTaskCountWithPersonWo
- com.x.processplatform.core.express.assemble.surface.jaxrs.application.ActionGetIconWo
- com.x.processplatform.core.express.assemble.surface.jaxrs.attachment.ActionDocToWordWi
- com.x.processplatform.core.express.assemble.surface.jaxrs.attachment.ActionDocToWordWorkOrWorkCompletedWi

## Key Flows

- 终止工作：`GET /jaxrs/processplatform/work/terminate/{id}` → `work_terminate` 执行 `UPDATE x_work SET work_status='terminated'`，返回 id/workStatus="terminated"/result="ok"
- 撤回工作：`GET /jaxrs/processplatform/work/retract/{id}` → 先查 x_work 原状态，无记录返回 error("work not found")，成功输出 workStatus="retracted" 与 previousStatus
- 工作/任务处理态：`GET .../work/processing/{id}|task/processing/{id}` → `query_one` 读 x_work(id/title/work_status/activity) 或 x_task(id/title/person/activity)
- 人员计数：`GET .../work/count/with/person/{id}|task/count/with/person/{id}` → `SELECT COUNT(*) FROM x_task WHERE person=$1`（work 版附加 `AND work IS NOT NULL`），输出 personId/count
- 路由注册：`processplatform_core_express_router(pool)` 挂 terminate/retract/processing/count 共 6 条 GET 路由并以 `.layer(Extension(pool))` 注入连接池；routes.rs 委托回 lib.rs

## Dependencies



- x_base_core_project
- x_query_core_entity
- x_cms_core_entity
- x_processplatform_core_entity
- x_correlation_core_express

**Rust（oa4rust/crates/processplatform_core_express）：**

- 内部 path 依赖：shared
- 关键外部依赖：axum、tokio、deadpool-postgres、serde/serde_json、thiserror、tracing、uuid、tower

## REST Endpoints



- `GET /jaxrs/processplatform/task/count/with/person/{id}`
- `GET /jaxrs/processplatform/task/processing/{id}`
- `GET /jaxrs/processplatform/work/count/with/person/{id}`
- `GET /jaxrs/processplatform/work/processing/{id}`
- `GET /jaxrs/processplatform/work/retract/{id}`
- `GET /jaxrs/processplatform/work/terminate/{id}`
