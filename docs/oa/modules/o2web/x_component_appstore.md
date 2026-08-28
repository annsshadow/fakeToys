# x_component_appstore

## Responsibility

应用商店组件，负责应用的分发和安装管理。

## Entry Point

- `x_component_appstore/package.json (no Main.js; modern build entry)`

## Build Pipeline

- `serve`: vue-cli-service serve
- `build`: vue-cli-service build
- `deploy`: vue-cli-service build && vue-cli-service deploy
- `o2-deploy`: vue-cli-service build --dest ../../dest/x_component_appstore
- `o2-build`: vue-cli-service build --dest ../../../target/o2server/servers/webServer/x_component_appstore

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

- 与后端 `MarketAction` 交互：在组件中调用 `MarketAction.installOffline`、`installOrUpdate`、`listCategory` 等方法完成 调用/调用/列出 等操作。
- 与后端 `CollectAction` 交互：在组件中调用 `CollectAction.login` 等方法完成 调用 等操作。
- 核心交互：应用商店组件，负责应用的分发和安装管理。

## Dependencies

**后端服务（o2server action / REST）：**
- `x_program_center`
**前端基础设施：**
- O2OA web 框架（MWF / o2.Actions / o2.xDesktop）、Vue 组件或 MooTools Class 组件模型
- 公共库：`o2_lib`（marked、PinYin 等）、`$OOUI` 组件库
