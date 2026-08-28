# x_component_Org

## Responsibility

组织管理组件，负责人事组织架构的浏览和选择操作。

## Entry Point

- `oa\o2web\source\x_component_Org\Main.js`

## Build Pipeline

- No build scripts detected

## Key Configuration Files

- `$Main\startMenu.json`: *To be described.*

- [List of key JSON config files with one-line explanations]

## Key Flows

- 与后端 `UnitAction` 交互：在组件中调用 `UnitAction.getRoot` 等方法完成 读取 等操作。
- 与后端 `PersonAction` 交互：在组件中调用 `PersonAction.addEvent`、`banPerson`、`hide`、`listFilterPaging`、`lockPerson`、`setPasswordExpiredTime`、`show`、`unbanPerson` 等方法完成 添加/调用/调用/列出/锁定/设置 等操作。
- 与后端 `UnitDutyAction` 交互：在组件中调用 `UnitDutyAction.edit`、`get` 等方法完成 调用/读取 等操作。
- 与后端 `RoleAction` 交互：在组件中调用 `RoleAction.listWithPerson` 等方法完成 列出 等操作。
- 与后端 `PasswordAction` 交互：在组件中调用 `PasswordAction.addEvent` 等方法完成 添加 等操作。
- 与后端 `TimeAction` 交互：在组件中调用 `TimeAction.addEvent` 等方法完成 添加 等操作。

## Dependencies

**后端服务（o2server action / REST）：**
- `x_general_assemble_control`
- `x_organization_assemble_control`
**前端基础设施：**
- O2OA web 框架（MWF / o2.Actions / o2.xDesktop）、Vue 组件或 MooTools Class 组件模型
- 公共库：`o2_lib`（marked、PinYin 等）、`$OOUI` 组件库
