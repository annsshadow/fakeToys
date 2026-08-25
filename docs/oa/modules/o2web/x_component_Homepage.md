# x_component_Homepage

## Responsibility

首页组件，负责门户首页的布局和个性化展示。

## Entry Point

- `oa\o2web\source\x_component_Homepage\Main.js`

## Build Pipeline

- No build scripts detected

## Key Configuration Files

- `$Main\icon.json`: *To be described.*

- [List of key JSON config files with one-line explanations]

## Key Flows

- 与后端 `ProcessAction` 交互：在组件中调用 `ProcessAction.addEvent` 等方法完成 添加 等操作。
- 与后端 `InforAction` 交互：在组件中调用 `InforAction.addEvent` 等方法完成 添加 等操作。
- 与后端 `Calendar_EventAction` 交互：在组件中调用 `Calendar_EventAction.listWithFilter`、`listWithFilterSample` 等方法完成 列出/列出 等操作。
- 与后端 `PortalAction` 交互：在组件中调用 `PortalAction.get` 等方法完成 读取 等操作。
- 与后端 `Attachment2Action` 交互：在组件中调用 `Attachment2Action.listTop` 等方法完成 列出 等操作。
- 与后端 `AttachmentAction` 交互：在组件中调用 `AttachmentAction.listTop` 等方法完成 列出 等操作。

## Dependencies

**后端服务（o2server action / REST）：**
- `x_calendar_assemble_control`
- `x_cms_assemble_control`
- `x_file_assemble_control`
- `x_hotpic_assemble_control`
- `x_meeting_assemble_control`
- `x_portal_assemble_surface`
- `x_processplatform_assemble_surface`
**依赖的其它 o2web 组件 / 应用：**
- `process.TaskCenter.lp.`
- `Calendar`（openApplication 打开的应用）
- `File`（openApplication 打开的应用）
- `ForumDocument`（openApplication 打开的应用）
- `Meeting`（openApplication 打开的应用）
- `cms`（openApplication 打开的应用）
- `portal`（openApplication 打开的应用）
- `process`（openApplication 打开的应用）
**前端基础设施：**
- O2OA web 框架（MWF / o2.Actions / o2.xDesktop）、Vue 组件或 MooTools Class 组件模型
- 公共库：`o2_lib`（marked、PinYin 等）、`$OOUI` 组件库
