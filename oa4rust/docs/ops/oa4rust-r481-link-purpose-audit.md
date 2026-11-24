# R481 WCAG 2.4.4 Link Purpose（重复文字按钮可访问名区分度）甄别报告

## 1. 概况

- 轮次：R481 / 任务 t353 / 甄别类型：只读甄别（不改入库源码）
- 准则：WCAG 2.4.4 Link Purpose (In Context)（Level A）—— 上下文中同名链接/按钮的目的须可区分
- 判据（队长 m04069）：同名「详情/查看/更多」按钮若祖先 listitem/tr 含行标识文本或 aria-label 已区分 → 正常；仅当多个同名按钮无可区分上下文（纯详情且祖先无行语义/无标识文本）→ 可达真缺陷。严禁仅凭源码重复文字预判。
- 来源声明：Playwright Chromium headless，PC 1440x900 DPR1，light/dark 双主题，**非物理真机**。测试服地址/凭据仅在 gitignored runner 运行时注入，本文不记真值。
- 基线：分支 opt1000/oa4rust-pc-ai-refactor，HEAD=770d0b7bc493e6bf953c264ffc5b6426e5aa5329，AB=0/0，工作树干净，本文为唯一新增文档。

## 2. 方法

1. 静态盘点：全 views 正则「>(详情|查看|更多)<」命中 13 处，逐处核对其 DOM 祖先是否含行标识。
2. 主实测：4 列表页 × 双主题 = 8 组，CDP `Accessibility.getPartialAXTree` 取每个目标按钮的 axName / axRole / axIgnored，DOM 同步采集祖先 5 层 aria-label / aria-describedby / title 及最近 li/tr/.tr 行文本。
3. 补采：对默认态无目标按钮的 tab 页，经真实 UI 点击 tab / 加载按钮后重采（attendance 切「v2考勤组/员工配置」，process 切「我发起的/已办」，query-query 点「加载」），双主题共 10 组。
4. 针对性探针：对补采中唯一 rowText 为空的 2 个按钮，单独抓取其完整 7 层祖先链、父节点文本与相邻节点，核实是否真无上下文。
5. 表格语义坐实：读取 AdminTable.vue 源码确认 document 页按钮所在表格是否为原生语义表格。

## 3. 静态候选与路由

- AttendanceApp.vue:167/262/266「详情」；DocumentApp.vue:52「详情」；ProcessWork.vue:71/93「查看」；QueryQueryApp.vue:72「详情」；QueryStatDesignerApp.vue:32 / TemplateApp.vue:37 / ViewApp.vue:31「详情」；LogViewerApp.vue:19 表头；FormDesigner.vue:193 option；ProcessDesigner.vue:1725 查看模式。
- 路由（main.ts）：attendance :101-105 /app/attendance/；process :63-67 /app/process/；document :164-166 /app/admin/document/；query-query :254-256 /app/admin/query-query/。
- 结构预判（须实测证伪/证实）：ProcessWork.vue:71 查看按钮在 li.item-card 内（含 item-title）；DocumentApp.vue:52 在 AdminTable 的 cell-actions 内（行标题列另在 td）；QueryQueryApp.vue:72 在原生 tr 内（含名称/分类/ID 单元格）；AttendanceApp:167 在 div.tr 内（含姓名 span）；:262/:266 在 div.cfg-item 内（含 cfg-name）。

## 4. 逐页实测（主 8 组）

| 页面/主题 | 目标按钮数 | axName / role / ignored | 行上下文 | 可区分性 |
|---|---|---|---|---|
| attendance / light | 1 | 「详情」/button/false | rowTag=div（div.tr），行文本「— 11/20 — — — — 详情 分析 归档 删除」 | 单行唯一；日期 11/20 为标识 |
| attendance / dark | 1 | 「详情」/button/false | 同上 | 同上 |
| document / light | 2 | 「详情」/button/false | rowTag=tr（原生表格行），行1「R330SLOTS_1795286985016 草稿 …」，行2「未命名 草稿 …」 | 行标题单元格区分（R330SLOTS… / 未命名） |
| document / dark | 2 | 「详情」/button/false | 同上 | 同上 |
| process-work / light | 0 | — | 默认「待处理」tab 按钮文本为「办理」，非「查看」 | 不适用（补采覆盖其他 tab） |
| process-work / dark | 0 | — | 同上 | 同上 |
| query-query / light | 0 | — | 默认「查询定义」tab 无「详情」按钮 | 不适用（补采覆盖表面查询） |
| query-query / dark | 0 | — | 同上 | 同上 |

