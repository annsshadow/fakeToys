# x_component_query_Query

## Responsibility

查询报表前端组件，负责数据应用的界面展示与交互操作。

## Entry Point

- `oa\o2web\source\x_component_query_Query\Main.js`

## Build Pipeline

- No build scripts detected

## Key Configuration Files

- `$ImporterRecord\default\detailListItem.json`: *To be described.*
- `$ImporterRecord\default\listItem.json`: *To be described.*
- `$Main\importer_test.json`: *To be described.*
- `$Main\importer_test_querytable.json`: *To be described.*

- [List of key JSON config files with one-line explanations]

## Key Flows

- 与后端 `IdentityAction` 交互：在组件中调用 `IdentityAction.listObject` 等方法完成 列出 等操作。
- 与后端 `PersonAction` 交互：在组件中调用 `PersonAction.listObject` 等方法完成 列出 等操作。
- 与后端 `UnitAction` 交互：在组件中调用 `UnitAction.listObject`、`listWithIdentitySupNested`、`listWithPerson` 等方法完成 列出/列出/列出 等操作。
- 与后端 `GroupAction` 交互：在组件中调用 `GroupAction.listObject` 等方法完成 列出 等操作。
- 与后端 `CloseAction` 交互：在组件中调用 `CloseAction.addEvent`、`hide`、`show` 等方法完成 添加/调用/调用 等操作。
- 与后端 `QueryAction` 交互：在组件中调用 `QueryAction.get` 等方法完成 读取 等操作。

## Dependencies

**后端服务（o2server action / REST）：**
- `x_cms_assemble_control`
- `x_organization_assemble_express`
- `x_processplatform_assemble_surface`
- `x_program_center`
- `x_query_assemble_surface`
**依赖的其它 o2web 组件 / 应用：**
- `cms`（openApplication 打开的应用）
- `process`（openApplication 打开的应用）
**前端基础设施：**
- O2OA web 框架（MWF / o2.Actions / o2.xDesktop）、Vue 组件或 MooTools Class 组件模型
- 公共库：`o2_lib`（marked、PinYin 等）、`$OOUI` 组件库
