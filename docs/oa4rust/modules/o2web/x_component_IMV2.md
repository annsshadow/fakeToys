# x_component_IMV2

## Responsibility

即时通讯组件（v2），负责消息收发和会话管理界面。

## Entry Point

- `x_component_IMV2/package.json (no Main.js; modern build entry)`

## Build Pipeline

- `dev`: vite
- `build`: vite build
- `o2-build`: vite build
- `o2-deploy`: vite build

## Key Configuration Files

- `o2.config.json`: *To be described.*
- `package-lock.json`: *To be described.*
- `node_modules\.package-lock.json`: *To be described.*
- `node_modules\@o2oa\util\jsdoc.config.json`: *To be described.*
- `node_modules\entities\src\generated\.eslintrc.json`: *To be described.*
- `node_modules\uglify-js\tools\domprops.json`: *To be described.*
- `src\assets\emoji.json`: *To be described.*

- [List of key JSON config files with one-line explanations]

## Key Flows

- 与后端 `TaskAction` 交互：在组件中调用 `TaskAction.V2ListPaging` 等方法完成 调用 等操作。
- 与后端 `ImAction` 交互：在组件中调用 `ImAction.uploadFile` 等方法完成 上传 等操作。
- 跨组件跳转：通过 `openApplication` 打开其它应用（OnlyOfficeEditor）。
- 依赖其它组件能力：通过 `requireApp` 引入 Selector.package 等组件模块。

## Dependencies

**后端服务（o2server action / REST）：**
- `x_message_assemble_communicate`
- `x_processplatform_assemble_surface`
**依赖的其它 o2web 组件 / 应用：**
- `Selector.package`
- `OnlyOfficeEditor`（openApplication 打开的应用）
**前端基础设施：**
- O2OA web 框架（MWF / o2.Actions / o2.xDesktop）、Vue 组件或 MooTools Class 组件模型
- 公共库：`o2_lib`（marked、PinYin 等）、`$OOUI` 组件库