全部目标按钮 ariaLabel=""、title=""、祖先 5 层 aria-label/aria-describedby/title 均为空（ctxLabels=[]）——这意味着**可区分性完全依赖行/列表的可见文本上下文**，故必须逐行核实。

## 5. 补采实测（切 tab / 点击加载，10 组）

| 页面动作/主题 | 目标按钮数 | axName / ignored | 行上下文（rowText 摘要） | 可区分性 |
|---|---|---|---|---|
| process-started（我发起的）/ light | 20 | 「查看」/false | rowTag=li，每行含唯一流程标识（R354E2E1795326456801 / R316flow_1795240874397 / S3 运行闭环 work-1795178849834-ygxm7y 等）+ 时间 | 唯一 ID + 时间，可区分 |
| process-started / dark | 20 | 「查看」/false | 同上 | 同上 |
| process-completed（已办）/ light | 4 | 「查看」/false | rowTag=li，「未命名流程 已办工作 🕒 2026/11/x 已办 查看」 | 标题重复但时间唯一（14:01:24 / 10:13:32 / 17:23:59 / 17:23:57），可区分 |
| process-completed / dark | 4 | 「查看」/false | 同上 | 同上 |
| attendance-employee（员工配置）/ light | 1 | 「详情」/false | 「— 11/20 — — — — 详情 分析 归档 删除」 | 单行唯一 |
| attendance-employee / dark | 1 | 「详情」/false | 同上 | 同上 |
| attendance-v2group（v2考勤组）/ light | 2 | 「详情」/false | 第1个 div.tr 行文本「— 11/20 …」；第2个 rowText 空（见 §7 针对性探针） | 分属两个不同 section，各自区块唯一，见 §7 |
| attendance-v2group / dark | 2 | 「详情」/false | 同上 | 同上 |
| query-surfaces（表面查询）/ light | 0 | — | default 分类无数据 | 不适用 |
| query-surfaces / dark | 0 | — | 同上 | 不适用 |

补采合计 54 个目标按钮，全部 axRole=button、axIgnored=false（均可达）；自动初筛 DISTINCT=52 / AMBIGUOUS=2，歧义 2 个经 §7 探针证实为误判。

## 6. 表格语义坐实

oa4rust/frontend/apps/desktop/src/components/admin/AdminTable.vue:13「<table class="admin-table-grid">」、:14「<thead>」、:16「<th v-for="col in columns">」、:33-35「<tbody>」v-for 行 /「<td>」——document 页「详情」按钮处于**原生语义表格**的 tr + 列头 + 行标题单元格上下文。屏幕阅读器在表格导航模式下可读取同行标题单元格（R330SLOTS… / 未命名）区分两个同名按钮，符合 WCAG 2.4.4 In Context 的「上下文可区分」要求。

## 7. 歧义按钮针对性探针（attendance-v2group 双主题）

补采分析脚本以 closest(li,tr,.tr,[role=listitem],[role=row]) 取行上下文，第 2 个「详情」按钮因祖先是 div.cfg-item（不匹配 .tr）而 rowText 为空。针对性抓取其 7 层祖先链后证实：

- 按钮0：outer「<button class="cfg-mini">详情</button>」，父 span.td.rec-act「详情 分析 归档 删除」，祖父 div.tr「— 11/20 — — — — 详情 分析 归档 删除」；所在 section 为「姓名 日期 … 考勤记录表」——属**考勤记录列表**的详情。
- 按钮1：outer 同上，父 div.cfg-act「详情 删除」，祖父 div.cfg-item「R324考勤组957301 dfe8910f-fa1d-4240-8a30-71cc53fc14a8 详情 删除」；所在 section 为「排班与配置 … v2考勤组 + 新建考勤组」——属**v2考勤组配置列表**的详情，祖先 cfg-item 含唯一组名 + UUID 标识。
- 两个「详情」按钮分属不同功能区块（考勤记录表 / 考勤组配置列表），**各自区块内仅 1 个「详情」**，且祖先 cfg-item 有唯一标识。既非「多个同名按钮共处同一无标识列表」，也不构成目的不可区分。light/dark 结构一致。故 AMBIGUOUS=2 判定撤销，改判可区分。

## 8. 三分判定

