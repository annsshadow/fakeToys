# x_component_process_workcenter

## Responsibility

流程引擎前端组件，负责办公中心的界面展示与交互操作。

## Entry Point

- `oa\o2web\source\x_component_process_workcenter\Main.js`

## Build Pipeline

- No build scripts detected

## Key Configuration Files

- No JSON config files detected

- [List of key JSON config files with one-line explanations]

## Key Flows

- 与后端 `WorkAction` 交互：在组件中调用 `WorkAction.countWithPerson` 等方法完成 统计 等操作。
- 与后端 `DraftAction` 交互：在组件中调用 `DraftAction.listMyFilterPaging`、`listMyPaging` 等方法完成 列出/列出 等操作。
- 与后端 `ReviewAction` 交互：在组件中调用 `ReviewAction.V2ListCreatePaging`、`V2ListPaging`、`countWithPerson`、`filterCreateEntry`、`filterEntry` 等方法完成 调用/调用/统计/调用/调用 等操作。
- 与后端 `ApplicationAction` 交互：在组件中调用 `ApplicationAction.getIcon`、`listWithPersonAndTerminal`、`listWithPersonComplex` 等方法完成 读取/列出/列出 等操作。
- 与后端 `AppInfoAction` 交互：在组件中调用 `AppInfoAction.listPublishWithProcess` 等方法完成 列出 等操作。
- 与后端 `TaskAction` 交互：在组件中调用 `TaskAction.get`、`listMyFilterPaging`、`processing` 等方法完成 读取/列出/调用 等操作。

## Dependencies

**后端服务（o2server action / REST）：**
- `x_cms_assemble_control`
- `x_processplatform_assemble_surface`
**依赖的其它 o2web 组件 / 应用：**
- `process`（openApplication 打开的应用）
**前端基础设施：**
- O2OA web 框架（MWF / o2.Actions / o2.xDesktop）、Vue 组件或 MooTools Class 组件模型
- 公共库：`o2_lib`（marked、PinYin 等）、`$OOUI` 组件库
