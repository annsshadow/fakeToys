# x_component_Collect

## Responsibility

数据采集组件，负责问卷或表单数据的收集和管理。

## Entry Point

- `oa\o2web\source\x_component_Collect\Main.js`

## Build Pipeline

- No build scripts detected

## Key Configuration Files

- No JSON config files detected

- [List of key JSON config files with one-line explanations]

## Key Flows

- 与后端 `AccountAction` 交互：在组件中调用 `AccountAction.addEvent` 等方法完成 添加 等操作。
- 与后端 `PwdAccountAction` 交互：在组件中调用 `PwdAccountAction.addEvent` 等方法完成 添加 等操作。
- 核心交互：数据采集组件，负责问卷或表单数据的收集和管理。

## Dependencies

**前端基础设施：**
- O2OA web 框架（MWF / o2.Actions / o2.xDesktop）、Vue 组件或 MooTools Class 组件模型
- 公共库：`o2_lib`（marked、PinYin 等）、`$OOUI` 组件库
