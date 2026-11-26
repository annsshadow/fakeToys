# R567 laiyipao/miniapp 敌人图鉴页五类裸 `@click` 文本伪按钮键盘可达性甄别（只读）

## 结论
判定：可达真缺陷 1 项 P2（WCAG 2.1 SC 2.1.1 Keyboard A，关联 SC 4.1.2 Name Role Value A）。enemies.vue 敌人图鉴页类别 chip（7 枚）、元素 chip（5 枚）、敌人名 rxn-name（3 个）、抗性图开关 resist-toggle（3 个）、清除搜索 clear-search 共 5 类 `text` 控件键盘完全不可达、辅助技术语义不可读。属新实例，不因同 SC 维度已被其他页覆盖而判 0，亦非据静态预判。

## 范围与入口
- 目标：`laiyipao/miniapp/src/pages/enemies/enemies.vue`（sha256 `956dad21f9267e413442374546b939a0f201a115b6613e7031b144a0ccc6b2d2`，335 行）。
- 5 类候选：:28-34 类别 chip `v-for=c in CATEGORY_FILTERS class="chip tap" @click="categoryFilter=c.key"`；:40-47 元素 chip `@click="counterElement=el"`；:78 `<text class="rxn-name tap" @click="toggleEnemy(en.id)">`；:81-83 `<text class="resist-toggle" @click="toggleEnemy(en.id)">`；:140 `<text class="clear-search" @click="keyword=''">清除搜索</text>`（搜索无结果态 v-if）。全部无 role/tabindex/aria/@keydown。
- 全页 grep `@click` 仅此 5 类，无原生 button、无 `@confirm`/@longpress、无 role/tabindex/@keydown。:113-114 `<svg class="resist-radar" aria-label="敌人抗性雷达图">` 是唯一 a11y 控件（本页正面对照）。
- 入口：index/index.vue:398 codexMenus `/pages/enemies/enemies`（敌人图鉴宫格）1 跳可达；类别/元素 chip 在 config 下发后常驻、零造数，rxn-name/resist-toggle 需 config 含敌人表（轻），clear-search 需搜索无结果态。

## 方法
- 自起 `pnpm dev:h5 --port <PORT>`（真值不入库，报告 base 写 `<TEST_HOST>:<PORT>`）；Playwright chromium headless 390x844 DPR2，stub 仅拦 `**/api/v1/**`，造 config（3 敌人：普通/飞行/Boss，五维抗性各异）/wallet/me/stars/me。
- ★入口去伪：该页无 onMounted login，config 由首页预热。探针先经首页 `#/pages/index/index` 预热（login→loadConfig），再 SPA 切 hash 到 `#/pages/enemies/enemies`；首版直接导航致 74 请求全静态、零 `/api/v1/`、敌人表空，已据实披露并修正。
- 决定性判据：DOM.getAttributes 取原生节点 role/tabindex/aria；DOM.focus 后读 document.activeElement；Accessibility.getPartialAXTree + getFullAXTree 按 backendDOMNodeId 映射读 role/name，过滤 focused=true 且非 RootWebArea 节点。严禁探宿主 wrapper（R535 假阳性教训），UNI-TEXT 身份独立确认。
- 6 组：light/dark × withData/empty/noresult，fatals=0、pageErrors=0。

