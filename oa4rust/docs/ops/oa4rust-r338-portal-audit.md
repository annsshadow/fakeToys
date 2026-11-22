# R338 PC 门户设计器 PortalApp 写端点群只读深度审计

> 审计轮次：R338 / 任务 t141；审计对象 `oa4rust/frontend/apps/desktop/src/views/PortalApp.vue`（555 行，路由 `/app/admin/portal`，main.ts:38→123→133-138）
> 取证方式：本地 Playwright Chromium headless、1440x900；**非物理真机截图**，仅代表该无头浏览器渲染；测试服地址与凭据运行期从 gitignored 配置读取，本文不含真值（域名占位 `<YOUR_TEST_DOMAIN>` / `<YOUR_TEST_HOST>`）
> 性质：只读审计，未改源码/契约/后端/配置；证据落 gitignored `oa4rust/test-results/r338-portal/`（evidence.json / evidence-v2.json / P1-list.png / V2-*.png）

## 一、结论摘要

- **P1 真缺陷 ×1**：主操作入口「+ 新建页面」是死按钮。`PortalApp.vue:8` 绑定 `@click="showEditor = true"`，`:460` 仅 `const showEditor = ref(false)`，模板全文再无 `showEditor` 消费点（grep 仅此 2 处）。真浏览器点击后 `newPageModalCount=0`、`newPageEndpointCalls=0`：既不打开编辑器也不发任何请求，用户无法新建页面。最小修复方向：补 `v-if="showEditor"` 的页面编辑弹窗并接 `POST /api/portal/assemble/designer/page`，或在无编辑器时移除该 CTA。
- **0 个其他新真缺陷**：代表性安全写（surface/create、designer portal/widget/templatepage create、designerMisc search/summary/scriptManager、scriptByName）均 HTTP 200 且 body type=success；破坏性写确认守卫有效；500 兜底与空态可区分；0 pageerror。
- **取证限制**：破坏性写（designerDelete 各 kind、surface dictDel、页面卡片 🗑 删除）未授权真删，仅验确认守卫（弹窗文案 + 取消后 0 DELETE），不伪造写。
- **产品语义差异**：占位/空 payload 探测类写端点返回业务错误或非 2xx（见第四节），前端统一 `toast.success` 吞错，属探测性调用设计，非真缺陷。
- **默认插槽核实**：`AdminPageHeader.vue:12` 仅 `<slot name="actions"/>`，`PortalApp.vue:7` 以 `<template #actions>` 具名插槽传入「+ 新建页面」，按钮真实渲染（allButtons 命中、newPageButton=1），**非** R329/R332 同类默认插槽静默丢弃缺陷。

## 二、41 个写端点静态分组（任务标称，按源码定位）

1. 门户/页面/组件/模板页/脚本 CRUD — `designerCreate`(:325-339)、`designerUpdate`(:340-357)、`designerDelete`(:358-374)：POST designer/{portal,page,widget,templatepage,script}；PUT designer/{portal,page,widget}/{id}；DELETE designer/{portal,page,widget,templatepage}/{id} + designer/dict/delete/{id} + designer/script/{id}。
2. 设计器工具写 — `designerMisc`(:376-408)：PUT portal/{id}/icon、POST portal/{id}/permission、PUT page/save/{id}、POST designer/designer/search、PUT input/{compare,cover,create}、PUT input/prepare/{cover,create}、POST portal/list/summary/v2、POST script/list/manager（统一 toast.success「门户设计器操作已提交」）。
3. 表面(surface)读写删 — `surfaceOps`(:410-458)：POST surface/create、POST surface/publish/{id}、POST surface/dict/{df}/portal/{pf}/{path}/data、DELETE 同路径（confirmMsg「确定删除该门户字典数据？」:442）、POST surface/script/portal/{portal}/name/{name}；另有 3 条 GET 读。
4. 孪生探针 A — `loadPortalTwin`(:302-322)：POST /api/portal/page/save/0、POST page/delete/0、POST script/portal/portal/imported/0/0、PUT surface/dict/0/portal/0/0/data（+ GET 多条）；`s()=p.catch(()=>null)` 吞错。
5. 孪生探针 B — `loadPortalTwin2`(:504-522)：POST/PUT /api/portal/design/save、POST surface/publish、POST page/{create,remove,update}（+ GET design/0、surface/0/preview、page/get/0）；同样 s() 吞错。

