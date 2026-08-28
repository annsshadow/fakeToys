# x_component_cms_ColumnManager

## Responsibility

CMS（内容管理）前端组件，负责内容管理栏目设置的界面展示与交互操作，通过 REST API 与后端模块通信。

## Entry Point

- `oa\o2web\source\x_component_cms_ColumnManager\Main.js`

## Build Pipeline

- No build scripts detected

## Key Configuration Files

- `$Main\startMenu.json`: *To be described.*

- [List of key JSON config files with one-line explanations]

## Key Flows

- 与后端 `EditAction` 交互：在组件中调用 `EditAction.addEvents`、`setStyles` 等方法完成 添加/设置 等操作。
- 与后端 `ReadAction` 交互：在组件中调用 `ReadAction.addEvents`、`setStyles` 等方法完成 添加/设置 等操作。
- 与后端 `DefaultCategoryViewAction` 交互：在组件中调用 `DefaultCategoryViewAction.addEvents`、`setStyles` 等方法完成 添加/设置 等操作。
- 与后端 `ViewAction` 交互：在组件中调用 `ViewAction.addEvent`、`addEvents`、`setStyles` 等方法完成 添加/添加/设置 等操作。
- 与后端 `EditViewAction` 交互：在组件中调用 `EditViewAction.addEvents`、`setStyles` 等方法完成 添加/设置 等操作。
- 与后端 `CategoryViewAction` 交互：在组件中调用 `CategoryViewAction.setStyle`、`setStyles` 等方法完成 设置/设置 等操作。

## Dependencies

**后端服务（o2server action / REST）：**
- `x_cms_assemble_control`
**依赖的其它 o2web 组件 / 应用：**
- `FindDesigner`（openApplication 打开的应用）
- `cms`（openApplication 打开的应用）
- `process`（openApplication 打开的应用）
- `query`（openApplication 打开的应用）
**前端基础设施：**
- O2OA web 框架（MWF / o2.Actions / o2.xDesktop）、Vue 组件或 MooTools Class 组件模型
- 公共库：`o2_lib`（marked、PinYin 等）、`$OOUI` 组件库
