# x_component_process_ProcessDesigner

## Responsibility

流程引擎前端组件，负责流程编辑的界面展示与交互操作。

## Entry Point

- `oa\o2web\source\x_component_process_ProcessDesigner\Main.js`

## Build Pipeline

- No build scripts detected

## Key Configuration Files

- `$Main\gadget.json`: *To be described.*
- `$Main\process.json`: *To be described.*
- `$Process\action.json`: *To be described.*
- `$Process\activity.json`: *To be described.*
- `$Process\process.json`: *To be described.*
- `$Process\route.json`: *To be described.*
- `$Process\serialRule.json`: *To be described.*
- `$Process\template\process.json`: *To be described.*
- `$Process\template\process_choice.json`: *To be described.*
- `$Process\template\process_condition.json`: *To be described.*

- [List of key JSON config files with one-line explanations]

## Key Flows

- 与后端 `ProcessAction` 交互：在组件中调用 `ProcessAction.enableProcess`、`get`、`listEdition`、`listWithApplication`、`upgrade` 等方法完成 调用/读取/列出/列出/调用 等操作。
- 与后端 `FormAction` 交互：在组件中调用 `FormAction.get`、`listWithApplication` 等方法完成 读取/列出 等操作。
- 与后端 `ItemAccessAction` 交互：在组件中调用 `ItemAccessAction.bachSave`、`deleteWithProcessWithPath`、`listWithProcess` 等方法完成 调用/删除/列出 等操作。
- 跨组件跳转：通过 `openApplication` 打开其它应用（process）。
- 依赖其它组件能力：通过 `requireApp` 引入 Template.Selector.Custom 等组件模块。

## Dependencies

**后端服务（o2server action / REST）：**
- `x_processplatform_assemble_designer`
**依赖的其它 o2web 组件 / 应用：**
- `Template.Selector.Custom`
- `process`（openApplication 打开的应用）
**前端基础设施：**
- O2OA web 框架（MWF / o2.Actions / o2.xDesktop）、Vue 组件或 MooTools Class 组件模型
- 公共库：`o2_lib`（marked、PinYin 等）、`$OOUI` 组件库
