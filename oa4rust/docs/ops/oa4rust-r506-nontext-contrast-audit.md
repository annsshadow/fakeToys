# R506 WCAG 2.1 AA 1.4.11 非文本对比度甄别（只读）

## 1. 依据与判据

- 依据：WCAG 2.1 AA 1.4.11 Non-text Contrast，用户界面组件的视觉边界与状态指示需与相邻颜色至少有 3:1 对比度；判据为“边界/指示器本身”，文本、placeholder、可访问名称存在不替代边界对比度要求。
- 前序边界：R487 与 nonTextContrast.contract.test.ts:105 仅覆盖 --border-subtle / --border-active；本轮聚焦两类未覆盖新场景：日历新建表单的 --ai-glass-border 低 alpha 边界；SQL 语句设计器（QSD）参数/可视化浮层使用未定义变量 --border-color 导致边框声明失效。
- 三分判定：可达真缺陷（边界对比度实测不足 3:1 或边界完全消失且无替代指示器，且为功能性控件）/不可达 L1（页面状态或数据不可达）/正常规范预期（达标或有替代指示器）；须真浏览器实测，不得仅凭源码预判。
- 读图 1440x900 headless Chromium，非物理真机；light/dark 双主题分别判定；截图与 JSON 存于 gitignored 的 test-results/r506-nontext-contrast/，域名真值仅存在 gitignored runner，文档内用占位符。

## 2. 方法与证据链

- 主轮：Playwright Chromium headless 1440x900 DPR1，localStorage oa4rust_theme 切主题后等 1600ms 避开 transition；真实交互打开日历新建表单（aria-label=新建日历）、QSD 参数绑定浮层（button[title=参数绑定] → +添加参数）、QSD 可视化浮层（+可视化 → +添加条件）。
- 量测：getComputedStyle 取 borderColor/borderWidth/borderStyle/backgroundColor，按层 alpha 合成 borderFinal 与外层相邻背景 outer， WCAG 相对亮度计算 ratio；CDP DOM + Accessibility.getPartialAXTree 坐实控件 role/name/ignored；同步截图。
- 补采：专测 QSD main .btn、param（.param-input/.param-type/.param-del/.modal-box .btn-close/.param-body .btn-sm）与 visual（.ve-input-sm/.ve-select-sm/.ve-add-btn/.ve-del-btn/.ve-select）共 30 控件，逐项取 border 与 bgRatio，判定 identifiable=max(borderRatio,bgRatio)>=3。
- 边界探针：对同一批控件额外取 boxShadow、outlineStyle/outlineWidth、backgroundImage、borderBottom、filter、opacity，排除替代边界指示器；输出 probe-boundary.json。
- 反预判核实：读取源码 QueryStatementDesigner.vue:289-310（参数浮层）/:346-390（可视化浮层）模板与 :2322/:2325/:2378 等 CSS，搜索确认 ai-tokens.css:36/:78 只定义 --ai-glass-border（dark rgba(148,163,184,0.16)、light rgba(15,23,42,0.1)），全前端无 --border-color 定义；CalendarApp.vue:46-70 表单与 :819-820 CSS 用 var(--ai-glass-border)。

## 3. 主轮实测（report.json，R506_DONE groups=6 shots=6 errors=0）

- 日历 light：.cal-create-input（placeholder 请输入日历名称/aria 日历名称）、.cal-create-color（aria 日历颜色）、.cal-create-cancel（文本 取消）三控件 border=1px solid rgba(15,23,42,0.1)、borderFinal={230,231,233}、outer={254,254,255}，ratio=1.23 < 3:1；AX .cal-create-cancel role=button/name=取消/ignored=false；控件可见且在视口内；funcFail=3。
- 日历 dark：同三控件 border=1px solid rgba(148,163,184,0.16)、borderFinal={34,42,57}、outer={12,19,33}，ratio=1.29 < 3:1；AX 同为 button/取消/ignored=false；funcFail=3。
- QSD 参数 light/dark：.param-input borderColor=rgb(15,23,42)/rgb(240,246,252) 但 borderWidth=0px/borderStyle=none（整条声明失效），bg=rgb(248,250,252)/rgb(19,30,51)，outer={241,245,249}/{12,15,26}，bgRatio≈1.05/1.15；AX role=textbox/name=默认:/ignored=false；主轮 funcFail=0（功能性边界缺失另由补采量化）。
- QSD 可视化 light/dark：.ve-input-sm 各两条同样 border=0px none、bgRatio≈1.05/1.02~1.15；AX role=textbox/name=值/ignored=false；主轮 funcFail=0。
- 主轮汇总：itemsTotal=6（均日历三控件双主题）、funcFail=6、overallMin=1.23、fails=6 条均为日历控件；QSD 边界消失不在主轮 func 统计内，补采单独判定。

## 4. 补采与边界探针（supplement.json / probe-boundary.json）

- 补采汇总：R506_SUP_DONE groups=2 shots=4 errors=0；controlsTotal=30、borderMissing=28、notIdentifiable=28、minRatio=1.02。
- 双主题 tokenBorderColor 实测为空字符串，坐实 --border-color 未定义；light main .btn(+新建) border=0px none/bgRatio=1.04；.param-input 与 .param-type bgRatio=1.05；.param-body .btn-sm(+全部添加/+添加参数) border=0px none/bg 透明；.ve-input-sm/.ve-select-sm/.ve-select bgRatio=1.05/1.02；.ve-add-btn/.ve-del-btn border=0px none/bg 透明；dark 对应 .param-input/type bgRatio=1.15、main .btn 1.11、.ve-input-sm i0/.ve-select-sm i0/i1 1.02、i1/.ve-select 1.15。
- 仅 2 个例外：.modal-box .btn-close（light borderRatio=19.17、dark=19.75）为浏览器 UA 默认 outset 边框，非设计系统边界，不纳入设计缺陷判定；其 bgRatio light 1.04、dark 3.71。
- 边界探针：QSD light/dark 全部 param（.param-input/.param-type/.param-body .btn-sm）与 visual（.ve-input-sm/.ve-select-sm/.ve-select/.ve-add-btn）均 boxShadow=none、outlineStyle=none（outlineWidth=3px 但样式 none 无效）、backgroundImage=none、border=0px none rgb(...)、borderBottom 同、filter=none、opacity=1 —— 无任何替代边界指示器。
- 边界探针日历 light/dark 三控件均 boxShadow=none、outlineStyle=none、backgroundImage=none、border=1px solid rgba(...,0.1/0.16)、borderRadius=8px —— 低 alpha 边框是唯一边界指示器，无替代。

