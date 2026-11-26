# R564 rank.vue 键盘可达修复文档

## 目标与判定

- 缺陷:laiyipao/miniapp \`src/pages/rank/rank.vue\` 排行榜 3 枚筛选 tab(战力榜/关卡榜/效率榜)为 \`<view class="tab" @click="switchTab(t.key)">\` 裸点击控件,键盘不可达、AT 不可读选中态。
- 规范:WCAG 2.1 SC 2.1.1(键盘可达,A)关联 SC 4.1.2(角色/名称/状态,A)。
- 真父 HEAD:\`98b194e210981127680e014e48faac438a5e662d\`;R563 甄别文档判 P2,按真 bug 纪律先 RED → GREEN。
- 验收:3 枚 tab 均可 Tab 命中,Enter/Space 均触发 switchTab,aria-pressed 正确反映选中态,鼠标零回归,0 pageerror。

## 修复形态决策(Fail Loud)

- 方案(a) WAI-ARIA Tabs + roving tabindex:选中项进 Tab 序列、其余需方向键导航。任务未要求方向键导航,半吊子 tablist 反伤 AT,排除。
- 方案(b) 采用:\`role=button\` + \`aria-pressed\` + \`tabindex=0\` + \`@keydown.enter/.space.prevent\` 复用 \`switchTab(t.key)\`。3 枚全部 Tab 可达,aria-pressed 程序化可读选中态,与 R561/R562 先例一致,改动最小。
- 契约守卫钉死:无 \`role=tab\` / \`role=tablist\` / \`aria-selected\`,防止后续误改成半吊子 tablist。

## 写域与零改边界

写域精确 3 文件:
- M \`laiyipao/miniapp/src/pages/rank/rank.vue\` — 仅 3 处开标签 a11y 属性追加(6 insertions,0 deletions)。
- A \`laiyipao/miniapp/src/pages/R564.rankKeyboardA11y.contract.test.ts\` — 契约 7 用例。
- A \`laiyipao/docs/ops/laiyipao-r564-rank-keyboard-fix.md\` — 本文档。

零改:\`switchTab\` 函数体、\`curType\`、\`tabs\` 硬编码、榜单行展示、样式、他页、\`main.ts\`、已有 a11y 契约(focus-visible/skipLink/labelA11y/inputPurpose)、battles 正面对照、\`@click\` 全保留。

## 源码修复

在 \`v-for="t in tabs"\` 的 \`<view class="tab" :class="{active:curType===t.key}" @click="switchTab(t.key)">\` 开标签追加:

\`\`\`
role="button"
tabindex="0"
:aria-label="t.label"
:aria-pressed="curType === t.key"
@keydown.enter="switchTab(t.key)"
@keydown.space.prevent="switchTab(t.key)"
\`\`\`

- 真父 rank.vue sha256:\`5ffbccdf8a2ebf16c39315ef9b5cc78912da48ae779a3d759e092289b528c926\`
- 修复版 rank.vue sha256:\`87e8f948abc39b65681aa54727f76a692b9ae8850cbc228ada564ab112ca675b\`
- git diff:6 insertions,0 deletions,纯属性追加。

## 契约 RED → GREEN

- 契约 sha256:\`2595ee229ebd48bc8c10054991930366856129dd5d896493d7b6dc894c29eb14\`
- 7 用例 = 3 核心(blockFrom 起始锚点向下扫至 \`trimEnd().endsWith('>')\` 定位多行开标签,断言 role/tabindex/aria-label/aria-pressed/keydown 复用 switchTab + 无 tablist/tab/aria-selected + 三硬编码文案 + @click 唯一)+ 4 守卫(switchTab/curType/load 零改 / 榜单展示+API 零改 / main.ts 零增强器+样式零改 / battles 正面对照零改)。
- 踩坑:初版 \`blockFrom('class="tab"')\` 致 v-for 在上方 → 核心 1 fail;改 \`blockFrom('v-for="t in tabs"')\`(replace_all 两处)+ my-rank 断言改 \`class="card my-rank"\` → GREEN 7pass。
- RED(真父覆盖):EXIT=1,2 failed(核心 1 role/核心 2 aria-pressed)|5 passed。日志 \`/tmp/r564-red.log\`,sha256 \`41bed0867242f6e1fd7950055fb2ab96a66f8304a4f9a5ecb15955ff124d47be\`。
- GREEN(修复版):EXIT=0,7 passed。日志 \`/tmp/r564-green.log\`,sha256 \`c368bd6ce9b58a3e216a174bed3dc0f3abd3cbb54368fd065afb45577bc33504\`。
- RED 用 \`git show 98b194e21:file\` 覆盖真父,随后从 gitignored 备份恢复修复版 sha256 \`87e8f948…\` 坐实。

## 三 verify(workdir \`laiyipao/miniapp\`,字面命令全 exit 0)

1. \`pnpm test src/pages/R564.rankKeyboardA11y.contract.test.ts\` — 7 passed。日志 \`/tmp/r564-v1-lit.log\`,sha256 \`41c6d3e2bf14b4c32d0ef53e22989b6d7243f18d324d18a33a03ebe3b85064dc\`。
2. \`pnpm test\` — 159 files passed | 1 skipped,1546 tests passed | 2 skipped。日志 \`/tmp/r564-v2-lit.log\`,sha256 \`dff6601144763e932fbfe4d7462d82fb28a02a4d3503eb49ead83ab5f5b161d8\`。
3. \`pnpm type-check\`(vue-tsc --noEmit)— 无错误。日志 \`/tmp/r564-v3-lit.log\`,sha256 \`37fb02605f763c6cdb22885730991887ea6c34bf242808be4857988263d5f84f\`。

## 真浏览器 CDP 决定性判据

- Runner:\`oa4rust/test-results/r564/r564-e2e.cjs\`,sha256 \`d5e924ca6120bed5480aa213c54c7e2fececb4f7d1920c2ba4fbb3b0d34a2365\`(gitignored,不入库)。
- 报告:\`oa4rust/test-results/r564/r564-report.json\`,sha256 \`fa21dc25252b5b57680cd080ecca96390e559633d47f1b296353efa985892ffc\`(runner 内部声明对象 sha256 \`72b5e71b2292a3dbe53d79117388c67e86942d3d8731ad19ccefe914fb8500c1\`,落盘格式化后文件 sha 不同,两者均据实记录)。
- 取证:自起 miniapp H5 服务(<TEST_HOST>:<PORT>,本地回环,占位),Playwright 390x844 DPR2 isMobile/hasTouch,CDP \`getFullAXTree\` 取 \`focused===true\` 非 RootWebArea 节点,\`focusProbe\` 用 \`page.evaluate(el.focus())\`;stub \`/leaderboard\` 返回 3 条 R564 数据(withData)/空数组(empty)。
- 最终 \`R564_DONE\` sha256 \`72b5e71b2292a3dbe53d79117388c67e86942d3d8731ad19ccefe914fb8500c1\`,cases=4(light/dark × withData/empty),judgebad=0,fatals=0,pe=0。
- 决定性判据全绿:3 枚 tab 的 \`document.activeElement=UNI-VIEW[role=button]\` isTarget=true isBody=false(未探宿主 wrapper,R535 教训),AX role=button name=战力榜/关卡榜/效率榜;Tab 命中 22 次/30;注入对照 aria removed→\`R564-injected-tab\` 失配→恢复战力榜;Enter 切关卡榜/Space 切效率榜(activeIndex+aria-pressed 同步);鼠标 dispatchEvent(click) 回战力榜;0 pageerror。
- 正面对照(Fail Loud 披露):初版拟用 battles 页 \`svg.trend-svg\`(aria-label=战绩分数趋势),实测 \`querySelectorAll\` count=0(该态未渲染)致 4 例 positive 全 false 假阴性;改跨项目 oa4rust calendar:296 需另起服务,本轮未起;最终改为在被测页注入原生 \`<button aria-label="R564正面对照控件">\`,CDP 验 AX role=button/name 达标,自包含无跨项目依赖。此对照仅证明 AX 采集通路有效,不替代 3 枚 tab 自身判据。
- 截图(780x1776,4 图)目视正常:withData-light \`5d909d5e994536d899db1b73f6200eb2f401dd3296c7bbdf2a3ef0aeaaefb607\`、withData-dark \`669a6adbbde601a7b59edad9f7c8bcea09e2da6a1caa8f940a69397e64a6b7ec\`、empty-light/empty-dark \`2c1cae04a93424543671bbd2eac08f14ca67bb0d13c9a56275e03775a37f4f3f\`。withData 显示 3 枚 tab + 3 条榜单 + 我的排名;empty 显示空态 + 3 枚 tab;效率榜橙框为 Space 切换后焦点环、战力榜橙框为 active 态,均属正常表现。H5 无主题机制,light/dark 内容一致(截图 PNG 编码差异仅随主题色变量)。

## 脱敏声明

- 全文仅含回环/占位 \`<TEST_HOST>:<PORT>\`,无真实域名/IP/SSH 端口/凭据/个人路径。
- 所有真值证据(报告/截图/runner/日志)位于 gitignored \`oa4rust/test-results/r564/\` 与 \`/tmp/\`,不入仓;本文档仅记录 sha256。

## sha 表

| 项 | sha256 |
| --- | --- |
| 真父 rank.vue | \`5ffbccdf8a2ebf16c39315ef9b5cc78912da48ae779a3d759e092289b528c926\` |
| 修复版 rank.vue | \`87e8f948abc39b65681aa54727f76a692b9ae8850cbc228ada564ab112ca675b\` |
| R564 契约 | \`2595ee229ebd48bc8c10054991930366856129dd5d896493d7b6dc894c29eb14\` |
| RED 日志 | \`41bed0867242f6e1fd7950055fb2ab96a66f8304a4f9a5ecb15955ff124d47be\` |
| GREEN 日志 | \`c368bd6ce9b58a3e216a174bed3dc0f3abd3cbb54368fd065afb45577bc33504\` |
| V1 定点日志 | \`41c6d3e2bf14b4c32d0ef53e22989b6d7243f18d324d18a33a03ebe3b85064dc\` |
| V2 全量日志 | \`dff6601144763e932fbfe4d7462d82fb28a02a4d3503eb49ead83ab5f5b161d8\` |
| V3 type-check 日志 | \`37fb02605f763c6cdb22885730991887ea6c34bf242808be4857988263d5f84f\` |
| E2E runner | \`d5e924ca6120bed5480aa213c54c7e2fececb4f7d1920c2ba4fbb3b0d34a2365\` |
| E2E report(落盘) | \`fa21dc25252b5b57680cd080ecca96390e559633d47f1b296353efa985892ffc\` |
| R564_DONE 声明 sha | \`72b5e71b2292a3dbe53d79117388c67e86942d3d8731ad19ccefe914fb8500c1\` |
| withData-light 图 | \`5d909d5e994536d899db1b73f6200eb2f401dd3296c7bbdf2a3ef0aeaaefb607\` |
| withData-dark 图 | \`669a6adbbde601a7b59edad9f7c8bcea09e2da6a1caa8f940a69397e64a6b7ec\` |
| empty-light/dark 图 | \`2c1cae04a93424543671bbd2eac08f14ca67bb0d13c9a56275e03775a37f4f3f\`
