# R342 PC 查询设计器 QueryManagerDeep 写端点群只读深度审计

> 证据来源：测试服 1440x900 Playwright Chromium headless 真浏览器实测（**非物理真机**）。域名/主机一律占位 `<YOUR_TEST_DOMAIN>` / `<YOUR_TEST_HOST>`，0 真值入库；测试账号凭据经运行期 SSH 读取，不回显、不入库。

## 一、覆盖甄别

R296/R302/R304/R306 等既有文档对 `QueryManagerDeep.vue` 均为静态枚举/基线冒烟/视觉改造层面点名（如 `docs/plans/optimization-1000-oa4rust-track-b.md:334` 记其 500 仅本地 slice；`docs/ops/oa4rust-r306-admin-a-visual.md:4` 列 QueryManager 为视觉改造页），`docs/ops` 全库 grep **未见任何对其写路径的真浏览器深审**。确认 63 写端点 0 真浏览器写闭环覆盖，本轮首次实测。

## 二、63 写端点静态分组（`QueryManagerDeep.vue`，路由 `/app/admin/query-manager-deep`，main.ts:236-238）

| 组 | 按钮/函数 | 代表端点（源码行） |
|---|---|---|
| 主新建 | createQuery(:875) | POST designer/create :865/867 |
| 五类建 | qdCreate(:580) | POST designer/query \| statement \| stat \| importmodel \| neural/model :580-584 |
| 八类改 | qdEdit(:595) | PUT designer/query/{id} \| stat/{id} \| view/edit/{id} \| importmodel/{id} \| neural/model/{id} \| importer/save/{id} :595-602 |
| 八类删 | qdDelete(:616) | DELETE designer/query \| statement \| stat \| view \| table \| importmodel \| neural/model \| importer/delete :616-623（均 confirmMsg 守卫 :613） |
| 六类权限 | qdPerm(:634) | POST designer/{query,statement,stat,view,table,importmodel}/{id}/permission :634-639 |
| 检索/杂项 | qdMisc(:648) | POST designer/search \| designer/search(V2) \| table/execute/{flag} \| statement/list/query/{qf}; PUT stat/{id}/simulate \| query/{flag}/icon \| icon/set/{flag} \| output/{flag}/select :648-667 |
| 二组写 | qdWrite2(:679) | PUT stat/edit/{id} :679; DELETE stat/delete/{id} :683; POST importmodel/{id} :686; PUT importmodel/edit/{id} :689; DELETE importmodel/delete/{id} :693; DELETE view/delete/{id} :697; DELETE table/delete/{flag} :701; POST statement/permission/{id} :704; POST statement/execute/{flag}/page/1/size/20 :707; PUT neural/update/model/{f} :710; DELETE neural/delete/model/{f} :714; POST neural/model/reset/{f}/init :717 |
| 设计器删/输入/表行 | qdDeleteDesigner(:771) qdInput(:779) qdTableRow(:794) | POST designer/delete/{id} :771; PUT input/compare \| cover \| create \| prepare/cover \| prepare/create :779-783; POST table/{flag}/row \| row/save :794-795; DELETE table/{flag}/row/delete/all :796; PUT row/{rid} :799; DELETE row/{rid} :800 |
| 执行/批处理/建 | runBatch(:1076) executeSql executeQuery(:1196) | POST designer/execute :812/:1076; POST designer/stat/do :832; POST designer/execute(sql) :1076; POST designer/create(q) :1196 |
| 查询删除 | deleteQuery(:856) | DELETE designer/delete/{id} :849（confirmMsg :856） |

## 三、真浏览器实测（脚本 `test-results/r342-query-deep.cjs` + `-v2.cjs`，gitignored）

