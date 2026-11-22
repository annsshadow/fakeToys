# R344 PC 查询视图 QueryViewApp 写端点群与搜索渲染深度审计

## 来源与范围
- 任务：t150 [R344] 只读深度审计，PC 查询视图写路径。
- 对象：frontend/apps/desktop/src/views/QueryViewApp.vue（989 行），路由 /app/admin/queryview（main.ts:176-178，另 :529 redirect）。
- inScope：本页 76 个 api 写端点按功能分组代表性实测、搜索结果渲染、删除守卫、加载失败兜底、孪生探针语义甄别。
- 硬约束：只读审计，未改源码/契约/后端/配置；唯一入库物为本文档；证据与脚本落 gitignored test-results/r344-queryview-audit/。
- 凭据仅运行期由 ssh oa4rust-test 读取、不回显；测试域名用 <YOUR_TEST_DOMAIN> / <YOUR_TEST_HOST> 占位，仓库无真值。

## 覆盖甄别（本页写路径此前为 0 真浏览器覆盖）
- R296：docs/ops 下 0 命中，无独立文档。
- R315：oa4rust-r315-search-envelope-fix.md 修的是 SearchApp 全局搜索的分组信封解析，其后端正是 /api/queryview/search；本次发现 QueryViewApp 未同步该修复（见 P1）。
- R306：oa4rust-r306-admin-a-visual.md 仅视觉改造基线。
- R314：契约漂移 + guard-blindspot 静态枚举，未做本页真浏览器写闭环。
- R332：oa4rust-r332-default-slot-audit.md:41-44 判本页默认插槽非缺陷（自造 table，allButtons 含 40+ 业务按钮），不在本轮重做。
- R337/R340/R342：文档仅点名本页，无写路径覆盖。
- 结论：76 写端点此前 0 真浏览器写闭环覆盖，本轮为首次。

## 静态梳理（76 写端点按功能分组）
- 查询/视图执行族：qvWrite viewExecute/viewBundle/viewExcel(:560-568)、qvMore viewAppExec/viewExec/viewExecV2/viewExecV2Id/bundlePost(:593-615)，端点 PUT/POST /api/queryview/view/{id}/execute|bundle|excel 等。
- 统计/导入模型族：qvWrite statExecute/importExecute(:569-574)、qvMore 各 statExec/importRun/importExecRecPost(:612-621)，端点 PUT /api/queryview/stat/{id}/execute、POST /api/queryview/importmodel/{id}/execute。
- 表格行 CRUD 族：qvWrite rowInsert/rowOneInsert/rowDelete/rowDeleteAll/rowPartUpdate(:539-559)、qvRows 六类(:687-717)、qvDesignerRows 五类 insert/update/delete/save/build/deleteAll(:719-781)，端点 POST/DELETE/PUT /api/queryview/table/... 与 /api/query/assemble/designer/table/row/...。
- 相似检索/杂项写：qvWrite morelikethis(:580-581) POST /api/queryview/morelikethis。
- 破坏性写守卫：rowDelete(:548)、rowDeleteAll(:552)、importRecordDelete(:577)、qvRows rowDeleteAll(:698)/rowDelete(:703)、qvDesignerRows deleteAll(:768)/delete(:774) 均先 confirmMsg 再发写。
- 孪生探针族：loadQueryTwin 13 条(:387-409)、loadQueryTwin2 15 条(:884-908)、loadQueryTwin3 24 条(:911-944)，均 s()=p.catch(()=>null) 吞错。
- 列表读：loadViews(:782-795) GET /api/queryview/view/list/query/all → views.value=r.data?.list??r.data??[]；doSearch(:169-189) POST /api/queryview/search。
- 本页用普通 ref 直接赋值，非 TanStack useQuery，R342 的 queryKey 不匹配缺陷不适用。

## 真浏览器实测（Playwright Chromium headless 1440x900，非物理真机）
### S0 列表加载与形状
- GET /api/queryview/view/list/query/all = 200，响应体 data 为裸数组（测试时 0 条），rowsBefore=0，pageerror=[]。
- 判据：列表空态「暂无查询视图 / 可通过上方「查询列表」等入口加载数据」真实渲染（截图 V0-list.png）。

