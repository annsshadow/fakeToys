# o2server

## Responsibility

AI 核心实体模块，定义 AI 应用、模型和对话数据模型。

## Core Classes and Interfaces

- com.x.ai.core.entity.AiModel
- com.x.ai.core.entity.AiModel_
- com.x.ai.core.entity.Clue
- com.x.ai.core.entity.Clue_
- com.x.ai.core.entity.Completion
- com.x.ai.core.entity.Completion_
- com.x.ai.core.entity.File
- com.x.ai.core.entity.File_
- com.x.ai.core.entity.PersistenceProperties
- com.x.ai.core.entity.ToolCall

## Key Flows

- 应用列表：`GET /jaxrs/ai/core/entity/app/list` → `app_list` 以 sea-orm `ai_app::Entity::find()` 查 `x_ai_app`，按 CreateTime 倒序取前 20 条，逐行输出 id/name/status（description 存在时附加）→ 包成 `{count, data}` 的 ActionResult
- 模型列表：`GET /jaxrs/ai/core/entity/model/list` → `model_list` 查 `x_ai_model` 按 Name 升序 limit 20，输出 id/name/provider/enabled
- 对话列表：`GET /jaxrs/ai/core/entity/conversation/list` → `conversation_list` 查 `x_ai_conversation` 按 CreateTime 倒序 limit 20；注意源码中 userId 字段名写作 `"\"userId\""`（带转义引号），createTime 为 Option 转字符串
- 路由注册：`ai_core_entity_router(pool)` 在 axum Router 上挂上述三条 GET 路由；routes.rs 仅委托回 lib.rs 的同名 router

## Dependencies



- x_base_core_project

**Rust（oa4rust/crates/ai_core_entity）：**

- 内部 path 依赖：shared
- 关键外部依赖：axum、tokio、sea-orm、deadpool-postgres、serde/serde_json、uuid、tower

## REST Endpoints

<!-- Generated from Swagger annotations or action JSON files. Omit this section if the module has no REST endpoints. -->

- [Endpoint list]
