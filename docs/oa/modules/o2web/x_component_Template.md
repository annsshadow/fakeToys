# x_component_Template

## Responsibility

模板组件，负责表单模板、页面模板的加载和渲染。

## Entry Point

- `oa\o2web\source\x_component_Template\Main.js`

## Build Pipeline

- No build scripts detected

## Key Configuration Files

- `$Explorer\complexListItem.json`: *To be described.*
- `$Explorer\complexListItem2.json`: *To be described.*
- `$Explorer\listItem.json`: *To be described.*
- `$Explorer\toolbar.json`: *To be described.*
- `$Main\navi.json`: *To be described.*

- [List of key JSON config files with one-line explanations]

## Key Flows

- 跨组件跳转：通过 `openApplication` 打开其它应用（cms）。
- 核心交互：模板组件，负责表单模板、页面模板的加载和渲染。
- 核心交互：模板组件，负责表单模板、页面模板的加载和渲染。

## Dependencies

**后端服务（o2server action / REST）：**
- `x_cms_assemble_control`
- `x_processplatform_assemble_surface`
**依赖的其它 o2web 组件 / 应用：**
- `cms`（openApplication 打开的应用）
**前端基础设施：**
- O2OA web 框架（MWF / o2.Actions / o2.xDesktop）、Vue 组件或 MooTools Class 组件模型
- 公共库：`o2_lib`（marked、PinYin 等）、`$OOUI` 组件库