### S1 搜索结果渲染 —— 判【真缺陷 P1】
- 后端 GET/POST /api/queryview/search=200，fetch 探针拿原始响应体：data 为分组信封 { designList:[...], key, viewList:[...] }，非数组。关键词 a → designList 3 条（S3 查询…）、query → designList 10 条、view → viewList 1 条（Test View）、R344 → 两组均 0。
- 前端 doSearch(:177) views.value = r.data ?? [] 把整个信封对象塞进声明为数组的 views；Vue v-for 遍历对象三个字段值，UI 恒定渲染 3 行「未命名 / 执行Excel」（flag 空），关键词 a/query/view/R344 四态 rows 恒 3、panel 文案不变（截图 V3-search-probe.png）。
- 后果：真实搜索结果（a=3、query=10、view=1）全部不渲染；无结果关键词（R344）仍显 3 行垃圾，无法进入空态；用户实际无法检索视图。
- 同根因：R315 已在 SearchApp 修复 /api/queryview/search 分组信封解析（解包 designList/viewList 归一化为数组后合并），本页漏同步。
- 定位：QueryViewApp.vue:177（doSearch 内 views.value = r.data ?? []）。
- 最小修复方向：对齐 R315 doSearch 解包——const envelope=r?.data??{}；designList/viewList 各按 Array.isArray 归一化，映射为视图项后合并展开赋 views；保留失败兜底与重试。
- 三分判定：真缺陷 P1（功能性搜索失效，数据存在但渲染为垃圾行），非取证限制。证据 0 pageerror 属静默失败（对象无长度不报错），与 R315 D1 表现一致。

### S2 代表性只读安全写（假 ID，不破坏真数据）
- 执行视图 PUT /api/queryview/view/R344-NOSUCH/execute = 200。
- 视图打包 PUT /api/queryview/view/R344-NOSUCH/bundle = 200。
- 视图导Excel PUT /api/queryview/view/R344-NOSUCH/excel = 200。
- 执行统计 PUT /api/queryview/stat/R344-NOSUCH/execute = 404（假 ID 路径该变体未命中注册/无数据，端点可达但非 2xx，如实标注）。
- 执行导入模型 POST /api/queryview/importmodel/R344-NOSUCH/execute = 200。
- 相似检索 POST /api/queryview/morelikethis = 200。
- 判据：查询执行/视图 bundle/excel/统计/导入模型五组写端点经真实页面按钮（DOM click 绕过 40+ 按钮横向溢出视口）驱动发出并到达后端，假 ID 不写真数据；0 pageerror（截图 V2-S2-writes.png）。

### S3 删除守卫（破坏性写未授权，仅验弹窗 + 取消）
- qvWrite 删行 → confirmMsg 弹窗 #oa4-confirm-overlay（role=dialog）文案「确定删除该数据行？」含取消/确认，点取消后 DELETE 调用增量 0。
- qvRows 清空表行 → confirmMsg 弹窗文案「确定清空该表所有行？」含取消/确认，点取消后 DELETE 调用增量 0。
- 判据：两类破坏性写均强制二次确认，取消后零 DELETE；真实删除/清空因无授权改库未执行，判【取证限制】（守卫本身 PASS，截图 V2-S3-guard.png）。

### S4 加载失败兜底与空态可区分
- page.route 将 GET view/list/query/all fulfill 500 后整页 reload：.list-error-empty 显示「查询视图加载失败：HTTP 500: Internal Server Error」+ 重试按钮。
- 与真的空态「暂无查询视图」文案与组件（ListLoadError mode=empty vs 空态 div.ai-empty）明显区分，PASS（截图 V2-S4-fail500.png）。

### S5 孪生探针语义甄别
- 孪生端点 13 请求（写 11）、孪生端点B 15（写 8）、服务处理读C 24（写 5），s()=p.catch(()=>null) 吞错。
- 判【产品语义差异】：与 R329/R342 一致，属端点可用性探测，错误被有意吞掉、不污染业务视图，非缺陷（截图 V2-S5-twins.png）。

## 三分判定汇总
- 【真缺陷 P1】×1：S1 doSearch 未解析 queryview/search 分组信封致搜索结果渲染为 3 行垃圾、真实结果不显示、无结果无法空态（R315 同型，QueryViewApp 漏同步）。
- 【取证限制】×2：S3 破坏性删/清空仅验守卫未真删；76 写端点按功能组取代表未逐条走查（假 ID stat/execute 返 404 亦如实记录）。
- 【产品语义差异】×3：孪生探针 s() 吞错；本页列表裸数组（无 R340 双层信封/字段大小写失配）；普通 ref 无 R342 queryKey 失配。
- 0 新增 P0，0 pageerror；加载失败兜底与空态可区分 PASS；删除守卫 PASS。

## 证据索引（真值不入仓）
- 截图与 evidence：test-results/r344-queryview-audit/（gitignored），含 V0-list.png / V1-search.png / V2-S1-search.png / V2-S2-writes.png / V2-S3-guard.png / V2-S4-fail500.png / V2-S5-twins.png / V3-search-probe.png / evidence.json / evidence-v2.json。
- 脚本：test-results/r344-queryview.cjs、r344-queryview-v2.cjs、r344-queryview-v3.cjs + 对应 run.sh（gitignored）。
- 全部截图为 Playwright Chromium headless 1440x900 本地真浏览器页面，非物理真机；测试域名用 <YOUR_TEST_DOMAIN> 占位。