## 5. 三分判定

- 可达真缺陷 2 类（P2/P3，按破坏性分级）：
  1. 日历新建表单三控件（CalendarApp.vue:819-820，使用 var(--ai-glass-border)）：light 1.23、dark 1.29，均 < 3:1，唯一边界指示器不满足 WCAG 1.4.11；input 有 label/placeholder、取消按钮有文本与 AX 名称，但这些是内容而非边界，不改变判定；影响新建日历表单的控件边界识别。建议把 --ai-glass-border 的功能性控件边界 alpha 提高到满足 3:1（light/dark 分别校准），或给这三个控件单独加更高对比度边框。
  2. QSD 参数绑定与可视化浮层控件（QueryStatementDesigner.vue:2322/:2325/:2378 等，使用 var(--border-color) 而全前端无定义）：computedStyle border=0px none，28/30 控件边界完全消失、bgRatio 最低 1.02，探针确认无 boxShadow/outline/backgroundImage 替代；AX role/name 正常但边界视觉不可辨识，命中 1.4.11。建议统一定义 --border-color 令牌（或在 QSD 局部回退到 --ai-glass-border 并满足 3:1），覆盖 .param-input/.param-type/.param-del/.ve-*/.param-body .btn-sm 等。
- 不可达 L1：0 项。两处页面浮层与表单均真实打开并量测，数据完整。
- 正常规范预期：日历提交与创建按钮（today-btn，实心高对比填充）、QSD .modal-box .btn-close（UA 默认 2px outset 边框，borderRatio 19.17/19.75）不作为设计缺陷；其余静态候选不在本轮实测范围，不预判。
- 反预判边界：本轮只抽测日历/QSD 两页两浮层，不对全库同类 CSS 变量做泛化结论；日历控件有文本不等于边界达标，QSD 控件有 placeholder/axName 不等于边界可见；两类机制不同（低 alpha vs 变量未定义）分别判定。

## 6. 截图与 sha256（命令行核验）

- report.json   0575b3c849c888d3f3d8da9c7ac08fd426ef18c9ad1aadd30374a90722c01094
- supplement.json 60c3d12d474c1a923f442f340c44605ea05c6aee6a3728d8385e9bcdc9266091
- probe-boundary.json b868f48b5d81fe54ed3e364e83f1231d37539b21287deeb356ac1c16285c543c
- 日历 light pc-calendar-light-r506.png c617c880ad9ced922bfb79a4b07314132cbef5141edcd5d42cb19eae1954d453
- 日历 dark pc-calendar-dark-r506.png 580fde0eb797b45e9df60bf25726b31cd37ab703312c9f13d3d6ff0f6a1f4ffa
- QSD 参数 light pc-qsd-param-light-r506.png 0bb55910028a4d127703e85df378dc041b063333255754a3c3b56c570409708e
- QSD 参数 dark pc-qsd-param-dark-r506.png 6d81c8909d79856003c96dc100b7c2cf8e135b59e31f28362f74149e4bf97eca
- QSD 可视化 light pc-qsd-visual-light-r506.png 7150b0d045d72da17c107a5566cdc0f9824046aaabdd2cabea877a1e9335095f
- QSD 可视化 dark pc-qsd-visual-dark-r506.png b01bc8d46f982680b366d86458ee5b867f49e6046c1f1ee84991a7963472030c
- 补采 QSD 参数 light/dark 与主轮同页同态同 sha（0bb55910.../6d81c890...）；补采可视化 light/dark 同主轮（7150b0d0.../b01bc8d4...）。
- 目视核对：pc-calendar-light-r506.png 显示日历工作日历页，新建日历表单展开（日历名称/颜色/创建/取消），1440x900 非物理真机，名实相符；pc-qsd-visual-sup-light-r506.png 显示 SQL 语句设计器页 + SQL 可视化编辑器浮层（SELECT 字段/FROM 表/WHERE 条件/排序与分页/预览 SQL），1440x900 非物理真机，名实相符。
- 据实披露：历史摘要中记录的主轮/补采 JSON 与日历 light PNG 的旧 sha 与磁盘实存不符（疑似抄录其它轮次值），本轮以命令行 sha256sum 实存值为准，并已逐项核验 JSON 内嵌 PNG sha 与命令行一致。

## 7. 脱敏边界与结论

- 测试域名/主机一律用 <YOUR_TEST_DOMAIN> / <TEST_HOST> 占位；口令由 runner 运行时从测试服读取注入环境变量 OA_TEST_USER/OA_TEST_PASS，文档与入库文件无真值；截图为 headless 浏览器实测，非物理真机。
- 结论：R506 甄别完成，发现两类可达真缺陷（日历 --ai-glass-border 低 alpha 边界 1.23/1.29、QSD --border-color 未定义致 28 控件边界消失），无不可达 L1，正常若干；建议交下一轮做 RED→GREEN 修复。本轮不修改业务代码。