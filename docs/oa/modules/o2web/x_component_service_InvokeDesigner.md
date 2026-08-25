# x_component_service_InvokeDesigner

## Responsibility

服务组件，负责接口编辑的界面展示与交互操作。

## Entry Point

- `oa\o2web\source\x_component_service_InvokeDesigner\Main.js`

## Build Pipeline

- No build scripts detected

## Key Configuration Files

- No JSON config files detected

- [List of key JSON config files with one-line explanations]

## Key Flows

- 与后端 `InvokeItemAction` 交互：在组件中调用 `InvokeItemAction.addEvent` 等方法完成 添加 等操作。
- 与后端 `ConfigAction` 交互：在组件中调用 `ConfigAction.getToken` 等方法完成 读取 等操作。
- 与后端 `InvokeAction` 交互：在组件中调用 `InvokeAction.execute`、`token` 等方法完成 执行/调用 等操作。
- 跨组件跳转：通过 `openApplication` 打开其它应用（LogViewer、service）。

## Dependencies

**后端服务（o2server action / REST）：**
- `x_program_center`
**依赖的其它 o2web 组件 / 应用：**
- `LogViewer`（openApplication 打开的应用）
- `service`（openApplication 打开的应用）
**前端基础设施：**
- O2OA web 框架（MWF / o2.Actions / o2.xDesktop）、Vue 组件或 MooTools Class 组件模型
- 公共库：`o2_lib`（marked、PinYin 等）、`$OOUI` 组件库
