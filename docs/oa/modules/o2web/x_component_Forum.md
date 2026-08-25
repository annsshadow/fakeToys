# x_component_Forum

## Responsibility

论坛首页组件，负责论坛板块的展示和导航。

## Entry Point

- `oa\o2web\source\x_component_Forum\Main.js`

## Build Pipeline

- No build scripts detected

## Key Configuration Files

- `$ColumnTemplate\template\setting.json`: *To be described.*
- `$ColumnTemplate\template\type_1_0.json`: *To be described.*
- `$ColumnTemplate\template\type_1_1.json`: *To be described.*
- `$ColumnTemplate\template\type_2_0.json`: *To be described.*
- `$ColumnTemplate\template\type_2_0_2.json`: *To be described.*
- `$ColumnTemplate\template\type_2_1.json`: *To be described.*
- `$ColumnTemplate\template\type_2_1_2.json`: *To be described.*
- `$ColumnTemplate\template\type_3_0.json`: *To be described.*
- `$ColumnTemplate\template\type_3_1.json`: *To be described.*
- `$ColumnTemplate\template\type_4_0.json`: *To be described.*

- [List of key JSON config files with one-line explanations]

## Key Flows

- 与后端 `PermissionInfoAction` 交互：在组件中调用 `PermissionInfoAction.getUserPermission` 等方法完成 读取 等操作。
- 与后端 `ShutupAction` 交互：在组件中调用 `ShutupAction.delete`、`getShutup`、`listPaging`、`save` 等方法完成 删除/读取/列出/保存 等操作。
- 与后端 `PersonAction` 交互：在组件中调用 `PersonAction.getNickName`、`listObject` 等方法完成 读取/列出 等操作。
- 与后端 `BBSConfigSettingAction` 交互：在组件中调用 `BBSConfigSettingAction.getByCode` 等方法完成 读取 等操作。
- 与后端 `BBSConfigSettingAnonymousAction` 交互：在组件中调用 `BBSConfigSettingAnonymousAction.getBBSName` 等方法完成 读取 等操作。
- 跨组件跳转：通过 `openApplication` 打开其它应用（ForumCategory、ForumDocument、ForumPerson、ForumSearch、ForumSection）。

## Dependencies

**后端服务（o2server action / REST）：**
- `x_bbs_assemble_control`
- `x_organization_assemble_express`
**依赖的其它 o2web 组件 / 应用：**
- `ForumCategory`（openApplication 打开的应用）
- `ForumDocument`（openApplication 打开的应用）
- `ForumPerson`（openApplication 打开的应用）
- `ForumSearch`（openApplication 打开的应用）
- `ForumSection`（openApplication 打开的应用）
**前端基础设施：**
- O2OA web 框架（MWF / o2.Actions / o2.xDesktop）、Vue 组件或 MooTools Class 组件模型
- 公共库：`o2_lib`（marked、PinYin 等）、`$OOUI` 组件库
