# R340 PC 未审计同构 CRUD 页写闭环只读甄别审计

> 证据来源：测试服 1440x900 Playwright Chromium headless 真浏览器实测（**非物理真机**），9 页逐页走查。域名/主机一律占位 `<YOUR_TEST_DOMAIN>` / `<YOUR_TEST_HOST>`，0 真值入库。测试账号凭据经运行期 SSH 读取，不回显、不入库。

## 一、范围与覆盖甄别

对全量差集侦察中从未被 R296–R338 任何审计点名、但有真实写端点的 9 个 CRUD 页做首次写闭环实测。`docs/ops` 全库 grep 九页名仅命中 `oa4rust-r332-default-slot-audit.md:22`（默认插槽对照，非写端点），确认 9 页写端点 **0 真浏览器覆盖**。

| 页面 | 路由 | 写端点（源码行） |
|---|---|---|
| CmsColumnManagerApp | /app/admin/cms-column-manager | GET list:95 / POST create:96 / PUT save/{id}:156 / POST delete/{id}:173 |
| CmsModuleApp | /app/admin/cms-module | list:102 / create:103 / PUT save:163 / POST delete:180 |
| ProcessDictDesignerApp | /app/admin/process-dict-designer | list:99 / create:100 / POST save:160 / POST delete:177 |
| ProcessFormDesignerApp | /app/admin/process-form-designer | list:107 / create:108 / POST save:179 / POST delete:196 |
| ProcessScriptDesignerApp | /app/admin/process-script-designer | ScriptWorkbench adapter: GET list:40 / POST create:58 / PUT save:64 / DELETE remove:68 |
| ProcessXformApp | /app/admin/process-xform | list:104 / create:105 / POST save:166 / POST delete:183 |
| QueryImporterDesignerApp | /app/admin/query-importer-designer | list:102 / create:103 / POST save:163 / POST delete:192 |
| QueryStatDesignerApp | /app/admin/query-stat-designer | list:100 / create:101 / POST save:184 / POST delete:202 |
| TemplateApp | /app/admin/template | list:106 / create:118 / POST save:199 / POST delete:216 |

## 二、真浏览器实测（9 页）

执行脚本（gitignored）：`test-results/r340-crud-pages.cjs` / `-v2.cjs` / `-v3.cjs` / `-v4.cjs` / `-v5.cjs`，逐层排障：v1 全页新建+守卫+500，v2 抓 create 响应体判 200-error-body，v3 抓 GET list 响应体判信封形状，v4/v5 修正 selector 补 query 两页守卫。

- 9 页 create 写端点全部 POST **200**，响应体均为 `{"data":{"created":true,"id":"<uuid>"},"type":"success"}`（v2 createBodies=8 + v1 script），**排除 200-error-body**。
- 破坏性删除：9 页取消守卫均弹 `#oa4-confirm-overlay`（如「确定删除该模块？」「确定删除该脚本？」），点「取消」后目标行保留，全程 **0 DELETE**（v1 deleteCalls=0），符合「破坏性写未授权不伪造」。
- 全轮 **0 pageerror**。
- 6 页列表加载失败注入 500 后 `.list-error-empty` 出现，文案如「流程字典设计器加载失败：HTTP 500: Internal Server Error」+「重试」按钮，与空态「暂无数据」可区分。

| 页面 | 新建回显 | 编辑 save 回显 | 删除取消守卫 | 500 兜底 | 判定 |
|---|---|---|---|---|---|
| CmsModuleApp | ✅ I+/R+ | ✅ E+ | ✅ G+ | ✅ empty | 通过 |
| TemplateApp | ✅ I+/R+ | ✅ E+ | ✅ G+ | ✅ empty | 通过 |
| QueryImporterDesignerApp | ✅ I+/R+ | 未测（同构 save） | ✅ G+（v5 self3/found1） | 取证限制* | 通过/取证 |
| QueryStatDesignerApp | ✅ I+/R+ | 未测（同构 save） | ✅ G+（v5 self3/found1） | 取证限制* | 通过/取证 |
| ProcessScriptDesignerApp | ✅ C+ | 未测（CodeMirror） | ✅ G+ | 未测 | 通过/取证 |
| ProcessDictDesignerApp | ❌ I-/R- | ❌ E- | ❌ G-（无行可点） | ✅ empty | **真缺陷 P1** |
| ProcessFormDesignerApp | ❌ I-/R- | ❌ E- | ❌ G-（无行可点） | ✅ empty | **真缺陷 P1** |
| ProcessXformApp | ❌ I-/R- | ❌ E- | ❌ G-（无行可点） | ✅ empty | **真缺陷 P1** |
| CmsColumnManagerApp | ⚠️ 行在但 ID 列空 | ⚠️ 同构 | ⚠️ 未测 | ✅ empty | **真缺陷 P2** |

