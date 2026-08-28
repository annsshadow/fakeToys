# x_component_Minder

## Responsibility

脑图组件，负责思维导图的浏览和基础操作。

## Entry Point

- `oa\o2web\source\x_component_Minder\Main.js`

## Build Pipeline

- No build scripts detected

## Key Configuration Files

- `$Main\icon.json`: *To be described.*
- `$MineExplorer\default\listItem.json`: *To be described.*
- `$MineExplorer\default\listItem_received.json`: *To be described.*
- `$MineExplorer\default\listItem_recycle.json`: *To be described.*
- `$MineExplorer\default\listItem_shared.json`: *To be described.*
- `$MineExplorer\default\tileItem.json`: *To be described.*
- `$MineExplorer\default\tileItem_recycle.json`: *To be described.*

- [List of key JSON config files with one-line explanations]

## Key Flows

- 与后端 `MindFolderInfoAction` 交互：在组件中调用 `MindFolderInfoAction.treeMyFolder` 等方法完成 调用 等操作。
- 与后端 `MindInfoAction` 交互：在组件中调用 `MindInfoAction.destoryFromNormal`、`listNextWithFilter` 等方法完成 调用/列出 等操作。
- 跨组件跳转：通过 `openApplication` 打开其它应用（MinderEditor、ReportDocument）。

## Dependencies

**后端服务（o2server action / REST）：**
- `x_mind_assemble_control`
**依赖的其它 o2web 组件 / 应用：**
- `MinderEditor`（openApplication 打开的应用）
- `ReportDocument`（openApplication 打开的应用）
**前端基础设施：**
- O2OA web 框架（MWF / o2.Actions / o2.xDesktop）、Vue 组件或 MooTools Class 组件模型
- 公共库：`o2_lib`（marked、PinYin 等）、`$OOUI` 组件库
