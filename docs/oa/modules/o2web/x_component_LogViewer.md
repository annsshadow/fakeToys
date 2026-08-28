# x_component_LogViewer

## Responsibility

日志查看器组件，负责系统日志和运行日志的浏览和筛选。

## Entry Point

- `oa\o2web\source\x_component_LogViewer\Main.js`

## Build Pipeline

- No build scripts detected

## Key Configuration Files

- No JSON config files detected

- [List of key JSON config files with one-line explanations]

## Key Flows

- 与后端 `CommandAction` 交互：在组件中调用 `CommandAction.executeCommand`、`getNodeInfoList` 等方法完成 执行/读取 等操作。
- 核心交互：日志查看器组件，负责系统日志和运行日志的浏览和筛选。
- 核心交互：日志查看器组件，负责系统日志和运行日志的浏览和筛选。

## Dependencies

**后端服务（o2server action / REST）：**
- `x_program_center`
**前端基础设施：**
- O2OA web 框架（MWF / o2.Actions / o2.xDesktop）、Vue 组件或 MooTools Class 组件模型
- 公共库：`o2_lib`（marked、PinYin 等）、`$OOUI` 组件库