- S0 列表：GET `/api/query/assemble/designer/list/all` **200**，响应体 `data` 为裸数组 9 条（非双层信封），9 条 `.qs-item` 正常渲染，0 pageerror。
- S1 主新建：`.modal-overlay` 填名称/SQL/分类→POST designer/create **200**；即时 9→**9** 条且新名不可见；reload 后才 9→10 条且新名可见 → **P1 真缺陷（见四）**。
- S2 五类建：建查询/建语句/建统计/建导入模型/建神经模型 5 个按钮点击（prompt 填名）→对应 5 个 POST 全 **200**。
- S3 三类改（假 ID）：改查询/改统计/改视图 → 3 个 PUT 全 **200**（后端对假 ID 容忍，未污染真数据）。
- S4 杂项/输入/权限：POST designer/search、designer/search(V2)、PUT input/compare、input/cover、input/create、POST query/{假ID}/permission 共 6 个写全 **200**。
- S5 删除守卫：点「删查询」→prompt 填假 ID→`#oa4-confirm-overlay` 文案「确定删除该query？」→点「取消」→**0 DELETE**，破坏性写未伪造。
- S6 500 兜底：route fulfill 500 → `.list-error-empty` 出现「查询定义加载失败：HTTP 500: Internal Server Error」+「重试」按钮，与空态「暂无查询定义」可区分。
- v2 补测：新建后点页面自带「🔄 刷新」→仍 11→**11** 条且新名不可见；后端 fetch 同端点已 **12** 条；整页 reload 才 12 条且新名可见；0 pageerror。截图 `Q7-refresh-btn.png`。

## 四、三分判定

### 【真缺陷 P1】queryKey 不匹配，新建/删除/刷新按钮均无法更新列表

- 现象：POST designer/create **200** 且后端已落库（v2 backend n=12），但新建后列表不回显，点页面「刷新」按钮仍不更新；只有整页 reload 才显示。删除同理（`deleteQuery` 与 `refresh` 同样失效）。用户视角=写入成功却看不到，且刷新按钮成为死按钮。
- 根因：列表 `useQuery` 订阅 `queryKey: ['query', 'defs']`（:495-496），而新建 `cm.onSuccess`（:872）、删除 `delM.onSuccess`（:851）、刷新 `refresh()`（:883）全部 `qc.invalidateQueries({ queryKey: ['qm', 'list'] })`——两个 queryKey 完全不匹配，失效打在无人订阅的键上。
- 最小修复方向：统一三处 invalidateQueries 的 queryKey 为 `['query', 'defs']`（或反转 useQuery 键），使写后失效命中真实订阅。仅建议，未改源码。

### 【取证限制】

- 八类删及二组写中 delete 类（stat/importmodel/view/table/neural 模型/设计器、表行删除/清空）破坏性写未授权执行，仅验「删查询」确认守卫 0 DELETE；如需坐实真删后刷新消失须另行授权。
- qdEdit/qdPerm/qdMisc/qdWrite2 使用假 ID，仅坐实端点可达 HTTP 200，**未验证真 ID 上的状态回显持久**（避免污染真数据）；63 端点未逐条走查，按功能组各取代表性验证。

### 【产品语义差异】

- 后端对假 ID 的 PUT/POST 权限/模拟等端点返回 200（幂等/宽松校验），属后端既定口径，非前端缺陷。
- 深度读 `loadDesignerDeepReads` 15 条 GET 用 `s()=p.catch(()=>null)` 吞错（:554-569），属可用性探测，与 R329/R338 同类语义，非缺陷。
- 与 R340 双层信封 `data.data` / 字段大小写失配不同：本页 list 为裸数组 `data[]` 且字段正常，故无该两类缺陷。

## 五、结论

63 写端点按 10 功能组梳理、各组代表性只读安全写实测：新建/五类建/三类改/六杂项共 15 个安全写端点 POST/PUT 全 200，删除守卫有效 0 DELETE，500 兜底与空态可区分，0 pageerror。**1 个 P1 真缺陷**：queryKey 不匹配致新建/删除/刷新按钮均不更新列表（POST 200 但用户不可见），影响面覆盖全页 CRUD 写闭环，建议优先另派修复。

## 六、证据文件（gitignored）

`test-results/r342-query-deep/`：`evidence.json`、`evidence-v2.json`；截图 `Q0-list.png`、`Q1-created.png`、`Q5-delete-guard.png`、`Q6-fail500.png`、`Q7-refresh-btn.png`（均 1440x900 Playwright headless 非物理真机）。
