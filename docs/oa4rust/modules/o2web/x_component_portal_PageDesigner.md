# x_component_portal_PageDesigner

## Responsibility

门户前端组件，负责页面编辑的界面展示与交互操作。

## Entry Point

- `oa\o2web\source\x_component_portal_PageDesigner\Main.js`

## Build Pipeline

- No build scripts detected

## Key Configuration Files

- `$Main\bottom\tools-data.json`: *To be described.*
- `$Main\bottom\tools-element.json`: *To be described.*
- `$Main\bottom\tools-form.json`: *To be described.*
- `$Main\bottom\tools-function.json`: *To be described.*
- `$Main\bottom\tools-layout.json`: *To be described.*
- `$Main\bottom\tools-o2oa.json`: *To be described.*
- `$Main\bottom\tools.json`: *To be described.*
- `$Main\bottom\toolsGroup.json`: *To be described.*
- `$Main\default\tools-data.json`: *To be described.*
- `$Main\default\tools-element.json`: *To be described.*

- [List of key JSON config files with one-line explanations]

## Key Flows

- 与后端 `PortalAction` 交互：在组件中调用 `PortalAction.list` 等方法完成 列出 等操作。
- 与后端 `PageVersionAction` 交互：在组件中调用 `PageVersionAction.get`、`listWithPage` 等方法完成 读取/列出 等操作。
- 与后端 `FormAction` 交互：在组件中调用 `FormAction.update` 等方法完成 更新 等操作。
- 跨组件跳转：通过 `openApplication` 打开其它应用（portal）。
- 依赖其它组件能力：通过 `requireApp` 引入 process.FormDesigner.History 等组件模块。

## Dependencies

**后端服务（o2server action / REST）：**
- `x_cms_assemble_control`
- `x_portal_assemble_designer`
- `x_processplatform_assemble_designer`
- `x_program_center`
**依赖的其它 o2web 组件 / 应用：**
- `process.FormDesigner.History`
- `portal`（openApplication 打开的应用）
**前端基础设施：**
- O2OA web 框架（MWF / o2.Actions / o2.xDesktop）、Vue 组件或 MooTools Class 组件模型
- 公共库：`o2_lib`（marked、PinYin 等）、`$OOUI` 组件库
