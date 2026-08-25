# o2server

## Responsibility

查询展现管控模块，处理查询视图的预览和结果展示。

## Core Classes and Interfaces

- com.x.query.assemble.surface.AbstractFactory
- com.x.query.assemble.surface.ApplicationServletContextListener
- com.x.query.assemble.surface.Business
- com.x.query.assemble.surface.NamedParameterStatement
- com.x.query.assemble.surface.ThisApplication
- com.x.query.assemble.surface.factory.AppInfoFactory
- com.x.query.assemble.surface.factory.ApplicationFactory
- com.x.query.assemble.surface.factory.CategoryInfoFactory
- com.x.query.assemble.surface.factory.ImportModelFactory
- com.x.query.assemble.surface.factory.IndexFactory

## Key Flows

- 查询表面 CRUD：`POST /jaxrs/query/assemble/surface/create` → `create_surface`（uuid v4，creator="system"）→ INSERT INTO `x_query_surface`；`GET .../get/{id}`、`GET .../list/{category}`（ORDER BY update_time DESC）、`POST .../save/{id}` UPDATE 同表
- 导入模型执行：`POST /jaxrs/importmodel/id/{id}/execute` → `importmodel_id_execute` 先按 id 查 `x_query_import_model` 取 model_flag → INSERT INTO `x_query_import_model_record` 生成执行记录 → 返回 recordId；记录分页与状态查询走 `x_query_import_model_record`
- 数据表动态查询：`GET .../table/list/paging/{page}/{size}` 查询 `x_query_table`；行数据经 `x_query_table_data` 支持分页/next/prev 游标及 ILIKE 条件筛选（`table_list_tableFlag_row_select_where_where`），另有 neural 计算结果查询 `x_query_neural_calculate`

## Dependencies



- x_base_core_project
- x_organization_core_entity
- x_organization_core_express
- x_query_core_entity
- x_query_core_express
- x_processplatform_core_entity
- x_cms_core_entity
- x_cms_core_express
- x_general_core_entity

**Rust（oa4rust/crates/query_assemble_surface）：**

- 内部 path 依赖：shared
- 关键外部依赖：axum、tokio、deadpool-postgres、serde/serde_json、uuid、tower、bcrypt、base64、anyhow、chrono、md5、urlencoding

## REST Endpoints



- `POST /jaxrs/importmodel/id/{id}/execute`
- `POST /jaxrs/query/assemble/surface/create`
- `POST /jaxrs/query/assemble/surface/delete/{id}`
- `GET /jaxrs/query/assemble/surface/get/{id}`
- `GET /jaxrs/query/assemble/surface/list/{category}`
- `GET /jaxrs/query/assemble/surface/preview/{id}`
- `POST /jaxrs/query/assemble/surface/save/{id}`
- `GET /jaxrs/queryview/flag/{view}/application/flag/{app}/execute`
- `GET /jaxrs/queryview/flag/{view}/application/flag/{app}/execute/page/{page}/size/{size}`
