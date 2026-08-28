# x_component_AI

## Responsibility

AI 助手组件，负责与 AI 模型对话交互，支持知识库检索和 MCP 工具调用。

## Entry Point

- `oa\o2web\source\x_component_AI\Main.js`

## Build Pipeline

- No build scripts detected

## Key Configuration Files

- No JSON config files detected

- [List of key JSON config files with one-line explanations]

## Key Flows

- 后端请求：向 `x_ai_assemble_control/jaxrs/chat/completion` 发起 POST 流式请求，处理返回数据并渲染。
- 后端请求：向 `x_ai_assemble_control/jaxrs/file/${file.id}/download/scale` 发起 GET 请求，处理返回数据并渲染。
- 后端请求：向 `x_organization_assemble_personal/jaxrs/icon/${layout.user.id}` 发起 GET 请求，处理返回数据并渲染。
- 与后端 `ConfigAction` 交互：在组件中调用 `ConfigAction.createMcpConfig`、`createModel`、`deleteMcpConfig`、`deleteModel`、`getBaseConfig`、`getConfig`、`getMcpConfig`、`getMcpExt` 等方法完成 创建/创建/删除/删除/读取/读取 等操作。
- 与后端 `ChatAction` 交互：在组件中调用 `ChatAction.delete`、`listCompletionPaging`、`listPaging` 等方法完成 删除/列出/列出 等操作。
- 与后端 `FileAction` 交互：在组件中调用 `FileAction.copyFile`、`get`、`upload` 等方法完成 复制/读取/上传 等操作。

## Dependencies

**后端服务（o2server action / REST）：**
- `x_ai_assemble_control`
- `x_cms_assemble_control`
- `x_organization_assemble_personal`
**依赖的其它 o2web 组件 / 应用：**
- `Selector.package`
- `OfficeOnlineEditor`（openApplication 打开的应用）
- `OnlyOfficeEditor`（openApplication 打开的应用）
- `PdfViewer`（openApplication 打开的应用）
- `WpsOfficeEditor`（openApplication 打开的应用）
- `YozoOfficeEditor`（openApplication 打开的应用）
- `cms`（openApplication 打开的应用）
- `portal`（openApplication 打开的应用）
**前端基础设施：**
- O2OA web 框架（MWF / o2.Actions / o2.xDesktop）、Vue 组件或 MooTools Class 组件模型
- 公共库：`o2_lib`（marked、PinYin 等）、`$OOUI` 组件库
