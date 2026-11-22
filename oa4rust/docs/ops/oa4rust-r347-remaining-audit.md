# R347 剩余 15 页只读深度审计（批量甄别）

## 来源与范围
- 任务：t154 [R347] 只读深度审计，20 页未审计清单的剩余 15 页（R346 已审五页不重复）。
- 对象（队长 m06174 修正，含 ControlPanelApp、排除 CreateReport）：CategoryApp / CommonApp / ConfigDesignerApp / ControlPanelApp / DesignCenterApp / FindDesignerApp / LogViewerApp / PdfViewerApp / PortalWidgetDesignerApp / ProcessDesigner / QueryStatementDesigner / QueryTableDesignerApp / ServerApp / ServiceInvokeDesignerApp / ThreeMemberApp。
- 路由均在 /app/admin/ 下：category / common / config-designer / control-panel / design-center / find-designer / log-viewer / pdf-viewer / portal-widget-designer / process-designer / query-statement-designer / query-table-designer / server / service-invoke-designer / three-member。
- 硬约束：只读审计，未改源码/契约/后端/配置；唯一入库物为本文档；脚本与证据落 gitignored test-results/r347-remaining/。
- 凭据仅运行期由 ssh oa4rust-test 读取、不回显；测试域名用 <YOUR_TEST_DOMAIN> / <YOUR_TEST_HOST> 占位，仓库无真值。
- 截图与 evidence 均来自本地 Playwright headless Chromium 1440x900 真浏览器实测，非物理真机。

## 覆盖甄别
- docs/ops 全库对上述 15 个 view 名 grep 命中 total=0，确认此前 0 审计覆盖。

## 真浏览器实测（1440x900 headless Chromium，非物理真机）
- 两轮：首轮 r347-remaining.cjs 的 CRUD 段因 page.evaluate 传两参数报错全部 FATAL（S0 探针仍有效）；v2 r347-remaining-v2.cjs 修复为单对象参数后重跑。
- 0 pageerror（两轮均为 []）。
- 只读/导航/查看器页全部渲染正常：ControlPanel（navCards=8，work/count 与 application/list 两 GET 均 200）、DesignCenter（navCards=14）、LogViewer（log/list 200）、FindDesigner（designer/list/all 200）、PdfViewer（空态）、QueryTableDesigner。
- 大型页只读正常：ProcessDesigner（work/count、designer/list/all 两 GET 200）、QueryStatement（designer/list/all 与 table/list/manage 200）、ServerApp（server/license 200）。

## 三分判定

### 真缺陷（1 个 P1）
- 【P1】PortalWidgetDesignerApp 双层信封解析失配致列表恒空：queryFn（PortalWidgetDesignerApp.vue:90）return Array.isArray(r?.data) ? (r.data as Item[]) : [] 只认一层；后端 GET /api/portal/assemble/designer/widget/list 实测返回 data={count:1, data:[1条]}（deep 探针 dataIsArray=false / innerIsArray=true / count=1 / innerN=1；后端路由见 generated_tests.rs:43590）。结果：rowsRendered=0 且显示「暂无门户组件」空态；新建 POST /portal/assemble/designer/widget/create=200 后 reload echo=false，v2-created 截图仍空态——与 R340 双层信封 P1 完全同型。最小修复方向：对齐 R340，queryFn 改为 Array.isArray(r?.data) ? r.data : (r?.data?.data ?? [])。

### 取证限制
- 破坏性写（ServerApp cache/clear、deploy/delete、send/message、command/execute；ProcessDesigner save；QueryStatement delete/execute；QueryTable execute；Category/Common/ServiceInvoke/ThreeMember delete）未授权，仅验只读渲染与确认守卫，未对真实数据执行。
- 仅取代表性闭环：新建 POST=200 与回显在 Common/ServiceInvoke/ThreeMember 坐实；编辑 save、删除守卫按页覆盖，未逐条走查全部写端点变体。
- Category 新建对话框 inputs=0（自定义结构），未走通用新建闭环；ConfigDesigner 为独立配置编辑器（非表格），仅验 GET /config/system=200 与 13 项配置渲染。
- 自定义表格页（如 ThreeMemberApp:22 裸 tr v-for）无 .admin-table-row class，DOM 行数以 content-panel 文本与空态文案为准。

### 产品语义差异
- Common / ServiceInvoke / ThreeMember：后端 data 为裸数组（dataIsArray=true / innerIsArray=false），单层 Array.isArray 解析正确，新建回显 echo=true（ThreeMember-v2-created 截图新行置顶可见），无 R340 缺陷。
- Common deep n=0、ServiceInvoke n=0 为真空态，与 PortalWidget 的假空态有本质区别。
- ConfigDesigner queryKey=[config,list] 而 queryFn 读 /api/config/system，写走 /api/config/*，系配置编辑器的选项卡混用设计，实测 GET 200 正常渲染，非缺陷。
- ControlPanel / DesignCenter 为纯导航与统计页、LogViewer / PdfViewer 为只读查看器，无写端点，符合设计。
- ProcessDesigner 的 s() 吞错探针、ServerApp s() 吞错探针（:308-361）属可用性探测语义，同 R342/R344 判定，非缺陷。

## 结论
- 剩余 15 页批量甄别完成：1 个 P1 真缺陷（PortalWidget 双层信封致列表恒空，待另派修复），0 个 P0。
- 其余 14 页通过或取证受限，未见其他真缺陷。