## 决定性证据
- 原始 DOM：5 类锚点 role/tabindex/aria 全 null（含 class 仅 chip tap / rxn-name tap / resist-toggle / clear-search）；partial/full AX role=generic、name=null、ignored=false。
- 焦点：类别/元素 chip、rxn-name、resist-toggle 原生 DOM.focus 后 activeElement=BODY、isBody=true、isTarget=false、axFocused=[]。clear-search 在搜索词填充后焦点停在搜索 INPUT（axFocused=[textbox]），系页面态而非该按钮可达，原始该按钮 AX 仍 generic/null，另以注入对照坐实。
- 干净 Tab 30 次：5 选择器 hits 全 0、distinct=[`INPUT:uni-input-input`, `BODY:`]，候选 0 命中，全程停在搜索框或 BODY。
- ★注入对照（决定性去伪）：注入 role=button + tabindex=0 + aria-label=R567-inj-* + keydown（Enter/Space 触发 click）后，AX role=button、name 可读；Tab 命中类别 chip=14/15、元素 chip=7/10、rxn-name=3、resist-toggle=3、clear-search=10（distinct 含 `UNI-TEXT:chip tap hit`/`UNI-TEXT:rxn-name tap`/`UNI-TEXT:resist-toggle`/`UNI-TEXT:clear-search`，UNI-TEXT 身份独立确认非 wrapper）。
- 功能真命中（非仅焦点移动）：类别 chip Enter 致 catHit 0→1、rxnCount 3→1、resultCount 显示 1/共 3；元素 chip Space 致 eleHit 0→1、克制排行重排（焰史莱姆 -30% 由第 3 升至第 1）；rxn-name Enter 致 toggleTexts[0] 抗性图▼→收起抗性图▲、radarCount 0→1；resist-toggle Enter 致雷达收起 radarCount 1→0；clear-search Enter 致 keyword 清空、rxnCount 0→3、resultCount 显示 0/共 3→显示 3/共 3。
- 鼠标对照证非死代码：点击类别 chip0 后 catHit 1→0 恢复全部。
- 正面对照：本页 `svg[aria-label=敌人抗性雷达图]`（rxn-name 展开后出现）AX role=image、name=敌人抗性雷达图、ignored=false、count=1，证 AX 通道正常、检测器有效，且本项目已有 aria-label 范式。

## 等价入口与排除
- 全页无 input（仅搜索框，非候选）/@confirm/原生 button/role/tabindex/@keydown 等价入口；候选均为 text（H5 编译为 UNI-TEXT），注入后 Tab 命中节点即 UNI-TEXT 本身。
- 注入 keydown 用 `el.dispatchEvent(new MouseEvent(click,{bubbles:true}))` 触发 Vue `@click` 链（与 R561 dispatchEvent tap 不走 uni 链不同）。
- withData/empty/noresult 三态结论一致；empty 态 rxn-name/resist-toggle/clear-search 不渲染属数据态正常，不改写类别/元素 chip 不可达结论。

## 最小修复方向
5 类显式补 role=button + tabindex=0 + 动态 aria-label（类别名/元素名/敌人名/抗性图开关/清除搜索）+ `@keydown.enter/.space.prevent`，分别复用原 handler（categoryFilter=c.key / counterElement=el / toggleEnemy(en.id) / keyword=''），`@click` 不动、业务零改；类别/元素 chip 属筛选组可选 role=radio + aria-checked 保留语义（沿用 R561/R562/R564/R566 范式）。

## 证据（gitignored：oa4rust/test-results/r567/，借用已忽略目录存放 laiyipao 证据，据实披露）
| 项 | sha256 |
| --- | --- |
| 修正版主报告 r567b-report.json | 0956b38bb0e944480656f2ba983c2f187ab2c50a8168383c3c3bbfb8bb502d2d |
| 首版报告（Fail Loud：直接导航致 config 未加载、4 组 fatal） | 59e186c3049a1df6efd54dc65fa3a5fb1af28e616f50023390301ff919674d6e |
| 截图 withData（light=dark 同 sha） | 6eab87fb290307dce2da168005bbc2a5124f4a018f385d586c78cc6630c9b487 |
| 截图 empty（light=dark 同 sha） | 2ac71134427a292b5c4c4ff47e84294aba8594155b57bc6c31b2ae7471a87ca4 |
| 截图 noresult-light | 177d387cb622a361e602c972c92a6b30407f0025252b9d25bb2525e8a03fc0a6 |
| 截图 noresult-dark | ca568beb6397128f3f9fec468a92bf144ee554287caa54207adc7c5380c74400 |
| 源码 enemies.vue | 956dad21f9267e413442374546b939a0f201a115b6613e7031b144a0ccc6b2d2 |
| 源码 index.vue | 68608d9b8b745210c9aabe2652ee2cb7dc13fe8e0f2805acec52e82e67463107 |
| 源码 api/client.ts | cbfe2a5ca9ba88f6e541a6d49712e4864d9b6cd6663600c25983eb85406c25a2 |
| 基线 HEAD | 026389be8d59be366dbb22d7f09c9713e5f5c6f9 |

截图 withData/empty/noresult-light 经 read_image 目视有效（类别 chip+元素 chip+克制排行+敌人卡+抗性链接 / 空态 / 搜索无结果+清除搜索）；miniapp H5 无主题切换机制，light/dark 分别列示。