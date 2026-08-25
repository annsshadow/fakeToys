# x_component_Profile

## Responsibility

个人设置组件，负责用户个人信息和偏好设置的管理。

## Entry Point

- `oa\o2web\source\x_component_Profile\Main.js`

## Build Pipeline

- No build scripts detected

## Key Configuration Files

- No JSON config files detected

- [List of key JSON config files with one-line explanations]

## Key Flows

- 与后端 `DataAction` 交互：在组件中调用 `DataAction.addEvent` 等方法完成 添加 等操作。
- 与后端 `DefaultDataAction` 交互：在组件中调用 `DefaultDataAction.addEvent` 等方法完成 添加 等操作。
- 与后端 `ForceDataAction` 交互：在组件中调用 `ForceDataAction.addEvent` 等方法完成 添加 等操作。
- 与后端 `SaveAction` 交互：在组件中调用 `SaveAction.addEvent`、`getNext` 等方法完成 添加/读取 等操作。
- 与后端 `SaveDefaultAction` 交互：在组件中调用 `SaveDefaultAction.addEvent` 等方法完成 添加 等操作。
- 与后端 `EmpowerLogAction` 交互：在组件中调用 `EmpowerLogAction.delete` 等方法完成 删除 等操作。

## Dependencies

**后端服务（o2server action / REST）：**
- `x_bbs_assemble_control`
- `x_organization_assemble_control`
- `x_organization_assemble_express`
- `x_organization_assemble_personal`
- `x_processplatform_assemble_surface`
**依赖的其它 o2web 组件 / 应用：**
- `FaceSet`（openApplication 打开的应用）
- `process`（openApplication 打开的应用）
**前端基础设施：**
- O2OA web 框架（MWF / o2.Actions / o2.xDesktop）、Vue 组件或 MooTools Class 组件模型
- 公共库：`o2_lib`（marked、PinYin 等）、`$OOUI` 组件库