- **可达真缺陷：0**。全部 60 个目标按钮（主 6 + 补采 54）均具可区分上下文：原生 tr 行标题单元格、li 唯一流程标识+时间、div.cfg-item 组名+UUID、不同 section 分区；无「多个同名按钮且祖先完全无标识」场景。
- **不可达 L1：0**。所有目标按钮均可见、axIgnored=false，选择器与 DOM 全部命中，无 aria-hidden / display:none / R306C 硬排除页候选。
- **正常规范预期：若干**（60 个按钮逐项有据，见 §4/§5/§6/§7）。
- **技术观察（非本轮缺陷，不判 P 级）**：① AttendanceApp 考勤记录用 div.tr / span.td 模拟表格行，非原生语义表格（属 WCAG 1.3.1 维度，本轮 2.4.4 不判）；② process-completed（已办）4 条标题均为「未命名流程」，靠时间列区分，建议后续数据层补唯一标题以增强可访问性。两项均无可见功能退化，本轮只读不改码。

## 9. 截图 sha256（命令行 sha256sum 原始值，18 张）

主 8 张：

- pc-attendance-light-link-purpose.png = 678bf7a97db9f909f44902582cb3cc4a9dcaa670b3e68032f41dfc16ac91144b
- pc-attendance-dark-link-purpose.png = 85e665fc7e5fad10eae3b95d83cf41381eb3dc24858cdf2ee5c4cb972f582df7
- pc-process-work-light-link-purpose.png = 1cdfa352dc73baedcc374b29e31413d3ca4c4c876b068c3af0dd2ebb53762a72
- pc-process-work-dark-link-purpose.png = 4e03ecfe70674aa5f7d922002b850bb20ee4fc8875bf509e34c977dfd99ef577
- pc-document-light-link-purpose.png = d04912d4e66442967f89a2ef283eccbdf94cd129d42a495fcf95a4d2061bed1f
- pc-document-dark-link-purpose.png = 46ee4ce775e3e01314c2c3ab6d2518e5f2e7849d38526bf0774c0c20fe96b69d
- pc-query-query-light-link-purpose.png = fab7513c0497cf38660876eda2016ff77378c9acdaea1141ffa2637052270889
- pc-query-query-dark-link-purpose.png = 6a0cdb8d6480c1706c01a07d9eb04343f51f0b4e849044b3872c0b14f9d4ef7a

补采 10 张：

- pc-attendance-v2group-light-extra.png = e31c711037e70fe50e10e2370e519277159e0ef1fbb879cae0097b739e01418b
- pc-attendance-v2group-dark-extra.png = a712a2833937cbdbc7f8b168d9714ae4d14f5e78dffb27da8ba8bc78aebacb6d
- pc-attendance-employee-light-extra.png = 1f85b6c8f5559b36c237544cf04f8f30ad747f051c610d9e25826722d0435b36
- pc-attendance-employee-dark-extra.png = 9cf7a55acf934b13da41908a547b0bdd103afd7b5c29f59638b316ba69b7385a
- pc-process-started-light-extra.png = 87e4acf2342421137619c8ba5290f58f238d54e6592507b71d2b2bef7594ed74
- pc-process-started-dark-extra.png = 5ea206970ded7ad354c87084c10a035a538a68158794732ec873cf7227fc2f58
- pc-process-completed-light-extra.png = 8e4ff7cf48dedddbe294b7a98a1797f9a5c3554b9ce450d41e4373cc865cb270
- pc-process-completed-dark-extra.png = a020bdd66a970d0803914d4dffb7b7b997bb2eac7af6ba66aad924c859d42bbc
- pc-query-surfaces-light-extra.png = 20e409272a044dc7fec2206cf067b449aca768672b00e54366aa0d4553f89075
- pc-query-surfaces-dark-extra.png = 051b7d40e2bb699051820056bba56d5df61ea58d209f2d83fce8ba6e37fd6658

全部 sha256 与 report.json / extra.json 内记录一致（命令行核验）。证据目录 oa4rust/test-results/r481-link-purpose/（18 PNG + report.json + extra.json + ambiguity.json + analysis.txt + extra-analysis.txt + 脚本），全量 gitignored（.gitignore:28）。

## 10. 真值纪律

- 测试服域名、IP、SSH 端口、凭据一律不入库；仅存在于 gitignored runner，运行时从受控凭据源提取注入进程环境。
- 本文以 <YOUR_TEST_DOMAIN> / <TEST_HOST> 占位；文中 R330SLOTS_/R324/R354E2E 等均为测试库业务标识（非凭据/域名/IP/个人路径），作为可访问性上下文证据保留。
- 截图为 headless 浏览器渲染，非物理真机，来源已在 §1 明示。
- 本轮只读甄别，未改任何入库源码/契约/台账。