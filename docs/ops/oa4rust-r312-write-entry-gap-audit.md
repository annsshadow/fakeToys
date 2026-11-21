# R312 候选域写端点可达性静态审计（A/B/C 分类）

## 来源与取证边界
- 执行者：page-work（只读静态审计，不改源码、不碰测试服）。
- 范围：候选域 `jpush` / `categoryinfo` / `document` / `hotpic` / `appinfo` / `form` / `program` / `cms` 配置写端点；交叉验证后端路由注册与桌面前端 UI 真实调用点。
- 分类口径：`A`＝后端注册 + 前端有明确用户触发入口；`B`＝后端注册 + SDK/apis 封装存在但桌面无 UI 调用点；`C`＝后端注册 + 前端无封装也无 UI 调用点。
- 取证边界：**纯静态源代码审查，未做真浏览器验证；本文任何“缺口”均为静态候选，不构成已确认缺陷。** 后续若进入实现轮，仍须按铁律走测试服真浏览器实测与 RED→GREEN。
- 敏感信息：本文所有域名用 `<YOUR_TEST_DOMAIN>` 占位，主机用 `<TEST_HOST>` 占位；0 真值（无口令、私钥、会话、真人名/手机/邮箱、真实服务器地址）。

## 一、逐域分类总表

| 域 | 代表写端点（后端注册） | 分类 | 前端 UI 证据（desktop/src） | 二次确认 / 结论 |
|---|---|---|---|---|
| jpush | `POST /api/jpush/create`（jpush_assemble_control/src/routes.rs:57）；另有 GET 孪生写变体 `:23 GET /api/jpush_assemble_control/create/jpush` | A（主入口）；GET 变体归 C/可疑 | `JPushApp.vue:177 POST /api/jpush/create`；`ServerApp.vue:314` 仅“孪生端点”诊断批量探测 GET 变体；`JPushApp.test.ts:85/123` 断言业务页不含该 GET | 主入口明确存在，非缺口；GET 写变体无业务 UI 入口，且属诊断探测，不应作为“新建推送”产品入口（C/可疑，低价值） |
| hotpic | `POST /api/hotpic/create/hotpic`（hotpic_assemble_control/src/routes.rs:70）、`POST /api/hotpic/assemble/control/user/hotpic`（:138）；另有 GET `:29` | A（两写入口）；GET 变体归 C/可疑 | `HotpicApp.vue:153` 按钮“建热图”→ POST `/api/hotpic/create/hotpic`；`Z(userCreate)` → POST `/api/hotpic/assemble/control/user/hotpic`；`HotpicApp.vue:305` 仅“热图孪生端点”诊断批量探测 GET 变体 | 主入口均存在，非缺口；GET 变体仅诊断探测，无业务新建 UI（C/可疑，低价值） |
| appinfo | `POST /api/appinfo`（cms_assemble_control/src/routes.rs:572） | A | `AppInfoApp.vue:197 api.post("/api/appinfo", { alias, appType })` | 有真实新建 UI；`packages/apis` `appinfoApi` 有 `createRequest("/api/appinfo")` 封装（但桌面直调字面路径，封装漂移非缺口） |
| categoryinfo | `POST /api/categoryinfo`（cms_assemble_control/src/routes.rs:574）；另 `POST /api/cms/category/create`（cms_core_entity/src/lib.rs:855） | A（两条资源均有 UI） | `CategoryApp.vue:101 POST /api/categoryinfo`；`CmsIndexApp.vue:60-61、391` 新建分类/文章 → `POST /api/cms/category/create`、`POST /api/cms/article/create` | 两条均可见入口，非缺口 |
| document | `POST /api/document`（cms_assemble_control/src/routes.rs:526） | A | `DocumentApp.vue:15`「+ 新建文档」→ `:66` 弹窗 → `:250 POST /api/document` | 真实用户入口存在；`packages/apis` `documentApi.create` 写 `/api/document/document` 与实际 `/api/document` 不一致，属**封装漂移**（须后续单独治理，不是“无 UI 入口”） |
| form | `POST /api/form`（cms_assemble_control/src/routes.rs:554）；另 `POST /api/cms/assemble/control/form/create`（:264） | A（两条均有 UI） | `FormApp.vue:100` 新建弹窗、`:112` 创建按钮 → `:326 POST /api/form`；`FormDesigner.vue:1113 POST /api/form`；`CmsFormDesignerApp.vue:95 createEp=/api/cms/assemble/control/form/create` | 真实入口存在，非缺口 |
| program | `POST /api/program_center/{application,agent,collect,input,invoke}/create`、`POST /api/program_center/invoke`、`POST /api/program_center/script`、`POST /api/program_center/dict`（program_center/src/routes.rs:323/330/334/452/466/471/570/566 等） | A（主体）；C/可疑（重复注册 `invoke/create`） | `ProgramCenterApp.vue:931 agent/create`、`:999 dict`、`:1296 POST /api/program_center/invoke`、`:1602-1605 scriptCreate`；`CollectApp.vue:101 collect/create`；`ProcessApplicationApp.vue:126 application/create`；`CmsScriptDesignerApp.vue:59 POST /api/script`（CMS 脚本，不同资源） | 主入口齐备；`invoke/create`（:471）与 `POST /api/program_center/invoke`（:466）语义重叠，桌面只用后者，前者无 UI 调用——静态候选 C，低价值（需先确认是否兼容别名） |
| cms 配置写 | `POST /api/{view,templateform,viewcategory,viewfieldconfig,comment,design/appdict}`、`POST /api/cms/assemble/control/{dict,form,view,xform}/create`（cms_assemble_control/src/routes.rs:248/256/264/272、:564/562/567/569/496/617 等） | A（全部） | `CmsIndexApp.vue:72/77/79/81/84/86` 高级操作按钮 → `:474/483/487/491/496/500` 对应 POST；`CmsDictDesignerApp.vue:90`、`CmsFormDesignerApp.vue:95`、`CmsViewDesignerApp.vue:95`、`CmsXformApp.vue:95` 四设计器 `createEp` 对应 control create | 二次确认按钮模板与 handler 均存在，真实可点击入口齐备；`packages/apis` 无这些封装但桌面直调，属架构风格非缺口 |

