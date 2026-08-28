# x_component_FindDesigner

## Responsibility

查找设计器组件，在表单和页面中提供通用查找和选择功能。

## Entry Point

- `oa\o2web\source\x_component_FindDesigner\Main.js`

## Build Pipeline

- No build scripts detected

## Key Configuration Files

- `propertys.json`: *To be described.*

- [List of key JSON config files with one-line explanations]

## Key Flows

- 与后端 `DesignAction` 交互：在组件中调用 `DesignAction.search` 等方法完成 检索 等操作。
- 与后端 `ScriptAction` 交互：在组件中调用 `ScriptAction.get` 等方法完成 读取 等操作。
- 与后端 `FomrAction` 交互：在组件中调用 `FomrAction.get` 等方法完成 读取 等操作。
- 与后端 `ProcessAction` 交互：在组件中调用 `ProcessAction.get` 等方法完成 读取 等操作。
- 与后端 `FormAction` 交互：在组件中调用 `FormAction.get` 等方法完成 读取 等操作。
- 与后端 `PageAction` 交互：在组件中调用 `PageAction.get` 等方法完成 读取 等操作。

## Dependencies

**后端服务（o2server action / REST）：**
- `x_cms_assemble_control`
- `x_portal_assemble_designer`
- `x_processplatform_assemble_designer`
- `x_program_center`
- `x_query_assemble_designer`
- `x_query_service_processing`
**依赖的其它 o2web 组件 / 应用：**
- `Selector.package`
- `cms`（openApplication 打开的应用）
- `portal`（openApplication 打开的应用）
- `process`（openApplication 打开的应用）
- `query`（openApplication 打开的应用）
- `service`（openApplication 打开的应用）
**前端基础设施：**
- O2OA web 框架（MWF / o2.Actions / o2.xDesktop）、Vue 组件或 MooTools Class 组件模型
- 公共库：`o2_lib`（marked、PinYin 等）、`$OOUI` 组件库
