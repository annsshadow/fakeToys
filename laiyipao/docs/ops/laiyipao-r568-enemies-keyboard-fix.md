# R568 laiyipao/miniapp 敌人图鉴页五类裸 \`@click\` 文本伪按钮键盘可达修复

## 结论
真缺陷 1 项 P2（WCAG 2.1 SC 2.1.1 Keyboard A，关联 SC 4.1.2 Name Role Value A）：\`enemies.vue\` 敌人图鉴页类别 chip（7 枚）、元素 chip（5 枚）、敌人名 rxn-name（3 个）、抗性图开关 resist-toggle（3 个）、清除搜索 clear-search（搜索无结果态）共 5 类 \`text\` 控件键盘完全不可达、辅助技术语义不可读。R567 甄别判定后按真 bug 纪律 RED→GREEN 修复，契约先红后绿，真浏览器决定性判据全绿。

## 方案取舍（Fail Loud）
- 方案 A（全局增强器自动补）**排除**：与 R544/R546/R562/R564/R566/R567 同维度结论一致——全局注入会越界到他页；且本页 aria-label 依赖运行时数据（类别名 c.label、元素名 ELEMENT_NAME[el]、敌人名 en.name、展开态三元），无法从 DOM 静态可靠派生；main.ts 无 MutationObserver 挂载点。半吊子全局注入会造成跨页污染与错误命名，Fail Loud 排除。
- 方案 B（逐页显式补）采纳：5 处开标签就地追加 \`role="button"\`、\`tabindex="0"\`、动态/静态 \`:aria-label\` 与 \`@keydown.enter\` / \`@keydown.space.prevent\`，分别复用原 \`@click\` handler（\`categoryFilter = c.key\` / \`counterElement = el\` / \`toggleEnemy(en.id)\` / \`keyword = ''\`）。原 \`@click\` 全保留，业务状态逻辑零改；与先例同构。
- 类别/元素 chip 未用 \`role="radio" + aria-checked\`：沿用 R561/R562/R564/R566 的 \`role="button"\` + 文案内嵌状态范式（本页 aria-label 即类别/元素名，hit 类由视觉呈现），避免引入未要求的 radiogroup 语义复杂度。

## 写域与零改约束
写域精确 3 文件：
- M \`laiyipao/miniapp/src/pages/enemies/enemies.vue\`（13 insertions / 3 deletions，纯开标签属性追加）
- A \`laiyipao/miniapp/src/pages/R568.enemiesKeyboardA11y.contract.test.ts\`
- A \`laiyipao/docs/ops/laiyipao-r568-enemies-keyboard-fix.md\`（本文）

零改：\`categoryFilter\`/\`counterElement\`/\`toggleEnemy\`/\`keyword\`/\`counterRanking\` 等状态逻辑与 computed、\`@click\` 原 handler、样式、他页、\`main.ts\`、既有 a11y 契约文件、battles/rank/stage 正面对照文件；无全局增强器。

## 5 处修复明细
| # | 控件 | 追加属性 | 复用 handler |
| --- | --- | --- | --- |
| 1 | 类别 chip（v-for=c in CATEGORY_FILTERS，:29-38） | role=button / tabindex=0 / :aria-label=c.label / keydown.enter+.space.prevent | \`categoryFilter = c.key\` |
| 2 | 元素 chip（v-for=el in ELEMENT_ORDER，:46-56） | role=button / tabindex=0 / :aria-label=ELEMENT_NAME[el] / keydown.enter+.space.prevent | \`counterElement = el\` |
| 3 | 敌人名 rxn-name（:88） | role=button / tabindex=0 / :aria-label=查看 + en.name + 抗性 / keydown.enter+.space.prevent | \`toggleEnemy(en.id)\` |
| 4 | 抗性开关 resist-toggle（:91 多行开标签） | role=button / tabindex=0 / :aria-label=（展开?收起:查看）+ en.name + 抗性图 / keydown.enter+.space.prevent | \`toggleEnemy(en.id)\` |
| 5 | 清除搜索 clear-search（搜索无结果态） | role=button / tabindex=0 / aria-label=清除搜索 / keydown.enter+.space.prevent | \`keyword = ''\` |

## 契约 RED → GREEN 与三 verify
契约 9 用例 = 5 核心（blockFrom 起始锚点向下扫至 \`trimEnd().endsWith('>')\` 定位多行开标签；hasA11y 断言 role/tabindex/aria-label/keydown 复用；\`@click\` 保留且总数 5）+ 4 守卫（业务状态逻辑 computed/函数零改 / 数据源样式零改无全局增强器 / battles-ranks-stage 正面对照 + svg 敌人抗性雷达图零改 / 五处 @click 保留且无 tablist/tab/aria-selected）。
- RED：\`git show <真父>:file\` 覆盖真父源码 → EXIT=1，5 failed / 4 passed（核心全红、守卫全绿）。
- GREEN：修复版 → EXIT=0，9 passed。
- V1 定点：\`pnpm test src/pages/R568.enemiesKeyboardA11y.contract.test.ts\` → EXIT=0，9 passed。
- V2 全量：\`pnpm test\` → EXIT=0，161 files passed | 1 skipped（162），1560 tests passed | 2 skipped（1562），32.44s。
- V3 类型：\`pnpm type-check\`（vue-tsc --noEmit -p tsconfig.json）→ EXIT=0，无错误。
三 verify 均在工作目录 \`laiyipao/miniapp\` 以字面命令执行。契约踩坑与修正：初版用例 4 断言 resist-toggle 文案在开标签行内失败（实际文案在下一行），改取 \`source.slice(i, i+600)\` 块断言 \`收起抗性图 ▲\` / \`抗性图 ▼\` 后 GREEN 全过。

## 真浏览器 CDP 决定性判据
自起 laiyipao H5（端口占位见 \`<TEST_HOST>:<PORT>\`，真值不入库），Playwright chromium headless 390x844 DPR2、isMobile/hasTouch；CDP \`Accessibility.getFullAXTree\` 取 \`focused=true\` 且非 RootWebArea 的节点 role/name；\`document.activeElement\` 独立确认（UNI-TEXT 本体，\`isTarget=true\`、\`isBody=false\`，严禁宿主 wrapper 假阳性，R535 教训）；\`DOM.getAttributes\` 取原生 role/tabindex/aria-label。
★ 入口去伪：该页 config 由首页预热，runner 先经首页 \`#/pages/index/index\` 等 1600ms 完成 login→loadConfig，再 SPA 切 hash 到 enemies 页；直接导航会致零 /api/v1/ 请求与敌人表空（R567 首版 4 组 fatal 教训）。

6 组（light/dark × withData/empty/noresult）全绿，fatals=0、pageErrors=0：
- 5 类锚点原生 role=button/tabindex=0，aria-label 精确为「全部/普通/焰/冰/查看 R568焰史莱姆 抗性/查看R568焰史莱姆抗性图/清除搜索」（按态存在），full AX role=button 且 name 一致。
- focus：UNI-TEXT 本体 target=true body=false，axFocused=\`[{role:button,name:...}]\`。
- clean Tab 30 次命中：withData 类别 11/元素 5/rxn-name 5/resist-toggle 5；empty 类别 14/元素 12（数据态无关卡名/开关/清除按钮，据实披露）。
- 注入对照 removed→injected：移除后 role/tabindex/aria 全 null，注入后 role=button/tabindex=0/aria=R568-injected-*，坐实检测器与 AX 通路有效。
- 功能真命中（非仅焦点移动）：类别 chip Enter 致 catHit 0→1、rxnCount 3→1；元素 chip Space 致 eleHit 0→1、克制排行重排（焰史莱姆 -30% 由第 3 升至第 1）；rxn-name Enter 致 toggleTexts 抗性图▼→收起抗性图▲、radarCount 0→1；resist-toggle Enter 致 radarCount 1→0；clear-search Enter 致 keyword 清空、rxnCount 0→3。
- 鼠标零回归：真实 \`.click()\` 后 catHit 1→0、rxnCount 1→3（独立补采脚本，pos-mouse 截图）。
- 正面对照：本页 \`svg[aria-label=敌人抗性雷达图]\` 在 rxn 展开后 AX role=image、name=敌人抗性雷达图、ignored=false（独立补采，sha 见下表）；证 AX 通道正常且本项目已有 aria-label 范式。

## 取证瑕疵披露（Fail Loud）
1. runner 内 \`posSvg\` 探针在 rxn Enter 展开之前采集，该态 svg 尚未渲染，报告里 \`posSvg.full=undefined\`；正面对照由独立补采脚本在展开后取得（role=image、name=敌人抗性雷达图），决定性正面对照数据不依赖 runner 内该字段。
2. noresult 截图拍在 clear-search Enter 之后（keyword 已清空、3 条敌人恢复），故 png 显示列表态而非搜索无结果态；无结果态决定性判据在截图前采集（keyword=不存在的敌人XYZ、clearExists=true、rxnCount=0、resultCount=显示 0 / 共 3 条），截图不承载决定性判据。
3. 报告 sha 口径必须区分：runner 控制台 \`sha(report)\` 用紧凑 \`JSON.stringify(report)\` 得 88732dc4…；文件 \`sha256sum\` 对缩进取证字节得 964cc00e…。R567 首版报告曾出现控制台 59e186c3… 与实测 c6f0e639… 偏差；本轮全部 sha 逐一 \`sha256sum\` 实测核对（见下表），杜绝复现。
4. miniapp H5 无主题切换机制，light/dark 多数同 sha，分别列示据实披露；empty 态 rxn-name/resist-toggle/clear-search 不渲染属数据态正常，不改写类别/元素 chip 可达结论。

## 证据 sha256（全部逐一实测核对；证据目录 gitignored：\`oa4rust/test-results/r568/\`）
| 项 | sha256 |
| --- | --- |
| 修复版源码 enemies.vue | 4f2f7715086150b770c59574437d10f98e1b19271696c72263e32990ae48ab7f |
| 真父源码 enemies.vue（基线） | 956dad21f9267e413442374546b939a0f201a115b6613e7031b144a0ccc6b2d2 |
| 修复版备份（gitignored） | 4f2f7715086150b770c59574437d10f98e1b19271696c72263e32990ae48ab7f |
| R568 契约 | 70008deb34da2e87fbe7bc93d5acb8818acb626356b00c9bb0e8035fd04bd57a |
| E2E runner | 746ad474cb8c0517a96cdb765a1f65792f5105637b503e670e7e7927513a268f |
| E2E 报告（文件 sha256） | 964cc00e6a7b189297ff5cff8e1001872be2d5d91a2952f21cd4a8c69d1ee976 |
| E2E 汇总（runner 内紧凑 JSON sha） | 88732dc42cb560dafc2f8bd2274ed569e48f8f7f01ba29f2f90bdcf0a36c33d1 |
| RED 日志 | 3ddff8b90e6e7e972409a6ace814568ef627349f910df62bdfff3c5c1782d3bb |
| GREEN 日志 | 2bf0decdcd1626ef39ff72b24b7f19e4eecc4360949b7f4cd7513d23bb9a1bb5 |
| V1 定点日志 | 154b5c0d58a672b93e0f885fd7e8fa0091baa3eac0ab228143318b5eb2aef2cd |
| V2 全量日志 | 4d0786443c5dd1fe30f01d2808d8d72bc01d2ce39f5f893e453b29f45233939e |
| V3 类型日志 | 37fb02605f763c6cdb22885730991887ea6c34bf242808be4857988263d5f84f |
| E2E 运行日志 | 03846fa90582272a8b058554f320f22aa2cf6e6ab4c15ac16b6b45b00ea160c9 |
| 正面对照 AX 对象 sha | 80cadd3a696b5853cfeb0417980e1466a619f0f1b7c9be2b0d4067f45baa482f |
| 截图 withData（light=dark 同 sha） | 3769d23772a4afaeaf592aae373a1766bf0fb56fbc6154982aa61e40a0cb2d48 |
| 截图 empty（light=dark 同 sha） | 2ac71134427a292b5c4c4ff47e84294aba8594155b57bc6c31b2ae7471a87ca4 |
| 截图 noresult-light | f04f4a7a20e1b78ff2743fb2cc18d929bd18bd5fae89df977cce2af13ce1a24b |
| 截图 noresult-dark | 382a2e92de39e390eb9389cbc913b8fd21068e4b011ccaa132d266467c37fa73 |
| 截图 pos-mouse-light（真实点击后） | 7048956a89864c21744fb7b39b1ca47e568d34a082ec00ef803290e29c7f83d6 |
| 真父 HEAD | 332192c22097be9d5ea40804bcdc9da2375fd02c
