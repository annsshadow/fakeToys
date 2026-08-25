# x_component_appstore_application

## Responsibility

应用商店应用详情组件，负责单个应用的安装和配置。

## Entry Point

- `x_component_appstore_application/package.json (no Main.js; modern build entry)`

## Build Pipeline

- `start`: react-scripts start
- `build`: react-scripts build
- `test`: react-scripts test
- `eject`: react-scripts eject
- `o2-deploy`: cross-env BUILD_PATH=../../dest/x_component_appstore_application PUBLIC_URL=../x_component_appstore_application/ react-scripts build
- `o2-build`: cross-env BUILD_PATH=../../../target/o2server/servers/webServer/x_component_appstore_application PUBLIC_URL=../x_component_appstore_application/ react-scripts build

## Key Configuration Files

- `package-lock.json`: *To be described.*
- `node_modules\.package-lock.json`: *To be described.*
- `node_modules\.cache\babel-loader\054a503446581106a4a8327e81078714f966da48aa78c90712404349988be42e.json`: *To be described.*
- `node_modules\.cache\babel-loader\08e4d7ddbf30cf3ef290ac525ef6e0b5c7985de1063e6711361b694b0791cd99.json`: *To be described.*
- `node_modules\.cache\babel-loader\3481c632c730cfcdb68de2eb56d2a420a4ac1acddffbcd7dd36e902845f8609f.json`: *To be described.*
- `node_modules\.cache\babel-loader\3489e87f7bb613d861f254d7307a48d2986d71e604ae2cb997c641c7e29854fa.json`: *To be described.*
- `node_modules\.cache\babel-loader\416e8c149b927dd651817e0abfc9b59ffc1eef980071da2e3213268a6d818d88.json`: *To be described.*
- `node_modules\.cache\babel-loader\4530dd3782f4a794f063cafd9c18cbbbe498e74d7a6d3f332306b4dacf735852.json`: *To be described.*
- `node_modules\.cache\babel-loader\65b0a040fe69e749f6614863bf8bc8e04502e7f3c1bfbacfbc978fb52ffc7681.json`: *To be described.*
- `node_modules\.cache\babel-loader\83f67fde0fde589b6b1d166dfa8830dce7867136090dfea47d4057d09d1fc063.json`: *To be described.*

- [List of key JSON config files with one-line explanations]

## Key Flows

- 应用商店应用详情组件，负责单个应用的安装和配置。（源码中未检出显式后端 action 调用，主要在前端完成交互与渲染。）
- 核心交互：应用商店应用详情组件，负责单个应用的安装和配置。
- 核心交互：应用商店应用详情组件，负责单个应用的安装和配置。

## Dependencies

**后端服务（o2server action / REST）：**
- `x_program_center`
**前端基础设施：**
- O2OA web 框架（MWF / o2.Actions / o2.xDesktop）、Vue 组件或 MooTools Class 组件模型
- 公共库：`o2_lib`（marked、PinYin 等）、`$OOUI` 组件库
