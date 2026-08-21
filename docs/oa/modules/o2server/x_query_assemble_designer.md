# o2server

## Responsibility

查询设计器管控模块，提供查询视图的设计、创建和管理。

## Core Classes and Interfaces

- com.x.query.assemble.designer.AbstractFactory
- com.x.query.assemble.designer.ApplicationServletContextListener
- com.x.query.assemble.designer.Business
- com.x.query.assemble.designer.CompareQuery
- com.x.query.assemble.designer.ThisApplication
- com.x.query.assemble.designer.factory.ImportModelFactory
- com.x.query.assemble.designer.factory.NeuralFactory
- com.x.query.assemble.designer.factory.ProcessFactory
- com.x.query.assemble.designer.factory.QueryFactory
- com.x.query.assemble.designer.factory.StatementFactory

## Key Flows

- 查询设计 CRUD：`POST /jaxrs/query/assemble/designer/create` → `create_designer`（uuid v4）→ INSERT INTO `x_query_design`（name/category/query_definition）；`POST .../designer/save/{id}` UPDATE 同表，`POST .../delete/{id}` 软删（SET deleted_at=NOW()），列表查询均带 `deleted_at IS NULL`
- 神经网络模型生命周期：创建走 INSERT INTO `x_query_neural_model`（初始 status='idle'）；`neural/generate/model/{modelFlag}`、`neural/learn/model/{modelFlag}` 分别 UPDATE status='generating'/'learning'；stop/reset 动作恢复为 'idle'（stop 带 status 前置条件）
- 导入模型与输入/输出：`importmodel/list/query/{queryFlag}` 按 query_flag 查询 `x_query_import_model`（含 permission 校验读取）；输入草稿经 `input/create` INSERT INTO `x_query_input`、cover 更新 content；`output/list` 查询 `x_query_output`（WHERE deleted_at IS NULL）

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

**Rust（oa4rust/crates/query_assemble_designer）：**

- 内部 path 依赖：shared
- 关键外部依赖：axum、tokio、deadpool-postgres、serde/serde_json、uuid、tower、bcrypt、base64、anyhow、chrono、md5、urlencoding

## REST Endpoints



- `POST /jaxrs/query/assemble/designer/create`
- `POST /jaxrs/query/assemble/designer/delete/{id}`
- `GET /jaxrs/query/assemble/designer/get/{id}`
- `GET /jaxrs/query/assemble/designer/list/{category}`
- `POST /jaxrs/query/assemble/designer/save/{id}`
