# x_component_ThreeMember

## Responsibility

三员管理组件，负责系统管理员、安全管理员和审计管理员的职责分离管理。

## Entry Point

- `oa\o2web\source\x_component_ThreeMember\Main.js`

## Build Pipeline

- No build scripts detected

## Key Configuration Files

- `$LogView\default\listItem.json`: *To be described.*
- `$LogView\default\listItemConfig.json`: *To be described.*

- [List of key JSON config files with one-line explanations]

## Key Flows

- 与后端 `ConfigAction` 交互：在组件中调用 `ConfigAction.destroy`、`getPerson`、`getTernaryManagement`、`getToken`、`setPerson`、`setTernaryManagement`、`setToken` 等方法完成 调用/读取/读取/读取/设置/设置 等操作。
- 与后端 `AuditLogAction` 交互：在组件中调用 `AuditLogAction.executeTodayDispatch`、`get`、`listPaging`、`toExcel` 等方法完成 执行/读取/列出/调用 等操作。
- 与后端 `AuditConfigAction` 交互：在组件中调用 `AuditConfigAction.delete`、`get`、`listModule`、`listOperation`、`listPaging`、`save`、`update` 等方法完成 删除/读取/列出/列出/列出/保存 等操作。
- 与后端 `ApplicationAction` 交互：在组件中调用 `ApplicationAction.get` 等方法完成 读取 等操作。
- 与后端 `ProcessAction` 交互：在组件中调用 `ProcessAction.listWithApplication` 等方法完成 列出 等操作。
- 与后端 `AppInfoAction` 交互：在组件中调用 `AppInfoAction.get` 等方法完成 读取 等操作。

## Dependencies

**后端服务（o2server action / REST）：**
- `x_auditlog_assemble_control`
- `x_cms_assemble_control`
- `x_portal_assemble_designer`
- `x_processplatform_assemble_designer`
- `x_program_center`
- `x_query_assemble_designer`
**前端基础设施：**
- O2OA web 框架（MWF / o2.Actions / o2.xDesktop）、Vue 组件或 MooTools Class 组件模型
- 公共库：`o2_lib`（marked、PinYin 等）、`$OOUI` 组件库
