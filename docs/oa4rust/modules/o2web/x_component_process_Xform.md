# x_component_process_Xform

## Responsibility

流程引擎前端组件，负责标题的界面展示与交互操作。

## Entry Point



## Build Pipeline

- No build scripts detected

## Key Configuration Files

- `$Form\toolbars.json`: *To be described.*
- `widget\action.json`: *To be described.*

- [List of key JSON config files with one-line explanations]

## Key Flows

- 与后端 `PersonAction` 交互：在组件中调用 `PersonAction.listObject` 等方法完成 列出 等操作。
- 与后端 `WorkAction` 交互：在组件中调用 `WorkAction.V2AddSplit`、`V2ListActivityGoBack`、`V2Reroute`、`V2Retract`、`V2Rollback`、`V2Terminate` 等方法完成 调用/调用/调用/调用/调用/调用 等操作。
- 与后端 `CorrelationAction` 交互：在组件中调用 `CorrelationAction.createWithJob`、`deleteWithJob`、`listWithJobWithSite`、`updateWithJob` 等方法完成 创建/删除/列出/更新 等操作。
- 与后端 `JobAction` 交互：在组件中调用 `JobAction.findWorkWorkCompleted` 等方法完成 查找 等操作。
- 与后端 `DocumentAction` 交互：在组件中调用 `DocumentAction.query_get` 等方法完成 查询 等操作。
- 与后端 `AttachmentAction` 交互：在组件中调用 `AttachmentAction.changeOrderNumber`、`delete`、`getOnlineInfo`、`getWithWorkOrWorkCompleted`、`uploadWorkInfo` 等方法完成 调用/删除/读取/读取/上传 等操作。

## Dependencies

**后端服务（o2server action / REST）：**
- `x_cms_assemble_control`
- `x_custom_smartbi_assemble_control`
- `x_general_assemble_control`
- `x_hotpic_assemble_control`
- `x_message_assemble_communicate`
- `x_officeonline_assemble_control`
- `x_onlyofficefile_assemble_control`
- `x_organization_assemble_express`
- `x_portal_assemble_surface`
- `x_processplatform_assemble_surface`
- `x_program_center`
- `x_query_assemble_surface`
- `x_wpsfile2_assemble_control`
- `x_wpsfile_assemble_control`
- `x_yozofile_assemble_control`
**依赖的其它 o2web 组件 / 应用：**
- `Selector.package`
- `Template.MTooltips`
- `Template.Selector.Custom`
- `process.Xform.$ElModule`
- `process.Xform.$Elinput`
- `process.Xform.$Input`
- `process.Xform.$Module`
- `process.Xform.Eldate`
- `process.Xform.widget.OOXML`
- `AI`（openApplication 打开的应用）
- `ForumDocument`（openApplication 打开的应用）
- `IMV2`（openApplication 打开的应用）
- `OfficeOnlineEditor`（openApplication 打开的应用）
- `OnlyOfficeEditor`（openApplication 打开的应用）
- `PdfViewer`（openApplication 打开的应用）
- `WpsOfficeEditor`（openApplication 打开的应用）
- `YozoOfficeEditor`（openApplication 打开的应用）
- `cms`（openApplication 打开的应用）
- `process`（openApplication 打开的应用）
**前端基础设施：**
- O2OA web 框架（MWF / o2.Actions / o2.xDesktop）、Vue 组件或 MooTools Class 组件模型
- 公共库：`o2_lib`（marked、PinYin 等）、`$OOUI` 组件库
