# x_component_process_Application

## Responsibility

流程引擎前端组件，负责流程应用的界面展示与交互操作。

## Entry Point

- `oa\o2web\source\x_component_process_Application\Main.js`

## Build Pipeline

- No build scripts detected

## Key Configuration Files

- No JSON config files detected

- [List of key JSON config files with one-line explanations]

## Key Flows

- 与后端 `ApplicationAction` 交互：在组件中调用 `ApplicationAction.get`、`isManager` 等方法完成 读取/调用 等操作。
- 与后端 `ProcessAction` 交互：在组件中调用 `ProcessAction.getAllowRerouteTo`、`getComplex`、`listControllableWithApplication`、`listWithPersonWithApplication` 等方法完成 读取/读取/列出/列出 等操作。
- 与后端 `WorkAction` 交互：在组件中调用 `WorkAction.V2Reroute`、`countWithPersonAndApplication`、`delete`、`manageListWithApplicationPaging`、`processing` 等方法完成 调用/统计/删除/调用/调用 等操作。
- 与后端 `DraftAction` 交互：在组件中调用 `DraftAction.listMyPaging` 等方法完成 列出 等操作。
- 与后端 `ReviewAction` 交互：在组件中调用 `ReviewAction.V2ListPaging`、`createWithWork`、`createWithWorkCompleted`、`listWithJob`、`manageDelete` 等方法完成 调用/创建/创建/列出/调用 等操作。
- 与后端 `TaskAction` 交互：在组件中调用 `TaskAction.V2ListPaging`、`V2Reset`、`listWithJob`、`listWithWork`、`manageDelete`、`processing`、`v3Add` 等方法完成 调用/调用/列出/列出/调用/调用 等操作。

## Dependencies

**后端服务（o2server action / REST）：**
- `x_processplatform_assemble_designer`
- `x_processplatform_assemble_surface`
**依赖的其它 o2web 组件 / 应用：**
- `Selector.package`
- `process.Work.Processor`
- `process`（openApplication 打开的应用）
**前端基础设施：**
- O2OA web 框架（MWF / o2.Actions / o2.xDesktop）、Vue 组件或 MooTools Class 组件模型
- 公共库：`o2_lib`（marked、PinYin 等）、`$OOUI` 组件库
