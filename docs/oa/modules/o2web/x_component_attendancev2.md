# x_component_attendancev2

## Responsibility

考勤管理组件（v2），负责新版考勤界面的展示和交互操作。

## Entry Point

- `x_component_attendancev2/package.json (no Main.js; modern build entry)`

## Build Pipeline

- `build`: cross-env NODE_ENV=production BUILD_PATH=dist PUBLIC_URL=/x_component_attendancev2/ webpack
- `start`: cross-env NODE_ENV=development BUILD_PATH=dist PUBLIC_URL=/x_component_attendancev2/ webpack-dev-server
- `o2-deploy`: cross-env NODE_ENV=production BUILD_PATH=../../dest/x_component_attendancev2 PUBLIC_URL=/x_component_attendancev2/ webpack
- `o2-build`: cross-env NODE_ENV=production BUILD_PATH=../../../target/o2server/servers/webServer/x_component_attendancev2 PUBLIC_URL=../x_component_attendancev2/ webpack

## Key Configuration Files

- `package-lock.json`: *To be described.*
- `node_modules\.package-lock.json`: *To be described.*
- `node_modules\@babel\compat-data\data\corejs2-built-ins.json`: *To be described.*
- `node_modules\@babel\compat-data\data\corejs3-shipped-proposals.json`: *To be described.*
- `node_modules\@babel\compat-data\data\native-modules.json`: *To be described.*
- `node_modules\@babel\compat-data\data\overlapping-plugins.json`: *To be described.*
- `node_modules\@babel\compat-data\data\plugin-bugfixes.json`: *To be described.*
- `node_modules\@babel\compat-data\data\plugins.json`: *To be described.*
- `node_modules\@babel\helper-globals\data\browser-upper.json`: *To be described.*
- `node_modules\@babel\helper-globals\data\builtin-lower.json`: *To be described.*

- [List of key JSON config files with one-line explanations]

## Key Flows

- 与后端 `LeaveAction` 交互：在组件中调用 `LeaveAction.input` 等方法完成 调用 等操作。
- 与后端 `RecordAction` 交互：在组件中调用 `RecordAction.input` 等方法完成 调用 等操作。
- 跨组件跳转：通过 `openApplication` 打开其它应用（aliface）。
- 依赖其它组件能力：通过 `requireApp` 引入 Template.widget.CronPicker 等组件模块。

## Dependencies

**后端服务（o2server action / REST）：**
- `x_attendance_assemble_control`
- `x_organization_assemble_control`
**依赖的其它 o2web 组件 / 应用：**
- `Template.widget.CronPicker`
- `aliface`（openApplication 打开的应用）
**前端基础设施：**
- O2OA web 框架（MWF / o2.Actions / o2.xDesktop）、Vue 组件或 MooTools Class 组件模型
- 公共库：`o2_lib`（marked、PinYin 等）、`$OOUI` 组件库
