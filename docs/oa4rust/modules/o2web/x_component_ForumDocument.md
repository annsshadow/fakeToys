# x_component_ForumDocument

## Responsibility

论坛帖子组件，负责帖子的阅读、回复和管理操作。

## Entry Point

- `oa\o2web\source\x_component_ForumDocument\Main.js`

## Build Pipeline

- No build scripts detected

## Key Configuration Files

- `$Main\default\listItemAnonymousSubject.json`: *To be described.*
- `$Main\default\listItemReply.json`: *To be described.*
- `$Main\default\listItemSatisfied.json`: *To be described.*
- `$Main\default\listItemSubject.json`: *To be described.*
- `$Mobile\default\listItemAnonymousSubject.json`: *To be described.*
- `$Mobile\default\listItemReply.json`: *To be described.*
- `$Mobile\default\listItemSatisfied.json`: *To be described.*
- `$Mobile\default\listItemSubject.json`: *To be described.*
- `$Vote\listItemVote.json`: *To be described.*

- [List of key JSON config files with one-line explanations]

## Key Flows

- 与后端 `ReplyAction` 交互：在组件中调用 `ReplyAction.addEvent`、`postMessage` 等方法完成 添加/调用 等操作。
- 与后端 `SubjectInfoManagerUserAction` 交互：在组件中调用 `SubjectInfoManagerUserAction.save` 等方法完成 保存 等操作。
- 跨组件跳转：通过 `openApplication` 打开其它应用（Forum、ForumCategory、ForumSection）。

## Dependencies

**后端服务（o2server action / REST）：**
- `x_bbs_assemble_control`
**依赖的其它 o2web 组件 / 应用：**
- `Forum`（openApplication 打开的应用）
- `ForumCategory`（openApplication 打开的应用）
- `ForumSection`（openApplication 打开的应用）
**前端基础设施：**
- O2OA web 框架（MWF / o2.Actions / o2.xDesktop）、Vue 组件或 MooTools Class 组件模型
- 公共库：`o2_lib`（marked、PinYin 等）、`$OOUI` 组件库
