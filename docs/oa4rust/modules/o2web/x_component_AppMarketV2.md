# x_component_AppMarketV2

## Responsibility

应用市场组件，负责应用的分类浏览、搜索和安装管理。

## Entry Point

- `oa\o2web\source\x_component_AppMarketV2\Main.js`

## Build Pipeline

- No build scripts detected

## Key Configuration Files

- No JSON config files detected

- [List of key JSON config files with one-line explanations]

## Key Flows

- 与后端 `MarketAction` 交互：在组件中调用 `MarketAction.getCoverPic`、`installOrUpdate`、`listCategory`、`listPaging` 等方法完成 读取/调用/列出/列出 等操作。
- 与后端 `CollectAction` 交互：在组件中调用 `CollectAction.bbs`、`login` 等方法完成 调用/调用 等操作。
- 核心交互：应用市场组件，负责应用的分类浏览、搜索和安装管理。

## Dependencies

**前端基础设施：**
- O2OA web 框架（MWF / o2.Actions / o2.xDesktop）、Vue 组件或 MooTools Class 组件模型
- 公共库：`o2_lib`（marked、PinYin 等）、`$OOUI` 组件库
