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
