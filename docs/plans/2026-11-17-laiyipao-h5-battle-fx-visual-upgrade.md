# laiyipao H5 战斗页特效重构 + 全物品视觉化（2026-11-17）

> **背景**：上一轮（`2026-11-16-laiyipao-h5-visual-redesign.md`）解决了布局引擎与 HUD 排版问题并已部署。
> 用户复测后反馈三类问题：
> 1. **战斗页底部那排红蓝黄色块看不出是干嘛的**（canvas `drawHud` 的热量条 + 技能色块，无任何语义说明）；
> 2. **战斗界面过于简陋**——技能开火、命中、元素反应几乎无视觉反馈（反应只有一行弹字）；
> 3. **装备 / 宝石 / 技能 / 皮肤等所有内容只有文字**，需要全面图标化、美化，达到符合游戏调性的观感。
>
> **红线（与上一轮一致）**：
> - 不改业务逻辑、不改接口契约、不改引擎数值与回放哈希路径 —— `src/game/engine.ts`、
>   `damage.ts`、`elements.ts`、`heatmap.ts` 等玩法文件**零改动**；
>   `render/canvas.ts` 只做呈现层增强（渲染层只读状态 + 消费事件，绝不写引擎状态，架构边界不变）。
> - 测试锚点全部保留（见 §三.0）；perf 预算（每帧 0 渐变对象 / 普通关 ≤120 draw call）不得突破。
> - 不引入任何 UI 依赖、不加路由、不改 `pages.json`。

---

## 一、取证（2026-11-17 代码审读）

### 1.1 「红蓝黄块」的来源

`render/canvas.ts` 的 `drawHud()`（第 1027 行起）在画布底部画了两样东西：

| 元素 | 现状 | 问题 |
|---|---|---|
| 热量条 | 60% 宽的纯色条（蓝 `#58a6ff` → 黄 `#ffd33d` → 橙 `#ff6b35`），上方 11px 小字「热量 90/100」 | 颜色语义（=热度警示档）没有任何说明；「热量」是什么、满了会怎样，玩家无从得知 |
| 技能槽 | 30px 方块 + 元素色描边 + 中央一块元素色实心方块 + 冷却时上半黑遮罩 | **零信息量**：看不到技能名、费用、冷却秒数、就绪状态；「中央色块」与「描边」含义不明 |

### 1.2 战斗反馈现状（逐事件盘点）

| 事件 | 现有反馈 | 缺口 |
|---|---|---|
| `fire` | 无（只有音效 shot） | 无施放视觉 |
| `hit` | 引擎飘字伤害数字 | 无命中火花、暴击无强化表现 |
| `reaction` | 一行弹字（反应名）+ 无效震屏已删 | **七种反应视觉完全同质**，与「反应图鉴」里的宣传完全对不上 |
| `kill` | BOSS 震屏 | 普通击杀无爆散 |
| `leak` | 震屏 | 防线受击无方向性提示 |
| `overheat` | 弹字「过热！」 | 无热量条联动的高亮 |

### 1.3 图鉴/背包现状

`skills/equipment/gems/skins/enemies/reactions` 六页全部是 `.rxn` 文本行（名字 + tag 字 + 描述 + stat 字）；
`bag.vue` 的技能槽 / 装备行 / 宝石 chip 同样纯文字。首页九宫格已经用上 `Icon.vue`（22 图标），
但**物品维度**（部位 / 属性 / 元素 / 稀有度 / 敌型）没有图标体系。

### 1.4 测试锚点（必须保留）

- `battle.test.ts`：`.mute-chip` `.pause-btn` `.pause-go` `.pause-overlay` `.speed-chip` `.hand-card` `.btn`、
  `.mode-note`、分享/结算/倍速/暂停/降级全部交互路径。
- `canvas.test.ts` 断言（呈现层意图，保留）：
  - 弹字文本：`蒸汽爆发` `第 2 波` `油桶` `not_a_terrain` `过热！` `通关 3 星` `防线失守`；
  - 弹字色：`#7ee787`（正常）/ `#8b949e`（抗性）；
  - 敌人：受击 `#ffffff`、护盾环 `#58a6ff`、冻结 `rgba(88,166,255,0.35)`、眩晕 `rgba(255,211,61,0.3)`、
    BOSS 描边 `#ff6b35`、非 BOSS 描边 `#0d1117`、元素层数小方块（ELEMENT_COLOR 计数）；
  - 地形七色；墙体三段色 `#58a6ff/#ffd33d/#ff6b35`；
  - HUD 文本 `热量 90/100`（startsWith）、`元素层数 2 技能`、`过热 ` 倒计时；HUD 黄 `#ffd33d` / 橙 `#ff6b35`。
  - 这些是「着色分支」判据，本轮**只增不改**：新增特效一律不得让上述断言的计数/文本漂移
    （特效层用独立颜色，不与被断言色冲突；`fillStyles` 计数断言场景下粒子层为空）。
