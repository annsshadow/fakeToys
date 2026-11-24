# R495 WCAG 2.1.1 剩余 v-for 列表交互控件键盘可达性甄别（只读审计）

## 1. 范围与判据
- 收敛 R493 §5 反证披露的其余 v-for + @click 非语义元素候选，本轮实采强候选：QueryStatementDesigner.vue:53 \`.sb-item\`、:312 \`.fav-item\`、:852/858/864/870 \`.hint-tag\`，以及 ProcessWork.vue:120/:151 附件/待阅 \`li.clickable\`。
- 硬纪律：严禁仅凭源码 @click 预判；逐项以真浏览器 CDP DOM + Accessibility.getPartialAXTree + 逐次 Tab + Enter/.focus() 实证；存在等价键盘入口不判缺陷；数据不可达判 L1；复杂设计器交互与 modalFocus 覆盖区不适用。
- ProcessWork.vue:40/62/84 \`li.item-card@click=openWork\` 已由 R493 判定正常（行内办理/查看按钮同功能），本轮不重复。
- 环境：PC 1440×900 DPR1，Playwright Chromium headless 非物理真机，light/dark 双主题；测试地址 \`<YOUR_TEST_DOMAIN>\`，凭据仅由运行期 runner 从环境变量注入，真值零入库。

## 2. 实测方法
- 主轮 \`test-results/r495-e2e.cjs\`：query/process × light/dark = 4 组；采集语句列表 DOM（tag/class/role/tabindex/tabIndexProp/cursor/行内按钮）+ CDP AXTree + 50 步 Tab + 截图 SHA。
- 补采 \`test-results/r495-supplement-v3.cjs\`（ detector 三度修正后定稿）：以 CodeMirror \`.cm-content\` 文本长度 + \`.stmt-name\` input 值 + \`.tb-info\`（\`{{ sql.length }} 字符\`）为激活检测器；对 \`.sb-item\` 做 focus/Enter/鼠标三态对照，对 \`.hint-tag\` 做 focus/Enter/Tab/DOM click 四态对照；并做 30 步 Tab 序列。
- 证据文件：\`test-results/r495-keyboard/report.json\`、\`supplement-v3.json\`、5 PNG，均 gitignored 不入库。
- 方法学教训：SqlEditor 是自定义 CodeMirror 组件（SqlEditor.vue:5-7 \`.cm-wrapper > .cm-editor\`，挂载后 DOM 为 \`.cm-content\`），并非 textarea；初版补采误取无关 \`textarea\` 导致全长度 0，属检测器错误，已弃用该批结论。

## 3. 逐项 programmatic 结果（与 JSON 逐项一致）
### 3.1 主轮（report.json，4 组，pageerrors 全 0）
| 组 | 目标元素 | AX role / ignored | tabindex / tabIndexProp | Tab 命中 | 关键字段 |
|---|---|---|---|---|---|
| query/light | \`.sb-item\` sb-1/sb-2 | generic / false | null / -1 | sbTab.hit=null（50 步被 50+ \`A.admin-nav-item\` 占满，采样不足） | statementCount=14，starButtons=28，tabToStar=36 |
| query/dark | 同上 | 采样为空（statementCount=0，加载时序） | — | sbTab 序列仅 \`A.跳转到主要内容/BODY\` 循环 | statementCount=0，无实证，不扩判 |
| process/light | 工作详情浮层 | — | — | 详情内 Tab 序列 BUTTON 关闭/催办/暂停/恢复/重置/待办转正/追加任务/进入处理/TEXTAREA/驳回/审批通过 循环 | workBtnCount=1，detailOpened=true，attachCount=0，readCount=0 |
| process/dark | 同上 | — | — | 同上 | 同上 |

- \`.sb-item\` DOM：tag=div、role 空、cursor=pointer、tabindex=null、tabIndexProp=-1；AX axName 空、axIgnored=false；行内按钮 \`✏ 编辑\` / \`🗑 删除\` 各 1。
- query/light 的 \`favItems.elements=[]\`、\`favPanelText=""\`、\`favFocus.had=false\`；query/dark 同。收藏面板主轮无条目。
- process 详情 \`attach.elements=[]\` / \`ax={}\` / \`attachFocus.had=false\`，\`readCount=0\`。

### 3.2 补采（supplement-v3.json，1 组 light，pageerrors=0）
- \`.sb-item\`（statementCount=14，\`cmPresent=1\`）：
  - 初始检测器：\`cmLen=0\`、\`name=""\`、\`info="1 行 · 0 字符"\`。
  - \`.focus()\` → \`becameActive=false\`、\`tabIndex=-1\`、tag=DIV：卡片本体不可聚焦。
  - 键盘 Enter → \`afterEnter\` 与 before 完全相同（\`name=""\`，\`cmLen=0\`）：无选中效果。
  - 鼠标点击 → \`name="R343Q313711424-查询"\`：检测器有效，鼠标路径确实把语句载入编辑器并写入 \`.stmt-name\`。
  - 30 步 Tab 自 \`.sb-input\`：\`tabHitSbButton=3\`（第 3 步即命中 sb-item 内 \`✏\`，随后 \`🗑\` 交替），但 \`isSb=true\` 从未出现——键盘只能到行内编辑/删除按钮，不能选中语句本体。
- \`.hint-tag\`（hintCount=74，首个文本 \`SELECT\`）：
  - \`hintFocus.becameActive=false\`、\`tabIndex=-1\`、tag=SPAN、role 空。
  - Enter / Tab / DOM click 四态下 \`cmLen\` 与 \`info\` 均无变化，\`detectorLive=false\`——鼠标对照未生效，检测器未通过自检，本轮不对该元素下键盘结论（不臆造）。
  - 模态 Tab 序列落在 \`A.admin-nav-item\`（\`modalTabHitHint=null\`），提示面板虽为 \`.modal-overlay\`（QueryStatementDesigner.vue:846-872），\`hint-tag\` 不在 modalFocus.ts:12-19 的 FOCUSABLE_SELECTOR 内；但 SQL 编辑区本身支持键盘输入同名 token，属等价路径。

## 4. 源码交叉坐实（决定性结构）
- QueryStatementDesigner.vue:53-55 \`<div v-for="s in filtered" class="sb-item" :class="{active:currentStatement?.id===s.id}" @click="selectStatement(s)">\`；:62-63 行内两按钮 \`@click.stop="editStatement(s)"\` / \`@click.stop="deleteStatement(s)"\`，均非同功能。
- \`selectStatement()\`（:1019-1022）：\`currentStatement.value = s; sql.value = s.sql ?? ''\`；\`currentStatement\` 初值 null（:924）。鼠标点击使 \`.stmt-name\`（:72，\`\${currentStatement?.name}\`）与 CodeMirror 内容更新，证实为有可感知效果的真实功能。
- QueryStatementDesigner.vue:312 \`<div v-for="s in favoriteStmts" class="fav-item" @click="selectStatement(s);showFavoritePanel=false">\`，:315 \`✕\` 按钮 \`@click.stop="toggleFav(s)"\`（非同功能）；\`favoriteIds=ref([])\`（:1217）纯内存无持久化，\`favoriteStmts=computed(...)\`（:1225），面板 \`v-if="showFavoritePanel"\`（:308）——主轮面板内 0 条。
- QueryStatementDesigner.vue:852/858/864/870 \`.hint-tag @click="insertHint(...)"\`；\`insertHint()\`（:2222-2225）\`sql.value += text\`，关闭提示面板。:846 \`<div v-if="showSqlHints" class="modal-overlay">\` 属 modalFocus.ts:10 OVERLAY_SELECTOR 覆盖区。
- ProcessWork.vue:120/:151 附件/待阅 \`li.clickable @click=viewAttachment/viewRead\`，对应 GET 在 :3002/:3013；当前工作详情内 attachCount=0/readCount=0，数据不可达，不判功能缺陷。

## 5. 三分判定
- **可达真缺陷 1 项（P2）**：QueryStatementDesigner.vue:53-55 \`.sb-item\` 语句列表项的『选中并载入编辑器』仅有鼠标 \`@click\` 一条路径。div 无 \`tabindex\`、无 \`role\`、无 \`@keydown\`，AX role=generic、可聚焦=false；键盘用户仅能 Tab 到行内 \`✏ 编辑\`/\`🗑 删除\`（二者非同功能，补采 tabHitSbButton=3），无法把已存语句载入 SQL 编辑区，违反 WCAG 2.1.1。证据：主轮 AXTree + 补采 focus/Enter 全 false、鼠标对照 name 由空变为 \`R343Q313711424-查询\`（检测器自检通过）。
  - 精确修复建议：:53-55 增加 \`role="button" tabindex="0" :aria-pressed="currentStatement?.id===s.id" @keydown.enter="selectStatement(s)" @keydown.space.prevent="selectStatement(s)"\`；行内两按钮保持 \`@click.stop\` 不变，业务 \`selectStatement\` 不修改。或增加一个可见的『打开/选择』按钮作为键盘等价入口。
- **不可达/不适用（L1）**：
  - \`.fav-item\`（:312）：\`favoriteIds\` 内存态、面板 V-if 且主轮 favCount=0，真实 UI 无可测条目，路径不可达，不判缺陷（如需覆盖须先经真实 UI 收藏造数）。
  - ProcessWork 附件/待阅 \`li.clickable\`：当前详情 attachCount=0/readCount=0，无可访问数据，功能是否被阻断不可达，不判缺陷。
  - \`.hint-tag\`（:852 等）：复杂设计器内的提示注入辅助，且位于 modalFocus 覆盖的模态中；SQL 编辑区可直接键盘输入同名 token，属等价路径；本轮检测器自检未通过，不作键盘判定。
- **正常/规范预期若干**：\`.sb-item\` 行内编辑/删除按钮键盘可达（tabHitSbButton=3，序列 ✏/🗑 交替）；process 工作详情浮层内 11 类按钮/文本框键盘可达且 detailOpened=true；R493 已判正常的 process/meeting/form 卡片不重复；skip-link 与 admin 导航装配完整。
- 反证披露：全 views 静态命中 34 处 v-for + @click；本轮实采 2 页 4 组 + 1 组补采，其余候选未逐一实证，依硬纪律不预判、不扩判。

## 6. 截图 SHA256（命令行核验，与 JSON 逐项吻合；SHA_ALL_OK=true）
主轮 report.json：
- pc-query-light-r495 = 049e3dd0a09605f6d7e315c8a0f3e94197b6ad5fd981479a63cd10af20e00ddc
- pc-query-dark-r495 = 98a90dd8579b94ebe3c206e480da3adc9622a5702afaa20cf1fc417480838e64
- pc-process-light-r495 = 7f6d1317b0978d2f03374dc21390bc7acef82f53e2fcbb27b1a17dd734661625
- pc-process-dark-r495 = fe4a637e7d15e4d78f468bf6551649e5270b40f1285527137249569a129998e1
补采 supplement-v3.json：
- pc-query-supplement-light-r495 = be502cd62e3f5804f1b66efcdadb3df12c0cfa5c94f9628ba68d40a901cdb4cd
- 目视核验：主轮 4 图 + 补采 1 图均 1440×900，名实相符（SQL 语句设计器/工作详情浮层）；全部为 headless 非物理真机，不冒充真机。

## 7. 结论
R495 甄别完成：可达真缺陷 1 项 P2（SQL 语句设计器语句项仅鼠标可选中），不可达/不适用 L1 3 类，正常若干。本轮只读不改码，缺陷与精确修复建议供后续轮次。项目继续迭代，非终态。
