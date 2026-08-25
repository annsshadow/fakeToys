# x_component_systemconfig

## Responsibility

系统配置组件，负责运行时的系统参数配置和管理。

## Entry Point

- `x_component_systemconfig/package.json (no Main.js; modern build entry)`

## Build Pipeline

- `serve`: vue-cli-service serve
- `build`: vue-cli-service build
- `deploy`: vue-cli-service build && vue-cli-service deploy
- `o2-build`: vue-cli-service build --dest ../../../target/o2server/servers/webServer/x_component_systemconfig
- `o2-deploy`: vue-cli-service build --dest ../../dest/x_component_systemconfig

## Key Configuration Files

- `package-lock.json`: *To be described.*
- `node_modules\.package-lock.json`: *To be described.*
- `node_modules\@babel\compat-data\data\corejs2-built-ins.json`: *To be described.*
- `node_modules\@babel\compat-data\data\corejs3-shipped-proposals.json`: *To be described.*
- `node_modules\@babel\compat-data\data\native-modules.json`: *To be described.*
- `node_modules\@babel\compat-data\data\overlapping-plugins.json`: *To be described.*
- `node_modules\@babel\compat-data\data\plugin-bugfixes.json`: *To be described.*
- `node_modules\@babel\compat-data\data\plugins.json`: *To be described.*
- `node_modules\@vue\cli-service\generator\template\jsconfig.json`: *To be described.*
- `node_modules\@vue\cli-service\types\tsconfig.json`: *To be described.*

- [List of key JSON config files with one-line explanations]

## Key Flows

- 与后端 `ConfigAction` 交互：在组件中调用 `ConfigAction.changePassword`、`getLicenseInfo`、`open`、`openRuntimeConfig`、`save` 等方法完成 调用/读取/打开/打开/保存 等操作。
- 与后端 `ComponentAction` 交互：在组件中调用 `ComponentAction.delete`、`get`、`listAll` 等方法完成 删除/读取/列出 等操作。
- 与后端 `DeployAction` 交互：在组件中调用 `DeployAction.get`、`listPaging` 等方法完成 读取/列出 等操作。
- 与后端 `ApplicationAction` 交互：在组件中调用 `ApplicationAction.listWithPerson` 等方法完成 列出 等操作。
- 与后端 `PortalAction` 交互：在组件中调用 `PortalAction.list` 等方法完成 列出 等操作。
- 与后端 `AppInfoAction` 交互：在组件中调用 `AppInfoAction.listWhatICanView_AllType` 等方法完成 列出 等操作。

## Dependencies

**后端服务（o2server action / REST）：**
- `x_cms_assemble_control`
- `x_component_assemble_control`
- `x_portal_assemble_surface`
- `x_processplatform_assemble_surface`
- `x_program_center`
- `x_query_assemble_surface`
- `x_query_service_processing`
**依赖的其它 o2web 组件 / 应用：**
- `Selector.package`
- `Template.widget.CronPicker`
- `ConfigDesigner`（openApplication 打开的应用）
**前端基础设施：**
- O2OA web 框架（MWF / o2.Actions / o2.xDesktop）、Vue 组件或 MooTools Class 组件模型
- 公共库：`o2_lib`（marked、PinYin 等）、`$OOUI` 组件库
