# x_component_AppCenter

## Responsibility

应用中心组件，负责 OA 平台应用的统一入口展示和应用管理。

## Entry Point

- `oa\o2web\source\x_component_AppCenter\Main.js`

## Build Pipeline

- No build scripts detected

## Key Configuration Files

- No JSON config files detected

- [List of key JSON config files with one-line explanations]

## Key Flows

- 与后端 `ConfigOpenAction` 交互：在组件中调用 `ConfigOpenAction.getDisableExportEnable` 等方法完成 读取 等操作。
- 与后端 `AllAction` 交互：在组件中调用 `AllAction.addEvent` 等方法完成 添加 等操作。
- 核心交互：应用中心组件，负责 OA 平台应用的统一入口展示和应用管理。

## Dependencies

**后端服务（o2server action / REST）：**
- `x_program_center`
**前端基础设施：**
- O2OA web 框架（MWF / o2.Actions / o2.xDesktop）、Vue 组件或 MooTools Class 组件模型
- 公共库：`o2_lib`（marked、PinYin 等）、`$OOUI` 组件库
