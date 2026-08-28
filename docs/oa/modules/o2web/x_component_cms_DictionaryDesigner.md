# x_component_cms_DictionaryDesigner

## Responsibility

CMS（内容管理）前端组件，负责数据字典编辑的界面展示与交互操作，通过 REST API 与后端模块通信。

## Entry Point

- `oa\o2web\source\x_component_cms_DictionaryDesigner\Main.js`

## Build Pipeline

- No build scripts detected

## Key Configuration Files

- No JSON config files detected

- [List of key JSON config files with one-line explanations]

## Key Flows

- 跨组件跳转：通过 `openApplication` 打开其它应用（cms）。
- 依赖其它组件能力：通过 `requireApp` 引入 Selector.package 等组件模块。
- 核心交互：CMS（内容管理）前端组件，负责数据字典编辑的界面展示与交互操作，通过 REST API 与后端模块通信。

## Dependencies

**依赖的其它 o2web 组件 / 应用：**
- `Selector.package`
- `cms`（openApplication 打开的应用）
**前端基础设施：**
- O2OA web 框架（MWF / o2.Actions / o2.xDesktop）、Vue 组件或 MooTools Class 组件模型
- 公共库：`o2_lib`（marked、PinYin 等）、`$OOUI` 组件库