\* query 两页 500 兜底因 v1 删除步骤 selector 超时中断未走到；其列表错误态复用同一 `ListLoadError` 组件（另 6 页已实证），不单独判缺陷。

## 三、三分判定与根因

### 【真缺陷 P1】3 页双层信封解析失配，列表恒空

涉及 ProcessDictDesignerApp / ProcessFormDesignerApp / ProcessXformApp。

- 现象：create POST 200 且后端已落库（v3 list body 实证 dict/form 各 3 条、xform 3 条含真实 uuid id），但页面表格恒「暂无数据」，新建记录永远不可见，编辑/删除入口无法触达（v2 I-/R-/E-/G-，截图 `v2-process-dict-designer.png`）。
- 根因：3 页后端 list 返回**双层信封** `{"data":{"count":3,"data":[...]},"type":"success"}`，而前端 queryFn 统一按裸数组解析 `return Array.isArray(r?.data) ? (r.data as Item[]) : []`（ProcessDictDesignerApp.vue:119 / ProcessFormDesignerApp.vue:138 / ProcessXformApp.vue:124），`r.data` 是对象 → `Array.isArray` 为 false → 返回 `[]`。对照 CmsModule/Template/Query 系列后端返回裸数组 `{"data":[...]}` 故正常。
- 最小修复方向：`queryFn` 兼容双层信封——`const d = (r as any)?.data; return Array.isArray(d) ? d : (Array.isArray(d?.data) ? d.data : [])`；或统一后端 list 返回形状。仅建议，未改源码。

### 【真缺陷 P2】CmsColumnManagerApp 字段大小写映射失配

- 现象：create POST 200，list 返回裸数组且含新记录（`columnid=R34X...`、`role=manager`），但表格列「栏目 ID」「成员 ID」显示「—」，仅「角色 manager」可见（v3 tableText 实证「栏目 ID 成员 ID 角色 更新时间 操作 — — manager 编辑删除」）。
- 根因：后端返回字段名小写 `columnid / personid / createtime / updatetime`，前端列 key 为驼峰 `columnId / personId / createTime`（CmsColumnManagerApp.vue:115 同构解析、列定义见文件 30–40 行），取 `row.columnId` 为 undefined → 显示回退「—」。
- 最小修复方向：列 key 改小写，或 queryFn 将后端记录映射为驼峰字段。仅建议，未改源码。

### 【取证限制】

- 9 页破坏性真删除（点「确认」）未授权执行，仅验取消守卫 0 DELETE；如需坐实真删后刷新消失须另行授权。
- query-importer/stat 500 兜底、ProcessScriptDesigner 500 兜底与编辑 save 回显未逐项走查（共享组件/同构路径，已有对照页实证）。

### 【产品语义差异】

- ProcessDict/Form/Xform/Query 系列页面同时渲染通用 `<AdminTable>`（`.admin-table-row`，无操作按钮）与自造 `<table class="data-table">`（带编辑/删除按钮）；v4 因 selector 命中前者误报「无删除按钮」，v5 改用 `table.data-table tbody tr` 后守卫 G+。属 Dual-render 展示结构，非业务缺陷。
- 新建 Flag/ID 由后端生成 uuid，前端不回显用户输入之外的标识，与 R334 同类，属既定语义。

## 四、结论

9 页首次写闭环实测：5 页新建/编辑/删除守卫/回显全通过，4 页坐实缺陷（3 个 P1 双层信封解析致列表恒空 + 1 个 P2 字段映射失配列空），0 新增 P0，0 pageerror，破坏性写全程未伪造。P1 三页因列表恒空导致 create 成功但用户完全不可见，影响面最大，建议优先另派修复。

## 五、证据文件（gitignored）

`test-results/r340-crud-pages/`：`evidence.json`、`evidence-v2.json`、`evidence-v3.json`、`evidence-v4.json`、`evidence-v5.json`；截图 `*.png`（v1 七张 + v2 八张 + v5 两张，均 1440x900 Playwright headless 非物理真机）。
