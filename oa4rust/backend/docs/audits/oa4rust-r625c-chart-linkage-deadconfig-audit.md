# R625c QueryStatementDesigner 图表联动 clMode/clYAxis 零消费甄别审计

## 元信息
- 任务: t269 (verification 纯甄别, 零源码改)；轮次: R625c
- 判级: P3（功能死配置, 非安全漏洞, 据实不夸大）
- 真父 HEAD: 371650c14ac4d8cb5179cbeb55513f2c8cb8e379（git 核实 MERGE=YES）
- 写域: 零源码改, 唯一交付本文档（≤80 行, 0 真值）
- runnerSha: 0e28057b033b225282d9b9ff64e80558ed131e3e5da1cef94c6cf0fe1ba96f3b
- 文档文件 sha256 由 sha256sum 独立采集, 文末不写真值, 与 runnerSha 分列且均不含自身 sha

## 源码逐行复核（全部零改, sha256sum 采集）
- oa4rust/frontend/apps/desktop/src/views/QueryStatementDesigner.vue sha=17989aa2eee988ade442076940762f1f5cf5c21a313af6add81e8f9022b02999（2828 行）
  - :20 入口按钮 @click=showChartLinkage 切换 title=图表联动配置（无门控）
  - :455-476 联动面板 v-if=showChartLinkage
  - :460 select v-model=clMode 三选项 filter(点击过滤)/detail(点击查看)/compare(点击对比)
  - :461 select v-model=clXAxis（候选 resultHeaders）；:462 select v-model=clYAxis（候选 numHeaders）
  - :463 input v-model=clFilterField；:467 clPreviewData 列表；:471 applyChartLinkage()；:472 testChartLinkage()
  - :1572-1577 refs 定义: showChartLinkage / clMode=ref(filter) / clXAxis / clYAxis / clFilterField / clPreviewData
  - :1650-1652 applyChartLinkage() 函数体仅 showChartLinkage.value=false, 无任何业务效果
  - :1653-1660 testChartLinkage() 仅当 clXAxis 与 clFilterField 均非空时把 resultData 前 3 行映射成 X 与过滤字段拼接预览, 否则提示先选 X 轴与过滤字段；全程不读 clMode / clYAxis
- 全文件 git grep clMode/clYAxis 各仅 2 处（:460/:462 模板绑定 + :1573/:1575 ref 定义）, 无 .value 读取, 消费链断在定义处
- main.ts sha=e0019e870c2a6b47ed9c3505e8acdfbefff3807ddfe04bb8bc25d76ba02ea3f0 :224-227 path query-statement-designer / name QueryStatementDesigner 真可达
- 后端 git grep chart.?linkage|cl_mode|chart_link 在 *.rs 零命中, 无持久化契约

## 三分据实
- 可达性条件: 登录 → SQL 语句设计页 → 点图表联动入口 → 改模式/Y 轴 → 应用, 100% 复现（静态源码链完整, 无门控）
- 危害: 低。三模式与 Y 轴选择死配置, 应用按钮仅关面板, 联动产品语义未落地；不越权、不写库、不崩溃、无数据破坏
- 限制: 联动产品语义未定义（filter/detail/compare 应产生何种行为无后端/产品契约）；test 按钮部分可用（X 轴+过滤字段预览）；des 包 typecheck=tsc --noEmit 不查 .vue 属盲区

## 判级理由（P3, 据实区分）
- 同页 vizType 死配置 R619 判 P2 因产品语义明确(bar/line/pie 三形态)+兄弟 QueryManagerDeep.vue EChartsView chart-type 先例, R620 已真修复
- 本案 clMode/clYAxis 产品语义未定义、无兄弟实现先例、无后端契约, test 按钮部分可用；故据实降判 P3, 不硬凑 P2
- 排除 P0/P1/L1/0: 非安全、非越权、非数据破坏、非可用功能整体失效

## 真测坐实
- vitest 源码契约探针 tests/r625c_probe.test.ts（用后已删, 64 行, sha=5a6adcabf6814dcd609e8eb1376a521149c996132a716ada24b3d224d7def141）: 4 用例全过（EXIT0, 4 passed）
  ① clMode/clYAxis 各仅 2 处且无 .value 读取；② applyChartLinkage 仅关面板零业务调用；③ testChartLinkage 读 X/过滤字段漏模式/Y 轴；④ 三模式选项+可达路由存在
  探针日志 sha=164f123f1f2e5a7d2753720d387c40cfc7014ee63c251a4c988b5726116dee40, 落 PROBE_LOG_PATH（gitignored）
- 临时 PostgreSQL oneshot（自建容器, 占 TEST_HOST:TEST_PORT, 用后即删）: 建库插样本 3 行正常, information_schema 查 chart/linkage 契约表=0、cl_mode/cl_yaxis/chart_link/linkage 契约列=0, 佐证后端无联动持久化契约；日志 sha=8a72d1995a26e787b2cf22e6ba79086e5ace4759ddbd2b93194184541b2bc9fa
- 台账 git grep docs/ops/current-state.md: R619 记降级披露不硬凑, R620-R625 持续列 clMode/clYAxis 为未甄别线索, 零专项甄别无真测坐实

## 据实披露
- 未跑浏览器 GUI 端到端验证（环境限制）, 以 vitest 源码探针 + PG oneshot 真执行替代
- des 包 typecheck=tsc --noEmit 非 vue-tsc, 不解析 .vue, 该畸形长期存活 HEAD
- 联动产品语义未定义, 本文档仅甄别零消费事实, 不预设应实现的行为
- 纯前端缺陷, PG oneshot 仅佐证后端无契约, 核心结论由 vitest 源码探针承担

## 结论
clMode/clYAxis 零消费 + applyChartLinkage 空实现坐实, 判 P3 据实成立；零源码改, 唯一交付本文档。
文档文件 sha256 由 sha256sum 独立采集, 本文档不含自身 sha。
