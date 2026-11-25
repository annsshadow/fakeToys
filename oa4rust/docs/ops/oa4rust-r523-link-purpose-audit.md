# R523 只读甄别：WCAG 2.1 SC 2.4.9 Link Purpose (Link Only)「查看全部」链接

- 类型：只读甄别（不改码、不提交源码）
- 真父：d4f7db77670790d0661f03b189e0dbb76024eb7a（台账 §514）
- 目标：工作台首页两个文本相同、目的地不同的「查看全部」链接（我的待办→流程中心 / 通知公告→论坛）
- 运行环境：Playwright Chromium headless，1440×900 DPR1，light/dark 双主题，真浏览器 + CDP DOM.getAttributes + Accessibility.getPartialAXTree + Accessibility.getFullAXTree（链接列表视图）+ DOM 祖先链探针
- 脱敏：报告与日志中的测试域名统一替换为占位符 \`<TEST_HOST>\`，不入库任何真值域名 / IP / 凭据；截图、runner 与探针脚本位于 gitignored 目录 \`oa4rust/test-results/r523-*\]

## 一、结论

**三分判定：可达真缺陷（1 个，SC 2.4.9 Link Purpose (Link Only)，Level AAA）。** 首页 \`<router-link data-testid="dashboard-todo-more">查看全部</router-link>\`（→\`/app/process\`）与 \`<router-link data-testid="dashboard-notice-more">查看全部</router-link>\`（→\`/app/bbs\`）在真浏览器 CDP 完整 AXTree 的链接列表中均为 \`role=link, name="查看全部", ignored=false, focusable:true\`，但目的地不同；去重统计 \`duplicateNames=[{name:"查看全部", count:2}]\`——即读屏用户的「链接列表」视图中出现两条完全无法区分的「查看全部」。两者 DOM 上 \`aria-label\` / \`aria-labelledby\` / \`aria-describedby\` 全为 \`null\`，链接目的**只能**来自文本本身，而文本「查看全部」未描述任一目的地。双主题 8 项探针（2 链接 × 2 主题 + 基线）一致，非工具假阴性。

**与 SC 2.4.4 的区分（关键去伪）：** SC 2.4.4 Link Purpose (In Context)（AA）允许用链接文本**连同编程确定的上下文**（如同容器标题）判断目的；本页两链接分别位于含 \`<h2>我的待办</h2>\` / \`<h2>通知公告</h2>\` 的 \`.panel-header\` 内，2.4.4 可由上下文满足（此前维度已甄别）。但 SC 2.4.9（AAA）明确更严格——要求目的可从 **link text alone** 识别。W3C Understanding 2.4.9 原文：\`links can be understood when they are out of context, such as when the user agent provides a list of all the links on a page\`，并指出「从链接文本可识别目的」是该条的立意。因此「同容器 h2 可作上下文」这一事实**只豁免 2.4.4，不豁免 2.4.9**；链接列表视图中的同名不可区分恰是 2.4.9 的典型失败。按本轮 SC 2.4.9 判据，判可达真缺陷，不臆造为 0。

- 适用层级提示（据实标注，非降格）：SC 2.4.9 属 **AAA**。若项目合规目标仅为 AA，则本条不适用、应以 2.4.4 结论为准；本轮按任务指定的 2.4.9 判据给出 AAA 结论，供队长据合规目标取舍。
- 反预判：结论全部来自 headless Chromium 运行时 CDP 探针，静态 grep 仅作旁证，未以源码推断代替浏览器实证。
- 去伪：已核验两链接 AX name 相同、目的 URL 不同、无任何 aria 编程命名、链接列表可复现 2 条重复；正面对照 quick-card 链接 AX name 自带目的且互异，排除「全站链接皆泛化」的误判。

## 二、源码甄别（逐行锚点）

| 文件 | 行 | 关键代码 | 判定 |
| --- | --- | --- | --- |
| \`oa4rust/frontend/apps/desktop/src/views/Dashboard.vue\` | 53–56 | \`<div class="panel-header"><h2>我的待办</h2><router-link to="/app/process" class="panel-more" data-testid="dashboard-todo-more">查看全部</router-link></div>\` | 缺陷链接 1：文本泛化，无 aria 命名 |
| 同上 | 102–107 | \`<div class="panel-header"><h2>通知公告</h2><div class="notice-actions"><button …>刷新通知</button><router-link to="/app/bbs" class="panel-more" data-testid="dashboard-notice-more">查看全部</router-link></div></div>\` | 缺陷链接 2：文本与链接 1 完全相同，目的地不同 |
| 同上 | 39–48 | \`<router-link v-for="app in quickApps" class="quick-card">…<span class="quick-label">{{ app.label }}</span></router-link>\` | **正面对照**：链接文本自带目的（组织架构 / 工作审批 / 日历 / 论坛…） |
| 同上 | 83 / 90 / 66 | \`打开日历\`(→/app/calendar) / \`去日历添加\`(→/app/calendar) / \`去发起流程\`(→/app/process) | 旁证链接：文本即描述目的，且同文本同目的地，不构成本条失败 |
| 同上 | 27 | \`<div class="stat-distribution" :aria-label="s.label + ' 占比'">\` | 同仓已有 aria-label 补名做法（命名手法基线） |

静态补充检索：\`aria-labelledby|aria-describedby\` 在 Dashboard.vue 中无命中，确认两链接无任何编程命名 / 描述关联。此为背景，决定性结论以第四节浏览器实证为准。

## 三、契约与判据

- **SC 2.4.9 Link Purpose (Link Only)（AAA）**：\`A mechanism is available to allow the purpose of each link to be identified from link text alone, except where the purpose of the link would be ambiguous to users in general.\` 关键在 \`from link text alone\`；链接文本无法区分、且目的对一般用户并不模糊时，不满足。
- **SC 2.4.4 Link Purpose (In Context)（AA）**：允许 link text + 编程上下文（同段 / 列表项 / 单元格 / 同标题区）。本例可由 \`<h2>\` 上下文满足，与 2.4.9 结论并存不矛盾——两级标准严格度不同。
- **读屏链接列表视图**：W3C Understanding 2.4.9 明确 \`such as when the user agent provides a list of all the links on a page\`；CDP \`Accessibility.getFullAXTree\` 的 link 节点 name 去重统计即对该视图的运行时复现。
- **三分判定口径**：①可达真缺陷＝真浏览器实证存在 WCAG 失败且控件可达；②不可达 L1＝控件不可达 / 需特殊权限；③正常＝规范硬要求齐备。本轮对 SC 2.4.9 判①。
- **规范参考**：\`https://www.w3.org/WAI/WCAG21/Understanding/link-purpose-link-only.html\`、\`https://www.w3.org/TR/WCAG21/#link-purpose-link-only\`（SC 2.4.9 / 2.4.4 原文）。

## 四、决定性 CDP 实证矩阵

探针：\`DOM.getDocument(depth:-1)\` → \`DOM.querySelectorAll\` → \`DOM.getAttributes\` → \`Accessibility.getPartialAXTree(fetchRelatives:false)\` → \`DOM 祖先链(page.evaluate)\` → \`Accessibility.getFullAXTree\`（链接列表视图）→ 截图。报告 \`base:'<TEST_HOST>'\`，\`viewport:'1440x900 DPR1'\`。

| 检查项 | 期望（去伪目标） | light 实测 | dark 实测 | 结论 |
| --- | --- | --- | --- | --- |
| \`dashboard-todo-more\` AX \`role/name/ignored/focusable\` | link / 查看全部 / false / true | \`link / 查看全部 / false / true\`；url=\`/app/process\` | 同 | 可达、文本无目的 |
| \`dashboard-notice-more\` AX \`role/name/ignored/focusable\` | link / 查看全部 / false / true | \`link / 查看全部 / false / true\`；url=\`/app/bbs\` | 同 | 可达、文本无目的 |
| 两链接 \`aria-label\` / \`aria-labelledby\` / \`aria-describedby\` | 若有编程命名则去伪为「目的可读」 | 三项全 \`null\` | 同 | 无编程命名，目的仅来自文本 |
| 两链接祖先链含 \`.panel-header\` 与 h2 | 上下文存在（仅 2.4.4 相关） | todo=\`[.panel-header, section.todo-section, .dashboard]\`；notice=\`[.notice-actions, .panel-header, section.notice-section]\` | 同 | h2 上下文存在但不豁免 2.4.9 |
| \`panel-header h2\` AX | heading 可读 | \`我的待办\`(level=2), \`通知公告\`(level=2), 均 ignored=false | 同 | 上下文标题确实存在 |
| \`getFullAXTree\` 链接列表 \`duplicateNames\` | 若为 0 则去伪为「可区分」 | \`[{name:"查看全部", count:2}]\`，fullLinkCount=13 | 同 | **决定性**：链接列表中 2 条不可区分 |
| 正面对照 quick-card AX name | 各链接 name 自带目的且互异 | \`🏢组织架构 / 📋工作审批 / 💬即时通讯 / 📅日历 / 👥会议 / 📁文件 / 💭论坛 / 🤖AI助手\`，全 \`focusable:true\`、互不相同 | 同 | 同仓存在合规基线，非全站泛化 |
| 探针 fatals | 0 | 0（login=ok，capture.error=null） | 0 | 取证完整 |

AX \`nameSources\` 字段由 \`getPartialAXTree\` 返回的是**可用来源类型枚举**（relatedElement / aria-label / contents / title），并非实际命中项；本例实际命名来自 \`contents\`（DOM 三项 aria 属性全 null 佐证），文档不将其误记为已使用 aria-label。

## 五、截图与哈希

- 双主题首页截图（1440×900，DPR1，界面为通用测试账号，无真值）：
  - \`dashboard-light.png\` sha256=\`55a8ca6df384e14fcfd6f1b99bf53ba233ba2f2bbefbe4a5c15dc4463a3b4eac\`（225379 bytes）
  - \`dashboard-dark.png\` sha256=\`c4cccda66247052d43889fbf93eb8fcf90b532062058128cf64e4a48c5c882cb\`（231158 bytes）
- 探针报告 \`r523-link-purpose/report.json\` sha256=\`436e398b2a77ced1cd541989bf1e8092b8107dc740b6bbd11b0a8cc9b93028c9\`
- 截图视觉复核：\`我的待办\` 面板右上角「查看全部」与 \`通知公告\` 面板右上角「查看全部」在 light / dark 下均仅显示为同名文本，无可见目的词；与 CDP AX name 一致。
- 证据文件 \`r523-e2e.cjs\` / \`r523-run.sh\` / \`report.json\` / 截图经 \`git check-ignore -v\` 确认被 \`.gitignore:28 oa4rust/test-results/\` 忽略，不入库。

## 六、三分判定

| 候选 | 判定 | 依据 |
| --- | --- | --- |
| \`dashboard-todo-more\` + \`dashboard-notice-more\`（SC 2.4.9） | **可达真缺陷（1）** | 真浏览器 AX 链接列表 2 条同名「查看全部」不可区分、目的地不同、无 aria 命名、控件可达；2.4.9 要求 link text alone，h2 上下文只豁免 2.4.4 |
| \`打开日历\` / \`去日历添加\` / \`去发起流程\` | 正常 | 文本本身描述目的，且同文本同目的地，链接列表中可区分 |
| 正面对照 quick-card 链接 | 正常（基线） | AX name 自带目的、互异，证明同仓有合规命名做法 |

**建议修复（仅建议，本轮不改码）**：将两条链接的 link text 扩展为含目的的文本（如视觉隐藏文本「我的待办」「通知公告」并入链接名）或补充 \`aria-label="查看全部待办"\` / \`aria-label="查看全部通知公告"\`，使 link text alone 可区分。

## 七、覆盖甄别与备案

- **与已覆盖 2.4.4 区分**：本轮结论限于 SC 2.4.9（AAA）；2.4.4（AA）由同容器 \`<h2>\` 上下文满足，此前维度已甄别，二者不冲突。
- **备料未主判**：\`SearchApp.vue:22–59\` \`summary[role="tab"]\`（全部结果 / 文档 / 流程 / 组织）未带 aria-selected，但去伪风险较高，本轮仅备案不作主判。
- **候选 2 备案（未主判）**：SC 2.4.11 Focus Not Obscured（\`QueryStatementDesigner.vue\` 可聚焦 th + sticky 表头 + body scroll-padding-top 容器疑点）需 CDP 几何实证，去伪高风险，本轮不作判定。
- **取证限制**：链接列表视图以 \`getFullAXTree\` 的 link name 去重复现，等价于读屏「链接列表」的信息维度；未逐个模拟具体读屏软件，但该维度正是 2.4.9 立意所在，证据充分。
- **基线**：真父 \`d4f7db77670790d0661f03b189e0dbb76024eb7a\`（台账 §514）；工作区他人未跟踪文件未触碰。
