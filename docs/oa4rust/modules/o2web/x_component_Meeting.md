# x_component_Meeting

## Responsibility

会议管理组件，负责会议的预约、日程查看和参与管理。

## Entry Point

- `oa\o2web\source\x_component_Meeting\Main.js`

## Build Pipeline

- No build scripts detected

## Key Configuration Files

- `$ListView\navi.json`: *To be described.*

- [List of key JSON config files with one-line explanations]

## Key Flows

- 与后端 `BuildingAction` 交互：在组件中调用 `BuildingAction.listWithStartCompletedRoom` 等方法完成 列出 等操作。
- 与后端 `MeetingAction` 交互：在组件中调用 `MeetingAction.addClass`、`editCompletedTime`、`editStartTime`、`get`、`listApplyMeetingPaging`、`listInviteMeetingPaging`、`listWaitAccept`、`removeClass` 等方法完成 添加/调用/调用/读取/列出/列出 等操作。
- 与后端 `IdentityAction` 交互：在组件中调用 `IdentityAction.listWithPerson` 等方法完成 列出 等操作。
- 与后端 `ConfigAction` 交互：在组件中调用 `ConfigAction.getConfigManage`、`getSystemConfig`、`saveSystemConfig` 等方法完成 读取/读取/保存 等操作。
- 跨组件跳转：通过 `openApplication` 打开其它应用（process）。

## Dependencies

**后端服务（o2server action / REST）：**
- `x_meeting_assemble_control`
- `x_organization_assemble_express`
**依赖的其它 o2web 组件 / 应用：**
- `process`（openApplication 打开的应用）
**前端基础设施：**
- O2OA web 框架（MWF / o2.Actions / o2.xDesktop）、Vue 组件或 MooTools Class 组件模型
- 公共库：`o2_lib`（marked、PinYin 等）、`$OOUI` 组件库
