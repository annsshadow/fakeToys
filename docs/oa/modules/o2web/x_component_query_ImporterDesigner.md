# x_component_query_ImporterDesigner

## Responsibility

查询报表前端组件，负责导入模型编辑的界面展示与交互操作。

## Entry Point

- `oa\o2web\source\x_component_query_ImporterDesigner\Main.js`

## Build Pipeline

- No build scripts detected

## Key Configuration Files

- `$Importer\calculateField.json`: *To be described.*
- `$Importer\column.json`: *To be described.*
- `$Importer\importer.json`: *To be described.*

- [List of key JSON config files with one-line explanations]

## Key Flows

- 与后端 `TableAction` 交互：在组件中调用 `TableAction.get` 等方法完成 读取 等操作。
- 与后端 `FormAction` 交互：在组件中调用 `FormAction.listWithApplication` 等方法完成 列出 等操作。
- 跨组件跳转：通过 `openApplication` 打开其它应用（query）。

## Dependencies

**后端服务（o2server action / REST）：**
- `x_processplatform_assemble_designer`
- `x_query_assemble_designer`
**依赖的其它 o2web 组件 / 应用：**
- `query`（openApplication 打开的应用）
**前端基础设施：**
- O2OA web 框架（MWF / o2.Actions / o2.xDesktop）、Vue 组件或 MooTools Class 组件模型
- 公共库：`o2_lib`（marked、PinYin 等）、`$OOUI` 组件库