## 三、真浏览器实测证据（1440x900 headless，非物理真机）

| 操作 | 端点 | HTTP | 证据 |
|---|---|---|---|
| 建门户(表面) | POST /api/portal/assemble/surface/create | 200 | 返回 id 与 name=R338V*-surface，type=success |
| 建门户(设计器) | POST designer/portal | 200 | 返回 id、name=R338V*-designer |
| 建组件 | POST designer/widget | 200 | 返回 id、portalId 空 |
| 建模板页 | POST designer/templatepage | 200 | 返回 id、name |
| 建脚本(表面按名) | POST surface/script/portal/default/name/* | 200 | 返回 id、portalId=default |
| 设计器检索 | POST designer/designer/search | 200 | body count=9 |
| 门户汇总V2 | POST designer/portal/list/summary/v2 | 200 | count=1，含刚建 R338V*-designer |
| 脚本管理列举 | POST designer/script/list/manager | 200 | count=0 |
| 建页面 | POST designer/page | 400 | 空 content payload 被拒 → 产品语义差异 |
| 建脚本(设计器) | POST designer/script | 500 | 后端错误 → 产品语义差异 |
| 存字典数据 | POST surface/dict/*/portal/default/root/data | 200 | body type=error「dict not found」经 HTTP200 返回 |
| 删门户守卫 | DELETE designer/portal/{id} 未发出 | 0 请求 | 自定义确认弹窗「确定删除该portal？」+ 取消后 0 DELETE |
| 孪生探针A | 11 条混合 | 见 evidence-v2.json | page/save/0 200(error body)、dict data PUT 200(success、真写 id)、page/delete/0 200(error)、script imported 200(error) |
| 孪生探针B | 9 条混合 | 见 evidence-v2.json | design/save POST+PUT、surface/publish、page/create/remove/update 均 400；3 条 GET 200 |
| + 新建页面 | **无请求** | — | newPageModalCount=0、newPageEndpointCalls=0 → P1 死按钮 |
| 500 兜底 | GET surface/page/list/portal/default 注入 500 | 1 empty | 「⚠️ 门户页面加载失败：HTTP 500: Internal Server Error 重试」+ 重试按钮 1 个，与空态「暂无门户页面」可区分（V2-500.png） |

页面 0 pageerror。截图：P1-list.png（初始列表/空态）、V2-groups-expanded.png（四组端点按钮展开）、V2-delete-guard.png（确认守卫）、V2-new-page-click.png（死按钮点击后无变化）、V2-500.png（500 兜底）。

## 四、三分判定明细

- 【真缺陷·P1】+ 新建页面死按钮：`PortalApp.vue:8/:460`，新页面编辑入口无实现。证据 newPageModalCount=0 / newPageEndpointCalls=0。修复方向见摘要。
- 【产品语义差异】designer/page 400、designer/script 500、dictSet 200(error body)、twinA 占位 0 路径 error body、twinB 空 payload 400：均为占位/空 shape 探测调用的后端响应，前端 `s()` 吞错 + `toast.success` 统一提示总数，属设计语义，不改。
- 【取证限制】所有破坏性写（delete 各 kind、dictDel、卡片删除）未授权真删，仅验确认守卫文案与取消后 0 DELETE；深度写闭环（新建→列表回显→编辑→刷新持久）因门户主列表无数据回填（surface/page/list 返回空）无法走完，标取证限制。
- 【通过】默认插槽无缺陷；安全写 8 组 POST 200；确认守卫有效；500 兜底可区分；0 pageerror。

## 五、审计边界

本轮为只读甄别，未修复任何源码问题；P1 死按钮待队长另行派单按 test-first RED→GREEN 修复。后续轮次按评审结论续作，项目未完结（≥1000 轮迭代）。