## 二、B 项二次确认（封装存在但 UI 调用点）
- `appinfoApi` / `categoryinfoApi`（packages/apis/src/index.ts:327-336）：均有 `createRequest` 封装；桌面 `AppInfoApp.vue:197`、`CategoryApp.vue:101` 另有直调，故不属 B。
- `programCenterApi.dictCreate`（:529）指向 `POST /api/program_center/dict/create`，桌面 `ProgramCenterApp.vue:999` 实际调 `/api/program_center/dict`；`documentApi.create`（:600-619）指向 `/api/document/document`，桌面实际 `/api/document`。这两处是**封装路径漂移**：封装声称的端点与 UI 实际端点不同，但 UI 入口真实存在。治理方向应是校正封装或统一端点，而非补 UI 入口；本轮不改源码。
- `formApi.request=createRequest("/api/form")`（:769-774）有通用封装；`viewcategoryApi`（:877-879）、`cmsCategoryApi`（:893-901）无显式 create 具名。桌面均有直调入口，不构成 B 类“有封装无 UI”。
- `cms/assemble/control/{dict,form,view,xform}/create` 在 packages/apis 无封装，但四设计器页面直调且按钮可点击；按分类口径为 A（有 UI），不是 B。

## 三、价值排序候选（供后续轮次决策，本轮不实施）

| 优先级 | 候选 | 静态判断 | 后续动作建议 |
|---|---|---|---|
| 中 | `programCenterApi.dictCreate` / `documentApi.create` 封装路径漂移 | 封装端点与桌面实际端点不一致，可能导致未来新调用方误用封装而 404 | 单独轮次核对后端注册，统一封装与直调路径，并补契约锁定 |
| 低 | `GET /api/jpush_assemble_control/create/jpush`、`GET /api/hotpic_assemble_control/create/hotpic` 写变体 | 后端用 GET 暴露写语义，仅在诊断“孪生端点”批量探测中出现，无业务 UI 入口 | 后端评估是否废弃或改 POST；前端诊断探测可保留，但不作为产品新建入口 |
| 低 | `POST /api/program_center/invoke/create`（routes.rs:471） | 与 `POST /api/program_center/invoke`（:466）语义重叠，桌面仅用后者 | 后端确认是否兼容别名；若否，评估删除或并入文档 |
| 低 | `appinfoApi` / `categoryinfoApi` 等封装与桌面直调并存 | 架构风格不统一（apis 包注释已声明桌面直调），非功能缺陷 | 长期统一调用层时一并治理 |

## 四、结论
- 在本次候选域范围内，**未发现“后端有业务写接口、前端有封装、但完全没有 UI 入口”的 B 类真实缺口**；主业务写端点均为 A 类（有真实桌面 UI 触发）。
- 存在的静态候选集中在：封装路径漂移（中价值）、GET 写变体与重复注册（低价值、需后端语义确认）。这些均**不是已确认缺陷**，需后续真浏览器与接口实测验证后再决策。
- 项目处于 ≥1000 轮持续迭代中，本文仅为 R312 静态审计快照，不代表项目完成。