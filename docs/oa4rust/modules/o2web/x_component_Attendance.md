# x_component_Attendance

## Responsibility

考勤管理组件，负责考勤记录的查看、打卡统计和申诉管理。

## Entry Point

- `oa\o2web\source\x_component_Attendance\Main.js`

## Build Pipeline

- No build scripts detected

## Key Configuration Files

- `$AbnormalExport\listItem.json`: *To be described.*
- `$AbnormalExport\toolbar.json`: *To be described.*
- `$AddressExplorer\toolbar.json`: *To be described.*
- `$AppealExplorer\listItem.json`: *To be described.*
- `$AppealExplorer\toolbar.json`: *To be described.*
- `$Common\fullcalendar\demos\json\events.json`: *To be described.*
- `$Explorer\listItem.json`: *To be described.*
- `$Explorer\toolbar.json`: *To be described.*
- `$HolidayExplorer\listItem.json`: *To be described.*
- `$HolidayExplorer\toolbar.json`: *To be described.*

- [List of key JSON config files with one-line explanations]

## Key Flows

- 与后端 `AttendanceSettingAction` 交互：在组件中调用 `AttendanceSettingAction.enableType` 等方法完成 调用 等操作。
- 与后端 `DingdingAttendanceAction` 交互：在组件中调用 `DingdingAttendanceAction.listNextDingdingAttendance` 等方法完成 列出 等操作。
- 与后端 `DingdingAttendanceStatisticAction` 交互：在组件中调用 `DingdingAttendanceStatisticAction.personMonth`、`personMonthWithUnit`、`unitMonth` 等方法完成 调用/调用/调用 等操作。
- 与后端 `QywxAttendanceAction` 交互：在组件中调用 `QywxAttendanceAction.listDingdingAttendance` 等方法完成 列出 等操作。
- 与后端 `QywxAttendanceStatisticAction` 交互：在组件中调用 `QywxAttendanceStatisticAction.personMonth`、`personMonthWithUnit`、`unitMonth` 等方法完成 调用/调用/调用 等操作。
- 跨组件跳转：通过 `openApplication` 打开其它应用（cms、process）。

## Dependencies

**后端服务（o2server action / REST）：**
- `x_attendance_assemble_control`
**依赖的其它 o2web 组件 / 应用：**
- `cms`（openApplication 打开的应用）
- `process`（openApplication 打开的应用）
**前端基础设施：**
- O2OA web 框架（MWF / o2.Actions / o2.xDesktop）、Vue 组件或 MooTools Class 组件模型
- 公共库：`o2_lib`（marked、PinYin 等）、`$OOUI` 组件库
