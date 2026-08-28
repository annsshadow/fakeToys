# x_component_cms_Module

## Responsibility

CMS（内容管理）前端组件，负责内容管理的界面展示与交互操作，通过 REST API 与后端模块通信。

## Entry Point

- `oa\o2web\source\x_component_cms_Module\Main.js`

## Build Pipeline

- No build scripts detected

## Key Configuration Files

- `$ListExplorer\filterItem.json`: *To be described.*
- `$ListExplorer\listItem.json`: *To be described.*
- `$ListExplorer\listItemForAll.json`: *To be described.*
- `$ListExplorer\listItemForDelay.json`: *To be described.*
- `$ListExplorer\toolbar.json`: *To be described.*
- `$ViewExplorer\filterItem.json`: *To be described.*
- `$ViewExplorer\listItem.json`: *To be described.*
- `$ViewExplorer\listItemForAll.json`: *To be described.*
- `$ViewExplorer\toolbar.json`: *To be described.*

- [List of key JSON config files with one-line explanations]

## Key Flows

- 与后端 `DocumentAction` 交互：在组件中调用 `DocumentAction.addClass`、`addEvents`、`query_listWithFilterPaging`、`removeClass`、`setStyles` 等方法完成 添加/添加/查询/移除/设置 等操作。
- 与后端 `RemoveAction` 交互：在组件中调用 `RemoveAction.addClass`、`addEvents`、`destroy`、`removeClass`、`set`、`setStyles` 等方法完成 添加/添加/调用/移除/设置/设置 等操作。
- 与后端 `RemoveConfirmAction` 交互：在组件中调用 `RemoveConfirmAction.addEvents`、`setStyle` 等方法完成 添加/设置 等操作。
- 与后端 `CategoryInfoAction` 交互：在组件中调用 `CategoryInfoAction.listPublishableCategoryInfo` 等方法完成 列出 等操作。
- 跨组件跳转：通过 `openApplication` 打开其它应用（cms）。

## Dependencies

**后端服务（o2server action / REST）：**
- `x_cms_assemble_control`
**依赖的其它 o2web 组件 / 应用：**
- `cms`（openApplication 打开的应用）
**前端基础设施：**
- O2OA web 框架（MWF / o2.Actions / o2.xDesktop）、Vue 组件或 MooTools Class 组件模型
- 公共库：`o2_lib`（marked、PinYin 等）、`$OOUI` 组件库
