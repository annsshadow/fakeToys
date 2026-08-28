# x_component_service_ScriptDesigner

## Responsibility

服务组件，负责的界面展示与交互操作。

## Entry Point

- `oa\o2web\source\x_component_service_ScriptDesigner\Main.js`

## Build Pipeline

- No build scripts detected

## Key Configuration Files

- No JSON config files detected

- [List of key JSON config files with one-line explanations]

## Key Flows

- 与后端 `ScriptItemAction` 交互：在组件中调用 `ScriptItemAction.addEvent` 等方法完成 添加 等操作。
- 跨组件跳转：通过 `openApplication` 打开其它应用（process、service）。
- 核心交互：服务组件，负责的界面展示与交互操作。

## Dependencies

**依赖的其它 o2web 组件 / 应用：**
- `process`（openApplication 打开的应用）
- `service`（openApplication 打开的应用）
**前端基础设施：**
- O2OA web 框架（MWF / o2.Actions / o2.xDesktop）、Vue 组件或 MooTools Class 组件模型
- 公共库：`o2_lib`（marked、PinYin 等）、`$OOUI` 组件库
