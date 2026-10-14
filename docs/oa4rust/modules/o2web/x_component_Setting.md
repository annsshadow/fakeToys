# x_component_Setting

## Responsibility

系统设置组件，负责系统参数的配置和管理。

## Entry Point

- `oa\o2web\source\x_component_Setting\Main.js`

## Build Pipeline

- No build scripts detected

## Key Configuration Files

- `components.json`: *To be described.*

- [List of key JSON config files with one-line explanations]

## Key Flows

- 与后端 `ConfigAction` 交互：在组件中调用 `ConfigAction.getProxy`、`getTernaryManagement`、`setTernaryManagement` 等方法完成 读取/读取/设置 等操作。
- 与后端 `MPWeixinAction` 交互：在组件中调用 `MPWeixinAction.menuAdd`、`menuCreate2Weixin`、`menuDelete`、`menuUpdate`、`menuWeixinList`、`menuWeixinSubscribe` 等方法完成 调用/调用/调用/调用/调用/调用 等操作。
- 与后端 `CollectAction` 交互：在组件中调用 `CollectAction.mobileCheckConnect` 等方法完成 调用 等操作。
- 与后端 `AppPackAction` 交互：在组件中调用 `AppPackAction.androidPackReStart`、`androidPackStart`、`connect`、`packInfo`、`publishApk` 等方法完成 调用/调用/调用/调用/发布 等操作。
- 与后端 `ModuleAction` 交互：在组件中调用 `ModuleAction.dispatchResource` 等方法完成 调用 等操作。
- 与后端 `CommandAction` 交互：在组件中调用 `CommandAction.getNodeInfoList`、`upload` 等方法完成 读取/上传 等操作。

## Dependencies

**后端服务（o2server action / REST）：**
- `x_program_center`
**依赖的其它 o2web 组件 / 应用：**
- `Collect`（openApplication 打开的应用）
- `ConfigDesigner`（openApplication 打开的应用）
**前端基础设施：**
- O2OA web 框架（MWF / o2.Actions / o2.xDesktop）、Vue 组件或 MooTools Class 组件模型
- 公共库：`o2_lib`（marked、PinYin 等）、`$OOUI` 组件库
