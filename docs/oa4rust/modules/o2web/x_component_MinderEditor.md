# x_component_MinderEditor

## Responsibility

脑图编辑器，负责思维导图的创建、编辑和节点管理。

## Entry Point

- `oa\o2web\source\x_component_MinderEditor\Main.js`

## Build Pipeline

- No build scripts detected

## Key Configuration Files

- `Actions\action.json`: *To be described.*

- [List of key JSON config files with one-line explanations]

## Key Flows

- 跨组件跳转：通过 `openApplication` 打开其它应用（Minder）。
- 核心交互：脑图编辑器，负责思维导图的创建、编辑和节点管理。
- 核心交互：脑图编辑器，负责思维导图的创建、编辑和节点管理。

## Dependencies

**后端服务（o2server action / REST）：**
- `x_file_assemble_control`
**依赖的其它 o2web 组件 / 应用：**
- `Minder`（openApplication 打开的应用）
**前端基础设施：**
- O2OA web 框架（MWF / o2.Actions / o2.xDesktop）、Vue 组件或 MooTools Class 组件模型
- 公共库：`o2_lib`（marked、PinYin 等）、`$OOUI` 组件库