- `perf_frame.test.ts`：`maxGrad = 0`（每帧零渐变对象）、`DRAW_CALL_BUDGET=120`、`STRESS_DRAW_BUDGET=700`。
  特效粒子必须走预烘焙精灵 + `drawImage`，数量封顶。

---

## 二、设计方案

### 2.1 战斗特效层（canvas.ts，纯呈现）

新增轻量粒子/特效系统（独立于 popups）：

- **数据结构**：`effects: Array<{kind, x, y, age, life, color, ...}>`，封顶 64 个；每帧按 `dt` 衰减、
  双写指针压缩剔除（与 popups 同款）；超限丢最旧。
- **精灵**：所有光斑/圆环/碎片走 `makeRadialSprite` / 一次性烘焙，**每帧 0 渐变对象**；
  每个 effect 每帧 ≤3 次 drawImage。
- **事件映射**（`handleEvents` 消费，同一批事件音效层也在用，互不干扰）：
  - `fire`：基座处施放光环（元素色 ring 扩散，320ms）；
  - `hit`：命中火花 4~6 粒（白/琥珀，crit 时金色大爆闪 + 双环）；
  - `reaction`：**七种反应各自专属**（下表）；
  - `kill`：死亡爆散 6 粒（BOSS 加震屏已有）；
  - `leak`：防线位置红色闪烁横条（方向性：靠墙）；
  - `overheat`：全屏边缘橙色脉冲 vignette（一次性 400ms）。

| 反应 | 专属特效（配元素主题色） |
|---|---|
| steam_burst 蒸汽爆发 | 白蓝色双环冲击波（120px 溅射半径可视化为环径）+ 上浮蒸汽粒 |
| overheat 过热 | 橙红爆闪 + 放射火花（90px 环）+ 眩晕星点 |
| burn_cloud 燃烧云 | 上浮火焰粒子群（焰色渐隐） |
| superconduct 超导 | 冰蓝电弧折线（随机 zigzag，2~3 段）+ 紫电光晕 |
| flash_freeze 急速冻结 | 冰晶六芒碎片爆散（ice 色）+ 冻结白环 |
| corrosion_spread 腐蚀扩散 | 绿色毒泡上浮 + 溅射环（120px） |
| armor_break 破甲击退 | 灰白金属碎片横飞 + 击退方向冲击线 |

- **反应弹字升级**：纯文字 → 「圆角气泡底 + 描边 + 元素色文字」，保留文本与颜色断言不变。

### 2.2 HUD 重做（drawHud，可读性优先）

- **热量条**：
  - 保留文本 `热量 {heat}/{cap}`（测试 startsWith 断言）与档位色 `#58a6ff/#ffd33d/#ff6b35`（断言）；
  - 新增：条内刻度线（每 25%）、就绪技能的费用刻度点、过热时条体脉冲（alpha 波动）；
  - 左侧加「火苗→仪表」隐喻图标（程序化小图形），右端百分比数字。
- **技能槽**（信息全量可视化）：
  - 尺寸 30→40px；圆角矩形 + 元素色描边 + 元素色**底光**（就绪时亮、冷却时暗）；
  - 中央元素图形（五元素程序化 glyph：焰=火苗、冰=六芒雪晶、电=闪电、毒=液滴泡、动能=冲击星）；
  - 右上角热量费用角标（数字圆片）；冷却时**径向扇形遮罩** + 剩余秒数居中；
  - 就绪瞬间（冷却归零帧）闪光一次；
  - 保留 `元素层数 {n} 技能` 右上文案（测试断言）——语义上它说明「这些槽就是元素层数来源」。
- 不改 HUD 位置/尺寸契约（底部安全距 18px 保留）。

### 2.3 战斗页说明层（battle.vue）

