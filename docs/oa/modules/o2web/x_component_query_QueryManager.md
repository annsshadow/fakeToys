# x_component_query_QueryManager

## Responsibility

查询报表前端组件，负责数据应用管理的界面展示与交互操作。

## Entry Point

- `oa\o2web\source\x_component_query_QueryManager\Main.js`

## Build Pipeline

- No build scripts detected

## Key Configuration Files

- `$Explorer\process.json`: *To be described.*
- `$Main\startMenu.json`: *To be described.*

- [List of key JSON config files with one-line explanations]

## Key Flows

- 与后端 `StatementAction` 交互：在组件中调用 `StatementAction.manageList` 等方法完成 调用 等操作。
- 与后端 `ItemsAction` 交互：在组件中调用 `ItemsAction.addEvent`、`fade`、`position` 等方法完成 添加/调用/调用 等操作。
- 与后端 `TableAction` 交互：在组件中调用 `TableAction.manageList` 等方法完成 调用 等操作。
- 跨组件跳转：通过 `openApplication` 打开其它应用（FindDesigner、portal、process、query）。

## Dependencies

**后端服务（o2server action / REST）：**
- `x_query_assemble_designer`
**依赖的其它 o2web 组件 / 应用：**
- `FindDesigner`（openApplication 打开的应用）
- `portal`（openApplication 打开的应用）
- `process`（openApplication 打开的应用）
- `query`（openApplication 打开的应用）
**前端基础设施：**
- O2OA web 框架（MWF / o2.Actions / o2.xDesktop）、Vue 组件或 MooTools Class 组件模型
- 公共库：`o2_lib`（marked、PinYin 等）、`$OOUI` 组件库
