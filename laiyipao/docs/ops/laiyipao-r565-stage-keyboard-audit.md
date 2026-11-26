# R565 laiyipao/miniapp 关卡页三处裸 `@click` 伪按钮键盘可达性甄别（只读）

## 结论
判定：可达真缺陷 1 项 P2（WCAG 2.1 SC 2.1.1 Keyboard A，关联 SC 4.1.2 Name Role Value A）。stage.vue 关卡页 2 枚章节 chip、关卡格、开始按钮共 3 类控件键盘完全不可达、辅助技术语义不可读。属新实例，不因同 SC 维度已被其他页覆盖而判 0，亦非据静态预判。

## 范围与入口
- 目标：`laiyipao/miniapp/src/pages/stage/stage.vue`（sha256 `0d3a1225b366829bf4a28555feb81168ee39a148ae4f2e7e9a3bfae0e6621705`）。
- 3 类候选：:16-22 `<view v-for=ch in chapters class=chapter-chip :class={active:curChapter===ch.id} @click=switchChapter(ch.id)>`（章节切换）；:104-116 `<view v-for=lv in chapterLevels class=level-cell :class={...} @click=enter(lv.id)>`（选关）；:218-224 `<view class="btn btn-primary start-btn" :class={btn-disabled:!canEnter} @click=startBattle>{{startLabel}}</view>`（开始战斗）。全部无 role/tabindex/aria/@keydown。
- 全页 grep `@click`/@tap 仅此 3 处；:50 `<svg class=diff-svg aria-label=关卡难度曲线>` 为唯一 aria-label；无原生 button、无 @confirm/@longpress、无 role/tabindex/@keydown。
- 入口：index/index.vue:377 playMenus[0]=`/pages/stage/stage`（关卡）1 跳游客态可达；config 下发后章节 chip/关卡格/开始按钮常驻，非造数写入；H5 hash `#/pages/stage/stage` 直达，预置 lyp_token 跳过 guestLogin。

## 方法
- 自起 `pnpm dev:h5 --port <PORT>`（真值不入库，报告 base 写 `<TEST_HOST>:<PORT>`）；Playwright chromium headless 390x844 DPR2，stub 仅拦 `**/api/v1/**`，造 config（章+关卡+掉落率）/wallet/me/stars/me/levels 详情。
- 决定性判据：DOM.getAttributes 取 role/tabindex/aria；DOM.focus 后读 document.activeElement；Accessibility.getPartialAXTree + getFullAXTree 按 backendDOMNodeId 映射读 role/name，过滤 focused=true 且非 RootWebArea 节点。严禁探宿主 wrapper（R535 假阳性教训），UNI-VIEW 身份独立确认。
- 主探针 4 组 light/dark × withData(3 关:章1两关/章2一关)/empty(0 关)，fatals=0；补充探针修正主探针三处 Fail Loud 缺口，fatals=0、pageErrors=0、consoleErrors=0。

## 决定性证据
- 原始 DOM：章节 chip(2)、关卡格、开始按钮 role/tabindex/aria 全 null；partial/full AX role=generic、name=null、ignored=false。
- 焦点：DOM.focus 后 activeElement=BODY、isBody=true、isTarget=false、axFocused=[]，无辅助技术可读焦点。
- 干净 Tab 30 次：3 选择器 hits 全 0、distinct=[`BODY:`]，候选 0 命中。
- ★注入对照（决定性去伪）：注入 role=button + tabindex=0 + aria-label=R565-inj-0..4 + keydown（Enter/Space 触发 click）后，partial AX role=button、name 可读，Tab 命中 chapter-chip=10/16、level-cell=10、start-btn=5/7（distinct 含 `UNI-VIEW:chapter-chip active`/`UNI-VIEW:level-cell`/`UNI-VIEW:btn btn-primary start-btn`，UNI-VIEW 身份独立确认非 wrapper）。
- 功能性真命中（非仅焦点移动）：章节 chip Enter 致 activeChapter 0→1、levelCount 2→1、详情卡清空；开始按钮 Space 致 hash→`#/pages/battle/battle?level=2`；补充探针关卡格 Enter 致 currentLevel 1→0、clazz=[level-cell current,...]、详情标题切「第 1 关」；关卡格 Space 致 currentLevel 0→2、clazz=[...,level-cell locked current]、startLabel=「第 3 关未解锁」。
- 鼠标对照证非死代码：点回章节 chip0 后 activeChapter 1→0、levelCount 1→2、详情标题恢复第 2 关。
- 正面对照：本页 :50 `svg[aria-label=关卡难度曲线]` AX role=image、name=关卡难度曲线、ignored=false（count=1），证 AX 通道正常、检测器有效，且本项目已有 aria-label 范式。
- withData/empty 两态 3 类控件恒在，结论不因数据态改变。

