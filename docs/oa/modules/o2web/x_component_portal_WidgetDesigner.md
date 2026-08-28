# x_component_portal_WidgetDesigner

## Responsibility

门户前端组件，负责部件编辑的界面展示与交互操作。

## Entry Point

- `oa\o2web\source\x_component_portal_WidgetDesigner\Main.js`

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

- 跨组件跳转：通过 `openApplication` 打开其它应用（portal）。
- 核心交互：门户前端组件，负责部件编辑的界面展示与交互操作。
- 核心交互：门户前端组件，负责部件编辑的界面展示与交互操作。

## Dependencies

**依赖的其它 o2web 组件 / 应用：**
- `portal`（openApplication 打开的应用）
**前端基础设施：**
- O2OA web 框架（MWF / o2.Actions / o2.xDesktop）、Vue 组件或 MooTools Class 组件模型
- 公共库：`o2_lib`（marked、PinYin 等）、`$OOUI` 组件库
