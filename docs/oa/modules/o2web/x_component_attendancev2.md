# x_component_attendancev2

## Responsibility



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
