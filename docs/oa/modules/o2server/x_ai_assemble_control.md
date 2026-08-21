# o2server

## Responsibility

AI 管控模块，处理 AI 应用、模型和对话的业务编排。

## Core Classes and Interfaces

- com.x.ai.assemble.control.AbstractFactory
- com.x.ai.assemble.control.ApplicationServletContextListener
- com.x.ai.assemble.control.Business
- com.x.ai.assemble.control.ThisApplication
- com.x.ai.assemble.control.bean.AiConfig
- com.x.ai.assemble.control.bean.ChartWi
- com.x.ai.assemble.control.bean.DocIndex
- com.x.ai.assemble.control.bean.McpConfig
- com.x.ai.assemble.control.factory.CmsItemFactory
- com.x.ai.assemble.control.jaxrs.ActionApplication

## Dependencies



- x_base_core_project
- x_organization_core_entity
- x_organization_core_express
- x_cms_core_entity
- x_processplatform_core_entity
- x_query_core_entity
- x_ai_core_entity
- jersey-media-sse
- jersey-client

## REST Endpoints



- `POST /jaxrs/ai/assemble/control/config/create/mcp`
- `POST /jaxrs/ai/assemble/control/config/delete/mcp/{id}`
- `GET /jaxrs/ai/assemble/control/config/get/mcp/{id}`
- `GET /jaxrs/ai/assemble/control/config/list/mcp/paging/{page}/size/{size}`
- `POST /jaxrs/ai/assemble/control/config/update/mcp/{id}`
- `POST /jaxrs/ai_assemble_control/chat/completion`
- `GET /jaxrs/ai_assemble_control/config/base/config`
- `GET /jaxrs/ai_assemble_control/config/create/mcp`
- `GET /jaxrs/ai_assemble_control/config/create/model`
- `GET /jaxrs/ai_assemble_control/config/delete/mcp/flag`
- `GET /jaxrs/ai_assemble_control/config/delete/model/flag`
- `GET /jaxrs/ai_assemble_control/config/get/mcp/ext/flag`
- `GET /jaxrs/ai_assemble_control/config/get/mcp/flag`
- `GET /jaxrs/ai_assemble_control/config/get/model/flag`
- `GET /jaxrs/ai_assemble_control/config/list/enable/model`
- `GET /jaxrs/ai_assemble_control/config/list/mcp/paging/page/size/size`
- `GET /jaxrs/ai_assemble_control/config/list/model/paging/page/size/size`
- `GET /jaxrs/ai_assemble_control/config/save`
- `GET /jaxrs/ai_assemble_control/config/update/mcp/flag`
- `GET /jaxrs/ai_assemble_control/config/update/model/flag`
- `GET /jaxrs/ai_assemble_control/file/copy/file`
- `GET /jaxrs/ai_assemble_control/file/delete/flag`
- `GET /jaxrs/ai_assemble_control/file/flag`
- `GET /jaxrs/ai_assemble_control/file/id/download`
- `GET /jaxrs/ai_assemble_control/file/id/download/scale`
- `GET /jaxrs/ai_assemble_control/file/list`
- `GET /jaxrs/ai_assemble_control/file/list/paging/page/size/size`
- `GET /jaxrs/ai_assemble_control/file/upload`
- `GET /jaxrs/ai_assemble_control/get/ai/control/config`
- `GET /jaxrs/ai_assemble_control/get/usage/stats`
- `GET /jaxrs/ai_assemble_control/index/cms/doc/docId`
- `GET /jaxrs/ai_assemble_control/index/cms/doc/with/app/appId`
- `GET /jaxrs/ai_assemble_control/index/delete/flag`
- `GET /jaxrs/ai_assemble_control/index/list/paging/page/size/size`
- `GET /jaxrs/ai_assemble_control/index/sync/to/knowledge`
- `GET /jaxrs/ai_assemble_control/list/ai/models`
- `GET /jaxrs/ai_assemble_control/update/ai/control/config`
