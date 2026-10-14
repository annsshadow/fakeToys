# x_component_cms_ScriptDesigner

## Responsibility

CMS（内容管理）前端组件，负责脚本编辑的界面展示与交互操作，通过 REST API 与后端模块通信。

## Entry Point

- `oa\o2web\source\x_component_cms_ScriptDesigner\Main.js`

## Build Pipeline

- No build scripts detected

## Key Configuration Files

- No JSON config files detected

- [List of key JSON config files with one-line explanations]

## Key Flows

- 与后端 `ScriptItemAction` 交互：在组件中调用 `ScriptItemAction.addEvent` 等方法完成 添加 等操作。
- 与后端 `ScriptVersionAction` 交互：在组件中调用 `ScriptVersionAction.get`、`listWithScript` 等方法完成 读取/列出 等操作。
- 跨组件跳转：通过 `openApplication` 打开其它应用（cms）。

## Dependencies

**后端服务（o2server action / REST）：**
- `x_cms_assemble_control`
**依赖的其它 o2web 组件 / 应用：**
- `cms`（openApplication 打开的应用）
**前端基础设施：**
- O2OA web 框架（MWF / o2.Actions / o2.xDesktop）、Vue 组件或 MooTools Class 组件模型
- 公共库：`o2_lib`（marked、PinYin 等）、`$OOUI` 组件库
