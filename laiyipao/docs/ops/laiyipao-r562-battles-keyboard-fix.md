# R562 laiyipao/miniapp 战报页 6 类裸 @click 伪按钮键盘可达修复

## 背景与方案取舍（Fail Loud）
R561 甄别判定 `laiyipao/miniapp/src/pages/battles/battles.vue` 存在 6 类裸 `@click` 伪按钮，键盘不可达（WCAG 2.1 SC 2.1.1 A，关联 4.1.2 A）。方案 A（全局增强器）被排除：会越界影响他页，且动态名（胜利/失败/关卡/分数）无法从 DOM 可靠派生；采用方案 B：逐页显式补 `role=button`、`tabindex=0`、动态/固定 `aria-label`、`@keydown.enter`/`@keydown.space.prevent` 复用原 handler，先例 R544–R560 全绿。

## 写域与零改
写域精确 3 文件：`laiyipao/miniapp/src/pages/battles/battles.vue`、`laiyipao/miniapp/src/pages/R562.battlesKeyboardA11y.contract.test.ts`、`laiyipao/docs/ops/laiyipao-r562-battles-keyboard-fix.md`。零改：业务 handler（reload/toggleDetail/loadDetail/goVerify/goStage/copyHash/loadMore）函数体、API client、全部 6 处 `@click`、样式、他页、`laiyipao/miniapp/src/main.ts`、R561 甄别文档、`svg aria-label="战绩分数趋势"` 正面对照、已有 a11y 契约。

## 修复明细（6 处，纯属性追加，14 insertions / 4 deletions）
1. 筛选 chip（3 个，`v-for f in filters`）：`role=button`、`tabindex=0`、`:aria-label="f.label + (filter === f.key ? '，当前选中' : '，点击筛选')"`、`@keydown.enter`/`@keydown.space.prevent` 复用 `filter=f.key`。
2. 空态“去关卡页挑一关开打 →”：`role=button`、`tabindex=0`、`aria-label="去关卡页挑一关开打"`、keydown 复用 `goStage`。
3. 战报行（`v-for it in rows`）：`role=button`、`tabindex=0`、`:aria-label="'查看战报，第' + it.level_id + '关，' + (it.result === 'win' ? '胜利' : '失败') + '，' + it.score + '分'"`、keydown 复用 `toggleDetail(it)`。
4. “复制回放哈希”：`role=button`、`tabindex=0`、`aria-label="复制回放哈希"`、`@keydown.enter.stop`/`@keydown.space.stop.prevent` 复用 `copyHash(detail.replay_hash)`（`.stop` 防父行误触，同 R558 先例）。
5. “去验真此局”：`role=button`、`tabindex=0`、`aria-label="去验真此局"`、keydown `.stop` 复用 `goVerify(it.id)`。
6. “加载更多”：`role=button`、`tabindex=0`、`aria-label="加载更多"`、keydown 复用 `loadMore`。

## 契约 RED → GREEN
契约 10 用例 = 6 核心（blockFrom 多行开标签定位 + hasA11y 断言 role/tabindex/aria-label/@keydown.enter/.space）+ 4 守卫（业务 handler 与 API 零改、6 处 @click 保留、main.ts 零增强器、样式/svg 正面对照零改且零原生 button）。踩坑：初版 blockFrom 从 @click 行起始致 class 在上方，2 核心失败；改从 `class="filter-chip"` / `class="battle-row tap"` 起始后 GREEN 全过。真父覆盖 → RED exit 1（6 failed / 4 passed）；修复版 → GREEN exit 0（10 passed）。

## 三 verify（workdir laiyipao/miniapp）
- V1 `pnpm test src/pages/R562.battlesKeyboardA11y.contract.test.ts`：10 passed，exit 0。
- V2 `pnpm test`：158 files passed / 1 skipped，1537 tests passed / 2 skipped，exit 0。
- V3 `pnpm type-check`（vue-tsc --noEmit -p tsconfig.json）：exit 0。

