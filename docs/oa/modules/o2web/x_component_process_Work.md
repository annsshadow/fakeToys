# x_component_process_Work

## Responsibility

流程引擎前端组件，负责工作的界面展示与交互操作。

## Entry Point

- `oa\o2web\source\x_component_process_Work\Main.js`

## Build Pipeline

- No build scripts detected

## Key Configuration Files

- No JSON config files detected

- [List of key JSON config files with one-line explanations]

## Key Flows

- 与后端 `ProcessAction` 交互：在组件中调用 `ProcessAction.getActivity` 等方法完成 读取 等操作。
- 与后端 `WorkAction` 交互：在组件中调用 `WorkAction.V2ListActivityGoBack` 等方法完成 调用 等操作。
- 与后端 `TaskProcessModeAction` 交互：在组件中调用 `TaskProcessModeAction.deleteMode`、`listMode`、`saveMode` 等方法完成 删除/列出/保存 等操作。
- 与后端 `TaskCompletedAction` 交互：在组件中调用 `TaskCompletedAction.get` 等方法完成 读取 等操作。
- 与后端 `UnitDutyAction` 交互：在组件中调用 `UnitDutyAction.listNameWithIdentity` 等方法完成 列出 等操作。
- 与后端 `RecordAction` 交互：在组件中调用 `RecordAction.addEvent` 等方法完成 添加 等操作。

## Dependencies

**后端服务（o2server action / REST）：**
- `x_organization_assemble_authentication`
- `x_organization_assemble_express`
- `x_processplatform_assemble_surface`
**依赖的其它 o2web 组件 / 应用：**
- `Profile`（openApplication 打开的应用）
**前端基础设施：**
- O2OA web 框架（MWF / o2.Actions / o2.xDesktop）、Vue 组件或 MooTools Class 组件模型
- 公共库：`o2_lib`（marked、PinYin 等）、`$OOUI` 组件库
