# x_component_cms_Column

## Responsibility

CMS（内容管理）前端组件，负责内容管理设置的界面展示与交互操作，通过 REST API 与后端模块通信。

## Entry Point

- `oa\o2web\source\x_component_cms_Column\Main.js`

## Build Pipeline

- No build scripts detected

## Key Configuration Files

- No JSON config files detected

- [List of key JSON config files with one-line explanations]

## Key Flows

- 与后端 `SelectAction` 交互：在组件中调用 `SelectAction.addEvent` 等方法完成 添加 等操作。
- 与后端 `ViewSelectAction` 交互：在组件中调用 `ViewSelectAction.addEvent` 等方法完成 添加 等操作。
- 与后端 `IconAction` 交互：在组件中调用 `IconAction.addEvent` 等方法完成 添加 等操作。
- 跨组件跳转：通过 `openApplication` 打开其它应用（FindDesigner、cms）。

## Dependencies

**依赖的其它 o2web 组件 / 应用：**
- `FindDesigner`（openApplication 打开的应用）
- `cms`（openApplication 打开的应用）
**前端基础设施：**
- O2OA web 框架（MWF / o2.Actions / o2.xDesktop）、Vue 组件或 MooTools Class 组件模型
- 公共库：`o2_lib`（marked、PinYin 等）、`$OOUI` 组件库
