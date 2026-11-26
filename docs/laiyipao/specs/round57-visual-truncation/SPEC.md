# R57 规范：公告页 768 溢出与敌人图鉴 768 占位截断

## 0. 判定标准与红线
- 来源：docs/laiyipao/recon/ROUND56_VISUAL_RECON.md（7 页 × 3 视口，21 张截图 SHA-256 互异，主驱动+独立驱动复现）。
- 立候选四条件：①客观可测数值+DOM 定位；②真浏览器可复现；③修复仅展示层、不碰 engine/replay/hash/score/settle/loot/tokens/后端结算；④非 R43/R44/R46/R47/R48/R55 已收口项。两候选均满足。
- 本轮只写规范+契约，不改业务源码、不实现、不 git、不写真值、不新增 --r57- 令牌。
- 可改文件白名单（留给实施轮）：laiyipao/miniapp/src/pages/announcements/announcements.vue 与 laiyipao/miniapp/src/styles/common.css。禁改 laiyipao/miniapp/src/pages/enemies/enemies.vue。

## 1. 候选1：768 公告页横向溢出 27px
### 1.1 客观证据（逐字数值）
- probe-768-announcements-announcements.json：innerW=768、docW=795、bodyW=795、hOverflow=27、trunc=[]、lowContrast=[]；repro2 同值。
- loc-768-announcements-announcements.json：.ann-date.mono x=597 w=197 right=795；内部 span x=610 w=173 right=783；文本 2026-11-24 01:10。
- 视口对照：390/1280 announcements hOverflow=0（ROUND56 §2 表 :18/:32）。

### 1.2 源码根因（带行号逐字）
- laiyipao/miniapp/src/pages/announcements/announcements.vue:103-107：
  <view class="ann-head">
    <text v-if="i === 0" class="ann-new" :class="{ hidden: !isFirstGroup(grp[0]) }">最新</text>
    <text class="ann-title">{{ a.title }}</text>
    <text class="ann-date mono">{{ formatDate(a.published_at) }}</text>
  </view>
- 同文件 :245-252 .ann-head { display:flex; flex-direction:row; justify-content:space-between; align-items:center; gap:16rpx; margin-bottom:8rpx }——flex 单行、无 flex-wrap。
- 同文件 :263-270 .ann-date { flex-shrink:0; font-size:20rpx; padding:4rpx 12rpx; border-radius:999rpx; color:var(--muted); background:var(--panel-2) }——固定不收缩。
- 同文件 :272-276 .ann-title { font-size:28rpx; font-weight:600; flex:1 }——flex:1 但无 min-width:0。
- 机制：flex 子项默认 min-width:auto，.ann-title 不设 min-width:0 时不能小于内容宽；.ann-date 又 flex-shrink:0，二者相加超出卡片宽 → 文档 scrollWidth 795，right=795>768。

### 1.3 修复判据（满足其一即可，仅展示层）
- 方案A（推荐）：给 .ann-title 增加 min-width:0（可配合 overflow:hidden/text-overflow:ellipsis/white-space:nowrap 形成省略），让标题在 flex 行内可收缩；.ann-date 保持 flex-shrink:0 与日期绑定不变。
- 方案B：给 .ann-head 增加 flex-wrap:wrap，允许日期在长标题时折行；不得删除「最新」徽标或改分组结构。
- 禁止：删除 .ann-date 或 formatDate(a.published_at) 绑定；改 .ann-date 的 flex-shrink:0；改动分组/过滤逻辑、后端、tokens。

## 2. 候选2：768 敌人图鉴搜索占位截断 57px（对比度正常）
### 2.1 客观证据
- probe-768-enemies-enemies.json：innerW=768、docW=768、hOverflow=0、truncCount=1、trunc[0]={cls:"uni-input-placeholder ph",cw:409,sw:466,t:"搜索敌人名称 / 类别（如 Boss、飞行）"}；sw-cw=57；repro2 同值。
- enemy-dom-768.json：.search-inp rect x=158 w=452；内部 input x=180 w=409；占位节点 uni-input-placeholder.ph w=409、color=rgb(150,160,173)。
- locph-768-enemies-enemies.json：该占位色对背景 rgb(26,34,51) 对比度 ratio=6（≥4.5）——非低对比度，颜色勿动（R55 .ph 走 var(--dim) 惯例正确）。
- 视口对照：390/1280 enemies trunc=0。