- **图例按钮 + 图例遮罩**：HUD 行 2 加 `?` chip（类名 `help-chip`，新钩子）；
  打开「战场图例」遮罩（`legend-overlay`）：逐项解释 防线/波次/击杀、热量与过热、技能槽与费用/冷却、
  元素层数与反应链（列出七反应与触发元素对）、倍速与暂停。图例样式复用元素色 tag。
  - 首次进入战斗不自动弹出（不干扰老玩家）；图例打开时战斗照常暂停语义**不做**——图例是透明的
    提示卡，玩家自行点暂停即可（避免改变暂停行为被测试钉住）。
- **选牌界面**：卡片加元素图标座（`DefIcon`）+ 稀有度角标 + 类型 chip 重排；弃牌按钮语义化（带 ×）。
- **结算界面**：统计行图标化（击杀/漏怪/反应/分数各配图标）、奖励行货币图标、
  星级保留 star-pop 动画；回放哈希行保持。
- 新增类名一律新钩子，不删旧钩子。

### 2.4 全物品视觉化（DefIcon 组件 + 六图鉴页 + bag）

- 新组件 `components/DefIcon.vue`（渲染函数 SVG，与 Icon.vue 同款工艺，**零依赖**）：
  - `slot:{weapon|helmet|armor|gloves|boots|charm}` 六部位图标；
  - `gem` 宝石切面（按属性 attr 配色）；
  - `elem:{fire|ice|lightning|corrosion|kinetic}` 五元素图形（与战斗内 glyph 同形，世界统一）；
  - `enemy:{normal|ranged|flying|special|elite|boss}` 六敌型图形（与 canvas 内形状一一对应）；
  - `skin` 面具图形；稀有度环 `rarity:{common|rare|epic|legendary}`。
- **稀有度令牌**（common.css）：`--rare:#58a6ff` `--epic:#c9a7ff` `--legendary:#ffd700` 及 soft 底色；
  `.icon-badge` 增加 `.badge-rare/.badge-epic/.badge-legendary/.badge-elem-*` 变体。
- **六图鉴页**：条目左侧加图标座（部位/元素/敌型/稀有度决定图标与配色），
  名称行加稀有度/阶数星标；stat chips 保留（数值不动）。
- **bag.vue**：技能槽格加元素图形与费用；技能库行加元素图标座；装备行加部位图标；
  宝石 chip 加切面图标；皮肤/专精不动（皮肤在图鉴页已覆盖；专精本轮不做）。

### 2.5 不做 / 不碰

- 引擎玩法文件（engine/damage/elements/heatmap/replay/lcg/score/terrain/tuning）零改动；
- api/store 层零改动；
- 不加新页面、不改 pages.json；
- 敌人绘制主体（形状/颜色/层数方块）保持断言色不变——特效只做**叠加层**；
- 专精页 mastery 本轮不做（用户清单未点名，且其节点图布局独立成体系）。

---

## 三、执行顺序

| 批次 | 内容 |
|---|---|
| B1 | canvas.ts 特效层 + 反应专属特效 + 弹字气泡 + drawHud 重做 + canvas.test 断言增量更新（只增） |
| B2 | battle.vue 图例/选牌/结算美化 + battle.test 新钩子用例 |
| B3 | DefIcon.vue + 六图鉴页视觉化 |
| B4 | common.css 稀有度令牌 + bag.vue 视觉化 |
| B5 | 全量验证：type-check / type-check:test / vitest 全量 / build:h5 / perf 预算 / 393×851 截图目检 |
| B6 | 回归审计（git diff 范围核对）+ 本文档执行记录 + 分批提交 |

每批完成即跑 type-check + 相关测试文件。

## 四、验收判据

| # | 判据 | 取证 | 结果 |
|---|---|---|---|
| V1 | 技能槽可读：元素图形 + 费用角标 + 冷却秒数 + 就绪态一眼可辨 | 真域截图（battle-1/2、combat-1..3） | ✓ |
| V2 | 热量条有「热量」文字与刻度，过热态有脉冲 | 截图 + canvas.test | ✓ |
| V3 | 七种反应各有专属特效 | handleEvents 映射 + canvas.test 七反应着色断言 + 截图（过热环 / 超导电弧） | ✓ |
| V4 | fire/hit/kill/leak/overheat 均有视觉反馈 | canvas.test 全事件反馈断言 + 过热环截图 | ✓ |
| V5 | 图例遮罩解释全部 HUD 元素与反应链 | 真域截图 legend-top / legend-mid2（七反应 + 元素对 + 控制说明） | ✓ |
| V6 | 六图鉴页 + bag 每条目有图标座，稀有度/元素配色生效 | 真域截图 codex-* / bag-* | ✓ |
| V7 | 测试锚点全绿：现有断言只增不删；全量 vitest 通过 | 1271 passed / 1 skipped（基线 1264 + 新增 7） | ✓ |
| V8 | perf：maxGrad 仍为 0；draw call 在预算内 | L100 max 110≤120、满载 628≤700、grad 0 | ✓ |
| V9 | 回归：api/store/game 玩法文件零 diff | `git diff --stat` 审计 = 空 | ✓ |
| V10 | build:h5 通过 | DONE Build complete + 真域部署验证 | ✓ |

