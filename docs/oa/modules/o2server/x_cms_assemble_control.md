# o2server

## Responsibility

CMS 管控模块，处理 CMS 栏目、文章、字典等内容的配置和管理。

## Core Classes and Interfaces

- com.x.cms.assemble.control.AbstractFactory
- com.x.cms.assemble.control.ApplicationServletContextListener
- com.x.cms.assemble.control.Business
- com.x.cms.assemble.control.CacheUtil
- com.x.cms.assemble.control.CompareAppInfo
- com.x.cms.assemble.control.Control
- com.x.cms.assemble.control.DocumentDataHelper
- com.x.cms.assemble.control.ExceptionDocumentDataWillBeEmpty
- com.x.cms.assemble.control.ExceptionWrapInConvert
- com.x.cms.assemble.control.MessageFactory

## Key Flows

- 全文检索：`GET /jaxrs/cms_assemble_control/document/search?q=` → `search::search_documents_smart`（Tantivy 本地索引优先、PG to_tsvector 静默回退）→ 返回带 rank 的文档列表
- 应用与栏目列表：`GET /jaxrs/appinfo/list/*`、`/jaxrs/categoryinfo/list/*` 族 → `list_from_table_filtered` 条件查询 `x_cms_appinfo`/`x_cms_categoryinfo`（deleted_at IS NULL）→ 返回 count+data JSON
- 文档数据与附件：`POST /jaxrs/fileinfo/upload/document/{docId}` → INSERT INTO `x_cms_fileinfo` RETURNING *；文档数据读写查询 `x_cms_data_document` 与 `x_cms_data_document_field`，下载走 `/jaxrs/fileinfo/download/document/stream/{id}`

## Dependencies



- x_base_core_project
- x_organization_core_entity
- x_organization_core_express
- x_cms_core_entity
- x_cms_core_express
- x_query_core_entity
- x_query_core_express
- x_processplatform_core_entity
- x_portal_core_entity
- x_general_core_entity
- x_program_center_core_entity
- x_correlation_core_entity
- x_correlation_core_express

**Rust（oa4rust/crates/cms_assemble_control）：**

- 内部 path 依赖：shared、search
- 关键外部依赖：axum、deadpool-postgres、tower

## REST Endpoints



- `GET /jaxrs/application/{id}`
- `GET /jaxrs/cms_assemble_control/get/control/config`
- `GET /jaxrs/cms_assemble_control/list/control/sections`
- `GET /jaxrs/cms_assemble_control/update/control/config`
- `GET /jaxrs/commend/list/paging/{docId}`
- `POST /jaxrs/document/{id}/view/count`
- `GET /jaxrs/queryview/flag/{view}/definition/{queryFlag}`
