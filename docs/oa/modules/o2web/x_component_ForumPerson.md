# x_component_ForumPerson

## Responsibility

论坛个人中心组件，负责用户在论坛中的活动和设置管理。

## Entry Point

- `oa\o2web\source\x_component_ForumPerson\Main.js`

## Build Pipeline

- No build scripts detected

## Key Configuration Files

- `$Main\default\listItem.json`: *To be described.*
- `$Main\default\listItemReply.json`: *To be described.*

- [List of key JSON config files with one-line explanations]

## Key Flows

- 跨组件跳转：通过 `openApplication` 打开其它应用（Forum、ForumDocument、ForumSection、Profile）。
- 核心交互：论坛个人中心组件，负责用户在论坛中的活动和设置管理。
- 核心交互：论坛个人中心组件，负责用户在论坛中的活动和设置管理。

## Dependencies

**依赖的其它 o2web 组件 / 应用：**
- `Forum`（openApplication 打开的应用）
- `ForumDocument`（openApplication 打开的应用）
- `ForumSection`（openApplication 打开的应用）
- `Profile`（openApplication 打开的应用）
**前端基础设施：**
- O2OA web 框架（MWF / o2.Actions / o2.xDesktop）、Vue 组件或 MooTools Class 组件模型
- 公共库：`o2_lib`（marked、PinYin 等）、`$OOUI` 组件库
