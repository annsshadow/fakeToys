# x_component_query_ViewDesigner

## Responsibility

查询报表前端组件，负责视图编辑的界面展示与交互操作。

## Entry Point

- `oa\o2web\source\x_component_query_ViewDesigner\Main.js`

## Build Pipeline

- No build scripts detected

## Key Configuration Files

- `$View\actionbar.json`: *To be described.*
- `$View\column.json`: *To be described.*
- `$View\paging.json`: *To be described.*
- `$View\toolbars.json`: *To be described.*
- `$View\view.json`: *To be described.*
- `$View\skin\config.json`: *To be described.*
- `$View\skin\styles_blue-flat.json`: *To be described.*
- `$View\skin\styles_blue-simple.json`: *To be described.*
- `$View\skin\styles_cmcc.json`: *To be described.*
- `$View\skin\styles_default.json`: *To be described.*

- [List of key JSON config files with one-line explanations]

## Key Flows

- 与后端 `ScriptAction` 交互：在组件中调用 `ScriptAction.flag` 等方法完成 调用 等操作。
- 跨组件跳转：通过 `openApplication` 打开其它应用（query）。
- 核心交互：查询报表前端组件，负责视图编辑的界面展示与交互操作。

## Dependencies

**后端服务（o2server action / REST）：**
- `x_program_center`
**依赖的其它 o2web 组件 / 应用：**
- `query`（openApplication 打开的应用）
**前端基础设施：**
- O2OA web 框架（MWF / o2.Actions / o2.xDesktop）、Vue 组件或 MooTools Class 组件模型
- 公共库：`o2_lib`（marked、PinYin 等）、`$OOUI` 组件库