## 等价入口与排除
- 全页无 input/@confirm/原生 button/role/tabindex/@keydown 等价入口；chip/关卡格/开始按钮均为 view（H5 编译为 UNI-VIEW），注入后 Tab 命中节点即 UNI-VIEW 本身。
- 注入 keydown 用 `el.dispatchEvent(new MouseEvent(click,{bubbles:true}))` 触发 Vue `@click` 链。
- 主探针 pageErrors=2 为 `The play() request was interrupted by a call to pause(). https://goo.gl/LdLk22` 音频中断提示（empty 态 pe=0），补充探针 pe=0 ce=0，非功能缺陷；因 start Space 跳转后主探针 withData 截图落在 battle 页「无法开始战斗」，据实披露并以补充探针 r565-sup-withData.png 为 stage 页有数据态真实样貌。

## 最小修复方向
3 处显式补 role=button + tabindex=0 + 动态 aria-label（章节名/关卡名/开始文案）+ `@keydown.enter/.space.prevent`，分别复用 switchChapter(ch.id)/enter(lv.id)/startBattle，`@click` 不动、业务零改；章节组可选 role=tab+aria-selected 保留 tablist 语义（沿用 R561/R562/R564 范式）。

## 证据（gitignored：oa4rust/test-results/r565/，借用已忽略目录存放 laiyipao 证据，据实披露）
| 项 | sha256 |
| --- | --- |
| 主报告 r565-report.json | af71412400a8424b31b6cd06506d0da57319f4ca4447e5f61518a0a964becf46 |
| 补充报告 r565-supplement-report.json | 33e12558f5e99245fde85b1520adfa9ccdf5c124c078d86c87bcae07195702cd |
| 截图 withData-light | 7eb69f3b42985e3d8343258cf7ed2e1bbbe09c0002f635efd467211e779fcd29 |
| 截图 empty-light | c1b7547ece55d51835c5865f0f62d717eb97564e62e54ee79cd952a0da69878e |
| 截图 withData-dark | 6f2468467cdd5b691e0105b7c8251258f604527cf6b1ef2c061cb0a4a76f659a |
| 截图 empty-dark | dd498792e2abb030c6d85e2d78bb7c82f38eae3fc8e289d2fe2d641d83e2e5e6 |
| 截图 sup-withData（stage 页真实有数据态） | 999923ad376a86e8bd144db31bab41f5816d7c5305d031ce488e7840a0f77083 |
| 源码 stage.vue | 0d3a1225b366829bf4a28555feb81168ee39a148ae4f2e7e9a3bfae0e6621705 |
| 源码 pages.json | cf3d8495926a41b81c5fb168f0ed69e032f364277d7730aa4d73fda5ea8c8191 |
| 源码 store/game.ts | 2055efb8ac15fb7cc3f01173448049e546233e5bcf2221e961e45e8f059090a6 |
| 源码 api/client.ts | cbfe2a5ca9ba88f6e541a6d49712e4864d9b6cd6663600c25983eb85406c25a2 |
| 基线 HEAD | bcde878ce7d9a81a89505093a9331e94a051f144 |

截图 empty-light 与 sup-withData 经 read_image 目视有效；miniapp H5 无主题切换机制，light/dark 截图分别列示。