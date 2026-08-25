# x_component_ftsearch

## Responsibility

全文搜索组件，提供对平台内容的全文检索功能。

## Entry Point

- `oa\o2web\source\x_component_ftsearch\Main.js`

## Build Pipeline

- No build scripts detected

## Key Configuration Files

- No JSON config files detected

- [List of key JSON config files with one-line explanations]

## Key Flows

- 与后端 `SearchAction` 交互：在组件中调用 `SearchAction.post` 等方法完成 调用 等操作。
- 依赖其它组件能力：通过 `requireApp` 引入 Template.MTooltips 等组件模块。
- 核心交互：全文搜索组件，提供对平台内容的全文检索功能。

## Dependencies

**后端服务（o2server action / REST）：**
- `x_query_assemble_surface`
**依赖的其它 o2web 组件 / 应用：**
- `Template.MTooltips`
**前端基础设施：**
- O2OA web 框架（MWF / o2.Actions / o2.xDesktop）、Vue 组件或 MooTools Class 组件模型
- 公共库：`o2_lib`（marked、PinYin 等）、`$OOUI` 组件库
