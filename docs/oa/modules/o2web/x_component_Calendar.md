# x_component_Calendar

## Responsibility

日程安排组件，负责日历视图的展示、日程的创建编辑和事件管理。

## Entry Point

- `oa\o2web\source\x_component_Calendar\Main.js`

## Build Pipeline

- No build scripts detected

## Key Configuration Files

- No JSON config files detected

- [List of key JSON config files with one-line explanations]

## Key Flows

- 与后端 `Calendar_EventAction` 交互：在组件中调用 `Calendar_EventAction.listWithFilterSample` 等方法完成 列出 等操作。
- 跨组件跳转：通过 `openApplication` 打开其它应用（process）。
- 核心交互：日程安排组件，负责日历视图的展示、日程的创建编辑和事件管理。

## Dependencies

**后端服务（o2server action / REST）：**
- `x_calendar_assemble_control`
**依赖的其它 o2web 组件 / 应用：**
- `process`（openApplication 打开的应用）
**前端基础设施：**
- O2OA web 框架（MWF / o2.Actions / o2.xDesktop）、Vue 组件或 MooTools Class 组件模型
- 公共库：`o2_lib`（marked、PinYin 等）、`$OOUI` 组件库