### 2.2 源码根因
- laiyipao/miniapp/src/pages/enemies/enemies.vue:34-39 input 已正确写 class="search-inp" placeholder="搜索敌人名称 / 类别（如 Boss、飞行）" placeholder-class="ph"——R55 惯例已落地，禁止回退。
- laiyipao/miniapp/src/styles/common.css:318-330 .inp,.search-inp { display:block; box-sizing:border-box; width:100%; height:76rpx; padding:0 20rpx; background:var(--panel-2); border:1rpx solid var(--border); border-radius:var(--radius-sm); font-size:var(--fs-base); color:var(--text) }——无占位文本省略/不换行/字号适配。
- 同文件 :383-385 .ph { color:var(--dim) }——只设色，无 ellipsis/nowrap。
- 同文件 :183-189 已有 .ellipsis { overflow:hidden; text-overflow:ellipsis; white-space:nowrap; min-width:0 } 同仓先例，可复用其声明而非新造变量。
- 机制：768 下 H5 编译的占位节点宽 409px < 文本原宽 466px，缺省略守卫导致硬截断。

### 2.3 修复判据（仅展示层 common.css）
- 给 .search-inp 或 .ph 增加 text-overflow:ellipsis + white-space:nowrap（建议并配 overflow:hidden），或做占位字号适配；优先复用 .ellipsis 既有声明。
- 禁止：改 enemies.vue（含 placeholder 文案与 placeholder-class="ph"）；改 .ph 的 var(--dim) 色；改令牌三锚点；引入 --r57-*。

## 3. 不变量
- 候选1：class="ann-date mono"、{{ formatDate(a.published_at) }} 绑定与 .ann-date 的 flex-shrink:0 逐字保留。
- 候选2：enemies.vue 的 placeholder-class="ph" 与占位文案逐字保留；.ph 仍走 var(--dim)。
- 通用：tokens.css 三锚点 --lyp-token-version: lyp-tokens-v1 / --lyp-r3-version: lyp-tokens-v3-r3 / --lyp-r6-version: lyp-tokens-r6 保留；全仓无 --r57-*。

## 4. 三视口 QA（实施后真浏览器复查口径）
- 390×844：两页 hOverflow=0、trunc=0（保持）。
- 768×1024：announcements docW 须回到 768（hOverflow=0）；enemies 占位节点 scrollWidth≤clientWidth 或可见省略号，trunc 归零；两页 lowContrast=0。
- 1280×800：两页保持通过。
- 截图与 probe/loc/locph/repro2 JSON 沿用 round56-visual-walk/data 命名，新增 post 版对比。

## 5. 红线
- 不碰 engine/replay/hash/score/settle/loot/tokens/后端结算；不改业务数值与权威口径。
- 只改展示层两文件；禁改 enemies.vue；禁写真值/本机绝对路径/口令/key；禁新增 --r57- 令牌。

## 6. test-first RED 门禁
- 契约：laiyipao/miniapp/src/styles/r57VisualTruncation.contract.test.ts（真实读源码+审计JSON+真实 WCAG 数学，无内嵌实现）。
- 命令：cd laiyipao/miniapp && pnpm exec vitest run src/styles/r57VisualTruncation.contract.test.ts --reporter=dot
- 期望：exit=1，失败仅来自候选1收缩守卫与候选2省略守卫两条真实缺口断言；不变量/令牌 it 通过；无解析失败/模块缺失/超时。
- 实测（2026-11-26）：
  - 命令：cd laiyipao/miniapp && pnpm exec vitest run src/styles/r57VisualTruncation.contract.test.ts --reporter=dot
  - 结果：exit=1；RUN v4.1.12；Test Files 1 failed(1)；Tests 2 failed|2 passed(4)；Duration 315ms（transform 25ms / import 39ms / tests 15ms）。
  - 失败①（:50:114 候选1）：AssertionError: 须给 .ann-title 加 min-width:0（配合省略）或给 .ann-head 加 flex-wrap:wrap 才能容纳长标题，否则 768 溢出 27px: expected false to be true。
  - 失败②（:69:122 候选2）：AssertionError: 占位截断 57px：须给 .search-inp 或 .ph 加 text-overflow:ellipsis + white-space:nowrap（或字号适配），禁止改 enemies.vue: expected false to be true。
  - 通过：候选1不变量（日期绑定+flex-shrink:0）、候选2不变量+令牌（ph 惯例/审计基线/WCAG≥4.5/无 --r57-）。
  - 判定：两条失败均为两候选真实缺口断言，无解析失败/模块缺失/超时；假阳性为零，RED 有效。

## 7. 修订轨迹
- 初稿：按 ROUND56_VISUAL_RECON 两候选落规范与契约。