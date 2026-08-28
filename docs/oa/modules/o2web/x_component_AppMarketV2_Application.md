# x_component_AppMarketV2_Application

## Responsibility

应用市场应用详情组件，负责单个应用的详细信息展示和操作。

## Entry Point

- `oa\o2web\source\x_component_AppMarketV2_Application\Main.js`

## Build Pipeline

- No build scripts detected

## Key Configuration Files

- No JSON config files detected

- [List of key JSON config files with one-line explanations]

## Key Flows

- 与后端 `CollectAction` 交互：在组件中调用 `CollectAction.bbs`、`login` 等方法完成 调用/调用 等操作。
- 与后端 `MarketAction` 交互：在组件中调用 `MarketAction.get`、`installOrUpdate` 等方法完成 读取/调用 等操作。
- 依赖其它组件能力：通过 `requireApp` 引入 AppMarketV2.Application.Comment 等组件模块。

## Dependencies

**依赖的其它 o2web 组件 / 应用：**
- `AppMarketV2.Application.Comment`
**前端基础设施：**
- O2OA web 框架（MWF / o2.Actions / o2.xDesktop）、Vue 组件或 MooTools Class 组件模型
- 公共库：`o2_lib`（marked、PinYin 等）、`$OOUI` 组件库
