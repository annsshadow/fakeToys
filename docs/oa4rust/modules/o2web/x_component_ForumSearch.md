# x_component_ForumSearch

## Responsibility

论坛搜索组件，负责帖子和话题的全文检索功能。

## Entry Point

- `oa\o2web\source\x_component_ForumSearch\Main.js`

## Build Pipeline

- No build scripts detected

## Key Configuration Files

- `$Main\default\listItem.json`: *To be described.*

- [List of key JSON config files with one-line explanations]

## Key Flows

- 跨组件跳转：通过 `openApplication` 打开其它应用（Forum、ForumDocument、ForumSection）。
- 核心交互：论坛搜索组件，负责帖子和话题的全文检索功能。
- 核心交互：论坛搜索组件，负责帖子和话题的全文检索功能。

## Dependencies

**依赖的其它 o2web 组件 / 应用：**
- `Forum`（openApplication 打开的应用）
- `ForumDocument`（openApplication 打开的应用）
- `ForumSection`（openApplication 打开的应用）
**前端基础设施：**
- O2OA web 框架（MWF / o2.Actions / o2.xDesktop）、Vue 组件或 MooTools Class 组件模型
- 公共库：`o2_lib`（marked、PinYin 等）、`$OOUI` 组件库
