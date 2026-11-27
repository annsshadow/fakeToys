# R619a QueryStatementDesigner vizType 图表类型死配置甄别（P2）

## 一、结论（三分据实）
- 判定：P2。真 UI 可达的功能缺陷，用户选择折线图或饼图与柱状图渲染结果完全相同；无越权、无数据破坏、无外部副作用。
- 严重度：非 P0（不影响可用性与数据）、非 P1（无越权与破坏），定为 P2（功能选择无效，属交互欺骗）。
- 处置：本轮为零源码改的验证型甄别（L1-0），唯一交付本脱敏文档；是否修复交队长排期，本轮不扩大写域。

## 二、锚点与根因（oa4rust/frontend/apps/desktop/src/views/QueryStatementDesigner.vue）
- 控件：第 591 行 select 以 v-model 绑定 vizType，提供 bar/line/pie 三选项。
- 定义：第 1775 行 const vizType = ref（bar），紧随 vizXAxis 与 vizYAxis 定义。
- 渲染函数：第 1782 至 1802 行 renderChart 只读取 vizXAxis 与 vizYAxis 做 Map 聚合，写入 vizBars、vizStats、vizRendered，全程零读取 vizType。
- 模板：第 596 至 602 行 viz-chart 区块内 v-for 遍历 vizBars 固定渲染 viz-bar 柱状条，无任何按类型分支。
- 可达性：第 33 行 click 直接置 showResultViz 为真、无门控；第 586 行弹窗、第 594 行 click 触发 renderChart，入口真实可用。
- 根因：vizType 仅有定义与模板绑定两处，渲染链路完全不消费它，属典型死配置；选折线或饼图与柱状图输出逐字节一致。

## 三、真执行探针（独立 vitest，非纯文本 grep）
- 方法：以 ts.transpileModule 转译 script setup，按大括号配对提取 renderChart 源码，用 new Function 注入 resultData 等 ref 形状上下文真实执行。
- 夹具：三行样本数据（region 与 amount 字段），分别以 bar/line/pie 调用 renderChart，采集 vizBars、vizStats、vizRendered 三值。
- 结果：4 用例全部通过。第 1 用例坐实 renderChart 真实产出柱状行（max 150 / min 80 / avg 117 / 3 点）；第 2 用例坐实三种 vizType 三值完全相等；第 3 用例坐实 renderChart 源码零引用 vizType；第 4 用例坐实模板固定 viz-bar 且无类型分支。
- 全文件 grep：vizType 在整个 oa4rust 范围仅 2 处命中，均为本文件第 591 行绑定与第 1775 行定义，确证零其它消费点。
- 探针为临时审计取证文件，运行后即删除，不入提交、不改任何 vue 源码。

## 四、兄弟对照、台账与盲区
- 兄弟铁证：QueryManagerDeep.vue 第 185 至 192 行已用 EChartsView 并按 chart-type 传 statConfig.chartType；其测试注记坐实同类 chartType 形同虚设缺陷，且已在 R618b 修复。说明同类死配置在本项目已识别为缺陷并有修复先例。
- 台账：在 audits 目录检索 vizType、renderChart、图表类型、可视化类型，命中数 0，无专项甄别记录，本轮为首次覆盖。
- 盲区：desktop 包 typecheck 实为 tsc --noEmit，非 vue-tsc，不检查 vue 模板与 script setup 内部，故此类零消费错误可长期存活于 HEAD。

## 五、同族候选降级披露（不硬凑为独立项）
- 候选2（clMode/clYAxis）：同页第 460/462 行绑定、第 1561/1563 行定义，testChartLinkage 第 1641 至 1647 行只读 clXAxis 与 clFilterField，确属零消费；因与主项同页同族、且本轮聚焦 vizType，降级为观察项，不单列。
- 候选3（ConfigDesignerApp editorLang/editorTheme）：第 21/26 行绑定、第 190/191 行定义，编辑器恒 JSON 解析而无 YAML 分支；但产品是否承诺 YAML 未定，缺需求依据，不判缺陷，降级观察。
- 候选4（HotpicApp keyword）：第 14/16 行绑定、第 98 行定义，搜索绑定是否生效需后端配合验证，超出本轮前端甄别范围，降级观察。
- 以上三项均为据实披露的相邻线索，本轮不作为独立缺陷项、不计入结论，避免硬凑。

## 六、Fail Loud 与哈希分列
- 未跑 GUI：以独立 vitest 真执行探针加源码逐行核对替代手工点击；探针夹具首版因把 resultData 传为裸数组（缺 value 层）报 TypeError，已修正为 ref 形状后 4/4 通过，此过程据实披露。
- 未做修复：本轮零源码改，未验证修复后行为，修复效果待单独任务。
- 真父提交：467944b66e65c5fed893a83343c5a3a9f6667ffd。
- 文件 sha256（QueryStatementDesigner.vue）：8c00655cac1b532fe182d16ae251c3f1d7421e13fc1722763079fcf7a97be7f7。
- 临时探针文件 sha256（跑后删除）：ac580f19445d3a1e12017e55ce2eb1b0c618f1295304bf52b78ec2f7a42ce6f3。
- 探针日志 sha256：7842449a4eb6158c6f6333db273f7c56316340d1b8d59ec2e1148b7f6cc23cc0。
- runnerSha（sha256 of JSON meta，node:crypto 无文件 IO）：9f1776178aa8e90cfd232feeca1d7519e546bffe9cb2df922bf1b2fba2027a90。
- 上述文件 sha256 与 runnerSha 分列，互不充当；全文零真值（无密钥、口令、私钥、真实域名、IP、端口映射或个人路径）。