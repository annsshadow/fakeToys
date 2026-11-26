# R563 laiyipao/miniapp 排行榜页三枚 `@click` 伪按钮 tab 键盘可达性甄别（只读）

## 结论
判定：可达真缺陷 1 项 P2（WCAG 2.1 SC 2.1.1 Keyboard A，关联 SC 4.1.2 Name Role Value A）。rank.vue 排行榜页 3 枚 tab 切换控件键盘完全不可达、辅助技术语义不可读。属新实例，不因同 SC 维度已被其他页覆盖而判 0，亦非据静态预判。

## 范围与入口
- 目标：`laiyipao/miniapp/src/pages/rank/rank.vue`（sha256 `5ffbccdf8a2ebf16c39315ef9b5cc78912da48ae779a3d759e092289b528c926`）。
- 3 枚候选：:5-13 `<view v-for=t in tabs class=tab :class={active:curType===t.key} @click=switchTab(t.key)>{{t.label}}</view>`，硬编码 power 战力榜 / stage 关卡榜 / efficiency 效率榜（:125-129）。全部无 role/tabindex/aria/@keydown。
- 全页唯一 @click 就是这 3 枚 tab；rank-row(:58-71)/my-rank(:79-103) 纯展示无 handler；grep 确认无原生 button、无 @confirm、无 role/tabindex/@keydown/aria-label。
- 入口：index/index.vue:387 排行入口（playMenus 第 7 项）→ `/pages/rank/rank` 1 跳直达首屏可见；H5 hash `#/pages/rank/rank` 直达；onMounted(:302-305) 未登录先 store.login() 再 load()，探针预置令牌跳过 guestLogin。

## 方法
- 自起 `pnpm dev:h5 --port <PORT>`（真值不入库，报告 base 写 `<TEST_HOST>:<PORT>`）；Playwright chromium headless 390x844 DPR2，stub 仅拦 `**/api/v1/**`，造 leaderboard/me-rank 数据。
- 决定性判据：DOM.getAttributes 取原生节点 role/tabindex/aria；DOM.focus 后读 document.activeElement；Accessibility.getPartialAXTree + getFullAXTree 按 backendDOMNodeId 映射读 role/name，过滤 focused=true 且非 RootWebArea 节点。严禁探宿主 wrapper（R535 假阳性教训），UNI-VIEW 身份独立确认。
- 4 组：light/dark × withData/empty，fatals=0、pageErrors=0。

## 决定性证据
- 原始 DOM：3 枚 tab(i=0/1/2) role/tabindex/aria 全 null，class 分别 `tab active`/`tab`/`tab`；partial/full AX role=generic、name=null、ignored=false。
- 焦点：DOM.focus 后 activeElement=BODY、isBody=true、isTarget=false、axFocused=[]，无辅助技术可读焦点。
- 干净 Tab 30 次：`hits{.tabs .tab:0}` distinct=[`BODY:`]，候选 0 命中，全程停在 BODY。
- ★注入对照（决定性去伪）：注入 role=button + tabindex=0 + aria-label=R563-inj-0/1/2 + keydown（Enter/Space 触发 click）后，AX role=button、name 可读，Tab 30 次命中 23 次（distinct=[`UNI-VIEW:tab active`, `UNI-VIEW:tab`, `BODY:`]，UNI-VIEW 身份独立确认非 wrapper）；Enter 切换致 activeIndex=0→1、desc=按最高通关关卡排名、clazz=[tab,tab active,tab]；Space 切换致 activeIndex=1→2、desc=按通关时使用的最低战力排名、clazz=[tab,tab,tab active]。功能真命中，非仅焦点移动。
- 鼠标对照非死代码：点击 tab 回战力榜 activeIndex=2→0、desc=按累计成长值排名。
- 正面对照：battles.vue:135-139 已修 role=button/tabindex=0/:aria-label=查看战报.../@keydown.enter/.space.prevent，AX role=button、name=`查看战报，第3关，胜利，5000分`（count=3）；:50 `svg[aria-label=战绩分数趋势]` AX role=image、name=战绩分数趋势（count=1）。证 AX 通道正常、检测器有效，且本项目已有 a11y 修复范式。

## 等价入口与排除
- rank.vue 无 input/@confirm、无原生 button、无 role/tabindex/@keydown 等价入口；tabs 为 v-for 渲染的 view（H5 编译为 UNI-VIEW），注入后 Tab 命中节点即 UNI-VIEW 本身。
- 注入 keydown 用 `el.dispatchEvent(new MouseEvent(click,{bubbles:true}))` 可触发 Vue @click 链（与 R561 dispatchEvent tap 不走 uni 链不同）。
- withData/empty 两态 tab 恒在，empty rankRows=0、hasMyRank=true，不因数据态改变可达性结论。

## 最小修复方向
3 枚 tab 属同组切换，按语义优先选 role=tab + aria-selected + tabindex（选中项 0/其余 -1）+ @keydown.enter/.space.prevent，或退一步 role=button + aria-pressed + tabindex=0 + @keydown.enter/.space.prevent，复用 switchTab(t.key)，@click 不动、业务零改（沿用 R561/R562 范式，容器可选补 role=tablist）。

## 证据（gitignored：oa4rust/test-results/r563/，借用已忽略目录存放 laiyipao 证据，据实披露）
| 项 | sha256 |
| --- | --- |
| 主报告 r563-report.json | 8152117d83c770d8c821c90fce1aff17b6f673e44c849d3f6d053b700e948b02 |
| 正面对照报告 r563-pos-report.json | 0a1bc7d383bea8b0004f808e5588d8b25cf7a79dce3571b4cef65b4f300dd3bc |
| 截图 withData（light=dark 同 sha） | 57445e0a6745f388ccfee774e73057a29db3e35abb70c40bf1d37a56f796f09e |
| 截图 empty（light=dark 同 sha） | 2c1cae04a93424543671bbd2eac08f14ca67bb0d13c9a56275e03775a37f4f3f |
| 源码 rank.vue | 5ffbccdf8a2ebf16c39315ef9b5cc78912da48ae779a3d759e092289b528c926 |
| 源码 index.vue | 68608d9b8b745210c9aabe2652ee2cb7dc13fe8e0f2805acec52e82e67463107 |
| 源码 battles.vue（正面对照已修） | 38dfe3902068d34672ff2caecae7aa29c5d9f45bb54ffaa1cee787864eac0950 |
| 源码 api/client.ts | cbfe2a5ca9ba88f6e541a6d49712e4864d9b6cd6663600c25983eb85406c25a2 |
| 基线 HEAD | 8c4a80d9cb993acaf0e127aef841cd16b401c985 |

截图 light/dark 同 sha：miniapp H5 无主题切换机制。withData-light 经 read_image 目视有效（排行榜标题 + 3 枚 tab + 分数分布/区间 + 3 榜行 + 我的排名）。