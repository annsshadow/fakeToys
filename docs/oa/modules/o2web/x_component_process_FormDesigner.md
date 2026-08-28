# x_component_process_FormDesigner

## Responsibility

流程引擎前端组件，负责的界面展示与交互操作。

## Entry Point

- `oa\o2web\source\x_component_process_FormDesigner\Main.js`

## Build Pipeline

- No build scripts detected

## Key Configuration Files

- `$Main\bottom\tools-element.json`: *To be described.*
- `$Main\bottom\tools-filetext.json`: *To be described.*
- `$Main\bottom\tools-form.json`: *To be described.*
- `$Main\bottom\tools-function.json`: *To be described.*
- `$Main\bottom\tools-layout.json`: *To be described.*
- `$Main\bottom\tools-o2oa.json`: *To be described.*
- `$Main\bottom\tools-process.json`: *To be described.*
- `$Main\bottom\tools.json`: *To be described.*
- `$Main\bottom\toolsGroup.json`: *To be described.*
- `$Main\default\tools-element.json`: *To be described.*

- [List of key JSON config files with one-line explanations]

## Key Flows

- 与后端 `JsonAction` 交互：在组件中调用 `JsonAction.addEvent`、`setStyle` 等方法完成 添加/设置 等操作。
- 与后端 `FormAction` 交互：在组件中调用 `FormAction.addEvent`、`setStyle`、`update` 等方法完成 添加/设置/更新 等操作。
- 与后端 `ComponentAction` 交互：在组件中调用 `ComponentAction.listAll` 等方法完成 列出 等操作。
- 与后端 `StatementAction` 交互：在组件中调用 `StatementAction.get` 等方法完成 读取 等操作。
- 与后端 `ResourceAction` 交互：在组件中调用 `ResourceAction.address`、`list`、`sync` 等方法完成 添加/列出/调用 等操作。
- 与后端 `FormVersionAction` 交互：在组件中调用 `FormVersionAction.get`、`listWithForm` 等方法完成 读取/列出 等操作。

## Dependencies

**后端服务（o2server action / REST）：**
- `x_cms_assemble_control`
- `x_component_assemble_control`
- `x_custom_smartbi_assemble_control`
- `x_portal_assemble_designer`
- `x_portal_assemble_surface`
- `x_processplatform_assemble_designer`
- `x_processplatform_assemble_surface`
- `x_program_center`
**依赖的其它 o2web 组件 / 应用：**
- `process.FormDesigner.History`
- `process.FormDesigner.lp.`
- `process.FormDesigner.widget.ElTreeEditor`
- `portal`（openApplication 打开的应用）
- `process`（openApplication 打开的应用）
**前端基础设施：**
- O2OA web 框架（MWF / o2.Actions / o2.xDesktop）、Vue 组件或 MooTools Class 组件模型
- 公共库：`o2_lib`（marked、PinYin 等）、`$OOUI` 组件库