## 真浏览器 CDP 决定性判据
自起 laiyipao/miniapp uni H5（本地回环 `<TEST_HOST>:<PORT>`），Playwright 390×844 DPR2 isMobile/hasTouch，`Accessibility.getFullAXTree` 中 `prop(n,"focused")===true` 且非 RootWebArea 节点为判据（`document.activeElement` 为目标元素、`isBody=false`，严禁宿主 wrapper 假阳性 R535 教训）。6 组 case（light/dark × withData/empty/error）：`R562_DONE sha=7d3b65ed… cases=6 namesBad=0 fatals=0 pe=0`。
- 焦点节点 AX name 全达标：筛选“全部，当前选中”、空态“去关卡页挑一关开打”、战报行“查看战报，第3关，胜利，1234分”、复制“复制回放哈希”、验真“去验真此局”、加载更多“加载更多”；均 `role=button`、`isTarget=true`、`isBody=false`。
- Tab 命中：withData `filter-chip:9 / battle-row:7 / copy:3 / verify:3 / load-more:4`；empty/error `filter-chip:18 / empty-link:6`。
- Enter/Space 等价：Enter 切筛选（aria 由“当前选中”→“点击筛选”），Space 切回（恢复“当前选中”）。
- 注入对照：战报行 aria 由 `查看战报，第3关，胜利，1234分` → `R562-injected-row` → 恢复原值，AX name 同步失配/恢复。
- 正面对照：`svg.trend-svg` `aria-label="战绩分数趋势"` 保持 `role=image`/`tabindex=null`（非本轮修复，据实披露）。
- 鼠标零回归：点击战报行仍展开详情（copyBtn=1、verifyBtn=1），空态点击仍跳转关卡页；0 pageerror / 0 fatal。
- E2E 迭代披露：初版判据要求每个 case 都存在全部控件导致 22 项假阴性（空态无 row/copy/verify/load-more），修正为按态只验存在控件；error 态截图显示为空态（stub 500 被前端兜底），截图不承载决定性判据。

## 截图与 sha 表
4 图 780px 宽（withData/error/empty × light/dark，H5 无主题切换机制 light/dark 同 sha），均目视正常。
| 项 | sha256 |
|---|---|
| battles.vue 修复版 | `38dfe3902068d34672ff2caecae7aa29c5d9f45bb54ffaa1cee787864eac0950` |
| battles.vue 真父版 | `84d51fa7eba2d7cbd78d27c3e0cd4d68b535dc8232e7ad9f7bdb90416e63ac20` |
| R562 契约 | `f40db8bcf0031290929976d28b4615c26f5ecd2b1f1fc39cf86236b7f2bff846` |
| r562-e2e.cjs | `462354ce560086aaa6c7b65ae308b16d8a8d98670e144d848199f657accdd25e` |
| r562-report.json | `f096a0aab13962a44c162ace6afae88ac08b5f1c0d5d9971abcafc17fe42cd79` |
| RED 日志 | `2b379954158e64112cc5f14d1b4633c4c8925e1015d74f528a1fd6befd554ca7` |
| GREEN 日志 | `dbaac9f7fde8e6cffa340b3e0181798a45605623615efe65d916488424d4b9b8` |
| V1 日志 | `f042a4e2be19c5b899de0b4390c115f021e6c8003e44c212ea513c76635f584a` |
| V2 日志 | `a8baa6b2db2717944dc31956f16012e1c612a29847b49e5ba388b704c5c110ad` |
| V3 日志 | `37fb02605f763c6cdb22885730991887ea6c34bf242808be4857988263d5f84f` |
| withData 图（light/dark） | `30561003058607f19b23ecde2cdf1d8b28f1543978f5e583e8193053d1a0d749` |
| error 图（light/dark） | `7f532ab9000af84f84ca5b74ebae9776e8dbf03d3befcc409ac441d771e250c4` |
| empty 图（light/dark） | `f4ec562a0f7e2c7a7754e2a63eda13cc3a7b50a6236cda977cf09c054f72e5c5` |

所有真值（宿主/端口）均以 `<TEST_HOST>:<PORT>` 占位，证据文件位于 gitignored `oa4rust/test-results/r562/`，不入库。