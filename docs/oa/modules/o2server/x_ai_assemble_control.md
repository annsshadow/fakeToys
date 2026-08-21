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

## Key Flows

- 对话补全：`POST /jaxrs/ai_assemble_control/chat/completion` → `chat_completion` 校验会话所有权（`x_ai_chat` 的 creator 集合 vs session.person_unique）→ 按 context_window 加载历史 → INSERT user 消息与 assistant 回复入 `x_ai_chat`；有 `AI_API_KEY` 时走 reqwest 调 LLM，否则返回内置回退文案
- SSE 流式对话：`chat/completion/stream` → `call_llm_stream`（reqwest `bytes_stream` + async-stream 解析增量 chunk）→ 以 axum `Sse<Event>` 逐 token 推送
- AI 配置管理：`GET/...update/ai/control/config` 读写 `x_ai_mcp_config` WHERE is_base=true（不存在则 INSERT is_base 基线行）；`config/create/model` INSERT INTO `x_ai_model_config`，分页列表按 update_time DESC；用量统计 COUNT `x_ai_file`/`x_ai_index`

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

**Rust（oa4rust/crates/ai_assemble_control）：**

- 内部 path 依赖：shared
- 关键外部依赖：axum、tokio、deadpool-postgres、serde/serde_json、uuid、tower、reqwest（json/rustls-tls/stream）、futures-util、async-stream

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
