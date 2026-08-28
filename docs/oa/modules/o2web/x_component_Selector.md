# x_component_Selector

## Responsibility

通用选择器组件，提供人员、单位、角色等数据的通用选择功能。

## Entry Point

- `oa\o2web\source\x_component_Selector\Main.js`

## Build Pipeline

- No build scripts detected

## Key Configuration Files

- No JSON config files detected

- [List of key JSON config files with one-line explanations]

## Key Flows

- 与后端 `ProcessAction` 交互：在组件中调用 `ProcessAction.getActivity` 等方法完成 读取 等操作。
- 与后端 `AgentAction` 交互：在组件中调用 `AgentAction.list` 等方法完成 列出 等操作。
- 与后端 `InvokeAction` 交互：在组件中调用 `InvokeAction.get`、`list` 等方法完成 读取/列出 等操作。
- 与后端 `ApplicationAction` 交互：在组件中调用 `ApplicationAction.list`、`listWithPerson` 等方法完成 列出/列出 等操作。
- 与后端 `ApplicationDictAction` 交互：在组件中调用 `ApplicationDictAction.listPaging` 等方法完成 列出 等操作。
- 与后端 `AppDictDesignAction` 交互：在组件中调用 `AppDictDesignAction.listPaging` 等方法完成 列出 等操作。

## Dependencies

**后端服务（o2server action / REST）：**
- `x_cms_assemble_control`
- `x_organization_assemble_control`
- `x_organization_assemble_express`
- `x_pan_assemble_control`
- `x_portal_assemble_designer`
- `x_portal_assemble_surface`
- `x_processplatform_assemble_designer`
- `x_processplatform_assemble_surface`
- `x_program_center`
**前端基础设施：**
- O2OA web 框架（MWF / o2.Actions / o2.xDesktop）、Vue 组件或 MooTools Class 组件模型
- 公共库：`o2_lib`（marked、PinYin 等）、`$OOUI` 组件库
