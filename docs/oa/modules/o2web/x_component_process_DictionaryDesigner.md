# x_component_process_DictionaryDesigner

## Responsibility

流程引擎前端组件，负责数据字典编辑的界面展示与交互操作。

## Entry Point

- `oa\o2web\source\x_component_process_DictionaryDesigner\Main.js`

## Build Pipeline

- No build scripts detected

## Key Configuration Files

- No JSON config files detected

- [List of key JSON config files with one-line explanations]

## Key Flows

- 跨组件跳转：通过 `openApplication` 打开其它应用（process）。
- 依赖其它组件能力：通过 `requireApp` 引入 Selector.package 等组件模块。
- 核心交互：流程引擎前端组件，负责数据字典编辑的界面展示与交互操作。

## Dependencies

**依赖的其它 o2web 组件 / 应用：**
- `Selector.package`
- `process`（openApplication 打开的应用）
**前端基础设施：**
- O2OA web 框架（MWF / o2.Actions / o2.xDesktop）、Vue 组件或 MooTools Class 组件模型
- 公共库：`o2_lib`（marked、PinYin 等）、`$OOUI` 组件库
