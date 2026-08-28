# x_component_HotArticle

## Responsibility

热点信息组件，负责热门文章和推荐内容的展示。

## Entry Point

- `oa\o2web\source\x_component_HotArticle\Main.js`

## Build Pipeline

- No build scripts detected

## Key Configuration Files

- No JSON config files detected

- [List of key JSON config files with one-line explanations]

## Key Flows

- 后端请求：向 `x_file_assemble_control/jaxrs/file/{{$.picId}}/download/stream` 发起 POST 流式请求，处理返回数据并渲染。
- 与后端 `HotPictureInfoAction` 交互：在组件中调用 `HotPictureInfoAction.changeTitle`、`delete`、`listForPage` 等方法完成 调用/删除/列出 等操作。
- 跨组件跳转：通过 `openApplication` 打开其它应用（ForumDocument、cms）。

## Dependencies

**后端服务（o2server action / REST）：**
- `x_file_assemble_control`
- `x_hotpic_assemble_control`
**依赖的其它 o2web 组件 / 应用：**
- `ForumDocument`（openApplication 打开的应用）
- `cms`（openApplication 打开的应用）
**前端基础设施：**
- O2OA web 框架（MWF / o2.Actions / o2.xDesktop）、Vue 组件或 MooTools Class 组件模型
- 公共库：`o2_lib`（marked、PinYin 等）、`$OOUI` 组件库
