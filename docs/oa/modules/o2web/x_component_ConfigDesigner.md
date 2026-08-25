# x_component_ConfigDesigner

## Responsibility

平台配置设计器，负责系统平台配置项的可视化编辑和管理。

## Entry Point

- `oa\o2web\source\x_component_ConfigDesigner\Main.js`

## Build Pipeline

- No build scripts detected

## Key Configuration Files

- `$Main\default\tools.json`: *To be described.*

- [List of key JSON config files with one-line explanations]

## Key Flows

- 与后端 `ConfigAction` 交互：在组件中调用 `ConfigAction.getList`、`open`、`save` 等方法完成 读取/打开/保存 等操作。
- 与后端 `ScriptItemAction` 交互：在组件中调用 `ScriptItemAction.addEvent` 等方法完成 添加 等操作。
- 与后端 `CommandAction` 交互：在组件中调用 `CommandAction.getNodeInfoList` 等方法完成 读取 等操作。

## Dependencies

**后端服务（o2server action / REST）：**
- `x_program_center`
**前端基础设施：**
- O2OA web 框架（MWF / o2.Actions / o2.xDesktop）、Vue 组件或 MooTools Class 组件模型
- 公共库：`o2_lib`（marked、PinYin 等）、`$OOUI` 组件库