## 五、执行记录

| 批次 | 状态 | 读数 |
|---|---|---|
| B1 特效层 + HUD | 完成 | canvas.ts：`Effect` 特效层（MAX_EFFECTS=6、高负载半频门、每特效 ≤2 draw call、零渐变）+ 七反应专属特效 + fire/hit/crit/kill/leak/overheat 全事件反馈 + 弹字保留；`drawHud` 重做（热量条 25% 刻度 + 过热脉冲、技能槽 38px 元素图形 + 费用角标 + 冷却扇形 + 剩余秒数）。canvas.test 39 用例全绿（新增 5 条：HUD 可读性 / 七反应着色 / 抗性不出特效 / 全事件反馈 / 上限封顶）；perf：L100 平均 72.5 / 最大 **110 ≤ 120**，满载最大 **628 ≤ 700**，渐变 **0** |
| B2 战斗页说明层 | 完成 | 图例遮罩（热量三档色语义 / 技能槽读法含费用角标 / 七反应逐条带触发元素对 / 顶部与控制逐项）；选牌卡加稀有度 chip + 卡面图标座（元素/属性/机制三形态）；结算统计四项加图标、奖励行加货币图标；battle.test 36 用例全绿（新增 2 条图例开合 + 内容覆盖） |
| B3 DefIcon + 图鉴 | 完成 | 新组件 `components/DefIcon.vue`（24 个内容图标：五元素与战斗画布 glyph 同形 / 六装备部位 / 宝石 / 六敌型与画布形状一一对应 / 皮肤 / 反应双环 / 技能书）；common.css 加稀有度令牌（--rare/--epic/--legendary）+ badge 变体 + rarity-chip；六图鉴页条目全部加图标座与配色（reactions 页加触发元素对 chip） |
| B4 背包视觉化 | 完成 | 出战技能槽元素图标化；技能库/装备/宝石/皮肤行加图标座；宝石属性与品质配色抽到 `utils/labels.ts`（gemAttrBadge / gemQualityBadge）两页共用；新增 `utils/reactionPairs.ts`（lookupReaction 穷举派生）图例与反应图鉴共用 |
| B5 全量验证 | 完成 | type-check ✓ / type-check:test ✓ / **vitest 全量 1271 通过**（基线 1264 + 新增 7，1 skipped 环境门控）/ build:h5 ✓；真域部署（备份 h5.old-20261117005420）后 393×851 逐页截图目检：战斗 HUD 与过热警示环/超导电弧特效 ✓、图例两屏内容完整 ✓、选牌卡 ✓、六图鉴 ✓、背包三页签 ✓ |
| B6 回归审计 | 完成 | `git diff --stat` 审计：**api/ store/ game/ 零改动**（引擎数值、回放哈希路径、接口契约未触碰）；diff 无任何真值（域名/密钥/口令）；提交前逐文件复核通过 |

### 6.1 实现坑位备忘（后续避坑）

- 无头测试桩（canvas.test StubContext）只支持 moveTo/lineTo/arc/closePath —— 新增绘制代码不得使用 quadratic/bezier/roundRect/ellipse（perf 测试的离屏桩无此限制，但两个桩取交集才安全）。
- 特效层颜色不得与 `canvas.test.ts` 的 `fillStyles` 计数断言同色叠加（元素层数方块计数），特效只在 handleEvents 后生成，直接 `draw()` 的分支测试天然为空。
- uni scroll-view 的滚动容器是内部 `.uni-scroll-view`，`scrollTop` 要设在包装层而不是组件根。
- VITE_API_BASE 烘焙进产物后，`||` 右侧的 127.0.0.1 回退字面量仍会留在 bundle 里 —— 验收要看主值而非 grep 计数。
